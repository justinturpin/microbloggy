use rand::Rng;
use async_std::task::{spawn, sleep};
use std::time::Duration;

use tide::prelude::*;
use tide::{Request, Redirect, Response, StatusCode};
use tera::Tera;
use tide_tera::prelude::*;

use sqlx::prelude::*;
use sqlx::{Sqlite, SqlitePool};
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};

use serde::Serialize;
use serde_json::Value;
use std::str::FromStr;

mod config;
mod routes;
mod tests;

#[derive(Clone)]
pub struct State {
    tera: Tera,
    sqlite_pool: sqlx::SqlitePool,
    config: config::Config
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

        request
    }));
}

fn register_routes(app: &mut tide::Server<State>) {
    // Main Routes
    app.at("/").get(routes::index);
    app.at("/user/login").get(routes::user_login);
    app.at("/user/login").post(routes::user_login_post);
    app.at("/user/profile").get(routes::user_profile);
    app.at("/post/create").post(routes::post_create);

    // Static Files (fonts, favicon, css)
    app.at("/static").serve_dir("static").unwrap();
}

async fn bootstrap_database(config: &config::Config) -> tide::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(config.database_url.as_str())?
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true);

    let sqlite_pool = SqlitePool::connect_with(options).await?;

    // Run migrations, bringing the database schema up-to-date
    sqlx::migrate!("./migrations")
        .run(&sqlite_pool)
        .await?;

    let mut connection: PoolConnection<Sqlite> = sqlite_pool.acquire().await?;

    // Bootstrap user (only 1 user for now hardcoded as user id 1)
    sqlx::query!(
            "REPLACE INTO users (rowid, username) VALUES (?1, ?2)",
            1,
            config.admin_username
        )
        .execute(&mut connection)
        .await?;

    Ok(sqlite_pool)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Load application config
    let config = config::Config::from_env();

    // Test performance with disabled logging
    // tide::log::start();

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
    register_routes(&mut app);

    app.listen(config.bind_host).await?;

    Ok(())
}
