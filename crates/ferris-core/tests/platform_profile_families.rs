use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const PLATFORM_PROFILE_SCHEMA: &str = "ferris.platform-profile/v1";
const FAMILY_SCHEMA: &str = "ferris.platform-profile-family/v1";

#[derive(Debug, Deserialize)]
struct FamilyManifest {
    schema: String,
    family: String,
    base: String,
    revisions: Vec<Revision>,
}

#[derive(Debug, Deserialize)]
struct Revision {
    revision: String,
    package_version: String,
    consumer_manifest: String,
    internal_whitespace: String,
    expected_source_digest: String,
    expected_profile_digest: String,
}

#[derive(Debug, Deserialize)]
struct CliFamilyManifest {
    schema: String,
    family: String,
    base: String,
    revisions: Vec<CliRevision>,
}

#[derive(Debug, Deserialize)]
struct CliRevision {
    revision: String,
    package_version: String,
    consumer_manifest: String,
    config_file: String,
    expected_source_digest: String,
    expected_profile_digest: String,
}

#[derive(Debug, Deserialize)]
struct HostedFamilyManifest {
    schema: String,
    family: String,
    base: String,
    revisions: Vec<HostedRevision>,
}

#[derive(Debug, Deserialize)]
struct HostedRevision {
    revision: String,
    package_version: String,
    consumer_manifest: String,
    readiness: String,
    expected_source_digest: String,
    expected_profile_digest: String,
}

#[derive(Debug, Deserialize)]
struct EmbeddedFamilyManifest {
    schema: String,
    family: String,
    base: String,
    revisions: Vec<EmbeddedRevision>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedRevision {
    revision: String,
    package_version: String,
    consumer_manifest: String,
    frame_contract: String,
    expected_source_digest: String,
    expected_profile_digest: String,
}

#[derive(Debug, Deserialize)]
struct BrowserWasmFamilyManifest {
    schema: String,
    family: String,
    base: String,
    revisions: Vec<BrowserWasmRevision>,
}

#[derive(Debug, Deserialize)]
struct BrowserWasmRevision {
    revision: String,
    package_version: String,
    consumer_manifest: String,
    accessibility: String,
    expected_source_digest: String,
    expected_profile_digest: String,
}

#[derive(Debug, Deserialize)]
struct ComponentFamilyManifest {
    schema: String,
    family: String,
    base: String,
    revisions: Vec<ComponentRevision>,
}

#[derive(Debug, Deserialize)]
struct ComponentRevision {
    revision: String,
    package_version: String,
    consumer_manifest: String,
    wit_contract: String,
    expected_source_digest: String,
    expected_profile_digest: String,
}

#[derive(Debug, Deserialize)]
struct NativeFamilyManifest {
    schema: String,
    family: String,
    base: String,
    revisions: Vec<NativeRevision>,
}

#[derive(Debug, Deserialize)]
struct NativeRevision {
    revision: String,
    package_version: String,
    consumer_manifest: String,
    native_boundary: String,
    expected_source_digest: String,
    expected_profile_digest: String,
}

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "ferris-platform-profile-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create isolated test directory");
        Self { path }
    }

    fn child(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/platform-profiles/pure-data")
}

fn directory_snapshot(root: &Path) -> BTreeMap<String, Option<Vec<u8>>> {
    fn collect(root: &Path, directory: &Path, snapshot: &mut BTreeMap<String, Option<Vec<u8>>>) {
        for entry in fs::read_dir(directory).expect("read snapshot directory") {
            let entry = entry.expect("read snapshot entry");
            let path = entry.path();
            let relative = path
                .strip_prefix(root)
                .expect("entry below snapshot root")
                .to_string_lossy()
                .replace('\\', "/");
            let file_type = entry.file_type().expect("read snapshot file type");
            if file_type.is_dir() {
                snapshot.insert(format!("{relative}/"), None);
                collect(root, &path, snapshot);
            } else {
                snapshot.insert(relative, Some(fs::read(&path).expect("read snapshot file")));
            }
        }
    }

    let mut snapshot = BTreeMap::new();
    collect(root, root, &mut snapshot);
    snapshot
}

fn framed_tree_digest(root: &Path) -> String {
    let snapshot = directory_snapshot(root);
    let mut hasher = Sha256::new();
    hasher.update(b"ferris.fixture-tree/v1");
    hasher.update([0]);
    for (path, contents) in snapshot {
        hasher.update(path.len().to_string().as_bytes());
        hasher.update([0]);
        hasher.update(path.as_bytes());
        hasher.update([0]);
        match contents {
            Some(bytes) => {
                hasher.update(b"file");
                hasher.update([0]);
                hasher.update(bytes.len().to_string().as_bytes());
                hasher.update([0]);
                hasher.update(bytes);
            }
            None => {
                hasher.update(b"directory");
                hasher.update([0]);
            }
        }
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn canonical_profile_digest(profile: &Value) -> String {
    let bytes = serde_json::to_vec(profile).expect("serialize materialized profile");
    let mut hasher = Sha256::new();
    hasher.update(PLATFORM_PROFILE_SCHEMA.as_bytes());
    hasher.update([0]);
    hasher.update(bytes.len().to_string().as_bytes());
    hasher.update([0]);
    hasher.update(bytes);
    format!("sha256:{:x}", hasher.finalize())
}

fn set(profile: &mut Value, pointer: &str, value: Value) {
    *profile.pointer_mut(pointer).expect("profile pointer") = value;
}

fn source(kind: &str, identity: &str, owner: &str, revision: &str, path: &str) -> Value {
    json!({
        "kind": kind,
        "identity": identity,
        "owner": owner,
        "revision": revision,
        "observed_at": "2026-08-12T00:00:00Z",
        "path": path
    })
}

fn tool(id: &str, name: &str, version: &str, owner: &str, path: &str) -> Value {
    json!({
        "id": id,
        "name": name,
        "version": version,
        "owner": owner,
        "state": "pass",
        "source": source("command", &format!("{id}.source"), owner, version, path)
    })
}

fn stage(
    revision: &Revision,
    kind: &str,
    state: &str,
    claim_class: &str,
    argv: &[&str],
    capabilities: &[&str],
    diagnostic: Option<&str>,
) -> Value {
    let mut evidence = json!({
        "id": format!("evidence.pure-data.{}.{}", revision.revision, kind),
        "state": state,
        "claim_class": claim_class,
        "owner": if state == "pass" { "cargo" } else { "fixture.consumer" },
        "subject": format!("Pure-data {kind} stage"),
        "scope": format!("Controlled pure-data {}", revision.revision),
        "observed_at": "2026-08-12T00:00:00Z",
        "expires_at": "2026-11-10T00:00:00Z",
        "source": source(
            if state == "pass" { "command" } else { "human-decision" },
            &format!("fixture.pure-data.{}.{}", revision.revision, kind),
            if state == "pass" { "cargo" } else { "fixture.consumer" },
            "1.95.0",
            if state == "pass" {
                "owner command result"
            } else {
                "controlled family requirements"
            }
        ),
        "limitations": []
    });
    if let Some(diagnostic) = diagnostic {
        evidence.as_object_mut().expect("evidence object").insert(
            "diagnostic".to_owned(),
            Value::String(diagnostic.to_owned()),
        );
    }

    json!({
        "id": format!("stage.pure-data.{}.{}", revision.revision, kind),
        "kind": kind,
        "state": state,
        "owner": if state == "pass" { "cargo" } else { "fixture.consumer" },
        "scope": format!("Controlled pure-data {}", revision.revision),
        "command": {
            "argv": argv,
            "working_directory": format!(
                "tests/fixtures/platform-profiles/pure-data/{}/consumer",
                revision.revision
            ),
            "environment": {
                "CARGO_NET_OFFLINE": "true",
                "RUSTUP_AUTO_INSTALL": "0"
            },
            "network": "disabled",
            "target_directory": format!("external/pure-data-{}-{kind}", revision.revision)
        },
        "evidence": evidence,
        "capabilities": capabilities,
        "limitations": []
    })
}

fn materialize_profile(base: &Value, revision: &Revision, source_digest: &str) -> Value {
    let mut profile = base.clone();
    let package_id = format!("ferris-profile-pure-data@{}", revision.package_version);
    let source_path = format!(
        "tests/fixtures/platform-profiles/pure-data/{}/consumer",
        revision.revision
    );

    set(&mut profile, "/revision", json!(revision.revision));
    set(
        &mut profile,
        "/operation/success_criteria",
        json!([
            "ASCII keys are trimmed and lowercased",
            if revision.internal_whitespace == "pass" {
                "Internal ASCII whitespace is collapsed to one hyphen"
            } else {
                "Internal ASCII whitespace is rejected"
            },
            "Empty and non-ASCII keys are rejected"
        ]),
    );
    set(&mut profile, "/selection/0/package_id", json!(package_id));
    set(
        &mut profile,
        "/selection/0/name",
        json!("ferris-profile-pure-data"),
    );
    set(
        &mut profile,
        "/selection/0/version",
        json!(revision.package_version),
    );
    set(
        &mut profile,
        "/selection/0/source",
        json!({
            "kind": "file",
            "identity": format!("fixture.source.pure-data.{}", revision.revision),
            "owner": "fixture.owner",
            "revision": revision.revision,
            "observed_at": "2026-08-12T00:00:00Z",
            "digest": source_digest,
            "path": source_path
        }),
    );

    for closure in profile["closures"].as_array_mut().expect("closures array") {
        closure["target"] = json!("controlled-windows-and-unix");
        closure["packages"][0]["package_id"] = json!(package_id);
        closure["source"] = source(
            "command",
            &format!(
                "fixture.command.pure-data.{}.{}",
                revision.revision,
                closure["kind"].as_str().expect("closure kind")
            ),
            "cargo",
            "1.95.0",
            "cargo metadata --locked --offline",
        );
    }
    profile["features"]["requested"][0]["package_id"] = json!(package_id);
    profile["features"]["effective"][0]["package_id"] = json!(package_id);
    profile["features"]["source"] = source(
        "command",
        &format!("fixture.command.pure-data.{}.features", revision.revision),
        "cargo",
        "1.95.0",
        "cargo metadata --locked --offline",
    );
    profile["contracts"][0]["version"] = json!(revision.revision);
    profile["contracts"][0]["scope"] = json!(if revision.internal_whitespace == "pass" {
        "Normalize ASCII record keys including internal whitespace"
    } else {
        "Normalize ASCII record keys without internal whitespace"
    });

    profile["environment"] = json!({
        "cargo": tool("tool.cargo", "cargo", "1.95.0", "rust-project", "cargo --version"),
        "rustc": tool("tool.rustc", "rustc", "1.95.0", "rust-project", "rustc --version --verbose"),
        "toolchain": tool("tool.toolchain", "rust", "1.95.0", "rust-project", "rustup show active-toolchain"),
        "host": tool(
            "host.controlled-matrix",
            "windows-and-unix",
            "2026-08-12",
            "fixture.owner",
            "platform validation receipt"
        ),
        "targets": [
            tool(
                "target.windows-msvc",
                "x86_64-pc-windows-msvc",
                "1.95.0",
                "rust-project",
                "rustc --version --verbose"
            ),
            tool(
                "target.linux-gnu",
                "x86_64-unknown-linux-gnu",
                "1.95.0",
                "rust-project",
                "rustc --version --verbose"
            )
        ],
        "components": [],
        "native_tools": [],
        "providers": [],
        "runtimes": [],
        "filesystem": "Windows checkout with isolated Windows and WSL target directories",
        "network": "disabled"
    });

    profile["stages"] = Value::Array(vec![
        stage(
            revision,
            "resolve",
            "pass",
            "directly-observed",
            &["cargo", "metadata", "--locked", "--offline"],
            &["Exact lock resolution"],
            None,
        ),
        stage(
            revision,
            "check",
            "pass",
            "directly-observed",
            &["cargo", "check", "--locked", "--offline"],
            &["Host-target type checking"],
            None,
        ),
        stage(
            revision,
            "lint",
            "pass",
            "directly-observed",
            &[
                "cargo",
                "clippy",
                "--locked",
                "--offline",
                "--",
                "-D",
                "warnings",
            ],
            &["Clippy warnings denied"],
            None,
        ),
        stage(
            revision,
            "build",
            "pass",
            "directly-observed",
            &["cargo", "build", "--locked", "--offline"],
            &["Host-target library build"],
            None,
        ),
        stage(
            revision,
            "link",
            "pass",
            "directly-observed",
            &["cargo", "build", "--locked", "--offline"],
            &["Host-target Rust library artifact"],
            Some("This is not native ABI evidence"),
        ),
        stage(
            revision,
            "execute",
            "unsupported",
            "owner-declared",
            &["owner-stage", "execute"],
            &[],
            Some("The controlled library has no standalone executable"),
        ),
        stage(
            revision,
            "unit-test",
            "pass",
            "directly-observed",
            &["cargo", "test", "--lib", "--locked", "--offline"],
            &["Positive and rejection behavior"],
            None,
        ),
        stage(
            revision,
            "integration-test",
            "unsupported",
            "owner-declared",
            &["owner-stage", "integration-test"],
            &[],
            Some("The controlled library has no external integration boundary"),
        ),
        stage(
            revision,
            "doctest",
            "pass",
            "directly-observed",
            &["cargo", "test", "--doc", "--locked", "--offline"],
            &["Documented owner example"],
            None,
        ),
        stage(
            revision,
            "contract-conformance",
            "pass",
            "directly-observed",
            &["cargo", "test", "--lib", "--locked", "--offline"],
            &["Revision-specific whitespace and rejection contract"],
            None,
        ),
        stage(
            revision,
            "package",
            "pass",
            "directly-observed",
            &["cargo", "package", "--locked", "--offline", "--no-verify"],
            &["Cargo package construction"],
            Some("Package installation is not observed"),
        ),
        stage(
            revision,
            "sign-attest",
            "not-observed",
            "owner-declared",
            &["owner-stage", "sign-attest"],
            &[],
            Some("No signing or attestation owner is configured"),
        ),
        stage(
            revision,
            "deploy",
            "unsupported",
            "owner-declared",
            &["owner-stage", "deploy"],
            &[],
            Some("The controlled pure-data library has no deployment operation"),
        ),
        stage(
            revision,
            "operational-validation",
            "unsupported",
            "owner-declared",
            &["owner-stage", "operational-validation"],
            &[],
            Some("The controlled pure-data library has no service operation"),
        ),
        stage(
            revision,
            "rollback",
            "not-observed",
            "owner-declared",
            &["owner-stage", "rollback"],
            &[],
            Some("Exact rollback is reserved for the renewal pulse"),
        ),
    ]);

    profile["assurance"] = json!([
        {
            "id": format!("assurance.pure-data.{}.safe-rust", revision.revision),
            "state": "pass",
            "claim_class": "directly-observed",
            "owner": "fixture.owner",
            "subject": "Controlled fixture source",
            "scope": "No unsafe block or dependency",
            "observed_at": "2026-08-12T00:00:00Z",
            "expires_at": "2026-11-10T00:00:00Z",
            "source": source(
                "file",
                &format!("fixture.source.pure-data.{}", revision.revision),
                "fixture.owner",
                &revision.revision,
                &source_path
            ),
            "diagnostic": "Safe Rust source is not a general soundness or security proof",
            "limitations": []
        }
    ]);
    profile["stewardship"] = json!([
        {
            "id": format!("stewardship.pure-data.{}", revision.revision),
            "state": "pass",
            "claim_class": "owner-declared",
            "owner": "fixture.owner",
            "subject": "Controlled fixture maintenance",
            "scope": "Repository-local conformance fixture",
            "observed_at": "2026-08-12T00:00:00Z",
            "expires_at": "2026-11-10T00:00:00Z",
            "source": source(
                "policy",
                "fixture.stewardship-policy",
                "fixture.owner",
                &revision.revision,
                "controlled family record"
            ),
            "limitations": []
        }
    ]);
    profile["support"][0]["scope"] = json!("Controlled pure-data fixture maintenance only");
    profile["support"][0]["source"]["revision"] = json!(revision.revision);

    for control in [
        "adoption",
        "renewal",
        "substitution",
        "emergency",
        "rollback",
        "removal",
    ] {
        profile["lifecycle"][control]["procedure_ref"] = json!(format!(
            "procedure.pure-data.{}.{}",
            revision.revision, control
        ));
    }
    profile["lifecycle"]["rollback"]["exact_restore_identity"] = json!(source_digest);
    profile["limitations"] = json!([
        {
            "id": "limit.controlled-only",
            "scope": "Entire profile",
            "description": "Controlled zero-dependency pure-data family",
            "consequence": "It cannot establish another family or ecosystem support",
            "expires_at": "2026-11-10T00:00:00Z"
        },
        {
            "id": "limit.lifecycle-planned",
            "scope": "Lifecycle controls",
            "description": "Renewal, substitution, emergency, rollback, and removal are planned but not executed",
            "consequence": "PLATFORM-001 lifecycle gates remain open",
            "expires_at": "2026-11-10T00:00:00Z"
        }
    ]);

    profile
}

fn validate_materialized_profile(profile: &Value, revision: &Revision, source_digest: &str) {
    assert_eq!(profile["schema"], PLATFORM_PROFILE_SCHEMA);
    assert_eq!(profile["family"], "pure-data");
    assert_eq!(profile["revision"], revision.revision);
    assert_eq!(profile["selection"][0]["source"]["digest"], source_digest);
    assert_eq!(profile["selection"][0]["version"], revision.package_version);

    let stages = profile["stages"].as_array().expect("stage array");
    assert_eq!(stages.len(), 15);
    let kinds = stages
        .iter()
        .map(|stage| stage["kind"].as_str().expect("stage kind"))
        .collect::<BTreeSet<_>>();
    assert_eq!(kinds.len(), 15);
    assert!(stages.iter().any(|stage| stage["state"] == "pass"));
    assert!(stages.iter().any(|stage| stage["state"] == "unsupported"));
    assert!(stages.iter().any(|stage| stage["state"] == "not-observed"));
}

fn cargo_command(manifest: &Path, target_directory: &Path, arguments: &[&str]) -> Output {
    let mut command = Command::new(option_env!("CARGO").unwrap_or("cargo"));
    command
        .current_dir(manifest.parent().expect("manifest directory"))
        .env("CARGO_NET_OFFLINE", "true")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .env("CARGO_TARGET_DIR", target_directory);
    if let Some(separator) = arguments.iter().position(|argument| *argument == "--") {
        command
            .args(&arguments[..separator])
            .arg("--manifest-path")
            .arg(manifest)
            .args(&arguments[separator..]);
    } else {
        command.args(arguments).arg("--manifest-path").arg(manifest);
    }
    command.output().expect("run owner Cargo command")
}

fn require_success(label: &str, output: Output) -> Output {
    assert!(
        output.status.success(),
        "{label} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn pure_data_family_preserves_owner_workflows_and_exact_profiles() {
    let root = fixture_root();
    let manifest: FamilyManifest = serde_json::from_slice(
        &fs::read(root.join("family.json")).expect("read pure-data family manifest"),
    )
    .expect("parse pure-data family manifest");
    assert_eq!(manifest.schema, FAMILY_SCHEMA);
    assert_eq!(manifest.family, "pure-data");
    assert_eq!(manifest.revisions.len(), 2);

    let base: Value =
        serde_json::from_slice(&fs::read(root.join(&manifest.base)).expect("read base profile"))
            .expect("parse base profile");
    let temporary = TestDirectory::new("pure-data");
    let mut measured = Vec::new();

    for revision in &manifest.revisions {
        let manifest_path = root.join(&revision.consumer_manifest);
        let consumer = manifest_path.parent().expect("consumer directory");
        let baseline = directory_snapshot(consumer);
        let source_digest = framed_tree_digest(consumer);

        let metadata = require_success(
            "metadata",
            cargo_command(
                &manifest_path,
                &temporary.child(&format!("{}-metadata", revision.revision)),
                &[
                    "metadata",
                    "--format-version",
                    "1",
                    "--no-deps",
                    "--locked",
                    "--offline",
                ],
            ),
        );
        let metadata: Value =
            serde_json::from_slice(&metadata.stdout).expect("parse Cargo metadata");
        assert_eq!(metadata["packages"].as_array().expect("packages").len(), 1);
        assert_eq!(metadata["packages"][0]["name"], "ferris-profile-pure-data");
        assert_eq!(metadata["packages"][0]["version"], revision.package_version);
        assert_eq!(directory_snapshot(consumer), baseline);

        let commands: [(&str, &[&str]); 6] = [
            ("check", &["check", "--locked", "--offline"]),
            ("build", &["build", "--locked", "--offline"]),
            (
                "clippy",
                &["clippy", "--locked", "--offline", "--", "-D", "warnings"],
            ),
            ("test", &["test", "--lib", "--locked", "--offline"]),
            ("doctest", &["test", "--doc", "--locked", "--offline"]),
            (
                "package",
                &[
                    "package",
                    "--locked",
                    "--offline",
                    "--allow-dirty",
                    "--no-verify",
                ],
            ),
        ];
        for (label, arguments) in commands {
            require_success(
                label,
                cargo_command(
                    &manifest_path,
                    &temporary.child(&format!("{}-{label}", revision.revision)),
                    arguments,
                ),
            );
            assert_eq!(
                directory_snapshot(consumer),
                baseline,
                "{label} changed {}",
                revision.revision
            );
        }

        let profile = materialize_profile(&base, revision, &source_digest);
        validate_materialized_profile(&profile, revision, &source_digest);
        measured.push((revision, source_digest, canonical_profile_digest(&profile)));
    }

    assert_ne!(measured[0].2, measured[1].2);
    for (revision, source_digest, profile_digest) in measured {
        println!(
            "{} source={} profile={}",
            revision.revision, source_digest, profile_digest
        );
        assert_eq!(source_digest, revision.expected_source_digest);
        assert_eq!(profile_digest, revision.expected_profile_digest);
    }
}

fn replace_family_strings(value: &mut Value, replacement: &str) {
    match value {
        Value::Array(values) => values
            .iter_mut()
            .for_each(|value| replace_family_strings(value, replacement)),
        Value::Object(values) => values
            .values_mut()
            .for_each(|value| replace_family_strings(value, replacement)),
        Value::String(text) => {
            *text = text.replace("pure-data", replacement);
        }
        _ => {}
    }
}

fn materialize_cli_profile(base: &Value, revision: &CliRevision, source_digest: &str) -> Value {
    let template = Revision {
        revision: revision.revision.clone(),
        package_version: revision.package_version.clone(),
        consumer_manifest: revision.consumer_manifest.clone(),
        internal_whitespace: "expected-rejection".to_owned(),
        expected_source_digest: String::new(),
        expected_profile_digest: String::new(),
    };
    let mut profile = materialize_profile(base, &template, source_digest);
    replace_family_strings(&mut profile, "cli-configuration");
    profile["profile_id"] = json!("fixture.cli-configuration");
    profile["family"] = json!("cli-configuration");
    profile["consumer"]["name"] = json!("Controlled CLI and configuration consumer");
    profile["operation"] = json!({
        "id": "fixture.resolve-name",
        "name": "Resolve one CLI configuration name",
        "subject": "Explicit CLI arguments, one owner environment variable, and optional explicit config bytes",
        "success_criteria": if revision.config_file == "pass" {
            json!([
                "CLI name overrides explicit config, environment, and default",
                "Explicit config overrides environment and default",
                "Missing, malformed, oversized, and non-UTF-8 config fails explicitly"
            ])
        } else {
            json!([
                "CLI name overrides environment and default",
                "Unknown or incomplete arguments fail explicitly",
                "Configuration files are unsupported"
            ])
        },
        "non_goals": [
            "Implicit configuration discovery",
            "Credential handling",
            "Installation or deployment"
        ]
    });
    profile["contracts"][0]["id"] = json!("fixture.contract.cli-configuration");
    profile["contracts"][0]["namespace"] = json!("fixture.cli-configuration");
    profile["contracts"][0]["scope"] = json!(if revision.config_file == "pass" {
        "CLI, explicit bounded config file, environment, and default precedence"
    } else {
        "CLI, environment, and default precedence"
    });
    profile["contracts"][0]["version"] = json!(revision.revision);
    profile["environment"]["filesystem"] = json!(if revision.config_file == "pass" {
        "Only one explicit configuration path; maximum 1 KiB; Windows and Unix process tests"
    } else {
        "No configuration file access; Windows and Unix process tests"
    });

    let integration = profile["stages"]
        .as_array_mut()
        .expect("stage array")
        .iter_mut()
        .find(|stage| stage["kind"] == "integration-test")
        .expect("integration stage");
    integration["state"] = json!("pass");
    integration["owner"] = json!("cargo");
    integration["command"]["argv"] =
        json!(["cargo", "test", "--all-targets", "--locked", "--offline"]);
    integration["evidence"]["state"] = json!("pass");
    integration["evidence"]["claim_class"] = json!("directly-observed");
    integration["evidence"]["owner"] = json!("cargo");
    integration["evidence"]
        .as_object_mut()
        .expect("evidence object")
        .remove("diagnostic");
    integration["capabilities"] = json!([
        "Process exit and stream behavior",
        if revision.config_file == "pass" {
            "Explicit configuration file bounds and precedence"
        } else {
            "CLI and environment precedence"
        }
    ]);
    profile["stages"]
        .as_array_mut()
        .expect("stage array")
        .iter_mut()
        .filter(|stage| {
            matches!(
                stage["kind"].as_str(),
                Some("unit-test" | "contract-conformance")
            )
        })
        .for_each(|stage| {
            stage["command"]["argv"] =
                json!(["cargo", "test", "--all-targets", "--locked", "--offline"]);
        });
    profile["limitations"] = json!([
        {
            "id": "limit.controlled-cli-only",
            "scope": "Entire profile",
            "description": "Controlled zero-dependency CLI/configuration family",
            "consequence": "It cannot establish another family, installation, deployment, or ecosystem support",
            "expires_at": "2026-11-10T00:00:00Z"
        },
        {
            "id": "limit.config-revision",
            "scope": "Configuration behavior",
            "description": if revision.config_file == "pass" {
                "Only one explicit local file with name=<value> and a 1 KiB bound is supported"
            } else {
                "Configuration files are unsupported"
            },
            "consequence": "No implicit search, merge, secret, or remote configuration claim",
            "expires_at": "2026-11-10T00:00:00Z"
        }
    ]);
    profile
}

#[test]
fn cli_configuration_family_preserves_process_and_owner_workflows() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/platform-profiles/cli-configuration");
    let manifest: CliFamilyManifest = serde_json::from_slice(
        &fs::read(root.join("family.json")).expect("read CLI family manifest"),
    )
    .expect("parse CLI family manifest");
    assert_eq!(manifest.schema, FAMILY_SCHEMA);
    assert_eq!(manifest.family, "cli-configuration");
    assert_eq!(manifest.revisions.len(), 2);
    let base: Value =
        serde_json::from_slice(&fs::read(root.join(&manifest.base)).expect("read base profile"))
            .expect("parse base profile");
    let temporary = TestDirectory::new("cli-configuration");
    let mut measured = Vec::new();

    for revision in &manifest.revisions {
        let manifest_path = root.join(&revision.consumer_manifest);
        let consumer = manifest_path.parent().expect("consumer directory");
        let baseline = directory_snapshot(consumer);
        let source_digest = framed_tree_digest(consumer);
        let metadata = require_success(
            "metadata",
            cargo_command(
                &manifest_path,
                &temporary.child(&format!("{}-metadata", revision.revision)),
                &[
                    "metadata",
                    "--format-version",
                    "1",
                    "--no-deps",
                    "--locked",
                    "--offline",
                ],
            ),
        );
        let metadata: Value =
            serde_json::from_slice(&metadata.stdout).expect("parse Cargo metadata");
        assert_eq!(metadata["packages"].as_array().expect("packages").len(), 1);
        assert_eq!(
            metadata["packages"][0]["name"],
            "ferris-profile-cli-configuration"
        );
        assert_eq!(metadata["packages"][0]["version"], revision.package_version);
        assert_eq!(directory_snapshot(consumer), baseline);

        let commands: [(&str, &[&str]); 6] = [
            ("check", &["check", "--locked", "--offline"]),
            ("build", &["build", "--locked", "--offline"]),
            (
                "clippy",
                &[
                    "clippy",
                    "--all-targets",
                    "--locked",
                    "--offline",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            ("test", &["test", "--all-targets", "--locked", "--offline"]),
            ("doctest", &["test", "--doc", "--locked", "--offline"]),
            (
                "package",
                &[
                    "package",
                    "--locked",
                    "--offline",
                    "--allow-dirty",
                    "--no-verify",
                ],
            ),
        ];
        for (label, arguments) in commands {
            require_success(
                label,
                cargo_command(
                    &manifest_path,
                    &temporary.child(&format!("{}-{label}", revision.revision)),
                    arguments,
                ),
            );
            assert_eq!(
                directory_snapshot(consumer),
                baseline,
                "{label} changed {}",
                revision.revision
            );
        }

        let profile = materialize_cli_profile(&base, revision, &source_digest);
        assert_eq!(profile["schema"], PLATFORM_PROFILE_SCHEMA);
        assert_eq!(profile["family"], "cli-configuration");
        assert_eq!(profile["revision"], revision.revision);
        assert_eq!(profile["selection"][0]["source"]["digest"], source_digest);
        assert_eq!(profile["stages"].as_array().expect("stages").len(), 15);
        measured.push((revision, source_digest, canonical_profile_digest(&profile)));
    }

    assert_ne!(measured[0].2, measured[1].2);
    for (revision, source_digest, profile_digest) in measured {
        println!(
            "{} source={} profile={}",
            revision.revision, source_digest, profile_digest
        );
        assert_eq!(source_digest, revision.expected_source_digest);
        assert_eq!(profile_digest, revision.expected_profile_digest);
    }
}

fn materialize_hosted_profile(
    base: &Value,
    revision: &HostedRevision,
    source_digest: &str,
) -> Value {
    let template = Revision {
        revision: revision.revision.clone(),
        package_version: revision.package_version.clone(),
        consumer_manifest: revision.consumer_manifest.clone(),
        internal_whitespace: "expected-rejection".to_owned(),
        expected_source_digest: String::new(),
        expected_profile_digest: String::new(),
    };
    let mut profile = materialize_profile(base, &template, source_digest);
    replace_family_strings(&mut profile, "hosted-service");
    profile["profile_id"] = json!("fixture.hosted-service");
    profile["family"] = json!("hosted-service");
    profile["consumer"]["name"] = json!("Controlled hosted-service consumer");
    profile["operation"] = json!({
        "id": "fixture.service-health",
        "name": "Handle in-process health and readiness requests",
        "subject": "One controlled in-process request",
        "success_criteria": if revision.readiness == "unsupported" {
            json!(["Health returns 200", "Malformed and cancelled requests fail explicitly", "Readiness is unsupported"])
        } else {
            json!(["Health returns 200", "Readiness remains unavailable before owner transition", "Ready transition returns 200"])
        },
        "non_goals": ["Network listener", "Database", "TLS", "Deployment"]
    });
    profile["contracts"][0]["id"] = json!("fixture.contract.hosted-service");
    profile["contracts"][0]["namespace"] = json!("fixture.hosted-service");
    profile["contracts"][0]["version"] = json!(revision.revision);
    profile["contracts"][0]["scope"] = json!(if revision.readiness == "unsupported" {
        "In-process health request with cancellation and malformed rejection"
    } else {
        "In-process health and readiness requests with explicit unavailable state"
    });
    profile["environment"]["runtimes"] = json!([tool(
        "runtime.in-process",
        "std-process",
        "1.95.0",
        "rust-project",
        "owner unit test"
    )]);
    profile["environment"]["network"] = json!("disabled");
    for kind in ["execute", "integration-test", "operational-validation"] {
        let stage = profile["stages"]
            .as_array_mut()
            .expect("stages")
            .iter_mut()
            .find(|stage| stage["kind"] == kind)
            .expect("hosted stage");
        stage["state"] = json!("pass");
        stage["owner"] = json!("cargo");
        stage["command"]["argv"] = json!(["cargo", "test", "--lib", "--locked", "--offline"]);
        stage["evidence"]["state"] = json!("pass");
        stage["evidence"]["claim_class"] = json!("directly-observed");
        stage["evidence"]["owner"] = json!("cargo");
        stage["evidence"]
            .as_object_mut()
            .expect("evidence")
            .remove("diagnostic");
        stage["capabilities"] = json!([if kind == "operational-validation" {
            "In-process health and readiness behavior only"
        } else {
            "In-process request behavior"
        }]);
    }
    profile["limitations"] = json!([
        {
            "id": "limit.in-process-only",
            "scope": "Entire profile",
            "description": "Controlled in-process hosted-service family with network disabled",
            "consequence": "No listener, wire, TLS, database, deployment, or production operations claim",
            "expires_at": "2026-11-10T00:00:00Z"
        },
        {
            "id": "limit.readiness-revision",
            "scope": "Readiness",
            "description": if revision.readiness == "unsupported" {
                "Readiness is unsupported"
            } else {
                "Readiness is unavailable until an explicit owner transition"
            },
            "consequence": "Unavailable must not be promoted to pass",
            "expires_at": "2026-11-10T00:00:00Z"
        }
    ]);
    profile
}

#[test]
fn hosted_service_family_preserves_runtime_states_and_owner_workflows() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/platform-profiles/hosted-service");
    let manifest: HostedFamilyManifest =
        serde_json::from_slice(&fs::read(root.join("family.json")).expect("read hosted manifest"))
            .expect("parse hosted manifest");
    assert_eq!(manifest.schema, FAMILY_SCHEMA);
    assert_eq!(manifest.family, "hosted-service");
    let base: Value =
        serde_json::from_slice(&fs::read(root.join(&manifest.base)).expect("read base profile"))
            .expect("parse base profile");
    let temporary = TestDirectory::new("hosted-service");
    let mut measured = Vec::new();
    for revision in &manifest.revisions {
        let manifest_path = root.join(&revision.consumer_manifest);
        let consumer = manifest_path.parent().expect("consumer");
        let baseline = directory_snapshot(consumer);
        let source_digest = framed_tree_digest(consumer);
        let commands: [(&str, &[&str]); 7] = [
            (
                "metadata",
                &[
                    "metadata",
                    "--format-version",
                    "1",
                    "--no-deps",
                    "--locked",
                    "--offline",
                ],
            ),
            ("check", &["check", "--locked", "--offline"]),
            ("build", &["build", "--locked", "--offline"]),
            (
                "clippy",
                &[
                    "clippy",
                    "--all-targets",
                    "--locked",
                    "--offline",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            ("test", &["test", "--lib", "--locked", "--offline"]),
            ("doctest", &["test", "--doc", "--locked", "--offline"]),
            (
                "package",
                &[
                    "package",
                    "--locked",
                    "--offline",
                    "--allow-dirty",
                    "--no-verify",
                ],
            ),
        ];
        for (label, arguments) in commands {
            require_success(
                label,
                cargo_command(
                    &manifest_path,
                    &temporary.child(&format!("{}-{label}", revision.revision)),
                    arguments,
                ),
            );
            assert_eq!(directory_snapshot(consumer), baseline);
        }
        let profile = materialize_hosted_profile(&base, revision, &source_digest);
        assert_eq!(profile["schema"], PLATFORM_PROFILE_SCHEMA);
        assert_eq!(profile["family"], "hosted-service");
        assert_eq!(profile["stages"].as_array().expect("stages").len(), 15);
        measured.push((revision, source_digest, canonical_profile_digest(&profile)));
    }
    assert_ne!(measured[0].2, measured[1].2);
    for (revision, source_digest, profile_digest) in measured {
        println!(
            "{} source={} profile={}",
            revision.revision, source_digest, profile_digest
        );
        assert_eq!(source_digest, revision.expected_source_digest);
        assert_eq!(profile_digest, revision.expected_profile_digest);
    }
}

fn materialize_embedded_profile(
    base: &Value,
    revision: &EmbeddedRevision,
    source_digest: &str,
) -> Value {
    let template = Revision {
        revision: revision.revision.clone(),
        package_version: revision.package_version.clone(),
        consumer_manifest: revision.consumer_manifest.clone(),
        internal_whitespace: "expected-rejection".to_owned(),
        expected_source_digest: String::new(),
        expected_profile_digest: String::new(),
    };
    let mut profile = materialize_profile(base, &template, source_digest);
    replace_family_strings(&mut profile, "embedded-no-std");
    profile["profile_id"] = json!("fixture.embedded-no-std");
    profile["family"] = json!("embedded-no-std");
    profile["consumer"]["name"] = json!("Controlled embedded no-std consumer");
    profile["operation"] = json!({
        "id": "fixture.encode-sensor-frame",
        "name": "Encode one bounded sensor frame",
        "subject": "Caller-provided fixed storage",
        "success_criteria": if revision.frame_contract == "reading-v1" {
            json!([
                "Encode one 12-bit reading into exactly four bytes",
                "Reject an out-of-range reading before mutation",
                "Reject an undersized output before mutation"
            ])
        } else {
            json!([
                "Encode one 12-bit reading and four status bits into exactly six bytes",
                "Append the deterministic XOR checksum",
                "Reject invalid flags, readings, and storage before mutation"
            ])
        },
        "non_goals": ["Board support", "Device I/O", "Allocator", "Firmware deployment"]
    });
    profile["contracts"][0]["id"] = json!("fixture.contract.embedded-no-std");
    profile["contracts"][0]["namespace"] = json!("fixture.embedded-no-std");
    profile["contracts"][0]["version"] = json!(revision.revision);
    profile["contracts"][0]["scope"] = json!(revision.frame_contract);
    for closure in profile["closures"].as_array_mut().expect("closures") {
        closure["target"] = json!("thumbv7em-none-eabi");
    }
    profile["environment"]["targets"] = json!([tool(
        "target.thumbv7em-none-eabi",
        "thumbv7em-none-eabi",
        "1.95.0",
        "rust-project",
        "rustup target list --installed"
    )]);
    profile["environment"]["components"] = json!([]);
    profile["environment"]["runtimes"] = json!([]);
    profile["environment"]["native_tools"] = json!([]);
    profile["environment"]["filesystem"] =
        json!("Caller-provided fixed storage; isolated host and target directories");
    profile["environment"]["network"] = json!("disabled");

    for kind in ["check", "lint", "build", "link"] {
        let stage = profile["stages"]
            .as_array_mut()
            .expect("stages")
            .iter_mut()
            .find(|stage| stage["kind"] == kind)
            .expect("embedded target stage");
        stage["command"]["argv"] = if kind == "lint" {
            json!([
                "cargo",
                "clippy",
                "--target",
                "thumbv7em-none-eabi",
                "--lib",
                "--locked",
                "--offline",
                "--",
                "-D",
                "warnings"
            ])
        } else {
            json!([
                "cargo",
                if kind == "check" { "check" } else { "build" },
                "--target",
                "thumbv7em-none-eabi",
                "--lib",
                "--locked",
                "--offline"
            ])
        };
        stage["capabilities"] = json!(["Exact no-std target compilation"]);
        if kind == "link" {
            stage["evidence"]["diagnostic"] =
                json!("Rust library artifact only; no firmware image or linker script");
        }
    }
    let execute = profile["stages"]
        .as_array_mut()
        .expect("stages")
        .iter_mut()
        .find(|stage| stage["kind"] == "execute")
        .expect("execute stage");
    execute["state"] = json!("unavailable");
    execute["evidence"]["state"] = json!("unavailable");
    execute["evidence"]["diagnostic"] =
        json!("No target runner, emulator, board, or device is configured");
    execute["capabilities"] = json!([]);

    for kind in ["unit-test", "doctest", "contract-conformance"] {
        let stage = profile["stages"]
            .as_array_mut()
            .expect("stages")
            .iter_mut()
            .find(|stage| stage["kind"] == kind)
            .expect("host test stage");
        stage["command"]["argv"] = if kind == "doctest" {
            json!(["cargo", "test", "--doc", "--locked", "--offline"])
        } else {
            json!(["cargo", "test", "--lib", "--locked", "--offline"])
        };
        stage["capabilities"] = json!(["Host execution of frame contract"]);
    }
    let operational = profile["stages"]
        .as_array_mut()
        .expect("stages")
        .iter_mut()
        .find(|stage| stage["kind"] == "operational-validation")
        .expect("operational stage");
    operational["state"] = json!("unavailable");
    operational["evidence"]["state"] = json!("unavailable");
    operational["evidence"]["diagnostic"] =
        json!("No physical device or target runner is configured");

    profile["assurance"] = json!([
        {
            "id": format!("assurance.embedded-no-std.{}.safe-rust", revision.revision),
            "state": "pass",
            "claim_class": "directly-observed",
            "owner": "fixture.owner",
            "subject": "Controlled no-std fixture source",
            "scope": "No unsafe block, allocator, build script, or dependency",
            "observed_at": "2026-08-12T00:00:00Z",
            "expires_at": "2026-11-10T00:00:00Z",
            "source": source(
                "file",
                &format!("fixture.source.embedded-no-std.{}", revision.revision),
                "fixture.owner",
                &revision.revision,
                &format!(
                    "tests/fixtures/platform-profiles/embedded-no-std/{}/consumer",
                    revision.revision
                )
            ),
            "diagnostic": "Safe no-std source and target compilation are not device safety proof",
            "limitations": []
        }
    ]);
    profile["limitations"] = json!([
        {
            "id": "limit.no-device",
            "scope": "Target execution",
            "description": "No board, runner, emulator, device I/O, linker script, or firmware image is configured",
            "consequence": "Execution and operational validation remain unavailable",
            "expires_at": "2026-11-10T00:00:00Z"
        },
        {
            "id": "limit.controlled-embedded-only",
            "scope": "Entire profile",
            "description": "Controlled core-only no-std library family",
            "consequence": "It cannot establish embedded ecosystem or hardware support",
            "expires_at": "2026-11-10T00:00:00Z"
        }
    ]);
    profile
}

#[test]
fn embedded_no_std_family_preserves_target_and_owner_workflows() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/platform-profiles/embedded-no-std");
    let manifest: EmbeddedFamilyManifest = serde_json::from_slice(
        &fs::read(root.join("family.json")).expect("read embedded manifest"),
    )
    .expect("parse embedded manifest");
    assert_eq!(manifest.schema, FAMILY_SCHEMA);
    assert_eq!(manifest.family, "embedded-no-std");
    let base: Value =
        serde_json::from_slice(&fs::read(root.join(&manifest.base)).expect("read base profile"))
            .expect("parse base profile");
    let temporary = TestDirectory::new("embedded-no-std");
    let mut measured = Vec::new();
    for revision in &manifest.revisions {
        let manifest_path = root.join(&revision.consumer_manifest);
        let consumer = manifest_path.parent().expect("consumer");
        let baseline = directory_snapshot(consumer);
        let source_digest = framed_tree_digest(consumer);
        let commands: [(&str, &[&str]); 7] = [
            (
                "metadata",
                &[
                    "metadata",
                    "--format-version",
                    "1",
                    "--no-deps",
                    "--locked",
                    "--offline",
                ],
            ),
            (
                "target-check",
                &[
                    "check",
                    "--target",
                    "thumbv7em-none-eabi",
                    "--lib",
                    "--locked",
                    "--offline",
                ],
            ),
            (
                "target-build",
                &[
                    "build",
                    "--target",
                    "thumbv7em-none-eabi",
                    "--lib",
                    "--locked",
                    "--offline",
                ],
            ),
            (
                "target-clippy",
                &[
                    "clippy",
                    "--target",
                    "thumbv7em-none-eabi",
                    "--lib",
                    "--locked",
                    "--offline",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            ("host-test", &["test", "--lib", "--locked", "--offline"]),
            ("doctest", &["test", "--doc", "--locked", "--offline"]),
            (
                "package",
                &[
                    "package",
                    "--locked",
                    "--offline",
                    "--allow-dirty",
                    "--no-verify",
                ],
            ),
        ];
        for (label, arguments) in commands {
            require_success(
                label,
                cargo_command(
                    &manifest_path,
                    &temporary.child(&format!("{}-{label}", revision.revision)),
                    arguments,
                ),
            );
            assert_eq!(directory_snapshot(consumer), baseline);
        }
        let profile = materialize_embedded_profile(&base, revision, &source_digest);
        assert_eq!(profile["schema"], PLATFORM_PROFILE_SCHEMA);
        assert_eq!(profile["family"], "embedded-no-std");
        assert_eq!(profile["stages"].as_array().expect("stages").len(), 15);
        measured.push((revision, source_digest, canonical_profile_digest(&profile)));
    }
    assert_ne!(measured[0].2, measured[1].2);
    for (revision, source_digest, profile_digest) in measured {
        println!(
            "{} source={} profile={}",
            revision.revision, source_digest, profile_digest
        );
        assert_eq!(source_digest, revision.expected_source_digest);
        assert_eq!(profile_digest, revision.expected_profile_digest);
    }
}

fn materialize_browser_wasm_profile(
    base: &Value,
    revision: &BrowserWasmRevision,
    source_digest: &str,
) -> Value {
    let template = Revision {
        revision: revision.revision.clone(),
        package_version: revision.package_version.clone(),
        consumer_manifest: revision.consumer_manifest.clone(),
        internal_whitespace: "expected-rejection".to_owned(),
        expected_source_digest: String::new(),
        expected_profile_digest: String::new(),
    };
    let mut profile = materialize_profile(base, &template, source_digest);
    replace_family_strings(&mut profile, "browser-wasm");
    profile["profile_id"] = json!("fixture.browser-wasm");
    profile["family"] = json!("browser-wasm");
    profile["consumer"]["name"] = json!("Controlled browser WASM consumer");
    profile["operation"] = json!({
        "id": "fixture.render-browser-status",
        "name": "Render one escaped browser status element",
        "subject": "Bounded caller text",
        "success_criteria": if revision.accessibility == "unsupported" {
            json!(["Escape HTML metacharacters", "Reject text over 128 bytes", "Accessibility metadata is unsupported"])
        } else {
            json!(["Escape HTML metacharacters", "Validate language metadata", "Emit an aria-live polite status"])
        },
        "non_goals": ["JavaScript binding", "DOM execution", "Browser automation", "Deployment"]
    });
    profile["contracts"][0]["id"] = json!("fixture.contract.browser-wasm");
    profile["contracts"][0]["namespace"] = json!("fixture.browser-wasm");
    profile["contracts"][0]["version"] = json!(revision.revision);
    profile["contracts"][0]["scope"] = json!(revision.accessibility);
    for closure in profile["closures"].as_array_mut().expect("closures") {
        closure["target"] = json!("wasm32-unknown-unknown");
    }
    profile["environment"]["targets"] = json!([tool(
        "target.wasm32-unknown-unknown",
        "wasm32-unknown-unknown",
        "1.95.0",
        "rust-project",
        "rustup target list --installed"
    )]);
    profile["environment"]["runtimes"] = json!([]);
    profile["environment"]["network"] = json!("disabled");
    profile["environment"]["filesystem"] =
        json!("Host behavior tests and isolated wasm target directories");

    for kind in ["check", "lint", "build", "link"] {
        let stage = profile["stages"]
            .as_array_mut()
            .expect("stages")
            .iter_mut()
            .find(|stage| stage["kind"] == kind)
            .expect("wasm target stage");
        stage["command"]["argv"] = if kind == "lint" {
            json!([
                "cargo",
                "clippy",
                "--target",
                "wasm32-unknown-unknown",
                "--lib",
                "--locked",
                "--offline",
                "--",
                "-D",
                "warnings"
            ])
        } else {
            json!([
                "cargo",
                if kind == "check" { "check" } else { "build" },
                "--target",
                "wasm32-unknown-unknown",
                "--lib",
                "--locked",
                "--offline"
            ])
        };
        stage["capabilities"] = json!(["Exact wasm32-unknown-unknown compilation"]);
    }
    for kind in ["execute", "operational-validation"] {
        let stage = profile["stages"]
            .as_array_mut()
            .expect("stages")
            .iter_mut()
            .find(|stage| stage["kind"] == kind)
            .expect("browser runtime stage");
        stage["state"] = json!("unavailable");
        stage["evidence"]["state"] = json!("unavailable");
        stage["evidence"]["diagnostic"] =
            json!("No JavaScript binding, browser, DOM, or automation owner is configured");
        stage["capabilities"] = json!([]);
    }
    for kind in ["unit-test", "doctest", "contract-conformance"] {
        let stage = profile["stages"]
            .as_array_mut()
            .expect("stages")
            .iter_mut()
            .find(|stage| stage["kind"] == kind)
            .expect("host test stage");
        stage["command"]["argv"] = if kind == "doctest" {
            json!(["cargo", "test", "--doc", "--locked", "--offline"])
        } else {
            json!(["cargo", "test", "--lib", "--locked", "--offline"])
        };
        stage["capabilities"] = json!(["Host execution of rendering contract"]);
    }
    profile["limitations"] = json!([
        {
            "id": "limit.no-browser-runtime",
            "scope": "Execution and operations",
            "description": "No JavaScript binding, DOM, browser, automation, network, storage, or bundler",
            "consequence": "Browser execution and operational validation remain unavailable",
            "expires_at": "2026-11-10T00:00:00Z"
        },
        {
            "id": "limit.controlled-browser-wasm-only",
            "scope": "Entire profile",
            "description": "Controlled zero-dependency rendering library",
            "consequence": "It cannot establish browser compatibility, accessibility conformance, security, or support",
            "expires_at": "2026-11-10T00:00:00Z"
        }
    ]);
    profile
}

#[test]
fn browser_wasm_family_preserves_target_and_owner_workflows() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/platform-profiles/browser-wasm");
    let manifest: BrowserWasmFamilyManifest = serde_json::from_slice(
        &fs::read(root.join("family.json")).expect("read browser WASM manifest"),
    )
    .expect("parse browser WASM manifest");
    assert_eq!(manifest.schema, FAMILY_SCHEMA);
    assert_eq!(manifest.family, "browser-wasm");
    let base: Value =
        serde_json::from_slice(&fs::read(root.join(&manifest.base)).expect("read base profile"))
            .expect("parse base profile");
    let temporary = TestDirectory::new("browser-wasm");
    let mut measured = Vec::new();
    for revision in &manifest.revisions {
        let manifest_path = root.join(&revision.consumer_manifest);
        let consumer = manifest_path.parent().expect("consumer");
        let baseline = directory_snapshot(consumer);
        let source_digest = framed_tree_digest(consumer);
        let commands: [(&str, &[&str]); 7] = [
            (
                "metadata",
                &[
                    "metadata",
                    "--format-version",
                    "1",
                    "--no-deps",
                    "--locked",
                    "--offline",
                ],
            ),
            (
                "target-check",
                &[
                    "check",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--lib",
                    "--locked",
                    "--offline",
                ],
            ),
            (
                "target-build",
                &[
                    "build",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--lib",
                    "--locked",
                    "--offline",
                ],
            ),
            (
                "target-clippy",
                &[
                    "clippy",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--lib",
                    "--locked",
                    "--offline",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            ("host-test", &["test", "--lib", "--locked", "--offline"]),
            ("doctest", &["test", "--doc", "--locked", "--offline"]),
            (
                "package",
                &[
                    "package",
                    "--locked",
                    "--offline",
                    "--allow-dirty",
                    "--no-verify",
                ],
            ),
        ];
        for (label, arguments) in commands {
            require_success(
                label,
                cargo_command(
                    &manifest_path,
                    &temporary.child(&format!("{}-{label}", revision.revision)),
                    arguments,
                ),
            );
            assert_eq!(directory_snapshot(consumer), baseline);
        }
        let profile = materialize_browser_wasm_profile(&base, revision, &source_digest);
        assert_eq!(profile["schema"], PLATFORM_PROFILE_SCHEMA);
        assert_eq!(profile["family"], "browser-wasm");
        assert_eq!(profile["stages"].as_array().expect("stages").len(), 15);
        measured.push((revision, source_digest, canonical_profile_digest(&profile)));
    }
    assert_ne!(measured[0].2, measured[1].2);
    for (revision, source_digest, profile_digest) in measured {
        println!(
            "{} source={} profile={}",
            revision.revision, source_digest, profile_digest
        );
        assert_eq!(source_digest, revision.expected_source_digest);
        assert_eq!(profile_digest, revision.expected_profile_digest);
    }
}

fn replace_string_fragment(value: &mut Value, from: &str, to: &str) {
    match value {
        Value::Array(values) => values
            .iter_mut()
            .for_each(|value| replace_string_fragment(value, from, to)),
        Value::Object(values) => values
            .values_mut()
            .for_each(|value| replace_string_fragment(value, from, to)),
        Value::String(text) => *text = text.replace(from, to),
        _ => {}
    }
}

fn materialize_component_profile(
    base: &Value,
    revision: &ComponentRevision,
    source_digest: &str,
) -> Value {
    let browser = BrowserWasmRevision {
        revision: revision.revision.clone(),
        package_version: revision.package_version.clone(),
        consumer_manifest: revision.consumer_manifest.clone(),
        accessibility: "unsupported".to_owned(),
        expected_source_digest: String::new(),
        expected_profile_digest: String::new(),
    };
    let mut profile = materialize_browser_wasm_profile(base, &browser, source_digest);
    replace_string_fragment(&mut profile, "browser-wasm", "wasm-component");
    replace_string_fragment(&mut profile, "wasm32-unknown-unknown", "wasm32-wasip2");
    profile["profile_id"] = json!("fixture.wasm-component");
    profile["family"] = json!("wasm-component");
    profile["consumer"]["name"] = json!("Controlled WebAssembly component consumer");
    profile["operation"] = json!({
        "id": "fixture.component-normalize",
        "name": "Normalize one component string",
        "subject": "Exact local WIT world and matching host semantics",
        "success_criteria": if revision.wit_contract == "infallible-v1" {
            json!(["Trim and lowercase input", "Export an infallible WIT string result"])
        } else {
            json!(["Trim and lowercase bounded ASCII input", "Return explicit too-long or invalid-character errors"])
        },
        "non_goals": ["Generated bindings", "Component runtime", "Composition", "Registry"]
    });
    profile["contracts"][0]["id"] = json!("fixture.contract.wasm-component");
    profile["contracts"][0]["namespace"] = json!("ferris:profile/normalizer");
    profile["contracts"][0]["version"] = json!(revision.revision);
    profile["contracts"][0]["scope"] = json!(revision.wit_contract);
    profile["environment"]["targets"] = json!([tool(
        "target.wasm32-wasip2",
        "wasm32-wasip2",
        "1.95.0",
        "rust-project",
        "rustup target list --installed"
    )]);
    profile["limitations"] = json!([
        {
            "id": "limit.no-component-runtime",
            "scope": "Execution and operations",
            "description": "No generated binding, component runtime, composition, registry, or deployment owner",
            "consequence": "Runtime and operational validation remain unavailable",
            "expires_at": "2026-11-10T00:00:00Z"
        },
        {
            "id": "limit.artifact-only",
            "scope": "Target artifact",
            "description": "The target emits a non-empty wasm32-wasip2 artifact; component structure is not independently inspected",
            "consequence": "No runtime compatibility or interoperability claim",
            "expires_at": "2026-11-10T00:00:00Z"
        }
    ]);
    profile
}

#[test]
fn wasm_component_family_preserves_contract_artifact_and_owner_workflows() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/platform-profiles/wasm-component");
    let manifest: ComponentFamilyManifest = serde_json::from_slice(
        &fs::read(root.join("family.json")).expect("read component manifest"),
    )
    .expect("parse component manifest");
    assert_eq!(manifest.schema, FAMILY_SCHEMA);
    assert_eq!(manifest.family, "wasm-component");
    let base: Value =
        serde_json::from_slice(&fs::read(root.join(&manifest.base)).expect("read base profile"))
            .expect("parse base profile");
    let temporary = TestDirectory::new("wasm-component");
    let mut measured = Vec::new();
    for revision in &manifest.revisions {
        let manifest_path = root.join(&revision.consumer_manifest);
        let consumer = manifest_path.parent().expect("consumer");
        let baseline = directory_snapshot(consumer);
        let source_digest = framed_tree_digest(consumer);
        let commands: [(&str, &[&str]); 7] = [
            (
                "metadata",
                &[
                    "metadata",
                    "--format-version",
                    "1",
                    "--no-deps",
                    "--locked",
                    "--offline",
                ],
            ),
            (
                "target-check",
                &[
                    "check",
                    "--target",
                    "wasm32-wasip2",
                    "--locked",
                    "--offline",
                ],
            ),
            (
                "target-build",
                &[
                    "build",
                    "--target",
                    "wasm32-wasip2",
                    "--locked",
                    "--offline",
                ],
            ),
            (
                "target-clippy",
                &[
                    "clippy",
                    "--target",
                    "wasm32-wasip2",
                    "--all-targets",
                    "--locked",
                    "--offline",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            ("host-test", &["test", "--lib", "--locked", "--offline"]),
            ("doctest", &["test", "--doc", "--locked", "--offline"]),
            (
                "package",
                &[
                    "package",
                    "--locked",
                    "--offline",
                    "--allow-dirty",
                    "--no-verify",
                ],
            ),
        ];
        for (label, arguments) in commands {
            let target = temporary.child(&format!("{}-{label}", revision.revision));
            require_success(label, cargo_command(&manifest_path, &target, arguments));
            if label == "target-build" {
                let artifact = target
                    .join("wasm32-wasip2")
                    .join("debug")
                    .join("ferris-profile-wasm-component.wasm");
                assert!(fs::metadata(artifact).expect("component artifact").len() > 0);
            }
            assert_eq!(directory_snapshot(consumer), baseline);
        }
        let profile = materialize_component_profile(&base, revision, &source_digest);
        measured.push((revision, source_digest, canonical_profile_digest(&profile)));
    }
    assert_ne!(measured[0].2, measured[1].2);
    for (revision, source_digest, profile_digest) in measured {
        println!(
            "{} source={} profile={}",
            revision.revision, source_digest, profile_digest
        );
        assert_eq!(source_digest, revision.expected_source_digest);
        assert_eq!(profile_digest, revision.expected_profile_digest);
    }
}

fn materialize_native_profile(
    base: &Value,
    revision: &NativeRevision,
    source_digest: &str,
) -> Value {
    let hosted = HostedRevision {
        revision: revision.revision.clone(),
        package_version: revision.package_version.clone(),
        consumer_manifest: revision.consumer_manifest.clone(),
        readiness: "unsupported".to_owned(),
        expected_source_digest: String::new(),
        expected_profile_digest: String::new(),
    };
    let mut profile = materialize_hosted_profile(base, &hosted, source_digest);
    replace_string_fragment(&mut profile, "hosted-service", "native-dependency");
    profile["profile_id"] = json!("fixture.native-dependency");
    profile["family"] = json!("native-dependency");
    profile["consumer"]["name"] = json!("Controlled system-native consumer");
    profile["operation"] = json!({
        "id": "fixture.native-process-identity",
        "name": "Read exact operating-system owner identities",
        "subject": "Windows kernel32 or Unix libc process API",
        "success_criteria": if revision.native_boundary == "current-process" {
            json!(["Return a nonzero current process identity"])
        } else {
            json!(["Return a nonzero current process identity", "Return a nonzero parent-process or thread identity"])
        },
        "non_goals": ["Native package discovery", "Dynamic loading", "Arbitrary FFI", "Deployment"]
    });
    profile["contracts"][0]["id"] = json!("fixture.contract.native-dependency");
    profile["contracts"][0]["namespace"] = json!("fixture.native-dependency");
    profile["contracts"][0]["version"] = json!(revision.revision);
    profile["contracts"][0]["scope"] = json!(revision.native_boundary);
    profile["environment"]["native_tools"] = json!([
        tool(
            "native.windows.kernel32",
            "kernel32",
            "host-owned",
            "microsoft",
            "rustc link result"
        ),
        tool(
            "native.unix.libc",
            "libc",
            "host-owned",
            "platform-distribution",
            "rustc link result"
        )
    ]);
    profile["environment"]["providers"] = json!([
        tool(
            "provider.windows-os",
            "Windows process API",
            "host-owned",
            "microsoft",
            "owner unit test"
        ),
        tool(
            "provider.unix-os",
            "POSIX process API",
            "host-owned",
            "platform-distribution",
            "owner unit test"
        )
    ]);
    profile["environment"]["network"] = json!("disabled");
    profile["assurance"] = json!([
        {
            "id": format!("assurance.native-dependency.{}.ffi-boundary", revision.revision),
            "state": "pass",
            "claim_class": "directly-observed",
            "owner": "fixture.owner",
            "subject": "Conditional system-native FFI boundary",
            "scope": "No pointers, buffers, allocation, callbacks, or dynamic loading",
            "observed_at": "2026-08-13T00:00:00Z",
            "expires_at": "2026-11-11T00:00:00Z",
            "source": source("file", &format!("fixture.source.native-dependency.{}", revision.revision), "fixture.owner", &revision.revision, "consumer/src/lib.rs"),
            "diagnostic": "Passing exact declarations and tests are not a general ABI, safety, or servicing proof",
            "limitations": []
        }
    ]);
    profile["limitations"] = json!([
        {
            "id": "limit.ambient-system-native",
            "scope": "Native provider",
            "description": "Windows kernel32 and Unix libc are installed, patched, and serviced outside Cargo",
            "consequence": "The Cargo graph does not establish native package identity or update ownership",
            "expires_at": "2026-11-11T00:00:00Z"
        },
        {
            "id": "limit.exact-ffi-only",
            "scope": "Interop boundary",
            "description": "Only the declared no-argument process identity functions are observed",
            "consequence": "No broader FFI or ABI portability claim",
            "expires_at": "2026-11-11T00:00:00Z"
        }
    ]);
    profile
}

#[test]
fn native_dependency_family_preserves_system_boundary_and_owner_workflows() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/platform-profiles/native-dependency");
    let manifest: NativeFamilyManifest =
        serde_json::from_slice(&fs::read(root.join("family.json")).expect("read native manifest"))
            .expect("parse native manifest");
    assert_eq!(manifest.schema, FAMILY_SCHEMA);
    assert_eq!(manifest.family, "native-dependency");
    let base: Value =
        serde_json::from_slice(&fs::read(root.join(&manifest.base)).expect("read base profile"))
            .expect("parse base profile");
    let temporary = TestDirectory::new("native-dependency");
    let mut measured = Vec::new();
    for revision in &manifest.revisions {
        let manifest_path = root.join(&revision.consumer_manifest);
        let consumer = manifest_path.parent().expect("consumer");
        let baseline = directory_snapshot(consumer);
        let source_digest = framed_tree_digest(consumer);
        let source = fs::read_to_string(consumer.join("src/lib.rs")).expect("native source");
        assert!(source.contains("unsafe extern"));
        assert!(!source.contains("*const"));
        assert!(!source.contains("*mut"));
        let commands: [(&str, &[&str]); 7] = [
            (
                "metadata",
                &[
                    "metadata",
                    "--format-version",
                    "1",
                    "--no-deps",
                    "--locked",
                    "--offline",
                ],
            ),
            ("check", &["check", "--locked", "--offline"]),
            ("build", &["build", "--locked", "--offline"]),
            (
                "clippy",
                &[
                    "clippy",
                    "--all-targets",
                    "--locked",
                    "--offline",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            ("test", &["test", "--lib", "--locked", "--offline"]),
            ("doctest", &["test", "--doc", "--locked", "--offline"]),
            (
                "package",
                &[
                    "package",
                    "--locked",
                    "--offline",
                    "--allow-dirty",
                    "--no-verify",
                ],
            ),
        ];
        for (label, arguments) in commands {
            require_success(
                label,
                cargo_command(
                    &manifest_path,
                    &temporary.child(&format!("{}-{label}", revision.revision)),
                    arguments,
                ),
            );
            assert_eq!(directory_snapshot(consumer), baseline);
        }
        let profile = materialize_native_profile(&base, revision, &source_digest);
        assert_eq!(profile["family"], "native-dependency");
        assert_eq!(profile["stages"].as_array().expect("stages").len(), 15);
        measured.push((revision, source_digest, canonical_profile_digest(&profile)));
    }
    assert_ne!(measured[0].2, measured[1].2);
    for (revision, source_digest, profile_digest) in measured {
        println!(
            "{} source={} profile={}",
            revision.revision, source_digest, profile_digest
        );
        assert_eq!(source_digest, revision.expected_source_digest);
        assert_eq!(profile_digest, revision.expected_profile_digest);
    }
}
