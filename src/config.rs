use std::env;

use dotenvy::dotenv;

pub struct DBConfig {
    pub db_url: String
}

impl Default for DBConfig {
    fn default() -> Self {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("Please Provide the database URL"));

        Self { db_url }
    }
}