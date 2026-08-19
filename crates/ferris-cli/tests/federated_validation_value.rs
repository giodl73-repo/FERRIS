use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const WORKSPACE_COUNT: usize = 8;

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/federated-value-application")
        .join(path)
}

fn run_plan(changed_package: Option<&str>, changed_path: Option<&Path>) -> (Value, Duration) {
    let changed_packages = changed_package.into_iter().collect::<Vec<_>>();
    let changed_paths = changed_path.into_iter().collect::<Vec<_>>();
    run_plan_inputs(&changed_packages, &changed_paths)
}

fn run_plan_inputs(changed_packages: &[&str], changed_paths: &[&Path]) -> (Value, Duration) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ferris"));
    command
        .arg("federated-validation-plan")
        .arg("--application")
        .arg(fixture("application.json"));
    for package in changed_packages {
        command.arg("--changed-package").arg(package);
    }
    for path in changed_paths {
        command.arg("--changed-path").arg(path);
    }
    command.args(["--format", "json"]);

    let started = Instant::now();
    let output = command.output().expect("run federated validation plan");
    let elapsed = started.elapsed();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    (
        serde_json::from_slice(&output.stdout).expect("federated validation JSON"),
        elapsed,
    )
}

fn disposition_count(value: &Value, disposition: &str) -> usize {
    value["record"]["workspaces"]
        .as_array()
        .expect("workspaces")
        .iter()
        .filter(|workspace| workspace["disposition"] == disposition)
        .count()
}

fn selected_workspace_count(value: &Value) -> usize {
    WORKSPACE_COUNT - disposition_count(value, "not_selected")
}

fn selected_workspace_ids(value: &Value) -> Vec<&str> {
    value["record"]["workspaces"]
        .as_array()
        .expect("workspaces")
        .iter()
        .filter(|workspace| workspace["disposition"] != "not_selected")
        .map(|workspace| workspace["workspace_id"].as_str().expect("workspace ID"))
        .collect()
}

fn workspace<'a>(value: &'a Value, workspace_id: &str) -> &'a Value {
    value["record"]["workspaces"]
        .as_array()
        .expect("workspaces")
        .iter()
        .find(|workspace| workspace["workspace_id"] == workspace_id)
        .expect("workspace record")
}

fn assert_common_contract(value: &Value) {
    assert_eq!(value["schema"], "ferris.command-result/v2");
    assert_eq!(
        value["record"]["schema"],
        "ferris.federated-validation-plan/v0"
    );
    assert_eq!(value["record"]["executable"], false);
    assert_eq!(
        value["record"]["workspaces"]
            .as_array()
            .expect("workspaces")
            .len(),
        WORKSPACE_COUNT
    );
}

#[test]
fn bounded_application_quantifies_narrowing_and_safe_fallback() {
    let (leaf, _) = run_plan(Some("ferris.benchmark/cli:fvb-cli"), None);
    assert_common_contract(&leaf);
    assert_eq!(disposition_count(&leaf, "direct_plan"), 1);
    assert_eq!(disposition_count(&leaf, "relationship_fallback"), 0);
    assert_eq!(disposition_count(&leaf, "not_selected"), 7);
    assert_eq!(selected_workspace_count(&leaf), 1);

    let (shared, _) = run_plan(Some("ferris.benchmark/domain:fvb-domain"), None);
    assert_common_contract(&shared);
    assert_eq!(disposition_count(&shared, "direct_plan"), 1);
    assert_eq!(disposition_count(&shared, "relationship_fallback"), 4);
    assert_eq!(disposition_count(&shared, "not_selected"), 3);
    assert_eq!(selected_workspace_count(&shared), 5);

    let policy = fixture("application-policy.txt");
    let (application, _) = run_plan(None, Some(&policy));
    assert_common_contract(&application);
    assert_eq!(
        disposition_count(&application, "application_fallback"),
        WORKSPACE_COUNT
    );
    assert_eq!(selected_workspace_count(&application), WORKSPACE_COUNT);
    assert_eq!(
        application["record"]["fallback"]["required_by_inputs"],
        true
    );
}

#[test]
fn scenario_matrix_preserves_monotonic_widening_and_input_union() {
    let response_curve = [
        ("ferris.benchmark/foundation:fvb-foundation", 8usize),
        ("ferris.benchmark/contracts:fvb-contracts", 6usize),
        ("ferris.benchmark/domain:fvb-domain", 5usize),
        ("ferris.benchmark/api:fvb-api", 3usize),
        ("ferris.benchmark/cli:fvb-cli", 1usize),
    ];
    for (changed_package, expected_selected) in response_curve {
        let (value, _) = run_plan(Some(changed_package), None);
        assert_common_contract(&value);
        assert_eq!(selected_workspace_count(&value), expected_selected);
    }

    let packages = [
        "ferris.benchmark/api:fvb-api",
        "ferris.benchmark/analytics:fvb-analytics",
    ];
    let (multi_branch, _) = run_plan_inputs(&packages, &[]);
    assert_common_contract(&multi_branch);
    assert_eq!(disposition_count(&multi_branch, "direct_plan"), 2);
    assert_eq!(disposition_count(&multi_branch, "relationship_fallback"), 2);
    assert_eq!(selected_workspace_count(&multi_branch), 4);
    assert_eq!(
        selected_workspace_ids(&multi_branch),
        [
            "ferris.benchmark/admin",
            "ferris.benchmark/analytics",
            "ferris.benchmark/api",
            "ferris.benchmark/cli",
        ]
    );

    let reversed = [packages[1], packages[0]];
    let (reordered, _) = run_plan_inputs(&reversed, &[]);
    assert_eq!(
        reordered["selection_identity"],
        multi_branch["selection_identity"]
    );
    assert_eq!(
        reordered["invocation_identity"],
        multi_branch["invocation_identity"]
    );
    assert_eq!(
        reordered["record"]["federated_validation_plan_id"],
        multi_branch["record"]["federated_validation_plan_id"]
    );
}

#[test]
fn manifest_and_mixed_inputs_preserve_nested_fallback_without_global_widening() {
    let manifest = fixture("domain/Cargo.toml");
    let (manifest_only, _) = run_plan(None, Some(&manifest));
    assert_common_contract(&manifest_only);
    assert_eq!(disposition_count(&manifest_only, "direct_plan"), 1);
    assert_eq!(
        disposition_count(&manifest_only, "relationship_fallback"),
        4
    );
    assert_eq!(selected_workspace_count(&manifest_only), 5);
    assert_eq!(
        manifest_only["record"]["fallback"]["required_by_inputs"],
        false
    );

    let domain = workspace(&manifest_only, "ferris.benchmark/domain");
    assert_eq!(
        domain["validation_plan"]["fallback"]["required_by_inputs"],
        true
    );
    assert_eq!(
        domain["validation_plan"]["selected_packages"]
            .as_array()
            .expect("selected packages")
            .len(),
        0
    );
    assert_eq!(
        domain["validation_plan"]["fallback"]["packages"]
            .as_array()
            .expect("fallback packages")
            .len(),
        1
    );

    let changed_packages = ["ferris.benchmark/analytics:fvb-analytics"];
    let changed_paths = [manifest.as_path()];
    let (mixed, _) = run_plan_inputs(&changed_packages, &changed_paths);
    assert_common_contract(&mixed);
    assert_eq!(disposition_count(&mixed, "direct_plan"), 2);
    assert_eq!(disposition_count(&mixed, "relationship_fallback"), 4);
    assert_eq!(selected_workspace_count(&mixed), 6);
    assert_eq!(
        selected_workspace_ids(&mixed),
        [
            "ferris.benchmark/admin",
            "ferris.benchmark/analytics",
            "ferris.benchmark/api",
            "ferris.benchmark/cli",
            "ferris.benchmark/domain",
            "ferris.benchmark/worker",
        ]
    );
}

#[test]
#[ignore = "run explicitly to emit local planning-overhead samples"]
fn report_local_planning_overhead_and_scope_reduction() {
    let scenarios = [
        ("leaf", Some("ferris.benchmark/cli:fvb-cli"), None, 1usize),
        (
            "shared",
            Some("ferris.benchmark/domain:fvb-domain"),
            None,
            5usize,
        ),
        (
            "application",
            None,
            Some(fixture("application-policy.txt")),
            8usize,
        ),
    ];

    for (name, changed_package, changed_path, expected_selected) in scenarios {
        let mut samples = Vec::new();
        for _ in 0..7 {
            let (value, elapsed) = run_plan(changed_package, changed_path.as_deref());
            assert_common_contract(&value);
            assert_eq!(selected_workspace_count(&value), expected_selected);
            samples.push(elapsed);
        }
        samples.sort();
        let median = samples[samples.len() / 2];
        let avoided = WORKSPACE_COUNT - expected_selected;
        println!(
            "{}",
            json!({
                "schema": "ferris.federated-validation-value/v0",
                "scenario": name,
                "samples": samples.len(),
                "median_planning_ms": median.as_secs_f64() * 1000.0,
                "baseline_workspace_scopes": WORKSPACE_COUNT,
                "required_workspace_scopes": expected_selected,
                "avoided_workspace_scopes": avoided,
                "scope_reduction_percent": avoided as f64 * 100.0 / WORKSPACE_COUNT as f64,
                "validation_executed": false
            })
        );
    }
}
