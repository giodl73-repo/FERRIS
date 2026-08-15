use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release";
const MANIFEST_RAW: &str =
    "sha256:807fed0ca1f630ea07d15bfad64ee4d0fb7d8f578c64be5ee48b1d975c4ba02a";
const MANIFEST_AGGREGATE: &str =
    "sha256:728cdbf64c520869b36cb902a9ca2dcadb88e5ff4ff734ff054ff05e9851a400";
const QUALIFICATION_RAW: &str =
    "sha256:9fd2368dc6c123707da40b6f9eefd04f8a680e40635c4f539db6818d34f19d98";
const QUALIFICATION_PAYLOAD: &str =
    "sha256:6006f98a103cd822dc51fb2e8297e3755848fea72e4ec50e15ca6cb04a83f8d5";
const SEAL_RAW: &str = "sha256:cbb2fc8eeaf82b90f5275dd1e8ed406c0ab215d52d8233824dd9c9af390755a4";
const SEAL_PAYLOAD: &str =
    "sha256:cbad676d88ec32ae53466946332385f5895b58274de82fb6e8ff4bd14a111747";

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
        .expect("run Pulse 56 Python validation")
}

#[test]
fn pulse_56_retained_build_custody_release_is_sealed_and_non_executing() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "generate_release.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "retained_build_custody.py".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-56-retained-build-receipt.v1.schema.json".to_owned(),
        "tests/test_retained_build_custody.py".to_owned(),
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
        "ferris.pulse-56-retained-build-custody-manifest/v1"
    );
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(aggregate(files), MANIFEST_AGGREGATE);
    assert_eq!(files.len() as u64, manifest["file_count"].as_u64().unwrap());
    assert_eq!(
        manifest["release_tree_file_count"],
        actual_paths.len() as u64
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

    let (qualification_bytes, qualification) =
        read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&qualification_bytes), QUALIFICATION_RAW);
    assert_eq!(qualification["payload_sha256"], QUALIFICATION_PAYLOAD);
    assert_eq!(qualification["receipt_id"], QUALIFICATION_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&qualification["payload"]),
        QUALIFICATION_PAYLOAD
    );
    assert_eq!(
        qualification["schema"],
        "ferris.pulse-56-retained-build-custody-qualification-envelope/v1"
    );
    assert_eq!(qualification["payload"]["ferris_executed"], false);
    for platform in ["windows", "ubuntu_24_04_wsl"] {
        let receipt = &qualification["payload"]["actual_probes"][platform]["qualification_receipt"];
        assert_eq!(receipt["payload"]["artifact"]["retained"], true);
        assert_eq!(receipt["payload"]["reproducibility"]["builds"], 2);
        assert_eq!(
            receipt["payload"]["reproducibility"]["artifact_bytes_identical"],
            true
        );
        assert_eq!(
            receipt["payload"]["reproducibility"]["distinct_checkout_roots"],
            true
        );
        assert_eq!(
            receipt["payload"]["reproducibility"]["distinct_target_roots"],
            true
        );
        assert_eq!(receipt["payload"]["safety"]["ferris_executed"], false);
        assert_eq!(
            receipt["payload"]["safety"]["public_receipt_is_evidence_only"],
            true
        );
        let toolchain = &receipt["payload"]["build"]["toolchain"];
        assert!(toolchain["selected_toolchain"].as_str().is_some());
        assert_ne!(
            toolchain["cargo_direct"]["file_sha256"],
            toolchain["rustup_selector"]["file_sha256"]
        );
        assert_ne!(
            toolchain["rustc_direct"]["file_sha256"],
            toolchain["rustup_selector"]["file_sha256"]
        );
    }
    assert_eq!(
        qualification["payload"]["actual_probes"]["windows"]["qualification_receipt"]["payload"]["build"]
            ["toolchain"]["linker"]["route"],
        "rust-toolchain-shipped-rust-lld"
    );
    assert_eq!(
        qualification["payload"]["actual_probes"]["ubuntu_24_04_wsl"]["qualification_receipt"]["payload"]
            ["build"]["toolchain"]["linker"]["route"],
        "bound-ubuntu-cc-collect2-gnu-ld-trace"
    );
    assert!(
        qualification["payload"]["actual_probes"]["ubuntu_24_04_wsl"]["qualification_receipt"]
            ["payload"]["build"]["toolchain"]["linker"]["actual_trace_selected_inputs"]
            .as_array()
            .expect("Ubuntu linker trace inputs")
            .len()
            >= 3
    );
    let controls = qualification["payload"]["synthetic"]["negative_controls"]
        .as_array()
        .expect("synthetic controls");
    assert!(
        controls
            .iter()
            .any(|value| value == "copied-or-forged-live-handle")
    );
    assert!(
        controls
            .iter()
            .any(|value| value == "native-linux-fd-inode-launch-after-path-mutation")
    );
    for required in [
        "early-close-exact-owned-root-cleanup",
        "active-close-refusal",
        "concurrent-last-use-cleanup-exactly-once",
        "fatal-cleanup-failure-is-not-completed-process",
        "substituted-runtime-root-refusal",
        "windows-os-handle-single-ownership",
    ] {
        assert!(
            controls.iter().any(|value| value == required),
            "missing lifecycle control {required}"
        );
    }
    assert_eq!(
        qualification["payload"]["synthetic"]["native_wsl_launch_path_qualified"],
        true
    );

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(seal["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);
    assert_eq!(
        seal["payload"]["qualification_receipt"]["raw_sha256"],
        QUALIFICATION_RAW
    );
    assert_eq!(
        seal["payload"]["qualification_receipt"]["payload_sha256"],
        QUALIFICATION_PAYLOAD
    );
    assert_eq!(seal["payload"]["scope"]["diagnostic_executor"], false);
    assert_eq!(seal["payload"]["scope"]["ferris_executed"], false);

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
