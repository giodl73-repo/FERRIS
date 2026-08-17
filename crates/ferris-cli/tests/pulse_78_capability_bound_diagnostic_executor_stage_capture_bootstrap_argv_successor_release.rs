use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RELEASE: &str =
    "docs/simulations/profile-diff-held-out/pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-release";
const MANIFEST_RAW: &str =
    "sha256:f5cd77f0f31474677bbc847a75e0d307b9a41e7b313dcf2be22a9135fb99b7fb";
const MANIFEST_AGGREGATE: &str =
    "sha256:93fb90f2b75deb660f9700573ba51c6ff5d6675dc1fa44bc993b0036e5f3de39";
const RECEIPT_RAW: &str = "sha256:cc0d9fd1817318c07942f652a4438bcfee5c89b9144c8ca9f48da7d0b1f96a52";
const RECEIPT_PAYLOAD: &str =
    "sha256:e235dc8da9884bf67ebb2ed36791cf8894d42ed0a525dbeee1f854ece6e495e4";
const SEAL_RAW: &str = "sha256:0ea8020b9fc2cbd04a4a72861a897c78304c2505e125425d81173bddd74de0ae";
const SEAL_PAYLOAD: &str =
    "sha256:a56612188fccec0c7f4cb7b0d40922f10d4128265d500878ca3ace6202e77a55";

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
    sha256(&serde_json::to_vec(value).expect("canonical payload"))
}

fn python_output(release: &Path, arguments: &[&str]) -> std::process::Output {
    Command::new(python())
        .current_dir(release)
        .args(arguments)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .expect("run Pulse 78 Python validation")
}

#[test]
fn pulse_78_capability_bound_diagnostic_executor_stage_capture_bootstrap_argv_successor_is_sealed_and_fake_only(
) {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "capability_bound_diagnostic_executor_successor.py".to_owned(),
        "fixtures/fake_p56.py".to_owned(),
        "fixtures/p75-public-gate-catalog.json".to_owned(),
        "fixtures/p78_fake_native_wsl.py".to_owned(),
        "generate_release.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor.v1.schema.json".to_owned(),
        "sealed_dependencies.py".to_owned(),
        "tests/test_capability_bound_diagnostic_executor_successor.py".to_owned(),
        "worker_sealed_dependencies.py".to_owned(),
        "wsl_session_worker.py".to_owned(),
    ]);
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths);
    for path in &actual_paths {
        read_lf(release.join(path));
    }

    let source = String::from_utf8(read_lf(
        release.join("capability_bound_diagnostic_executor_successor.py"),
    ))
    .expect("UTF-8 executor");
    assert!(source.contains("def run_capability_bound_diagnostic_executor("));
    assert!(source.contains("_SEALED.load_exact_p75_stack(REPO_ROOT)"));
    assert!(source.contains("_WSL_BUNDLE_STAGE_BOOTSTRAP"));
    assert!(source.contains("_WSL_WORKER_BOOTSTRAP"));
    assert!(source.contains("_bind_local_sealed_lock_manager_module"));
    assert!(source.contains("P78-INDETERMINATE-STAGE-CLEANUP"));
    assert!(source.contains("P57-INDETERMINATE-CLEANUP"));
    assert!(source.contains("dependency_relative"));
    assert!(source.contains("sys.argv[12:]"));
    assert!(source.contains("\"worker/wsl_session_worker.py\""));
    assert!(source.contains("\"worker_sealed_dependencies.py\""));
    assert!(!source.contains("from sealed_dependencies import"));

    let worker_source =
        String::from_utf8(read_lf(release.join("wsl_session_worker.py"))).expect("UTF-8 worker");
    assert!(worker_source.contains("/proc/self/fd/"));
    assert!(worker_source.contains("P57-INDETERMINATE-CLEANUP"));

    let sealed_source = String::from_utf8(read_lf(release.join("sealed_dependencies.py")))
        .expect("UTF-8 sealed dependencies");
    assert!(sealed_source.contains("P75 = ReleaseIdentity("));
    assert!(sealed_source.contains("load_exact_p75_stack"));
    assert!(sealed_source.contains("release_identities"));
    assert!(sealed_source.contains("_KERNEL_LOCK_NAMESPACE_PREFIX = \"ferris-p78\""));
    assert!(sealed_source.contains("CreateMutexW"));
    assert!(sealed_source.contains("socket.AF_UNIX"));
    assert!(sealed_source.contains("_ACTIVE_SEALED_LOADING_LOCK"));
    assert!(sealed_source.contains("_bind_internal_lock_manager"));
    assert!(sealed_source.contains("P78-SEALED-LOCK-CROSS-INSTANCE-REENTRY"));

    let (_, catalog) = read_json(release.join("fixtures/p75-public-gate-catalog.json"));
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

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-manifest/v1"
    );
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len() as u64, manifest["file_count"].as_u64().unwrap());
    assert_eq!(aggregate(files), MANIFEST_AGGREGATE);
    assert_eq!(manifest["release_tree_file_count"], actual_paths.len() as u64);
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
        manifest["predecessors"]["pulse_75"]["source_sha256"],
        "sha256:6b351e68aead4e6341b6fcd880d26ed9433d96397d1cd4ccce6dc4fbbd1190a6"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_75"]["stack"]["pulse_72"]["source_sha256"],
        "sha256:408cc0013861c398e76e125d7e62ec6f24cfd0aec8e9083f2d699e3ad04b2901"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_75"]["stack"]["pulse_72"]["stack"]["pulse_69"]["stack"]["pulse_57"]["source"],
        "sha256:bcb5eac2cd5aa0abd271dec2e93963ec855faa1c5ecbd628dfef61f52358c2c0"
    );

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        receipt["schema"],
        "ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-receipt/v1"
    );
    assert_eq!(canonical_payload_sha256(&receipt["payload"]), RECEIPT_PAYLOAD);
    let payload = &receipt["payload"];
    assert_eq!(payload["schema"], "ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-qualification/v1");
    assert_eq!(payload["cycles_required"], 20);
    assert_eq!(payload["cycles_passed"], 20);
    assert_eq!(payload["fake_launches_total"], 2_760);
    assert_eq!(payload["ferris_executed"], false);
    assert_eq!(payload["negative_control_tests_run"], 18);
    assert_eq!(payload["negative_control_tests_passed"], 18);
    assert_eq!(payload["exact_p75_binding_verified"], true);
    assert_eq!(payload["local_loader_explicit_binding_verified"], true);
    assert_eq!(payload["fresh_module_loading_verified"], true);
    assert_eq!(payload["stage_identity_capture_verified"], true);
    assert_eq!(payload["stage_post_create_failure_cleanup_verified"], true);
    assert_eq!(
        payload["stage_create_open_substitution_indeterminate_verified"],
        true
    );
    assert_eq!(payload["stage_cleanup_indeterminate_precedence_verified"], true);
    assert_eq!(payload["worker_bootstrap_ready_close_verified"], true);
    assert_eq!(payload["worker_dependency_loader_binding_verified"], true);
    assert_eq!(payload["worker_bootstrap_root_swap_rejected"], true);
    assert_eq!(payload["worker_bootstrap_path_swap_rejected"], true);
    assert_eq!(payload["prelaunch_root_substitution_rejected"], true);
    assert_eq!(payload["prelaunch_parent_substitution_rejected"], true);
    assert_eq!(payload["owned_bundle_cleanup_total"], 20);
    assert_eq!(payload["staged_identity_revalidation_total"], 20);
    assert_eq!(payload["bundle_retained_during_worker_lifetime_verified"], true);
    assert_eq!(payload["cleanup_precedence_verified"], true);
    assert_eq!(payload["zero_residue_after_close_verified"], true);
    let control_ids = payload["negative_control_test_ids"]
        .as_array()
        .expect("receipt control IDs");
    assert_eq!(control_ids.len(), 18);
    let actual_control_ids = control_ids
        .iter()
        .map(|value| value.as_str().expect("receipt control ID").to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_control_ids,
        BTreeSet::from([
            "bundle-retained-and-zero-residue".to_owned(),
            "cleanup-precedence-over-protocol-failure".to_owned(),
            "concurrent-owned-bundles-isolated".to_owned(),
            "exact-p75-binding-and-signature".to_owned(),
            "local-loader-fresh-modules".to_owned(),
            "local-loader-ignores-ambient-module".to_owned(),
            "prelaunch-parent-substitution-rejected".to_owned(),
            "prelaunch-root-substitution-rejected".to_owned(),
            "stage-bootstrap-identity-revalidation".to_owned(),
            "stage-create-open-substitution-indeterminate".to_owned(),
            "stage-cleanup-indeterminate-precedence".to_owned(),
            "stage-post-create-failure-cleanup".to_owned(),
            "startup-failure-cleanup".to_owned(),
            "terminate-kill-then-bundle-cleanup".to_owned(),
            "worker-bootstrap-dependency-loader-binding".to_owned(),
            "worker-bootstrap-ready-close-exact-args".to_owned(),
            "worker-bootstrap-path-swap-rejected".to_owned(),
            "worker-bootstrap-root-swap-rejected".to_owned(),
        ])
    );
    assert_eq!(payload["cycles"].as_array().unwrap().len(), 20);

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(seal["payload"]["scope"]["exact_p75_execution"], true);
    assert_eq!(
        seal["payload"]["scope"]["bootstrap_named_worker_args_only"],
        true
    );
    assert_eq!(
        seal["payload"]["scope"]["bootstrap_worker_ready_close_tested"],
        true
    );
    assert_eq!(
        seal["payload"]["scope"]["prelaunch_stage_capture_indeterminate"],
        true
    );
    assert_eq!(
        seal["payload"]["scope"]["stage_failure_cleanup_owned_in_bootstrap"],
        true
    );
    assert_eq!(
        seal["payload"]["scope"]["worker_bootstrap_identity_bound"],
        true
    );
    assert_eq!(
        seal["payload"]["scope"]["worker_dependency_identity_bound"],
        true
    );
    assert_eq!(
        seal["payload"]["limits"]["cleanup_substitution_disposition"],
        "P57-INDETERMINATE-CLEANUP"
    );
    assert_eq!(
        seal["payload"]["limits"]["prelaunch_capture_substitution_disposition"],
        "P78-INDETERMINATE-STAGE-CLEANUP"
    );
    assert_eq!(
        seal["payload"]["limits"]["parent_or_root_replacement_deletion"],
        false
    );
    assert_eq!(
        seal["payload"]["limits"]["bootstrap_dependency_loader_binding_without_match"],
        false
    );
    assert_eq!(
        seal["payload"]["limits"]["worker_launch_without_identity_match"],
        false
    );
    assert_eq!(
        seal["payload"]["limits"]["worker_dependency_load_without_identity_match"],
        false
    );
    assert_eq!(
        seal["payload"]["qualification_receipt"]["staged_identity_revalidation_total"],
        20
    );
    assert_eq!(
        seal["payload"]["qualification_receipt"]["owned_bundle_cleanup_total"],
        20
    );

    let schema: Value = serde_json::from_slice(&read_lf(release.join(
        "schemas/ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor.v1.schema.json",
    )))
    .expect("parse qualification schema");
    assert_eq!(
        schema["$id"],
        "https://ferris.dev/schema/ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor.v1.schema.json"
    );
    assert_eq!(
        schema["properties"]["schema"]["const"],
        "ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-qualification/v1"
    );
    assert_eq!(
        schema["properties"]["negative_control_tests_run"]["const"],
        18
    );
    assert_eq!(
        schema["properties"]["stage_post_create_failure_cleanup_verified"]["const"],
        true
    );
    assert_eq!(
        schema["properties"]["stage_create_open_substitution_indeterminate_verified"]["const"],
        true
    );
    assert_eq!(
        schema["properties"]["stage_cleanup_indeterminate_precedence_verified"]["const"],
        true
    );
    assert_eq!(
        schema["properties"]["worker_bootstrap_ready_close_verified"]["const"],
        true
    );
    assert_eq!(
        schema["properties"]["worker_dependency_loader_binding_verified"]["const"],
        true
    );
    assert_eq!(
        schema["properties"]["worker_bootstrap_root_swap_rejected"]["const"],
        true
    );
    assert_eq!(
        schema["properties"]["worker_bootstrap_path_swap_rejected"]["const"],
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
