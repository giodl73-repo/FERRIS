use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CUTOFF: &str = "dfc889b178e1737bc816595b49b5c9f66de14691";
const PULSE_82_COMMIT: &str = "4549aef5748345bb3e17e2234c51f7ec460061d3";
const PULSE_82_ROOT: &str = "docs/simulations/profile-diff-held-out/pulse-82-witness-preserving-capability-materialization-executor-pulse35-release-tree-successor-release";
const MANIFEST_RAW: &str =
    "sha256:7b08a16a3c6b07bf3759a54ea98d4cb887c3f2789d8fc25569356836f05266fd";
const MANIFEST_AGGREGATE: &str =
    "sha256:a6a529e5ca960a519852e048f21320ce45d9a9da7be73074498f578d9f7ae0c2";
const SEAL_RAW: &str = "sha256:0f57a5601dd24ae51cee2e54eca584c34cdac17fecb72499b6dcfe483bb71efd";
const SOURCE_RAW: &str = "sha256:20a85b3009d2a75eba8684a4d17a3be24f16d832b34928cb21d59ebd1a0f8543";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn git(arguments: &[String]) -> std::process::Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(arguments)
        .output()
        .expect("run Git")
}

fn cutoff_blob(relative: &str) -> Vec<u8> {
    let output = git(&["show".to_owned(), format!("{CUTOFF}:{relative}")]);
    assert!(output.status.success(), "missing cutoff blob {relative}");
    output.stdout
}

fn cutoff_json(relative: &str) -> (Vec<u8>, Value) {
    let bytes = cutoff_blob(relative);
    let value = serde_json::from_slice(&bytes).expect("parse cutoff JSON");
    (bytes, value)
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn assert_ancestor(commit: &str) {
    assert!(
        git(&[
            "merge-base".to_owned(),
            "--is-ancestor".to_owned(),
            commit.to_owned(),
            CUTOFF.to_owned(),
        ])
        .status
        .success(),
        "{commit} must precede {CUTOFF}"
    );
}

fn assert_cutoff_absent(relative: &str) {
    assert!(
        !git(&[
            "cat-file".to_owned(),
            "-e".to_owned(),
            format!("{CUTOFF}:{relative}"),
        ])
        .status
        .success(),
        "readiness cutoff must exclude {relative}"
    );
}

fn cutoff_tree_paths() -> BTreeSet<String> {
    let output = git(&[
        "ls-tree".to_owned(),
        "-r".to_owned(),
        "--name-only".to_owned(),
        CUTOFF.to_owned(),
        "--".to_owned(),
        PULSE_82_ROOT.to_owned(),
    ]);
    assert!(output.status.success(), "list Pulse 82 cutoff tree");
    let prefix = format!("{PULSE_82_ROOT}/");
    String::from_utf8(output.stdout)
        .expect("UTF-8 Git paths")
        .lines()
        .map(|path| {
            path.strip_prefix(&prefix)
                .expect("release-relative path")
                .to_owned()
        })
        .collect()
}

#[test]
fn pulse_83_binds_exact_pulse_82_cutoff_release() {
    assert_ancestor(PULSE_82_COMMIT);

    let manifest_path = format!("{PULSE_82_ROOT}/public-manifest.json");
    let seal_path = format!("{PULSE_82_ROOT}/release-seal.json");
    let source_path =
        format!("{PULSE_82_ROOT}/witness_preserving_capability_materialization_executor.py");
    let (manifest_bytes, manifest) = cutoff_json(&manifest_path);
    let (seal_bytes, seal) = cutoff_json(&seal_path);
    let source = cutoff_blob(&source_path);

    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(manifest["file_count"], 13);
    assert_eq!(manifest["release_tree_file_count"], 15);
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);
    assert_eq!(seal["payload"]["manifest"]["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(sha256(&source), SOURCE_RAW);

    let paths = cutoff_tree_paths();
    assert_eq!(paths.len(), 15);
    assert!(paths.contains("public-manifest.json"));
    assert!(paths.contains("release-seal.json"));
    for file in manifest["files"].as_array().expect("manifest files") {
        let relative = file["path"].as_str().expect("manifest path");
        assert!(paths.contains(relative), "missing release file {relative}");
        let bytes = cutoff_blob(&format!("{PULSE_82_ROOT}/{relative}"));
        assert_eq!(bytes.len() as u64, file["size"]);
        assert_eq!(sha256(&bytes), file["sha256"]);
    }

    let source = String::from_utf8(source).expect("UTF-8 Pulse 82 source");
    assert!(source.contains("def run_witness_preserving_capability_materialization_executor(\n"));
    assert!(source.contains("repo_root: Path,"));
    assert!(source.contains("ubuntu_runtime_parent: str,"));
}

#[test]
fn pulse_83_is_static_self_excluding_readiness_only() {
    for relative in [
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-83.md",
        "docs/simulations/profile-diff-held-out/PULSE_83_AUTHORITY_READINESS.md",
        "docs/plans/reviews/PULSE-83-POST-PULSE82-AUTHORITY-READINESS-ROLE-REVIEW.md",
        "crates/ferris-cli/tests/pulse_83_authority_readiness.rs",
    ] {
        assert_cutoff_absent(relative);
    }

    let readiness = fs::read_to_string(
        repo_root().join("docs/simulations/profile-diff-held-out/PULSE_83_AUTHORITY_READINESS.md"),
    )
    .expect("read Pulse 83 readiness");
    for required in [
        "ready-for-separate-authority-drafting",
        "Authority: none",
        "Diagnostic execution: none",
        "working-tree or runtime truth",
        "SLSA provenance",
        "does not itself grant permission to execute",
    ] {
        assert!(
            readiness.contains(required),
            "missing readiness term {required}"
        );
    }
}

#[test]
fn pulse_83_maps_every_pulse_68_blocker_to_a_sealed_successor() {
    let readiness = fs::read_to_string(
        repo_root().join("docs/simulations/profile-diff-held-out/PULSE_83_AUTHORITY_READINESS.md"),
    )
    .expect("read Pulse 83 readiness");
    for required in [
        "P68-P57-STAGED-BUNDLE-CLEANUP",
        "Pulse 69",
        "Pulse 72",
        "Pulse 75",
        "Pulse 78",
        "Pulse 81",
        "Pulse 82",
        "A future pulse must use a later immutable cutoff",
    ] {
        assert!(
            readiness.contains(required),
            "missing blocker mapping {required}"
        );
    }
}
