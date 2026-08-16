use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const RELEASE: &str = "docs/simulations/profile-diff-held-out/pulse-71-witness-preserving-capability-materialization-executor-successor-release";
const MANIFEST_RAW: &str =
    "sha256:3b955dc7c9ef727b885e3482631e52a02e4c5838ef1e98dd8607dc86d210b164";
const MANIFEST_AGGREGATE: &str =
    "sha256:24a537b7edeaa2b419d1f7f0300a1026071ab8e831c99c0bc68bbae88be5bf03";
const RECEIPT_RAW: &str = "sha256:7c5afaab6d5db2387dd5e45300869eead7f4be6d6e9086e02bbf94d76e3c4dd9";
const RECEIPT_PAYLOAD: &str =
    "sha256:c28d1949eaa0ccfa4e3c1ac34d2a00415954a5b0f57275d959b20e1032767bd1";
const SEAL_RAW: &str = "sha256:2659af99db781145eaa1a7b8bb4a70765fd38b257614e2524a87cbb990b69414";
const SEAL_PAYLOAD: &str =
    "sha256:6daed7b33c939c74fb7b54d083c3dfe444bf2bbdf7d25f9ce597d6cb0744e804";

fn repo_root() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .expect("canonical repository root")
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

#[test]
fn pulse_71_witness_preserving_capability_materialization_executor_successor_is_sealed() {
    let root = repo_root();
    let release = root.join(RELEASE);
    let expected_paths = BTreeSet::from([
        "README.md".to_owned(),
        "fixtures/__init__.py".to_owned(),
        "fixtures/fake_p56.py".to_owned(),
        "fixtures/p52_synthetic_fixture.py".to_owned(),
        "fixtures/p71-public-gate-catalog.json".to_owned(),
        "generate_release.py".to_owned(),
        "public-manifest.json".to_owned(),
        "qualification-receipt.json".to_owned(),
        "qualify.py".to_owned(),
        "release-seal.json".to_owned(),
        "root-cause-report.md".to_owned(),
        "schemas/ferris.pulse-71-witness-preserving-capability-materialization-executor-successor.v1.schema.json".to_owned(),
        "sealed_dependencies.py".to_owned(),
        "tests/test_witness_preserving_capability_materialization_executor.py".to_owned(),
        "witness_preserving_capability_materialization_executor.py".to_owned(),
    ]);
    let mut actual_paths = BTreeSet::new();
    collect_files(&release, &release, &mut actual_paths);
    assert_eq!(actual_paths, expected_paths);
    for path in &actual_paths {
        read_lf(release.join(path));
    }

    let source = String::from_utf8(read_lf(
        release.join("witness_preserving_capability_materialization_executor.py"),
    ))
    .expect("UTF-8 executor");
    assert!(source.contains("def run_witness_preserving_capability_materialization_executor("));
    assert!(source.contains("p70.run_ordered_capability_materialization_executor("));
    assert!(source.contains("p70._run_qualification_executor("));
    assert!(source.contains("def _load_local_sealed_dependencies()"));
    assert!(source.contains("LOCAL_SEALED_DEPENDENCIES_SHA256"));
    assert!(source.contains("_LOCAL_SEALED_DEPENDENCIES_RUNTIME_PREFIX"));
    assert!(source.contains("uuid.uuid4().hex"));
    assert!(source.contains("fresh-sibling-of-private-runtime-root"));
    assert!(source.contains("published-failure-witness"));
    assert!(source.contains("invalid-witness-publication"));
    assert!(source.contains("class _Pulse71LinuxLockManager"));
    assert!(source.contains("register_at_fork"));
    assert!(source.contains("_CROSS_INSTANCE_REENTRY_STATE_KEY"));
    assert!(source.contains("advisory_conflict"));
    assert!(source.contains("_bind_local_sealed_lock_manager_module"));
    assert!(!source.contains("from sealed_dependencies import"));
    assert!(!source.contains("_LOCAL_SEALED_MODULE_PREFIX"));
    assert!(!source.contains("_process_local_sealed_registry"));
    assert!(!source.contains("run_capability_bound_diagnostic_executor("));

    let sealed_source = String::from_utf8(read_lf(release.join("sealed_dependencies.py")))
        .expect("UTF-8 sealed dependencies");
    assert!(!sealed_source.contains("_P58_MODULE"));
    assert!(sealed_source.contains("CreateMutexW"));
    assert!(sealed_source.contains("WaitForSingleObject"));
    assert!(sealed_source.contains("socket.AF_UNIX"));
    assert!(sealed_source.contains("_KERNEL_LOCK_NAMESPACE_PREFIX = \"ferris-p71\""));
    assert!(sealed_source.contains("return f\"\\0{_KERNEL_LOCK_NAMESPACE_PREFIX}-{value}\""));
    assert!(sealed_source.contains("_kernel_lock_name"));
    assert!(sealed_source.contains("_current_pid"));
    assert!(sealed_source.contains("_current_thread_id"));
    assert!(sealed_source.contains("_SEALED_LOCK_TIMEOUT_SECONDS"));
    assert!(sealed_source.contains("_ACTIVE_SEALED_LOADING_LOCK"));
    assert!(sealed_source.contains("_normalize_active_loading_lock"));
    assert!(sealed_source.contains("owner_thread_id"));
    assert!(sealed_source.contains("owner_token.live = False"));
    assert!(sealed_source.contains("_P71_INTERNAL_LOCK_MANAGER"));
    assert!(sealed_source.contains("_bind_internal_lock_manager"));
    assert!(sealed_source.contains("P71-SEALED-LOCK-CROSS-INSTANCE-REENTRY"));
    assert!(sealed_source.contains("_WINDOWS_WAIT_ABANDONED"));
    assert!(!sealed_source.contains("register_at_fork"));
    assert!(!sealed_source.contains("pulse-71-sealed-loader-locks"));
    assert!(!sealed_source.contains("_lock_file_path"));
    assert!(!sealed_source.contains("sem_open"));
    assert!(sealed_source.contains("if current is not dependencies"));
    assert!(!sealed_source.contains("threading.RLock"));

    let (_, catalog) = read_json(release.join("fixtures/p71-public-gate-catalog.json"));
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
        "ferris.pulse-71-witness-preserving-capability-materialization-executor-public-manifest/v1"
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
        manifest["predecessors"]["pulse_70"]["commit"],
        "b33cc2fdf2b067bc2bd448402164d939e69b5515"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_70"]["source_sha256"],
        "sha256:08fad48927db43db0546b8cf44f97992b2373929f09a44ba43c0db148757f76d"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_43"]["source"],
        "sha256:38ebc7ce84ae29c2ad20ada593d8baeb0352b59e7c48438c4a9c224a0ea4a6c6"
    );
    assert_eq!(
        manifest["predecessors"]["pulse_47"]["source"],
        "sha256:4a402d3c2e034597a574368e628af0b87966b74ec2cdef947b38db2881cf4760"
    );

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    let payload = &receipt["payload"];
    assert_eq!(payload["cycles_required"], 20);
    assert_eq!(payload["cycles_passed"], 20);
    assert_eq!(payload["fake_launches_total"], 2_760);
    assert_eq!(payload["published_results"], 10);
    assert_eq!(payload["published_failure_witnesses"], 10);
    assert_eq!(payload["invalid_witness_publications"], 0);
    assert_eq!(payload["failure_witness_postures"]["absent"], 4);
    assert_eq!(payload["failure_witness_postures"]["rolled-back"], 3);
    assert_eq!(payload["failure_witness_postures"]["indeterminate"], 3);
    assert_eq!(payload["ferris_executed"], false);
    assert_eq!(payload["behavioral_control_tests_run"], 39);
    assert_eq!(payload["behavioral_control_tests_passed"], 39);
    assert_eq!(
        payload["p70_bound_commit"],
        "b33cc2fdf2b067bc2bd448402164d939e69b5515"
    );
    assert_eq!(payload["p70_publication_invocations"], 0);
    assert_eq!(payload["p71_publication_invocations"], 20);
    assert_eq!(
        payload["terminal_root_policy"],
        "fresh-sibling-of-private-runtime-root"
    );
    let control_ids = payload["behavioral_control_test_ids"]
        .as_array()
        .expect("control IDs");
    assert_eq!(control_ids.len(), 39);
    let actual_control_ids = control_ids
        .iter()
        .map(|value| value.as_str().expect("control ID").to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_control_ids,
        BTreeSet::from([
            "absent-failure-witness".to_owned(),
            "cleanup-indeterminate-precedence".to_owned(),
            "concurrent-p70-load-restores-foreign-sentinel".to_owned(),
            "exact-p70-binding-and-signature".to_owned(),
            "indeterminate-failure-witness".to_owned(),
            "local-binder-exception-cleans-runtime-slot".to_owned(),
            "local-binder-ignores-external-resolution".to_owned(),
            "local-binder-mutation-does-not-persist".to_owned(),
            "kernel-lock-acquire-failure-cleans-up".to_owned(),
            "kernel-lock-at-fork-registration-idempotent".to_owned(),
            "kernel-lock-context-copy-thread-blocks".to_owned(),
            "kernel-lock-context-replay-blocks".to_owned(),
            "kernel-lock-crash-recovery".to_owned(),
            "kernel-lock-cross-instance-reentry-fails-closed".to_owned(),
            "kernel-lock-fork-child-cleanup-reacquire".to_owned(),
            "kernel-lock-fork-reacquire-no-count-inflation".to_owned(),
            "kernel-lock-name-stable-across-instances".to_owned(),
            "kernel-lock-no-file-artifacts".to_owned(),
            "kernel-lock-pid-mismatch-reacquire".to_owned(),
            "kernel-lock-process-stress".to_owned(),
            "kernel-lock-reentrant-depth-single-acquire".to_owned(),
            "kernel-lock-releases-after-exception".to_owned(),
            "kernel-lock-unsupported-posix-platform".to_owned(),
            "kernel-lock-wait-abandoned-release".to_owned(),
            "malformed-hash-mismatch-residue-cleanup".to_owned(),
            "no-retry-terminal-seam".to_owned(),
            "old-private-binder-key-ignored".to_owned(),
            "old-registry-key-ignored".to_owned(),
            "p70-import-exception-restores-generic-slot".to_owned(),
            "p70-prelaunch-failure-publication-not-attempted".to_owned(),
            "path-free-transfer-descriptor".to_owned(),
            "preexisting-terminal-root-rejected".to_owned(),
            "production-surface-rejects-injection".to_owned(),
            "published-result-survives-private-cleanup".to_owned(),
            "qualification-delegates-to-p70".to_owned(),
            "release-generator-rejects-cache-residue".to_owned(),
            "rolled-back-failure-witness".to_owned(),
            "transitive-concurrent-load-stress".to_owned(),
            "two-executor-instances-load-fresh-binders".to_owned(),
        ])
    );
    assert_eq!(payload["cycles"].as_array().unwrap().len(), 20);

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(seal["payload"]["scope"]["exact_p70_execution"], true);
    assert_eq!(seal["payload"]["scope"]["terminal_root_injected"], false);
    assert_eq!(
        seal["payload"]["limits"]["terminal_root_policy"],
        "fresh-sibling-of-private-runtime-root"
    );
    assert_eq!(
        seal["payload"]["limits"]["publication_not_attempted_before_p70_completion"],
        true
    );

    let schema: Value = serde_json::from_slice(&read_lf(release.join(
        "schemas/ferris.pulse-71-witness-preserving-capability-materialization-executor-successor.v1.schema.json",
    )))
    .expect("parse public return schema");
    assert_eq!(
        schema["$id"],
        "urn:ferris:schema:pulse-71-witness-preserving-capability-materialization-executor:v1"
    );
    assert_eq!(
        schema["$defs"]["terminalPublication"]["properties"]["disposition"]["enum"],
        serde_json::json!([
            "not-attempted",
            "published-result",
            "published-failure-witness",
            "invalid-witness-publication"
        ])
    );
}
