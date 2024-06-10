use std::env;
use serde::Deserialize;
use config::{Config, File, Environment, ConfigError};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub max_connections: usize,
    pub timeout: u64,
    pub log_level: String,
    pub database_url: String,
    pub cache_size: usize,
    pub features: HashMap<String, bool>,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))?;
        s.merge(Environment::with_prefix("APP"))?;
        s.try_into()
    }

    pub fn from_file(file: &str) -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(file))?;
        s.try_into()
    }
}

pub fn get_env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn set_env_var(key: &str, value: &str) {
    env::set_var(key, value);
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    AppConfig::new()
}

pub fn load_custom_config(file: &str) -> Result<AppConfig, ConfigError> {
    AppConfig::from_file(file)
}
