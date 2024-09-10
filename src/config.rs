use serde_derive::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
struct Config {
    env: Env,
}

#[derive(Deserialize)]
pub struct Env {
    pub token: String,
    pub guilds: Vec<String>,
    pub _phrases: String,
}

pub fn read_config(config_file: &str) -> Env {
    let file = fs::read_to_string(config_file).unwrap();
    let config: Config = toml::from_str(&file).unwrap();

    return config.env;
}
