use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use std::env;

pub fn build_auth_string(key_variable: &str) -> String {
    let mut auth_string = String::from("Bearer ");
    auth_string.push_str(key_variable);
    auth_string
}

pub fn send_request(api_key_env_var: &String, post_data: &Value) -> Value {
    // Send a POST request to the API completions endpoint with a blocking
    // request.
    let client = reqwest::blocking::Client::new();
    let api_key = env::var(api_key_env_var).unwrap();
    let res = client
        .post("https://api.openai.com/v1/completions")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, build_auth_string(&api_key))
        .json(post_data)
        .send();

    // Receive the API response and parse into a JSON serde Value struct.
    let res_body = res.unwrap().text().unwrap();
    let v: Value = serde_json::from_str(&res_body).unwrap();
    v
}
