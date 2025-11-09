pub mod domain;
pub mod state;

use diesel::Connection;
use diesel::pg::PgConnection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostgresConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub ssl_mode: String,
}

pub fn establish_connection(config: PostgresConfig) -> PgConnection {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode={}",
        config.user, config.password, config.host, config.port, config.database, config.ssl_mode
    );

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("failed to connect to database: {}", database_url))
}
