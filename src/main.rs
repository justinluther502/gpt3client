mod api_request;
mod cfg_parser;
mod post_payload;
mod text_writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = cfg_parser::parse_config();
    let user = &config.user;
    let model = &config.model;
    let post_data = post_payload::post_content(&user, &model);
    let res_body = api_request::send_request(&user.api_key_env_var, &post_data);
    text_writer::write_resp_choices(
        &res_body,
        &config.user.prompt_filename,
        &config.user.suffix_filename,
    );
    Ok(())
}
