use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-45-binary-custody-event-bridge-release";
const MANIFEST_RAW: &str =
    "sha256:f8574972a8dc7791580d26dcf17a0ffcb0c55024e8d753616dcbba7c592dd544";
const MANIFEST_AGGREGATE: &str =
    "sha256:4a6c3fb5093aeff681c62636e36b78dc581e2491672207bbc64ecf0e01bd434d";
const RECEIPT_RAW: &str = "sha256:40b9dac86b496be10dd550e9119fa250f70a0acd6f63b019fd66c6496c1086ce";
const RECEIPT_PAYLOAD: &str =
    "sha256:fb7049852a417baaa2afd41decd26b508ad5727d6e2252a05d4f79ab44989bd9";
const SEAL_RAW: &str = "sha256:7a087787d040103643436c2b6bee5bb58f803d1a5c0a897d9cb9f8e935f75c86";
const SEAL_PAYLOAD: &str =
    "sha256:f39e38597f479467bc5f154a17edb8b1a97e5df8aa7d6c3dca0e755019dc4588";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
}

fn python() -> PathBuf {
    env::var_os("PYTHON")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("python"))
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
}

fn read_lf(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let bytes = fs::read(path).expect("read release file");
    assert!(!bytes.contains(&b'\r'), "{path:?} must contain no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse release JSON");
    (bytes, value)
}

fn decode_sha256(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64, "SHA-256 hexadecimal length");
    let mut bytes = [0_u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).expect("hex digest");
    }
    bytes
}

fn manifest_aggregate(files: &[Value]) -> String {
    let mut ordered = files.to_vec();
    ordered.sort_by_key(|file| file["path"].as_str().expect("manifest path").to_owned());
    let mut hasher = Sha256::new();
    for file in ordered {
        let path = file["path"].as_str().expect("manifest path");
        hasher.update((path.len() as u64).to_be_bytes());
        hasher.update(path.as_bytes());
        hasher.update(decode_sha256(
            file["sha256"]
                .as_str()
                .expect("manifest digest")
                .strip_prefix("sha256:")
                .expect("SHA-256 prefix"),
        ));
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn collect_files(root: &Path, directory: &Path, files: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let entry = entry.expect("release entry");
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

fn run_composition(release: &Path, runtime: &Path) -> Value {
    let program = r#"
import importlib.util, json, os, sys
from pathlib import Path

release = Path(os.environ["P45_RELEASE"])
runtime = Path(os.environ["P45_RUNTIME"])
sys.path.insert(0, str(release))
import binary_custody_event_bridge as bridge

def load(name, source):
    spec = importlib.util.spec_from_file_location(name, source)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module

p43 = load(
    "pulse_45_rust_p43",
    release.parent / "pulse-43-ordered-result-publisher-release" / "ordered_result_publisher.py",
)
published = json.loads((release / "fixtures" / "pulse-44-published-summary.json").read_text())
failed = json.loads((release / "fixtures" / "pulse-44-failed-summary.json").read_text())
calls = []
def invoke(repo, cutoff, platform, work_root, final_root):
    calls.append((repo, cutoff, platform, work_root, final_root))
    return published

windows = bridge.bridge_pulse_44(
    runtime, "29517d732db13cc2ffa304684b344f3538ab587d", "windows-x86_64",
    runtime / "windows-work", runtime / "windows-final", invoker=invoke)
ubuntu = bridge.bridge_pulse_44(
    runtime, "29517d732db13cc2ffa304684b344f3538ab587d", "ubuntu-24.04-x86_64",
    runtime / "ubuntu-work", runtime / "ubuntu-final", invoker=invoke)
catalog = {
    "schema": "ferris.pulse-43-ordered-gate-catalog/v1",
    "gate_ids": [
        "windows-retained-binary-custody",
        "ubuntu-retained-binary-custody",
        "later-public-gate",
    ],
}
later = {
    "classification": "ordered-execution",
    "event_kind": "terminal-stop",
    "gate_id": "later-public-gate",
    "outcome": "completed",
    "schema": "ferris.pulse-43-ordered-result-event/v1",
}
complete, _ = p43.build_result(
    catalog,
    [windows["ordered_execution_event"], ubuntu["ordered_execution_event"], later],
)
failure = bridge.bridge_pulse_44(
    runtime, "29517d732db13cc2ffa304684b344f3538ab587d", "windows-x86_64",
    runtime / "failed-work", runtime / "failed-final", invoker=lambda *_: failed)
try:
    p43.build_result(catalog, [failure["ordered_execution_event"], later])
except p43.PublicFailure as error:
    failure_code = error.code
else:
    raise AssertionError("a later gate must not follow a terminal failure")
print(json.dumps({
    "calls": len(calls),
    "complete_gates": complete["ordered_execution"]["completed_gate_count"],
    "failure_code": failure_code,
    "ubuntu_event": ubuntu["ordered_execution_event"],
    "windows_event": windows["ordered_execution_event"],
}, sort_keys=True))
"#;
    let output = Command::new(python())
        .arg("-c")
        .arg(program)
        .env("P45_RELEASE", release)
        .env("P45_RUNTIME", runtime)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 45 composition");
    assert!(
        output.status.success(),
        "composition failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("composition JSON")
}

#[test]
fn pulse_45_binary_custody_event_bridge_release_is_sealed_and_composable() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "binary_custody_event_bridge.py".to_owned(),
        "fixtures/pulse-44-failed-summary.json".to_owned(),
        "fixtures/pulse-44-published-summary.json".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "release-seal.json".to_owned(),
        "schemas/ferris.pulse-45-binary-custody-event-bridge.v1.schema.json".to_owned(),
        "tests/test_binary_custody_event_bridge.py".to_owned(),
    ]);
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths);
    for path in &actual_paths {
        read_lf(release.join(path));
    }

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-45-binary-custody-event-bridge-public-manifest/v1"
    );
    assert_eq!(manifest["file_count"], 6);
    assert_eq!(manifest["release_tree_file_count"], 9);
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len(), 6);
    assert_eq!(manifest_aggregate(files), MANIFEST_AGGREGATE);
    let mut total = 0_u64;
    for file in files {
        let path = file["path"].as_str().expect("manifest path");
        assert!(!Path::new(path).is_absolute());
        assert!(!path.contains(".."));
        let bytes = read_lf(release.join(path));
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("manifest size")
        );
        assert_eq!(sha256(&bytes), file["sha256"], "{path} binding");
        total += bytes.len() as u64;
    }
    assert_eq!(total, 49_769);
    assert_eq!(manifest["total_bytes"], 49_769);

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    assert_eq!(
        receipt["payload"]["qualification"]["python_test_methods"],
        14
    );
    assert_eq!(
        receipt["payload"]["qualification"]["success_platform_mappings"],
        2
    );
    assert_eq!(
        receipt["payload"]["qualification"]["failed_platform_terminations"],
        2
    );
    assert_eq!(receipt["payload"]["qualification"]["retries"], 0);
    assert_eq!(receipt["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(seal["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);
    assert_eq!(
        seal["payload"]["qualification_receipt"]["payload_sha256"],
        RECEIPT_PAYLOAD
    );

    let python_test = Command::new(python())
        .arg(release.join("tests/test_binary_custody_event_bridge.py"))
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 45 Python qualification");
    assert!(
        python_test.status.success(),
        "Python qualification failed: stdout={} stderr={}",
        String::from_utf8_lossy(&python_test.stdout),
        String::from_utf8_lossy(&python_test.stderr)
    );
    assert!(
        String::from_utf8_lossy(&python_test.stderr).contains("Ran 14 tests"),
        "qualification count: {}",
        String::from_utf8_lossy(&python_test.stderr)
    );

    let runtime = root
        .join("target")
        .join(format!("pulse-45-rust-{}", std::process::id()));
    if runtime.exists() {
        fs::remove_dir_all(&runtime).expect("clean Rust integration directory");
    }
    fs::create_dir_all(&runtime).expect("create Rust integration directory");
    let composition = run_composition(&release, &runtime);
    assert_eq!(composition["calls"], 2);
    assert_eq!(composition["complete_gates"], 3);
    assert_eq!(composition["failure_code"], "P43-ORDERED-AFTER-TERMINAL");
    assert_eq!(
        composition["windows_event"],
        json!({
            "classification": "ordered-execution",
            "event_kind": "gate-complete",
            "gate_id": "windows-retained-binary-custody",
            "outcome": "passed",
            "schema": "ferris.pulse-43-ordered-result-event/v1",
        })
    );
    assert_eq!(
        composition["ubuntu_event"],
        json!({
            "classification": "ordered-execution",
            "event_kind": "gate-complete",
            "gate_id": "ubuntu-retained-binary-custody",
            "outcome": "passed",
            "schema": "ferris.pulse-43-ordered-result-event/v1",
        })
    );
    fs::remove_dir_all(&runtime).expect("remove Rust integration directory");
}
