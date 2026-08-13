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
