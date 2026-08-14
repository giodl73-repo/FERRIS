use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const CUTOFF: &str = "29517d732db13cc2ffa304684b344f3538ab587d";
const MANIFEST_RAW: &str =
    "sha256:9082bb18ab72e5e5ced2ec43811ecc5ce14ac43c9cd2878e4db0f10bf7a741fd";
const MANIFEST_AGGREGATE: &str =
    "sha256:07df7bc02cab288adccbdc0f87e45f3fd52939ebc9d22c8d064f773843e861a4";
const QUALIFICATION_RAW: &str =
    "sha256:84c09348fe1af7c639510d4ca175bdde0eed51a27a3e2e6f2b80414c80fc10a0";
const QUALIFICATION_PAYLOAD: &str =
    "sha256:0e64090a6fa7cddfa44e63f7a6be7963498dfc9f34ef15fa1c290fa73dbac48e";
const ROOT_CAUSE_RAW: &str =
    "sha256:9c299af5548a5df004676c1dd79108d76ea0774861f8bc4d0758d44fd7a1e16b";
const ROOT_CAUSE_PAYLOAD: &str =
    "sha256:e72921f8433d2a787c9142ad056bc5beff05f71836a0ab38b7fad90797d2babc";
const SYNTHETIC_RAW: &str =
    "sha256:e36f3deca0de99f09268fdfdd9088b8adfe5bb86c7666f744a816c00b78129ca";
const SYNTHETIC_PAYLOAD: &str =
    "sha256:8ca82fee60c484c9b18113ee5aa6dd9326a9f29d8c33982891a435403c32914a";
const SEAL_RAW: &str = "sha256:057f6dea59665401331b29ad984e203cca474143d7576a6617588922bf678cbd";
const SEAL_PAYLOAD: &str =
    "sha256:7ebb70ddc2a610b8c7638f30d03d0707b7d00c3eabe56ab679f085d7035f109a";
const UBUNTU_RECEIPT_RAW: &str =
    "sha256:23e4f56dc26be96adc140f5a1aa181389a8cdcd8497ca30fc47c15763dfc91c0";
const UBUNTU_RECEIPT_PAYLOAD: &str =
    "sha256:b01e45259e340309772e1d5d5c947cff163ada8dad8b9ddf19775f0a537c4cae";
const WINDOWS_RECEIPT_RAW: &str =
    "sha256:3d1624d02fc5784a7b3daab9403123b377761bc8f63ec3d46aea7411ca460622";
const WINDOWS_RECEIPT_PAYLOAD: &str =
    "sha256:b1d42470ca709406c5869bae9e677334539e745faf8e0f400e4ae93f34cf7d7a";
const UBUNTU_ARTIFACT: &str =
    "sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4";
const WINDOWS_ARTIFACT: &str =
    "sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn release_root() -> PathBuf {
    repo_root().join("docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn read_json(relative: &str) -> (Vec<u8>, Value) {
    let bytes = fs::read(release_root().join(relative)).expect("read Pulse 33 JSON");
    let value = serde_json::from_slice(&bytes).expect("parse Pulse 33 JSON");
    (bytes, value)
}

fn canonical_payload_sha256(value: &Value) -> String {
    sha256(&serde_json::to_vec(value).expect("serialize canonical payload"))
}

fn envelope(relative: &str, raw: &str, payload: &str) -> Value {
    let (bytes, value) = read_json(relative);
    assert_eq!(sha256(&bytes), raw, "{relative} raw digest");
    assert_eq!(value["payload_sha256"], payload, "{relative} payload field");
    assert_eq!(
        canonical_payload_sha256(&value["payload"]),
        payload,
        "{relative} canonical payload"
    );
    value
}

fn collect_public_files(directory: &Path, root: &Path, files: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).expect("read release directory") {
        let path = entry.expect("release entry").path();
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            if name != "__pycache__" && name != ".work" {
                collect_public_files(&path, root, files);
            }
        } else {
            let relative = path
                .strip_prefix(root)
                .expect("release-relative path")
                .to_string_lossy()
                .replace('\\', "/");
            if relative != "public-manifest.json" && relative != "release-seal.json" {
                files.insert(relative);
            }
        }
    }
}

#[test]
fn pulse_33_manifest_verifies_every_public_file_hash_size_and_aggregate() {
    let root = release_root();
    let (manifest_bytes, manifest) = read_json("public-manifest.json");
    assert_eq!(sha256(&manifest_bytes), MANIFEST_RAW);
    assert_eq!(
        manifest["schema"],
        "ferris.pulse-33-public-build-freeze-manifest/v1"
    );
    assert_eq!(manifest["cutoff"], CUTOFF);
    assert_eq!(
        manifest["aggregate_algorithm"],
        "sha256-length-path-filedigest-v1"
    );
    assert_eq!(manifest["aggregate"], MANIFEST_AGGREGATE);
    assert_eq!(manifest["file_count"], 37);
    assert_eq!(manifest["total_bytes"], 59_895);

    let entries = manifest["files"].as_array().expect("manifest files");
    assert_eq!(entries.len(), 37);
    let mut listed = BTreeSet::new();
    let mut aggregate = Sha256::new();
    let mut total_bytes = 0_u64;
    let mut previous = String::new();
    for entry in entries {
        let relative = entry["path"].as_str().expect("manifest path");
        assert!(
            previous.as_str() < relative,
            "manifest paths must be sorted"
        );
        previous = relative.to_owned();
        assert!(
            listed.insert(relative.to_owned()),
            "duplicate manifest path"
        );

        let bytes = fs::read(root.join(relative)).expect("read manifest file");
        let size = entry["size"].as_u64().expect("manifest size");
        assert_eq!(bytes.len() as u64, size, "{relative} size");
        assert_eq!(sha256(&bytes), entry["sha256"], "{relative} digest");
        total_bytes += size;

        aggregate.update(size.to_string().as_bytes());
        aggregate.update(b"\0");
        aggregate.update(relative.as_bytes());
        aggregate.update(b"\0");
        aggregate.update(
            entry["sha256"]
                .as_str()
                .expect("manifest digest")
                .trim_start_matches("sha256:")
                .as_bytes(),
        );
        aggregate.update(b"\n");
    }
    assert_eq!(total_bytes, 59_895);
    assert_eq!(
        format!("sha256:{:x}", aggregate.finalize()),
        MANIFEST_AGGREGATE
    );

    let mut actual = BTreeSet::new();
    collect_public_files(&root, &root, &mut actual);
    assert_eq!(actual, listed);

    let attributes =
        fs::read_to_string(repo_root().join(".gitattributes")).expect("read gitattributes");
    assert!(attributes.contains(
        "/docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/** binary"
    ));
}

#[test]
fn pulse_33_receipts_reports_and_evidence_close_the_build_blocker_only() {
    let qualification = envelope(
        "qualification-receipt.json",
        QUALIFICATION_RAW,
        QUALIFICATION_PAYLOAD,
    );
    let root_cause = envelope("root-cause-report.json", ROOT_CAUSE_RAW, ROOT_CAUSE_PAYLOAD);
    let synthetic = envelope("synthetic-checks.json", SYNTHETIC_RAW, SYNTHETIC_PAYLOAD);
    let seal = envelope("release-seal.json", SEAL_RAW, SEAL_PAYLOAD);
    let ubuntu = envelope(
        &format!("frozen/ferris-ubuntu-24.04-x86_64-{CUTOFF}.receipt.json"),
        UBUNTU_RECEIPT_RAW,
        UBUNTU_RECEIPT_PAYLOAD,
    );
    let windows = envelope(
        &format!("frozen/ferris-windows-x86_64-{CUTOFF}.exe.receipt.json"),
        WINDOWS_RECEIPT_RAW,
        WINDOWS_RECEIPT_PAYLOAD,
    );

    let qualification = &qualification["payload"];
    assert_eq!(qualification["outcome"], "pass");
    assert_eq!(qualification["diagnostic_execution"], false);
    assert_eq!(qualification["product_files_modified"], false);
    assert_eq!(qualification["unit_tests"]["passed"], 14);
    assert_eq!(qualification["synthetic_checks"]["passed"], 20);
    assert_eq!(qualification["actual_build_freezes"]["passed"], 2);
    assert_eq!(
        qualification["deterministic_clean_rebuilds"]["ubuntu"]["builds"],
        2
    );
    assert_eq!(
        qualification["deterministic_clean_rebuilds"]["windows"]["builds"],
        2
    );
    assert_eq!(
        qualification["deterministic_clean_rebuilds"]["windows"]["linker_control"],
        "RUSTFLAGS=-C link-arg=/Brepro"
    );
    assert_eq!(
        qualification["actual_build_freezes"]["receipts"],
        serde_json::json!([UBUNTU_RECEIPT_PAYLOAD, WINDOWS_RECEIPT_PAYLOAD])
    );

    let root_cause = &root_cause["payload"];
    assert_eq!(
        root_cause["root_cause"],
        "WSL non-login shell orchestration omitted the ordinary rustup Cargo directory from PATH"
    );
    assert_eq!(root_cause["generic_failure"]["exit_code"], 127);
    assert_eq!(
        root_cause["generic_failure"]["message"],
        "cargo: command not found"
    );
    assert_eq!(root_cause["generic_failure"]["stage"], "before-cargo-start");
    assert_eq!(root_cause["product_change_required"], false);
    assert_eq!(
        root_cause["platform_builds"]["ubuntu-24.04-x86_64"]["cargo_exit"],
        0
    );
    assert_eq!(
        root_cause["platform_builds"]["ubuntu-24.04-x86_64"]["sha256"],
        UBUNTU_ARTIFACT
    );
    assert_eq!(
        root_cause["platform_builds"]["windows-x86_64"]["cargo_exit"],
        0
    );
    assert_eq!(
        root_cause["platform_builds"]["windows-x86_64"]["sha256"],
        WINDOWS_ARTIFACT
    );

    let checks = synthetic["payload"]["checks"]
        .as_array()
        .expect("synthetic checks");
    assert_eq!(checks.len(), 20);
    assert_eq!(synthetic["payload"]["passed"], 20);
    assert_eq!(synthetic["payload"]["failed"], 0);
    assert_eq!(synthetic["payload"]["diagnostic_execution"], false);
    for (index, check) in checks.iter().enumerate() {
        assert_eq!(check["check"], index + 1);
        assert_eq!(check["outcome"], "pass");
    }

    for (receipt, platform, artifact, size) in [
        (&ubuntu, "ubuntu-24.04-x86_64", UBUNTU_ARTIFACT, 1_945_448),
        (&windows, "windows-x86_64", WINDOWS_ARTIFACT, 1_436_672),
    ] {
        let payload = &receipt["payload"];
        assert_eq!(payload["cutoff"], CUTOFF);
        assert_eq!(payload["platform"], platform);
        assert_eq!(payload["checkout"]["exact_commit"], true);
        assert_eq!(payload["checkout"]["core_autocrlf"], false);
        assert_eq!(payload["checkout"]["tracked_files_clean"], true);
        assert_eq!(
            payload["artifact"]["discovery"],
            "cargo-compiler-artifact-json"
        );
        assert_eq!(payload["artifact"]["sha256"], artifact);
        assert_eq!(payload["artifact"]["size"], size);
        assert_eq!(payload["artifact"]["retained_in_public_bundle"], false);
        assert_eq!(
            payload["build"]["command"][8],
            "--message-format=json-render-diagnostics"
        );
        assert_eq!(payload["safety"]["diagnostic_execution"], false);
        assert_eq!(payload["safety"]["product_files_modified"], false);
    }

    let seal = &seal["payload"];
    assert_eq!(seal["manifest"]["sha256"], MANIFEST_RAW);
    assert_eq!(
        seal["artifacts"]["qualification_receipt"]["sha256"],
        QUALIFICATION_RAW
    );
    assert_eq!(
        seal["artifacts"]["root_cause_report"]["sha256"],
        ROOT_CAUSE_RAW
    );
    assert_eq!(seal["qualification"]["unit_tests_passed"], 14);
    assert_eq!(seal["qualification"]["synthetic_checks_passed"], 20);
    assert_eq!(seal["qualification"]["clean_rebuilds_passed"], 4);
    assert_eq!(seal["qualification"]["actual_build_freezes_passed"], 2);
    assert_eq!(seal["product_change_required"], false);

    let ubuntu_output = read_json("evidence/ubuntu-adapter-output.json").1;
    let windows_output = read_json("evidence/windows-adapter-output.json").1;
    assert_eq!(ubuntu_output, ubuntu);
    assert_eq!(windows_output, windows);

    let integrity = read_json("evidence/final-build-integrity.json").1;
    assert_eq!(integrity["ubuntu_sha256"], UBUNTU_ARTIFACT);
    assert_eq!(integrity["windows_sha256"], WINDOWS_ARTIFACT);
    assert_eq!(integrity["ubuntu_retained"], false);
    assert_eq!(integrity["windows_retained"], false);
    assert_eq!(integrity["ferris_tracked_status"], Value::Null);
    assert_eq!(integrity["windows_checkout_tracked_status"], Value::Null);
    assert_eq!(integrity["ubuntu_checkout_tracked_status"], Value::Null);

    for relative in [
        "evidence/ubuntu-adapter-determinism.json",
        "evidence/windows-adapter-determinism.json",
    ] {
        let evidence = read_json(relative).1;
        assert_eq!(evidence["clean_builds"], 2);
        assert_eq!(evidence["same"], true);
    }
    assert_eq!(
        read_json("evidence/ubuntu-adapter-determinism.json").1["first_sha256"],
        UBUNTU_ARTIFACT
    );
    assert_eq!(
        read_json("evidence/windows-adapter-determinism.json").1["first_sha256"],
        WINDOWS_ARTIFACT
    );
    assert_eq!(
        read_json("evidence/windows-rebuild-determinism.json").1["same"],
        false
    );
    assert_eq!(
        read_json("evidence/windows-brepro-probe.json").1["same"],
        true
    );
    assert_eq!(
        read_json("evidence/line-ending-summary.json").1["identical_counts"],
        true
    );
    assert_eq!(
        read_json("evidence/path-translation-summary.json").1["translated_windows_checkout_metadata_exit"],
        0
    );

    let text = |relative: &str| {
        fs::read_to_string(release_root().join(relative)).expect("read text evidence")
    };
    assert!(text("evidence/ubuntu-toolchain-discovery.txt").contains("cargo_on_path=false"));
    assert!(text("evidence/ubuntu-toolchain-discovery.txt").contains("cargo_home_executable=true"));
    assert!(text("evidence/ubuntu-build-exit.txt").contains("127"));
    assert!(text("evidence/ubuntu-cargo-build.log").contains("cargo: command not found"));
    assert!(text("evidence/ubuntu-build-login-exit.txt").contains('0'));
    assert!(text("evidence/ubuntu-explicit-cargo-non-login-exit.txt").contains('0'));
    assert!(text("evidence/ubuntu-explicit-cargo-non-login.log").contains("Finished `release`"));
    assert!(text("evidence/unit-tests.log").contains("Ran 14 tests"));
    assert!(text("evidence/synthetic-checks.log").contains("\"passed\": 20"));

    let adapter = text("build_freeze.py");
    assert!(adapter.contains("Path.home() / \".cargo\" / \"bin\""));
    assert!(adapter.contains("--message-format=json-render-diagnostics"));
}

#[test]
fn pulse_33_governance_defers_pulse_34_until_the_release_commit_exists() {
    for relative in [
        "context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-33.md",
        "docs/plans/reviews/PULSE-33-PUBLIC-BUILD-FREEZE-RELEASE-ROLE-REVIEW.md",
    ] {
        let text = fs::read_to_string(repo_root().join(relative)).expect("Pulse 33 document");
        for required in [
            CUTOFF,
            MANIFEST_RAW,
            MANIFEST_AGGREGATE,
            QUALIFICATION_PAYLOAD,
            ROOT_CAUSE_PAYLOAD,
            SEAL_PAYLOAD,
            UBUNTU_ARTIFACT,
            WINDOWS_ARTIFACT,
            "14",
            "20",
            "four clean rebuilds",
            "37",
            "no diagnostic execution",
            "no product change",
            "future Pulse 33 commit",
            "No placeholder or self-containing cutoff",
        ] {
            assert!(text.contains(required), "{relative} missing {required}");
        }
    }
}
