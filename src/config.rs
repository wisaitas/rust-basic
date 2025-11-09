use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

pub static MASK_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| HashMap::new());

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenvy::dotenv().ok();

    match envy::prefixed("").from_env::<Config>() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Failed to load environment variables: {}", error);
            std::process::exit(1);
        }
    }
});

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(flatten)]
    pub server: ServerConfig,
    #[serde(flatten)]
    pub postgres: PostgresConfig,
    #[serde(flatten)]
    pub redis: RedisConfig,
    #[serde(flatten)]
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ServerConfig {
    #[serde(default = "default_env")]
    pub server_env: String,

    #[serde(default = "default_name")]
    pub server_name: String,

    #[serde(default = "default_port")]
    pub server_port: u16,

    #[serde(default = "default_max_file_size")]
    pub server_max_file_size: u32,

    #[serde(default = "default_body_limit")]
    pub server_body_limit: u32,

    #[serde(default = "default_read_timeout")]
    pub server_read_timeout: u64,

    #[serde(default = "default_write_timeout")]
    pub server_write_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PostgresConfig {
    #[serde(default = "default_postgres_host")]
    pub postgres_host: String,

    #[serde(default = "default_postgres_port")]
    pub postgres_port: String,

    #[serde(default = "default_postgres_user")]
    pub postgres_user: String,

    #[serde(default = "default_postgres_password")]
    pub postgres_password: String,

    #[serde(default = "default_postgres_database")]
    pub postgres_database: String,

    #[serde(default = "default_postgres_schema")]
    pub postgres_schema: String,

    #[serde(default = "default_postgres_ssl_mode")]
    pub postgres_ssl_mode: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RedisConfig {
    #[serde(default = "default_redis_addr")]
    pub redis_addr: String,

    #[serde(default = "default_redis_password")]
    pub redis_password: String,

    #[serde(default = "default_redis_username")]
    pub redis_username: String,

    #[serde(default = "default_redis_db")]
    pub redis_db: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct JwtConfig {
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
}

// Default values for ServerConfig
fn default_env() -> String {
    "dev".to_string()
}
fn default_name() -> String {
    "rust-101".to_string()
}
fn default_port() -> u16 {
    8080
}
fn default_max_file_size() -> u32 {
    5
}
fn default_body_limit() -> u32 {
    1024
}
fn default_read_timeout() -> u64 {
    10
}
fn default_write_timeout() -> u64 {
    10
}

// Default values for PostgresConfig
fn default_postgres_host() -> String {
    "localhost".to_string()
}
fn default_postgres_port() -> String {
    "5432".to_string()
}
fn default_postgres_user() -> String {
    "postgres".to_string()
}
fn default_postgres_password() -> String {
    "postgres".to_string()
}
fn default_postgres_database() -> String {
    "rust-db".to_string()
}
fn default_postgres_schema() -> String {
    "public".to_string()
}
fn default_postgres_ssl_mode() -> String {
    "disable".to_string()
}

// Default values for RedisConfig
fn default_redis_addr() -> String {
    "127.0.0.1:6379".to_string()
}
fn default_redis_password() -> String {
    "".to_string()
}
fn default_redis_username() -> String {
    "default".to_string()
}
fn default_redis_db() -> i32 {
    0
}

// Default values for JwtConfig
fn default_jwt_secret() -> String {
    "secret".to_string()
}

impl PostgresConfig {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database,
            self.postgres_ssl_mode
        )
    }
}
