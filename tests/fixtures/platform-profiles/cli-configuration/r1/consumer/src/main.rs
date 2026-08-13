use ferris_profile_cli_configuration::resolve_name;

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let environment = std::env::var("FERRIS_FIXTURE_NAME").ok();
    match resolve_name(&arguments, environment.as_deref()) {
        Ok(name) => println!("{name}"),
        Err(error) => {
            eprintln!("invalid configuration: {error:?}");
            std::process::exit(2);
        }
    }
}
