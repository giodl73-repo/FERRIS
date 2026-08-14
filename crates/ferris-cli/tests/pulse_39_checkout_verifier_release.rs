use serde_json::Value;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const CUTOFF: &str = "6807bd68aa01cbf0c819198765b7d6b5aa443328";
const RELEASE: &str = "docs/simulations/profile-diff-held-out/pulse-39-checkout-verifier-release";
const MANIFEST_RAW: &str =
    "sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c";
const MANIFEST_AGGREGATE: &str =
    "sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c";
const REPORT_RAW: &str = "sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd";
const REPORT_PAYLOAD: &str =
    "sha256:fcfdf7c44c0f4084a6b6339d43626e67fa7b5a1e3b268c9262ae3587f9a4c5ab";
const RECEIPT_RAW: &str = "sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8";
const RECEIPT_PAYLOAD: &str =
    "sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546";
const SEAL_RAW: &str = "sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c";
const SEAL_PAYLOAD: &str =
    "sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b";
const P29_RECEIPT_RAW: &str =
    "sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225";
const P29_RECEIPT_PAYLOAD: &str =
    "sha256:92e245685cbb1b6ce938701a901c4de9b9202f9149537690e646d13a113deb40";
const P25_ROOT: &str = "docs/simulations/profile-diff-held-out/pulse-25-collector-source-release";
const P27_ROOT: &str = "docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn release_root() -> PathBuf {
    repo_root().join(RELEASE)
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
    assert!(!bytes.contains(&b'\r'), "{path:?} must have no CR byte");
    assert!(bytes.ends_with(b"\n"), "{path:?} must end in LF");
    bytes
}

fn read_json(path: impl AsRef<Path>) -> (Vec<u8>, Value) {
    let bytes = read_lf(path);
    let value = serde_json::from_slice(&bytes).expect("parse JSON");
    (bytes, value)
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
                .expect("sha256 prefix"),
        ));
    }

    fn decode_sha256(value: &str) -> [u8; 32] {
        assert_eq!(value.len(), 64, "SHA-256 hex length");
        let mut bytes = [0_u8; 32];
        for (index, byte) in bytes.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16)
                .expect("SHA-256 hexadecimal");
        }
        bytes
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn run_checked(command: &mut Command) {
    let output = command.output().expect("run command");
    assert!(
        output.status.success(),
        "command failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn python() -> PathBuf {
    env::var_os("PYTHON")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("python"))
}

fn verifier_output(checkout: &Path, cwd: &Path, pulse_25_root: &str) -> Output {
    Command::new(python())
        .arg(release_root().join("checkout_verifier.py"))
        .args(["--checkout-root", checkout.to_str().expect("checkout path")])
        .args(["--pulse25-root", pulse_25_root])
        .args(["--pulse27-root", P27_ROOT])
        .current_dir(cwd)
        .output()
        .expect("run checkout verifier")
}

fn remove_tree(path: &Path) {
    if !path.exists() {
        return;
    }
    let metadata = fs::symlink_metadata(path).expect("workspace metadata");
    if metadata.is_dir() {
        for entry in fs::read_dir(path).expect("workspace directory") {
            remove_tree(&entry.expect("workspace entry").path());
        }
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);
        fs::set_permissions(path, permissions).expect("make workspace directory writable");
        fs::remove_dir(path).expect("remove workspace directory");
    } else {
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);
        fs::set_permissions(path, permissions).expect("make workspace file writable");
        fs::remove_file(path).expect("remove workspace file");
    }
}

#[test]
fn pulse_39_checkout_verifier_release_is_sealed_and_root_anchored() {
    let root = repo_root();
    let release = release_root();

    let (manifest_bytes, manifest) = read_json(release.join("public-manifest.json"));
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-39-checkout-verifier-public-manifest/v1"
    );
    let files = manifest["files"].as_array().expect("manifest files");
    assert_eq!(files.len(), 5);
    assert_eq!(manifest["file_count"], 5);
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(manifest_aggregate(files), MANIFEST_AGGREGATE);
    let mut total = 0_u64;
    for file in files {
        let path = file["path"].as_str().expect("manifest path");
        assert!(
            !Path::new(path).is_absolute(),
            "manifest path stays relative"
        );
        assert!(!path.contains(".."), "manifest path stays contained");
        let bytes = read_lf(release.join(path));
        assert_eq!(
            bytes.len() as u64,
            file["size"].as_u64().expect("file size")
        );
        assert_eq!(sha256(&bytes), file["sha256"], "{path} binding");
        total += bytes.len() as u64;
    }
    assert_eq!(total, 26_455);
    assert_eq!(manifest["total_bytes"], total);

    let (report_bytes, report) = read_json(release.join("root-cause-report.json"));
    assert_eq!(sha256(&report_bytes), REPORT_RAW);
    assert_eq!(report["payload_sha256"], REPORT_PAYLOAD);
    assert_eq!(report["report_id"], REPORT_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&report["payload"]), REPORT_PAYLOAD);
    assert_eq!(report["payload"]["cutoff"], CUTOFF);
    assert_eq!(
        report["payload"]["correct_invocation"],
        "git -C <checkout-root> check-attr -z --stdin text eol"
    );
    assert_eq!(
        report["payload"]["verifier_control"]["check_attr_invocations"],
        1
    );
    assert_eq!(
        report["payload"]["verifier_control"]["git_version_probes"],
        1
    );
    assert_eq!(
        report["payload"]["verifier_control"]["total_git_processes"],
        2
    );
    assert_eq!(report["payload"]["verifier_control"]["retries"], 0);
    assert_eq!(
        report["payload"]["verifier_control"]["fallback_check_attr_form"],
        false
    );
    assert_eq!(report["payload"]["pulse_38"]["disposition"], "invalid");
    assert_eq!(report["payload"]["pulse_38"]["retry_authorized"], false);

    let (receipt_bytes, receipt) = read_json(release.join("qualification-receipt.json"));
    assert_eq!(sha256(&receipt_bytes), RECEIPT_RAW);
    assert_eq!(receipt["payload_sha256"], RECEIPT_PAYLOAD);
    assert_eq!(receipt["receipt_id"], RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&receipt["payload"]),
        RECEIPT_PAYLOAD
    );
    assert_eq!(receipt["payload"]["cutoff"], CUTOFF);
    assert_eq!(
        receipt["payload"]["git_version"],
        "git version 2.55.0.windows.3"
    );
    assert_eq!(receipt["payload"]["materialization"]["core_autocrlf"], true);
    assert_eq!(
        receipt["payload"]["status_counts"],
        serde_json::json!({
            "attribute_files_passed": 36,
            "expected_files": 36,
            "lf_files_passed": 36,
            "zero_cr_files": 36
        })
    );
    assert_eq!(
        receipt["payload"]["pulse_29_binding_receipt"]["binding_checks_passed"],
        76
    );
    assert_eq!(
        receipt["payload"]["pulse_29_binding_receipt"]["binding_checks_total"],
        76
    );
    let expected_git_process_accounting = serde_json::json!({
        "check_attr_invocations": 1,
        "fallback_check_attr_form": false,
        "git_version_probes": 1,
        "retries": 0,
        "total_git_processes": 2
    });
    assert_eq!(
        receipt["payload"]["git_process_accounting"],
        expected_git_process_accounting
    );

    let (seal_bytes, seal) = read_json(release.join("release-seal.json"));
    assert_eq!(sha256(&seal_bytes), SEAL_RAW);
    assert_eq!(seal["payload_sha256"], SEAL_PAYLOAD);
    assert_eq!(seal["seal_id"], SEAL_PAYLOAD);
    assert_eq!(canonical_payload_sha256(&seal["payload"]), SEAL_PAYLOAD);
    assert_eq!(seal["payload"]["manifest"]["raw_sha256"], MANIFEST_RAW);
    assert_eq!(
        seal["payload"]["git_process_accounting"],
        expected_git_process_accounting
    );
    assert_eq!(seal["payload"]["manifest"]["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(
        seal["payload"]["qualification_receipt"]["raw_sha256"],
        RECEIPT_RAW
    );
    assert_eq!(
        seal["payload"]["root_cause_report"]["raw_sha256"],
        REPORT_RAW
    );
    assert_eq!(
        seal["payload"]["verification"]["normalized_bindings"],
        "76/76"
    );
    assert_eq!(seal["payload"]["release_limits"]["ferris_execution"], false);
    assert_eq!(
        seal["payload"]["release_limits"]["diagnostic_authority"],
        false
    );

    let (p29_bytes, p29) = read_json(
        root.join(
            "docs/simulations/profile-diff-held-out/pulse-29-checkout-normalization/PULSE-29-CHECKOUT-NORMALIZATION-RECEIPT.json",
        ),
    );
    assert_eq!(sha256(&p29_bytes), P29_RECEIPT_RAW);
    assert_eq!(p29["payload_sha256"], P29_RECEIPT_PAYLOAD);
    assert_eq!(p29["receipt_id"], P29_RECEIPT_PAYLOAD);
    assert_eq!(
        canonical_payload_sha256(&p29["payload"]),
        P29_RECEIPT_PAYLOAD
    );
    assert_eq!(p29["payload"]["binding_checks"]["passed"], 76);
    assert_eq!(p29["payload"]["binding_checks"]["failed"], 0);

    let attributes = fs::read_to_string(root.join(".gitattributes")).expect("attributes");
    for rule in [
        format!("/{RELEASE}/** text eol=lf"),
        "/context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-39.md text eol=lf"
            .to_owned(),
        "/docs/plans/reviews/PULSE-39-CHECKOUT-VERIFIER-RELEASE-ROLE-REVIEW.md text eol=lf"
            .to_owned(),
        "/crates/ferris-cli/tests/pulse_39_checkout_verifier_release.rs text eol=lf".to_owned(),
    ] {
        assert!(
            attributes.lines().any(|line| line == rule),
            "missing {rule}"
        );
    }
    for relative in [
        "CONTEXT.md",
        "README.md",
        "docs/simulations/profile-diff-held-out/README.md",
        "context/waves/2026-08-12-platform-profile-conformance/WAVE.md",
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-39.md",
        "docs/plans/reviews/PULSE-39-CHECKOUT-VERIFIER-RELEASE-ROLE-REVIEW.md",
    ] {
        let text = fs::read_to_string(root.join(relative)).expect("Pulse 39 index");
        assert!(text.contains("Pulse 39"), "{relative} indexes Pulse 39");
        assert!(text.contains(MANIFEST_RAW), "{relative} binds manifest");
        assert!(text.contains("Pulse 38"), "{relative} preserves Pulse 38");
    }

    let workspace = root.join("target/pulse-39-rust-checkout-verifier");
    remove_tree(&workspace);
    let checkout = workspace.join("cutoff");
    fs::create_dir_all(&workspace).expect("create test workspace");
    run_checked(
        Command::new("git")
            .args(["clone", "--no-local", "--no-checkout"])
            .arg(&root)
            .arg(&checkout),
    );
    run_checked(
        Command::new("git")
            .args(["-C", checkout.to_str().expect("checkout path"), "config"])
            .args(["core.autocrlf", "true"]),
    );
    run_checked(
        Command::new("git")
            .args([
                "-C",
                checkout.to_str().expect("checkout path"),
                "checkout",
                "--force",
            ])
            .arg(CUTOFF),
    );

    let nested = checkout.join("docs/simulations/profile-diff-held-out");
    let output = verifier_output(&checkout, &nested, P25_ROOT);
    assert!(
        output.status.success(),
        "verifier stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).expect("public verifier JSON");
    assert_eq!(report["status"], "pass");
    let git_version = report["git_version"]
        .as_str()
        .expect("live Git version string");
    assert!(git_version.starts_with("git version "));
    assert!(git_version.len() <= 128);
    assert!(!git_version.contains('\r'));
    assert!(!git_version.contains('\n'));
    assert_eq!(report["count"], 36);
    assert_eq!(report["attribute_files"], 36);
    assert_eq!(report["lf_files"], 36);
    assert_eq!(report["zero_cr_files"], 36);
    let paths = report["files"].as_array().expect("public relative paths");
    assert_eq!(paths.len(), 36);
    assert!(paths.iter().all(|path| {
        let path = path.as_str().expect("public path");
        !Path::new(path).is_absolute() && !path.contains("..") && !path.contains('\\')
    }));

    let mut mutated =
        fs::read(checkout.join(format!("{P25_ROOT}/README.md"))).expect("read target");
    mutated.push(b'\r');
    fs::write(checkout.join(format!("{P25_ROOT}/README.md")), mutated).expect("mutate clone only");
    let cr_failure = verifier_output(&checkout, &nested, P25_ROOT);
    assert!(!cr_failure.status.success());
    let cr_json: Value = serde_json::from_slice(&cr_failure.stdout).expect("CR failure JSON");
    assert_eq!(
        cr_json,
        serde_json::json!({"code":"P39-CR-BYTES","status":"fail"})
    );

    let path_failure = verifier_output(&checkout, &nested, "../outside");
    assert!(!path_failure.status.success());
    let path_json: Value = serde_json::from_slice(&path_failure.stdout).expect("path failure JSON");
    assert_eq!(
        path_json,
        serde_json::json!({"code":"P39-PATH-INVALID","status":"fail"})
    );

    remove_tree(&workspace);
}
