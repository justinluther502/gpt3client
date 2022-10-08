use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub model: ModelConfig,
    pub user: UserConfig,
}

#[derive(Deserialize)]
pub struct ModelConfig {
    pub model: String,
    pub temperature: f32,
    pub choices: u32,
    pub include_suffix: bool,
}

#[derive(Deserialize)]
pub struct UserConfig {
    pub api_key_env_var: String,
    pub prompt_filename: String,
    pub suffix_filename: String,
}

pub fn parse_config() -> Config {
    let config_contents = fs::read_to_string("api_config.toml")
        .expect("Could not read api_config.toml. Is it missing?");
    let config: Config = toml::from_str(&config_contents).expect("Couldn't parse api_config.toml.");
    config
}
