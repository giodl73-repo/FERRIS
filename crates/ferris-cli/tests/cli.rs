use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(path)
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
    let status = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/missing",
            "--manifest-path",
            fixture("does-not-exist/Cargo.toml")
                .to_str()
                .expect("fixture path"),
        ])
        .status()
        .expect("run ferris");
    assert_eq!(status.code(), Some(2));
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
    assert_eq!(value["schema"], "ferris.command-result/v1");
    assert_eq!(value["semantic_command_id"], "plan");
    assert_eq!(value["result_class"], "invalid");
    assert_eq!(value["process_exit_code"], 2);
    assert!(
        value["result_identity"]
            .as_str()
            .expect("result identity")
            .starts_with("result:")
    );
    assert_eq!(value["diagnostics"][0]["code"], "FERRIS-CLI-INVALID");
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
