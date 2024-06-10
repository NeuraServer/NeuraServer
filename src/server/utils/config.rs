use std::env;
use serde::Deserialize;
use config::{Config, File, Environment, ConfigError};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub max_connections: usize,
    pub timeout: u64,
    pub log_level: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))?;
        s.merge(Environment::with_prefix("APP"))?;
        s.try_into()
    }
}

