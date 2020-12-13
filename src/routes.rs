use super::{State, Post};

use tide_tera::prelude::*;
use sqlx::prelude::*;
use tide::{Request, Response, Redirect};
use serde::Deserialize;
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

pub async fn index(req: Request<State>) -> tide::Result<tide::Response> {
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
}

pub async fn user_login(req: Request<State>) -> tide::Result<tide::Response> {
    let mut tera = &req.state().tera;

    tera.render_response("login.html", &tera::Context::new())
}

pub async fn user_login_post(mut req: Request<State>) -> tide::Result<tide::Response> {
    let login_form: LoginFormInput = req.body_form().await?;
    let config = &req.state().config;

    // TODO: use hashing instead of plaintext comparisons
    if login_form.username == config.admin_username &&
        login_form.password == config.admin_password {
        Ok(Redirect::new("/").into())
    } else {
        Ok(Redirect::new("/user/login").into())
    }
}

pub async fn post_create(mut req: Request<State>) -> tide::Result<tide::Response> {
    let mut db_conn = (&req.state()).sqlite_pool.acquire().await?;

    let form_input: PostFormInput = req.body_form().await?;
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
