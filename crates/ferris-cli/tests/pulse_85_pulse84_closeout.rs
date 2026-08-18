use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CUTOFF: &str = "f874ebfe29e58460fc0a553418d11d6785e84df9";
const AUTHORITY_IDENTITY: &str =
    "sha256:f5a76eebaa70d5c07de53e25fa34287083e166e29e0ede5f682732fd6dd1da5f";
const CLOSEOUT_IDENTITY: &str =
    "sha256:bd7819d1c0def32e55ef982050b7fef726f2bbd9740765862664dfc428d15c3d";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read LF artifact");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&read_lf(path)).expect("parse JSON")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn git_output(arguments: &[String]) -> std::process::Output {
    Command::new("git")
        .current_dir(repo_root())
        .args(arguments)
        .output()
        .expect("run Git")
}

#[test]
fn pulse_85_records_the_single_consumed_call_and_exact_stop() {
    let root = repo_root();
    let envelope = read_json(root.join(
        "docs/simulations/profile-diff-held-out/pulse-85-pulse84-closeout/PULSE-85-PUBLIC-CLOSEOUT.json",
    ));
    assert_eq!(
        sha256(&serde_json::to_vec(&envelope["payload"]).expect("payload bytes")),
        CLOSEOUT_IDENTITY
    );
    assert_eq!(envelope["payload_sha256"], CLOSEOUT_IDENTITY);
    assert_eq!(envelope["receipt_id"], CLOSEOUT_IDENTITY);

    let payload = &envelope["payload"];
    assert_eq!(
        payload["authority"]["declaration_identity"],
        AUTHORITY_IDENTITY
    );
    assert_eq!(payload["authority"]["cutoff"], CUTOFF);
    assert_eq!(payload["authority"]["consumptions"], 1);
    assert_eq!(payload["authority"]["callable_invocation_attempts"], 1);
    assert_eq!(payload["authority"]["callable_invocations"], 1);
    assert_eq!(payload["authority"]["retry_permitted"], false);
    assert_eq!(payload["authority"]["resume_permitted"], false);
    assert_eq!(payload["result"]["status"], "permanently-closed");
    assert_eq!(payload["result"]["disposition"], "not-attempted");
    assert_eq!(
        payload["result"]["stage"],
        "ubuntu-capability-build-custody"
    );
    assert_eq!(payload["result"]["failure_code"], "P57-WSL-BUNDLE");
    assert_eq!(
        payload["result"]["last_completed_gate"],
        "windows-capability-build-custody"
    );
    assert_eq!(
        payload["result"]["failed_gate"],
        "ubuntu-capability-build-custody"
    );
    assert_eq!(payload["result"]["transfer_descriptor_present"], false);
}

#[test]
fn pulse_85_preserves_zero_downstream_activity_null_conclusions_and_cleanup() {
    let root = repo_root();
    let envelope = read_json(root.join(
        "docs/simulations/profile-diff-held-out/pulse-85-pulse84-closeout/PULSE-85-PUBLIC-CLOSEOUT.json",
    ));
    let payload = &envelope["payload"];
    for field in [
        "seed_calls",
        "seed_bytes",
        "p27_invocations",
        "materializer_invocations",
        "verifier_invocations",
        "windows_candidate_processes",
        "ubuntu_candidate_processes",
        "result_transfers",
        "failure_witness_transfers",
    ] {
        assert_eq!(payload["counts"][field], 0, "{field} must remain zero");
    }
    for conclusion in payload["conclusions"]
        .as_object()
        .expect("conclusions")
        .values()
    {
        assert!(conclusion.is_null());
    }
    for cleanup in payload["cleanup"]
        .as_object()
        .expect("cleanup")
        .iter()
        .filter(|(key, _)| *key != "pulse_41_custody_files_recorded")
        .map(|(_, value)| value)
    {
        assert_eq!(*cleanup, Value::Bool(true));
    }
    assert_eq!(payload["cleanup"]["pulse_41_custody_files_recorded"], 8);
    assert_eq!(payload["publication"]["attempted"], false);
    assert_eq!(payload["publication"]["public_result_created"], false);
    assert_eq!(payload["publication"]["public_witness_created"], false);
    assert!(!root
        .join("docs/simulations/profile-diff-held-out/pulse-84-public-result")
        .exists());
    assert!(!root
        .join("docs/simulations/profile-diff-held-out/pulse-84-publication-witness")
        .exists());
}

#[test]
fn pulse_85_is_public_safe_and_canonical_state_closes_authority() {
    let root = repo_root();
    let closeout_path = root.join(
        "docs/simulations/profile-diff-held-out/pulse-85-pulse84-closeout/PULSE-85-PUBLIC-CLOSEOUT.json",
    );
    let closeout = String::from_utf8(read_lf(&closeout_path)).expect("UTF-8 closeout");
    for forbidden in ["C:\\\\", "/home/"] {
        assert!(
            !closeout.contains(forbidden),
            "public closeout leaked {forbidden}"
        );
    }

    let wave = fs::read_to_string(
        root.join("context/waves/2026-08-12-platform-profile-conformance/WAVE.md"),
    )
    .expect("wave");
    let context = fs::read_to_string(root.join("CONTEXT.md")).expect("context");
    let review = fs::read_to_string(
        root.join("docs/plans/reviews/PULSE-85-PULSE84-CLOSEOUT-ROLE-REVIEW.md"),
    )
    .expect("review");
    for required in [
        "Pulse 84 is permanently consumed",
        "P57-WSL-BUNDLE",
        "all conclusions remain null",
    ] {
        assert!(
            format!("{wave}\n{context}").contains(required),
            "missing canonical closeout term {required}"
        );
    }
    for required in [
        "Completed revisions",
        "Remaining gates",
        "Implementation authority",
    ] {
        assert!(review.contains(required), "missing review field {required}");
    }

    let cutoff_exists = git_output(&[
        "cat-file".to_owned(),
        "-e".to_owned(),
        format!("{CUTOFF}^{{commit}}"),
    ]);
    assert!(cutoff_exists.status.success());
}
