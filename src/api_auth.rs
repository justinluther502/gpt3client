use std::env;

pub fn build_auth_string(key_variable: &String) -> String {
    let api_key = env::var(key_variable).unwrap();
    let mut auth_string = String::from("Bearer ");
    auth_string.push_str(&api_key);
    auth_string
}
