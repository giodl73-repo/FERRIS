use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str = "docs/simulations/profile-diff-held-out/pulse-81-ordered-capability-materialization-executor-pulse35-release-tree-successor-release";
const MANIFEST_RAW: &str =
    "sha256:9f8eb930a2b150bba5c65ff820633fc7cd7ade882e30877459eebe1ddea8bfee";
const MANIFEST_AGGREGATE: &str =
    "sha256:82b23e689c3984aab143ffccb4e489d7a8d97baacfd99a232790803e54a65be9";
const RECEIPT_RAW: &str = "sha256:e00c69c70c680ebd202a1ed90b0d840dc6448e0b49cc3323af4971080aafb7e9";
const RECEIPT_PAYLOAD: &str =
    "sha256:9272246e38a15c414ff4273391ff61ff4c6138d477969c4c4dfcf14c2790afe8";
const SEAL_RAW: &str = "sha256:8be65961f0c5d95d2ba2917e163eba966745744e3e3c76d426160f5d22db247a";
const SEAL_PAYLOAD: &str =
    "sha256:93c0a1630fb814094ab70bf55339f8898a0d79d15e14cf5af6bf44038a59f881";
const P78_COMMIT: &str = "7c60f6b384bec1494fbbabaa244b8f0f8eece355";

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
    let value = serde_json::from_slice(&bytes).expect("parse JSON");
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
            assert_ne!(entry.file_name(), "__pycache__", "no Python residue");
            collect_files(root, &path, output);
        } else {
            assert!(metadata.is_file(), "{path:?} must be regular");
            output.insert(
                path.strip_prefix(root)
                    .expect("relative path")
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("canonical JSON payload"))
}

fn python_output(release: &Path, arguments: &[&str]) -> std::process::Output {
    Command::new(python())
        .current_dir(release)
        .args(arguments)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 81 Python validation")
}

#[test]
fn pulse_81_ordered_capability_materialization_executor_stage_capture_bootstrap_argv_successor_is_sealed_and_fake_only(
) {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "fixtures/__init__.py".to_owned(),
        "fixtures/fake_p56.py".to_owned(),
        "fixtures/p52_synthetic_fixture.py".to_owned(),
        "fixtures/p81-public-gate-catalog.json".to_owned(),
        "generate_release.py".to_owned(),
        "ordered_capability_materialization_executor.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-81-ordered-capability-materialization-executor-pulse35-release-tree-successor.v1.schema.json"
            .to_owned(),
        "sealed_dependencies.py".to_owned(),
        "tests/test_ordered_capability_materialization_executor.py".to_owned(),
    ]);
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths);
    for path in &actual_paths {
        read_lf(release.join(path));
    }

    let source = String::from_utf8(read_lf(
        release.join("ordered_capability_materialization_executor.py"),
    ))
    .expect("UTF-8 executor");
    assert!(source.contains("def run_ordered_capability_materialization_executor("));
    assert!(source.contains("LOCAL_SEALED_DEPENDENCIES_SHA256"));
    assert!(source.contains("_LOCAL_SEALED_DEPENDENCIES_RUNTIME_PREFIX"));
    assert!(source.contains("uuid.uuid4().hex"));
    assert!(source.contains("load_exact_p78_stack"));
    assert!(source.contains("load_exact_p78_stack = _SEALED.load_exact_p78_stack"));
    assert!(source.contains("P58-INDETERMINATE-CLEANUP"));
    assert!(source.contains("_verify_public_prelaunch_custody"));
    assert!(source.contains("_execute_p57_semantics"));
    assert!(!source.contains("from sealed_dependencies import"));
    assert!(!source.contains("load_exact_p69_stack"));

    let sealed_source = String::from_utf8(read_lf(release.join("sealed_dependencies.py")))
        .expect("UTF-8 sealed dependencies");
    assert!(sealed_source.contains("P78 = ReleaseIdentity("));
    assert!(sealed_source.contains("P78_COMMIT"));
    assert!(sealed_source.contains("load_exact_p39_and_p41"));
    assert!(sealed_source.contains("load_exact_p52_stage_reader"));
    assert!(sealed_source.contains("load_exact_p35_materializer_and_verifier"));
    assert!(sealed_source.contains("load_exact_p78_stack"));
    assert!(sealed_source.contains("release_identities"));
    assert!(sealed_source.contains("P35 = ReleaseIdentity("));
    assert!(sealed_source.contains("P35_VERIFIER_SOURCE_SHA256"));
    assert!(sealed_source.contains("pulse_78"));
    assert!(sealed_source.contains("P81-SEALED-LOCK-CROSS-INSTANCE-REENTRY"));
    assert!(!sealed_source.contains("load_exact_p69_stack"));
    assert!(!sealed_source.contains("P35_MATERIALIZER_HASHES"));
    assert!(!sealed_source.contains("P35_VERIFIER_HASHES"));

    let (_, catalog) = read_json(release.join("fixtures/p81-public-gate-catalog.json"));
    assert_eq!(
        catalog["gate_ids"],
        serde_json::json!([
            "pulse-41-pulse-39-public-custody",
            "sealed-predecessor-binding",
            "windows-capability-build-custody",
            "ubuntu-capability-build-custody",
            "exact-adapter-preflight",
            "pulse-31-public-input",
            "pulse-35-pulse-37-normalization",
            "bounded-materialization",
            "descriptor-validation",
            "bounded-process-exit-search"
        ])
    );

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-81-ordered-capability-materialization-executor-pulse35-release-tree-successor-manifest/v1"
    );
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len() as u64, manifest["file_count"].as_u64().unwrap());
    assert_eq!(aggregate(files), MANIFEST_AGGREGATE);
    assert_eq!(
        manifest["release_tree_file_count"],
        actual_paths.len() as u64
    );
    let mut total = 0_u64;
    for file in files {
        let path = file["path"].as_str().expect("manifest path");
        let bytes = read_lf(release.join(path));
        assert_eq!(bytes.len() as u64, file["size"].as_u64().unwrap());
        assert_eq!(sha256(&bytes), file["sha256"], "{path} digest");
        total += bytes.len() as u64;
    }
    assert_eq!(total, manifest["total_bytes"].as_u64().unwrap());
    assert_eq!(
        manifest["predecessors"]["pulse_78"]["manifest"],
        "sha256:f5cd77f0f31474677bbc847a75e0d307b9a41e7b313dcf2be22a9135fb99b7fb"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_78"]["seal"],
        "sha256:a56612188fccec0c7f4cb7b0d40922f10d4128265d500878ca3ace6202e77a55"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_78"]["commit"],
        P78_COMMIT
    );
    assert_eq!(
        manifest["predecessors"]["pulse_78"]["release_root"],
        "docs/simulations/profile-diff-held-out/pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-release"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_35"]["aggregate"],
        "sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_35"]["receipt"],
        "sha256:4c4f4ad1d9fa437e23f655083eb74c754114c5bea43ae111d2127fc7f051a037"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_35"]["receipt_payload"],
        "sha256:7f1154ca94009cef966ab2f43ba74a9f017989ed5dbbdbfd8c3ce8fe64fe5cee"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_35"]["seal"],
        "sha256:17459123c674f2664d7d09ea03c00dcba72129bb1cf532cfe11f8cf4edeffd23"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_35"]["seal_payload"],
        "sha256:834781867ea008dc14a54d7b811002ee1b8fa759c0b1d7f32432ea6c0d5c5375"
    );

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        receipt["schema"],
        "ferris.pulse-81-ordered-capability-materialization-executor-pulse35-release-tree-successor-qualification-envelope/v1"
    );
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    let payload = &receipt["payload"];
    assert_eq!(payload["cycles_required"], 20);
    assert_eq!(payload["cycles_passed"], 20);
    assert_eq!(payload["fake_launches_total"], 2_760);
    assert_eq!(payload["ferris_executed"], false);
    assert_eq!(payload["negative_control_tests_run"], 28);
    assert_eq!(payload["negative_control_tests_passed"], 28);
    assert_eq!(payload["exact_p35_complete_release_tree_verified"], true);
    assert_eq!(payload["exact_p78_binding_verified"], true);
    assert_eq!(payload["local_loader_explicit_binding_verified"], true);
    assert_eq!(payload["fresh_module_loading_verified"], true);
    assert_eq!(
        payload["transitive_sealed_loading_serialization_verified"],
        true
    );
    assert_eq!(
        payload["kernel_lock_cross_process_serialization_verified"],
        true
    );
    assert_eq!(payload["private_material_disclosed"], false);
    assert_eq!(payload["p39_execution_scope"], "exact-p39-semantics-only");
    assert_eq!(payload["p44_p45_execution_invocations"], 0);
    assert_eq!(payload["publication_invocations"], 0);
    let control_ids = payload["negative_control_test_ids"]
        .as_array()
        .expect("receipt control IDs");
    assert_eq!(control_ids.len(), 28);
    let actual_control_ids = control_ids
        .iter()
        .map(|value| value.as_str().expect("receipt control ID").to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_control_ids,
        BTreeSet::from([
            "capability-built-before-seed".to_owned(),
            "concurrent-100-complete-load-graph-serialized".to_owned(),
            "directory-substitution-rejected".to_owned(),
            "directory-symlink-and-wsl-no-follow".to_owned(),
            "exact-p78-binding-and-local-loader".to_owned(),
            "final-cleanup".to_owned(),
            "first-semantic-mismatch-stop".to_owned(),
            "local-loader-fresh-modules".to_owned(),
            "no-launch-topology".to_owned(),
            "ordinal-69-semantic-mismatch-cleanup".to_owned(),
            "ordinal-69-ubuntu-failure-cleanup".to_owned(),
            "p27-bounded-failure-cleanup".to_owned(),
            "p35-extra-tree-file-rejected".to_owned(),
            "p35-old-alternate-digest-rejected".to_owned(),
            "p35-receipt-tamper-rejected".to_owned(),
            "p35-seal-tamper-rejected".to_owned(),
            "p39-failure-terminal-cleanup".to_owned(),
            "p41-failure-terminal-cleanup".to_owned(),
            "process-stress-complete-load-graph-serialized".to_owned(),
            "production-surface-rejects-injection".to_owned(),
            "release-generator-rejects-cache-residue".to_owned(),
            "seed-zero-public-failure".to_owned(),
            "single-materialization-per-cycle".to_owned(),
            "single-seed-per-cycle".to_owned(),
            "synthetic-p39-root-only".to_owned(),
            "unknown-fault-cleanup-indeterminate".to_owned(),
            "unknown-fault-cleanup-reraise".to_owned(),
            "worker-protocol-replay-rejected".to_owned(),
        ])
    );
    assert_eq!(payload["cycles"].as_array().unwrap().len(), 20);

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(
        seal["payload"]["scope"]["exact_p35_complete_release_tree_binding"],
        true
    );
    assert_eq!(
        seal["payload"]["scope"]["exact_p78_capability_binding"],
        true
    );
    assert_eq!(
        seal["payload"]["scope"]["local_sibling_sealed_loader"],
        true
    );
    assert_eq!(
        seal["payload"]["scope"]["transitive_sealed_loading_serialization"],
        true
    );
    assert_eq!(
        seal["payload"]["scope"]["kernel_lock_cross_process_serialization"],
        true
    );
    assert_eq!(seal["payload"]["scope"]["public_publication"], false);
    assert_eq!(
        seal["payload"]["limits"]["ambient_sealed_dependency_resolution"],
        false
    );
    assert_eq!(seal["payload"]["limits"]["p44_p45_execution"], false);
    assert_eq!(
        seal["payload"]["limits"]["topology_per_platform"],
        "70/69/1"
    );
    assert_eq!(
        seal["payload"]["qualification_receipt"]["payload_sha256"],
        RECEIPT_PAYLOAD
    );

    let schema: Value = serde_json::from_slice(&read_lf(release.join(
        "schemas/ferris.pulse-81-ordered-capability-materialization-executor-pulse35-release-tree-successor.v1.schema.json",
    )))
    .expect("parse qualification schema");
    assert_eq!(
        schema["$id"],
        "https://ferris.dev/schema/ferris.pulse-81-ordered-capability-materialization-executor-pulse35-release-tree-successor.v1.schema.json"
    );
    assert_eq!(
        schema["properties"]["schema"]["const"],
        "ferris.pulse-81-ordered-capability-materialization-executor-pulse35-release-tree-successor-qualification/v1"
    );
    assert_eq!(
        schema["properties"]["negative_control_tests_run"]["const"],
        28
    );
    assert_eq!(
        schema["properties"]["exact_p35_complete_release_tree_verified"]["const"],
        true
    );
    assert_eq!(
        schema["properties"]["exact_p78_binding_verified"]["const"],
        true
    );
    assert_eq!(
        schema["properties"]["transitive_sealed_loading_serialization_verified"]["const"],
        true
    );
    assert_eq!(
        schema["properties"]["kernel_lock_cross_process_serialization_verified"]["const"],
        true
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
