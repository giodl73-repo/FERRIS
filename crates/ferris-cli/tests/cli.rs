use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(path)
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
