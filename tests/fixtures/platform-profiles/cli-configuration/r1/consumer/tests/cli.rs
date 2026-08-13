use std::process::Command;

fn fixture() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris-profile-cli-configuration"))
}

#[test]
fn process_applies_cli_environment_default_precedence() {
    let output = fixture()
        .env("FERRIS_FIXTURE_NAME", "environment")
        .args(["--name", "cli"])
        .output()
        .expect("run fixture");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"cli\n");

    let output = fixture()
        .env("FERRIS_FIXTURE_NAME", "environment")
        .output()
        .expect("run fixture");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"environment\n");

    let output = fixture()
        .env_remove("FERRIS_FIXTURE_NAME")
        .output()
        .expect("run fixture");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"item\n");
}

#[test]
fn process_rejects_unknown_argument() {
    let output = fixture().arg("--other").output().expect("run fixture");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid configuration"));
}
