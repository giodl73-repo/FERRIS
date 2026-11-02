use serde::Deserialize;
use std::io::{self, Read};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    before: String,
    after: String,
}
fn main() {
    let mut text = String::new();
    // Accommodates JSON escaping of two bounded 1 MiB input strings.
    if let Err(error) = io::stdin()
        .take(12 * 1024 * 1024 + 1024)
        .read_to_string(&mut text)
    {
        eprintln!("UTF-8 request: {error}");
        std::process::exit(2);
    }
    match serde_json::from_str::<Request>(&text) {
        Ok(request) => println!(
            "{}",
            ferris_profile_inspector::compare_profiles(&request.before, &request.after)
        ),
        Err(error) => {
            eprintln!("Request JSON: {error}");
            std::process::exit(2);
        }
    }
}
