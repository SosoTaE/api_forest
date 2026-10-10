use figment::{
    Figment, providers::{self, Env, Format, Toml},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    #[serde(default = "default_refresh_token_ttl_days")]
    pub refresh_token_ttl_days: i64,

    pub jwt_secret: String,

    #[serde(default = "default_access_token_ttl_minutes")]
    pub access_token_ttl_minutes: i64
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_log_level() -> String {
    "api_forest=debug,tower_http=debug".to_string()
}

fn default_refresh_token_ttl_days() -> i64 {
    1
}


fn default_access_token_ttl_minutes() -> i64 {
    5
}

impl Config {

    pub fn load(file_path: &str) -> anyhow::Result<Self> {
        Ok(Figment::new()
            .merge(Toml::file(file_path))
            .merge(Env::prefixed("APP_"))
            .merge(Env::raw().only(&["DATABASE_URL"]))
            .extract()?)
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
