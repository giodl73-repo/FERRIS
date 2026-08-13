use ferris_profile_cli_configuration::{parse_arguments, resolve_name};

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let request = match parse_arguments(&arguments) {
        Ok(request) => request,
        Err(error) => fail(2, &format!("invalid arguments: {error:?}")),
    };
    let config = match request.config_path.as_deref() {
        Some(path) => match std::fs::read(path) {
            Ok(bytes) => Some(bytes),
            Err(error) => fail(5, &format!("configuration unavailable: {error}")),
        },
        None => None,
    };
    let environment = std::env::var("FERRIS_FIXTURE_NAME").ok();
    match resolve_name(&request, config.as_deref(), environment.as_deref()) {
        Ok(name) => println!("{name}"),
        Err(error) => fail(2, &format!("invalid configuration: {error:?}")),
    }
}

fn fail(code: i32, message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(code);
}
