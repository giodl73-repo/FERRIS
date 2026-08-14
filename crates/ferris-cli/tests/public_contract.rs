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
const REPOSITORY_SELECTIONS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/simulations/profile-diff-held-out/repository-selections"
);
const PUBLIC_SAFE_RESULT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/simulations/profile-diff-held-out/PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.json"
);

fn fixture(name: &str) -> Value {
    serde_json::from_slice(&fixture_bytes(name)).expect("parse fixture")
}

fn fixture_bytes(name: &str) -> Vec<u8> {
    fs::read(Path::new(FIXTURES).join(name)).expect("read fixture")
}

fn repository_selection(name: &str) -> Value {
    serde_json::from_slice(
        &fs::read(Path::new(REPOSITORY_SELECTIONS).join(name)).expect("read repository selection"),
    )
    .expect("parse repository selection")
}

fn fixture_target(target: &str) -> Value {
    let (name, pointer) = target.split_once('#').unwrap_or((target, ""));
    let value = fixture(name);
    if pointer.is_empty() {
        value
    } else {
        value.pointer(pointer).expect("fixture pointer").clone()
    }
}

fn vector_bytes(vector: &Value, stream: &str) -> Vec<u8> {
    let source = &vector[format!("{stream}_bytes")];
    match source["encoding"].as_str().expect("stream encoding") {
        "utf8" => source["value"].as_str().unwrap().as_bytes().to_vec(),
        "repeat-byte" => {
            vec![source["byte"].as_u64().unwrap() as u8; source["count"].as_u64().unwrap() as usize]
        }
        encoding => panic!("unknown stream encoding {encoding}"),
    }
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
        contract_revision: 3,
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

fn assert_one_lf(path: &Path) {
    let bytes = fs::read(path).expect("read normative fixture");
    assert!(!bytes.contains(&b'\r'), "{} contains CR", path.display());
    assert!(
        bytes.ends_with(b"\n"),
        "{} is missing terminal LF",
        path.display()
    );
    assert!(
        !bytes.ends_with(b"\n\n"),
        "{} has an extra terminal LF",
        path.display()
    );
}

#[test]
fn public_normative_artifacts_use_exact_lf_bytes() {
    for directory in [FIXTURES, SCHEMAS, REPOSITORY_SELECTIONS] {
        for entry in fs::read_dir(directory).expect("public contract directory") {
            let path = entry.expect("public contract entry").path();
            if path.extension().and_then(|value| value.to_str()) == Some("json") {
                assert_one_lf(&path);
            }
        }
    }
    for name in ["human-result-success.txt", "human-result-difference.txt"] {
        assert_one_lf(&Path::new(FIXTURES).join(name));
    }
    assert_one_lf(Path::new(PUBLIC_SAFE_RESULT));
}

#[test]
fn public_safe_stage_b_c_result_is_exact_and_closed() {
    let result: Value =
        serde_json::from_slice(&fs::read(PUBLIC_SAFE_RESULT).expect("read public-safe result"))
            .expect("parse public-safe result");

    assert!(allowed_keys(
        &result,
        &[
            "schema",
            "contract_revision",
            "program",
            "fixture_id",
            "cutoff",
            "disposition",
            "score_validity",
            "custody_disposition",
            "failure_categories",
            "attempts",
            "processes",
            "collection_integrity",
            "result_class_counts",
            "repository_workflows",
            "artifacts",
            "quarantine",
            "disclosure",
        ],
    ));
    assert_eq!(
        result["schema"],
        "ferris.profile-diff-public-safe-result/v1"
    );
    assert_eq!(result["contract_revision"], 3);
    assert_eq!(result["program"], "pulse-17-stage-b-c");
    assert_eq!(result["fixture_id"], "P17-R3-D6B553CBC3B1240B673B8190");
    assert_eq!(result["cutoff"], "8cbb5356fd7b3acca435bc9fad4e97dabab66bb5");
    assert_eq!(result["disposition"], "fail");
    assert_eq!(result["score_validity"], "valid-implementation-fail");
    assert_eq!(result["custody_disposition"], "valid");
    assert_eq!(
        result["failure_categories"],
        serde_json::json!(["process-exit-agreement"])
    );

    let attempts = &result["attempts"];
    assert!(allowed_keys(
        attempts,
        &[
            "first_score_attempt",
            "scorer_attempt",
            "retries",
            "rescores",
        ],
    ));
    assert_eq!(attempts["first_score_attempt"], 1);
    assert_eq!(attempts["scorer_attempt"], 1);
    assert_eq!(attempts["retries"], 0);
    assert_eq!(attempts["rescores"], 0);

    let processes = &result["processes"];
    assert!(allowed_keys(
        processes,
        &["expected", "collected", "platforms"],
    ));
    assert_eq!(processes["expected"], 112);
    assert_eq!(processes["collected"], 112);
    assert!(allowed_keys(
        &processes["platforms"],
        &["windows", "ubuntu_24_04"],
    ));
    assert_eq!(processes["platforms"]["windows"], 56);
    assert_eq!(processes["platforms"]["ubuntu_24_04"], 56);
    let platform_total: u64 = processes["platforms"]
        .as_object()
        .unwrap()
        .values()
        .map(|count| count.as_u64().unwrap())
        .sum();
    assert_eq!(platform_total, 112);

    let integrity = &result["collection_integrity"];
    assert!(allowed_keys(
        integrity,
        &[
            "missing",
            "duplicate",
            "retried",
            "extra",
            "launch_failures",
            "abnormal_terminations",
            "stream_failures",
            "privacy_hits",
            "aggregate_digest",
            "seal",
        ],
    ));
    for field in [
        "missing",
        "duplicate",
        "retried",
        "extra",
        "launch_failures",
        "abnormal_terminations",
        "stream_failures",
        "privacy_hits",
    ] {
        assert_eq!(integrity[field], 0);
    }
    assert_eq!(
        integrity["aggregate_digest"],
        "sha256:8d1b157ef79bf22741d53e6b4ff68302f88cc04ac317e8b3364b9a83832ef9ba"
    );
    assert_eq!(
        integrity["seal"],
        "sha256:71916d7d98e0bcbebbe46ceb25dc619c00fcd69f5224ef5ce01c98ab2534e3b1"
    );

    let counts = &result["result_class_counts"];
    assert!(allowed_keys(
        counts,
        &[
            "success",
            "difference",
            "invalid",
            "unsupported",
            "incomplete",
            "blocked",
        ],
    ));
    let expected_counts = [
        ("success", 8),
        ("difference", 58),
        ("invalid", 22),
        ("unsupported", 2),
        ("incomplete", 20),
        ("blocked", 2),
    ];
    for (class, count) in expected_counts {
        assert_eq!(counts[class], count);
    }
    let result_total: u64 = counts
        .as_object()
        .unwrap()
        .values()
        .map(|count| count.as_u64().unwrap())
        .sum();
    assert_eq!(result_total, 112);

    let repositories = &result["repository_workflows"];
    assert!(allowed_keys(
        repositories,
        &[
            "aggregate_disposition",
            "slots",
            "owner_commands",
            "owner_failures",
            "comparison_failures",
            "lifecycle_failures",
            "cleanup_complete",
        ],
    ));
    assert_eq!(repositories["aggregate_disposition"], "pass");
    assert!(allowed_keys(
        &repositories["slots"],
        &["hosted", "cross_target_no_std", "native_bound"],
    ));
    for slot in ["hosted", "cross_target_no_std", "native_bound"] {
        assert_eq!(repositories["slots"][slot], "pass");
    }
    assert_eq!(repositories["owner_commands"], 28);
    assert_eq!(repositories["owner_failures"], 0);
    assert_eq!(repositories["comparison_failures"], 0);
    assert_eq!(repositories["lifecycle_failures"], 0);
    assert_eq!(repositories["cleanup_complete"], true);

    let artifacts = &result["artifacts"];
    assert!(allowed_keys(
        artifacts,
        &["source_digest", "binary_digests", "report_seal"],
    ));
    assert_eq!(
        artifacts["source_digest"],
        "sha256:5bec6598a5274fd27e8c8c4c275a9cd85ef01bb250df810898a2e13962757910"
    );
    assert!(allowed_keys(
        &artifacts["binary_digests"],
        &["windows", "ubuntu_24_04"],
    ));
    assert_eq!(
        artifacts["binary_digests"]["windows"],
        "sha256:aef4bd137d49400649186dfd88d2ae37ea100c55bdb31ce1ed136b17f9c9eec1"
    );
    assert_eq!(
        artifacts["binary_digests"]["ubuntu_24_04"],
        "sha256:1f3c9acb44002e77fa11f79f4205d9dfb167889f481a507bf0e37aa284de90ad"
    );
    assert_eq!(
        artifacts["report_seal"],
        "sha256:33c1fa87344ee2ef6b186fb59457eff10ca7137e2c2d7019174a11bb96fdf4d0"
    );

    let quarantine = &result["quarantine"];
    assert!(allowed_keys(
        quarantine,
        &[
            "sealed",
            "cleanup_complete",
            "reuse_allowed",
            "retry_allowed",
            "rescore_allowed",
        ],
    ));
    assert_eq!(quarantine["sealed"], true);
    assert_eq!(quarantine["cleanup_complete"], true);
    assert_eq!(quarantine["reuse_allowed"], false);
    assert_eq!(quarantine["retry_allowed"], false);
    assert_eq!(quarantine["rescore_allowed"], false);
    assert!(allowed_keys(
        &result["disclosure"],
        &["hidden_material_disclosed"],
    ));
    assert_eq!(result["disclosure"]["hidden_material_disclosed"], false);
}

#[test]
fn public_repository_selection_binding_is_frozen() {
    let binding = repository_selection("binding.json");
    assert_eq!(binding["schema"], "ferris.repository-selection-binding/v1");
    assert_eq!(binding["contract_revision"], 3);
    assert_eq!(
        binding["contract_cutoff"],
        "4371f4f6eb54097bff9badb29278c530d49e2f36"
    );
    assert_eq!(binding["stage_a_validation"]["disposition"], "pass");
    assert_eq!(binding["stage_a_validation"]["assertions"], 789);
    assert_eq!(binding["stage_a_validation"]["public_blockers"], 0);
    assert_eq!(
        binding["stage_a_validation"]["first_score_integrity_preserved"],
        true
    );
    assert_eq!(binding["selection_count"], 3);
    assert_eq!(
        binding["comparison_policy"]["excluded_pointers"],
        serde_json::json!([
            "/revision",
            "/sections/stages/phase",
            "/sections/lifecycle/state"
        ])
    );
    for field in [
        "omissions_required",
        "promotions_required",
        "prohibited_conclusions_required",
        "privacy_canary_hits_required",
    ] {
        assert_eq!(binding["comparison_policy"][field], 0);
    }

    let expected = [
        (
            "hosted",
            "hosted.json",
            "https://github.com/cncf/gitvote",
            "d4bce0e2670cc61ea53f24838366d21eeca0a68a",
        ),
        (
            "cross_target_no_std",
            "cross_target_no_std.json",
            "https://github.com/dalek-cryptography/curve25519-dalek",
            "07bef73ff85998a206cd2cea7f2605c801d0d1c9",
        ),
        (
            "native_bound",
            "native_bound.json",
            "https://github.com/BurntSushi/ripgrep",
            "e89fff89ac9af12e8d4ce9d5fd07beb408ca730f",
        ),
    ];
    let indexes = binding["selections"].as_array().unwrap();
    let evidence_records = binding["evidence_records"].as_array().unwrap();
    assert_eq!(indexes.len(), expected.len());
    assert_eq!(evidence_records.len(), expected.len());

    let mut slots = BTreeSet::new();
    let mut repositories = BTreeSet::new();
    for (slot, record_path, repository_url, commit) in expected {
        let index = indexes
            .iter()
            .find(|entry| entry["slot"] == slot)
            .expect("selection index");
        assert_eq!(index["record_path"], record_path);

        let record = repository_selection(record_path);
        assert!(validate_repository_selection(&record));
        assert_eq!(record["slot"], slot);
        assert_eq!(record["repository_url"], repository_url);
        assert_eq!(record["commit"], commit);
        assert_eq!(record["workflow_id"], binding["workflow_id"]);
        assert_eq!(record["selected_at"], binding["selected_at"]);
        assert_eq!(record["host_target"], "x86_64-pc-windows-msvc");
        assert_eq!(record["execution_policy"]["locale"], "C.UTF-8");
        assert_eq!(
            receipt_identity(
                "ferris.repository-selection/v1",
                &record,
                "receipt_identity"
            ),
            record["receipt_identity"]
        );
        assert_eq!(index["receipt_identity"], record["receipt_identity"]);
        assert_eq!(
            index["record_digest"],
            sha256(&serde_json::to_vec(&record).unwrap())
        );
        assert_eq!(
            index["license_evidence_digest"],
            evidence_digest(
                "ferris.repository-license-evidence/v1",
                &record["license_evidence"]
            )
        );
        assert_eq!(
            index["eligibility_evidence_digest"],
            evidence_digest(
                "ferris.repository-eligibility-evidence/v1",
                &record["eligibility"]
            )
        );
        assert_eq!(
            index["owner_command_templates_digest"],
            evidence_digest(
                "ferris.repository-command-surface/v1",
                &record["owner_command_templates"]
            )
        );
        assert_eq!(
            index["change_policy_digest"],
            evidence_digest(
                "ferris.repository-change-policy/v1",
                &record["change_policy"]
            )
        );

        let evidence = evidence_records
            .iter()
            .find(|entry| entry["slot"] == slot)
            .expect("stage A evidence");
        let mut evidence_input = evidence.clone();
        evidence_input["evidence_digest"] = Value::String(String::new());
        assert_eq!(
            evidence_digest(
                "ferris.repository-selection-stage-a-evidence/v1",
                &evidence_input
            ),
            evidence["evidence_digest"]
        );
        assert_eq!(
            index["stage_a_evidence_digest"],
            evidence["evidence_digest"]
        );
        assert!(
            evidence["public_evidence_urls"]
                .as_array()
                .unwrap()
                .iter()
                .all(|url| url.as_str().is_some_and(|url| url.contains(commit)))
        );
        assert!(
            record["eligibility"]
                .as_object()
                .unwrap()
                .values()
                .all(|assertion| assertion["evidence_digest"] == evidence["evidence_digest"])
        );
        assert!(
            record["license_evidence"]["source_url"]
                .as_str()
                .is_some_and(|url| url.contains(commit))
        );
        assert!(
            record["owner_command_templates"]
                .as_object()
                .unwrap()
                .values()
                .flat_map(|rows| rows.as_array().unwrap())
                .flat_map(|argv| argv.as_array().unwrap())
                .filter_map(Value::as_str)
                .all(
                    |argument| !argument.contains("<CHECKOUT>") && !argument.contains("<RUN_ROOT>")
                )
        );

        assert!(slots.insert(slot));
        assert!(repositories.insert(repository_url));
    }
    assert_eq!(slots.len(), 3);
    assert_eq!(repositories.len(), 3);
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
    assert_eq!(human.stdout, fixture_bytes("human-result-difference.txt"));
    assert_eq!(human.stdout, human_from_record(&envelope).as_bytes());

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
    assert!(success_human.stderr.is_empty());
    assert_eq!(
        success_human.stdout,
        fixture_bytes("human-result-success.txt")
    );
    assert_eq!(
        success_human.stdout,
        human_from_record(&success_envelope).as_bytes()
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
            "termination",
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
        || row["duration_millis"]
            .as_u64()
            .is_none_or(|value| value > 60_000)
        || !validate_collection_stream(&row["stdout"])
        || !validate_collection_stream(&row["stderr"])
    {
        return false;
    }
    if !allowed_keys(&row["launch"], &["status", "error_class", "error_digest"])
        || !allowed_keys(
            &row["termination"],
            &[
                "kind",
                "output_bound_stream",
                "process_result",
                "error_class",
                "error_digest",
            ],
        )
    {
        return false;
    }
    match (
        row["launch"]["status"].as_str(),
        row["termination"]["kind"].as_str(),
    ) {
        (Some("failed"), Some("launch-failed")) => {
            row["launch"]["error_class"].is_string()
                && is_digest(&row["launch"]["error_digest"])
                && row["termination"]["output_bound_stream"].is_null()
                && row["termination"]["process_result"].is_null()
                && row["termination"]["error_class"].is_string()
                && is_digest(&row["termination"]["error_digest"])
                && row["stdout"]["read_status"] == "not-attempted"
                && row["stderr"]["read_status"] == "not-attempted"
                && row["process_exit_code"].is_null()
        }
        (Some("started"), Some("timeout")) => {
            row["launch"]["error_class"].is_null()
                && row["launch"]["error_digest"].is_null()
                && row["termination"]["output_bound_stream"].is_null()
                && row["termination"]["process_result"].is_null()
                && row["termination"]["error_class"].is_string()
                && is_digest(&row["termination"]["error_digest"])
                && row["process_exit_code"].is_null()
        }
        (Some("started"), Some("output-bound")) => {
            let Some(stream_name) = row["termination"]["output_bound_stream"].as_str() else {
                return false;
            };
            let stream = match stream_name {
                "stdout" => &row["stdout"],
                "stderr" => &row["stderr"],
                _ => return false,
            };
            row["launch"]["error_class"].is_null()
                && row["launch"]["error_digest"].is_null()
                && row["termination"]["process_result"].is_null()
                && row["termination"]["error_class"] == "output-bound"
                && is_digest(&row["termination"]["error_digest"])
                && stream["byte_count"] == 8_388_608
                && stream["complete"] == false
                && stream["truncated"] == true
                && stream["read_status"] == "ok"
                && row["process_exit_code"].is_null()
        }
        (Some("started"), Some("process-exit")) => {
            let Some(exit) = row["process_exit_code"].as_u64() else {
                return false;
            };
            row["launch"]["error_class"].is_null()
                && row["launch"]["error_digest"].is_null()
                && row["termination"]["output_bound_stream"].is_null()
                && row["termination"]["error_class"].is_null()
                && row["termination"]["error_digest"].is_null()
                && match row["termination"]["process_result"].as_str() {
                    Some("success") => exit == 0,
                    Some("failure") => (1..=255).contains(&exit),
                    _ => false,
                }
        }
        _ => false,
    }
}

fn is_digest(value: &Value) -> bool {
    value.as_str().is_some_and(|value| {
        value.len() == 71
            && value.starts_with("sha256:")
            && value[7..].bytes().all(|byte| byte.is_ascii_hexdigit())
            && value[7..].bytes().all(|byte| !byte.is_ascii_uppercase())
    })
}

fn validate_collection_stream(stream: &Value) -> bool {
    if !allowed_keys(
        stream,
        &[
            "capture_path",
            "byte_count",
            "digest",
            "complete",
            "truncated",
            "read_status",
            "read_error_class",
            "read_error_digest",
        ],
    ) || stream["capture_path"].as_str().is_none_or(str::is_empty)
        || stream["byte_count"]
            .as_u64()
            .is_none_or(|value| value > 8_388_608)
        || !is_digest(&stream["digest"])
        || !stream["complete"].is_boolean()
        || !stream["truncated"].is_boolean()
    {
        return false;
    }
    match stream["read_status"].as_str() {
        Some("ok") => {
            stream["read_error_class"].is_null()
                && stream["read_error_digest"].is_null()
                && !(stream["complete"] == true && stream["truncated"] == true)
        }
        Some("failed") => {
            stream["complete"] == false
                && stream["truncated"] == false
                && stream["read_error_class"].is_string()
                && is_digest(&stream["read_error_digest"])
        }
        Some("not-attempted") => {
            stream["byte_count"] == 0
                && stream["digest"]
                    == "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                && stream["complete"] == false
                && stream["truncated"] == false
                && stream["read_error_class"].is_null()
                && stream["read_error_digest"].is_null()
        }
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
        match operation {
            "remove" => {
                array.remove(index);
            }
            "add" => array.insert(index, replacement.unwrap()),
            "replace" => array[index] = replacement.unwrap(),
            _ => panic!("unknown mutation"),
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

fn evidence_digest(domain: &str, value: &Value) -> String {
    let mut bytes = format!("{domain}\0").into_bytes();
    bytes.extend(serde_json::to_vec(value).unwrap());
    sha256(&bytes)
}

fn feature_args(value: &Value) -> Option<Vec<String>> {
    match value["feature_mode"].as_str()? {
        "default" => Some(Vec::new()),
        "all-features" => Some(vec!["--all-features".to_owned()]),
        "no-default-features" => Some(vec!["--no-default-features".to_owned()]),
        "selected-no-default-features" => Some(vec![
            "--no-default-features".to_owned(),
            "--features".to_owned(),
            value["features"]
                .as_array()?
                .iter()
                .map(|feature| feature.as_str())
                .collect::<Option<Vec<_>>>()?
                .join(","),
        ]),
        _ => None,
    }
}

fn argv_matches(value: &Value, expected: &[String]) -> bool {
    value.as_array().is_some_and(|arguments| {
        arguments.len() == expected.len()
            && arguments.iter().zip(expected).all(|(actual, expected)| {
                actual
                    .as_str()
                    .is_some_and(|actual| actual.replace('\\', "/") == *expected)
            })
    })
}

fn validate_owner_command_surface(value: &Value) -> bool {
    let Some(manifest_path) = value["manifest_path"].as_str() else {
        return false;
    };
    let Some(feature_args) = feature_args(value) else {
        return false;
    };
    let Some(commands) = value["owner_command_templates"].as_object() else {
        return false;
    };
    let phases = [
        "baseline",
        "changed",
        "full_reference",
        "renewal",
        "rollback",
        "removal",
        "cleanup",
    ];
    let package = value["packages"]
        .as_array()
        .and_then(|packages| packages.first())
        .and_then(Value::as_str);
    let Some(manifest_argument) = commands["baseline"]
        .as_array()
        .and_then(|rows| rows.first())
        .and_then(Value::as_array)
        .and_then(|argv| argv.get(5))
        .and_then(Value::as_str)
        .map(|value| value.replace('\\', "/"))
    else {
        return false;
    };
    if manifest_argument != manifest_path
        && !manifest_argument.ends_with(&format!("/{manifest_path}"))
    {
        return false;
    }
    let Some(baseline_target) = commands["baseline"]
        .as_array()
        .and_then(|rows| rows.first())
        .and_then(Value::as_array)
        .and_then(|argv| argv.last())
        .and_then(Value::as_str)
        .map(|value| value.replace('\\', "/"))
    else {
        return false;
    };
    let baseline_suffix = match value["slot"].as_str() {
        Some("hosted") => "/hosted/baseline/target",
        Some("cross_target_no_std") => "/cross_target_no_std/baseline/host-target",
        Some("native_bound") => "/native_bound/baseline/target",
        _ => return false,
    };
    let Some(run_root) = baseline_target.strip_suffix(baseline_suffix) else {
        return false;
    };
    for phase in phases {
        let Some(rows) = commands[phase].as_array() else {
            return false;
        };
        match value["slot"].as_str() {
            Some("hosted") => {
                let mut expected = [
                    "cargo",
                    "test",
                    "--locked",
                    "--offline",
                    "--manifest-path",
                    &manifest_argument,
                    "--workspace",
                    "--all-targets",
                ]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>();
                expected.extend(feature_args.clone());
                expected.extend([
                    "--target-dir".to_owned(),
                    format!("{run_root}/hosted/{phase}/target"),
                ]);
                if rows.len() != 1 || !argv_matches(&rows[0], &expected) {
                    return false;
                }
            }
            Some("cross_target_no_std") => {
                let (Some(package), Some(target)) = (package, value["cross_target"].as_str())
                else {
                    return false;
                };
                let mut host = [
                    "cargo",
                    "test",
                    "--locked",
                    "--offline",
                    "--manifest-path",
                    &manifest_argument,
                    "-p",
                    package,
                ]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>();
                host.extend(feature_args.clone());
                host.extend([
                    "--target-dir".to_owned(),
                    format!("{run_root}/cross_target_no_std/{phase}/host-target"),
                ]);
                let mut cross = [
                    "cargo",
                    "check",
                    "--locked",
                    "--offline",
                    "--manifest-path",
                    &manifest_argument,
                    "-p",
                    package,
                ]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>();
                cross.extend(feature_args.clone());
                cross.extend([
                    "--target".to_owned(),
                    target.to_owned(),
                    "--target-dir".to_owned(),
                    format!("{run_root}/cross_target_no_std/{phase}/cross-target"),
                ]);
                if rows.len() != 2
                    || !argv_matches(&rows[0], &host)
                    || !argv_matches(&rows[1], &cross)
                {
                    return false;
                }
            }
            Some("native_bound") => {
                let Some(package) = package else {
                    return false;
                };
                let mut expected = [
                    "cargo",
                    "test",
                    "--locked",
                    "--offline",
                    "--manifest-path",
                    &manifest_argument,
                    "-p",
                    package,
                    "--all-targets",
                ]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>();
                expected.extend(feature_args.clone());
                expected.extend([
                    "--target-dir".to_owned(),
                    format!("{run_root}/native_bound/{phase}/target"),
                ]);
                if rows.len() != 1 || !argv_matches(&rows[0], &expected) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

fn validate_workflow_bounds(value: &Value) -> bool {
    let expected = [
        ("repository_count", 3_u64),
        ("phase_count", 7),
        ("sealed_change_count", 3),
        ("source_files_per_patch", 1),
        ("patch_min_bytes", 1),
        ("patch_max_bytes", 16_384),
        ("checkout_regular_files_max", 100_000),
        ("checkout_bytes_max", 2_147_483_648),
        ("owner_command_timeout_millis", 900_000),
        ("stdout_max_bytes", 1_048_576),
        ("stderr_max_bytes", 1_048_576),
        ("environment_allowlist_max_entries", 32),
        ("environment_value_max_bytes", 4_096),
        ("command_attempts", 1),
        ("network_requests_after_materialization", 0),
    ];
    let keys = expected.iter().map(|(name, _)| *name).collect::<Vec<_>>();
    allowed_keys(value, &keys)
        && expected
            .iter()
            .all(|(name, expected)| value[*name] == *expected)
}

fn validate_repository_selection(value: &Value) -> bool {
    const ELIGIBILITY: [&str; 24] = [
        "public_https",
        "committed_lockfile",
        "clean_detached_checkout",
        "baseline_passed",
        "full_reference_passed",
        "offline_owner_commands",
        "no_reusable_secrets",
        "no_account",
        "no_live_service",
        "no_privileged_operation",
        "no_package_installation",
        "no_container_daemon",
        "no_mutable_external_system",
        "within_command_bounds",
        "no_submodules",
        "no_git_lfs",
        "no_generated_credentials",
        "no_network_at_test",
        "ordinary_cargo_preserved",
        "reversible_source_only_change",
        "independent_selection",
        "not_implementation_author_fork",
        "not_previously_used_to_tune_ferris",
        "hidden_change_independent",
    ];
    if !allowed_keys(
        value,
        &[
            "schema",
            "receipt_identity",
            "workflow_id",
            "slot",
            "repository_url",
            "commit",
            "manifest_path",
            "lockfile_path",
            "packages",
            "feature_mode",
            "features",
            "host_target",
            "cross_target",
            "native_prerequisites",
            "license_evidence",
            "license_evidence_digest",
            "eligibility",
            "eligibility_evidence_digest",
            "workflow_bounds",
            "execution_policy",
            "owner_command_templates",
            "change_policy",
            "selected_at",
        ],
    ) || value["schema"] != "ferris.repository-selection/v1"
        || !is_digest(&value["receipt_identity"])
        || !is_digest(&value["license_evidence_digest"])
        || !is_digest(&value["eligibility_evidence_digest"])
        || !validate_workflow_bounds(&value["workflow_bounds"])
        || !allowed_keys(
            &value["license_evidence"],
            &[
                "conclusion",
                "spdx_expression",
                "license_files",
                "source_url",
            ],
        )
        || value["license_evidence"]["conclusion"] != "permits-temporary-local-validation"
        || !(value["license_evidence"]["spdx_expression"].is_string()
            || value["license_evidence"]["spdx_expression"].is_null())
        || !(value["license_evidence"]["source_url"].is_string()
            || value["license_evidence"]["source_url"].is_null())
        || value["license_evidence"]["license_files"]
            .as_array()
            .is_none_or(|files| {
                files.is_empty()
                    || files.iter().any(|file| {
                        !allowed_keys(file, &["path", "digest"])
                            || file["path"].as_str().is_none_or(str::is_empty)
                            || !is_digest(&file["digest"])
                    })
            })
        || !allowed_keys(&value["eligibility"], &ELIGIBILITY)
        || !ELIGIBILITY.iter().all(|name| {
            let assertion = &value["eligibility"][*name];
            allowed_keys(assertion, &["satisfied", "evidence_digest"])
                && assertion["satisfied"] == true
                && is_digest(&assertion["evidence_digest"])
        })
        || evidence_digest(
            "ferris.repository-license-evidence/v1",
            &value["license_evidence"],
        ) != value["license_evidence_digest"]
        || evidence_digest(
            "ferris.repository-eligibility-evidence/v1",
            &value["eligibility"],
        ) != value["eligibility_evidence_digest"]
        || receipt_identity("ferris.repository-selection/v1", value, "receipt_identity")
            != value["receipt_identity"]
        || !allowed_keys(
            &value["execution_policy"],
            &[
                "offline_after_materialization",
                "cargo_net_offline",
                "rustup_auto_install",
                "external_target_directories",
                "unique_target_per_phase_command",
                "shared_compilation_cache",
                "rustc_wrapper",
                "rustc_workspace_wrapper",
                "locale",
                "shell_recorded",
                "argv_reinterpreted_by_shell",
            ],
        )
        || value["execution_policy"]["offline_after_materialization"] != true
        || value["execution_policy"]["cargo_net_offline"] != true
        || value["execution_policy"]["rustup_auto_install"] != false
        || value["execution_policy"]["external_target_directories"] != true
        || value["execution_policy"]["unique_target_per_phase_command"] != true
        || value["execution_policy"]["shared_compilation_cache"] != false
        || !value["execution_policy"]["rustc_wrapper"].is_null()
        || !value["execution_policy"]["rustc_workspace_wrapper"].is_null()
        || value["execution_policy"]["shell_recorded"] != true
        || value["execution_policy"]["argv_reinterpreted_by_shell"] != false
        || !allowed_keys(
            &value["change_policy"],
            &[
                "category",
                "logical_changes",
                "modified_regular_utf8_rust_files",
                "added_paths",
                "deleted_paths",
                "patch_min_bytes",
                "patch_max_bytes",
            ],
        )
        || value["change_policy"]["logical_changes"] != 1
        || value["change_policy"]["modified_regular_utf8_rust_files"] != 1
        || value["change_policy"]["added_paths"] != 0
        || value["change_policy"]["deleted_paths"] != 0
        || value["change_policy"]["patch_min_bytes"] != 1
        || value["change_policy"]["patch_max_bytes"] != 16_384
        || !validate_owner_command_surface(value)
    {
        return false;
    }
    let Some(packages) = value["packages"].as_array() else {
        return false;
    };
    let Some(features) = value["features"].as_array() else {
        return false;
    };
    if packages.is_empty()
        || match value["feature_mode"].as_str() {
            Some("selected-no-default-features") => features.is_empty(),
            Some("default" | "all-features" | "no-default-features") => !features.is_empty(),
            _ => true,
        }
    {
        return false;
    }
    let Some(commands) = value["owner_command_templates"].as_object() else {
        return false;
    };
    let phase_names = [
        "baseline",
        "changed",
        "full_reference",
        "renewal",
        "rollback",
        "removal",
        "cleanup",
    ];
    if commands.len() != phase_names.len()
        || phase_names
            .iter()
            .any(|phase| !commands.contains_key(*phase))
    {
        return false;
    }
    let expected_commands = if value["slot"] == "cross_target_no_std" {
        2
    } else {
        1
    };
    if commands.values().any(|phase| {
        phase
            .as_array()
            .is_none_or(|rows| rows.len() != expected_commands)
    }) {
        return false;
    }
    match value["slot"].as_str() {
        Some("hosted") => {
            value["cross_target"].is_null()
                && value["native_prerequisites"]
                    .as_array()
                    .is_some_and(Vec::is_empty)
                && value["change_policy"]["category"] == "hosted-observable-behavior"
        }
        Some("cross_target_no_std") => {
            packages.len() == 1
                && value["cross_target"].is_string()
                && value["native_prerequisites"]
                    .as_array()
                    .is_some_and(Vec::is_empty)
                && value["change_policy"]["category"] == "target-conditional-behavior"
        }
        Some("native_bound") => {
            packages.len() == 1
                && value["cross_target"].is_null()
                && value["native_prerequisites"]
                    .as_array()
                    .is_some_and(|items| !items.is_empty())
                && value["change_policy"]["category"] == "native-boundary-behavior"
        }
        _ => false,
    }
}

fn validate_owner_stream(value: &Value) -> bool {
    if !allowed_keys(
        value,
        &[
            "capture_path",
            "retained_bytes",
            "observed_bytes",
            "omitted_observed_bytes",
            "unobserved_bytes_unknown",
            "digest",
            "complete",
            "truncated",
            "read_failed",
        ],
    ) || value["capture_path"].as_str().is_none_or(str::is_empty)
        || value["retained_bytes"]
            .as_u64()
            .is_none_or(|bytes| bytes > 1_048_576)
        || value["observed_bytes"].as_u64().is_none()
        || value["omitted_observed_bytes"].as_u64().is_none()
        || !value["unobserved_bytes_unknown"].is_boolean()
        || !is_digest(&value["digest"])
        || !value["complete"].is_boolean()
        || !value["truncated"].is_boolean()
        || !value["read_failed"].is_boolean()
    {
        return false;
    }
    let retained = value["retained_bytes"].as_u64().unwrap();
    let observed = value["observed_bytes"].as_u64().unwrap();
    let omitted = value["omitted_observed_bytes"].as_u64().unwrap();
    if observed < retained || omitted != observed - retained {
        return false;
    }
    if value["complete"] == true {
        observed == retained
            && omitted == 0
            && value["unobserved_bytes_unknown"] == false
            && value["truncated"] == false
            && value["read_failed"] == false
    } else if value["truncated"] == true {
        retained == 1_048_576 && value["read_failed"] == false
    } else if value["read_failed"] == true {
        value["unobserved_bytes_unknown"] == true
    } else {
        true
    }
}

fn validate_owner_command_receipt(value: &Value) -> bool {
    if !allowed_keys(
        value,
        &[
            "schema",
            "receipt_identity",
            "workflow_id",
            "slot",
            "phase",
            "command_index",
            "check_id",
            "command",
            "current_directory",
            "target_directory",
            "environment_allowlist",
            "environment_digest",
            "started_at",
            "completed_at",
            "duration_millis",
            "launch",
            "termination",
            "termination_cleanup_complete",
            "stdout",
            "stderr",
            "process_exit_code",
            "network_requests_observed",
            "source_tree_digest_before",
            "source_tree_digest_after",
            "result",
        ],
    ) || value["schema"] != "ferris.owner-command-receipt/v1"
        || receipt_identity("ferris.owner-command-receipt/v1", value, "receipt_identity")
            != value["receipt_identity"]
        || !allowed_keys(&value["command"], &["argv", "digest"])
        || value["command"]["argv"]
            .as_array()
            .is_none_or(Vec::is_empty)
        || evidence_digest("ferris.command-argv/v1", &value["command"]["argv"])
            != value["command"]["digest"]
        || value["current_directory"]
            .as_str()
            .is_none_or(str::is_empty)
        || value["target_directory"].as_str().is_none_or(str::is_empty)
        || value["environment_allowlist"]
            .as_array()
            .is_none_or(|entries| {
                entries.len() > 32
                    || entries.iter().any(|entry| {
                        !allowed_keys(entry, &["name", "value_digest"])
                            || entry["name"].as_str().is_none_or(str::is_empty)
                            || !is_digest(&entry["value_digest"])
                    })
                    || entries.windows(2).any(|pair| {
                        pair[0]["name"].as_str().unwrap() >= pair[1]["name"].as_str().unwrap()
                    })
            })
        || evidence_digest(
            "ferris.environment-allowlist/v1",
            &value["environment_allowlist"],
        ) != value["environment_digest"]
        || value["duration_millis"]
            .as_u64()
            .is_none_or(|duration| duration > 900_000)
        || !allowed_keys(&value["launch"], &["status", "error_class", "error_digest"])
        || !value["termination_cleanup_complete"].is_boolean()
        || !validate_owner_stream(&value["stdout"])
        || !validate_owner_stream(&value["stderr"])
        || value["network_requests_observed"] != 0
        || !is_digest(&value["source_tree_digest_before"])
        || !is_digest(&value["source_tree_digest_after"])
    {
        return false;
    }
    let launch_valid = match value["launch"]["status"].as_str() {
        Some("started") => {
            value["launch"]["error_class"].is_null() && value["launch"]["error_digest"].is_null()
        }
        Some("failed") => {
            value["launch"]["error_class"].is_string()
                && is_digest(&value["launch"]["error_digest"])
        }
        _ => false,
    };
    if !launch_valid {
        return false;
    }
    match value["termination"].as_str() {
        Some("completed") => {
            let Some(exit) = value["process_exit_code"].as_u64() else {
                return false;
            };
            value["launch"]["status"] == "started"
                && value["stdout"]["complete"] == true
                && value["stderr"]["complete"] == true
                && match value["result"].as_str() {
                    Some("pass") => exit == 0,
                    Some("fail") => (1..=255).contains(&exit),
                    _ => false,
                }
        }
        Some("launch-failed") => {
            value["launch"]["status"] == "failed"
                && value["process_exit_code"].is_null()
                && value["result"] == "fail"
                && value["stdout"]["complete"] == false
                && value["stderr"]["complete"] == false
        }
        Some("timeout") => {
            value["launch"]["status"] == "started"
                && value["process_exit_code"].is_null()
                && value["result"] == "fail"
        }
        Some("output-bound") => {
            value["launch"]["status"] == "started"
                && value["process_exit_code"].is_null()
                && value["result"] == "fail"
                && (value["stdout"]["truncated"] == true || value["stderr"]["truncated"] == true)
        }
        Some("read-failed") => {
            value["launch"]["status"] == "started"
                && value["process_exit_code"].is_null()
                && value["result"] == "fail"
                && (value["stdout"]["read_failed"] == true
                    || value["stderr"]["read_failed"] == true)
        }
        _ => false,
    }
}

fn validate_owner_check_inventory(value: &Value) -> bool {
    if !allowed_keys(
        value,
        &[
            "schema",
            "inventory_identity",
            "workflow_id",
            "slot",
            "selection_receipt_digest",
            "checks",
            "phase_command_digests",
        ],
    ) || value["schema"] != "ferris.owner-check-inventory/v1"
        || receipt_identity(
            "ferris.owner-check-inventory/v1",
            value,
            "inventory_identity",
        ) != value["inventory_identity"]
        || !is_digest(&value["selection_receipt_digest"])
    {
        return false;
    }
    let expected_count = if value["slot"] == "cross_target_no_std" {
        2
    } else {
        1
    };
    if value["checks"].as_array().is_none_or(|checks| {
        checks.len() != expected_count
            || checks.iter().any(|check| {
                !allowed_keys(
                    check,
                    &["check_id", "category", "owner", "required", "target"],
                ) || check["owner"] != "Cargo"
                    || check["required"] != true
                    || !matches!(
                        check["category"].as_str(),
                        Some("host-owner-test" | "cross-target-check" | "native-owner-test")
                    )
                    || !(check["target"].is_string() || check["target"].is_null())
            })
    }) || !allowed_keys(
        &value["phase_command_digests"],
        &[
            "baseline",
            "changed",
            "full_reference",
            "renewal",
            "rollback",
            "removal",
            "cleanup",
        ],
    ) || value["phase_command_digests"]
        .as_object()
        .is_none_or(|phases| {
            phases.values().any(|digests| {
                digests.as_array().is_none_or(|digests| {
                    digests.len() != expected_count
                        || digests.iter().any(|digest| !is_digest(digest))
                })
            })
        })
    {
        return false;
    }
    true
}

fn validate_profile_comparison(value: &Value) -> bool {
    if !allowed_keys(
        value,
        &[
            "schema",
            "comparison_identity",
            "workflow_id",
            "slot",
            "selected_diff_identity",
            "full_reference_diff_identity",
            "selected_changes",
            "full_reference_changes",
            "phase_exclusions",
            "omissions",
            "promotions",
            "prohibited_conclusions",
            "privacy_hits",
            "unexpected_changes",
            "all_predicates_pass",
            "disposition",
        ],
    ) || value["schema"] != "ferris.profile-comparison/v1"
        || receipt_identity("ferris.profile-comparison/v1", value, "comparison_identity")
            != value["comparison_identity"]
        || value["phase_exclusions"]
            != serde_json::json!([
                "/revision",
                "/sections/stages/phase",
                "/sections/lifecycle/state"
            ])
    {
        return false;
    }
    let failures = [
        "omissions",
        "promotions",
        "prohibited_conclusions",
        "privacy_hits",
        "unexpected_changes",
    ];
    match value["disposition"].as_str() {
        Some("pass") => {
            value["all_predicates_pass"] == true
                && failures
                    .iter()
                    .all(|field| value[*field].as_array().is_some_and(Vec::is_empty))
        }
        Some("fail") => {
            value["all_predicates_pass"] == false
                && failures.iter().any(|field| {
                    value[*field]
                        .as_array()
                        .is_some_and(|items| !items.is_empty())
                })
        }
        _ => false,
    }
}

fn validate_immutability_receipt(value: &Value) -> bool {
    if !allowed_keys(
        value,
        &[
            "schema",
            "receipt_identity",
            "workflow_id",
            "slot",
            "phase",
            "before_tree_digest",
            "after_tree_digest",
            "before_path_count",
            "after_path_count",
            "before_total_bytes",
            "after_total_bytes",
            "allowed_changed_paths",
            "added_paths",
            "removed_paths",
            "modified_paths",
            "exact",
            "observed_at",
        ],
    ) || value["schema"] != "ferris.repository-immutability-receipt/v1"
        || receipt_identity(
            "ferris.repository-immutability-receipt/v1",
            value,
            "receipt_identity",
        ) != value["receipt_identity"]
        || !is_digest(&value["before_tree_digest"])
        || !is_digest(&value["after_tree_digest"])
        || value["before_path_count"]
            .as_u64()
            .is_none_or(|count| count > 100_000)
        || value["after_path_count"]
            .as_u64()
            .is_none_or(|count| count > 100_000)
        || value["before_total_bytes"]
            .as_u64()
            .is_none_or(|bytes| bytes > 2_147_483_648)
        || value["after_total_bytes"]
            .as_u64()
            .is_none_or(|bytes| bytes > 2_147_483_648)
        || !value["exact"].is_boolean()
    {
        return false;
    }
    [
        "allowed_changed_paths",
        "added_paths",
        "removed_paths",
        "modified_paths",
    ]
    .iter()
    .all(|field| {
        value[*field].as_array().is_some_and(|paths| {
            paths.iter().all(|path| {
                path.as_str().is_some_and(|path| {
                    !path.is_empty()
                        && !path.starts_with('/')
                        && !path.contains('\\')
                        && !path.split('/').any(|part| part == "..")
                })
            })
        })
    })
}

fn validate_repository_profile(value: &Value) -> bool {
    if !allowed_keys(
        value,
        &["schema", "profile_id", "revision", "consumer", "sections"],
    ) || value["schema"] != "ferris.profile-evidence/v0"
        || value["consumer"] != "independent-custodian"
        || !allowed_keys(
            &value["sections"],
            &[
                "identity",
                "closure",
                "features",
                "toolchain",
                "targets",
                "providers",
                "native",
                "stages",
                "assurance",
                "stewardship",
                "support",
                "lifecycle",
            ],
        )
    {
        return false;
    }
    let sections = &value["sections"];
    for (name, keys) in [
        (
            "identity",
            &[
                "slot",
                "repository_selection_digest",
                "source_tree_digest",
                "sealed_change_digest",
            ][..],
        ),
        (
            "closure",
            &[
                "lockfile_digest",
                "package_selection",
                "owner_check_inventory_digest",
                "owner_command_templates_digest",
            ][..],
        ),
        ("features", &["feature_args"][..]),
        (
            "toolchain",
            &["rustc", "cargo", "host", "environment_receipt_digest"][..],
        ),
        (
            "targets",
            &["host_target", "cross_target", "target_state"][..],
        ),
        ("providers", &["state", "evidence_digest"][..]),
        (
            "native",
            &["state", "prerequisite_digest", "owner_receipt_digests"][..],
        ),
        (
            "stages",
            &[
                "phase",
                "owner_result",
                "owner_receipt_digests",
                "command_timeout_millis",
                "stdout_max_bytes",
                "stderr_max_bytes",
                "attempt",
                "network_requests_observed",
                "output_bound_violations",
            ][..],
        ),
        (
            "assurance",
            &[
                "immutability_receipt_digest",
                "all_required_checks_pass",
                "workflow_bounds",
            ][..],
        ),
        (
            "stewardship",
            &[
                "custody_revision",
                "selection_receipt_digest",
                "license_evidence_digest",
                "eligibility_evidence_digest",
                "change_policy_digest",
            ][..],
        ),
        ("support", &["conclusion"][..]),
        (
            "lifecycle",
            &[
                "state",
                "predecessor_digest",
                "rollback_digest",
                "removal_digest",
            ][..],
        ),
    ] {
        if !allowed_keys(&sections[name], keys) {
            return false;
        }
    }
    let phase = value["revision"].as_str();
    if phase.is_none()
        || phase != sections["stages"]["phase"].as_str()
        || phase != sections["lifecycle"]["state"].as_str()
        || sections["stages"]["command_timeout_millis"] != 900_000
        || sections["stages"]["stdout_max_bytes"] != 1_048_576
        || sections["stages"]["stderr_max_bytes"] != 1_048_576
        || sections["stages"]["attempt"] != 1
        || sections["stages"]["network_requests_observed"] != 0
        || sections["stages"]["output_bound_violations"]
            .as_u64()
            .is_none()
        || !validate_workflow_bounds(&sections["assurance"]["workflow_bounds"])
        || sections["stewardship"]["custody_revision"] != 3
        || !is_digest(&sections["identity"]["repository_selection_digest"])
        || !is_digest(&sections["identity"]["source_tree_digest"])
        || !is_digest(&sections["closure"]["lockfile_digest"])
        || sections["closure"]["package_selection"]
            .as_array()
            .is_none_or(Vec::is_empty)
        || !is_digest(&sections["closure"]["owner_check_inventory_digest"])
        || !is_digest(&sections["closure"]["owner_command_templates_digest"])
        || sections["features"]["feature_args"].as_array().is_none()
        || sections["toolchain"]["rustc"]
            .as_str()
            .is_none_or(str::is_empty)
        || sections["toolchain"]["cargo"]
            .as_str()
            .is_none_or(str::is_empty)
        || sections["toolchain"]["host"]
            .as_str()
            .is_none_or(str::is_empty)
        || !is_digest(&sections["toolchain"]["environment_receipt_digest"])
        || sections["targets"]["target_state"] != "available"
        || sections["providers"]["state"] != "not_assessed"
        || !sections["providers"]["evidence_digest"].is_null()
        || !is_digest(&sections["assurance"]["immutability_receipt_digest"])
        || !sections["assurance"]["all_required_checks_pass"].is_boolean()
        || !is_digest(&sections["stewardship"]["selection_receipt_digest"])
        || !is_digest(&sections["stewardship"]["license_evidence_digest"])
        || !is_digest(&sections["stewardship"]["eligibility_evidence_digest"])
        || !is_digest(&sections["stewardship"]["change_policy_digest"])
        || sections["support"]["conclusion"] != "not_assessed"
    {
        return false;
    }
    let (slot, receipt_count) = match value["profile_id"].as_str() {
        Some("pulse17.public-repository.hosted") => ("hosted", 1),
        Some("pulse17.public-repository.cross_target_no_std") => ("cross_target_no_std", 2),
        Some("pulse17.public-repository.native_bound") => ("native_bound", 1),
        _ => return false,
    };
    if sections["identity"]["slot"] != slot
        || sections["identity"]["repository_selection_digest"]
            != sections["stewardship"]["selection_receipt_digest"]
        || sections["targets"]["host_target"] != sections["toolchain"]["host"]
        || sections["stages"]["owner_receipt_digests"]
            .as_array()
            .is_none_or(|digests| {
                digests.len() != receipt_count || digests.iter().any(|digest| !is_digest(digest))
            })
        || !matches!(
            sections["stages"]["owner_result"].as_str(),
            Some("pass" | "fail" | "unsupported" | "blocked")
        )
        || (sections["stages"]["owner_result"] == "pass"
            && (sections["stages"]["output_bound_violations"] != 0
                || sections["assurance"]["all_required_checks_pass"] != true))
    {
        return false;
    }
    match slot {
        "cross_target_no_std" => {
            if !sections["targets"]["cross_target"].is_string()
                || sections["native"]["state"] != "not_applicable"
                || !sections["native"]["prerequisite_digest"].is_null()
            {
                return false;
            }
        }
        "native_bound" => {
            if !sections["targets"]["cross_target"].is_null()
                || sections["native"]["state"] != "available"
                || !is_digest(&sections["native"]["prerequisite_digest"])
            {
                return false;
            }
        }
        _ => {
            if !sections["targets"]["cross_target"].is_null()
                || sections["native"]["state"] != "not_applicable"
                || !sections["native"]["prerequisite_digest"].is_null()
            {
                return false;
            }
        }
    }
    let lifecycle = &sections["lifecycle"];
    match phase.unwrap() {
        "baseline" => {
            sections["identity"]["sealed_change_digest"].is_null()
                && lifecycle["predecessor_digest"].is_null()
                && lifecycle["rollback_digest"].is_null()
                && lifecycle["removal_digest"].is_null()
        }
        "changed" | "full_reference" | "renewal" => {
            is_digest(&sections["identity"]["sealed_change_digest"])
                && is_digest(&lifecycle["predecessor_digest"])
                && lifecycle["rollback_digest"].is_null()
                && lifecycle["removal_digest"].is_null()
        }
        "rollback" => {
            is_digest(&sections["identity"]["sealed_change_digest"])
                && is_digest(&lifecycle["predecessor_digest"])
                && is_digest(&lifecycle["rollback_digest"])
                && lifecycle["removal_digest"].is_null()
        }
        "removal" | "cleanup" => {
            is_digest(&sections["identity"]["sealed_change_digest"])
                && is_digest(&lifecycle["predecessor_digest"])
                && is_digest(&lifecycle["rollback_digest"])
                && is_digest(&lifecycle["removal_digest"])
        }
        _ => false,
    }
}

fn validate_lifecycle_receipt(value: &Value) -> bool {
    if !allowed_keys(
        value,
        &[
            "schema",
            "receipt_identity",
            "workflow_id",
            "slot",
            "selection_receipt_digest",
            "sealed_change_digest",
            "baseline_tree_digest",
            "changed_tree_digest",
            "renewed_profile_digest",
            "rollback_tree_digest",
            "removed_tree_digest",
            "cleanup_tree_digest",
            "phase_owner_receipts",
            "exact_rollback",
            "rollback_owner_commands_passed",
            "complete_removal",
            "removal_owner_commands_passed",
            "external_cleanup_complete",
            "final_checkout_clean",
            "all_predicates_pass",
            "disposition",
        ],
    ) || value["schema"] != "ferris.repository-lifecycle-receipt/v1"
        || receipt_identity(
            "ferris.repository-lifecycle-receipt/v1",
            value,
            "receipt_identity",
        ) != value["receipt_identity"]
    {
        return false;
    }
    match value["disposition"].as_str() {
        Some("pass") => [
            "exact_rollback",
            "rollback_owner_commands_passed",
            "complete_removal",
            "removal_owner_commands_passed",
            "external_cleanup_complete",
            "final_checkout_clean",
            "all_predicates_pass",
        ]
        .iter()
        .all(|name| value[*name] == true),
        Some("fail") => value["all_predicates_pass"] == false,
        Some("invalid" | "unsupported" | "blocked") => true,
        _ => false,
    }
}

fn disposition(inputs: &Value) -> &'static str {
    if !inputs["qualification_valid"].as_bool().unwrap()
        || !inputs["selection_count_valid"].as_bool().unwrap()
        || !inputs["distinct_slots_and_repositories"].as_bool().unwrap()
        || !inputs["check_inventory_count_valid"].as_bool().unwrap()
        || !inputs["phase_owner_row_counts_valid"].as_bool().unwrap()
        || !inputs["owner_rows_complete"].as_bool().unwrap()
        || !inputs["owner_rows_unique"].as_bool().unwrap()
        || !inputs["owner_rows_attempt_one"].as_bool().unwrap()
        || !inputs["owner_rows_no_extra"].as_bool().unwrap()
        || !inputs["sealed_change_count_valid"].as_bool().unwrap()
        || !inputs["sealed_change_file_count_valid"].as_bool().unwrap()
        || !inputs["sealed_change_patch_bounds_valid"]
            .as_bool()
            .unwrap()
        || !inputs["initial_checkout_clean"].as_bool().unwrap()
        || !inputs["cache_isolated"].as_bool().unwrap()
        || !inputs["rollback_application_valid"].as_bool().unwrap()
        || !inputs["removal_receipt_valid"].as_bool().unwrap()
        || !inputs["cleanup_receipt_valid"].as_bool().unwrap()
        || !inputs["nonzero_exit_retained"].as_bool().unwrap()
        || !inputs["later_rows_collected"].as_bool().unwrap()
        || (!inputs["candidate_executed"].as_bool().unwrap()
            && inputs["environment_supported"].as_bool().unwrap()
            && inputs["external_prerequisite_ready"].as_bool().unwrap())
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
        "source_mutations_outside_sealed_file",
        "output_bound_failures",
        "network_attempts",
        "owner_nonzero_exits",
        "owner_timeout_failures",
    ]
    .iter()
    .any(|name| inputs[*name].as_u64().unwrap() != 0)
        || !inputs["rollback_exact"].as_bool().unwrap()
        || !inputs["rollback_owner_commands_passed"].as_bool().unwrap()
        || !inputs["removal_complete"].as_bool().unwrap()
        || !inputs["removal_owner_commands_passed"].as_bool().unwrap()
        || !inputs["external_cleanup_complete"].as_bool().unwrap()
        || !inputs["final_checkout_clean"].as_bool().unwrap()
        || !inputs["all_predicates_pass"].as_bool().unwrap()
    {
        "fail"
    } else {
        "pass"
    }
}

#[test]
fn public_preflight_vectors_enforce_cardinality_schemas_and_dispositions() {
    let mut schema_count = 0;
    for entry in fs::read_dir(SCHEMAS).expect("schema directory") {
        let path = entry.expect("schema entry").path();
        if path.extension().and_then(|value| value.to_str()) == Some("json") {
            schema_count += 1;
            let schema: Value = serde_json::from_slice(&fs::read(&path).expect("read schema"))
                .expect("schema JSON");
            assert_eq!(
                schema["$schema"],
                "https://json-schema.org/draft/2020-12/schema"
            );
            if path.file_name().and_then(|value| value.to_str())
                != Some("ferris.profile-evidence.v0.schema.json")
            {
                assert!(strict_object_schemas(&schema), "{}", path.display());
            }
        }
    }
    assert_eq!(schema_count, 19);

    let preflight = fixture("preflight-vectors.json");
    let rows = preflight["rows"].as_array().expect("preflight rows");
    assert_eq!(rows.len() as u64, preflight["archetype_row_count"]);
    let mut keys = BTreeSet::new();
    let mut stream_branches = BTreeSet::new();
    let mut outcome_branches = BTreeSet::new();
    let mut saw_nonzero = false;
    for vector in rows {
        let row = &vector["receipt"];
        assert!(validate_collection_row(row));
        assert_eq!(
            receipt_identity("ferris.profile-diff-collection-row/v1", row, "row_identity"),
            row["row_identity"]
        );
        let stdout = vector_bytes(vector, "stdout");
        let stderr = vector_bytes(vector, "stderr");
        assert_eq!(sha256(&stdout), row["stdout"]["digest"]);
        assert_eq!(sha256(&stderr), row["stderr"]["digest"]);
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
        outcome_branches.insert(vector["id"].as_str().unwrap().to_owned());
        saw_nonzero |= row["process_exit_code"]
            .as_u64()
            .is_some_and(|exit| exit != 0);
    }
    assert_eq!(keys.len(), rows.len());
    assert_eq!(stream_branches.len(), 4);
    assert!(saw_nonzero);
    assert_eq!(
        outcome_branches,
        [
            "launch-failed",
            "process-exit-both-streams",
            "process-exit-empty-streams",
            "process-exit-failure",
            "process-exit-success",
            "stderr-output-bound",
            "stderr-read-failed",
            "stdout-output-bound",
            "stdout-read-failed",
            "timeout",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );

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
                        1_u64,
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
    extra_collection.push(("case-057".to_owned(), "windows-x86_64".to_owned(), 1));
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
    let mut retried_collection = complete_collection.clone();
    let mut retry = complete_collection[0].clone();
    retry.2 = 2;
    retried_collection.push(retry);
    assert!(
        retried_collection
            .iter()
            .any(|(_, _, attempt)| *attempt != 1)
    );

    let evidence = fixture("repository-evidence-vectors.json");
    let selections = evidence["selections"].as_array().unwrap();
    let owner_receipts = evidence["owner_command_receipts"].as_array().unwrap();
    let inventories = evidence["owner_check_inventories"].as_array().unwrap();
    let profiles = evidence["profiles"].as_array().unwrap();
    let comparisons = evidence["comparisons"].as_array().unwrap();
    let lifecycle_receipts = evidence["lifecycle_receipts"].as_array().unwrap();
    let immutability_receipts = evidence["immutability_receipts"].as_array().unwrap();
    assert_eq!(selections.len() as u64, evidence["selection_count"]);
    assert_eq!(
        owner_receipts.len() as u64,
        evidence["owner_command_receipt_count"]
    );
    assert_eq!(
        inventories.len() as u64,
        evidence["owner_check_inventory_count"]
    );
    assert_eq!(profiles.len() as u64, evidence["profile_count"]);
    assert_eq!(comparisons.len() as u64, evidence["comparison_count"]);
    assert_eq!(
        lifecycle_receipts.len() as u64,
        evidence["lifecycle_receipt_count"]
    );
    assert_eq!(
        immutability_receipts.len() as u64,
        evidence["immutability_receipt_count"]
    );
    assert!(selections.iter().all(validate_repository_selection));
    assert!(owner_receipts.iter().all(validate_owner_command_receipt));
    assert!(inventories.iter().all(validate_owner_check_inventory));
    assert!(profiles.iter().all(validate_repository_profile));
    assert!(comparisons.iter().all(validate_profile_comparison));
    assert!(lifecycle_receipts.iter().all(validate_lifecycle_receipt));
    assert!(
        immutability_receipts
            .iter()
            .all(validate_immutability_receipt)
    );

    for selection in selections {
        let slot = selection["slot"].as_str().unwrap();
        let inventory = inventories
            .iter()
            .find(|inventory| inventory["slot"] == slot)
            .unwrap();
        assert_eq!(
            inventory["selection_receipt_digest"],
            selection["receipt_identity"]
        );
        for (phase, commands) in selection["owner_command_templates"].as_object().unwrap() {
            let expected = commands
                .as_array()
                .unwrap()
                .iter()
                .map(|command| evidence_digest("ferris.command-argv/v1", command))
                .collect::<Vec<_>>();
            assert_eq!(
                inventory["phase_command_digests"][phase],
                serde_json::to_value(expected).unwrap()
            );
        }
    }
    for profile in profiles {
        let slot = profile["sections"]["identity"]["slot"].as_str().unwrap();
        let selection = selections
            .iter()
            .find(|selection| selection["slot"] == slot)
            .unwrap();
        let inventory = inventories
            .iter()
            .find(|inventory| inventory["slot"] == slot)
            .unwrap();
        assert_eq!(
            profile["sections"]["identity"]["repository_selection_digest"],
            selection["receipt_identity"]
        );
        assert_eq!(
            profile["sections"]["stewardship"]["license_evidence_digest"],
            selection["license_evidence_digest"]
        );
        assert_eq!(
            profile["sections"]["stewardship"]["eligibility_evidence_digest"],
            selection["eligibility_evidence_digest"]
        );
        assert_eq!(
            profile["sections"]["assurance"]["workflow_bounds"],
            selection["workflow_bounds"]
        );
        assert_eq!(
            profile["sections"]["closure"]["owner_check_inventory_digest"],
            inventory["inventory_identity"]
        );
        assert_eq!(
            profile["sections"]["closure"]["owner_command_templates_digest"],
            evidence_digest(
                "ferris.repository-command-surface/v1",
                &selection["owner_command_templates"],
            )
        );
        assert_eq!(
            profile["sections"]["stewardship"]["change_policy_digest"],
            evidence_digest(
                "ferris.repository-change-policy/v1",
                &selection["change_policy"],
            )
        );
    }
    let positive_schema_instance_count = 3
        + rows.len()
        + environments.len()
        + selections.len()
        + owner_receipts.len()
        + inventories.len()
        + profiles.len()
        + comparisons.len()
        + lifecycle_receipts.len()
        + immutability_receipts.len();
    assert_eq!(positive_schema_instance_count, 41);

    let mutations = fixture("schema-mutations.json");
    assert_eq!(mutations["mutations"].as_array().unwrap().len(), 38);
    for mutation in mutations["mutations"].as_array().unwrap() {
        let target = mutation["fixture"].as_str().unwrap();
        let mut value = fixture_target(target);
        mutate(
            &mut value,
            mutation["operation"].as_str().unwrap(),
            mutation["pointer"].as_str().unwrap(),
            mutation.get("value").cloned(),
        );
        let valid = if target.starts_with("preflight-vectors.json#") {
            validate_collection_row(&value)
        } else if target.contains("#/selections/") {
            validate_repository_selection(&value)
        } else if target.contains("#/owner_command_receipts/") {
            validate_owner_command_receipt(&value)
        } else if target.contains("#/owner_check_inventories/") {
            validate_owner_check_inventory(&value)
        } else if target.contains("#/profiles/") {
            validate_repository_profile(&value)
        } else if target.contains("#/comparisons/") {
            validate_profile_comparison(&value)
        } else if target.contains("#/lifecycle_receipts/") {
            validate_lifecycle_receipt(&value)
        } else if target.contains("#/immutability_receipts/") {
            validate_immutability_receipt(&value)
        } else {
            validate_command_result(&value)
        };
        assert!(!valid, "mutation {} was accepted", mutation["id"]);
    }

    let dispositions = fixture("repository-disposition-vectors.json");
    let expected_branches = [
        "pass",
        "invalid-qualification",
        "invalid-selection-count",
        "invalid-distinct-slots-repositories",
        "invalid-check-inventory-count",
        "invalid-phase-owner-row-count",
        "invalid-owner-row-missing",
        "invalid-owner-row-duplicate",
        "invalid-owner-row-retry",
        "invalid-owner-row-extra",
        "invalid-sealed-change-count",
        "invalid-sealed-change-file-count",
        "invalid-sealed-change-patch-bound",
        "invalid-initial-checkout-dirty",
        "invalid-cache-sharing",
        "invalid-rollback-application",
        "invalid-removal-receipt",
        "invalid-cleanup-receipt",
        "invalid-nonzero-exit-not-retained",
        "invalid-collection-aborted-after-nonzero",
        "invalid-candidate-not-executed",
        "unsupported-environment",
        "blocked-external-prerequisite",
        "fail-owner-nonzero-exit",
        "fail-owner-timeout",
        "fail-omission",
        "fail-promotion",
        "fail-privacy-leakage",
        "fail-prohibited-conclusion",
        "fail-unexpected-changed-path",
        "fail-source-mutation-outside-sealed-file",
        "fail-output-bound",
        "fail-network-attempt",
        "fail-rollback-mismatch",
        "fail-rollback-owner-command",
        "fail-removal-residue",
        "fail-removal-owner-command",
        "fail-external-cleanup",
        "fail-final-checkout-dirty",
        "fail-all-predicates",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let workflow = fs::read_to_string(
        Path::new(SCHEMAS)
            .parent()
            .unwrap()
            .join("THREE_REPOSITORY_WORKFLOW.md"),
    )
    .unwrap();
    let workflow = workflow.replace("\r\n", "\n");
    let documented_branches = workflow
        .split_once(
            "The public qualification vectors MUST contain exactly this branch-name set:\n\n```text\n",
        )
        .unwrap()
        .1
        .split_once("\n```")
        .unwrap()
        .0
        .lines()
        .collect::<BTreeSet<_>>();
    let declared_branches = dispositions["required_branch_names"]
        .as_array()
        .unwrap()
        .iter()
        .map(|branch| branch.as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let vector_branches = dispositions["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|vector| vector["branch"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(documented_branches, expected_branches);
    assert_eq!(declared_branches, expected_branches);
    assert_eq!(vector_branches, expected_branches);
    for vector in dispositions["vectors"].as_array().unwrap() {
        assert_eq!(disposition(&vector["inputs"]), vector["expected"]);
    }
}
