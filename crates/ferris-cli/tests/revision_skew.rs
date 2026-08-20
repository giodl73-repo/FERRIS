use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "ferris-revision-skew-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create test directory");
        Self(path)
    }

    fn path(&self, value: &str) -> PathBuf {
        self.0.join(value)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run(command: &mut Command) -> Output {
    let output = command.output().expect("run command");
    assert!(
        output.status.success(),
        "command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn git(repository: &Path, arguments: &[&str]) -> Output {
    run(Command::new("git")
        .arg("-C")
        .arg(repository)
        .args(arguments))
}

fn revision(repository: &Path) -> String {
    String::from_utf8(git(repository, &["rev-parse", "HEAD"]).stdout)
        .expect("revision UTF-8")
        .trim()
        .to_owned()
}

fn file_url(path: &Path) -> String {
    let value = path
        .canonicalize()
        .expect("canonical path")
        .to_string_lossy()
        .replace('\\', "/");
    let value = value.strip_prefix("//?/").unwrap_or(&value);
    if value.starts_with('/') {
        format!("file://{value}")
    } else {
        format!("file:///{value}")
    }
}

fn write_request(path: &Path, repository_url: &str, observed_revision: &str, schema: &str) {
    fs::write(
        path,
        serde_json::to_vec_pretty(&json!({
            "schema": schema,
            "analysis_id": "ferris.test/revision-skew",
            "producers": [{
                "producer_id": "ferris.test/producer",
                "repository_url": repository_url,
                "checkout_path": "producer",
                "observed_revision": observed_revision
            }],
            "consumers": [{
                "consumer_id": "ferris.test/consumer",
                "manifest_path": "consumer/Cargo.toml",
                "dependencies": [{
                    "producer_id": "ferris.test/producer",
                    "package_name": "depcrate"
                }]
            }]
        }))
        .expect("serialize request"),
    )
    .expect("write request");
}

fn setup() -> (TestDirectory, String, String, String) {
    let directory = TestDirectory::new();
    let producer = directory.path("producer");
    fs::create_dir_all(producer.join("src")).expect("create producer");
    fs::write(
        producer.join("Cargo.toml"),
        "[package]\nname = \"depcrate\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write producer manifest");
    fs::write(producer.join("src/lib.rs"), "pub const VALUE: u8 = 1;\n")
        .expect("write producer source");
    run(Command::new("git")
        .arg("init")
        .args(["--initial-branch", "main"])
        .arg(&producer));
    git(
        &producer,
        &["config", "user.email", "ferris@example.invalid"],
    );
    git(&producer, &["config", "user.name", "Ferris Test"]);
    git(&producer, &["add", "."]);
    git(&producer, &["commit", "-m", "initial"]);
    let resolved_revision = revision(&producer);

    fs::write(producer.join("src/lib.rs"), "pub const VALUE: u8 = 2;\n")
        .expect("update producer source");
    git(&producer, &["add", "."]);
    git(&producer, &["commit", "-m", "advance"]);
    let observed_revision = revision(&producer);
    let repository_url = file_url(&producer);

    let consumer = directory.path("consumer");
    fs::create_dir_all(consumer.join("src")).expect("create consumer");
    fs::write(
        consumer.join("Cargo.toml"),
        format!(
            "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ndepcrate = {{ git = \"{repository_url}\", rev = \"{resolved_revision}\" }}\n"
        ),
    )
    .expect("write consumer manifest");
    fs::write(
        consumer.join("src/lib.rs"),
        "pub fn value() -> u8 { depcrate::VALUE }\n",
    )
    .expect("write consumer source");
    run(Command::new("cargo")
        .args(["generate-lockfile", "--manifest-path"])
        .arg(consumer.join("Cargo.toml")));

    (
        directory,
        repository_url,
        resolved_revision,
        observed_revision,
    )
}

#[test]
fn reports_locked_revision_behind_observed_checkout() {
    let (directory, repository_url, resolved_revision, observed_revision) = setup();
    let request = directory.path("request.json");
    write_request(
        &request,
        &repository_url,
        &observed_revision,
        "ferris.revision-skew-request/v0",
    );

    let output = ferris()
        .arg("revision-skew")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run revision-skew");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let value: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
    assert_eq!(value["semantic_command_id"], "revision-skew");
    assert_eq!(value["record"]["schema"], "ferris.revision-skew-report/v0");
    let dependency = &value["record"]["dependencies"][0];
    assert_eq!(dependency["declaration"]["kind"], "revision");
    assert_eq!(
        dependency["declaration"]["sources"][0],
        format!("revision:{resolved_revision}")
    );
    assert_eq!(dependency["resolved_revision"], resolved_revision);
    assert_eq!(dependency["observed_revision"], observed_revision);
    assert_eq!(dependency["status"], "behind");
}

#[test]
fn rejects_observed_revision_that_is_not_checkout_head() {
    let (directory, repository_url, resolved_revision, _) = setup();
    let request = directory.path("request.json");
    write_request(
        &request,
        &repository_url,
        &resolved_revision,
        "ferris.revision-skew-request/v0",
    );

    let output = ferris()
        .arg("revision-skew")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run revision-skew");
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
    assert_eq!(value["record"]["dependencies"][0]["status"], "unavailable");
    assert!(
        value["record"]["dependencies"][0]["reasons"]
            .as_array()
            .expect("reasons")
            .iter()
            .any(|reason| reason
                .as_str()
                .expect("reason")
                .contains("does not equal the local checkout HEAD"))
    );
}

#[test]
fn rejects_dirty_producer_checkout_as_unavailable() {
    let (directory, repository_url, _, observed_revision) = setup();
    fs::write(
        directory.path("producer/src/lib.rs"),
        "pub const VALUE: u8 = 3;\n",
    )
    .expect("dirty producer source");
    let request = directory.path("request.json");
    write_request(
        &request,
        &repository_url,
        &observed_revision,
        "ferris.revision-skew-request/v0",
    );

    let output = ferris()
        .arg("revision-skew")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run revision-skew");
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
    let dependency = &value["record"]["dependencies"][0];
    assert_eq!(dependency["status"], "unavailable");
    assert!(
        dependency["reasons"]
            .as_array()
            .expect("reasons")
            .iter()
            .any(|reason| reason
                .as_str()
                .expect("reason")
                .contains("checkout is dirty"))
    );
}

#[test]
fn dirty_producer_identity_precedes_missing_lock_unknown() {
    let (directory, repository_url, _, observed_revision) = setup();
    fs::write(
        directory.path("producer/src/lib.rs"),
        "pub const VALUE: u8 = 3;\n",
    )
    .expect("dirty producer source");
    fs::remove_file(directory.path("consumer/Cargo.lock")).expect("remove lock");
    let request = directory.path("request.json");
    write_request(
        &request,
        &repository_url,
        &observed_revision,
        "ferris.revision-skew-request/v0",
    );

    let output = ferris()
        .arg("revision-skew")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run revision-skew");
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
    let dependency = &value["record"]["dependencies"][0];
    assert_eq!(dependency["status"], "unavailable");
    let reasons = dependency["reasons"].as_array().expect("reasons");
    assert!(reasons.iter().any(|reason| {
        reason
            .as_str()
            .expect("reason")
            .contains("checkout is dirty")
    }));
    assert!(reasons.iter().any(|reason| {
        reason
            .as_str()
            .expect("reason")
            .contains("lockfile was missing")
    }));
}

#[test]
fn reports_divergent_locked_and_observed_revisions() {
    let (directory, repository_url, resolved_revision, observed_revision) = setup();
    let producer = directory.path("producer");
    git(&producer, &["checkout", "--detach", &resolved_revision]);
    fs::write(producer.join("src/lib.rs"), "pub const VALUE: u8 = 4;\n")
        .expect("write divergent producer source");
    git(&producer, &["add", "."]);
    git(&producer, &["commit", "-m", "diverge"]);
    let divergent_revision = revision(&producer);

    fs::write(
        directory.path("consumer/Cargo.toml"),
        format!(
            "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ndepcrate = {{ git = \"{repository_url}\", rev = \"{divergent_revision}\" }}\n"
        ),
    )
    .expect("write divergent consumer manifest");
    fs::remove_file(directory.path("consumer/Cargo.lock")).expect("remove old lock");
    run(Command::new("cargo")
        .args(["generate-lockfile", "--manifest-path"])
        .arg(directory.path("consumer/Cargo.toml")));
    git(&producer, &["checkout", "--detach", &observed_revision]);

    let request = directory.path("request.json");
    write_request(
        &request,
        &repository_url,
        &observed_revision,
        "ferris.revision-skew-request/v0",
    );
    let output = ferris()
        .arg("revision-skew")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run revision-skew");
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
    let dependency = &value["record"]["dependencies"][0];
    assert_eq!(dependency["resolved_revision"], divergent_revision);
    assert_eq!(dependency["status"], "divergent");
}

#[test]
fn reports_duplicate_matching_lock_revisions_as_unknown() {
    let (directory, repository_url, _, observed_revision) = setup();
    let lock_path = directory.path("consumer/Cargo.lock");
    let mut lock = fs::read_to_string(&lock_path).expect("read lock");
    lock.push_str(&format!(
        "\n[[package]]\nname = \"depcrate\"\nversion = \"0.1.0\"\nsource = \"git+{repository_url}?rev={observed_revision}#{observed_revision}\"\n"
    ));
    fs::write(&lock_path, lock).expect("write duplicate lock entry");

    let request = directory.path("request.json");
    write_request(
        &request,
        &repository_url,
        &observed_revision,
        "ferris.revision-skew-request/v0",
    );
    let output = ferris()
        .arg("revision-skew")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run revision-skew");
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
    let dependency = &value["record"]["dependencies"][0];
    assert!(dependency["resolved_revision"].is_null());
    assert_eq!(dependency["status"], "unknown");
    assert!(
        dependency["reasons"]
            .as_array()
            .expect("reasons")
            .iter()
            .any(|reason| reason
                .as_str()
                .expect("reason")
                .contains("multiple revisions"))
    );
}

#[test]
fn reports_missing_lockfile_as_unknown() {
    let (directory, repository_url, _, observed_revision) = setup();
    fs::remove_file(directory.path("consumer/Cargo.lock")).expect("remove lock");
    let request = directory.path("request.json");
    write_request(
        &request,
        &repository_url,
        &observed_revision,
        "ferris.revision-skew-request/v0",
    );

    let output = ferris()
        .arg("revision-skew")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run revision-skew");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let value: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
    let dependency = &value["record"]["dependencies"][0];
    assert!(dependency["resolved_revision"].is_null());
    assert_eq!(dependency["status"], "unknown");
    assert!(
        dependency["reasons"]
            .as_array()
            .expect("reasons")
            .iter()
            .any(|reason| reason
                .as_str()
                .expect("reason")
                .contains("lockfile was missing"))
    );
}

#[test]
fn rejects_producer_path_parent_traversal() {
    let (directory, repository_url, _, observed_revision) = setup();
    let request = directory.path("request.json");
    write_request(
        &request,
        &repository_url,
        &observed_revision,
        "ferris.revision-skew-request/v0",
    );
    let mut value: Value =
        serde_json::from_slice(&fs::read(&request).expect("read request")).expect("request JSON");
    value["producers"][0]["checkout_path"] = json!("../producer");
    fs::write(
        &request,
        serde_json::to_vec_pretty(&value).expect("serialize traversal request"),
    )
    .expect("write traversal request");

    let output = ferris()
        .arg("revision-skew")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run revision-skew");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(value["result_class"], "invalid");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "FERRIS-REVISION-SKEW-PRODUCER-INVALID"
    );
}

#[test]
fn rejects_unsupported_request_schema_with_typed_error() {
    let (directory, repository_url, _, observed_revision) = setup();
    let request = directory.path("request.json");
    write_request(
        &request,
        &repository_url,
        &observed_revision,
        "ferris.revision-skew-request/v1",
    );

    let output = ferris()
        .arg("revision-skew")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run revision-skew");
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(value["semantic_command_id"], "revision-skew");
    assert_eq!(value["result_class"], "unsupported");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "FERRIS-REVISION-SKEW-SCHEMA-UNSUPPORTED"
    );
}
