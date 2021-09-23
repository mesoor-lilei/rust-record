use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub path: String,
    pub exclude_suffix: Vec<String>,
    pub exclude_prefix: Vec<String>,
}

pub fn read_config() -> Config {
    let str = fs::read_to_string("config.toml").unwrap();
    toml::from_str(&str).unwrap()
}
