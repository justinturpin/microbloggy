#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub admin_username: String,
    pub admin_password: String,
    pub session_secret: String,
    pub bind_host: String
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
            database_url: std::env::var("DATABASE_URL").unwrap(),
            admin_username: std::env::var("ADMIN_USERNAME").unwrap(),
            admin_password: std::env::var("ADMIN_PASSWORD").unwrap(),
            session_secret: session_secret,
            bind_host: std::env::var("BIND_HOST").unwrap_or("127.0.0.1:8080".to_string())
        }
    }
}
