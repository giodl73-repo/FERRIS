use ferris_core::{
    OWNER_ENTRYPOINTS_SCHEMA, OWNER_EXECUTABLE_STAGING_RECEIPT_SCHEMA, OwnerEntrypointDeclaration,
    OwnerExecutableStagingReceipt, file_content_identity,
};
use serde_json::{Value, json};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

const RECEIPT_SCHEMA_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/schemas/owner-executable-staging/ferris.owner-executable-staging-receipt.v1.schema.json"
);

struct TestDirectory {
    base: PathBuf,
    root: PathBuf,
}

impl TestDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let base = std::env::temp_dir().join(format!(
            "ferris-owner-executable-staging-{}-{nonce}",
            std::process::id()
        ));
        let root = base.join("repository");
        fs::create_dir_all(root.join(".ferris/runtime/bin")).expect("create staging parent");
        Self { base, root }
    }

    fn source(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.base.join(name);
        fs::write(&path, bytes).expect("write source executable");
        #[cfg(unix)]
        fs::set_permissions(&path, fs::Permissions::from_mode(0o750))
            .expect("make source executable");
        path
    }

    fn stage(&self, source: &Path, destination: &str, format: &str) -> Output {
        ferris()
            .current_dir(&self.root)
            .args([
                "stage-owner-executable",
                "--source",
                source.to_str().expect("UTF-8 test source"),
                "--destination",
                destination,
                "--format",
                format,
            ])
            .output()
            .expect("stage owner executable")
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}

fn diagnostic_code(output: &Output) -> String {
    assert!(!output.status.success(), "unexpected success: {output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    let value: Value = serde_json::from_slice(&output.stderr).expect("error envelope");
    assert_eq!(value["semantic_command_id"], "stage-owner-executable");
    value["diagnostics"][0]["code"]
        .as_str()
        .expect("diagnostic code")
        .to_owned()
}

#[test]
fn checked_in_schema_describes_the_staging_receipt() {
    let schema: Value = serde_json::from_slice(
        &fs::read(RECEIPT_SCHEMA_PATH).expect("read executable staging receipt schema"),
    )
    .expect("parse executable staging receipt schema");
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(
        schema["properties"]["schema"]["const"],
        OWNER_EXECUTABLE_STAGING_RECEIPT_SCHEMA
    );
    assert_eq!(
        schema["required"],
        json!([
            "schema",
            "staging_id",
            "destination",
            "content_identity",
            "byte_length",
            "source_path_retained"
        ])
    );
    assert_eq!(schema["properties"]["byte_length"]["maximum"], 536870912);
    assert_eq!(schema["properties"]["source_path_retained"]["const"], false);
}

fn run_git(root: &Path, arguments: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(arguments)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn stages_exact_bytes_and_emits_path_private_receipts() {
    let directory = TestDirectory::new();
    let bytes = b"#!/bin/sh\nexit 0\n";
    let source = directory.source("selected-owner-tool", bytes);
    let destination = ".ferris/runtime/bin/owner-tool";

    let output = directory.stage(&source, destination, "json");
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let receipt: OwnerExecutableStagingReceipt =
        serde_json::from_slice(&output.stdout).expect("staging receipt");
    assert_eq!(receipt.schema, OWNER_EXECUTABLE_STAGING_RECEIPT_SCHEMA);
    assert!(receipt.staging_id.starts_with("owner-executable-stage:"));
    assert_eq!(receipt.destination, destination);
    assert_eq!(receipt.byte_length, bytes.len() as u64);
    assert!(!receipt.source_path_retained);
    assert_eq!(fs::read(directory.root.join(destination)).unwrap(), bytes);
    assert_eq!(
        receipt.content_identity,
        file_content_identity(&directory.root.join(destination)).expect("destination identity")
    );
    assert!(!String::from_utf8_lossy(&output.stdout).contains(&source.to_string_lossy()[..]));

    let repeated = directory.stage(&source, destination, "json");
    assert!(repeated.status.success(), "{repeated:?}");
    assert_eq!(repeated.stdout, output.stdout);

    let human_destination = ".ferris/runtime/bin/owner-tool-human";
    let human = directory.stage(&source, human_destination, "human");
    assert!(human.status.success(), "{human:?}");
    let human_text = String::from_utf8(human.stdout).expect("human output UTF-8");
    assert!(human_text.contains(human_destination));
    assert!(human_text.contains("Source path retained: no"));
    assert!(!human_text.contains(&source.to_string_lossy()[..]));

    #[cfg(unix)]
    assert_eq!(
        fs::metadata(directory.root.join(destination))
            .expect("destination metadata")
            .mode()
            & 0o777,
        0o750
    );
}

#[test]
fn refuses_overwrite_and_leaves_existing_destination_unchanged() {
    let directory = TestDirectory::new();
    let source = directory.source("selected-owner-tool", b"new bytes");
    let destination = ".ferris/runtime/bin/owner-tool";
    fs::write(directory.root.join(destination), b"existing bytes").expect("existing destination");

    let output = directory.stage(&source, destination, "json");
    assert_eq!(
        diagnostic_code(&output),
        "FERRIS-EXECUTABLE-STAGING-DESTINATION-EXISTS"
    );
    assert_eq!(
        fs::read(directory.root.join(destination)).unwrap(),
        b"existing bytes"
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains(&source.to_string_lossy()[..]));
}

#[cfg(unix)]
#[test]
fn refuses_reuse_when_existing_destination_permissions_differ() {
    let directory = TestDirectory::new();
    let bytes = b"#!/bin/sh\nexit 0\n";
    let source = directory.source("selected-owner-tool", bytes);
    let destination = ".ferris/runtime/bin/owner-tool";
    fs::write(directory.root.join(destination), bytes).expect("existing destination");
    fs::set_permissions(
        directory.root.join(destination),
        fs::Permissions::from_mode(0o755),
    )
    .expect("set different destination permissions");

    let output = directory.stage(&source, destination, "json");
    assert_eq!(
        diagnostic_code(&output),
        "FERRIS-EXECUTABLE-STAGING-DESTINATION-EXISTS"
    );
    assert_eq!(
        fs::metadata(directory.root.join(destination))
            .expect("destination metadata")
            .mode()
            & 0o777,
        0o755
    );
}

#[test]
fn rejects_non_portable_escaped_and_unprepared_destinations() {
    let directory = TestDirectory::new();
    let source = directory.source("selected-owner-tool", b"owner bytes");
    for destination in [
        "../owner-tool",
        ".ferris\\runtime\\bin\\owner-tool",
        ".ferris//runtime/bin/owner-tool",
        ".ferris/runtime/bin/",
        ".ferris/runtime/./owner-tool",
        "C:owner-tool",
        ".ferris/runtime/bin:owner-tool",
        ".",
    ] {
        let output = directory.stage(&source, destination, "json");
        assert_eq!(
            diagnostic_code(&output),
            "FERRIS-EXECUTABLE-STAGING-DESTINATION-INVALID",
            "{destination}"
        );
    }

    let missing_parent = directory.stage(&source, "missing/bin/owner-tool", "json");
    assert_eq!(
        diagnostic_code(&missing_parent),
        "FERRIS-EXECUTABLE-STAGING-DESTINATION-PARENT-INVALID"
    );
}

#[test]
fn rejects_unavailable_empty_directory_and_oversized_sources_without_residue() {
    let directory = TestDirectory::new();
    let destination = ".ferris/runtime/bin/owner-tool";

    let missing = directory.stage(&directory.base.join("missing"), destination, "json");
    assert_eq!(
        diagnostic_code(&missing),
        "FERRIS-EXECUTABLE-STAGING-SOURCE-UNAVAILABLE"
    );

    let source_directory = directory.base.join("source-directory");
    fs::create_dir(&source_directory).expect("create source directory");
    let directory_output = directory.stage(&source_directory, destination, "json");
    assert_eq!(
        diagnostic_code(&directory_output),
        "FERRIS-EXECUTABLE-STAGING-SOURCE-INVALID"
    );

    let empty = directory.source("empty", b"");
    let empty_output = directory.stage(&empty, destination, "json");
    assert_eq!(
        diagnostic_code(&empty_output),
        "FERRIS-EXECUTABLE-STAGING-SOURCE-BOUND-INVALID"
    );

    let oversized = directory.base.join("oversized");
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&oversized)
        .expect("create oversized source");
    file.set_len(512 * 1024 * 1024 + 1)
        .expect("size oversized source");
    drop(file);
    #[cfg(unix)]
    fs::set_permissions(&oversized, fs::Permissions::from_mode(0o750))
        .expect("make oversized source executable");
    let oversized_output = directory.stage(&oversized, destination, "json");
    assert_eq!(
        diagnostic_code(&oversized_output),
        "FERRIS-EXECUTABLE-STAGING-SOURCE-BOUND-INVALID"
    );

    assert!(!directory.root.join(destination).exists());
    let residue = fs::read_dir(directory.root.join(".ferris/runtime/bin"))
        .expect("read staging directory")
        .collect::<Result<Vec<_>, _>>()
        .expect("staging entries");
    assert!(
        residue.is_empty(),
        "unexpected staging residue: {residue:?}"
    );
}

#[test]
fn competing_stages_publish_exactly_one_destination() {
    let directory = TestDirectory::new();
    let first_source = directory.source("first-owner-tool", b"first owner bytes");
    let second_source = directory.source("second-owner-tool", b"second owner bytes");
    let destination = ".ferris/runtime/bin/owner-tool";

    let first = ferris()
        .current_dir(&directory.root)
        .args([
            "stage-owner-executable",
            "--source",
            first_source.to_str().unwrap(),
            "--destination",
            destination,
            "--format",
            "json",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn first staging");
    let second = ferris()
        .current_dir(&directory.root)
        .args([
            "stage-owner-executable",
            "--source",
            second_source.to_str().unwrap(),
            "--destination",
            destination,
            "--format",
            "json",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn second staging");
    let first_output = first.wait_with_output().expect("wait first staging");
    let second_output = second.wait_with_output().expect("wait second staging");
    assert_ne!(
        first_output.status.success(),
        second_output.status.success()
    );

    let bytes = fs::read(directory.root.join(destination)).expect("published destination");
    assert!(bytes == b"first owner bytes" || bytes == b"second owner bytes");
    let residue = fs::read_dir(directory.root.join(".ferris/runtime/bin"))
        .expect("read staging directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"))
        .count();
    assert_eq!(residue, 0);
}

#[test]
fn staged_executable_is_consumed_by_owner_entrypoint_binding() {
    let directory = TestDirectory::new();
    let source = directory.source("selected-owner-tool", b"#!/bin/sh\nexit 0\n");
    let destination = ".ferris/runtime/bin/owner-tool";
    let staged = directory.stage(&source, destination, "json");
    assert!(staged.status.success(), "{staged:?}");
    let receipt: OwnerExecutableStagingReceipt =
        serde_json::from_slice(&staged.stdout).expect("staging receipt");

    fs::write(directory.root.join("config.txt"), b"owner config\n").expect("write config");
    fs::write(
        directory.root.join("intents.json"),
        serde_json::to_vec_pretty(&json!({
            "schema": "ferris.owner-entrypoint-intents/v1",
            "entrypoints": [{
                "entrypoint_id": "owner/check",
                "owner": "owner/build-platform",
                "executable": destination,
                "argv": ["--check"],
                "working_directory": ".",
                "inherited_environment": [],
                "credential_class": "none",
                "bound_files": [destination, "config.txt"]
            }]
        }))
        .expect("serialize intents"),
    )
    .expect("write intents");
    run_git(&directory.root, &["init", "--quiet"]);
    run_git(
        &directory.root,
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(&directory.root, &["config", "user.name", "Ferris Test"]);
    run_git(&directory.root, &["add", "."]);
    run_git(
        &directory.root,
        &[
            "-c",
            "commit.gpgsign=false",
            "commit",
            "--quiet",
            "-m",
            "fixture",
        ],
    );

    let bound = ferris()
        .current_dir(&directory.root)
        .args([
            "bind-owner-entrypoints",
            "--intents",
            "intents.json",
            "--output",
            "entrypoints.json",
            "--format",
            "json",
        ])
        .output()
        .expect("bind staged executable");
    assert!(bound.status.success(), "{bound:?}");
    let declaration: OwnerEntrypointDeclaration =
        serde_json::from_slice(&bound.stdout).expect("entrypoint declaration");
    assert_eq!(declaration.schema, OWNER_ENTRYPOINTS_SCHEMA);
    assert_eq!(
        declaration.entrypoints[0].command.files[0].identity,
        receipt.content_identity
    );
}
