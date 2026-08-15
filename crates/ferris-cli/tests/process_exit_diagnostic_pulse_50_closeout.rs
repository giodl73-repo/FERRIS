use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const AUTHORITY_COMMIT: &str = "48fe9fdcdda03378f68781cae342796c9f11720d";
const CUTOFF: &str = "94d473563a1686091be94a72f491b0ff0d903800";
const DECLARATION_IDENTITY: &str =
    "sha256:b87a3041085bffe66688dff6b675b89839a43ac55a54fe7731769cee92e05f4d";
const DECLARATION: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-50-authority.json";
const SCHEMA: &str = "docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-50-authority.v1.schema.json";
const MUTATIONS: &str = "docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-50-authority-mutations.json";
const PULSE_51_RELEASE_COMMIT: &str = "d09c923c1e2cd2be003026597f4ad2a0e2d3764f";
const PULSE_51_RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-51-diagnostic-executor-release";
const PULSE_51_MANIFEST_RAW: &str =
    "sha256:5799416be367b9293c85fbbc3db0a340184045a1a33e75c9d7a4351d5741efbc";
const PULSE_51_MANIFEST_AGGREGATE: &str =
    "sha256:18d61962245d75e42fed30f581555a5b436e0a83d89e3383d059dca035e978e6";
const PULSE_51_RECEIPT_RAW: &str =
    "sha256:ef2b423520e1f2680c0cadd246a51c0af1a4502f45d757f018982f42c326f1c9";
const PULSE_51_RECEIPT_PAYLOAD: &str =
    "sha256:77408aabd377801c3c578a889523c18ee95eb286ac55b04df6c30f74d45ef452";
const PULSE_51_SEAL_RAW: &str =
    "sha256:968f495555b4617329318686b5adb460faf3fe95a07c8da160e163c9395eb767";
const PULSE_51_SEAL_PAYLOAD: &str =
    "sha256:1d22ad1248a2f47c78984d8020c3c6507253c468b53f30073efcfb5ab880c0d4";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read public artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse JSON");
    (bytes, value)
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
}

fn git_output(args: &[&str]) -> Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(args)
        .output()
        .expect("run Git")
}

fn git_text(args: &[&str]) -> String {
    let output = git_output(args);
    assert!(
        output.status.success(),
        "Git command failed {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("Git output is UTF-8")
        .trim()
        .to_owned()
}

fn git_blob(revision: &str, relative_path: &str) -> Vec<u8> {
    let spec = format!("{revision}:{relative_path}");
    let output = git_output(&["show", &spec]);
    assert!(
        output.status.success(),
        "missing Git blob {spec}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn collect_files(root: &Path, directory: &Path, files: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let entry = entry.expect("release directory entry");
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).expect("release metadata");
        assert!(
            !metadata.file_type().is_symlink(),
            "{path:?} must not be a symlink"
        );
        if metadata.is_dir() {
            collect_files(root, &path, files);
        } else {
            assert!(metadata.is_file(), "{path:?} must be a regular file");
            files.insert(
                path.strip_prefix(root)
                    .expect("release-relative path")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn assert_zero_execution_state(state: &Value) {
    for (field, value) in state.as_object().expect("execution state object") {
        match value {
            Value::Number(number) => assert_eq!(
                number.as_u64(),
                Some(0),
                "execution state {field} must remain zero"
            ),
            Value::Bool(value) => assert!(!value, "execution state {field} must remain false"),
            Value::Null => {}
            value => panic!("execution state {field} has non-prelaunch value {value}"),
        }
    }
}

fn assert_no_pulse_50_result_or_witness(paths: Vec<String>, label: &str) {
    let prohibited = paths
        .into_iter()
        .filter(|path| {
            let path = path.to_ascii_lowercase();
            (path.contains("pulse-50") || path.contains("pulse_50"))
                && (path.contains("result") || path.contains("witness"))
        })
        .collect::<Vec<_>>();
    assert!(
        prohibited.is_empty(),
        "{label} must not retain Pulse 50 result/witness roots: {prohibited:?}"
    );
}

#[test]
fn pulse_50_is_permanently_withdrawn_before_launch_when_pulse_51_is_post_cutoff() {
    let authority_revision = format!("{AUTHORITY_COMMIT}^{{commit}}");
    let cutoff_revision = format!("{CUTOFF}^{{commit}}");
    let pulse_51_revision = format!("{PULSE_51_RELEASE_COMMIT}^{{commit}}");
    assert_eq!(
        git_text(&["rev-parse", &authority_revision]),
        AUTHORITY_COMMIT
    );
    assert_eq!(git_text(&["rev-parse", &cutoff_revision]), CUTOFF);
    assert_eq!(
        git_text(&["rev-parse", &pulse_51_revision]),
        PULSE_51_RELEASE_COMMIT
    );
    assert_eq!(
        git_text(&["merge-base", CUTOFF, AUTHORITY_COMMIT]),
        CUTOFF,
        "the immutable cutoff predates its authority"
    );
    let pulse_51_parent = format!("{PULSE_51_RELEASE_COMMIT}^");
    assert_eq!(
        git_text(&["rev-parse", &pulse_51_parent]),
        AUTHORITY_COMMIT,
        "Pulse 51 is published after the historical authority"
    );

    let root = repo_root();
    for path in [DECLARATION, SCHEMA, MUTATIONS] {
        assert_eq!(
            read_lf(root.join(path)),
            git_blob(AUTHORITY_COMMIT, path),
            "{path} must remain an exact historical authority artifact"
        );
    }

    let (declaration_bytes, declaration) = read_json(root.join(DECLARATION));
    assert_eq!(
        sha256(&declaration_bytes),
        sha256(&git_blob(AUTHORITY_COMMIT, DECLARATION))
    );
    assert_eq!(
        declaration["declaration_identity"], DECLARATION_IDENTITY,
        "the historical declaration identity remains bound"
    );
    assert_eq!(declaration["status"], "authorized-unexecuted");
    assert_eq!(declaration["immutable_ferris"]["cutoff"], CUTOFF);
    assert_eq!(
        declaration["immutable_ferris"]["authority_present_at_cutoff"],
        false
    );
    assert_zero_execution_state(&declaration["execution_state"]);
    for field in [
        "candidate_processes",
        "materializer_invocations",
        "private_materialization_invocations",
        "pulse_50_search_processes",
        "pulse_47_witness_invocations",
        "pulse_43_invocations_via_pulse_47",
        "pulse_43_direct_terminal_publisher_invocations",
        "publication_attempts",
    ] {
        assert_eq!(
            declaration["execution_state"][field], 0,
            "{field} remains unconsumed"
        );
    }
    for field in [
        "private_seed_created",
        "custody_workspace_created",
        "p43_result_root_path_sets_observed_in_actual_main_workspace",
        "pulse_47_witness_root_path_sets_observed_in_actual_main_workspace",
        "pulse_47_witness_transaction_published",
    ] {
        assert_eq!(
            declaration["execution_state"][field], false,
            "{field} remains absent"
        );
    }
    for field in [
        "category_conclusion",
        "external_public_summary",
        "reproducer",
        "result_receipt",
        "terminal_disposition",
    ] {
        assert_eq!(
            declaration["execution_state"][field],
            Value::Null,
            "{field} remains null"
        );
    }
    assert_eq!(
        declaration["authority"]["pulse_50_private_runtime_roots_committed"],
        false
    );
    assert_eq!(declaration["authority"]["pulse_50_inference"], false);

    let authority_paths = git_text(&["ls-tree", "-r", "--name-only", AUTHORITY_COMMIT])
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert_no_pulse_50_result_or_witness(authority_paths, "historical authority");
    let release_paths = git_text(&["ls-tree", "-r", "--name-only", PULSE_51_RELEASE_COMMIT])
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert_no_pulse_50_result_or_witness(release_paths, "Pulse 51 release");
    let current_paths = git_text(&["ls-files"])
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert_no_pulse_50_result_or_witness(current_paths, "current checkout");

    let release = root.join(PULSE_51_RELEASE);
    let release_prefix = format!("{PULSE_51_RELEASE}/");
    let expected_paths = git_text(&[
        "ls-tree",
        "-r",
        "--name-only",
        PULSE_51_RELEASE_COMMIT,
        "--",
        PULSE_51_RELEASE,
    ])
    .lines()
    .map(|path| {
        path.strip_prefix(&release_prefix)
            .expect("Pulse 51 release-relative path")
            .to_owned()
    })
    .collect::<BTreeSet<_>>();
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(
        actual_paths, expected_paths,
        "the complete Pulse 51 release tree remains at its release commit"
    );
    for path in &actual_paths {
        assert_eq!(
            read_lf(release.join(path)),
            git_blob(
                PULSE_51_RELEASE_COMMIT,
                &format!("{PULSE_51_RELEASE}/{path}")
            ),
            "Pulse 51 release file {path} remains sealed"
        );
    }

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), PULSE_51_MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-51-diagnostic-executor-public-manifest/v1"
    );
    assert_eq!(manifest["aggregate"], PULSE_51_MANIFEST_AGGREGATE);
    assert_eq!(manifest["release_tree_file_count"], 17);

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), PULSE_51_RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], PULSE_51_RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        PULSE_51_RECEIPT_PAYLOAD
    );
    assert_eq!(receipt["payload"]["ferris_executed"], false);
    assert_eq!(receipt["payload"]["private_seed_created"], false);
    assert_eq!(
        receipt["payload"]["p43_terminal_publication_invocations"],
        0
    );
    assert_eq!(
        receipt["payload"]["p47_terminal_publication_invocations"],
        0
    );

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), PULSE_51_SEAL_RAW);
    assert_eq!(seal["payload_sha256"], PULSE_51_SEAL_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&seal["payload"]),
        PULSE_51_SEAL_PAYLOAD
    );
    assert_eq!(
        seal["payload"]["manifest"]["raw_sha256"],
        PULSE_51_MANIFEST_RAW
    );
    assert_eq!(
        seal["payload"]["qualification_receipt"]["raw_sha256"],
        PULSE_51_RECEIPT_RAW
    );
    assert_eq!(
        seal["payload"]["release_limits"]["diagnostic_execution"],
        false
    );
    assert_eq!(
        seal["payload"]["release_limits"]["private_seed_created"],
        false
    );
    assert_eq!(
        seal["payload"]["release_limits"]["terminal_publication_invocations"],
        0
    );

    let closeout = fs::read_to_string(
        root.join("context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-50.md"),
    )
    .expect("read UTF-8 Pulse 50 closeout");
    for required in [
        "invalid-prelaunch-infrastructure-integrity",
        "P50-EXECUTOR-RELEASE-OUTSIDE-AUTHORITY-CUTOFF",
        "prelaunch-public-infrastructure",
        AUTHORITY_COMMIT,
        CUTOFF,
        PULSE_51_RELEASE_COMMIT,
    ] {
        assert!(closeout.contains(required), "closeout must bind {required}");
    }
}
