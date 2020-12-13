use super::State;

use tide_tera::prelude::*;
use sqlx::prelude::*;
use tide::{Request, Response, Redirect};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Deserialize)]
pub struct LoginFormInput {
    username: String,
    password: String,

    #[serde(rename = "csrf-token")]
    csrf_token: String,
}

#[derive(Deserialize)]
pub struct PostFormInput {
    content: String,

    #[serde(rename = "csrf-token")]
    csrf_token: String,
}

#[derive(Serialize)]
pub struct Post {
    username: String,
    name: String,
    user_id: i64,
    content: String,
    posted_timestamp: String
}

pub async fn index(req: Request<State>) -> tide::Result<tide::Response> {
    let tera = &req.state().tera;
    let session = req.session();
    let csrf_token = session.get::<String>("csrf_token").unwrap();

    let mut db_conn = (&req.state()).sqlite_pool.acquire().await?;
    let mut context = tera::Context::new();

    let result = db_conn
        .fetch_all(
            r#"SELECT users.username, users.name, users.rowid, posts.content, posts.posted_timestamp
            FROM users, posts WHERE users.rowid=posts.user_id
            ORDER BY posted_timestamp desc LIMIT 50"#
        )
        .await?;

    let mut posts = std::vec::Vec::new();

    for row in result {
        posts.push(Post{
            username: row.get(0),
            name: row.get(1),
            user_id: row.get(2),
            content: row.get(3),
            posted_timestamp: row.get(4),
        });
    }

    context.insert("posts", &posts);
    context.insert("logged_in", &session.get::<bool>("logged_in").unwrap_or(false));
    context.insert("csrf_token", &csrf_token);

    tera.render_response("index.html", &context)
}

pub async fn user_login(req: Request<State>) -> tide::Result<tide::Response> {
    let tera = &req.state().tera;
    let session = req.session();
    let csrf_token = session.get::<String>("csrf_token").unwrap();

    let mut context = tera::Context::new();
    context.insert("csrf_token", &csrf_token);

    tera.render_response("login.html", &context)
}

pub async fn user_login_post(mut req: Request<State>) -> tide::Result<tide::Response> {
    let login_form: LoginFormInput = req.body_form().await?;
    let csrf_token = req.session().get::<String>("csrf_token").unwrap();
    let config = &req.state().config.clone();

    // TODO: use hashing instead of plaintext comparisons
    if login_form.username == config.admin_username &&
        login_form.password == config.admin_password &&
        login_form.csrf_token == csrf_token {

        req.session_mut().insert("logged_in", true).unwrap();

        // Login correct, set session
        Ok(Redirect::new("/").into())
    } else {
        Ok(Redirect::new("/user/login").into())
    }
}

pub async fn post_create(mut req: Request<State>) -> tide::Result<tide::Response> {
    let session = req.session();
    let csrf_token = req.session().get::<String>("csrf_token").unwrap();

    if !session.get::<bool>("logged_in").unwrap() {
        Ok(Redirect::new("/").into())
    } else {
        let mut db_conn = (&req.state()).sqlite_pool.acquire().await?;

        let form_input: PostFormInput = req.body_form().await?;

        // Validate CSRF token
        if form_input.csrf_token != csrf_token {
            Ok(Redirect::new("/").into())
        } else {
            let now = Utc::now().to_rfc3339();

            sqlx::query!(
                "INSERT INTO posts (user_id, content, posted_timestamp) VALUES (?1, ?2, ?3)",
                1,
                form_input.content,
                now
            ).execute(&mut db_conn).await?;

            let response: Response = Redirect::new("/").into();

            Ok(response)
        }
    }
}
