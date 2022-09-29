use std::env;
use serde_json::{json, Value};

use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPEN_API_ACCESS_KEY").unwrap();
    let mut auth_string = String::from("Bearer ");
    auth_string.push_str(&api_key);

    let post_data = json!({
        "model": "text-davinci-002",
        "prompt": "Write a joke about racoons",
        "temperature": 0,
        "max_tokens": 500,
    });

    let client = reqwest::Client::new();
    let res = client.post("https://api.openai.com/v1/completions")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, auth_string)
        .json(&post_data)
        .send()
        .await?;

    let res_body = res
        .text()
        .await?;
        
    let v: Value = serde_json::from_str(&res_body)?;    
    println!("{:#?}", v["choices"]);
    Ok(())
}
