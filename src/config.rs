use std::env::var;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub admin_username: String,
    pub admin_password: String,
    pub session_secret: String,
    pub bind_host: String,
    pub uploads_path: PathBuf,
    pub graphicsmagick_path: PathBuf,
    pub posts_per_page: u64,
    pub restore_path: Option<PathBuf>
}

impl Config {
    pub fn from_env() -> Config {
        let session_secret = var("SESSION_SECRET").expect(
            "Expected environment variable SESSION_SECRET"
        );

        if session_secret.len() < 32 {
            panic!("SESSION_SECRET must be at least 32 bytes long")
        };

        Config {
            database_url: var("DATABASE_URL").unwrap(),
            admin_username: var("ADMIN_USERNAME").unwrap(),
            admin_password: var("ADMIN_PASSWORD").unwrap(),
            session_secret: session_secret,
            bind_host: var("BIND_HOST").unwrap_or("127.0.0.1:8080".to_string()),
            posts_per_page: match var("POSTS_PER_PAGE") {
                Ok(value) => value.parse().unwrap(),
                Err(_) => 50
            },
            uploads_path: PathBuf::from(var("UPLOADS_PATH").unwrap()),
            graphicsmagick_path: PathBuf::from(var("GRAPHICSMAGICK_PATH").unwrap_or("gm".to_string())),
            restore_path: match var("RESTORE_PATH") {
                Ok(value) => Some(PathBuf::from(value)),
                Err(_) => None
            }
        }
    }
}
