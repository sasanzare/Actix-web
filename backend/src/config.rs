use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
                .to_owned(),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8000".to_owned())
                .parse()
                .expect("Failed to parse SERVER_PORT"),
        }
    }
}