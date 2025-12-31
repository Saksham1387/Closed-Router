use diesel::{Connection, ConnectionError, PgConnection};

use crate::config::DBConfig;

pub struct Store {
    pub conn: PgConnection
}

impl Store {
    pub fn new() -> Result<Self,ConnectionError> {
        let config = DBConfig::default();
        let conn = PgConnection::establish(&config.db_url)?;
        Ok(Self { conn })
    }
}