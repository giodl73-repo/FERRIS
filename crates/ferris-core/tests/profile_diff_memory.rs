use ferris_core::{ProfileDiffRecord, profile_diff_error_envelope};
use ferris_core::{ResultClass, create_profile_diff, create_profile_diff_from_bytes};
use serde_json::{Value, json};
use std::{fs, path::PathBuf};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/profile-evidence")
}
fn example() -> Vec<u8> {
    fs::read(fixture_root().join("pure-data/before.json")).unwrap()
}

#[test]
fn profile_diff_memory_matches_all_nine_native_development_families() {
    let mut count = 0;
    for directory in fs::read_dir(fixture_root()).unwrap() {
        let root = directory.unwrap().path();
        if !root.is_dir() {
            continue;
        }
        let before = root.join("before.json");
        let after = root.join("after.json");
        let native = create_profile_diff(&before, &after).unwrap();
        let memory =
            create_profile_diff_from_bytes(&fs::read(before).unwrap(), &fs::read(after).unwrap());
        assert_eq!(
            serde_json::to_value(memory).unwrap(),
            serde_json::to_value(native).unwrap(),
            "{}",
            root.display()
        );
        count += 1;
    }
    assert_eq!(count, 9);
    let before = fixture_root().join("pure-data/before.json");
    let after = fixture_root().join("browser-wasm/before.json");
    let error =
        create_profile_diff(&before, &after).expect_err("different declared profile identity");
    let native = profile_diff_error_envelope::<ProfileDiffRecord>(&before, &after, &error);
    let memory =
        create_profile_diff_from_bytes(&fs::read(&before).unwrap(), &fs::read(&after).unwrap());
    assert_eq!(
        serde_json::to_value(memory).unwrap(),
        serde_json::to_value(native).unwrap()
    );
}

#[test]
fn profile_diff_memory_canonical_identity_ignores_formatting() {
    let before = example();
    let value: Value = serde_json::from_slice(&before).unwrap();
    let compact = serde_json::to_vec(&value).unwrap();
    let first = create_profile_diff_from_bytes(&before, &before);
    let second = create_profile_diff_from_bytes(&compact, &before);
    assert_eq!(first.result_class, ResultClass::Success);
    assert_eq!(
        serde_json::to_value(first).unwrap(),
        serde_json::to_value(second).unwrap()
    );
}

#[test]
fn profile_diff_memory_errors_keep_native_classes_and_duplicate_rejection() {
    let good = example();
    let mut unsupported: Value = serde_json::from_slice(&good).unwrap();
    unsupported["schema"] = json!("unsupported/v9");
    let mut mismatch: Value = serde_json::from_slice(&good).unwrap();
    mismatch["consumer"] = json!("other-consumer");
    for (input, class, code) in [
        (
            b"{".to_vec(),
            ResultClass::Invalid,
            "FERRIS-PROFILE-JSON-INVALID",
        ),
        (
            b"{\"x\":1,\"x\":2}".to_vec(),
            ResultClass::Invalid,
            "FERRIS-PROFILE-JSON-DUPLICATE-MEMBER",
        ),
        (
            b"{}".to_vec(),
            ResultClass::Invalid,
            "FERRIS-PROFILE-SHAPE-INVALID",
        ),
        (
            vec![b' '; 1024 * 1024 + 1],
            ResultClass::Incomplete,
            "FERRIS-PROFILE-INPUT-OVERSIZED",
        ),
        (
            serde_json::to_vec(&unsupported).unwrap(),
            ResultClass::Unsupported,
            "FERRIS-PROFILE-SCHEMA-UNSUPPORTED",
        ),
    ] {
        let result = create_profile_diff_from_bytes(&input, &good);
        assert_eq!(result.result_class, class);
        assert_eq!(result.diagnostics[0].code, code);
        assert!(result.record.is_none());
    }
    let result = create_profile_diff_from_bytes(&good, &serde_json::to_vec(&mismatch).unwrap());
    assert_eq!(result.result_class, ResultClass::Invalid);
    assert_eq!(
        result.diagnostics[0].code,
        "FERRIS-PROFILE-DIFF-CONSUMER-MISMATCH"
    );
}

#[test]
fn profile_diff_memory_failures_bind_both_ordered_inputs() {
    let good = example();
    let first = create_profile_diff_from_bytes(b"{", &good);
    let second = create_profile_diff_from_bytes(b"{", b"{}");
    let swapped = create_profile_diff_from_bytes(&good, b"{");
    assert_ne!(first.selection_identity, second.selection_identity);
    assert_ne!(first.selection_identity, swapped.selection_identity);
    assert_eq!(
        first.selection_identity,
        create_profile_diff_from_bytes(b"{", &good).selection_identity
    );
}

#[test]
fn profile_diff_memory_retains_redaction_and_pointer_escaping() {
    let before = example();
    let mut after: Value = serde_json::from_slice(&before).unwrap();
    after["sections"]["support"] = json!({"slash/~field": "RAW_SECTION_VALUE_MUST_NOT_ESCAPE"});
    let result = create_profile_diff_from_bytes(&before, &serde_json::to_vec(&after).unwrap());
    assert_eq!(result.result_class, ResultClass::Difference);
    let record = result.record.as_ref().unwrap();
    assert!(!record.executable);
    let encoded = serde_json::to_string(&result).unwrap();
    assert!(!encoded.contains("RAW_SECTION_VALUE_MUST_NOT_ESCAPE"));
    assert!(encoded.contains("slash~1~0field"));
}

#[test]
fn profile_diff_memory_blocks_over_limit_without_partial_diff() {
    let mut before: Value = serde_json::from_slice(&example()).unwrap();
    let mut after = before.clone();
    before["sections"]["features"] = json!([]);
    after["sections"]["features"] = json!(vec![0; 10001]);
    let result = create_profile_diff_from_bytes(
        &serde_json::to_vec(&before).unwrap(),
        &serde_json::to_vec(&after).unwrap(),
    );
    assert_eq!(result.result_class, ResultClass::Blocked);
    assert!(result.record.is_none());
}
