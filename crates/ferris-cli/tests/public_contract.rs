use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/simulations/profile-diff-held-out/fixtures"
);
const SCHEMAS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/simulations/profile-diff-held-out/schemas"
);

fn fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(Path::new(FIXTURES).join(name)).expect("read fixture"))
        .expect("parse fixture")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn framed_identity(prefix: &str, parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0]);
    }
    format!("{prefix}:{:x}", hasher.finalize())
}

fn lexically_normalize_path(value: &str) -> String {
    let normalized = value
        .strip_prefix(r"\\?\")
        .unwrap_or(value)
        .replace('\\', "/");
    let (prefix, remainder, rooted) = if let Some(remainder) = normalized.strip_prefix("//") {
        ("//", remainder, true)
    } else if let Some(remainder) = normalized.strip_prefix('/') {
        ("/", remainder, true)
    } else if normalized.len() >= 3
        && normalized.as_bytes()[1] == b':'
        && normalized.as_bytes()[2] == b'/'
    {
        (&normalized[..3], &normalized[3..], true)
    } else {
        ("", normalized.as_str(), false)
    };
    let mut components = Vec::new();
    for component in remainder.split('/') {
        match component {
            "" | "." => {}
            ".." if components.last().is_some_and(|last| *last != "..") => {
                components.pop();
            }
            ".." if !rooted => components.push(component),
            ".." => {}
            _ => components.push(component),
        }
    }
    let joined = components.join("/");
    if prefix.is_empty() {
        if joined.is_empty() {
            ".".to_owned()
        } else {
            joined
        }
    } else if joined.is_empty() {
        prefix.to_owned()
    } else if prefix.ends_with('/') {
        format!("{prefix}{joined}")
    } else {
        format!("{prefix}/{joined}")
    }
}

fn request_digest(value: &str) -> String {
    sha256(lexically_normalize_path(value).as_bytes())
}

#[derive(Deserialize, Serialize)]
struct ProfileEvidence {
    schema: String,
    profile_id: String,
    revision: String,
    consumer: String,
    sections: ProfileSections,
}

#[derive(Deserialize, Serialize)]
struct ProfileSections {
    identity: Value,
    closure: Value,
    features: Value,
    toolchain: Value,
    targets: Value,
    providers: Value,
    native: Value,
    stages: Value,
    assurance: Value,
    stewardship: Value,
    support: Value,
    lifecycle: Value,
}

#[derive(Deserialize, Serialize)]
struct ProfileReference {
    profile_id: String,
    revision: String,
    consumer: String,
    content_digest: String,
}

#[derive(Deserialize, Serialize)]
struct ProfileChange {
    path: String,
    change_kind: String,
    before_value_digest: Option<String>,
    after_value_digest: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct ProfileDiffIdentityPayload {
    schema: String,
    diff_id: String,
    before: ProfileReference,
    after: ProfileReference,
    changed_sections: Vec<String>,
    changes: Vec<ProfileChange>,
    unchanged_sections: Vec<String>,
    unknowns: Vec<String>,
    limitations: Vec<String>,
    executable: bool,
}

#[derive(Deserialize, Serialize)]
struct Diagnostic {
    code: String,
    severity: String,
    result_class: String,
    message: String,
    source_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bounded_output: Option<Value>,
    next_actions: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct ResultIdentityPayload {
    schema: String,
    command_version: String,
    semantic_command_id: String,
    selection_identity: String,
    invocation_identity: String,
    result_class: String,
    process_exit_code: u8,
    diagnostics: Vec<Diagnostic>,
    record: Option<ProfileDiffIdentityPayload>,
}

#[derive(Deserialize, Serialize)]
struct AggregateRow {
    platform: String,
    declared_case_id: String,
    attempt: u64,
    stdout_digest: String,
    stderr_digest: String,
    process_exit_code: u8,
}

#[derive(Serialize)]
struct AggregatePayload {
    schema: &'static str,
    contract_revision: u64,
    rows: Vec<AggregateRow>,
}

struct ContractDirectory(PathBuf);

impl ContractDirectory {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/public-contract-tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create contract directory");
        Self(path)
    }
}

impl Drop for ContractDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn write_json(path: &Path, value: &Value) {
    let mut bytes = serde_json::to_vec_pretty(value).expect("serialize source profile");
    bytes.push(b'\n');
    fs::write(path, bytes).expect("write source profile");
}

fn ferris(directory: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
        .current_dir(directory)
        .args(args)
        .output()
        .expect("run ferris")
}

fn normalized_cli_parts(args: &[String]) -> Vec<String> {
    let mut normalized = Vec::new();
    let mut arguments = args.iter().skip(1);
    if arguments.clone().next().is_some_and(|argument| {
        matches!(
            argument.as_str(),
            "plan" | "explain" | "graph" | "doctor" | "profile-diff"
        )
    }) {
        arguments.next();
    }
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--before" | "--after" => {
                normalized.push(format!("option:{}", argument.trim_start_matches('-')));
                normalized.push(
                    arguments
                        .next()
                        .map(|value| format!("value:{}", request_digest(value)))
                        .unwrap_or_else(|| "missing-value".to_owned()),
                );
            }
            "--format" => {
                normalized.push("option:format".to_owned());
                normalized.push(
                    arguments
                        .next()
                        .map(|value| format!("value:{}", value.to_ascii_lowercase()))
                        .unwrap_or_else(|| "missing-value".to_owned()),
                );
            }
            value if value.starts_with("--before=") || value.starts_with("--after=") => {
                let (option, path) = value.split_once('=').expect("assigned option");
                normalized.push(format!("option:{option}"));
                normalized.push(format!("value:{}", request_digest(path)));
            }
            value if value.starts_with("--format=") => {
                normalized.push("option:format".to_owned());
                normalized.push(format!(
                    "value:{}",
                    value.trim_start_matches("--format=").to_ascii_lowercase()
                ));
            }
            value if value.starts_with('-') => {
                if let Some((option, option_value)) = value.split_once('=') {
                    normalized.push(format!("option:{option}"));
                    normalized.push(format!("value:{}", sha256(option_value.as_bytes())));
                } else {
                    normalized.push(format!("option:{value}"));
                }
            }
            value => normalized.push(format!("argument:{}", sha256(value.as_bytes()))),
        }
    }
    normalized
}

#[test]
fn public_identity_vectors_recompute_and_match_current_cli() {
    let vectors = fixture("identity-vectors.json");

    for vector in vectors["path_normalization"]
        .as_array()
        .expect("path vectors")
    {
        let input = vector["input"].as_str().expect("path input");
        assert_eq!(lexically_normalize_path(input), vector["expected"]);
        assert_eq!(request_digest(input), vector["request_digest"]);
    }

    for vector in vectors["content_digests"]
        .as_array()
        .expect("content vectors")
    {
        let profile: ProfileEvidence =
            serde_json::from_value(vector["source_value"].clone()).expect("profile value");
        let bytes = serde_json::to_vec(&profile).expect("canonical profile");
        assert_eq!(
            String::from_utf8(bytes.clone()).unwrap(),
            vector["canonical_json"]
        );
        assert_eq!(sha256(&bytes), vector["expected"]);
    }

    for vector in vectors["value_digests"].as_array().expect("value vectors") {
        let bytes = serde_json::to_vec(&vector["source_value"]).expect("canonical value");
        assert_eq!(
            String::from_utf8(bytes.clone()).unwrap(),
            vector["canonical_json"]
        );
        assert_eq!(sha256(&bytes), vector["expected"]);
    }

    for vector in vectors["selection_identities"]
        .as_array()
        .expect("selection vectors")
    {
        let expected = match vector["branch"].as_str().expect("selection branch") {
            "content" => framed_identity(
                "selection",
                &[
                    "profile-diff-selection",
                    vector["before_content_digest"].as_str().unwrap(),
                    vector["after_content_digest"].as_str().unwrap(),
                ],
            ),
            "pre_read" => {
                let material = format!(
                    "before-request={};after-request={}",
                    request_digest(vector["before_path"].as_str().unwrap()),
                    request_digest(vector["after_path"].as_str().unwrap())
                );
                assert_eq!(material, vector["selection_material"]);
                framed_identity("selection", &["profile-diff-selection", &material])
            }
            "second_read" => {
                let material = format!(
                    "before={};after-request={}",
                    vector["before_content_digest"].as_str().unwrap(),
                    request_digest(vector["after_path"].as_str().unwrap())
                );
                assert_eq!(material, vector["selection_material"]);
                framed_identity("selection", &["profile-diff-selection", &material])
            }
            branch => panic!("unknown selection branch {branch}"),
        };
        assert_eq!(expected, vector["expected"]);
    }

    for vector in vectors["invocation_identities"]
        .as_array()
        .expect("invocation vectors")
    {
        let parts = vector["parts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|part| part.as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(framed_identity("invocation", &parts), vector["expected"]);
    }

    let cli = &vectors["command_line_identity"];
    let args = cli["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|arg| arg.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let normalized = normalized_cli_parts(&args);
    assert_eq!(
        normalized,
        cli["normalized_parts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|part| part.as_str().unwrap().to_owned())
            .collect::<Vec<_>>()
    );
    let normalized_digest = sha256(normalized.join("\0").as_bytes());
    assert_eq!(normalized_digest, cli["normalized_digest"]);
    assert_eq!(
        framed_identity(
            "selection",
            &["selection", "profile-diff", &normalized_digest]
        ),
        cli["selection_identity"]
    );
    let mut invocation_parts = vec!["profile-diff"];
    invocation_parts.extend(normalized.iter().map(String::as_str));
    assert_eq!(
        framed_identity("invocation", &invocation_parts),
        cli["invocation_identity"]
    );

    for vector in vectors["diff_identities"].as_array().expect("diff vectors") {
        let payload: ProfileDiffIdentityPayload =
            serde_json::from_value(vector["identity_payload"].clone()).expect("diff payload");
        let identity = format!(
            "profile-diff:{:x}",
            Sha256::digest(serde_json::to_vec(&payload).unwrap())
        );
        assert_eq!(identity, vector["expected"]);
    }
    for vector in vectors["result_identities"]
        .as_array()
        .expect("result vectors")
    {
        let payload: ResultIdentityPayload =
            serde_json::from_value(vector["identity_payload"].clone()).expect("result payload");
        let identity = format!(
            "result:{:x}",
            Sha256::digest(serde_json::to_vec(&payload).unwrap())
        );
        assert_eq!(identity, vector["expected"]);
    }

    let aggregate = &vectors["aggregate_identity"];
    let mut rows = aggregate["source_rows"]
        .as_array()
        .unwrap()
        .iter()
        .cloned()
        .map(|row| serde_json::from_value::<AggregateRow>(row).unwrap())
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        (
            left.platform.as_str(),
            left.declared_case_id.as_str(),
            left.attempt,
        )
            .cmp(&(
                right.platform.as_str(),
                right.declared_case_id.as_str(),
                right.attempt,
            ))
    });
    let payload = AggregatePayload {
        schema: "ferris.aggregate-public-output/v1",
        contract_revision: 2,
        rows,
    };
    assert_eq!(
        serde_json::to_value(&payload).unwrap(),
        aggregate["sorted_payload"]
    );
    let mut bytes = b"ferris.aggregate-public-output/v1\0".to_vec();
    bytes.extend(serde_json::to_vec(&payload).unwrap());
    assert_eq!(sha256(&bytes), aggregate["expected"]);

    let directory = ContractDirectory::new();
    let before = vectors["content_digests"][0]["source_value"].clone();
    let after = vectors["content_digests"][1]["source_value"].clone();
    write_json(&directory.0.join("before.json"), &before);
    write_json(&directory.0.join("after.json"), &after);
    let output = ferris(
        &directory.0,
        &[
            "profile-diff",
            "--before",
            "before.json",
            "--after",
            "after.json",
            "--format",
            "json",
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    let actual: Value = serde_json::from_slice(&output.stdout).expect("CLI JSON");
    assert_eq!(actual, fixture("command-result-difference.json"));

    let success = ferris(
        &directory.0,
        &[
            "profile-diff",
            "--before",
            "before.json",
            "--after",
            "before.json",
            "--format",
            "json",
        ],
    );
    assert_eq!(success.status.code(), Some(0));
    assert_eq!(
        serde_json::from_slice::<Value>(&success.stdout).unwrap(),
        fixture("command-result-success.json")
    );

    let pre_read = ferris(
        &directory.0,
        &[
            "profile-diff",
            "--before",
            "missing/dir/../before-missing.json",
            "--after",
            "profiles/./after.json",
            "--format",
            "json",
        ],
    );
    assert_eq!(pre_read.status.code(), Some(5));
    assert_eq!(
        serde_json::from_slice::<Value>(&pre_read.stderr).unwrap(),
        fixture("command-result-incomplete.json")
    );

    let second_read = ferris(
        &directory.0,
        &[
            "profile-diff",
            "--before",
            "before.json",
            "--after",
            "missing/../absent.json",
            "--format",
            "json",
        ],
    );
    assert_eq!(second_read.status.code(), Some(5));
    let second_actual: Value = serde_json::from_slice(&second_read.stderr).unwrap();
    let second_vector = vectors["result_identities"]
        .as_array()
        .unwrap()
        .iter()
        .find(|vector| vector["id"] == "second-input-incomplete")
        .unwrap();
    for field in [
        "selection_identity",
        "invocation_identity",
        "result_class",
        "process_exit_code",
        "diagnostics",
        "record",
    ] {
        assert_eq!(
            second_actual[field],
            second_vector["identity_payload"][field]
        );
    }
    assert_eq!(second_actual["result_identity"], second_vector["expected"]);

    let cli_args = cli["args"]
        .as_array()
        .unwrap()
        .iter()
        .skip(1)
        .map(|argument| argument.as_str().unwrap())
        .collect::<Vec<_>>();
    let cli_invalid = ferris(&directory.0, &cli_args);
    assert_eq!(cli_invalid.status.code(), Some(2));
    let cli_actual: Value = serde_json::from_slice(&cli_invalid.stderr).unwrap();
    assert_eq!(cli_actual["selection_identity"], cli["selection_identity"]);
    assert_eq!(
        cli_actual["invocation_identity"],
        cli["invocation_identity"]
    );
}

fn parse_complete_json_line(bytes: &[u8]) -> Result<Value, String> {
    if !bytes.ends_with(b"\n") || bytes.len() < 2 || bytes[bytes.len() - 2] != b'}' {
        return Err("one terminal LF must immediately follow the JSON object".to_owned());
    }
    let body = &bytes[..bytes.len() - 1];
    std::str::from_utf8(body).map_err(|_| "not UTF-8".to_owned())?;
    let mut deserializer = serde_json::Deserializer::from_slice(body);
    let value = Value::deserialize(&mut deserializer).map_err(|error| error.to_string())?;
    deserializer.end().map_err(|error| error.to_string())?;
    Ok(value)
}

fn human_from_record(envelope: &Value) -> String {
    let record = &envelope["record"];
    let mut output = format!(
        "Ferris profile diff {}\nSchema: {}\nResult: {}\nExecutable: {}\nBefore: profile_id={}, revision={}, consumer={}, content_digest={}\nAfter: profile_id={}, revision={}, consumer={}, content_digest={}\nChanged sections:\n",
        record["diff_id"].as_str().unwrap(),
        record["schema"].as_str().unwrap(),
        envelope["result_class"].as_str().unwrap(),
        record["executable"].as_bool().unwrap(),
        record["before"]["profile_id"].as_str().unwrap(),
        record["before"]["revision"].as_str().unwrap(),
        record["before"]["consumer"].as_str().unwrap(),
        record["before"]["content_digest"].as_str().unwrap(),
        record["after"]["profile_id"].as_str().unwrap(),
        record["after"]["revision"].as_str().unwrap(),
        record["after"]["consumer"].as_str().unwrap(),
        record["after"]["content_digest"].as_str().unwrap(),
    );
    let changed_sections = record["changed_sections"].as_array().unwrap();
    if changed_sections.is_empty() {
        output.push_str("  - none\n");
    } else {
        for section in changed_sections {
            output.push_str(&format!("  - {}\n", section.as_str().unwrap()));
        }
    }
    output.push_str("Changes:\n");
    let changes = record["changes"].as_array().unwrap();
    if changes.is_empty() {
        output.push_str("  - none\n");
    } else {
        for change in changes {
            output.push_str(&format!(
                "  - {}: {} (before_digest={}, after_digest={})\n",
                change["path"].as_str().unwrap(),
                change["change_kind"].as_str().unwrap(),
                change["before_value_digest"].as_str().unwrap_or("none"),
                change["after_value_digest"].as_str().unwrap_or("none"),
            ));
        }
    }
    output.push_str("Unchanged sections:\n");
    let unchanged_sections = record["unchanged_sections"].as_array().unwrap();
    if unchanged_sections.is_empty() {
        output.push_str("  - none\n");
    } else {
        for section in unchanged_sections {
            output.push_str(&format!("  - {}\n", section.as_str().unwrap()));
        }
    }
    output.push_str("Unknowns:\n");
    let unknowns = record["unknowns"].as_array().unwrap();
    if unknowns.is_empty() {
        output.push_str("  - none\n");
    } else {
        for unknown in unknowns {
            output.push_str(&format!("  - {}\n", unknown.as_str().unwrap()));
        }
    }
    output.push_str("Limitations:\n");
    for limitation in record["limitations"].as_array().unwrap() {
        output.push_str(&format!("  - {}\n", limitation.as_str().unwrap()));
    }
    output
}

#[test]
fn public_stream_and_human_contract_is_exact() {
    let vectors = fixture("identity-vectors.json");
    let directory = ContractDirectory::new();
    write_json(
        &directory.0.join("before.json"),
        &vectors["content_digests"][0]["source_value"],
    );
    write_json(
        &directory.0.join("after.json"),
        &vectors["content_digests"][1]["source_value"],
    );

    let json = ferris(
        &directory.0,
        &[
            "profile-diff",
            "--before",
            "before.json",
            "--after",
            "after.json",
            "--format",
            "json",
        ],
    );
    assert_eq!(json.status.code(), Some(1));
    assert!(json.stderr.is_empty());
    let envelope = parse_complete_json_line(&json.stdout).expect("complete JSON line");
    let mut double_newline = json.stdout.clone();
    double_newline.push(b'\n');
    assert!(parse_complete_json_line(&double_newline).is_err());
    let mut trailing = json.stdout.clone();
    trailing.extend_from_slice(b"trailing");
    assert!(parse_complete_json_line(&trailing).is_err());

    let human = ferris(
        &directory.0,
        &[
            "profile-diff",
            "--before",
            "before.json",
            "--after",
            "after.json",
            "--format",
            "human",
        ],
    );
    assert_eq!(human.status.code(), Some(1));
    assert!(human.stderr.is_empty());
    assert_eq!(
        String::from_utf8(human.stdout).unwrap(),
        human_from_record(&envelope)
    );

    let success_json = ferris(
        &directory.0,
        &[
            "profile-diff",
            "--before",
            "before.json",
            "--after",
            "before.json",
            "--format",
            "json",
        ],
    );
    let success_envelope = parse_complete_json_line(&success_json.stdout).unwrap();
    let success_human = ferris(
        &directory.0,
        &[
            "profile-diff",
            "--before",
            "before.json",
            "--after",
            "before.json",
            "--format",
            "human",
        ],
    );
    assert_eq!(success_human.status.code(), Some(0));
    assert_eq!(
        String::from_utf8(success_human.stdout).unwrap(),
        human_from_record(&success_envelope)
    );

    let failure = ferris(
        &directory.0,
        &[
            "profile-diff",
            "--before",
            "missing.json",
            "--after",
            "after.json",
            "--format",
            "human",
        ],
    );
    assert_eq!(failure.status.code(), Some(5));
    assert!(failure.stdout.is_empty());
    assert_eq!(
        parse_complete_json_line(&failure.stderr).unwrap()["process_exit_code"],
        5
    );
}

fn receipt_identity(domain: &str, receipt: &Value, field: &str) -> String {
    let mut payload = receipt.clone();
    payload[field] = Value::String(String::new());
    let mut bytes = format!("{domain}\0").into_bytes();
    bytes.extend(serde_json::to_vec(&payload).unwrap());
    sha256(&bytes)
}

fn allowed_keys(value: &Value, keys: &[&str]) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    let allowed = keys.iter().copied().collect::<BTreeSet<_>>();
    object.keys().all(|key| allowed.contains(key.as_str()))
        && keys.iter().all(|key| object.contains_key(*key))
}

fn validate_profile_diff(record: &Value) -> bool {
    if !allowed_keys(
        record,
        &[
            "schema",
            "diff_id",
            "before",
            "after",
            "changed_sections",
            "changes",
            "unchanged_sections",
            "unknowns",
            "limitations",
            "executable",
        ],
    ) || record["schema"] != "ferris.profile-diff/v0"
        || record["executable"] != false
    {
        return false;
    }
    let Some(changes) = record["changes"].as_array() else {
        return false;
    };
    changes.iter().all(|change| {
        if !allowed_keys(
            change,
            &[
                "path",
                "change_kind",
                "before_value_digest",
                "after_value_digest",
            ],
        ) {
            return false;
        }
        match change["change_kind"].as_str() {
            Some("added") => {
                change["before_value_digest"].is_null() && change["after_value_digest"].is_string()
            }
            Some("removed") => {
                change["before_value_digest"].is_string() && change["after_value_digest"].is_null()
            }
            Some("changed") => {
                change["before_value_digest"].is_string()
                    && change["after_value_digest"].is_string()
            }
            _ => false,
        }
    })
}

fn validate_command_result(value: &Value) -> bool {
    if !allowed_keys(
        value,
        &[
            "schema",
            "command_version",
            "semantic_command_id",
            "selection_identity",
            "invocation_identity",
            "result_identity",
            "result_class",
            "process_exit_code",
            "diagnostics",
            "record",
        ],
    ) || value["schema"] != "ferris.command-result/v2"
        || value["semantic_command_id"] != "profile-diff"
    {
        return false;
    }
    let expected_exit = match value["result_class"].as_str() {
        Some("success") => 0,
        Some("difference") => 1,
        Some("invalid") => 2,
        Some("unsupported") => 4,
        Some("incomplete") => 5,
        Some("blocked") => 7,
        Some("internal") => 11,
        _ => return false,
    };
    if value["process_exit_code"] != expected_exit {
        return false;
    }
    let Some(diagnostics) = value["diagnostics"].as_array() else {
        return false;
    };
    if expected_exit <= 1 {
        diagnostics.is_empty() && validate_profile_diff(&value["record"])
    } else {
        value["record"].is_null()
            && diagnostics.len() == 1
            && allowed_keys(
                &diagnostics[0],
                &[
                    "code",
                    "severity",
                    "result_class",
                    "message",
                    "source_digest",
                    "next_actions",
                ],
            )
            && diagnostics[0]["result_class"] == value["result_class"]
    }
}

fn validate_collection_row(row: &Value) -> bool {
    if !allowed_keys(
        row,
        &[
            "schema",
            "collection_id",
            "row_identity",
            "declaration",
            "expected",
            "executable",
            "command",
            "current_directory",
            "environment_digest",
            "environment_allowlist",
            "started_at",
            "completed_at",
            "duration_millis",
            "launch",
            "stdout",
            "stderr",
            "process_exit_code",
        ],
    ) || row["schema"] != "ferris.profile-diff-collection-row/v1"
        || !allowed_keys(
            &row["declaration"],
            &[
                "declared_case_id",
                "case_number",
                "case_class",
                "platform",
                "format",
                "attempt",
            ],
        )
        || row["declaration"]["attempt"] != 1
    {
        return false;
    }
    match row["launch"]["status"].as_str() {
        Some("started") => row["process_exit_code"].is_u64(),
        Some("failed") => row["process_exit_code"].is_null(),
        _ => false,
    }
}

fn strict_object_schemas(value: &Value) -> bool {
    match value {
        Value::Object(object) => {
            if object.get("type") == Some(&Value::String("object".to_owned()))
                && object.get("additionalProperties") != Some(&Value::Bool(false))
            {
                return false;
            }
            object.values().all(strict_object_schemas)
        }
        Value::Array(array) => array.iter().all(strict_object_schemas),
        _ => true,
    }
}

fn pointer_tokens(pointer: &str) -> Vec<String> {
    pointer
        .split('/')
        .skip(1)
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .collect()
}

fn mutate(value: &mut Value, operation: &str, pointer: &str, replacement: Option<Value>) {
    let tokens = pointer_tokens(pointer);
    let (last, parents) = tokens.split_last().expect("non-root pointer");
    let mut current = value;
    for token in parents {
        current = if let Ok(index) = token.parse::<usize>() {
            &mut current.as_array_mut().unwrap()[index]
        } else {
            current.as_object_mut().unwrap().get_mut(token).unwrap()
        };
    }
    if let Ok(index) = last.parse::<usize>() {
        let array = current.as_array_mut().unwrap();
        if operation == "remove" {
            array.remove(index);
        } else {
            array[index] = replacement.unwrap();
        }
    } else {
        let object = current.as_object_mut().unwrap();
        match operation {
            "remove" => {
                object.remove(last);
            }
            "add" | "replace" => {
                object.insert(last.clone(), replacement.unwrap());
            }
            _ => panic!("unknown mutation"),
        }
    }
}

fn disposition(inputs: &Value) -> &'static str {
    if !inputs["qualification_valid"].as_bool().unwrap()
        || !inputs["cardinality_valid"].as_bool().unwrap()
        || !inputs["initial_checkout_clean"].as_bool().unwrap()
        || !inputs["rollback_exact"].as_bool().unwrap()
        || !inputs["removal_complete"].as_bool().unwrap()
        || !inputs["cleanup_clean"].as_bool().unwrap()
    {
        "invalid"
    } else if !inputs["environment_supported"].as_bool().unwrap() {
        "unsupported"
    } else if !inputs["external_prerequisite_ready"].as_bool().unwrap() {
        "blocked"
    } else if [
        "omissions",
        "promotions",
        "privacy_hits",
        "prohibited_conclusions",
        "unexpected_changes",
        "output_bound_failures",
    ]
    .iter()
    .any(|name| inputs[*name].as_u64().unwrap() != 0)
    {
        "fail"
    } else {
        "pass"
    }
}

#[test]
fn public_preflight_vectors_enforce_cardinality_schemas_and_dispositions() {
    for entry in fs::read_dir(SCHEMAS).expect("schema directory") {
        let path = entry.expect("schema entry").path();
        if path.extension().and_then(|value| value.to_str()) == Some("json") {
            let schema: Value = serde_json::from_slice(&fs::read(&path).expect("read schema"))
                .expect("schema JSON");
            assert_eq!(
                schema["$schema"],
                "https://json-schema.org/draft/2020-12/schema"
            );
            assert!(strict_object_schemas(&schema), "{}", path.display());
        }
    }

    let preflight = fixture("preflight-vectors.json");
    let rows = preflight["rows"].as_array().expect("preflight rows");
    assert_eq!(rows.len() as u64, preflight["archetype_row_count"]);
    let mut keys = BTreeSet::new();
    let mut stream_branches = BTreeSet::new();
    let mut saw_nonzero = false;
    for vector in rows {
        let row = &vector["receipt"];
        assert!(validate_collection_row(row));
        assert_eq!(
            receipt_identity("ferris.profile-diff-collection-row/v1", row, "row_identity"),
            row["row_identity"]
        );
        let stdout = vector["stdout_utf8"].as_str().unwrap().as_bytes();
        let stderr = vector["stderr_utf8"].as_str().unwrap().as_bytes();
        assert_eq!(sha256(stdout), row["stdout"]["digest"]);
        assert_eq!(sha256(stderr), row["stderr"]["digest"]);
        assert_eq!(stdout.len() as u64, row["stdout"]["byte_count"]);
        assert_eq!(stderr.len() as u64, row["stderr"]["byte_count"]);
        keys.insert((
            row["declaration"]["declared_case_id"]
                .as_str()
                .unwrap()
                .to_owned(),
            row["declaration"]["platform"].as_str().unwrap().to_owned(),
        ));
        stream_branches.insert((stdout.is_empty(), stderr.is_empty()));
        saw_nonzero |= row["process_exit_code"].as_u64().unwrap() != 0;
    }
    assert_eq!(keys.len(), rows.len());
    assert_eq!(stream_branches.len(), 4);
    assert!(saw_nonzero);

    let environments = preflight["environment_receipts"].as_array().unwrap();
    assert_eq!(environments.len(), 2);
    for environment in environments {
        assert_eq!(
            receipt_identity(
                "ferris.profile-diff-environment-receipt/v1",
                environment,
                "receipt_identity"
            ),
            environment["receipt_identity"]
        );
    }

    let expected_keys = (1_u64..=preflight["declared_case_count"].as_u64().unwrap())
        .flat_map(|case_number| {
            preflight["platforms"]
                .as_array()
                .unwrap()
                .iter()
                .map(move |platform| {
                    (
                        format!("case-{case_number:03}"),
                        platform.as_str().unwrap().to_owned(),
                    )
                })
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        expected_keys.len() as u64,
        preflight["expected_scored_row_count"]
    );
    let complete_collection = expected_keys.iter().cloned().collect::<Vec<_>>();
    assert_eq!(
        complete_collection.iter().cloned().collect::<BTreeSet<_>>(),
        expected_keys
    );
    let mut missing_collection = complete_collection.clone();
    missing_collection.pop();
    assert_ne!(
        missing_collection.iter().cloned().collect::<BTreeSet<_>>(),
        expected_keys
    );
    let mut extra_collection = complete_collection.clone();
    extra_collection.push(("case-057".to_owned(), "windows-x86_64".to_owned()));
    assert_ne!(
        extra_collection.iter().cloned().collect::<BTreeSet<_>>(),
        expected_keys
    );

    let duplicate = [rows[0].clone(), rows[0].clone()];
    let duplicate_keys = duplicate
        .iter()
        .map(|vector| {
            (
                vector["receipt"]["declaration"]["declared_case_id"]
                    .as_str()
                    .unwrap(),
                vector["receipt"]["declaration"]["platform"]
                    .as_str()
                    .unwrap(),
            )
        })
        .collect::<BTreeSet<_>>();
    assert_ne!(duplicate_keys.len(), duplicate.len());
    let mut duplicated_collection = complete_collection.clone();
    duplicated_collection.push(complete_collection[0].clone());
    assert_ne!(
        duplicated_collection.len(),
        duplicated_collection
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .len()
    );

    let mutations = fixture("schema-mutations.json");
    for mutation in mutations["mutations"].as_array().unwrap() {
        let target = mutation["fixture"].as_str().unwrap();
        let mut value = if target == "preflight-vectors.json#row0" {
            rows[0]["receipt"].clone()
        } else {
            fixture(target)
        };
        mutate(
            &mut value,
            mutation["operation"].as_str().unwrap(),
            mutation["pointer"].as_str().unwrap(),
            mutation.get("value").cloned(),
        );
        let valid = if target == "preflight-vectors.json#row0" {
            validate_collection_row(&value)
        } else {
            validate_command_result(&value)
        };
        assert!(!valid, "mutation {} was accepted", mutation["id"]);
    }

    let dispositions = fixture("repository-disposition-vectors.json");
    for vector in dispositions["vectors"].as_array().unwrap() {
        assert_eq!(disposition(&vector["inputs"]), vector["expected"]);
    }
}
