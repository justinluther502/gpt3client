mod api_request;
mod cfg_parser;
mod post_payload;
mod text_writer;
use api_request::send_request;
use post_payload::build_post_payload;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = cfg_parser::parse_config();
    let post_data = build_post_payload(&config.user, &config.model);
    let resp_body = send_request(&config.user.api_key_env_var, &post_data);
    text_writer::ai_write(
        &resp_body,
        &config.user.prompt_filename,
        &config.user.suffix_filename,
    );

    Ok(())
}
