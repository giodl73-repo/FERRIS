use ferris_core::{
    ACTION_PLAN_SCHEMA, ActionLane, ActionPlan, BoundFile, CleanupState, EXECUTION_APPROVAL_SCHEMA,
    EXECUTION_RECEIPT_SCHEMA, EntrypointCommand, ExecutionAggregateStatus, ExecutionApproval,
    ExecutionReceipt, LaneTerminalStatus, OWNER_ENTRYPOINTS_SCHEMA, OwnerEntrypoint,
    OwnerEntrypointDeclaration, action_plan_identity, current_execution_platform,
    execute_action_plan_with_cancellation, execution_approval_identity, file_content_identity,
    owner_entrypoint_declaration_identity, owner_entrypoint_identity, verify_execution_receipt,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn ferris() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ferris"))
}

static EXECUTION_TEST_LOCK: Mutex<()> = Mutex::new(());

fn serialize_execution_test() -> MutexGuard<'static, ()> {
    EXECUTION_TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

struct TestRepository {
    root: PathBuf,
    executable_relative: String,
    declaration: OwnerEntrypointDeclaration,
    approval: ExecutionApproval,
    plan: ActionPlan,
}

impl TestRepository {
    fn new(environment: Vec<String>, lane_count: usize) -> Self {
        Self::new_with_helper(environment, lane_count, "execution_helper_process")
    }

    fn new_with_helper(environment: Vec<String>, lane_count: usize, helper_test: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("ferris-execution-{}-{nonce}", std::process::id()));
        fs::create_dir_all(root.join("work")).expect("create repository");
        fs::write(root.join("tracked.txt"), "source\n").expect("write tracked file");
        run_git(&root, &["init", "--quiet"]);
        run_git(&root, &["config", "user.email", "ferris@example.invalid"]);
        run_git(&root, &["config", "user.name", "Ferris Test"]);
        run_git(&root, &["add", "tracked.txt"]);
        run_git(
            &root,
            &[
                "-c",
                "commit.gpgsign=false",
                "commit",
                "--quiet",
                "-m",
                "fixture",
            ],
        );
        let revision = git_output(&root, &["rev-parse", "HEAD"]);

        fs::create_dir_all(root.join("bin")).expect("create bin");
        let executable_name = if cfg!(windows) {
            "execution-helper.exe"
        } else {
            "execution-helper"
        };
        let executable_relative = format!("bin/{executable_name}");
        let executable = root.join("bin").join(executable_name);
        fs::copy(
            std::env::current_exe().expect("test executable"),
            &executable,
        )
        .expect("copy execution helper");
        let file_identity = file_content_identity(&executable).expect("helper identity");
        let command = EntrypointCommand {
            owner: "owner/test".to_owned(),
            executable: executable_relative.clone(),
            argv: vec![
                "--exact".to_owned(),
                helper_test.to_owned(),
                "--nocapture".to_owned(),
            ],
            working_directory: "work".to_owned(),
            inherited_environment: environment.clone(),
            credential_class: "none".to_owned(),
            files: vec![BoundFile {
                path: executable_relative.clone(),
                identity: file_identity,
            }],
        };
        let mut entrypoint = OwnerEntrypoint {
            entrypoint_id: "owner/test".to_owned(),
            entrypoint_identity: String::new(),
            command: command.clone(),
        };
        entrypoint.entrypoint_identity = owner_entrypoint_identity(&entrypoint);
        let mut declaration = OwnerEntrypointDeclaration {
            schema: OWNER_ENTRYPOINTS_SCHEMA.to_owned(),
            declaration_id: String::new(),
            source_revision: revision.clone(),
            entrypoints: vec![entrypoint.clone()],
        };
        declaration.declaration_id = owner_entrypoint_declaration_identity(&declaration);
        let mut approval = ExecutionApproval {
            schema: EXECUTION_APPROVAL_SCHEMA.to_owned(),
            approval_id: String::new(),
            action_plan_id: String::new(),
            principal: "test/principal".to_owned(),
            allowed_environment: environment,
            expires_at: "2999-01-01T00:00:00Z".to_owned(),
            revoked: false,
        };
        let lanes = (0..lane_count)
            .map(|index| ActionLane {
                lane_id: format!("lane-{index}"),
                owner_gate_id: format!("owner/gate-{index}"),
                required: true,
                depends_on: if index == 0 {
                    Vec::new()
                } else {
                    vec![format!("lane-{}", index - 1)]
                },
                entrypoint_id: entrypoint.entrypoint_id.clone(),
                entrypoint_identity: entrypoint.entrypoint_identity.clone(),
                command: command.clone(),
                timeout_ms: 2_000,
                stdout_limit_bytes: 64 * 1024,
                stderr_limit_bytes: 64 * 1024,
            })
            .collect();
        let mut plan = ActionPlan {
            schema: ACTION_PLAN_SCHEMA.to_owned(),
            action_plan_id: String::new(),
            repository_id: "owner/test-repository".to_owned(),
            source_revision: revision,
            topology_id: "test/topology".to_owned(),
            declaration_id: declaration.declaration_id.clone(),
            approval_id: String::new(),
            lanes,
        };
        Self::bind_plan_and_approval(&mut plan, &mut approval);
        let repository = Self {
            root,
            executable_relative,
            declaration,
            approval,
            plan,
        };
        repository.write_files();
        repository
    }

    fn bind_plan_and_approval(plan: &mut ActionPlan, approval: &mut ExecutionApproval) {
        plan.action_plan_id = action_plan_identity(plan);
        approval.action_plan_id = plan.action_plan_id.clone();
        approval.approval_id = execution_approval_identity(approval);
        plan.approval_id = approval.approval_id.clone();
        assert_eq!(action_plan_identity(plan), plan.action_plan_id);
    }

    #[cfg(windows)]
    fn use_immediate_descendant_helper(&mut self) {
        let source = self.root.join("immediate-descendant.rs");
        fs::write(
            &source,
            r#"use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let mut args = std::env::args();
    let executable = args.next().expect("executable");
    let mode = args.next().expect("mode");
    let heartbeat = args.next().expect("heartbeat");
    if mode == "parent" {
        Command::new(executable)
            .args(["child", &heartbeat])
            .spawn()
            .expect("spawn immediate child");
        thread::sleep(Duration::from_secs(60));
    } else {
        loop {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&heartbeat)
                .expect("open heartbeat")
                .write_all(b"x")
                .expect("write heartbeat");
            thread::sleep(Duration::from_millis(20));
        }
    }
}
"#,
        )
        .expect("write immediate descendant helper");
        let executable = self.root.join(&self.executable_relative);
        let output = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
            .args(["--edition=2024"])
            .arg(&source)
            .arg("-o")
            .arg(&executable)
            .output()
            .expect("compile immediate descendant helper");
        assert!(output.status.success(), "{output:?}");

        let command = {
            let command = &mut self.declaration.entrypoints[0].command;
            command.argv = vec!["parent".to_owned(), "../heartbeat".to_owned()];
            command.files[0].identity =
                file_content_identity(&executable).expect("immediate helper identity");
            command.clone()
        };
        self.declaration.entrypoints[0].entrypoint_identity =
            owner_entrypoint_identity(&self.declaration.entrypoints[0]);
        self.declaration.declaration_id = owner_entrypoint_declaration_identity(&self.declaration);
        for lane in &mut self.plan.lanes {
            lane.entrypoint_identity = self.declaration.entrypoints[0].entrypoint_identity.clone();
            lane.command = command.clone();
        }
        self.plan.declaration_id = self.declaration.declaration_id.clone();
        Self::bind_plan_and_approval(&mut self.plan, &mut self.approval);
        self.write_files();
    }

    fn write_files(&self) {
        write_execution_file(
            &self.root,
            "entrypoints",
            &self.declaration.declaration_id,
            &self.declaration,
        );
        write_execution_file(
            &self.root,
            "approvals",
            &self.approval.approval_id,
            &self.approval,
        );
        write_execution_file(
            &self.root,
            "action-plans",
            &self.plan.action_plan_id,
            &self.plan,
        );
    }

    fn run_go(&self, environment: &[(&str, &str)]) -> Output {
        let mut command = ferris();
        command
            .current_dir(&self.root)
            .args(["go", "--action-plan", &self.plan.action_plan_id]);
        for (name, value) in environment {
            command.env(name, value);
        }
        command.output().expect("run ferris go")
    }

    fn receipt_path(&self, receipt: &ExecutionReceipt) -> PathBuf {
        self.root.join(".ferris").join("receipts").join(format!(
            "{}.json",
            receipt
                .receipt_id
                .strip_prefix("sha256:")
                .expect("receipt digest")
        ))
    }
}

impl Drop for TestRepository {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn write_execution_file(root: &Path, kind: &str, identity: &str, value: &impl serde::Serialize) {
    let directory = root.join(".ferris").join(kind);
    fs::create_dir_all(&directory).expect("create execution file directory");
    let path = directory.join(format!(
        "{}.json",
        identity.strip_prefix("sha256:").expect("sha256 identity")
    ));
    fs::write(
        path,
        serde_json::to_vec_pretty(value).expect("serialize execution file"),
    )
    .expect("write execution file");
}

fn run_git(root: &Path, arguments: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(arguments)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{output:?}");
}

fn git_output(root: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(root)
        .args(arguments)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{output:?}");
    String::from_utf8(output.stdout)
        .expect("git UTF-8")
        .trim()
        .to_owned()
}

fn parse_receipt(output: &Output) -> ExecutionReceipt {
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: ExecutionReceipt =
        serde_json::from_slice(&output.stdout).expect("execution receipt");
    assert_eq!(receipt.schema, EXECUTION_RECEIPT_SCHEMA);
    receipt
}

#[test]
#[allow(clippy::zombie_processes)]
fn execution_helper_process() {
    let Ok(mode) = std::env::var("FERRIS_TEST_MODE") else {
        return;
    };
    match mode.as_str() {
        "inspect" => {
            let environment = std::env::vars()
                .map(|(name, _)| name)
                .collect::<BTreeSet<_>>();
            println!(
                "FERRIS_FIXTURE:{}",
                serde_json::to_string(&json!({
                    "argv": std::env::args().skip(1).collect::<Vec<_>>(),
                    "cwd": std::env::current_dir().expect("cwd"),
                    "environment": environment,
                }))
                .expect("fixture JSON")
            );
        }
        "fail" => panic!("owner command failed"),
        "sleep" => thread::sleep(Duration::from_secs(60)),
        "tree-parent" => {
            let mut child = Command::new(std::env::current_exe().expect("current executable"));
            child
                .args(["--exact", "execution_helper_process", "--nocapture"])
                .env("FERRIS_TEST_MODE", "tree-child");
            child.spawn().expect("spawn descendant");
            thread::sleep(Duration::from_secs(60));
        }
        "tree-child" => {
            let heartbeat = std::env::var_os("FERRIS_TEST_HEARTBEAT")
                .map(PathBuf::from)
                .expect("heartbeat path");
            loop {
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&heartbeat)
                    .expect("open heartbeat");
                file.write_all(b"x").expect("write heartbeat");
                drop(file);
                thread::sleep(Duration::from_millis(20));
            }
        }
        "leak" => println!(
            "{}",
            std::env::var("FERRIS_ALLOWED_VALUE").expect("allowed value")
        ),
        "overflow" => println!("{}", "x".repeat(32 * 1024)),
        other => panic!("unknown helper mode {other}"),
    }
}

#[test]
#[allow(clippy::zombie_processes)]
fn execution_cancellation_parent_process() {
    if !running_as_copied_helper() {
        return;
    }
    Command::new(std::env::current_exe().expect("current executable"))
        .args([
            "--exact",
            "execution_cancellation_descendant_process",
            "--nocapture",
        ])
        .spawn()
        .expect("spawn cancellation descendant");
    thread::sleep(Duration::from_secs(60));
}

#[test]
fn execution_cancellation_descendant_process() {
    if !running_as_copied_helper() {
        return;
    }
    let heartbeat = std::env::current_dir()
        .expect("current directory")
        .parent()
        .expect("repository root")
        .join("cancel-heartbeat");
    loop {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&heartbeat)
            .expect("open cancellation heartbeat");
        file.write_all(b"x").expect("write cancellation heartbeat");
        drop(file);
        thread::sleep(Duration::from_millis(20));
    }
}

fn running_as_copied_helper() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .and_then(|path| path.file_name().map(|name| name.to_owned()))
        .is_some_and(|name| name == "bin")
}

#[test]
fn executes_exact_argv_cwd_and_environment() {
    let _guard = serialize_execution_test();
    let repository = TestRepository::new(
        vec![
            "FERRIS_ALLOWED_VALUE".to_owned(),
            "FERRIS_TEST_MODE".to_owned(),
        ],
        1,
    );
    let output = repository.run_go(&[
        ("FERRIS_TEST_MODE", "inspect"),
        ("FERRIS_ALLOWED_VALUE", "allowed-value-not-output"),
        ("FERRIS_UNAPPROVED", "must-not-be-inherited"),
    ]);
    assert!(output.status.success(), "{output:?}");
    let receipt = parse_receipt(&output);
    assert_eq!(receipt.lanes[0].status, LaneTerminalStatus::Succeeded);
    assert_eq!(receipt.repository_id, "owner/test-repository");
    assert_eq!(receipt.lanes[0].owner_gate_id, "owner/gate-0");
    assert_eq!(receipt.platform, current_execution_platform());
    assert_ne!(
        receipt.lanes[0].environment_identity,
        "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"
    );
    let marker = receipt.lanes[0]
        .stdout
        .diagnostic_tail
        .lines()
        .find_map(|line| line.strip_prefix("FERRIS_FIXTURE:"))
        .expect("fixture marker");
    let observed: Value = serde_json::from_str(marker).expect("fixture output");
    assert_eq!(
        observed["argv"],
        json!(["--exact", "execution_helper_process", "--nocapture"])
    );
    assert_eq!(
        PathBuf::from(observed["cwd"].as_str().expect("cwd"))
            .canonicalize()
            .expect("canonical observed cwd"),
        repository
            .root
            .join("work")
            .canonicalize()
            .expect("canonical expected cwd")
    );
    assert_eq!(
        observed["environment"],
        json!(["FERRIS_ALLOWED_VALUE", "FERRIS_TEST_MODE"])
    );
    assert!(repository.receipt_path(&receipt).is_file());
}

#[test]
fn rejects_changed_plan_id_and_bound_file_before_launch() {
    let _guard = serialize_execution_test();
    let mut changed_id = TestRepository::new(vec!["FERRIS_TEST_MODE".to_owned()], 1);
    changed_id.plan.action_plan_id =
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned();
    write_execution_file(
        &changed_id.root,
        "action-plans",
        "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        &changed_id.plan,
    );
    let output = ferris()
        .current_dir(&changed_id.root)
        .args([
            "go",
            "--action-plan",
            "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        ])
        .output()
        .expect("run changed identity");
    assert_eq!(output.status.code(), Some(ResultClassCode::Stale as i32));

    let changed_file = TestRepository::new(vec!["FERRIS_TEST_MODE".to_owned()], 1);
    fs::OpenOptions::new()
        .append(true)
        .open(changed_file.root.join(&changed_file.executable_relative))
        .expect("open helper")
        .write_all(b"changed")
        .expect("change helper");
    let output = changed_file.run_go(&[("FERRIS_TEST_MODE", "inspect")]);
    assert_eq!(output.status.code(), Some(ResultClassCode::Stale as i32));
}

#[test]
fn approval_identity_covers_and_enforces_the_plan_binding() {
    let _guard = serialize_execution_test();
    let mut repository = TestRepository::new(vec!["FERRIS_TEST_MODE".to_owned()], 1);
    let original_plan_id = repository.plan.action_plan_id.clone();
    let original_approval_id = repository.approval.approval_id.clone();

    let mut selected_approval_changed = repository.plan.clone();
    selected_approval_changed.approval_id =
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned();
    assert_eq!(
        action_plan_identity(&selected_approval_changed),
        original_plan_id,
        "approval selection must not create an identity cycle"
    );

    repository.approval.action_plan_id =
        "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_owned();
    assert_ne!(
        execution_approval_identity(&repository.approval),
        original_approval_id,
        "approval identity must cover action_plan_id"
    );
    write_execution_file(
        &repository.root,
        "approvals",
        &original_approval_id,
        &repository.approval,
    );
    let output = repository.run_go(&[("FERRIS_TEST_MODE", "inspect")]);
    assert_eq!(output.status.code(), Some(ResultClassCode::Stale as i32));

    let mut rebound = TestRepository::new(vec!["FERRIS_TEST_MODE".to_owned()], 1);
    rebound.approval.action_plan_id =
        "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_owned();
    rebound.approval.approval_id = execution_approval_identity(&rebound.approval);
    rebound.plan.approval_id = rebound.approval.approval_id.clone();
    assert_eq!(
        action_plan_identity(&rebound.plan),
        rebound.plan.action_plan_id
    );
    rebound.write_files();
    let output = rebound.run_go(&[("FERRIS_TEST_MODE", "inspect")]);
    assert_eq!(output.status.code(), Some(ResultClassCode::Stale as i32));
}

#[test]
fn rejects_unknown_entrypoint_before_launch() {
    let _guard = serialize_execution_test();
    let mut repository = TestRepository::new(vec!["FERRIS_TEST_MODE".to_owned()], 1);
    repository.plan.lanes[0].entrypoint_id = "owner/missing".to_owned();
    TestRepository::bind_plan_and_approval(&mut repository.plan, &mut repository.approval);
    repository.write_files();
    let output = repository.run_go(&[("FERRIS_TEST_MODE", "inspect")]);
    assert_eq!(output.status.code(), Some(ResultClassCode::Invalid as i32));
}

#[test]
fn preserves_nonzero_and_blocks_every_dependent_lane() {
    let _guard = serialize_execution_test();
    let repository = TestRepository::new(vec!["FERRIS_TEST_MODE".to_owned()], 2);
    let output = repository.run_go(&[("FERRIS_TEST_MODE", "fail")]);
    assert_eq!(output.status.code(), Some(ResultClassCode::Failed as i32));
    let receipt = parse_receipt(&output);
    assert_eq!(receipt.selected_lane_count, 2);
    assert_eq!(receipt.lanes.len(), 2);
    assert_eq!(receipt.lanes[0].status, LaneTerminalStatus::Failed);
    assert_eq!(receipt.lanes[0].exit_code, Some(101));
    assert_eq!(
        receipt.lanes[1].status,
        LaneTerminalStatus::BlockedByDependency
    );
}

#[test]
fn timeout_terminates_the_full_process_tree() {
    let _guard = serialize_execution_test();
    let repository = TestRepository::new(
        vec![
            "FERRIS_TEST_HEARTBEAT".to_owned(),
            "FERRIS_TEST_MODE".to_owned(),
        ],
        1,
    );
    #[cfg(windows)]
    let mut repository = repository;
    #[cfg(windows)]
    repository.use_immediate_descendant_helper();
    let heartbeat = repository.root.join("heartbeat");
    let mut plan = repository.plan.clone();
    plan.lanes[0].timeout_ms = 350;
    let mut approval = repository.approval.clone();
    TestRepository::bind_plan_and_approval(&mut plan, &mut approval);
    write_execution_file(
        &repository.root,
        "action-plans",
        &plan.action_plan_id,
        &plan,
    );
    write_execution_file(
        &repository.root,
        "approvals",
        &approval.approval_id,
        &approval,
    );
    let output = ferris()
        .current_dir(&repository.root)
        .env("FERRIS_TEST_MODE", "tree-parent")
        .env("FERRIS_TEST_HEARTBEAT", &heartbeat)
        .args(["go", "--action-plan", &plan.action_plan_id])
        .output()
        .expect("run timeout");
    assert_eq!(output.status.code(), Some(ResultClassCode::Failed as i32));
    let receipt = parse_receipt(&output);
    assert_eq!(receipt.lanes[0].status, LaneTerminalStatus::TimedOut);
    let deadline = Instant::now() + Duration::from_secs(2);
    while !heartbeat.is_file() && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(20));
    }
    assert!(heartbeat.is_file(), "descendant produced no heartbeat");
    thread::sleep(Duration::from_millis(200));
    let first = fs::metadata(&heartbeat).expect("heartbeat metadata").len();
    thread::sleep(Duration::from_millis(250));
    let second = fs::metadata(&heartbeat).expect("heartbeat metadata").len();
    assert_eq!(first, second, "descendant survived process-tree cleanup");
}

#[test]
fn cancellation_terminates_tree_and_accounts_for_remaining_lanes() {
    let _guard = serialize_execution_test();
    let mut repository =
        TestRepository::new_with_helper(Vec::new(), 3, "execution_cancellation_parent_process");
    repository.plan.lanes[2].depends_on.clear();
    TestRepository::bind_plan_and_approval(&mut repository.plan, &mut repository.approval);
    repository.write_files();

    let heartbeat = repository.root.join("cancel-heartbeat");
    let cancellation = Arc::new(AtomicBool::new(false));
    let setter_cancellation = Arc::clone(&cancellation);
    let setter_heartbeat = heartbeat.clone();
    let setter = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(10);
        while !setter_heartbeat.is_file() && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(10));
        }
        let started = setter_heartbeat.is_file();
        setter_cancellation.store(true, Ordering::Release);
        started
    });

    let outcome = execute_action_plan_with_cancellation(
        &repository.root,
        &repository.plan.action_plan_id,
        cancellation.as_ref(),
    )
    .expect("cancel execution");
    assert!(
        setter.join().expect("cancellation setter"),
        "lane did not start"
    );
    assert_eq!(
        outcome.receipt.aggregate_status,
        ExecutionAggregateStatus::Cancelled
    );
    assert_eq!(
        outcome.receipt.result_class().exit_code(),
        ResultClassCode::Cancelled as u8
    );
    assert_eq!(outcome.receipt.lanes.len(), 3);
    assert_eq!(
        outcome.receipt.lanes[0].status,
        LaneTerminalStatus::Cancelled
    );
    assert_eq!(outcome.receipt.lanes[0].cleanup, CleanupState::Complete);
    assert_eq!(
        outcome.receipt.lanes[1].status,
        LaneTerminalStatus::BlockedByDependency
    );
    assert_eq!(
        outcome.receipt.lanes[2].status,
        LaneTerminalStatus::Cancelled
    );
    verify_execution_receipt(&outcome.receipt_path).expect("verify cancellation receipt");

    thread::sleep(Duration::from_millis(200));
    let first = fs::metadata(&heartbeat)
        .expect("cancellation heartbeat metadata")
        .len();
    thread::sleep(Duration::from_millis(250));
    let second = fs::metadata(&heartbeat)
        .expect("cancellation heartbeat metadata")
        .len();
    assert_eq!(first, second, "descendant survived cancellation cleanup");
}

#[test]
fn leaked_environment_value_is_redacted_and_terminal() {
    let _guard = serialize_execution_test();
    let repository = TestRepository::new(
        vec![
            "FERRIS_ALLOWED_VALUE".to_owned(),
            "FERRIS_TEST_MODE".to_owned(),
        ],
        1,
    );
    let secret = "allowed-value-must-not-persist";
    let output = repository.run_go(&[
        ("FERRIS_TEST_MODE", "leak"),
        ("FERRIS_ALLOWED_VALUE", secret),
    ]);
    assert_eq!(output.status.code(), Some(ResultClassCode::Failed as i32));
    assert!(
        !output
            .stdout
            .windows(secret.len())
            .any(|part| part == secret.as_bytes())
    );
    let receipt = parse_receipt(&output);
    assert_eq!(receipt.lanes[0].status, LaneTerminalStatus::LeakedSecret);
    assert_eq!(receipt.lanes[0].cleanup, CleanupState::Complete);
    assert!(
        receipt.lanes[0]
            .stdout
            .diagnostic_tail
            .contains("[REDACTED]")
    );
    let persisted = fs::read(repository.receipt_path(&receipt)).expect("receipt bytes");
    assert!(
        !persisted
            .windows(secret.len())
            .any(|part| part == secret.as_bytes())
    );
}

#[test]
fn optional_leaked_secret_forces_failed_aggregate() {
    let _guard = serialize_execution_test();
    let mut repository = TestRepository::new(
        vec![
            "FERRIS_ALLOWED_VALUE".to_owned(),
            "FERRIS_TEST_MODE".to_owned(),
        ],
        1,
    );
    repository.plan.lanes[0].required = false;
    TestRepository::bind_plan_and_approval(&mut repository.plan, &mut repository.approval);
    repository.write_files();

    let output = repository.run_go(&[
        ("FERRIS_TEST_MODE", "leak"),
        ("FERRIS_ALLOWED_VALUE", "optional-leak-must-fail"),
    ]);
    assert_eq!(output.status.code(), Some(ResultClassCode::Failed as i32));
    let receipt = parse_receipt(&output);
    assert_eq!(receipt.lanes[0].status, LaneTerminalStatus::LeakedSecret);
    assert_eq!(receipt.aggregate_status, ExecutionAggregateStatus::Failed);
}

#[test]
fn output_overflow_is_typed_and_bounded() {
    let _guard = serialize_execution_test();
    let repository = TestRepository::new(vec!["FERRIS_TEST_MODE".to_owned()], 1);
    let mut plan = repository.plan.clone();
    plan.lanes[0].stdout_limit_bytes = 1024;
    let mut approval = repository.approval.clone();
    TestRepository::bind_plan_and_approval(&mut plan, &mut approval);
    write_execution_file(
        &repository.root,
        "action-plans",
        &plan.action_plan_id,
        &plan,
    );
    write_execution_file(
        &repository.root,
        "approvals",
        &approval.approval_id,
        &approval,
    );
    let output = ferris()
        .current_dir(&repository.root)
        .env("FERRIS_TEST_MODE", "overflow")
        .args(["go", "--action-plan", &plan.action_plan_id])
        .output()
        .expect("run overflow");
    let receipt = parse_receipt(&output);
    assert_eq!(
        receipt.lanes[0].status,
        LaneTerminalStatus::OutputLimitExceeded
    );
    assert!(receipt.lanes[0].stdout.retained_bytes <= 1024);
    assert!(receipt.lanes[0].stdout.truncated);
}

#[test]
fn verifies_receipt_semantics_and_excludes_elapsed_time() {
    let _guard = serialize_execution_test();
    let repository = TestRepository::new(vec!["FERRIS_TEST_MODE".to_owned()], 1);
    let output = repository.run_go(&[("FERRIS_TEST_MODE", "inspect")]);
    let receipt = parse_receipt(&output);
    let receipt_path = repository.receipt_path(&receipt);
    let verify = ferris()
        .args(["verify"])
        .arg(&receipt_path)
        .output()
        .expect("verify receipt");
    assert!(verify.status.success(), "{verify:?}");
    let verification: Value = serde_json::from_slice(&verify.stdout).expect("verification JSON");
    assert_eq!(verification["valid"], true);
    assert_eq!(verification["receipt_id"], receipt.receipt_id);

    let mut elapsed_changed = receipt.clone();
    elapsed_changed.lanes[0].elapsed_ms = elapsed_changed.lanes[0].elapsed_ms.saturating_add(9_999);
    fs::write(
        &receipt_path,
        serde_json::to_vec_pretty(&elapsed_changed).expect("elapsed receipt"),
    )
    .expect("write elapsed receipt");
    let elapsed_verification = ferris()
        .args(["verify"])
        .arg(&receipt_path)
        .output()
        .expect("verify elapsed receipt");
    assert!(elapsed_verification.status.success());

    elapsed_changed.lanes[0].status = LaneTerminalStatus::Failed;
    fs::write(
        &receipt_path,
        serde_json::to_vec_pretty(&elapsed_changed).expect("changed receipt"),
    )
    .expect("write changed receipt");
    let changed_verification = ferris()
        .args(["verify"])
        .arg(&receipt_path)
        .output()
        .expect("verify changed receipt");
    assert_eq!(
        changed_verification.status.code(),
        Some(ResultClassCode::Invalid as i32)
    );
}

#[test]
fn go_changed_and_full_are_structurally_rejected() {
    let _guard = serialize_execution_test();
    let id = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    for flag in ["--changed", "--full"] {
        let output = ferris()
            .args(["go", "--action-plan", id, flag])
            .output()
            .expect("run forbidden flag");
        assert_eq!(output.status.code(), Some(ResultClassCode::Invalid as i32));
    }
}

#[repr(i32)]
enum ResultClassCode {
    Invalid = 2,
    Stale = 6,
    Cancelled = 8,
    Failed = 10,
}
