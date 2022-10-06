use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_derive::Deserialize;
use serde_json::{json, Value};
use std::{env, fs, io::Write};

#[derive(Deserialize)]
struct Config {
    model: ModelConfig,
    user: UserConfig,
}

#[derive(Deserialize)]
struct ModelConfig {
    model: String,
    temperature: f32,
    choices: u32,
    include_suffix: bool,
}

#[derive(Deserialize)]
struct UserConfig {
    api_key_env_var: String,
    prompt_filename: String,
    suffix_filename: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Grab config from api_config.toml
    let config_contents = fs::read_to_string("api_config.toml")
        .expect("Could not read api_config.toml. Is it missing?");
    let config: Config = toml::from_str(&config_contents)
        .expect("Couldn't parse api_config.toml.");

    // Build up the API request data
    let post_data = build_post_payload(&config.user, &config.model);
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/completions")
        .header(CONTENT_TYPE, "application/json")
        .header(
            AUTHORIZATION,
            build_auth_string(&config.user.api_key_env_var),
        )
        .json(&post_data)
        .send()
        .await?;

    // Receive the API response
    let res_body = res.text().await?;

    // Parse the response into a serde json Value struct and write the responses
    // out to the file.
    let v: Value = serde_json::from_str(&res_body)?;
    let divider = b"\n___________________________\n";
    for choice in v["choices"].as_array().unwrap() {
        let choice_obj = choice.as_object().unwrap();
        let text = choice_obj["text"].as_str().unwrap();

        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&config.user.prompt_filename)
            .unwrap();
        file.write_all(divider).unwrap();
        file.write_all(text.as_bytes()).unwrap();
    }

    // Write out a final divider and then append the suffix.
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&config.user.prompt_filename)
        .unwrap();
    file.write_all(divider).unwrap();
    let suffix = fs::read(&config.user.suffix_filename).unwrap();
    file.write_all(&suffix).unwrap();
    Ok(())
}

fn build_auth_string(key_variable: &String) -> String {
    let api_key = env::var(key_variable).unwrap();
    let mut auth_string = String::from("Bearer ");
    auth_string.push_str(&api_key);
    auth_string
}

fn build_post_payload(user_cfg: &UserConfig, model_cfg: &ModelConfig) -> Value {
    let prompt = fs::read_to_string(&user_cfg.prompt_filename).unwrap();
    let suffix = fs::read_to_string(&user_cfg.suffix_filename).unwrap();
    let used_chars = prompt.chars().count() + suffix.chars().count();
    let used_tokens = 0.3 * used_chars as f32;

    // Davinci-002 has a higher max token amount than other models.
    let max_tokens = if model_cfg.model == "text-davinci-002" {
        4093 - used_tokens as u32
    } else {
        2048 - used_tokens as u32
    };

    // Return JSON, including suffix value based on model config value.
    if model_cfg.include_suffix {
        json!({
            "model": &model_cfg.model,
            "prompt": prompt,
            "suffix": suffix,
            "temperature": &model_cfg.temperature,
            "max_tokens": max_tokens,
            "n": &model_cfg.choices,
        })
    } else {
        json!({
            "model": &model_cfg.model,
            "prompt": prompt,
            "temperature": &model_cfg.temperature,
            "max_tokens": max_tokens,
            "n": &model_cfg.choices,
        })
    }
}
