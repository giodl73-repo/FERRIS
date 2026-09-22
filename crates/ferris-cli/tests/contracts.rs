use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const CONTRACT_CATALOG_SCHEMA_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/schemas/contract-catalog/ferris.contract-catalog.v1.schema.json"
);
const CONTRACT_REQUIREMENTS_SCHEMA_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/schemas/contract-compatibility/ferris.contract-requirements.v1.schema.json"
);
const CONTRACT_COMPATIBILITY_SCHEMA_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/schemas/contract-compatibility/ferris.contract-compatibility-report.v1.schema.json"
);
const SYNTHETIC_REQUIREMENTS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/contracts/synthetic-corpus-v1.json"
);
const UNSATISFIED_REQUIREMENTS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/contracts/unsatisfied-v1.json"
);

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

fn cargo_ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cargo-ferris"))
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn contract_catalog_json_matches_across_entrypoints() {
    let direct = ferris()
        .args(["contracts", "--format", "json"])
        .output()
        .expect("run ferris contracts");
    let cargo_direct = cargo_ferris()
        .args(["contracts", "--format", "json"])
        .output()
        .expect("run cargo-ferris contracts");
    let cargo_style = cargo_ferris()
        .args(["ferris", "contracts", "--format", "json"])
        .output()
        .expect("run cargo ferris contracts");
    assert_success(&direct);
    assert_success(&cargo_direct);
    assert_success(&cargo_style);
    assert_eq!(direct.stdout, cargo_direct.stdout);
    assert_eq!(direct.stdout, cargo_style.stdout);

    let envelope: Value = serde_json::from_slice(&direct.stdout).expect("catalog JSON");
    assert_eq!(envelope["schema"], "ferris.command-result/v2");
    assert_eq!(envelope["semantic_command_id"], "contracts");
    assert_eq!(envelope["record"]["schema"], "ferris.contract-catalog/v1");
    assert_eq!(envelope["record"]["product_status"], "incubation");
    assert_eq!(
        envelope["record"]["compatibility_policy"],
        "exact-schema-identifier"
    );

    let contracts = envelope["record"]["contracts"]
        .as_array()
        .expect("contracts array");
    assert_eq!(contracts.len(), 45);
    assert!(contracts.windows(2).all(|pair| {
        pair[0]["schema"].as_str().expect("left schema")
            < pair[1]["schema"].as_str().expect("right schema")
    }));
    let legacy = contracts
        .iter()
        .find(|contract| contract["schema"] == "ferris.execution-receipt/v1")
        .expect("legacy receipt");
    assert_eq!(legacy["accepted"], true);
    assert_eq!(legacy["emitted"], false);
    assert_eq!(legacy["lifecycle"], "legacy_read_only");
    let requirements = contracts
        .iter()
        .find(|contract| contract["schema"] == "ferris.contract-requirements/v1")
        .expect("requirements contract");
    assert_eq!(requirements["accepted"], true);
    assert_eq!(requirements["emitted"], false);
    let report = contracts
        .iter()
        .find(|contract| contract["schema"] == "ferris.contract-compatibility-report/v1")
        .expect("compatibility report contract");
    assert_eq!(report["accepted"], false);
    assert_eq!(report["emitted"], true);
}

#[test]
fn synthetic_corpus_requirements_match_across_entrypoints() {
    let arguments = [
        "contracts",
        "--requirements",
        SYNTHETIC_REQUIREMENTS_PATH,
        "--format",
        "json",
    ];
    let direct = ferris()
        .args(arguments)
        .output()
        .expect("run ferris contracts");
    let cargo_direct = cargo_ferris()
        .args(arguments)
        .output()
        .expect("run cargo-ferris contracts");
    let cargo_style = cargo_ferris()
        .args(std::iter::once("ferris").chain(arguments))
        .output()
        .expect("run cargo ferris contracts");
    assert_success(&direct);
    assert_success(&cargo_direct);
    assert_success(&cargo_style);
    assert_eq!(direct.stdout, cargo_direct.stdout);
    assert_eq!(direct.stdout, cargo_style.stdout);

    let envelope: Value = serde_json::from_slice(&direct.stdout).expect("compatibility JSON");
    assert_eq!(envelope["result_class"], "success");
    assert_eq!(
        envelope["record"]["schema"],
        "ferris.contract-compatibility-report/v1"
    );
    assert_eq!(envelope["record"]["consumer_id"], "synthetic/corpus-v1");
    assert_eq!(envelope["record"]["compatible"], true);
    assert_eq!(envelope["record"]["unsatisfied_count"], 0);
    assert_eq!(
        envelope["record"]["requirements"]
            .as_array()
            .expect("requirements")
            .len(),
        4
    );
}

#[test]
fn unsatisfied_requirements_emit_a_report_bearing_difference() {
    let output = ferris()
        .args([
            "contracts",
            "--requirements",
            UNSATISFIED_REQUIREMENTS_PATH,
            "--format",
            "json",
        ])
        .output()
        .expect("run incompatible requirements");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let envelope: Value = serde_json::from_slice(&output.stdout).expect("difference JSON");
    assert_eq!(envelope["result_class"], "difference");
    assert_eq!(envelope["record"]["compatible"], false);
    assert_eq!(envelope["record"]["unsatisfied_count"], 2);
    assert_eq!(
        envelope["diagnostics"][0]["code"],
        "FERRIS-CONTRACT-REQUIREMENTS-UNSATISFIED"
    );
}

#[test]
fn unavailable_requirements_are_typed_and_path_private() {
    let private_marker = "private-contract-requirements-do-not-disclose.json";
    let output = ferris()
        .args([
            "contracts",
            "--requirements",
            private_marker,
            "--format",
            "json",
        ])
        .output()
        .expect("run unavailable requirements");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("error JSON");
    assert!(!stderr.contains(private_marker));
    let envelope: Value = serde_json::from_str(&stderr).expect("error envelope");
    assert_eq!(envelope["result_class"], "invalid");
    assert_eq!(
        envelope["diagnostics"][0]["code"],
        "FERRIS-CONTRACT-REQUIREMENTS-UNAVAILABLE"
    );
}

#[test]
fn contract_catalog_human_output_states_boundaries() {
    let output = ferris()
        .arg("contracts")
        .output()
        .expect("run human catalog");
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("human output");
    assert!(stdout.starts_with("Ferris contract catalog contract-catalog:"));
    assert!(stdout.contains("Unknown schemas: reject"));
    assert!(stdout.contains("ferris.execution-receipt/v1 (accepted; legacy read-only)"));
    assert!(stdout.contains("not production support"));
}

#[test]
fn checked_in_schema_describes_the_emitted_catalog_shape() {
    let schema: Value = serde_json::from_slice(
        &fs::read(Path::new(CONTRACT_CATALOG_SCHEMA_PATH)).expect("read catalog schema"),
    )
    .expect("parse catalog schema");
    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(
        schema["properties"]["schema"]["const"],
        "ferris.contract-catalog/v1"
    );

    let output = ferris()
        .args(["contracts", "--format", "json"])
        .output()
        .expect("run catalog");
    assert_success(&output);
    let envelope: Value = serde_json::from_slice(&output.stdout).expect("catalog JSON");
    let record = envelope["record"].as_object().expect("catalog record");
    for field in schema["required"].as_array().expect("required fields") {
        assert!(
            record.contains_key(field.as_str().expect("required field name")),
            "missing required field {field}"
        );
    }
    assert!(record.keys().all(|field| {
        schema["properties"]
            .as_object()
            .expect("schema properties")
            .contains_key(field)
    }));
}

#[test]
fn checked_in_schemas_describe_requirements_and_compatibility_report() {
    let requirements_schema: Value = serde_json::from_slice(
        &fs::read(Path::new(CONTRACT_REQUIREMENTS_SCHEMA_PATH)).expect("read requirements schema"),
    )
    .expect("parse requirements schema");
    let report_schema: Value = serde_json::from_slice(
        &fs::read(Path::new(CONTRACT_COMPATIBILITY_SCHEMA_PATH)).expect("read report schema"),
    )
    .expect("parse report schema");
    for schema in [&requirements_schema, &report_schema] {
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
        assert_eq!(schema["additionalProperties"], false);
    }

    let requirements: Value = serde_json::from_slice(
        &fs::read(Path::new(SYNTHETIC_REQUIREMENTS_PATH)).expect("read requirements fixture"),
    )
    .expect("parse requirements fixture");
    for field in requirements_schema["required"]
        .as_array()
        .expect("requirements required fields")
    {
        assert!(
            requirements
                .get(field.as_str().expect("field name"))
                .is_some()
        );
    }

    let output = ferris()
        .args([
            "contracts",
            "--requirements",
            SYNTHETIC_REQUIREMENTS_PATH,
            "--format",
            "json",
        ])
        .output()
        .expect("run compatibility report");
    assert_success(&output);
    let envelope: Value = serde_json::from_slice(&output.stdout).expect("report JSON");
    let record = envelope["record"].as_object().expect("report record");
    for field in report_schema["required"]
        .as_array()
        .expect("report required fields")
    {
        assert!(record.contains_key(field.as_str().expect("field name")));
    }
    assert!(record.keys().all(|field| {
        report_schema["properties"]
            .as_object()
            .expect("report properties")
            .contains_key(field)
    }));
}
