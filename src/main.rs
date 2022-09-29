use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};
use std::env;
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
    
    // Parse the response into a serde json Value struct and get the value for 
    // the "choices" key.
    let v: Value = serde_json::from_str(&res_body)?;    
    println!("{:#?}", v["choices"]);
    Ok(())
}

fn build_auth_string() -> String {
    let api_key = env::var("OPEN_API_ACCESS_KEY").unwrap();
    let mut auth_string = String::from("Bearer ");
    auth_string.push_str(&api_key);
    auth_string
}

fn build_post_payload() -> Value {
    let post_data = json!({
        "model": "text-davinci-002",
        "prompt": "Write a joke about racoons",
        "temperature": 0,
        "max_tokens": 500,
    });
    post_data
}