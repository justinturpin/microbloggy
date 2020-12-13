#[derive(Clone)]
pub struct Config {
    pub database_path: String,
    pub admin_username: String,
    pub admin_password: String,
    pub session_secret: String
}

impl Config {
    pub fn from_env() -> Config {
        let session_secret = std::env::var("SESSION_SECRET").expect(
            "Expected environment variable SESSION_SECRET"
        );

        if session_secret.len() < 32 {
            panic!("SESSION_SECRET must be at least 32 bytes long")
        };

        Config {
            database_path: std::env::var("DATABASE_PATH").unwrap(),
            admin_username: std::env::var("ADMIN_USERNAME").unwrap(),
            admin_password: std::env::var("ADMIN_PASSWORD").unwrap(),
            session_secret: session_secret,
        }
    }
}
