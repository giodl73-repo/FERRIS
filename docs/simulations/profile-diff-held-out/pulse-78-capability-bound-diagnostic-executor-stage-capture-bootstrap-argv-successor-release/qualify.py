#!/usr/bin/env python3
"""Run deterministic fake-only Pulse 78 qualification cycles."""

from __future__ import annotations

import argparse
import json
import shutil
import sys
import unittest
from pathlib import Path
from unittest.mock import patch


sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent
REPO_ROOT = ROOT.parents[3]
PROFILE_DIFF_ROOT = REPO_ROOT / "docs" / "simulations" / "profile-diff-held-out"
for cache in PROFILE_DIFF_ROOT.rglob("__pycache__"):
    shutil.rmtree(cache, ignore_errors=True)
P51_ROOT = PROFILE_DIFF_ROOT / "pulse-51-diagnostic-executor-release"
WORK_ROOT = REPO_ROOT / "target" / "pulse-78-qualification-runtime"
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import capability_bound_diagnostic_executor_successor as executor  # noqa: E402
from fixtures.fake_p56 import FakeP56  # noqa: E402
from fixtures.p78_fake_native_wsl import (  # noqa: E402
    FakeBundleManager,
    FakeWorkerProcessFactory,
)
import sealed_dependencies as local_sealed  # noqa: E402

executor._bind_local_sealed_lock_manager_module(local_sealed)
canonical_bytes = local_sealed.canonical_bytes
load_exact_p75_stack = local_sealed.load_exact_p75_stack
load_p51_synthetic_fixture = local_sealed.load_p51_synthetic_fixture
sha256_bytes = local_sealed.sha256_bytes

CONTROL_METHODS = {
    "exact-p75-binding-and-signature": "test_exact_p75_binding_and_production_signature_match",
    "local-loader-ignores-ambient-module": "test_local_loader_reads_sibling_module_not_ambient_state",
    "local-loader-fresh-modules": "test_local_loader_returns_fresh_module_each_time",
    "stage-bootstrap-identity-revalidation": "test_stage_bootstrap_binds_identity_and_revalidates_before_spawn",
    "stage-post-create-failure-cleanup": "test_stage_post_create_failure_cleanup_is_owned_inside_bootstrap",
    "stage-cleanup-indeterminate-precedence": "test_stage_failure_cleanup_indeterminate_takes_precedence",
    "worker-bootstrap-root-swap-rejected": "test_worker_bootstrap_rejects_root_swap_after_revalidation",
    "worker-bootstrap-path-swap-rejected": "test_worker_bootstrap_rejects_worker_path_swap",
    "worker-bootstrap-ready-close-exact-args": "test_worker_bootstrap_ready_close_uses_exact_worker_args",
    "worker-bootstrap-dependency-loader-binding": "test_worker_bootstrap_rejects_dependency_loader_binding_mismatch",
    "stage-create-open-substitution-indeterminate": "test_stage_create_open_substitution_is_indeterminate_not_clean_failure",
    "bundle-retained-and-zero-residue": "test_full_fake_cycle_retains_bundle_until_close_and_leaves_no_residue",
    "startup-failure-cleanup": "test_startup_failure_removes_staged_bundle",
    "terminate-kill-then-bundle-cleanup": "test_close_timeout_kills_worker_and_removes_bundle",
    "prelaunch-root-substitution-rejected": "test_prelaunch_root_substitution_causes_indeterminate_cleanup",
    "prelaunch-parent-substitution-rejected": "test_prelaunch_parent_substitution_causes_indeterminate_cleanup",
    "cleanup-precedence-over-protocol-failure": "test_cleanup_failure_takes_precedence_over_protocol_failure",
    "concurrent-owned-bundles-isolated": "test_two_sessions_remove_only_their_owned_bundles",
}
NEGATIVE_CONTROL_TESTS = tuple(CONTROL_METHODS)
_METHOD_TO_CONTROL = {method: control for control, method in CONTROL_METHODS.items()}


def _p27_success(path: Path) -> dict[str, object]:
    if path.exists():
        raise RuntimeError("P78 fake P27 root must be absent")
    path.mkdir()
    return {
        "schema": "exact-two-preflight-cycle-v1",
        "outcome": "pass",
        "pair_ids": ["preflight-pair-000", "preflight-pair-001"],
        "pair_count": 2,
        "windows_record_count": 2,
        "ubuntu_record_count": 2,
        "process_record_count": 4,
        "pair_seal_count": 2,
        "durable_write_count": 6,
        "fresh_process_reload_count": 2,
        "fresh_verifiers": {"windows": {}, "ubuntu": {}},
        "residue_count": 0,
        "retries": 0,
    }


def _fake_release(root: Path, mode: str) -> Path:
    release = root / f"fake-release-{mode}"
    (release / "fixtures").mkdir(parents=True)
    for name in ("frozen_profile_diff.py", "p31_contract_verifier.py"):
        shutil.copyfile(P51_ROOT / name, release / name)
    artifact = release / "fixtures" / "fake_ferris.py"
    artifact.write_bytes(
        (P51_ROOT / "fixtures" / "fake_ferris.py").read_bytes()
        + f"\n# pulse-78-{mode}\n".encode("ascii")
    )
    return artifact


def _controls(fake: FakeP56, p51: object) -> executor._Controls:
    return executor._Controls(
        p51,
        fake,
        _p27_success,
        lambda root, parent, api: executor._NativeWslSession(root, parent, api),
    )


class _QualificationResult(unittest.TextTestResult):
    def __init__(self, *args: object, **kwargs: object) -> None:
        super().__init__(*args, **kwargs)
        self.executed_controls: list[str] = []

    def startTest(self, test: unittest.case.TestCase) -> None:
        super().startTest(test)
        control = _METHOD_TO_CONTROL.get(test.id().rsplit(".", 1)[-1])
        if control is not None:
            self.executed_controls.append(control)


def _run_negative_controls() -> tuple[str, ...]:
    suite = unittest.defaultTestLoader.discover(str(ROOT / "tests"), pattern="test_*.py")
    result = unittest.TextTestRunner(
        stream=sys.stderr,
        verbosity=0,
        resultclass=_QualificationResult,
    ).run(suite)
    if (
        not result.wasSuccessful()
        or result.testsRun != len(CONTROL_METHODS)
        or len(result.executed_controls) != len(CONTROL_METHODS)
        or set(result.executed_controls) != set(NEGATIVE_CONTROL_TESTS)
    ):
        raise RuntimeError("P78 qualification negative-control failure")
    return tuple(result.executed_controls)


def _run_cycle(root: Path, cycle: int, p51: object) -> dict[str, object]:
    runtime_root = root / "runtime"
    runtime_root.mkdir()
    mode = "alpha" if cycle % 2 else "beta"
    artifact = _fake_release(root, mode)
    fake = FakeP56(artifact)
    manager = FakeBundleManager(executor, root / "bundles")
    process_factory = FakeWorkerProcessFactory(executor, fake, root)
    descriptor_root = load_p51_synthetic_fixture(REPO_ROOT, p51).create_descriptor_root(
        runtime_root / "descriptors"
    )
    with (
        patch.object(executor, "_stage_owned_bundle", manager.stage),
        patch.object(executor, "_revalidate_staged_bundle", manager.revalidate),
        patch.object(executor, "_cleanup_owned_bundle", manager.cleanup),
        patch.object(executor, "_spawn_wsl_worker", process_factory),
    ):
        result = executor._run_qualification_executor(
            REPO_ROOT,
            descriptor_root,
            runtime_root,
            runtime_root / "p27-cycle",
            _controls(fake, p51),
        )
    if (
        result.private_record["outcome"] != "completed"
        or result.private_record["process_counts"]
        != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
        or len(result.private_record["no_launch_records"]) != 2
        or len(fake.launches) != 138
        or len(manager.records) != 1
        or manager.records[0].revalidate_calls != 1
        or manager.records[0].cleanup_calls != 1
        or manager.residue()
        or len(process_factory.processes) != 1
        or len(process_factory.processes[0].requests) != 69
    ):
        raise RuntimeError("P78 qualification topology or stage-capture/bootstrap-argv failure")
    return {
        "cycle": cycle,
        "cycle_mode": mode,
        "fake_artifact_sha256": sha256_bytes(artifact.read_bytes()),
        "fake_launches": 138,
        "staged_identity_revalidations": manager.records[0].revalidate_calls,
        "owned_bundle_cleanup_calls": manager.records[0].cleanup_calls,
        "ordered_event_sha256": sha256_bytes(
            canonical_bytes({"catalog": result.catalog, "events": result.events})
        ),
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cycles", type=int, default=20)
    parser.add_argument("--write-receipt", action="store_true")
    args = parser.parse_args(argv)
    if args.cycles < 20:
        parser.error("--cycles must be at least 20")
    negative_control_ids = _run_negative_controls()
    if WORK_ROOT.exists():
        shutil.rmtree(WORK_ROOT)
    WORK_ROOT.mkdir(parents=True)
    _p75, _p57, p51, _p56 = load_exact_p75_stack(REPO_ROOT)
    cycles: list[dict[str, object]] = []
    try:
        for cycle in range(1, args.cycles + 1):
            cycle_root = WORK_ROOT / f"cycle-{cycle:03d}"
            cycle_root.mkdir()
            cycles.append(_run_cycle(cycle_root, cycle, p51))
    finally:
        if WORK_ROOT.exists():
            shutil.rmtree(WORK_ROOT)
    payload = {
        "schema": "ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-qualification/v1",
        "outcome": "pass",
        "cycles_required": 20,
        "cycles_run": args.cycles,
        "cycles_passed": args.cycles,
        "cycles_failed": 0,
        "fake_launches_per_platform_per_cycle": 69,
        "fake_launches_total": args.cycles * 138,
        "staged_identity_revalidation_per_cycle": 1,
        "staged_identity_revalidation_total": args.cycles,
        "owned_bundle_cleanup_per_cycle": 1,
        "owned_bundle_cleanup_total": args.cycles,
        "exact_p75_binding_verified": True,
        "local_loader_explicit_binding_verified": True,
        "fresh_module_loading_verified": True,
        "stage_identity_capture_verified": True,
        "stage_post_create_failure_cleanup_verified": True,
        "stage_create_open_substitution_indeterminate_verified": True,
        "stage_cleanup_indeterminate_precedence_verified": True,
        "prelaunch_root_substitution_rejected": True,
        "prelaunch_parent_substitution_rejected": True,
        "worker_bootstrap_ready_close_verified": True,
        "worker_dependency_loader_binding_verified": True,
        "worker_bootstrap_root_swap_rejected": True,
        "worker_bootstrap_path_swap_rejected": True,
        "bundle_retained_during_worker_lifetime_verified": True,
        "cleanup_precedence_verified": True,
        "zero_residue_after_close_verified": True,
        "ferris_executed": False,
        "negative_control_tests_run": len(negative_control_ids),
        "negative_control_tests_passed": len(negative_control_ids),
        "negative_control_test_ids": list(negative_control_ids),
        "cycles": cycles,
    }
    if args.write_receipt:
        receipt = {
            "payload": payload,
            "payload_sha256": sha256_bytes(canonical_bytes(payload)),
            "receipt_id": sha256_bytes(canonical_bytes(payload)),
            "schema": "ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-receipt/v1",
        }
        (ROOT / "qualification-receipt.json").write_bytes(canonical_bytes(receipt) + b"\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
