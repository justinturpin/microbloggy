use rand::Rng;
use async_std::task::{spawn, sleep};
use std::time::Duration;

use tide::prelude::*;
use tide::{Request, Redirect, Response, StatusCode};
use tera::Tera;
use tide_tera::prelude::*;

use sqlx::prelude::*;

use serde::Serialize;
use serde_json::Value;

mod config;
mod routes;

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

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Load application config
    let config = config::Config::from_env();

    tide::log::start();

    // Tera template stuff
    let mut tera = Tera::new("templates/**/*.html")?;

    tera.register_filter("markdown", markdown_filter);
    tera.autoescape_on(vec!["html"]);

    // Database stuff
    let sqlite_pool = sqlx::SqlitePool::connect(
        format!("sqlite:{}", config.database_path).as_str()
    ).await?;

    let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> = sqlite_pool.acquire().await?;

    // Bootstrap the schema (TODO: use sqlx migration)
    connection.execute(
        r#"CREATE TABLE IF NOT EXISTS users (
            username TEXT NOT NULL,
            name TEXT NOT NULL DEFAULT "Default User",
            bio TEXT NOT NULL DEFAULT "Default Bio"
        )"#
    ).await?;

    connection.execute(
        r#"CREATE TABLE IF NOT EXISTS posts (
            user_id INT NOT NULL,
            content TEXT NOT NULL,
            posted_timestamp TEXT NOT NULL,
            short_url TEXT
        )"#
    ).await?;

    // Bootstrap user (only 1 user for now hardcoded as user id 1)
    sqlx::query!(
        r#"REPLACE INTO users (rowid, username)
        VALUES (?1, ?2)"#,
        1,
        config.admin_username
    )
    .execute(&mut connection)
    .await?;

    // State
    let state = State {
        tera: tera,
        sqlite_pool: sqlite_pool,
        config: config.clone()
    };

    // Create Tide app and Middleware
    let mut app = tide::with_state(state);

    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        config.session_secret.as_bytes()
    ));

    app.with(tide::utils::Before(|mut request: Request<State>| async move {
        let session = request.session_mut();

        if session.get::<String>("csrf_token").is_none() {
            // TODO: use CSPRNG
            session.insert("csrf_token", format!("{}", rand::thread_rng().gen::<u64>())).unwrap();
        }

        request
    }));

    // Main Routes
    app.at("/").get(routes::index);
    app.at("/user/login").get(routes::user_login);
    app.at("/user/login").post(routes::user_login_post);
    app.at("/user/profile").get(routes::user_profile);
    app.at("/post/create").post(routes::post_create);

    // Static Files (fonts, favicon, css)
    app.at("/static").serve_dir("static")?;

    // Spawn background process
    spawn(async{
        loop {
            println!("Started background job");

            // TODO: use a clone of db connection

            sleep(Duration::from_secs(60)).await;
        }
     });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
