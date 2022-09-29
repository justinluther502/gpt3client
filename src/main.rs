use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};
use std::{env, fs, io::Write};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Build up the API request data
    let post_data = build_post_payload();
    let client = reqwest::Client::new();
    let res = client.post("https://api.openai.com/v1/completions")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, build_auth_string())
        .json(&post_data)
        .send()
        .await?;

    // Receive the API response    
    let res_body = res
        .text()
        .await?;
    
    // Parse the response into a serde json Value struct and write the responses
    // out to the file.
    let v: Value = serde_json::from_str(&res_body)?;
    println!("{:?}", v);
    let divider = b"\n___________________________\n";
    for choice in v["choices"].as_array().unwrap() {
        let choice_obj = choice.as_object().unwrap();
        let text = choice_obj["text"].as_str().unwrap();

        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("prompt.txt")
            .unwrap();
        file.write_all(divider).unwrap();
        file.write_all(text.as_bytes()).unwrap();
    }

    // Write out a final divider and then append the suffix.
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("prompt.txt")
        .unwrap();
    file.write_all(divider).unwrap();
    let suffix = fs::read("suffix.txt").unwrap();
    file.write_all(&suffix).unwrap();
    Ok(())
}

fn build_auth_string() -> String {
    let api_key = env::var("OPEN_API_ACCESS_KEY").unwrap();
    let mut auth_string = String::from("Bearer ");
    auth_string.push_str(&api_key);
    auth_string
}

fn build_post_payload() -> Value {
    let prompt = fs::read_to_string("prompt.txt").unwrap();
    let suffix = fs::read_to_string("suffix.txt").unwrap();
    let used_chars = prompt.chars().count() + suffix.chars().count();
    let used_tokens = 0.3 * used_chars as f32;
    let max_tokens = 4093.0 - used_tokens;
    let post_data = json!({
        "model": "text-davinci-002",
        "prompt": prompt,
        "suffix": suffix,
        "temperature": 0.8,
        "max_tokens": max_tokens as u32,
        "n": 3,
    });
    post_data
}