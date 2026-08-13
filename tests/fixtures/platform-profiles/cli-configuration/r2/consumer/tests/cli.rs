use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn fixture() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris-profile-cli-configuration"))
}

fn temporary_file(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferris-cli-config-{label}-{}-{nonce}.conf",
        std::process::id()
    ))
}

#[test]
fn process_applies_cli_config_environment_default_precedence() {
    let config = temporary_file("precedence");
    fs::write(&config, b"name=config\n").expect("write config");

    let output = fixture()
        .env("FERRIS_FIXTURE_NAME", "environment")
        .arg("--config")
        .arg(&config)
        .args(["--name", "cli"])
        .output()
        .expect("run fixture");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"cli\n");

    let output = fixture()
        .env("FERRIS_FIXTURE_NAME", "environment")
        .arg("--config")
        .arg(&config)
        .output()
        .expect("run fixture");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"config\n");

    fs::remove_file(config).expect("remove config");
}

#[test]
fn process_rejects_missing_malformed_oversized_and_non_utf8_config() {
    let missing = temporary_file("missing");
    let output = fixture()
        .arg("--config")
        .arg(&missing)
        .output()
        .expect("run fixture");
    assert_eq!(output.status.code(), Some(5));

    for (label, bytes) in [
        ("malformed", b"other=value\n".to_vec()),
        ("oversized", vec![b'x'; 1025]),
        ("non-utf8", vec![0xff]),
    ] {
        let config = temporary_file(label);
        fs::write(&config, bytes).expect("write config");
        let output = fixture()
            .arg("--config")
            .arg(&config)
            .output()
            .expect("run fixture");
        assert_eq!(output.status.code(), Some(2), "{label}");
        assert!(output.stdout.is_empty(), "{label}");
        fs::remove_file(config).expect("remove config");
    }
}
