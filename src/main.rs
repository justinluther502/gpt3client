use std::{fs, io::Write};

mod api_request;
mod cfg_parser;
mod post_payload;
use api_request::send_request;
use post_payload::build_post_payload;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = cfg_parser::parse_config();
    let post_data = build_post_payload(&config.user, &config.model);
    let resp_body = send_request(&config.user.api_key_env_var, &post_data);

    let divider = b"\n___________________________\n";
    for choice in resp_body["choices"].as_array().unwrap() {
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
