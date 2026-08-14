use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-44-retained-binary-custody-release";
const MANIFEST_RAW: &str =
    "sha256:eae4db6c4add7f20a919cd301dc307cc7845f808f458219b5627c135ed5f0c94";
const MANIFEST_AGGREGATE: &str =
    "sha256:a22efbbb233ee53550c8ac9771a83af3829c16ce8f7f7a2ff15638adf2f58f94";
const RECEIPT_RAW: &str = "sha256:d17ac162d7e8d5afb9f41fa789afe43c2512f2ee1dd30b4afaae4bde16491f1b";
const RECEIPT_PAYLOAD: &str =
    "sha256:a5a5be3d0832476ba0addb4edda2790d3e02acda49a1266601e6065bc0f9cf29";
const SEAL_RAW: &str = "sha256:97598062129317e89862407cc00971aa11ac179420088f4d508678b535cab2a8";
const SEAL_PAYLOAD: &str =
    "sha256:4b90c678255fe3567760ce2ef253192a5489ee684ae57a4eb15446f038c189b5";

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

fn run_synthetic_adapter(release: &Path, runtime: &Path) -> Value {
    let program = r#"
import hashlib, json, os, sys
from pathlib import Path
release = Path(os.environ["P44_RELEASE"])
runtime = Path(os.environ["P44_RUNTIME"])
sys.path.insert(0, str(release))
import retained_binary_custody as m

def build(retained=True):
    def invoke(repo, cutoff, platform, output, *, retain_executable):
        assert retain_executable is True
        output.mkdir()
        name = m._logical_filename(platform, cutoff)
        binary = b"bounded rust integration executable\n"
        (output / name).write_bytes(binary)
        payload = {
            "artifact": {"discovery": "cargo-compiler-artifact-json", "logical_filename": name,
                         "retained_in_public_bundle": retained,
                         "sha256": m.sha256_bytes(binary), "size": len(binary)},
            "build": {"binary": "ferris", "command": ["cargo", "build", "--locked", "--release",
                      "--package", "ferris-cli", "--bin", "ferris"],
                      "package": "ferris-cli", "profile": "release"},
            "checkout": {"core_autocrlf": False, "exact_commit": True, "tracked_files_clean": True},
            "cutoff": cutoff, "platform": platform,
            "safety": {"diagnostic_execution": False, "product_files_modified": False},
            "schema": "ferris.public-build-freeze-receipt/v1",
        }
        receipt = {"payload": payload, "payload_sha256": m.sha256_bytes(m.canonical_bytes(payload)),
                   "schema": "ferris.public-build-freeze-envelope/v1"}
        (output / (name + ".receipt.json")).write_bytes(
            json.dumps(receipt, sort_keys=True).encode("utf-8") + b"\n")
        return receipt
    return invoke

def sync(_):
    return m.SyncPosture("synced", m.SYNC_MECHANISM, None)

success = m.retain_binary_custody(
    Path.cwd(), m.P33_CUTOFF, "windows-x86_64", runtime / "work",
    runtime / "final", builder=build(), synchronizer=sync)
failure = m.retain_binary_custody(
    Path.cwd(), m.P33_CUTOFF, "windows-x86_64", runtime / "bad-work",
    runtime / "bad-final", builder=build(False), synchronizer=sync)
print(json.dumps({"success": success, "failure": failure}, sort_keys=True))
"#;
    let output = Command::new(python())
        .arg("-c")
        .arg(program)
        .env("P44_RELEASE", release)
        .env("P44_RUNTIME", runtime)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 44 synthetic adapter");
    assert!(
        output.status.success(),
        "synthetic adapter failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("synthetic adapter JSON")
}

#[test]
fn pulse_44_retained_binary_custody_release_is_sealed_and_mutation_controlled() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "fixtures/synthetic-build-receipt.json".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "release-seal.json".to_owned(),
        "retained_binary_custody.py".to_owned(),
        "schemas/ferris.pulse-44-retained-binary-custody.v1.schema.json".to_owned(),
        "tests/test_retained_binary_custody.py".to_owned(),
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
        "ferris.pulse-44-retained-binary-custody-public-manifest/v1"
    );
    assert_eq!(manifest["file_count"], 5);
    assert_eq!(manifest["release_tree_file_count"], 8);
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len(), 5);
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
    assert_eq!(
        total,
        manifest["total_bytes"].as_u64().expect("manifest total")
    );

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
        29
    );
    assert_eq!(receipt["payload"]["qualification"]["retries"], 0);
    assert_eq!(receipt["payload"]["actual_windows"]["attempts"], 2);
    assert_eq!(
        receipt["payload"]["actual_windows"]["dirty_checkout_rejections"],
        1
    );
    assert_eq!(
        receipt["payload"]["actual_windows"]["final_custody_published"],
        1
    );
    assert_eq!(
        receipt["payload"]["actual_windows"]["verified_files"],
        "2/2"
    );
    assert_eq!(
        receipt["payload"]["actual_windows"]["runtime_roots_absent"],
        true
    );

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
        .arg(release.join("tests/test_retained_binary_custody.py"))
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 44 Python qualification");
    assert!(
        python_test.status.success(),
        "Python qualification failed: stdout={} stderr={}",
        String::from_utf8_lossy(&python_test.stdout),
        String::from_utf8_lossy(&python_test.stderr)
    );
    assert!(
        String::from_utf8_lossy(&python_test.stderr).contains("Ran 29 tests"),
        "qualification count: {}",
        String::from_utf8_lossy(&python_test.stderr)
    );

    let runtime = root
        .join("target")
        .join(format!("pulse-44-rust-{}", std::process::id()));
    if runtime.exists() {
        fs::remove_dir_all(&runtime).expect("clean Rust integration directory");
    }
    fs::create_dir_all(&runtime).expect("create Rust integration directory");
    let results = run_synthetic_adapter(&release, &runtime);
    let success = &results["success"];
    assert_eq!(success["outcome"], "published");
    assert_eq!(success["custody"]["files"], "2/2");
    assert_eq!(success["custody"]["rename_attempts"], 1);
    assert_eq!(success["custody"]["retries"], 0);
    assert_eq!(
        success["ordered_execution_event"],
        json!({
            "classification": "ordered-execution",
            "event_kind": "terminal-stop",
            "gate_id": "retained-binary-custody",
            "outcome": "completed",
            "schema": "ferris.pulse-43-ordered-result-event/v1",
        })
    );
    assert!(runtime.join("final").is_dir());
    assert!(!runtime.join("work").exists());

    let failure = &results["failure"];
    assert_eq!(failure["outcome"], "failed");
    assert_eq!(failure["failure_code"], "P44-WORK-VERIFY-FAILURE");
    assert_eq!(failure["custody"]["state"], "absent");
    assert_eq!(failure["custody"]["retries"], 0);
    assert_eq!(failure["ordered_execution_event"]["outcome"], "failed");
    assert!(!runtime.join("bad-final").exists());
    assert!(!runtime.join("bad-work").exists());
    fs::remove_dir_all(&runtime).expect("remove Rust integration directory");
}
