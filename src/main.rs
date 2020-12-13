use sqlx::Connection;
use tide::prelude::*;
use tide::{Request, Redirect, Response, StatusCode};
use tera::Tera;
use tide_tera::prelude::*;

use sqlx::prelude::*;
use sqlx::sqlite::Sqlite;

use serde::Serialize;


struct Config {
    database_path: String,
    admin_username: String,
    admin_password: String,
}

impl Config {
    fn from_env() -> Config {
        panic!("not implemented my guy")
    }
}

#[derive(Clone)]
struct State {
    tera: Tera,
    sqlite_pool: sqlx::SqlitePool
}

#[derive(Serialize)]
struct Post {
    user_id: i64,
    content: String,
    posted_timestamp: String
}

#[derive(Deserialize)]
struct FormInput {
    csrf: String,
    content: String
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // let config = Config::from_env();

    tide::log::start();

    // Tera stuff
    let mut tera = Tera::new("templates/**/*.html")?;

    tera.autoescape_on(vec!["html"]);

    // Database stuff
    let sqlite_pool = sqlx::SqlitePool::connect("sqlite:mydb.sqlite").await?;

    let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> = sqlite_pool.acquire().await?;

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
        sqlite_pool: sqlite_pool
    };

    let mut app = tide::with_state(state);

    // Homepage
    app.at("/").get(|req: Request<State>| async move {
        let tera = &req.state().tera;
        let mut db_conn = (&req.state()).sqlite_pool.acquire().await?;
        let mut context = tera::Context::new();

        let result = db_conn
            .fetch_all("select user_id, content, posted_timestamp FROM posts ORDER BY posted_timestamp desc")
            .await?;

        let mut posts = std::vec::Vec::new();

        for row in result {
            posts.push(Post{
                user_id: row.get(0),
                content: row.get(1),
                posted_timestamp: row.get(2),
            });
        }

        context.insert("posts", &posts);

        tera.render_response("index.html", &context)
    });

    app.at("/post/create").post(|mut req: Request<State>| async move {
        let mut db_conn = (&req.state()).sqlite_pool.acquire().await?;

        let form_input: FormInput = req.body_form().await?;

        sqlx::query!(
            "INSERT INTO posts (user_id, content, posted_timestamp) VALUES (?1, ?2, ?3)",
            1,
            form_input.content,
            "right friggn now"
        ).execute(&mut db_conn).await?;

        let response: Response = Redirect::new("/").into();

        Ok(response)
    });

    // Static Files
    app.at("/static").serve_dir("static")?;

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
