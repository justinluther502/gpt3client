use crate::cfg_parser::{ModelConfig, UserConfig};
use serde_json::{json, Value};
use std::fs;

pub fn post_content(user_cfg: &UserConfig, model_cfg: &ModelConfig) -> Value {
    let prompt = fs::read_to_string(&user_cfg.prompt_filename).unwrap();
    let suffix = fs::read_to_string(&user_cfg.suffix_filename).unwrap();
    let used_chars = prompt.chars().count() + suffix.chars().count();

    // Rough estimate of how many tokens have been used up by prompt and suffix,
    // using 0.3 as an approximate value of tokens per character.
    let used_tokens = 0.3 * used_chars as f32;

    // Davinci-002 has a higher max token amount than other models.
    let max_tokens = if model_cfg.model == "text-davinci-002" {
        4093 - used_tokens as u32
    } else {
        2048 - used_tokens as u32
    };

    // Return JSON Value, possibly including suffix value based on model config.
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
