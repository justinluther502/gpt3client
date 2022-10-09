use serde_json::Value;
use std::{fs, io::Write};

pub fn write_resp_choices(json_body: &Value, prompt: &String, suffix: &String) {
    let divider = b"\n___________________________\n";

    // Write out the choices returned in the JSON response body, separated by
    // a line divider.
    for choice in json_body["choices"].as_array().unwrap() {
        let choice_obj = choice.as_object().unwrap();
        let text = choice_obj["text"].as_str().unwrap();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(prompt)
            .unwrap();
        file.write_all(divider).unwrap();
        file.write_all(text.as_bytes()).unwrap();
    }

    // Write out a final divider and then append the suffix.
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(prompt)
        .unwrap();
    file.write_all(divider).unwrap();
    let suffix = fs::read(suffix).unwrap();
    file.write_all(&suffix).unwrap();
    
}
