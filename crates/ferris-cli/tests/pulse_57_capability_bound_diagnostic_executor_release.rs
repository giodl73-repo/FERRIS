use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-57-capability-bound-diagnostic-executor-release";
const MANIFEST_RAW: &str =
    "sha256:455a029ed9cfcfea6a47b80b6bf8631760654d7d0e636d1d30f36ddcac1d0291";
const MANIFEST_AGGREGATE: &str =
    "sha256:ea0afe873e138cbe3ab9148ac0effd4b5defc94ebdf33ada4a6a3c0b468b1b46";
const RECEIPT_RAW: &str = "sha256:8f0b35bd61bd147bbb43e898f0f817936b035688e3eb889d7530d8fc1b6a3a5d";
const RECEIPT_PAYLOAD: &str =
    "sha256:5cedec87b57e350d3ab11245c09b9cd7be1f485682d88cb9c1190a939f6bd134";
const SEAL_RAW: &str = "sha256:b18407fd2def541486405d18e2dd92b9bb343e5e9aeaa2899f3ed4f312b68ea8";
const SEAL_PAYLOAD: &str =
    "sha256:727e1806ede5ca9b5438b5f3b00bec3a59b4e36404ccc1b72e6adb661ebee144";

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
    let hex = value.strip_prefix("sha256:").expect("SHA-256 prefix");
    assert_eq!(hex.len(), 64, "SHA-256 hex length");
    let mut bytes = [0_u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("hex");
    }
    bytes
}

fn aggregate(files: &[Value]) -> String {
    let mut ordered = files.to_vec();
    ordered.sort_by_key(|value| value["path"].as_str().expect("manifest path").to_owned());
    let mut hasher = Sha256::new();
    for file in ordered {
        let path = file["path"].as_str().expect("manifest path");
        hasher.update((path.len() as u64).to_be_bytes());
        hasher.update(path.as_bytes());
        hasher.update(decode_sha256(
            file["sha256"].as_str().expect("manifest digest"),
        ));
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn collect_files(root: &Path, directory: &Path, output: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let entry = entry.expect("release entry");
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).expect("release metadata");
        assert!(
            !metadata.file_type().is_symlink(),
            "{path:?} must not be symlink"
        );
        if metadata.is_dir() {
            assert_ne!(
                entry.file_name(),
                "__pycache__",
                "sealed tree has Python residue"
            );
            collect_files(root, &path, output);
        } else {
            assert!(metadata.is_file(), "{path:?} must be regular");
            output.insert(
                path.strip_prefix(root)
                    .expect("release-relative path")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("canonical payload"))
}

fn python_output(release: &Path, arguments: &[&str]) -> std::process::Output {
    Command::new(python())
        .current_dir(release)
        .args(arguments)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 57 Python validation")
}

#[test]
fn pulse_57_capability_bound_diagnostic_executor_release_is_sealed_and_fake_only() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "capability_bound_executor.py".to_owned(),
        "fixtures/fake_p56.py".to_owned(),
        "fixtures/p57-public-gate-catalog.json".to_owned(),
        "generate_release.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-57-capability-bound-diagnostic-executor.v1.schema.json".to_owned(),
        "sealed_dependencies.py".to_owned(),
        "tests/test_capability_bound_executor.py".to_owned(),
        "wsl_session_worker.py".to_owned(),
    ]);
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths);
    for path in &actual_paths {
        read_lf(release.join(path));
    }
    let (_, catalog) = read_json(release.join("fixtures/p57-public-gate-catalog.json"));
    assert_eq!(
        catalog["gate_ids"],
        serde_json::json!([
            "sealed-predecessor-binding",
            "windows-capability-build-custody",
            "ubuntu-capability-build-custody",
            "exact-adapter-preflight",
            "pulse-31-public-input",
            "pulse-35-pulse-37-normalization",
            "descriptor-validation",
            "bounded-process-exit-search"
        ])
    );
    let worker =
        String::from_utf8(read_lf(release.join("wsl_session_worker.py"))).expect("worker is UTF-8");
    let executor = String::from_utf8(read_lf(release.join("capability_bound_executor.py")))
        .expect("executor is UTF-8");
    assert!(
        executor.contains("\"-I\"") && executor.contains("\"-S\"") && executor.contains("\"-B\""),
        "worker must use isolated Python"
    );
    assert!(
        !worker.contains("from sealed_dependencies import"),
        "worker must verify staged local dependencies before importing them"
    );
    assert!(
        worker.contains("sys.dont_write_bytecode = True") && !worker.contains("importlib.util"),
        "worker imports must stay bytecode-free and path-loader-free"
    );
    assert!(
        executor.contains("bound_release_files")
            && executor.contains("_WSL_WORKER_BOOTSTRAP")
            && executor.contains("\"/proc/self/fd/\"")
            && executor.contains("\"-c\""),
        "WSL worker must compile the staged worker from a verified held descriptor"
    );

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-57-capability-bound-diagnostic-executor-manifest/v1"
    );
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len() as u64, manifest["file_count"].as_u64().unwrap());
    assert_eq!(aggregate(files), MANIFEST_AGGREGATE);
    assert_eq!(
        manifest["release_tree_file_count"],
        actual_paths.len() as u64,
        "manifest binds complete release tree"
    );
    let mut total = 0_u64;
    for file in files {
        let path = file["path"].as_str().expect("manifest path");
        assert!(!Path::new(path).is_absolute());
        assert!(!path.contains(".."));
        let bytes = read_lf(release.join(path));
        assert_eq!(bytes.len() as u64, file["size"].as_u64().unwrap());
        assert_eq!(sha256(&bytes), file["sha256"], "{path} hash");
        total += bytes.len() as u64;
    }
    assert_eq!(total, manifest["total_bytes"].as_u64().unwrap());

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    assert_eq!(receipt["payload"]["cycles_required"], 20);
    assert_eq!(receipt["payload"]["cycles_passed"], 20);
    assert_eq!(receipt["payload"]["fake_launches_total"], 2_760);
    assert_eq!(receipt["payload"]["ferris_executed"], false);
    assert_eq!(receipt["payload"]["negative_control_tests_run"], 22);
    assert_eq!(receipt["payload"]["negative_control_tests_passed"], 22);
    assert_eq!(
        receipt["payload"]["negative_control_test_ids"]
            .as_array()
            .expect("negative controls")
            .len(),
        22
    );
    assert_eq!(receipt["payload"]["p44_p45_execution_invocations"], 0);
    assert_eq!(
        receipt["payload"]["windows_p56_capability_publishes_per_cycle"],
        1
    );
    assert_eq!(
        receipt["payload"]["ubuntu_p56_capability_publishes_per_cycle"],
        1
    );
    assert_eq!(
        receipt["payload"]["alternating_cycle_modes"],
        serde_json::json!(["alpha", "beta"])
    );
    let cycles = receipt["payload"]["cycles"]
        .as_array()
        .expect("qualification cycles");
    assert_eq!(cycles.len(), 20);
    assert_eq!(cycles[0]["cycle_mode"], "alpha");
    assert_eq!(cycles[1]["cycle_mode"], "beta");
    let artifacts = cycles
        .iter()
        .map(|cycle| {
            cycle["fake_artifact_sha256"]
                .as_str()
                .expect("fake artifact digest")
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(artifacts.len(), 2, "two alternating valid fake artifacts");

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(
        seal["payload"]["scope"]["p44_p45_execution"], false,
        "Pulse 57 must not claim retired bridge execution"
    );
    assert_eq!(
        seal["payload"]["predecessors"]["pulse_51"]["manifest"],
        "sha256:5799416be367b9293c85fbbc3db0a340184045a1a33e75c9d7a4351d5741efbc"
    );
    assert_eq!(
        seal["payload"]["predecessors"]["pulse_56"]["manifest"],
        "sha256:807fed0ca1f630ea07d15bfad64ee4d0fb7d8f578c64be5ee48b1d975c4ba02a"
    );

    let unit = python_output(
        &release,
        &["-B", "-m", "unittest", "discover", "-s", "tests", "-v"],
    );
    assert!(
        unit.status.success(),
        "Python tests failed: stdout={} stderr={}",
        String::from_utf8_lossy(&unit.stdout),
        String::from_utf8_lossy(&unit.stderr)
    );
}
