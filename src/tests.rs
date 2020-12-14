
use std::time::Duration;

use tide::prelude::*;
use tide::{Request, Redirect, Response, StatusCode};
use tera::Tera;
use tide_tera::prelude::*;

use sqlx::prelude::*;

use serde::Serialize;
use serde_json::Value;

use super::config::Config;
use super::State;
use super::routes;
use tide_testing::TideTestingExt;


#[async_std::test]
async fn bootstrap_test() -> std::io::Result<()> {
    // Tera template stuff
    let config = Config {
        admin_username: "testuser".to_string(),
        admin_password: "testpassword".to_string(),
        database_url: std::env::var("DATABASE_URL").unwrap().to_string(),
        session_secret: "testsessionsecrettestsessionsecrettestsessionsecret".to_string()
    };

    let mut tera = Tera::new("templates/**/*.html").unwrap();

    // tera.register_filter("markdown", markdown_filter);
    tera.autoescape_on(vec!["html"]);

    // Database stuff
    let sqlite_pool = super::bootstrap_database(&config).await.unwrap();

    let state = State {
        tera: tera,
        sqlite_pool: sqlite_pool,
        config: config.clone()
    };

    // Create Tide app and Middleware
    let mut app = tide::with_state(state);

    super::register_middleware(&mut app, &config);
    super::register_routes(&mut app);

    // Test home page
    assert_eq!(
        app.get("/").await.unwrap().status(),
        tide::http::StatusCode::Ok
    );

    // Todo: test other routes, login, post creation flow

    Ok(())
}
