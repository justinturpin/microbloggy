use rand::Rng;

use tide::Request;
use tera::Tera;

use sqlx::{Sqlite, SqlitePool};
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::Executor;

use serde_json::Value;
use std::str::FromStr;

mod config;
mod routes;
mod routes_api;
mod tests;
mod images;

#[derive(Clone)]
pub struct State {
    tera: Tera,
    sqlite_pool: sqlx::SqlitePool,
    config: config::Config
}

#[derive(Clone, Default, Debug)]
pub struct MessageFlashes {
    messages: String
}

/// Tera Markdown Filter - Uses pulldown_cmark to parse Markdown
/// text and render it as HTML. The output should be piped through
/// "safe" to ensure Tera doesn't try to sanitize it.
fn markdown_filter(value: &Value, _: &std::collections::HashMap<String, Value>) -> tera::Result<Value> {
    let parser = pulldown_cmark::Parser::new(value.as_str().unwrap());

    let mut output = String::new();

    pulldown_cmark::html::push_html(&mut output, parser);

    Ok(serde_json::value::to_value(output).unwrap())
}

fn register_middleware(app: &mut tide::Server<State>, config: &config::Config) {
    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        config.session_secret.as_bytes()
    ));

    app.with(tide::utils::Before(|mut request: Request<State>| async move {
        let session = request.session_mut();

        if session.get::<String>("csrf_token").is_none() {
            // Use system-provided CSPRNG source. This will block if there's
            // not enough randomness, which is fine.
            let mut rand = rand::rngs::OsRng;

            session.insert("csrf_token", format!("{}", rand.gen::<u64>())).unwrap();
        }

        let messages = session.get::<String>("messages");

        if let Some(value) = messages {
            request.set_ext(MessageFlashes{
                messages: value
            });

            // Re-borrow session for lord knows why
            request.session_mut().remove("messages");
        }

        request
    }));

    // Add Security Headers
    app.with(tide::utils::After(|mut res: tide::Response| async move {
        res.append_header("X-Frame-Options", "DENY");
        res.append_header("X-Content-Type-Options", "nosniff");
        // res.append_header("Content-Security-Policy", "default-src 'self'");

        Ok(res)
    }));
}

fn register_routes(app: &mut tide::Server<State>, config: &config::Config) {
    // Main Routes
    app.at("/").get(routes::index);

    app.at("/user/login").get(routes::user_login);
    app.at("/user/login").post(routes::user_login_post);
    app.at("/user/profile").get(routes::user_profile);
    app.at("/user/profile/edit").get(routes::user_profile_edit);
    app.at("/user/profile/edit").post(routes::user_profile_update);

    app.at("/post/create").post(routes::post_create);
    app.at("/post/view/:post_id").get(routes::post_view);
    app.at("/post/share/:short_url").get(routes::post_view_share);
    app.at("/post/edit/:post_id").post(routes::post_edit);
    app.at("/post/delete/:post_id").post(routes::post_delete);
    app.at("/post/image-upload").put(routes::put_image_upload);

    app.at("/api/index").get(routes_api::index_api);

    // Static Files (fonts, favicon, css)
    app.at("/static").serve_dir("static").unwrap();

    // User-uploaded images
    app.at("/uploads").serve_dir(config.uploads_path.as_path()).unwrap();
}

async fn bootstrap_database(config: &config::Config) -> tide::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(config.database_url.as_str())?
        .journal_mode(SqliteJournalMode::Delete)
        .create_if_missing(true);

    let sqlite_pool = SqlitePool::connect_with(options).await?;

    // Run migrations, bringing the database schema up-to-date
    sqlx::migrate!("./migrations")
        .run(&sqlite_pool)
        .await?;

    let mut connection: PoolConnection<Sqlite> = sqlite_pool.acquire().await?;

    // Bootstrap user (only 1 user for now hardcoded as user id 1)
    let user = sqlx::query!("SELECT username FROM users")
        .fetch_optional(&mut connection)
        .await?;

    match user {
        None => {
            sqlx::query!(
                "INSERT INTO users (rowid, username, name, bio) VALUES (?, ?, ?, ?)",
                1,
                config.admin_username,
                "Default User",
                "Default Bio"
            )
            .execute(&mut connection)
            .await?;
        },
        _ => {}
    };

    if let Some(path) = &config.restore_path {
        let restore_contents = std::fs::read_to_string(path).unwrap();

        match connection.execute(restore_contents.as_str()).await {
            Ok(_) => println!("Successfully restored data"),
            Err(e) => eprintln!("Failed to restore data: {}", e)
        }
    }

    Ok(sqlite_pool)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Load application config
    let config = config::Config::from_env();

    // TODO: test that uploads path is writable

    // Test performance with disabled logging
    tide::log::start();

    // Tera template stuff
    let mut tera = Tera::new("templates/**/*.html")?;

    tera.register_filter("markdown", markdown_filter);
    tera.autoescape_on(vec!["html"]);

    // Bootstrap Database
    let sqlite_pool = bootstrap_database(&config).await?;

    // State
    let state = State {
        tera: tera,
        sqlite_pool: sqlite_pool,
        config: config.clone()
    };

    // Create Tide app and Middleware
    let mut app = tide::with_state(state);

    register_middleware(&mut app, &config);
    register_routes(&mut app, &config);

    app.listen(config.bind_host).await?;

    Ok(())
}
