use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

fn cargo_ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cargo-ferris"))
}

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(path)
}

fn assert_matching_json_outputs(
    expected: std::process::Output,
    actual: std::process::Output,
    label: &str,
) {
    assert_eq!(expected.status.code(), Some(0), "{label} expected exit");
    assert_eq!(actual.status.code(), Some(0), "{label} actual exit");
    assert!(expected.stderr.is_empty(), "{label} expected stderr");
    assert!(actual.stderr.is_empty(), "{label} actual stderr");
    let expected_value: Value =
        serde_json::from_slice(&expected.stdout).expect("expected JSON output");
    let actual_value: Value = serde_json::from_slice(&actual.stdout).expect("actual JSON output");
    assert_eq!(actual_value, expected_value, "{label}");
}

fn assert_version_output(output: std::process::Output, expected_name: &str) {
    assert!(output.status.success(), "{expected_name} exit");
    assert!(output.stderr.is_empty(), "{expected_name} stderr");
    assert_eq!(
        String::from_utf8(output.stdout).expect("version output"),
        format!("{expected_name} {}\n", env!("CARGO_PKG_VERSION"))
    );
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
            "ferris-cli-integration-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create isolated test directory");
        Self { path }
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create fixture destination");
    for entry in fs::read_dir(source).expect("read fixture directory") {
        let entry = entry.expect("fixture entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type().expect("fixture file type").is_dir() {
            copy_tree(&source_path, &destination_path);
        } else {
            fs::copy(source_path, destination_path).expect("copy fixture file");
        }
    }
}

fn run_git(repository: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(repository)
        .args(arguments)
        .output()
        .expect("run Git fixture command");
    assert!(
        output.status.success(),
        "Git fixture command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("Git fixture output")
        .trim()
        .to_owned()
}

fn revision_validation_fixture(label: &str) -> (TestDirectory, PathBuf, String, String) {
    let directory = TestDirectory::new(label);
    let repository = directory.path.join("repository");
    copy_tree(&fixture("simple-workspace"), &repository);
    run_git(&repository, &["init", "--initial-branch", "main"]);
    run_git(
        &repository,
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(&repository, &["config", "user.name", "Ferris Test"]);
    run_git(&repository, &["add", "."]);
    run_git(&repository, &["commit", "-m", "base"]);
    let base = run_git(&repository, &["rev-parse", "HEAD"]);
    run_git(
        &repository,
        &[
            "mv",
            "web/docs/package.json",
            "web/docs/package-renamed.json",
        ],
    );
    run_git(&repository, &["commit", "-m", "rename web input"]);
    let head = run_git(&repository, &["rev-parse", "HEAD"]);
    (directory, repository, base, head)
}

fn revision_validation_output(
    repository: &Path,
    base: &str,
    head: &str,
    tested: &str,
) -> std::process::Output {
    revision_validation_command(repository, base, head, tested)
        .output()
        .expect("run revision-bound validation-plan")
}

fn revision_validation_command(repository: &Path, base: &str, head: &str, tested: &str) -> Command {
    let mut command = ferris();
    command.args([
        "validation-plan",
        "--workspace-id",
        "ferris.test/simple",
        "--manifest-path",
        repository
            .join("Cargo.toml")
            .to_str()
            .expect("manifest path"),
        "--base-revision",
        base,
        "--head-revision",
        head,
        "--tested-revision",
        tested,
        "--owner-domains",
        repository
            .join("owner-domains.json")
            .to_str()
            .expect("owner domains path"),
        "--format",
        "json",
    ]);
    command
}

fn directory_entries(path: &Path) -> Vec<String> {
    let mut entries = fs::read_dir(path)
        .expect("read directory")
        .map(|entry| {
            entry
                .expect("read directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    entries.sort();
    entries
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

fn cargo_metadata(manifest: &Path) -> std::process::Output {
    Command::new(option_env!("CARGO").unwrap_or("cargo"))
        .current_dir(manifest.parent().expect("manifest directory"))
        .env("CARGO_NET_OFFLINE", "true")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--locked",
            "--offline",
            "--manifest-path",
        ])
        .arg(manifest)
        .output()
        .expect("run owner Cargo metadata")
}

fn cargo_test(manifest: &Path, target_directory: &Path) -> std::process::Output {
    Command::new(option_env!("CARGO").unwrap_or("cargo"))
        .current_dir(manifest.parent().expect("manifest directory"))
        .env("CARGO_NET_OFFLINE", "true")
        .env("RUSTUP_AUTO_INSTALL", "0")
        .args([
            "test",
            "--quiet",
            "--locked",
            "--offline",
            "--manifest-path",
        ])
        .arg(manifest)
        .arg("--target-dir")
        .arg(target_directory)
        .output()
        .expect("run owner Cargo tests")
}

#[test]
fn help_surfaces_match_between_all_invocations() {
    let ferris_output = ferris().arg("--help").output().expect("run ferris help");
    let cargo_output = cargo_ferris()
        .arg("--help")
        .output()
        .expect("run cargo-ferris help");
    let cargo_style_output = cargo_ferris()
        .args(["ferris", "--help"])
        .output()
        .expect("run cargo-style cargo-ferris help");
    assert!(ferris_output.status.success());
    assert!(cargo_output.status.success());
    assert!(cargo_style_output.status.success());
    assert!(ferris_output.stderr.is_empty());
    assert!(cargo_output.stderr.is_empty());
    assert!(cargo_style_output.stderr.is_empty());

    let ferris_help = String::from_utf8(ferris_output.stdout).expect("ferris help");
    let cargo_help = String::from_utf8(cargo_output.stdout).expect("cargo-ferris help");
    let cargo_style_help = String::from_utf8(cargo_style_output.stdout).expect("cargo-style help");
    for command_name in [
        "plan",
        "validation-plan",
        "explain",
        "graph",
        "doctor",
        "profile-diff",
        "federated-plan",
        "federated-validation-plan",
        "revision-skew",
        "replay",
        "schedule",
        "artifacts",
        "go",
        "verify",
    ] {
        assert!(ferris_help.contains(command_name), "{command_name}");
        assert!(cargo_help.contains(command_name), "{command_name}");
        assert!(cargo_style_help.contains(command_name), "{command_name}");
    }
    assert!(ferris_help.contains("Usage: ferris"));
    assert!(cargo_help.contains("Usage: cargo-ferris"));
    assert!(cargo_style_help.contains("Usage: cargo ferris"));
}

#[test]
fn version_banners_match_invocation_names() {
    assert_version_output(
        ferris()
            .arg("--version")
            .output()
            .expect("run ferris version"),
        "ferris",
    );
    assert_version_output(
        cargo_ferris()
            .arg("--version")
            .output()
            .expect("run cargo-ferris version"),
        "cargo-ferris",
    );
    assert_version_output(
        cargo_ferris()
            .args(["ferris", "--version"])
            .output()
            .expect("run cargo-style cargo-ferris version"),
        "cargo ferris",
    );
}

#[test]
fn ferris_cli_package_has_no_lib_target_in_cargo_metadata() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let metadata = cargo_metadata(&manifest);
    assert!(metadata.status.success());
    assert!(metadata.stderr.is_empty());

    let value: Value = serde_json::from_slice(&metadata.stdout).expect("Cargo metadata JSON");
    let package = value["packages"]
        .as_array()
        .expect("package list")
        .iter()
        .find(|package| package["name"] == "ferris-cli")
        .expect("ferris-cli package");
    let targets = package["targets"].as_array().expect("target list");

    assert!(
        !targets.iter().any(|target| {
            target["kind"]
                .as_array()
                .expect("target kinds")
                .iter()
                .any(|kind| kind.as_str() == Some("lib"))
        }),
        "ferris-cli should remain binary-only"
    );

    let mut binary_names = targets
        .iter()
        .filter(|target| {
            target["kind"]
                .as_array()
                .expect("target kinds")
                .iter()
                .any(|kind| kind.as_str() == Some("bin"))
        })
        .map(|target| {
            target["name"]
                .as_str()
                .expect("binary target name")
                .to_owned()
        })
        .collect::<Vec<_>>();
    binary_names.sort();
    assert_eq!(
        binary_names,
        vec!["cargo-ferris".to_owned(), "ferris".to_owned()]
    );
}

#[test]
fn cargo_ferris_plan_json_matches_ferris() {
    let ferris_output = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    let cargo_output = cargo_ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run cargo-ferris");

    assert_matching_json_outputs(ferris_output, cargo_output, "plan parity");
}

#[test]
fn cargo_adapters_default_to_cargos_current_workspace_manifest() {
    let manifest = fixture("simple-workspace/Cargo.toml");
    let nested_directory = fixture("simple-workspace/alpha/src");
    let ferris_output = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            manifest.to_str().expect("fixture manifest"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    let direct_output = cargo_ferris()
        .current_dir(&nested_directory)
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--format",
            "json",
        ])
        .output()
        .expect("run direct cargo-ferris");
    let cargo_style_output = cargo_ferris()
        .current_dir(&nested_directory)
        .args([
            "ferris",
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--format",
            "json",
        ])
        .output()
        .expect("run cargo-style cargo-ferris");

    let expected: Value = serde_json::from_slice(&ferris_output.stdout).expect("ferris JSON");
    assert!(ferris_output.status.success());
    for (label, output) in [
        ("direct cargo-ferris", direct_output),
        ("cargo ferris", cargo_style_output),
    ] {
        assert!(output.status.success(), "{label} exit");
        assert!(output.stderr.is_empty(), "{label} stderr");
        let actual: Value = serde_json::from_slice(&output.stdout).expect("adapter JSON");
        assert_eq!(actual, expected, "{label} parity");
    }
}

#[test]
fn cargo_discovery_accepts_success_with_owner_diagnostics() {
    let output = cargo_ferris()
        .current_dir(fixture("simple-workspace/alpha/src"))
        .env("CARGO_LOG", "trace")
        .args([
            "ferris",
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--format",
            "json",
        ])
        .output()
        .expect("run cargo-style cargo-ferris with Cargo diagnostics");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let value: Value = serde_json::from_slice(&output.stdout).expect("plan JSON");
    assert_eq!(value["semantic_command_id"], "plan");
    assert_eq!(value["result_class"], "success");
}

#[test]
fn cargo_style_current_workspace_default_covers_existing_workspace_commands() {
    let manifest = fixture("simple-workspace/Cargo.toml");
    let nested_directory = fixture("simple-workspace/alpha/src");
    for command_name in ["explain", "graph", "doctor"] {
        let ferris_output = ferris()
            .args([
                command_name,
                "--workspace-id",
                "ferris.test/simple",
                "--manifest-path",
                manifest.to_str().expect("fixture manifest"),
                "--format",
                "json",
            ])
            .output()
            .expect("run ferris");
        let cargo_output = cargo_ferris()
            .current_dir(&nested_directory)
            .args([
                "ferris",
                command_name,
                "--workspace-id",
                "ferris.test/simple",
                "--format",
                "json",
            ])
            .output()
            .expect("run cargo-style cargo-ferris");

        assert_matching_json_outputs(ferris_output, cargo_output, command_name);
    }

    let changed_path = fixture("simple-workspace/alpha/src/lib.rs");
    let ferris_output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            manifest.to_str().expect("fixture manifest"),
            "--changed-path",
            changed_path.to_str().expect("changed path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris validation-plan");
    let cargo_output = cargo_ferris()
        .current_dir(&nested_directory)
        .args([
            "ferris",
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--changed-path",
            changed_path.to_str().expect("changed path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run cargo-style validation-plan");
    assert_matching_json_outputs(ferris_output, cargo_output, "validation-plan default");
}

#[test]
fn explicit_cargo_adapter_manifest_bypasses_current_workspace_discovery() {
    let directory = TestDirectory::new("explicit-manifest");
    let output = cargo_ferris()
        .current_dir(&directory.path)
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture manifest"),
            "--format",
            "json",
        ])
        .output()
        .expect("run cargo-ferris");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}

#[test]
fn standalone_ferris_still_requires_explicit_manifest() {
    let output = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let value: Value = serde_json::from_slice(&output.stderr).expect("typed CLI error");
    assert_eq!(value["semantic_command_id"], "plan");
    assert_eq!(value["result_class"], "invalid");
}

#[test]
fn cargo_workspace_discovery_failure_is_typed_and_path_free() {
    let directory = TestDirectory::new("discovery-failure");
    let output = cargo_ferris()
        .current_dir(&directory.path)
        .args([
            "ferris",
            "plan",
            "--workspace-id",
            "ferris.test/missing",
            "--format",
            "json",
        ])
        .output()
        .expect("run cargo-style cargo-ferris");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("typed discovery error");
    assert!(!stderr.contains(&directory.path.to_string_lossy().into_owned()));
    let value: Value = serde_json::from_str(&stderr).expect("discovery JSON");
    assert_eq!(value["semantic_command_id"], "plan");
    assert_eq!(value["result_class"], "invalid");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "FERRIS-WORKSPACE-DISCOVERY-NOT-FOUND"
    );
}

#[test]
fn invalid_workspace_id_fails_before_current_workspace_discovery() {
    let directory = TestDirectory::new("invalid-workspace-id");
    let output = cargo_ferris()
        .current_dir(&directory.path)
        .args([
            "ferris",
            "plan",
            "--workspace-id",
            "invalid workspace id",
            "--format",
            "json",
        ])
        .output()
        .expect("run cargo-style cargo-ferris");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let value: Value = serde_json::from_slice(&output.stderr).expect("typed workspace ID error");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "FERRIS-WORKSPACE-ID-INVALID"
    );
}

#[test]
fn cargo_style_validation_plan_json_matches_ferris() {
    let ferris_output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--changed-path",
            fixture("simple-workspace/alpha/src/lib.rs")
                .to_str()
                .expect("fixture path"),
            "--changed-package",
            "fixture-alpha",
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    let cargo_output = cargo_ferris()
        .args([
            "ferris",
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--changed-path",
            fixture("simple-workspace/alpha/src/lib.rs")
                .to_str()
                .expect("fixture path"),
            "--changed-package",
            "fixture-alpha",
            "--format",
            "json",
        ])
        .output()
        .expect("run cargo-style cargo-ferris");

    assert_matching_json_outputs(ferris_output, cargo_output, "validation-plan parity");
}

#[test]
fn mixed_case_cargo_style_validation_plan_obeys_platform_rules() {
    let ferris_output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--changed-path",
            fixture("simple-workspace/alpha/src/lib.rs")
                .to_str()
                .expect("fixture path"),
            "--changed-package",
            "fixture-alpha",
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    let cargo_output = cargo_ferris()
        .args([
            "Ferris",
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--changed-path",
            fixture("simple-workspace/alpha/src/lib.rs")
                .to_str()
                .expect("fixture path"),
            "--changed-package",
            "fixture-alpha",
            "--format",
            "json",
        ])
        .output()
        .expect("run mixed-case cargo-style cargo-ferris");

    if cfg!(windows) {
        assert_matching_json_outputs(ferris_output, cargo_output, "mixed-case validation-plan");
    } else {
        assert_eq!(cargo_output.status.code(), Some(2));
        assert!(cargo_output.stdout.is_empty());
        let value: Value = serde_json::from_slice(&cargo_output.stderr).expect("error JSON");
        assert_eq!(value["semantic_command_id"], "cli");
        assert_eq!(value["result_class"], "invalid");
        assert_eq!(
            value["diagnostics"][0]["next_actions"][0],
            "Run cargo-ferris --help or cargo-ferris <command> --help."
        );
    }
}

#[test]
fn cargo_ferris_invalid_cli_mentions_direct_help_name() {
    let output = cargo_ferris()
        .args(["plan", "--unknown-option"])
        .output()
        .expect("run cargo-ferris");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());

    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(value["semantic_command_id"], "plan");
    assert_eq!(value["result_class"], "invalid");
    assert_eq!(
        value["diagnostics"][0]["next_actions"][0],
        "Run cargo-ferris --help or cargo-ferris <command> --help."
    );
}

#[test]
fn ferris_does_not_strip_literal_ferris_argument() {
    let output = ferris()
        .args([
            "ferris",
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());

    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(value["semantic_command_id"], "cli");
    assert_eq!(value["result_class"], "invalid");
    assert_eq!(value["diagnostics"][0]["code"], "FERRIS-CLI-INVALID");
}

#[test]
fn plan_json_is_non_executable() {
    let output = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("plan JSON");
    assert_eq!(value["result_class"], "success");
    assert_eq!(value["process_exit_code"], 0);
    assert!(
        value["result_identity"]
            .as_str()
            .expect("result identity")
            .starts_with("result:")
    );
    assert!(
        value["selection_identity"]
            .as_str()
            .expect("selection identity")
            .starts_with("selection:")
    );
    assert_eq!(value["record"]["executable"], false);
    assert_eq!(value["record"]["packages"].as_array().unwrap().len(), 2);
    assert_eq!(value["record"]["workspace_root"], ".");
    assert_eq!(value["record"]["workspace_id"], "ferris.test/simple");
    assert_eq!(value["record"]["selected_manifest"], "Cargo.toml");
    assert_eq!(
        value["record"]["plan_id"],
        "plan:cdc17a3318a6ae17c4dbc847635f3dc4aa07b527f41d632ecedfc7be64a3290b"
    );
    let serialized = String::from_utf8(output.stdout).expect("utf-8 output");
    assert!(!serialized.contains(r"C:\src\FERRIS"));
    assert!(!serialized.contains(r"\\?\"));
}

#[test]
fn graph_preserves_declared_workspace_and_external_edges() {
    let output = ferris()
        .args([
            "graph",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("graph JSON");
    assert_eq!(value["semantic_command_id"], "graph");
    assert_eq!(value["record"]["schema"], "ferris.workspace-graph/v0");
    assert_eq!(value["record"]["executable"], false);
    assert_eq!(
        value["record"]["graph_id"],
        "graph:020c32e19d9276552c83a0fa1beed86ff1177ce4676d5596ea09a43551d3c8f0"
    );
    assert_eq!(value["record"]["nodes"].as_array().unwrap().len(), 2);
    assert_eq!(value["record"]["edges"].as_array().unwrap().len(), 3);

    let edges = value["record"]["edges"].as_array().expect("edges");
    assert!(edges.iter().any(|edge| {
        edge["dependency_name"] == "fixture-alpha"
            && edge["optional"] == true
            && edge["resolution"] == "workspace-member"
    }));
    assert!(edges.iter().any(|edge| {
        edge["dependency_alias"] == "alpha-dev"
            && edge["kind"] == "dev"
            && edge["target_condition"] == "cfg(windows)"
    }));
    assert!(edges.iter().any(|edge| {
        edge["dependency_name"] == "serde"
            && edge["target"].is_null()
            && edge["resolution"] == "external-unresolved"
    }));

    let serialized = String::from_utf8(output.stdout).expect("utf-8 output");
    assert!(!serialized.contains(r"C:\src\FERRIS"));
    assert!(!serialized.contains("/mnt/c/src/FERRIS"));
}

#[test]
fn explain_human_names_selected_packages() {
    let output = ferris()
        .args([
            "explain",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("utf-8 output");
    assert!(stdout.contains("fixture-alpha"));
    assert!(stdout.contains("fixture-beta"));
    assert!(stdout.contains("Omitted:"));
    assert!(stdout.contains("Unknowns:"));
    assert!(stdout.contains("Evidence owner: Cargo"));
    assert!(stdout.contains("Evidence that would change the result:"));
    assert!(stdout.contains("ordinary Cargo commands"));
}

#[test]
fn validation_plan_json_selects_supported_package_closure() {
    let output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--changed-path",
            fixture("simple-workspace/alpha/src/lib.rs")
                .to_str()
                .expect("fixture path"),
            "--changed-package",
            "fixture-alpha",
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("validation-plan JSON");
    assert_eq!(value["semantic_command_id"], "validation-plan");
    assert_eq!(value["record"]["schema"], "ferris.validation-plan/v0");
    assert_eq!(
        value["record"]["selected_packages"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        value["record"]["selected_packages"][0]["package"]["name"],
        "fixture-alpha"
    );
    assert_eq!(
        value["record"]["selected_packages"][1]["package"]["name"],
        "fixture-beta"
    );
    assert_eq!(
        value["record"]["selected_activities"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(value["record"]["fallback"]["required_by_inputs"], false);
    assert_eq!(value["record"]["inputs"][0]["value"], "fixture-alpha");
    assert_eq!(value["record"]["inputs"][1]["value"], "alpha/src/lib.rs");

    let serialized = String::from_utf8(output.stdout).expect("utf-8 output");
    assert!(!serialized.contains(r"C:\src\FERRIS"));
    assert!(!serialized.contains(r"\\?\"));
}

#[test]
fn validation_plan_resolves_existing_relative_paths_from_caller_directory() {
    let workspace = fixture("simple-workspace");
    let output = ferris()
        .current_dir(&workspace)
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            "Cargo.toml",
            "--changed-path",
            "alpha/src/lib.rs",
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris from workspace");
    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("validation-plan JSON");
    assert_eq!(
        value["record"]["inputs"][0]["disposition"],
        "owned_rust_path"
    );
    assert_eq!(value["record"]["fallback"]["required_by_inputs"], false);
}

#[test]
fn validation_plan_json_selects_owner_domain_entrypoint() {
    let output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--changed-path",
            fixture("simple-workspace/web/docs/package.json")
                .to_str()
                .expect("fixture path"),
            "--owner-domains",
            fixture("simple-workspace/owner-domains.json")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let value: Value = serde_json::from_slice(&output.stdout).expect("validation-plan JSON");
    assert_eq!(
        value["record"]["inputs"][0]["disposition"],
        "owner_domain_path"
    );
    assert_eq!(
        value["record"]["owner_domain_contract"]["schema"],
        "ferris.owner-validation-domains/v1"
    );
    assert_eq!(
        value["record"]["selected_owner_entrypoints"][0],
        "web-docs-build"
    );
    assert_eq!(value["record"]["fallback"]["required_by_inputs"], false);
}

#[test]
fn validation_plan_human_reports_owner_domain_without_full_workspace_fallback() {
    let output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--changed-path",
            fixture("simple-workspace/web/docs/package.json")
                .to_str()
                .expect("fixture path"),
            "--owner-domains",
            fixture("simple-workspace/owner-domains.json")
                .to_str()
                .expect("fixture path"),
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("utf-8 output");
    assert!(stdout.contains("Selected owner domains:"));
    assert!(stdout.contains("web-docs (path prefix: web/docs; entrypoints: web-docs-build)"));
    assert!(
        stdout.contains("owner-domain selection does not imply package-specific Cargo selection")
    );
    assert!(
        stdout.contains("run the selected owner entrypoints through repository-owned automation")
    );
    assert!(stdout.contains("required by inputs: false"));
    assert!(!stdout.contains("widens directly to the full workspace fallback"));
    assert!(!stdout.contains("widened immediately to the full workspace fallback"));
}

#[test]
fn validation_plan_json_classifies_deleted_workspace_relative_path() {
    let output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--deleted-path",
            "web/docs/deleted.ts",
            "--owner-domains",
            fixture("simple-workspace/owner-domains.json")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("validation-plan JSON");
    assert_eq!(value["record"]["inputs"][0]["value"], "web/docs/deleted.ts");
    assert_eq!(
        value["record"]["inputs"][0]["disposition"],
        "owner_domain_path"
    );
    assert_eq!(
        value["record"]["inputs"][0]["path_evidence"],
        "lexical_missing"
    );
    assert_eq!(value["record"]["fallback"]["required_by_inputs"], false);
}

#[test]
fn validation_plan_json_omits_owner_identity_for_ambiguous_package_root_match() {
    let output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/ambiguous",
            "--manifest-path",
            fixture("ambiguous-package-roots/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--changed-path",
            fixture("ambiguous-package-roots/outer/member/src/lib.rs")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("validation-plan JSON");
    assert_eq!(
        value["record"]["inputs"][0]["disposition"],
        "full_workspace_fallback"
    );
    assert!(value["record"]["inputs"][0]["package_identity"].is_null());
    assert_eq!(
        value["record"]["selected_packages"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        value["record"]["selected_activities"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(value["record"]["fallback"]["required_by_inputs"], true);
}

#[test]
fn validation_plan_human_reports_full_workspace_fallback() {
    let output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--changed-path",
            fixture("simple-workspace/workspace-policy.txt")
                .to_str()
                .expect("fixture path"),
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("utf-8 output");
    assert!(stdout.contains("Fallback validation:"));
    assert!(stdout.contains("required by inputs: true"));
    assert!(stdout.contains("workspace-policy.txt"));
    assert!(stdout.contains("full workspace fallback"));
    assert!(stdout.contains("Selected validation:"));
}

#[test]
fn revision_bound_validation_plan_is_web_only_and_rename_config_independent() {
    let (_directory, repository, base, head) = revision_validation_fixture("revision-web-only");
    fs::write(
        repository.join("web/docs/package.json"),
        "{\"name\":\"untracked-stale-output\"}\n",
    )
    .expect("recreate deleted path as untracked working-tree content");
    run_git(&repository, &["config", "diff.renames", "true"]);
    let first = revision_validation_output(&repository, &base, &head, &head);
    assert!(first.status.success());
    let first_value: Value =
        serde_json::from_slice(&first.stdout).expect("first revision-bound output");

    let inputs = first_value["record"]["inputs"].as_array().expect("inputs");
    assert_eq!(inputs.len(), 2);
    assert!(inputs.iter().all(|input| {
        input["owner_domain_id"] == "web-docs" && input["disposition"] == "owner_domain_path"
    }));
    assert_eq!(
        first_value["record"]["revision_binding"]["relationship"],
        "tested_is_head"
    );
    assert_eq!(
        first_value["record"]["revision_binding"]["changed_path_count"],
        1
    );
    assert_eq!(
        first_value["record"]["revision_binding"]["deleted_path_count"],
        1
    );
    assert_eq!(
        first_value["record"]["revision_binding"]["working_tree"],
        "dirty"
    );
    assert_eq!(
        first_value["record"]["fallback"]["required_by_inputs"],
        false
    );

    run_git(&repository, &["config", "diff.renames", "false"]);
    let second = revision_validation_output(&repository, &base, &head, &head);
    assert!(second.status.success());
    let second_value: Value =
        serde_json::from_slice(&second.stdout).expect("second revision-bound output");
    assert_eq!(
        second_value["record"]["validation_plan_id"],
        first_value["record"]["validation_plan_id"]
    );
    assert_eq!(
        second_value["record"]["revision_binding"]["change_set_id"],
        first_value["record"]["revision_binding"]["change_set_id"]
    );
    assert_eq!(
        second_value["record"]["revision_binding"]["revision_binding_id"],
        first_value["record"]["revision_binding"]["revision_binding_id"]
    );
    run_git(&repository, &["replace", &head, &base]);
    let replacement_ignored = revision_validation_output(&repository, &base, &head, &head);
    assert!(
        replacement_ignored.status.success(),
        "{}",
        String::from_utf8_lossy(&replacement_ignored.stderr)
    );
    let replacement_value: Value =
        serde_json::from_slice(&replacement_ignored.stdout).expect("replacement-safe JSON");
    assert_eq!(
        replacement_value["record"]["revision_binding"]["revision_binding_id"],
        first_value["record"]["revision_binding"]["revision_binding_id"]
    );
    run_git(&repository, &["replace", "-d", &head]);

    let foreign_directory = TestDirectory::new("revision-foreign-git-environment");
    let foreign_repository = foreign_directory.path.join("repository");
    copy_tree(&fixture("simple-workspace"), &foreign_repository);
    run_git(
        &foreign_repository,
        &["init", "--initial-branch", "foreign"],
    );
    run_git(
        &foreign_repository,
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(&foreign_repository, &["config", "user.name", "Ferris Test"]);
    run_git(&foreign_repository, &["add", "."]);
    run_git(&foreign_repository, &["commit", "-m", "foreign"]);
    let ambient_routing_ignored = revision_validation_command(&repository, &base, &head, &head)
        .env("GIT_DIR", foreign_repository.join(".git"))
        .env("GIT_WORK_TREE", &repository)
        .output()
        .expect("run with foreign ambient Git routing");
    assert!(
        ambient_routing_ignored.status.success(),
        "{}",
        String::from_utf8_lossy(&ambient_routing_ignored.stderr)
    );
    let ambient_value: Value =
        serde_json::from_slice(&ambient_routing_ignored.stdout).expect("ambient-safe JSON");
    assert_eq!(
        ambient_value["record"]["revision_binding"]["revision_binding_id"],
        first_value["record"]["revision_binding"]["revision_binding_id"]
    );
    fs::remove_file(repository.join("web/docs/package.json"))
        .expect("remove untracked deleted-path collision");

    let explicit = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            repository
                .join("Cargo.toml")
                .to_str()
                .expect("manifest path"),
            "--changed-path",
            repository
                .join("web/docs/package-renamed.json")
                .to_str()
                .expect("changed path"),
            "--deleted-path",
            "web/docs/package.json",
            "--owner-domains",
            repository
                .join("owner-domains.json")
                .to_str()
                .expect("owner domains path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run explicit validation-plan");
    assert!(explicit.status.success());
    let explicit_value: Value =
        serde_json::from_slice(&explicit.stdout).expect("explicit validation output");
    assert_eq!(
        explicit_value["record"]["validation_plan_id"],
        first_value["record"]["validation_plan_id"]
    );
    assert!(explicit_value["record"].get("revision_binding").is_none());

    let human = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            repository
                .join("Cargo.toml")
                .to_str()
                .expect("manifest path"),
            "--base-revision",
            &base,
            "--head-revision",
            &head,
            "--tested-revision",
            &head,
            "--owner-domains",
            repository
                .join("owner-domains.json")
                .to_str()
                .expect("owner domains path"),
        ])
        .output()
        .expect("run human revision-bound validation-plan");
    assert!(human.status.success());
    let human_output = String::from_utf8(human.stdout).expect("human output");
    assert!(human_output.contains("Revision binding: validation-revision-binding:"));
    assert!(human_output.contains("relationship: tested_is_head"));
    assert!(human_output.contains("working tree: clean"));
}

#[test]
fn revision_bound_validation_uses_committed_regular_file_mode_for_cargo_scope() {
    let (_directory, repository, base, _) =
        revision_validation_fixture("revision-committed-regular-mode");
    let rust_path = repository.join("alpha/src/feature.rs");
    fs::write(&rust_path, "pub fn feature() {}\n").expect("write committed Rust path");
    run_git(&repository, &["add", "alpha/src/feature.rs"]);
    run_git(&repository, &["commit", "-m", "change Rust input"]);
    let head = run_git(&repository, &["rev-parse", "HEAD"]);

    let regular = revision_validation_output(&repository, &base, &head, &head);
    assert!(
        regular.status.success(),
        "{}",
        String::from_utf8_lossy(&regular.stderr)
    );
    let regular_value: Value =
        serde_json::from_slice(&regular.stdout).expect("regular-path output");
    assert!(
        regular_value["record"]["inputs"]
            .as_array()
            .expect("inputs")
            .iter()
            .any(|input| {
                input["value"] == "alpha/src/feature.rs"
                    && input["disposition"] == "owned_rust_path"
            })
    );

    fs::remove_file(&rust_path).expect("remove working-tree Rust file");
    fs::create_dir(&rust_path).expect("replace working-tree Rust file with directory");
    let dirty_type = revision_validation_output(&repository, &base, &head, &head);
    assert!(
        dirty_type.status.success(),
        "{}",
        String::from_utf8_lossy(&dirty_type.stderr)
    );
    let dirty_type_value: Value =
        serde_json::from_slice(&dirty_type.stdout).expect("dirty-type output");
    assert_eq!(
        dirty_type_value["record"]["validation_plan_id"],
        regular_value["record"]["validation_plan_id"]
    );
    assert_eq!(
        dirty_type_value["record"]["inputs"],
        regular_value["record"]["inputs"]
    );
}

#[test]
fn revision_bound_validation_keeps_case_only_rename_identity_platform_independent() {
    let directory = TestDirectory::new("revision-case-only-rename");
    let repository = directory.path.join("repository");
    copy_tree(&fixture("simple-workspace"), &repository);
    run_git(&repository, &["init", "--initial-branch", "main"]);
    run_git(
        &repository,
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(&repository, &["config", "user.name", "Ferris Test"]);
    run_git(&repository, &["add", "."]);
    run_git(&repository, &["commit", "-m", "base"]);
    let base = run_git(&repository, &["rev-parse", "HEAD"]);
    run_git(
        &repository,
        &[
            "mv",
            "web/docs/package.json",
            "web/docs/package.intermediate",
        ],
    );
    run_git(
        &repository,
        &[
            "mv",
            "web/docs/package.intermediate",
            "web/docs/Package.json",
        ],
    );
    run_git(&repository, &["commit", "-m", "case-only rename"]);
    let head = run_git(&repository, &["rev-parse", "HEAD"]);

    let output = revision_validation_output(&repository, &base, &head, &head);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).expect("case-only rename JSON");
    assert_eq!(value["record"]["revision_binding"]["changed_path_count"], 1);
    assert_eq!(value["record"]["revision_binding"]["deleted_path_count"], 1);
    let inputs = value["record"]["inputs"].as_array().expect("inputs");
    assert!(inputs.iter().any(|input| {
        input["value"] == "web/docs/Package.json" && input.get("path_evidence").is_none()
    }));
    assert!(inputs.iter().any(|input| {
        input["value"] == "web/docs/package.json" && input["path_evidence"] == "lexical_missing"
    }));
}

#[test]
fn revision_bound_validation_ignores_ambient_submodule_diff_configuration() {
    let directory = TestDirectory::new("revision-submodule");
    let dependency = directory.path.join("dependency");
    fs::create_dir_all(&dependency).expect("create dependency repository");
    run_git(&dependency, &["init", "--initial-branch", "main"]);
    run_git(
        &dependency,
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(&dependency, &["config", "user.name", "Ferris Test"]);
    fs::write(dependency.join("version.txt"), "one\n").expect("write dependency v1");
    run_git(&dependency, &["add", "."]);
    run_git(&dependency, &["commit", "-m", "dependency v1"]);

    let repository = directory.path.join("repository");
    copy_tree(&fixture("simple-workspace"), &repository);
    run_git(&repository, &["init", "--initial-branch", "main"]);
    run_git(
        &repository,
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(&repository, &["config", "user.name", "Ferris Test"]);
    run_git(&repository, &["add", "."]);
    run_git(&repository, &["commit", "-m", "workspace"]);
    run_git(
        &repository,
        &[
            "-c",
            "protocol.file.allow=always",
            "submodule",
            "add",
            dependency.to_str().expect("dependency path"),
            "vendor/sub",
        ],
    );
    run_git(&repository, &["commit", "-am", "submodule v1"]);
    let base = run_git(&repository, &["rev-parse", "HEAD"]);

    fs::write(repository.join("vendor/sub/version.txt"), "two\n").expect("write dependency v2");
    run_git(&repository.join("vendor/sub"), &["add", "."]);
    run_git(
        &repository.join("vendor/sub"),
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(
        &repository.join("vendor/sub"),
        &["config", "user.name", "Ferris Test"],
    );
    run_git(
        &repository.join("vendor/sub"),
        &["commit", "-m", "dependency v2"],
    );
    run_git(&repository, &["add", "vendor/sub"]);
    run_git(&repository, &["commit", "-m", "submodule v2"]);
    let head = run_git(&repository, &["rev-parse", "HEAD"]);
    run_git(&repository, &["config", "diff.ignoreSubmodules", "all"]);
    fs::write(repository.join("vendor/sub/version.txt"), "dirty\n")
        .expect("dirty submodule working tree");

    let output = revision_validation_output(&repository, &base, &head, &head);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).expect("submodule JSON");
    assert_eq!(value["record"]["revision_binding"]["changed_path_count"], 1);
    assert_eq!(value["record"]["inputs"][0]["value"], "vendor/sub");
    assert_eq!(
        value["record"]["inputs"][0]["disposition"],
        "full_workspace_fallback"
    );
    assert_eq!(value["record"]["revision_binding"]["working_tree"], "dirty");
}

#[test]
fn revision_bound_validation_uses_committed_symlink_mode_for_cargo_scope() {
    let directory = TestDirectory::new("revision-symlink-mode");
    let repository = directory.path.join("repository");
    copy_tree(&fixture("simple-workspace"), &repository);
    run_git(&repository, &["init", "--initial-branch", "main"]);
    run_git(
        &repository,
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(&repository, &["config", "user.name", "Ferris Test"]);
    run_git(&repository, &["add", "."]);
    run_git(&repository, &["commit", "-m", "base"]);
    let base = run_git(&repository, &["rev-parse", "HEAD"]);

    let link_path = repository.join("web/docs/link.rs");
    fs::write(&link_path, "target.rs").expect("write symbolic-link blob");
    let blob = run_git(&repository, &["hash-object", "-w", "web/docs/link.rs"]);
    let cache_info = format!("120000,{blob},web/docs/link.rs");
    run_git(
        &repository,
        &["update-index", "--add", "--cacheinfo", &cache_info],
    );
    run_git(&repository, &["commit", "-m", "add symbolic link"]);
    let head = run_git(&repository, &["rev-parse", "HEAD"]);

    let output = revision_validation_output(&repository, &base, &head, &head);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).expect("symlink JSON");
    assert_eq!(
        value["record"]["inputs"][0]["disposition"],
        "owner_domain_path"
    );
    assert!(
        value["record"]["selected_packages"]
            .as_array()
            .expect("selected packages")
            .is_empty()
    );
    assert_eq!(
        value["record"]["selected_owner_entrypoints"][0],
        "web-docs-build"
    );
}

#[test]
fn revision_bound_validation_rejects_stale_tested_revision_and_reclassifies_reintroduced_delete() {
    let (_directory, repository, base, head) = revision_validation_fixture("revision-tested-merge");
    fs::copy(
        repository.join("web/docs/package-renamed.json"),
        repository.join("web/docs/package.json"),
    )
    .expect("reintroduce deleted path");
    run_git(&repository, &["add", "web/docs/package.json"]);
    run_git(
        &repository,
        &["commit", "-m", "tested merge reintroduces path"],
    );
    let tested = run_git(&repository, &["rev-parse", "HEAD"]);

    let stale = revision_validation_output(&repository, &base, &head, &head);
    assert_eq!(stale.status.code(), Some(6));
    let stale_value: Value = serde_json::from_slice(&stale.stderr).expect("stale error JSON");
    assert_eq!(
        stale_value["diagnostics"][0]["code"],
        "FERRIS-VALIDATION-TESTED-REVISION-STALE"
    );

    let accepted = revision_validation_output(&repository, &base, &head, &tested);
    assert!(accepted.status.success());
    let accepted_value: Value =
        serde_json::from_slice(&accepted.stdout).expect("accepted merge output");
    assert_eq!(
        accepted_value["record"]["revision_binding"]["relationship"],
        "tested_contains_head"
    );
    assert_eq!(
        accepted_value["record"]["revision_binding"]["changed_path_count"],
        2
    );
    assert_eq!(
        accepted_value["record"]["revision_binding"]["deleted_path_count"],
        0
    );
    assert!(
        accepted_value["record"]["inputs"]
            .as_array()
            .expect("inputs")
            .iter()
            .all(|input| input.get("path_evidence").is_none())
    );
}

#[test]
fn revision_bound_validation_accepts_a_linked_worktree_with_matching_canonical_roots() {
    let (directory, repository, base, head) =
        revision_validation_fixture("revision-linked-worktree");
    let linked = directory.path.join("linked");
    run_git(
        &repository,
        &[
            "worktree",
            "add",
            "--detach",
            linked.to_str().expect("linked worktree path"),
            &head,
        ],
    );
    let main_output = revision_validation_output(&repository, &base, &head, &head);
    assert!(main_output.status.success());
    let output = revision_validation_output(&linked, &base, &head, &head);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let main_value: Value =
        serde_json::from_slice(&main_output.stdout).expect("main worktree JSON");
    let linked_value: Value = serde_json::from_slice(&output.stdout).expect("linked worktree JSON");
    assert_eq!(
        linked_value["record"]["validation_plan_id"],
        main_value["record"]["validation_plan_id"]
    );
    assert_eq!(
        linked_value["record"]["revision_binding"]["revision_binding_id"],
        main_value["record"]["revision_binding"]["revision_binding_id"]
    );
}

#[test]
fn revision_bound_validation_retains_unavailable_working_tree_observation() {
    let (_directory, repository, base, head) =
        revision_validation_fixture("revision-working-tree-not-observed");
    let output = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            repository
                .join("Cargo.toml")
                .to_str()
                .expect("manifest path"),
            "--base-revision",
            &base,
            "--head-revision",
            &head,
            "--tested-revision",
            &head,
            "--owner-domains",
            repository
                .join("owner-domains.json")
                .to_str()
                .expect("owner domains path"),
            "--format",
            "json",
        ])
        .env("GIT_INDEX_FILE", repository.join(".git"))
        .output()
        .expect("run revision-bound validation with unavailable index");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value =
        serde_json::from_slice(&output.stdout).expect("not-observed working-tree JSON");
    assert_eq!(
        value["record"]["revision_binding"]["working_tree"],
        "not_observed"
    );
}

#[test]
fn revision_bound_validation_rejects_a_tested_commit_that_does_not_contain_head() {
    let (_directory, repository, base, head) =
        revision_validation_fixture("revision-divergent-tested");
    run_git(&repository, &["checkout", "--detach", &base]);
    fs::write(
        repository.join("workspace-policy.txt"),
        "divergent tested revision\n",
    )
    .expect("write divergent change");
    run_git(&repository, &["add", "workspace-policy.txt"]);
    run_git(&repository, &["commit", "-m", "divergent tested revision"]);
    let tested = run_git(&repository, &["rev-parse", "HEAD"]);

    let output = revision_validation_output(&repository, &base, &head, &tested);
    assert_eq!(output.status.code(), Some(6));
    let value: Value = serde_json::from_slice(&output.stderr).expect("divergent error JSON");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "FERRIS-VALIDATION-TESTED-REVISION-DOES-NOT-CONTAIN-HEAD"
    );
}

#[test]
fn revision_bound_validation_rejects_a_git_root_above_the_cargo_workspace() {
    let directory = TestDirectory::new("revision-root-mismatch");
    let repository = directory.path.join("repository");
    let workspace = repository.join("nested");
    copy_tree(&fixture("simple-workspace"), &workspace);
    run_git(&repository, &["init", "--initial-branch", "main"]);
    run_git(
        &repository,
        &["config", "user.email", "ferris@example.invalid"],
    );
    run_git(&repository, &["config", "user.name", "Ferris Test"]);
    run_git(&repository, &["add", "."]);
    run_git(&repository, &["commit", "-m", "base"]);
    let base = run_git(&repository, &["rev-parse", "HEAD"]);
    fs::write(
        workspace.join("web/docs/package.json"),
        "{\"name\":\"web-docs\",\"revision\":2}\n",
    )
    .expect("change nested workspace");
    run_git(&repository, &["add", "."]);
    run_git(&repository, &["commit", "-m", "head"]);
    let head = run_git(&repository, &["rev-parse", "HEAD"]);

    let output = revision_validation_output(&workspace, &base, &head, &head);
    assert_eq!(output.status.code(), Some(2));
    let value: Value = serde_json::from_slice(&output.stderr).expect("root mismatch JSON");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "FERRIS-VALIDATION-GIT-CARGO-ROOT-MISMATCH"
    );
}

#[test]
fn revision_options_are_atomic_and_exclusive_with_explicit_inputs() {
    let (_directory, repository, base, head) =
        revision_validation_fixture("revision-argument-contract");
    let partial = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            repository
                .join("Cargo.toml")
                .to_str()
                .expect("manifest path"),
            "--base-revision",
            &base,
            "--head-revision",
            &head,
            "--format",
            "json",
        ])
        .output()
        .expect("run partial revision request");
    assert_eq!(partial.status.code(), Some(2));
    let partial_value: Value = serde_json::from_slice(&partial.stderr).expect("partial error JSON");
    assert_eq!(
        partial_value["diagnostics"][0]["code"],
        "FERRIS-VALIDATION-REVISION-TRIPLE-INCOMPLETE"
    );
    let different_partial = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            repository
                .join("Cargo.toml")
                .to_str()
                .expect("manifest path"),
            "--base-revision",
            &head,
            "--head-revision",
            &head,
            "--format",
            "json",
        ])
        .output()
        .expect("run different partial revision request");
    let different_partial_value: Value =
        serde_json::from_slice(&different_partial.stderr).expect("different partial error JSON");
    assert_ne!(
        different_partial_value["selection_identity"],
        partial_value["selection_identity"]
    );

    let mixed = ferris()
        .args([
            "validation-plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            repository
                .join("Cargo.toml")
                .to_str()
                .expect("manifest path"),
            "--base-revision",
            &base,
            "--head-revision",
            &head,
            "--tested-revision",
            &head,
            "--changed-path",
            repository
                .join("web/docs/package-renamed.json")
                .to_str()
                .expect("changed path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run mixed revision request");
    assert_eq!(mixed.status.code(), Some(2));
    let mixed_value: Value = serde_json::from_slice(&mixed.stderr).expect("mixed error JSON");
    assert_eq!(
        mixed_value["diagnostics"][0]["code"],
        "FERRIS-VALIDATION-REVISION-INPUT-CONFLICT"
    );
    assert_ne!(
        mixed_value["selection_identity"],
        partial_value["selection_identity"]
    );
    let serialized = String::from_utf8(mixed.stderr).expect("mixed error output");
    assert!(!serialized.contains(repository.to_str().expect("repository path")));
}

#[test]
fn explicit_workspace_does_not_discover_sibling() {
    let output = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/selected",
            "--manifest-path",
            fixture("sibling-workspaces/selected/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("plan JSON");
    let packages = value["record"]["packages"].as_array().expect("packages");
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0]["name"], "selected-member");
    assert_eq!(value["record"]["workspace_root"], ".");
    assert_eq!(value["record"]["selected_manifest"], "Cargo.toml");
    assert!(
        !String::from_utf8(output.stdout)
            .expect("utf-8 output")
            .contains("sibling-member")
    );
}

#[test]
fn missing_manifest_returns_fixed_invalid_code() {
    let output = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/missing",
            "--manifest-path",
            fixture("does-not-exist/Cargo.toml")
                .to_str()
                .expect("fixture path"),
        ])
        .output()
        .expect("run ferris");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(value["schema"], "ferris.command-result/v2");
    assert_eq!(value["result_class"], "invalid");
    assert_eq!(value["process_exit_code"], 2);
}

#[test]
fn malformed_manifest_returns_fixed_invalid_code() {
    let output = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/invalid",
            "--manifest-path",
            fixture("invalid-manifest/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.ends_with(b"\n"));

    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(value["result_class"], "invalid");
    assert_eq!(value["process_exit_code"], 2);
    assert_eq!(value["diagnostics"][0]["code"], "FERRIS-MANIFEST-INVALID");
    assert!(
        value["diagnostics"][0]["source_digest"]
            .as_str()
            .expect("source digest")
            .starts_with("sha256:")
    );
    let serialized = String::from_utf8(output.stderr).expect("utf-8 output");
    assert!(
        !serialized.contains(
            fixture("invalid-manifest/Cargo.toml")
                .to_str()
                .expect("fixture path")
        )
    );
}

#[test]
fn locked_resolution_failure_returns_fixed_blocked_code() {
    let output = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/locked",
            "--manifest-path",
            fixture("locked-resolution/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert_eq!(output.status.code(), Some(7));

    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(value["result_class"], "blocked");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "FERRIS-CARGO-METADATA-BLOCKED"
    );
}

#[test]
fn graph_human_exposes_material_edge_semantics() {
    let output = ferris()
        .args([
            "graph",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("utf-8 output");
    assert!(stdout.contains("alias=alpha-dev"));
    assert!(stdout.contains("kind=dev"));
    assert!(stdout.contains("optional=true"));
    assert!(stdout.contains("condition=cfg(windows)"));
    assert!(stdout.contains("resolution=external-unresolved"));
    assert!(stdout.contains("Unknowns:"));
    assert!(stdout.contains("Limitations:"));
    assert!(stdout.contains("Selected manifest: Cargo.toml"));
    assert!(stdout.contains("manifest=alpha/Cargo.toml"));
    assert!(stdout.contains("Evidence: owner=Cargo"));
    assert!(stdout.contains("representation=portable-equivalent"));
    assert!(stdout.contains("output-digest=sha256:"));
    assert!(stdout.contains("Command: cargo metadata"));
}

#[test]
fn json_parse_failure_uses_ferris_envelope() {
    let output = ferris()
        .args(["plan", "--format", "json", "--unknown-option"])
        .output()
        .expect("run ferris");
    assert_eq!(output.status.code(), Some(2));

    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(value["schema"], "ferris.command-result/v2");
    assert_eq!(value["semantic_command_id"], "plan");
    assert_eq!(value["result_class"], "invalid");
    assert_eq!(value["process_exit_code"], 2);
    assert!(
        value["result_identity"]
            .as_str()
            .expect("result identity")
            .starts_with("result:")
    );
    assert!(
        value["selection_identity"]
            .as_str()
            .expect("selection identity")
            .starts_with("selection:")
    );
    assert_eq!(value["diagnostics"][0]["code"], "FERRIS-CLI-INVALID");
}

#[test]
fn cli_parse_failure_without_format_uses_ferris_envelope() {
    let output = ferris()
        .args(["doctor", "--unknown-option"])
        .output()
        .expect("run ferris");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());

    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(value["schema"], "ferris.command-result/v2");
    assert_eq!(value["semantic_command_id"], "doctor");
    assert_eq!(value["result_class"], "invalid");
    assert_eq!(value["process_exit_code"], 2);
    assert!(value["record"].is_null());
}

#[test]
fn doctor_reports_passive_prerequisites_without_paths() {
    let output = ferris()
        .args([
            "doctor",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert!(output.stdout.ends_with(b"\n"));

    let value: Value = serde_json::from_slice(&output.stdout).expect("doctor JSON");
    assert_eq!(value["semantic_command_id"], "doctor");
    assert_eq!(value["record"]["schema"], "ferris.doctor-report/v0");
    assert_eq!(value["record"]["workspace_id"], "ferris.test/simple");
    assert_eq!(value["record"]["checks"].as_array().unwrap().len(), 4);
    assert_eq!(value["record"]["evidence"]["network_requested"], false);
    assert_eq!(value["record"]["evidence"]["owner_work_requested"], false);
    assert_eq!(value["record"]["evidence"]["cargo_network_offline"], true);
    assert_eq!(value["record"]["evidence"]["rustup_auto_install"], false);
    assert_eq!(
        value["record"]["evidence"]["toolchain_selection"],
        "owner-resolution-from-selected-manifest-directory-and-environment"
    );
    assert_eq!(value["record"]["bounds"]["manifest_max_bytes"], 1_048_576);
    assert_eq!(value["record"]["bounds"]["probe_timeout_millis"], 5_000);
    assert_eq!(value["record"]["bounds"]["stdout_max_bytes"], 65_536);
    assert_eq!(value["record"]["bounds"]["stderr_max_bytes"], 65_536);
    assert_eq!(
        value["record"]["bounds"]["owner_output_framing"],
        "length-prefixed-stdout-stderr/v1"
    );
    assert_eq!(
        value["record"]["evidence"]["stdout_retained_bytes"],
        value["record"]["evidence"]["stdout_observed_bytes"]
    );
    assert_eq!(
        value["record"]["evidence"]["stdout_omitted_observed_bytes"],
        0
    );
    assert_eq!(
        value["record"]["evidence"]["stdout_unobserved_bytes_unknown"],
        false
    );
    assert_eq!(value["record"]["evidence"]["stdout_complete"], true);
    assert_eq!(value["record"]["evidence"]["stdout_truncated"], false);
    assert_eq!(
        value["record"]["evidence"]["stderr_retained_bytes"],
        value["record"]["evidence"]["stderr_observed_bytes"]
    );
    assert_eq!(
        value["record"]["evidence"]["stderr_omitted_observed_bytes"],
        0
    );
    assert_eq!(
        value["record"]["evidence"]["stderr_unobserved_bytes_unknown"],
        false
    );
    assert_eq!(value["record"]["evidence"]["stderr_complete"], true);
    assert_eq!(value["record"]["evidence"]["stderr_truncated"], false);
    assert!(
        value["record"]["manifest_digest"]
            .as_str()
            .expect("manifest digest")
            .starts_with("sha256:")
    );

    let serialized = String::from_utf8(output.stdout).expect("utf-8 output");
    assert!(!serialized.contains(r"C:\src\FERRIS"));
    assert!(!serialized.contains("/mnt/c/src/FERRIS"));
    assert!(!serialized.contains("[workspace]"));
}

#[test]
fn doctor_human_exposes_checks_unknowns_and_fallback() {
    let output = ferris()
        .args([
            "doctor",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture path"),
        ])
        .output()
        .expect("run ferris");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("utf-8 output");
    assert!(stdout.contains("Checks:"));
    assert!(stdout.contains("manifest-readable: pass"));
    assert!(stdout.contains("cargo-version-parse: pass"));
    assert!(stdout.contains("Unknowns:"));
    assert!(stdout.contains("Limitations:"));
    assert!(stdout.contains("Command: cargo --version"));
    assert!(stdout.contains("Bounds: manifest-bytes=1048576"));
    assert!(stdout.contains("Fallback:"));
}

#[test]
fn profile_diff_fixture_matrix_covers_all_required_families() {
    let cases = [
        (
            "hosted-service",
            "stages",
            "/sections/stages/deploy/state",
            "HOSTED-SERVICE-RAW",
        ),
        (
            "cli-configuration",
            "lifecycle",
            "/sections/lifecycle/removal/state",
            "CLI-CONFIG-RAW",
        ),
        (
            "pure-data",
            "closure",
            "/sections/closure/active/digest",
            "PURE-DATA-RAW",
        ),
        (
            "embedded-no-std",
            "targets",
            "/sections/targets/thumbv7em-none-eabihf/state",
            "EMBEDDED-RAW",
        ),
        (
            "browser-wasm",
            "targets",
            "/sections/targets/wasm32-unknown-unknown/runtime",
            "BROWSER-WASM-RAW",
        ),
        (
            "wasm-component",
            "identity",
            "/sections/identity/component_contract",
            "WASM-COMPONENT-RAW",
        ),
        (
            "native-dependency",
            "native",
            "/sections/native/openssl/source_mode",
            "NATIVE-DEPENDENCY-RAW",
        ),
        (
            "identity-crypto-provider",
            "providers",
            "/sections/providers/tls/provider",
            "IDENTITY-CRYPTO-RAW",
        ),
        (
            "assurance-packaging-deployment",
            "assurance",
            "/sections/assurance/package_attestation/state",
            "ASSURANCE-PACKAGING-RAW",
        ),
    ];

    for (family, changed_section, changed_path, raw_marker) in cases {
        let before = fixture(&format!("profile-evidence/{family}/before.json"));
        let after = fixture(&format!("profile-evidence/{family}/after.json"));
        let output = ferris()
            .arg("profile-diff")
            .arg("--before")
            .arg(&before)
            .arg("--after")
            .arg(&after)
            .args(["--format", "json"])
            .output()
            .expect("run profile fixture diff");

        assert_eq!(output.status.code(), Some(1), "{family}");
        assert!(output.stderr.is_empty(), "{family}");

        let value: Value = serde_json::from_slice(&output.stdout).expect("profile diff JSON");
        assert_eq!(value["result_class"], "difference", "{family}");
        assert_eq!(
            value["record"]["before"]["profile_id"],
            format!("fixture.{family}"),
            "{family}"
        );
        assert_eq!(
            value["record"]["after"]["profile_id"],
            format!("fixture.{family}"),
            "{family}"
        );
        assert!(
            value["record"]["changed_sections"]
                .as_array()
                .expect("changed sections")
                .iter()
                .any(|section| section == changed_section),
            "{family}"
        );
        assert!(
            value["record"]["changes"]
                .as_array()
                .expect("changes")
                .iter()
                .any(|change| change["path"] == changed_path),
            "{family}"
        );

        let serialized = String::from_utf8(output.stdout).expect("utf-8 output");
        assert!(
            !serialized.contains(&format!("{raw_marker}-BEFORE")),
            "{family}"
        );
        assert!(
            !serialized.contains(&format!("{raw_marker}-AFTER")),
            "{family}"
        );
    }
}

#[test]
fn profile_diff_does_not_mutate_inputs_or_working_directory() {
    let families = [
        "hosted-service",
        "cli-configuration",
        "pure-data",
        "embedded-no-std",
        "browser-wasm",
        "wasm-component",
        "native-dependency",
        "identity-crypto-provider",
        "assurance-packaging-deployment",
    ];
    let working_directory = TestDirectory::new("profile-diff-read-only");

    for family in families {
        let before = fixture(&format!("profile-evidence/{family}/before.json"));
        let after = fixture(&format!("profile-evidence/{family}/after.json"));
        let input_directory = before.parent().expect("profile fixture directory");
        let before_bytes = fs::read(&before).expect("read before fixture");
        let after_bytes = fs::read(&after).expect("read after fixture");
        let before_metadata = fs::metadata(&before).expect("before metadata");
        let after_metadata = fs::metadata(&after).expect("after metadata");
        let input_entries = directory_entries(input_directory);

        let output = ferris()
            .current_dir(&working_directory.path)
            .arg("profile-diff")
            .arg("--before")
            .arg(&before)
            .arg("--after")
            .arg(&after)
            .args(["--format", "json"])
            .output()
            .expect("run read-only profile diff");

        assert_eq!(output.status.code(), Some(1), "{family}");
        assert!(output.stderr.is_empty(), "{family}");
        assert_eq!(fs::read(&before).expect("reread before"), before_bytes);
        assert_eq!(fs::read(&after).expect("reread after"), after_bytes);

        let final_before_metadata = fs::metadata(&before).expect("final before metadata");
        let final_after_metadata = fs::metadata(&after).expect("final after metadata");
        assert_eq!(
            final_before_metadata.len(),
            before_metadata.len(),
            "{family}"
        );
        assert_eq!(final_after_metadata.len(), after_metadata.len(), "{family}");
        assert_eq!(
            final_before_metadata.modified().ok(),
            before_metadata.modified().ok(),
            "{family}"
        );
        assert_eq!(
            final_after_metadata.modified().ok(),
            after_metadata.modified().ok(),
            "{family}"
        );
        assert_eq!(
            directory_entries(input_directory),
            input_entries,
            "{family}"
        );
        assert!(
            directory_entries(&working_directory.path).is_empty(),
            "{family}"
        );
    }
}

#[test]
fn profile_diff_preserves_ordinary_cargo_workflow() {
    let workspace = fixture("profile-cargo-preservation");
    let manifest = workspace.join("Cargo.toml");
    let before_profile = fixture("profile-evidence/pure-data/before.json");
    let after_profile = fixture("profile-evidence/pure-data/after.json");
    let targets = TestDirectory::new("profile-cargo-preservation");
    let initial_workspace = directory_snapshot(&workspace);

    let metadata_before = cargo_metadata(&manifest);
    assert!(metadata_before.status.success());
    assert!(metadata_before.stderr.is_empty());
    let metadata_before_json: Value =
        serde_json::from_slice(&metadata_before.stdout).expect("before Cargo metadata JSON");
    assert_eq!(directory_snapshot(&workspace), initial_workspace);

    let test_before = cargo_test(&manifest, &targets.path.join("before-target"));
    assert!(
        test_before.status.success(),
        "{}",
        String::from_utf8_lossy(&test_before.stderr)
    );
    assert!(
        String::from_utf8_lossy(&test_before.stdout).contains("1 passed"),
        "{}",
        String::from_utf8_lossy(&test_before.stdout)
    );
    assert_eq!(directory_snapshot(&workspace), initial_workspace);

    let diff = ferris()
        .current_dir(&workspace)
        .arg("profile-diff")
        .arg("--before")
        .arg(&before_profile)
        .arg("--after")
        .arg(&after_profile)
        .args(["--format", "json"])
        .output()
        .expect("run profile diff from Cargo consumer");
    assert_eq!(diff.status.code(), Some(1));
    assert!(diff.stderr.is_empty());
    assert_eq!(directory_snapshot(&workspace), initial_workspace);

    let metadata_after = cargo_metadata(&manifest);
    assert!(metadata_after.status.success());
    assert_eq!(metadata_after.stderr, metadata_before.stderr);
    let metadata_after_json: Value =
        serde_json::from_slice(&metadata_after.stdout).expect("after Cargo metadata JSON");
    assert_eq!(metadata_after_json, metadata_before_json);
    assert_eq!(directory_snapshot(&workspace), initial_workspace);

    let test_after = cargo_test(&manifest, &targets.path.join("after-target"));
    assert!(
        test_after.status.success(),
        "{}",
        String::from_utf8_lossy(&test_after.stderr)
    );
    assert!(
        String::from_utf8_lossy(&test_after.stdout).contains("1 passed"),
        "{}",
        String::from_utf8_lossy(&test_after.stdout)
    );
    assert_eq!(directory_snapshot(&workspace), initial_workspace);
}

#[test]
fn doctor_rejects_non_manifest_files() {
    let non_manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../AGENTS.md");
    let output = ferris()
        .args([
            "doctor",
            "--workspace-id",
            "ferris.test/not-a-manifest",
            "--manifest-path",
            non_manifest.to_str().expect("non-manifest path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run ferris");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.ends_with(b"\n"));

    let value: Value = serde_json::from_slice(&output.stderr).expect("error JSON");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "FERRIS-DOCTOR-MANIFEST-NAME-INVALID"
    );
    assert!(
        !String::from_utf8(output.stderr)
            .expect("utf-8 output")
            .contains(non_manifest.to_str().expect("non-manifest path"))
    );
}
