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
    let now = Utc::now().to_rfc3339();

    let before_timestamp = req
        .param("before_timestamp")
        .unwrap_or(now.as_str());

    // Query for all posts, joined on users of that post
    let result = sqlx::query!(
            r#"SELECT users.username, users.name, users.rowid AS user_id, posts.content, posts.posted_timestamp
            FROM users, posts
            WHERE users.rowid=posts.user_id AND posts.posted_timestamp <= ?
            ORDER BY posted_timestamp desc LIMIT 50"#,
            before_timestamp
        )
        .fetch_all(&mut db_conn)
        .await?;

    // TODO: I think you can collect all of this into a Vec of some struct
    let mut posts = std::vec::Vec::new();

    for row in result {
        posts.push(Post{
            username: row.username,
            name: row.name,
            user_id: row.user_id.unwrap(),
            content: row.content,
            posted_timestamp: row.posted_timestamp,
        });
    }

    context.insert("posts", &posts);
    context.insert("logged_in", &session.get::<bool>("logged_in").unwrap_or(false));
    context.insert("csrf_token", &csrf_token);

    tera.render_response("index.html", &context)
}

/// Show user login form
pub async fn user_login(req: Request<State>) -> tide::Result<tide::Response> {
    let tera = &req.state().tera;
    let session = req.session();
    let csrf_token = session.get::<String>("csrf_token").unwrap();

    let mut context = tera::Context::new();
    context.insert("csrf_token", &csrf_token);

    tera.render_response("login.html", &context)
}

/// Handle user login attempt
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
        // TODO: show a nice error on failure
        Ok(Redirect::new("/user/login").into())
    }
}

/// User profile
pub async fn user_profile(mut req: Request<State>) -> tide::Result<Response> {
    let state: &State = req.state();
    let session = req.session();
    let tera: &tera::Tera = &state.tera;
    let config = &state.config;

    if !session.get::<bool>("logged_in").unwrap_or(false) {
        Ok(
            tide::Response::builder(400)
                .body("Unauthorized")
                .content_type(tide::http::mime::HTML)
                .build()
        )
    } else {
        let mut context = tera::Context::new();

        context.insert("name", "Default User");
        context.insert("username", &config.admin_username);
        context.insert("bio", "Default Bio");

        tera.render_response("profile.html", &context)
    }
}

/// Handle post creation
pub async fn post_create(mut req: Request<State>) -> tide::Result<Response> {
    let session = req.session();
    let csrf_token = req.session().get::<String>("csrf_token").unwrap();

    // Ensure logged in
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

            // TODO: hardcoded user id of 1 should be dynamic, probably
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
