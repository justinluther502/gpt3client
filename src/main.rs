use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use std::{fs, io::Write};

mod api_auth;
mod cfg_parser;
mod post_payload;
use api_auth::build_auth_string;
use post_payload::build_post_payload;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = cfg_parser::parse_config();
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
