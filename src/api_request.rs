use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use std::env;

pub fn build_auth_string(key_variable: &String) -> String {
    let mut auth_string = String::from("Bearer ");
    auth_string.push_str(&key_variable);
    auth_string
}

pub fn send_request(api_key_env_var: &String, post_data: &Value) -> Value {
    let client = reqwest::blocking::Client::new();
    let api_key = env::var(api_key_env_var).unwrap();
    let res = client
        .post("https://api.openai.com/v1/completions")
        .header(CONTENT_TYPE, "application/json")
        .header(
            AUTHORIZATION,
            build_auth_string(&api_key),
        )
        .json(post_data)
        .send();

    // Receive the API response
    let res_body = res.unwrap().text().unwrap();

    // Parse the response into a serde json Value struct and write the responses
    // out to the file.
    let v: Value = serde_json::from_str(&res_body).unwrap();
    v
}