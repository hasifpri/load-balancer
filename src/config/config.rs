use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub backends: Backends,
}

#[derive(Deserialize)]
pub struct Backends {
    pub servers: Vec<String>,
}

pub fn load_config() -> Config {

    let content = fs::read_to_string("config.toml")
        .expect("con't read config");
    toml::from_str(&content)
        .expect("invalid config")
}