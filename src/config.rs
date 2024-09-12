use serde_derive::Deserialize;
use std::fs;

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

pub fn read_config(config_file: &str) -> Result<Env, anyhow::Error> {
    let file = fs::read_to_string(config_file)?;
    let config: Config = toml::from_str(&file)?;

    Ok(config.env)
}
