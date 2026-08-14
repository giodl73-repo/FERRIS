use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-47-publication-outcome-witness-release";
const MANIFEST_RAW: &str =
    "sha256:44d5c72b9eb09dc7e24b476a4535fed662eadde3edee6ecbfe1fdfa644082f8b";
const MANIFEST_AGGREGATE: &str =
    "sha256:5cb97276ee2752888c40d44a50e45079c9e550f7e26398e5aa4841d98083143d";
const RECEIPT_RAW: &str = "sha256:be73ee9a87377e58a87c04308557ef118afbb7ed0fb117b039cc569f9040b265";
const RECEIPT_PAYLOAD: &str =
    "sha256:dbe44afbb9f0ad43549113028da8dc5d2d0ca5fe9faa15824d7cd80e3edea355";
const SEAL_RAW: &str = "sha256:4300f5ba89bdaefb938b91092adf7d1c62dbf11ba6e1a4350c9a34c03cce1a8e";
const SEAL_PAYLOAD: &str =
    "sha256:a00478e73897781ddd88e8e0fcbca2d1453a72758cbbd8ec06ccd9d0c228f681";

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

fn run_cases(release: &Path, runtime: &Path) -> Value {
    let program = r#"
import importlib.util, json, os, sys
from pathlib import Path

release = Path(os.environ["P47_RELEASE"])
runtime = Path(os.environ["P47_RUNTIME"])
sys.path.insert(0, str(release))
import publication_outcome_witness as p47

def load(name, source):
    spec = importlib.util.spec_from_file_location(name, source)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module

p43_root = release.parent / "pulse-43-ordered-result-publisher-release"
p43 = load("pulse_47_rust_p43", p43_root / "ordered_result_publisher.py")
catalog = json.loads((p43_root / "fixtures" / "public-gate-catalog.json").read_text())
events = json.loads((p43_root / "fixtures" / "complete-events.json").read_text())
indeterminate = json.loads(
    (release / "fixtures" / "pulse-43-indeterminate-summary.json").read_text()
)
published = json.loads(
    (release / "fixtures" / "pulse-43-published-summary.json").read_text()
)

real_calls = []
def invoke_real(catalog_value, events_value, final_root):
    real_calls.append(final_root)
    return p47.invoke_real_pulse_43(catalog_value, events_value, final_root)

success = p47.witness_pulse_43(
    catalog, events, runtime / "p43-success", runtime / "witness-success",
    invoker=invoke_real)

indeterminate_calls = []
def invoke_indeterminate(*_):
    indeterminate_calls.append(1)
    return indeterminate

indeterminate_result = p47.witness_pulse_43(
    catalog, events, runtime / "p43-indeterminate", runtime / "witness-indeterminate",
    invoker=invoke_indeterminate)

failure_calls = []
def invoke_failure(*_):
    failure_calls.append(1)
    return published

def fail_writer(*_):
    raise OSError("injected witness write failure")

witness_failure = p47.witness_pulse_43(
    catalog, events, runtime / "p43-witness-failure", runtime / "witness-failure",
    invoker=invoke_failure, writer=fail_writer)

success_root = runtime / "witness-success"
raw_witness = (success_root / "publication-witness.json").read_bytes()
raw_receipt = (success_root / "release-receipt.json").read_bytes()
print(json.dumps({
    "indeterminate": indeterminate_result,
    "indeterminate_calls": len(indeterminate_calls),
    "persistent": {
        "files": sorted(item.name for item in success_root.iterdir()),
        "receipt_raw_sha256": p47.sha256_bytes(raw_receipt),
        "witness_raw_sha256": p47.sha256_bytes(raw_witness),
    },
    "real_calls": len(real_calls),
    "success": success,
    "witness_failure": witness_failure,
    "witness_failure_calls": len(failure_calls),
}, sort_keys=True))
"#;
    let output = Command::new(python())
        .arg("-c")
        .arg(program)
        .env("P47_RELEASE", release)
        .env("P47_RUNTIME", runtime)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 47 representative cases");
    assert!(
        output.status.success(),
        "cases failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("cases JSON")
}

#[test]
fn pulse_47_publication_outcome_witness_release_is_sealed_and_persistent() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "fixtures/pulse-43-indeterminate-summary.json".to_owned(),
        "fixtures/pulse-43-published-summary.json".to_owned(),
        "publication_outcome_witness.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "release-seal.json".to_owned(),
        "schemas/ferris.pulse-47-publication-outcome-witness.v1.schema.json".to_owned(),
        "tests/test_publication_outcome_witness.py".to_owned(),
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
        "ferris.pulse-47-publication-outcome-witness-public-manifest/v1"
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
    assert_eq!(total, 64_779);
    assert_eq!(manifest["total_bytes"], 64_779);

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
        17
    );
    assert_eq!(
        receipt["payload"]["qualification"]["failure_control_test_methods"],
        16
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
        .arg(release.join("tests/test_publication_outcome_witness.py"))
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 47 Python qualification");
    assert!(
        python_test.status.success(),
        "Python qualification failed: stdout={} stderr={}",
        String::from_utf8_lossy(&python_test.stdout),
        String::from_utf8_lossy(&python_test.stderr)
    );
    assert!(
        String::from_utf8_lossy(&python_test.stderr).contains("Ran 17 tests"),
        "qualification count: {}",
        String::from_utf8_lossy(&python_test.stderr)
    );

    let runtime = root
        .join("target")
        .join(format!("pulse-47-rust-{}", std::process::id()));
    if runtime.exists() {
        fs::remove_dir_all(&runtime).expect("clean Rust integration directory");
    }
    fs::create_dir_all(&runtime).expect("create Rust integration directory");
    let cases = run_cases(&release, &runtime);
    let success = &cases["success"];
    assert_eq!(cases["real_calls"], 1);
    assert_eq!(success["outcome"], "published");
    assert_eq!(success["witness_publication"]["files"], "2/2");
    assert_eq!(success["witness_publication"]["rename_attempts"], 1);
    assert_eq!(success["witness_publication"]["retries"], 0);
    assert_eq!(
        success["publication_outcome"]["kind"], "published",
        "real Pulse 43 success is witnessed"
    );
    assert_eq!(
        cases["persistent"]["files"],
        serde_json::json!(["publication-witness.json", "release-receipt.json"])
    );
    assert_eq!(
        cases["persistent"]["witness_raw_sha256"],
        success["witness_publication"]["raw_hashes"]["witness_raw_sha256"]
    );
    assert_eq!(
        cases["persistent"]["receipt_raw_sha256"],
        success["witness_publication"]["raw_hashes"]["receipt_raw_sha256"]
    );

    let indeterminate = &cases["indeterminate"];
    assert_eq!(cases["indeterminate_calls"], 1);
    assert_eq!(indeterminate["outcome"], "published");
    assert_eq!(
        indeterminate["publication_outcome"]["failure_code"],
        "P43-INDETERMINATE-PUBLICATION"
    );
    assert_eq!(
        indeterminate["publication_outcome"]["publication"]["state"],
        "indeterminate"
    );
    assert!(
        indeterminate["publication_outcome"]
            .get("ordered_execution")
            .is_none()
    );

    let witness_failure = &cases["witness_failure"];
    assert_eq!(cases["witness_failure_calls"], 1);
    assert_eq!(
        witness_failure["failure_code"],
        "P47-WITNESS-STAGE-COPY-FAILURE"
    );
    assert_eq!(witness_failure["witness_publication"]["state"], "absent");
    assert!(witness_failure.get("publication_outcome").is_none());
    assert!(witness_failure.get("pulse_43").is_none());

    fs::remove_dir_all(&runtime).expect("remove Rust integration directory");
}
