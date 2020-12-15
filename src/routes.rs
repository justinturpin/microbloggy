use super::State;

use tide_tera::prelude::*;
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

#[derive(Deserialize)]
pub struct PostEditFormInput {
    content: String,

    #[serde(rename = "csrf-token")]
    csrf_token: String,
}

#[derive(Deserialize)]
pub struct PostDeleteFormInput {
    #[serde(rename = "csrf-token")]
    csrf_token: String,
}

#[derive(Deserialize)]
pub struct ProfileUpdateFormInput {
    name: String,
    bio: String,

    #[serde(rename = "csrf-token")]
    csrf_token: String,
}


#[derive(Serialize)]
pub struct Post {
    username: String,
    post_id: i64,
    name: String,
    user_id: i64,
    content: String,
    posted_timestamp: String
}

pub async fn index(req: Request<State>) -> tide::Result<tide::Response> {
    let tera = &req.state().tera;
    let session = req.session();

    let csrf_token = session.get::<String>("csrf_token").unwrap();
    let logged_in = session.get::<bool>("logged_in").unwrap_or(false);

    let mut db_conn = (&req.state()).sqlite_pool.acquire().await?;
    let mut context = tera::Context::new();
    let now = Utc::now().to_rfc3339();

    let before_timestamp = req
        .param("before_timestamp")
        .unwrap_or(now.as_str());

    // Query for all posts, joined on users of that post
    let result = sqlx::query!(
            r#"SELECT
                users.username, users.name, users.rowid AS user_id,
                posts.rowid AS post_id, posts.content, posts.posted_timestamp
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
            post_id: row.post_id.unwrap(),
            content: row.content,
            posted_timestamp: row.posted_timestamp,
        });
    }

    context.insert("posts", &posts);
    context.insert("logged_in", &logged_in);
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
pub async fn user_profile(req: Request<State>) -> tide::Result<Response> {
    let state: &State = req.state();
    let session = req.session();
    let tera: &tera::Tera = &state.tera;

    let logged_in = session.get::<bool>("logged_in").unwrap_or(false);
    let csrf_token = session.get::<String>("csrf_token").unwrap();

    if !logged_in {
        // TODO: anonymous users should be able to see a non-editable profile pages right?
        Ok(
            tide::Response::builder(400)
                .body("Unauthorized")
                .content_type(tide::http::mime::HTML)
                .build()
        )
    } else {
        let mut context = tera::Context::new();
        let mut db_conn = state.sqlite_pool.acquire().await?;

        let row = sqlx::query!(
                "SELECT name, username, bio FROM users WHERE rowid=1"
            )
            .fetch_one(&mut db_conn)
            .await?;

        context.insert("name" , &row.name);
        context.insert("username", &row.username);
        context.insert("bio", &row.bio);
        context.insert("csrf_token", &csrf_token);

        tera.render_response("profile.html", &context)
    }
}

/// Update user profile
pub async fn user_profile_update(mut req: Request<State>) -> tide::Result<Response> {
    let state: &State = req.state();
    let session = req.session();

    let csrf_token = session.get::<String>("csrf_token").unwrap();
    let logged_in = session.get::<bool>("logged_in").unwrap_or(false);

    if !logged_in {
        Ok(
            tide::Response::builder(400)
                .body("Unauthorized")
                .content_type(tide::http::mime::HTML)
                .build()
        )
    } else {
        let mut db_conn = state.sqlite_pool.acquire().await?;

        let form_input: ProfileUpdateFormInput = req.body_form().await?;

         // Validate CSRF
         if form_input.csrf_token != csrf_token {
            Ok(tide::Response::builder(400).body("Invalid CSRF").build())
        } else {
            sqlx::query!(
                    "UPDATE users SET name=?1, bio=?2 WHERE rowid=?3",
                    form_input.name,
                    form_input.bio,
                    1
                )
                .execute(&mut db_conn)
                .await?;

            Ok(tide::Redirect::new("/user/profile").into())
        }
    }
}

/// View a single post
pub async fn post_view(req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let session = req.session();
    let tera = &state.tera;

    let csrf_token = session.get::<String>("csrf_token").unwrap();
    let logged_in = session.get::<bool>("logged_in").unwrap_or(false);

    let mut db_conn = state.sqlite_pool.acquire().await?;
    let post_id = req.param("post_id").unwrap();

    let row = sqlx::query!(
                r#"SELECT users.username, users.name, users.rowid AS user_id,
                    posts.content, posts.posted_timestamp
                FROM users, posts
                WHERE users.rowid=posts.user_id AND posts.rowid=?"#,
            post_id)
        .fetch_one(&mut db_conn)
        .await?;

    // TODO: this will just panic if the post doesn't exist instead of 404ing

    let mut context = tera::Context::new();

    context.insert("username", &row.username);
    context.insert("name", &row.name);
    context.insert("user_id", &row.user_id.unwrap());
    context.insert("content", &row.content);
    context.insert("post_id", &post_id);
    context.insert("csrf_token", &csrf_token);
    context.insert("logged_in", &logged_in);

    tera.render_response("post.html", &context)
}

/// Handle post creation
pub async fn post_create(mut req: Request<State>) -> tide::Result<Response> {
    let session = req.session();

    let csrf_token = session.get::<String>("csrf_token").unwrap();
    let logged_in = session.get::<bool>("logged_in").unwrap_or(false);

    // Ensure logged in
    if !logged_in {
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

/// Edit a post
pub async fn post_edit(mut req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let session = req.session();

    let csrf_token = session.get::<String>("csrf_token").unwrap();
    let logged_in = session.get::<bool>("logged_in").unwrap_or(false);

    let mut db_conn = state.sqlite_pool.acquire().await?;
    let form_input: PostEditFormInput = req.body_form().await?;
    let post_id = req.param("post_id").unwrap();

    if !logged_in {
        Ok(tide::Response::builder(400).body("Forbidden").build())
    }  else {
        // Validate CSRF
        if form_input.csrf_token != csrf_token {
            Ok(tide::Response::builder(400).body("Invalid CSRF").build())
        } else {
            sqlx::query!(
                    "UPDATE posts SET content=? WHERE rowid=?",
                    form_input.content,
                    post_id
                )
                .execute(&mut db_conn)
                .await?;

            Ok(
                tide::Redirect::new(
                    format!("/post/view/{}", post_id).as_str()
                )
                .into()
            )
        }
    }
}

/// Delete a post
pub async fn post_delete(mut req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let session = req.session();

    let csrf_token = session.get::<String>("csrf_token").unwrap();
    let logged_in = session.get::<bool>("logged_in").unwrap_or(false);

    let mut db_conn = state.sqlite_pool.acquire().await?;
    let form_input: PostDeleteFormInput = req.body_form().await?;
    let post_id = req.param("post_id").unwrap();

    if !logged_in {
        Ok(tide::Response::builder(400).body("Forbidden").build())
    }  else {
        // Validate CSRF
        if form_input.csrf_token != csrf_token {
            Ok(tide::Response::builder(400).body("Invalid CSRF").build())
        } else {
            sqlx::query!(
                    "DELETE FROM posts WHERE rowid=?",
                    post_id
                )
                .execute(&mut db_conn)
                .await?;

            Ok(tide::Redirect::new("/").into())
        }
    }
}
