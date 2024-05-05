use std::env;
use std::sync::Arc;
use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub redis_url: String
}

impl Config {
    pub fn from_env() -> Arc<Self> {
        // Load environment variables from .env file
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_HOST not set in .env");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL not set in .env");

        Arc::new(Config {
            database_url,
            redis_url
        })
    }
}