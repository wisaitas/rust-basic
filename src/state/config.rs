use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_port: u16,

    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_host: String,
    pub postgres_port: u16,
    pub postgres_database: String,
    pub postgres_ssl_mode: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    envy::from_env::<Config>().unwrap()
});
