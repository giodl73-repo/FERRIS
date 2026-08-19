use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

fn cargo_ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_cargo-ferris"))
}

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(path)
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root")
}

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "ferris-federated-plan-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create test directory");
        Self(path)
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create fixture destination");
    for entry in fs::read_dir(source).expect("read fixture directory") {
        let entry = entry.expect("fixture entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type().expect("fixture file type").is_dir() {
            copy_tree(&source_path, &destination_path);
        } else {
            fs::copy(source_path, destination_path).expect("copy fixture file");
        }
    }
}

fn run_json(command: &mut Command, request: &Path) -> Output {
    command
        .arg("federated-plan")
        .arg("--request")
        .arg(request)
        .args(["--format", "json"])
        .output()
        .expect("run federated-plan")
}

fn write_request(path: &Path, value: &Value) {
    fs::write(
        path,
        serde_json::to_vec_pretty(value).expect("serialize request"),
    )
    .expect("write request");
}

fn base_request() -> Value {
    json!({
        "schema": "ferris.federated-plan-request/v0",
        "application_id": "ferris.test/federated",
        "revision": "r1",
        "owner": "ferris.test/owner",
        "workspaces": [
            {
                "workspace_id": "ferris.test/beta",
                "manifest_path": "workspace-beta/Cargo.toml"
            },
            {
                "workspace_id": "ferris.test/alpha",
                "manifest_path": "workspace-alpha/Cargo.toml"
            }
        ]
    })
}

fn assert_typed_error(output: Output, code: &str) -> Value {
    assert_typed_error_with_exit(output, code, 2)
}

fn assert_typed_error_with_exit(output: Output, code: &str, exit_code: i32) -> Value {
    assert_eq!(output.status.code(), Some(exit_code), "{code} exit");
    assert!(output.stdout.is_empty(), "{code} stdout");
    let value: Value = serde_json::from_slice(&output.stderr).expect("typed error JSON");
    assert_eq!(value["schema"], "ferris.command-result/v2");
    assert_eq!(value["semantic_command_id"], "federated-plan");
    let result_class = match exit_code {
        2 => "invalid",
        4 => "unsupported",
        other => panic!("unsupported test exit code {other}"),
    };
    assert_eq!(value["result_class"], result_class);
    assert_eq!(value["diagnostics"][0]["code"], code);
    value
}

fn assert_diagnostic_path_free(value: &Value, paths: &[&Path]) {
    let serialized = serde_json::to_string(value).expect("diagnostic JSON");
    let root = repository_root();
    assert!(
        !serialized.contains(&root.to_string_lossy().into_owned()),
        "diagnostic leaked repository root"
    );
    for path in paths {
        assert!(
            !serialized.contains(&path.to_string_lossy().into_owned()),
            "diagnostic leaked a filesystem path"
        );
    }
}

#[test]
fn federated_plan_json_and_human_outputs_are_bounded_and_sorted() {
    let request = fixture("federated-plan/request.json");
    let json_output = run_json(&mut ferris(), &request);
    assert!(json_output.status.success());
    assert!(json_output.stderr.is_empty());
    let value: Value = serde_json::from_slice(&json_output.stdout).expect("federated plan JSON");

    assert_eq!(value["schema"], "ferris.command-result/v2");
    assert_eq!(value["semantic_command_id"], "federated-plan");
    assert_eq!(value["record"]["schema"], "ferris.federated-plan/v0");
    assert_eq!(value["record"]["application_id"], "ferris.test/federated");
    assert_eq!(value["record"]["revision"], "r1");
    assert_eq!(value["record"]["owner"], "ferris.test/owner");
    assert_eq!(value["record"]["executable"], false);
    assert!(
        value["record"]["limitations"]
            .as_array()
            .expect("limitations")
            .iter()
            .any(|limitation| limitation
                .as_str()
                .is_some_and(|text| text.contains("30-second timeout")
                    && text.contains("4194304-byte limit per stream")))
    );
    assert_eq!(
        value["record"]["workspaces"][0]["workspace_id"],
        "ferris.test/alpha"
    );
    assert_eq!(
        value["record"]["workspaces"][1]["workspace_id"],
        "ferris.test/beta"
    );
    for workspace in value["record"]["workspaces"]
        .as_array()
        .expect("workspace plans")
    {
        assert_eq!(workspace["plan"]["schema"], "ferris.blueprint-plan/v0");
        assert_eq!(workspace["plan"]["executable"], false);
        assert_eq!(workspace["plan"]["workspace_root"], ".");
        assert_eq!(workspace["plan"]["selected_manifest"], "Cargo.toml");
        assert_eq!(workspace["plan"]["packages"].as_array().unwrap().len(), 1);
    }
    let serialized = String::from_utf8(json_output.stdout).expect("JSON text");
    assert!(!serialized.contains(&request.to_string_lossy().into_owned()));
    assert!(!serialized.contains(&repository_root().to_string_lossy().into_owned()));

    let human_output = ferris()
        .arg("federated-plan")
        .arg("--request")
        .arg(&request)
        .output()
        .expect("run human federated-plan");
    assert!(human_output.status.success());
    assert!(human_output.stderr.is_empty());
    let human = String::from_utf8(human_output.stdout).expect("human output");
    assert!(human.contains("Application ID: ferris.test/federated"));
    assert!(human.contains("Revision: r1"));
    assert!(human.contains("Owner: ferris.test/owner"));
    assert!(human.contains("Executable: no"));
    assert!(human.contains("ferris.test/alpha"));
    assert!(human.contains("ferris.test/beta"));
    assert!(human.contains("Plan ID: plan:"));
    assert!(human.contains("Package count: 1"));
}

#[test]
fn federated_plan_matches_all_shared_cli_invocations() {
    let request = fixture("federated-plan/request.json");
    let ferris_output = run_json(&mut ferris(), &request);
    let direct_output = run_json(&mut cargo_ferris(), &request);
    let cargo_style_output = cargo_ferris()
        .arg("ferris")
        .arg("federated-plan")
        .arg("--request")
        .arg(&request)
        .args(["--format", "json"])
        .output()
        .expect("run cargo-style federated-plan");
    for output in [&ferris_output, &direct_output, &cargo_style_output] {
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
    }
    let expected: Value = serde_json::from_slice(&ferris_output.stdout).expect("ferris JSON");
    assert_eq!(
        serde_json::from_slice::<Value>(&direct_output.stdout).expect("direct JSON"),
        expected
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&cargo_style_output.stdout).expect("cargo-style JSON"),
        expected
    );

    let missing_request = cargo_ferris()
        .current_dir(fixture("simple-workspace/alpha"))
        .args(["ferris", "federated-plan", "--format", "json"])
        .output()
        .expect("run missing-request federated-plan");
    assert_typed_error(missing_request, "FERRIS-CLI-INVALID");
}

#[test]
fn federated_plan_output_identities_are_stable_after_fixture_relocation() {
    let first = TestDirectory::new("relocation-a");
    let second = TestDirectory::new("relocation-b");
    copy_tree(&fixture("federated-plan"), &first.path("federated-plan"));
    copy_tree(&fixture("federated-plan"), &second.path("federated-plan"));
    let first_request = first.path("federated-plan/request.json");
    let second_request = second.path("federated-plan/request.json");
    let first_output = run_json(&mut ferris(), &first_request);
    let second_output = run_json(&mut ferris(), &second_request);
    assert!(first_output.status.success());
    assert!(second_output.status.success());
    let first_value: Value = serde_json::from_slice(&first_output.stdout).expect("first JSON");
    let second_value: Value = serde_json::from_slice(&second_output.stdout).expect("second JSON");

    assert_eq!(
        first_value["selection_identity"],
        second_value["selection_identity"]
    );
    assert_eq!(
        first_value["invocation_identity"],
        second_value["invocation_identity"]
    );
    assert_eq!(
        first_value["record"]["federated_plan_id"],
        second_value["record"]["federated_plan_id"]
    );
    for index in 0..2 {
        assert_eq!(
            first_value["record"]["workspaces"][index]["plan"]["plan_id"],
            second_value["record"]["workspaces"][index]["plan"]["plan_id"]
        );
    }
    let first_text = String::from_utf8(first_output.stdout).expect("first text");
    let second_text = String::from_utf8(second_output.stdout).expect("second text");
    assert!(!first_text.contains(&first.0.to_string_lossy().into_owned()));
    assert!(!second_text.contains(&second.0.to_string_lossy().into_owned()));
}

#[test]
fn federated_plan_rejects_strict_shape_bounds_and_duplicates() {
    let directory = TestDirectory::new("invalid-requests");
    copy_tree(
        &fixture("federated-plan/workspace-alpha"),
        &directory.path("workspace-alpha"),
    );
    copy_tree(
        &fixture("federated-plan/workspace-beta"),
        &directory.path("workspace-beta"),
    );
    let request_path = directory.path("request.json");

    let mut cases = Vec::new();
    let mut unknown_root = base_request();
    unknown_root["unexpected"] = json!(true);
    cases.push((
        unknown_root,
        "FERRIS-FEDERATED-PLAN-REQUEST-SHAPE-INVALID",
        2,
    ));
    let mut unknown_workspace = base_request();
    unknown_workspace["workspaces"][0]["unexpected"] = json!(true);
    cases.push((
        unknown_workspace,
        "FERRIS-FEDERATED-PLAN-REQUEST-SHAPE-INVALID",
        2,
    ));
    let mut unsupported_schema = base_request();
    unsupported_schema["schema"] = json!("ferris.federated-plan-request/v99");
    cases.push((
        unsupported_schema,
        "FERRIS-FEDERATED-PLAN-REQUEST-SCHEMA-UNSUPPORTED",
        4,
    ));
    let mut invalid_application_id = base_request();
    invalid_application_id["application_id"] = json!("invalid application");
    cases.push((invalid_application_id, "FERRIS-APPLICATION-ID-INVALID", 2));
    let mut invalid_workspace_id = base_request();
    invalid_workspace_id["workspaces"][0]["workspace_id"] = json!("invalid workspace");
    cases.push((invalid_workspace_id, "FERRIS-WORKSPACE-ID-INVALID", 2));
    let mut one_workspace = base_request();
    one_workspace["workspaces"]
        .as_array_mut()
        .expect("workspaces")
        .truncate(1);
    cases.push((
        one_workspace,
        "FERRIS-FEDERATED-PLAN-WORKSPACE-BOUND-INVALID",
        2,
    ));
    let mut too_many_workspaces = base_request();
    too_many_workspaces["workspaces"] = Value::Array(
        (0..17)
            .map(|index| {
                json!({
                    "workspace_id": format!("ferris.test/workspace-{index}"),
                    "manifest_path": format!("not-resolved-{index}/Cargo.toml")
                })
            })
            .collect(),
    );
    cases.push((
        too_many_workspaces,
        "FERRIS-FEDERATED-PLAN-WORKSPACE-BOUND-INVALID",
        2,
    ));
    let mut duplicate_ids = base_request();
    duplicate_ids["workspaces"][1]["workspace_id"] = json!("ferris.test/beta");
    cases.push((
        duplicate_ids,
        "FERRIS-FEDERATED-PLAN-WORKSPACE-ID-DUPLICATE",
        2,
    ));
    let mut duplicate_manifests = base_request();
    duplicate_manifests["workspaces"][1]["manifest_path"] = json!("workspace-beta/./Cargo.toml");
    cases.push((
        duplicate_manifests,
        "FERRIS-FEDERATED-PLAN-MANIFEST-DUPLICATE",
        2,
    ));
    let mut absolute_manifest = base_request();
    absolute_manifest["workspaces"][0]["manifest_path"] = json!(
        fixture("federated-plan/workspace-beta/Cargo.toml")
            .to_string_lossy()
            .into_owned()
    );
    cases.push((
        absolute_manifest,
        "FERRIS-FEDERATED-PLAN-MANIFEST-ABSOLUTE",
        2,
    ));
    let mut backslash_manifest = base_request();
    backslash_manifest["workspaces"][0]["manifest_path"] = json!(r"workspace-beta\Cargo.toml");
    cases.push((
        backslash_manifest,
        "FERRIS-FEDERATED-PLAN-MANIFEST-SYNTAX-INVALID",
        2,
    ));
    let mut traversal_manifest = base_request();
    traversal_manifest["workspaces"][0]["manifest_path"] =
        json!("workspace-beta/../workspace-alpha/Cargo.toml");
    cases.push((
        traversal_manifest,
        "FERRIS-FEDERATED-PLAN-MANIFEST-TRAVERSAL",
        2,
    ));

    for (request, code, exit_code) in cases {
        write_request(&request_path, &request);
        let value =
            assert_typed_error_with_exit(run_json(&mut ferris(), &request_path), code, exit_code);
        assert_diagnostic_path_free(&value, &[&request_path, &directory.0]);
    }
}

#[test]
fn federated_plan_rejects_root_and_member_of_one_cargo_workspace() {
    let directory = TestDirectory::new("duplicate-workspace-root");
    copy_tree(
        &fixture("simple-workspace"),
        &directory.path("simple-workspace"),
    );
    let request_path = directory.path("request.json");
    let request = json!({
        "schema": "ferris.federated-plan-request/v0",
        "application_id": "ferris.test/federated",
        "revision": "r1",
        "owner": "ferris.test/owner",
        "workspaces": [
            {
                "workspace_id": "ferris.test/root",
                "manifest_path": "simple-workspace/Cargo.toml"
            },
            {
                "workspace_id": "ferris.test/member",
                "manifest_path": "simple-workspace/alpha/Cargo.toml"
            }
        ]
    });
    write_request(&request_path, &request);

    let value = assert_typed_error(
        run_json(&mut ferris(), &request_path),
        "FERRIS-FEDERATED-PLAN-WORKSPACE-ROOT-DUPLICATE",
    );
    assert!(
        value["diagnostics"][0]["message"]
            .as_str()
            .expect("diagnostic message")
            .contains("ferris.test/root")
    );
    assert_diagnostic_path_free(&value, &[&request_path, &directory.0]);
}

#[test]
fn federated_plan_rejects_workspace_root_above_request_parent() {
    let directory = TestDirectory::new("workspace-root-outside-request");
    copy_tree(
        &fixture("simple-workspace"),
        &directory.path("simple-workspace"),
    );
    let request_parent = directory.path("simple-workspace/alpha");
    fs::create_dir_all(request_parent.join("standalone")).expect("create second manifest parent");
    fs::write(
        request_parent.join("standalone/Cargo.toml"),
        b"[package]\nname = \"standalone\"\nversion = \"0.1.0\"\n",
    )
    .expect("write second manifest");
    let request_path = request_parent.join("request.json");
    let request = json!({
        "schema": "ferris.federated-plan-request/v0",
        "application_id": "ferris.test/federated",
        "revision": "r1",
        "owner": "ferris.test/owner",
        "workspaces": [
            {
                "workspace_id": "ferris.test/a-outer-root",
                "manifest_path": "Cargo.toml"
            },
            {
                "workspace_id": "ferris.test/z-standalone",
                "manifest_path": "standalone/Cargo.toml"
            }
        ]
    });
    write_request(&request_path, &request);

    let value = assert_typed_error(
        run_json(&mut ferris(), &request_path),
        "FERRIS-FEDERATED-PLAN-WORKSPACE-ROOT-OUTSIDE-REQUEST",
    );
    assert!(
        value["diagnostics"][0]["message"]
            .as_str()
            .expect("diagnostic message")
            .contains("ferris.test/a-outer-root")
    );
    assert_diagnostic_path_free(&value, &[&request_path, &directory.0]);
}

#[test]
fn federated_plan_error_identities_distinguish_schema_and_unavailable_request() {
    let directory = TestDirectory::new("error-identities");
    copy_tree(
        &fixture("federated-plan/workspace-alpha"),
        &directory.path("workspace-alpha"),
    );
    copy_tree(
        &fixture("federated-plan/workspace-beta"),
        &directory.path("workspace-beta"),
    );
    let request_path = directory.path("request.json");
    let mut unsupported = base_request();
    unsupported["schema"] = json!("ferris.federated-plan-request/v98");
    write_request(&request_path, &unsupported);
    let first = assert_typed_error_with_exit(
        run_json(&mut ferris(), &request_path),
        "FERRIS-FEDERATED-PLAN-REQUEST-SCHEMA-UNSUPPORTED",
        4,
    );
    unsupported["schema"] = json!("ferris.federated-plan-request/v99");
    write_request(&request_path, &unsupported);
    let second = assert_typed_error_with_exit(
        run_json(&mut ferris(), &request_path),
        "FERRIS-FEDERATED-PLAN-REQUEST-SCHEMA-UNSUPPORTED",
        4,
    );
    assert_ne!(first["selection_identity"], second["selection_identity"]);
    assert_ne!(first["invocation_identity"], second["invocation_identity"]);

    let first_missing = directory.path("missing-a/request.json");
    let second_missing = directory.path("missing-b/request.json");
    let first_output = run_json(&mut ferris(), &first_missing);
    let second_output = run_json(&mut ferris(), &second_missing);
    assert_eq!(first_output.status.code(), Some(5));
    assert_eq!(second_output.status.code(), Some(5));
    let first_error: Value =
        serde_json::from_slice(&first_output.stderr).expect("first unavailable request JSON");
    let second_error: Value =
        serde_json::from_slice(&second_output.stderr).expect("second unavailable request JSON");
    assert_eq!(
        first_error["diagnostics"][0]["code"],
        "FERRIS-FEDERATED-PLAN-REQUEST-UNAVAILABLE"
    );
    assert_eq!(
        second_error["diagnostics"][0]["code"],
        "FERRIS-FEDERATED-PLAN-REQUEST-UNAVAILABLE"
    );
    assert_ne!(
        first_error["selection_identity"],
        second_error["selection_identity"]
    );
    assert_ne!(
        first_error["invocation_identity"],
        second_error["invocation_identity"]
    );
    assert_diagnostic_path_free(&first_error, &[&first_missing, &directory.0]);
    assert_diagnostic_path_free(&second_error, &[&second_missing, &directory.0]);
}

#[test]
fn federated_plan_invalid_existing_manifest_names_workspace_without_paths() {
    let directory = TestDirectory::new("invalid-existing-manifest");
    copy_tree(
        &fixture("federated-plan/workspace-alpha"),
        &directory.path("workspace-alpha"),
    );
    fs::create_dir_all(directory.path("broken")).expect("create broken workspace");
    let invalid_manifest = directory.path("broken/Cargo.toml");
    fs::write(&invalid_manifest, b"[package\nname = \"broken\"\n").expect("write invalid manifest");
    let request_path = directory.path("request.json");
    let request = json!({
        "schema": "ferris.federated-plan-request/v0",
        "application_id": "ferris.test/federated",
        "revision": "r1",
        "owner": "ferris.test/owner",
        "workspaces": [
            {
                "workspace_id": "ferris.test/broken",
                "manifest_path": "broken/Cargo.toml"
            },
            {
                "workspace_id": "ferris.test/valid",
                "manifest_path": "workspace-alpha/Cargo.toml"
            }
        ]
    });
    write_request(&request_path, &request);

    let value = assert_typed_error(
        run_json(&mut ferris(), &request_path),
        "FERRIS-MANIFEST-INVALID",
    );
    assert!(
        value["diagnostics"][0]["message"]
            .as_str()
            .expect("diagnostic message")
            .contains("ferris.test/broken")
    );
    assert!(
        value["diagnostics"][0]["bounded_output"].is_object(),
        "Cargo failure should retain bounded output evidence"
    );
    assert_diagnostic_path_free(&value, &[&request_path, &invalid_manifest, &directory.0]);
}

#[test]
fn federated_plan_missing_manifest_error_is_typed_and_path_free() {
    let directory = TestDirectory::new("missing-manifest");
    let request_path = directory.path("request.json");
    let mut request = base_request();
    request["workspaces"][0]["manifest_path"] = json!("private/missing/Cargo.toml");
    write_request(&request_path, &request);

    let output = run_json(&mut ferris(), &request_path);
    let stderr = String::from_utf8(output.stderr.clone()).expect("error text");
    let value = assert_typed_error(output, "FERRIS-MANIFEST-NOT-FOUND");
    assert_eq!(value["record"], Value::Null);
    assert!(
        value["diagnostics"][0]["message"]
            .as_str()
            .expect("diagnostic message")
            .contains("ferris.test/beta")
    );
    assert!(!stderr.contains(&request_path.to_string_lossy().into_owned()));
    assert!(!stderr.contains("private/missing"));
    assert!(!stderr.contains(&directory.0.to_string_lossy().into_owned()));
    assert_diagnostic_path_free(&value, &[&request_path, &directory.0]);
}

#[test]
fn federated_plan_rejects_oversized_request_and_existing_plan_is_unchanged() {
    let directory = TestDirectory::new("oversized");
    let request_path = directory.path("request.json");
    fs::write(&request_path, vec![b'x'; 1024 * 1024 + 1]).expect("write oversized request");
    let output = run_json(&mut ferris(), &request_path);
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    let error: Value = serde_json::from_slice(&output.stderr).expect("oversized error JSON");
    assert_eq!(error["semantic_command_id"], "federated-plan");
    assert_eq!(error["result_class"], "incomplete");
    assert_eq!(
        error["diagnostics"][0]["code"],
        "FERRIS-FEDERATED-PLAN-REQUEST-OVERSIZED"
    );

    let plan = ferris()
        .args([
            "plan",
            "--workspace-id",
            "ferris.test/simple",
            "--manifest-path",
            fixture("simple-workspace/Cargo.toml")
                .to_str()
                .expect("fixture manifest"),
            "--format",
            "json",
        ])
        .output()
        .expect("run existing plan");
    assert!(plan.status.success());
    let value: Value = serde_json::from_slice(&plan.stdout).expect("plan JSON");
    assert_eq!(value["semantic_command_id"], "plan");
    assert_eq!(
        value["record"]["plan_id"],
        "plan:cdc17a3318a6ae17c4dbc847635f3dc4aa07b527f41d632ecedfc7be64a3290b"
    );
}
