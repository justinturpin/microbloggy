use async_std::task::{spawn, sleep};
use std::time::Duration;

use tide::prelude::*;
use tide::{Request, Redirect, Response, StatusCode};
use tera::Tera;
use tide_tera::prelude::*;

use sqlx::prelude::*;

use serde::Serialize;

mod config;
mod routes;

#[derive(Clone)]
pub struct State {
    tera: Tera,
    sqlite_pool: sqlx::SqlitePool,
    config: config::Config
}

#[derive(Serialize)]
pub struct Post {
    user_id: i64,
    content: String,
    posted_timestamp: String
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Load application config
    let config = config::Config::from_env();

    tide::log::start();

    // Tera stuff
    let mut tera = Tera::new("templates/**/*.html")?;

    tera.autoescape_on(vec!["html"]);

    // Database stuff
    let sqlite_pool = sqlx::SqlitePool::connect(
        format!("sqlite:{}", config.database_path).as_str()
    ).await?;

    let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> = sqlite_pool.acquire().await?;

    // Bootstrap the schema (TODO: use sqlx migration)
    connection.execute(
        r#"CREATE TABLE IF NOT EXISTS users
        (username TEXT NOT NULL, name TEXT NOT NULL, bio TEXT NOT NULL)"#
    ).await?;

    connection.execute(
        r#"CREATE TABLE IF NOT EXISTS posts
        (user_id INT NOT NULL, content TEXT NOT NULL, posted_timestamp TEXT NOT NULL)"#
    ).await?;

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

    // Main Routes
    app.at("/").get(routes::index);
    app.at("/user/login").get(routes::user_login);
    app.at("/user/login").post(routes::user_login_post);
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
