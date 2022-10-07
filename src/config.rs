use serde_derive::Deserialize;

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