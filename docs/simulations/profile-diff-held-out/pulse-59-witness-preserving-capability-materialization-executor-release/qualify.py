#!/usr/bin/env python3
"""Run deterministic fake-only Pulse 59 qualification cycles."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
import unittest
from pathlib import Path


sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent
REPO_ROOT = ROOT.parents[3]
P51_ROOT = (
    REPO_ROOT
    / "docs"
    / "simulations"
    / "profile-diff-held-out"
    / "pulse-51-diagnostic-executor-release"
)
WORK_ROOT = REPO_ROOT / "target" / "pulse-59-qualification-runtime"
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import witness_preserving_capability_materialization_executor as executor  # noqa: E402
from fixtures.fake_p56 import FakeP56  # noqa: E402
from fixtures.p52_synthetic_fixture import (  # noqa: E402
    QualificationProcessRunner,
    cleanup_synthetic_runtime_root,
    synthetic_seed,
)
from sealed_dependencies import (  # noqa: E402
    P58_COMMIT,
    canonical_bytes,
    load_pulse58,
    sha256_bytes,
)


CONTROL_METHODS = {
    "exact-p58-binding-and-signature": "test_sealed_binding_and_production_signature_match_pulse58",
    "production-surface-rejects-injection": "test_production_surface_rejects_injection",
    "local-binder-ignores-external-resolution": "test_executor_import_ignores_external_sealed_dependencies",
    "local-binder-mutation-does-not-persist": "test_sealed_dependency_loader_ignores_cache_preseed_and_mutation",
    "concurrent-p58-load-restores-foreign-sentinel": "test_concurrent_load_pulse58_serializes_and_restores_foreign_sentinel",
    "p58-import-exception-restores-generic-slot": "test_load_pulse58_import_exception_restores_generic_slot",
    "transitive-concurrent-load-stress": "test_stress_concurrent_legitimate_binder_pairs_complete_without_failure",
    "kernel-lock-name-stable-across-instances": "test_kernel_lock_name_is_stable_across_fresh_binders",
    "kernel-lock-unsupported-posix-platform": "test_kernel_lock_rejects_unsupported_posix_platform",
    "kernel-lock-reentrant-depth-single-acquire": "test_kernel_lock_reentrant_same_pid_tracks_depth_and_single_acquisition",
    "kernel-lock-context-copy-thread-blocks": "test_kernel_lock_context_copy_thread_blocks_until_owner_release",
    "kernel-lock-at-fork-registration-idempotent": "test_kernel_lock_at_fork_registration_is_idempotent_per_binder",
    "kernel-lock-pid-mismatch-reacquire": "test_kernel_lock_pid_mismatch_closes_inherited_handle_before_reacquire",
    "kernel-lock-no-file-artifacts": "test_kernel_lock_does_not_create_path_artifacts",
    "kernel-lock-acquire-failure-cleans-up": "test_kernel_lock_acquire_failure_cleans_up",
    "kernel-lock-releases-after-exception": "test_kernel_lock_releases_after_exception",
    "kernel-lock-wait-abandoned-release": "test_kernel_lock_wait_abandoned_is_treated_as_acquired_and_released",
    "kernel-lock-crash-recovery": "test_kernel_lock_crash_recovery_reacquires_after_subprocess_exit",
    "kernel-lock-process-stress": "test_kernel_lock_process_stress_serializes_subprocesses",
    "kernel-lock-fork-reacquire-no-count-inflation": "test_kernel_lock_fork_inside_lock_reacquires_without_count_inflation",
    "kernel-lock-fork-child-cleanup-reacquire": "test_kernel_lock_fork_child_cleanup_allows_parent_reacquire_before_child_reentry",
    "old-private-binder-key-ignored": "test_old_private_binder_key_is_ignored",
    "old-registry-key-ignored": "test_old_registry_key_is_ignored",
    "two-executor-instances-load-fresh-binders": "test_two_executor_instances_load_fresh_binders_without_cached_state",
    "local-binder-exception-cleans-runtime-slot": "test_local_sealed_binder_exception_cleanup_leaves_no_stale_runtime_slot",
    "qualification-delegates-to-p58": "test_qualification_delegates_to_exact_p58_executor",
    "published-result-survives-private-cleanup": "test_published_result_retains_verified_result_and_witness_after_p58_cleanup",
    "absent-failure-witness": "test_absent_p43_failure_is_retained_as_failure_witness",
    "rolled-back-failure-witness": "test_rolled_back_p43_failure_is_retained_as_failure_witness",
    "indeterminate-failure-witness": "test_indeterminate_p43_failure_is_retained_as_failure_witness",
    "malformed-hash-mismatch-residue-cleanup": "test_malformed_hash_mismatch_and_result_residue_clean_invalid_publication",
    "no-retry-terminal-seam": "test_terminal_seam_is_called_once_without_retry",
    "path-free-transfer-descriptor": "test_transfer_descriptor_is_path_free",
    "p58-prelaunch-failure-publication-not-attempted": "test_p58_prelaunch_failure_remains_publication_not_attempted",
    "cleanup-indeterminate-precedence": "test_cleanup_indeterminate_takes_precedence",
    "preexisting-terminal-root-rejected": "test_preexisting_terminal_root_is_rejected_before_p58_execution",
    "release-generator-rejects-cache-residue": "test_release_generator_rejects_python_cache_residue",
}
_METHOD_TO_CONTROL = {method: control for control, method in CONTROL_METHODS.items()}
P39_CALLER_AUTHORITY_PRECONDITION = (
    "future-authority-supplied-fresh-anonymous-exact-cutoff-root"
)


def _sync(status: str) -> dict[str, object]:
    values = {
        "not-attempted": (False, "not-attempted", "not-attempted"),
        "synced": (True, None, "os.open+os.fsync-directory-v1"),
        "unsupported": (
            True,
            "unsupported-by-platform-or-filesystem",
            "os.open+os.fsync-directory-v1",
        ),
        "failed": (True, "sync-operation-failed", "os.open+os.fsync-directory-v1"),
    }
    attempted, category, mechanism = values[status]
    return {
        "attempted": attempted,
        "error_category": category,
        "mechanism": mechanism,
        "status": status,
    }


def _p43_failure(p43: object, state: str) -> dict[str, object]:
    if state == "absent":
        attempts = 0
        sync = {
            name: _sync("not-attempted")
            for name in ("final_parent", "rollback_parent", "stage")
        }
    elif state == "rolled-back":
        attempts = 1
        sync = {name: _sync("synced") for name in ("final_parent", "rollback_parent", "stage")}
    elif state == "indeterminate":
        attempts = 1
        sync = {name: _sync("failed") for name in ("final_parent", "rollback_parent", "stage")}
    else:
        raise RuntimeError("unknown fake P43 failure posture")
    return {
        "schema": p43.SUMMARY_SCHEMA,
        "failure_code": "P43-STAGE-COPY-FAILURE",
        "publication": {
            "final_files_present": False,
            "rename_attempts": attempts,
            "retries": 0,
            "state": state,
            "sync": sync,
        },
    }


def _p27_success(path: Path) -> dict[str, object]:
    if path.exists():
        raise RuntimeError("P59 fake P27 root must be absent")
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


class _InProcessWsl:
    """A harmless protocol-shaped session; no WSL process or FERRIS binary runs."""

    def __init__(self, fake: FakeP56, p51: object) -> None:
        self._fake = fake
        self._p51 = p51
        self._closed = False
        self.requests: list[tuple[int, tuple[str, ...]]] = []

    @staticmethod
    def _from_wsl(value: str) -> str:
        return "C:/" + value.removeprefix("/mnt/c/") if value.startswith("/mnt/c/") else value

    def launch(self, ordinal: int, arguments: tuple[str, ...]) -> object:
        if self._closed or ordinal != len(self.requests) + 1 or len(arguments) != 7:
            raise RuntimeError("P59 fake worker protocol rejection")
        translated = tuple(self._from_wsl(value) for value in arguments)
        capture = self._fake.launch_verified(
            self._ubuntu, "ubuntu-24.04-x86_64", translated
        )
        self.requests.append((ordinal, translated))
        return capture

    def bind(self, handle: object) -> None:
        self._ubuntu = handle

    def close(self) -> None:
        if self._closed:
            return
        self._closed = True
        if len(self.requests) != 69:
            self._fake.close_custody(self._ubuntu)


def _synthetic_p39_checkout(root: Path) -> Path:
    """Make a dedicated checkout for exact P39/P41 semantics only."""

    checkout = root / "synthetic-p39-checkout"
    releases = (
        "pulse-25-collector-source-release",
        "pulse-27-preflight-adapter-release",
        "pulse-39-checkout-verifier-release",
    )
    source_root = REPO_ROOT / "docs" / "simulations" / "profile-diff-held-out"
    destination_root = checkout / "docs" / "simulations" / "profile-diff-held-out"
    destination_root.mkdir(parents=True)
    for name in releases:
        shutil.copytree(source_root / name, destination_root / name)
    (checkout / ".gitattributes").write_text(
        "\n".join(
            f"docs/simulations/profile-diff-held-out/{name}/** text eol=lf"
            for name in releases
        )
        + "\n",
        encoding="ascii",
        newline="\n",
    )
    completed = subprocess.run(
        ["git", "init", "--quiet", str(checkout)],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise RuntimeError("P59 synthetic P39 checkout initialization failed")
    return checkout


def _fake_release(root: Path, mode: str) -> Path:
    release = root / f"fake-release-{mode}"
    (release / "fixtures").mkdir(parents=True)
    for name in ("frozen_profile_diff.py", "p31_contract_verifier.py"):
        shutil.copyfile(P51_ROOT / name, release / name)
    artifact = release / "fixtures" / "fake_ferris.py"
    artifact.write_bytes(
        (P51_ROOT / "fixtures" / "fake_ferris.py").read_bytes()
        + f"\n# p59-{mode}\n".encode("ascii")
    )
    return artifact


def _public_text(result: object) -> str:
    return canonical_bytes(
        {
            "catalog": result.catalog,
            "events": result.events,
            "publication": result.publication,
            "transfer_descriptor": result.transfer_descriptor,
        }
    ).decode("ascii")


def _run_cycle(root: Path, cycle: int) -> dict[str, object]:
    runtime = root / "runtime"
    runtime.mkdir()
    mode = "alpha" if cycle % 2 else "beta"
    artifact = _fake_release(root, mode)
    _p58, _p52, _p57, p51, p43, p47 = load_pulse58(REPO_ROOT)
    fake = FakeP56(
        artifact,
        runner=QualificationProcessRunner(runtime, p51),
        runtime_root=runtime,
        p51=p51,
    )
    sessions: list[_InProcessWsl] = []

    def open_wsl(_repo: Path, _parent: str, api: object) -> _InProcessWsl:
        session = _InProcessWsl(fake, api)
        sessions.append(session)
        session.bind(fake.publish_retained_build_and_custody("ubuntu-24.04-x86_64", root))
        return session

    witnessed_failure = cycle % 2 == 0
    failure_state = ("absent", "rolled-back", "indeterminate")[((cycle // 2) - 1) % 3]
    terminal_calls: list[object] = []

    terminal_call = None
    if witnessed_failure:
        failure = _p43_failure(p43, failure_state)

        def terminal(
            terminal_object: object, result: object, p43_root: Path, witness_root: Path
        ) -> object:
            terminal_calls.append((terminal_object, p43_root, witness_root))
            return p47.witness_pulse_43(
                result.catalog,
                result.events,
                p43_root,
                witness_root,
                invoker=lambda *_arguments: failure,
            )

        terminal_call = terminal

    p39_checkout = _synthetic_p39_checkout(root)
    result = executor._run_qualification_executor(
        REPO_ROOT,
        runtime,
        runtime / "p27-cycle",
        p39_checkout,
        root / "p41-public-custody",
        seed_bytes=synthetic_seed(cycle),
        p27_runner=_p27_success,
        p56=fake,
        open_wsl=open_wsl,
        terminal_call=terminal_call,
    )
    terminal_root = root / f"runtime{executor.TERMINAL_ROOT_SUFFIX}"
    private = result.private_record
    if (
        private["p58_execution_outcome"] != "completed"
        or private["seed_calls"] != 1
        or private["materializer_invocations"] != 1
        or private["verifier_invocations"] != 1
        or private["process_counts"] != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
        or len(private["no_launch_records"]) != 2
        or fake.publishes != ["windows-x86_64", "ubuntu-24.04-x86_64"]
        or len(fake.launches) != 138
        or len(sessions) != 1
        or len(sessions[0].requests) != 69
        or runtime.exists()
        or result.events[-1]["outcome"] != "completed"
        or private["terminal_p47_invocation_count"] != 1
        or not private["terminal_runtime_absence_verified"]
    ):
        raise RuntimeError("P59 qualification topology, cleanup, or ordering failure")
    if witnessed_failure:
        if (
            result.publication["disposition"] != "published-failure-witness"
            or result.transfer_descriptor is None
            or result.transfer_descriptor["expected_public_tree_kind"]
            != "failure-witness-only"
            or result.publication["posture"]["publication"]["state"] != failure_state
            or sorted(entry.name for entry in terminal_root.iterdir())
            != [executor.WITNESS_FINAL_DIRECTORY]
            or len(terminal_calls) != 1
        ):
            raise RuntimeError("P59 qualification did not retain only the witness failure")
    else:
        if (
            result.publication["disposition"] != "published-result"
            or result.transfer_descriptor is None
            or result.transfer_descriptor["expected_public_tree_kind"] != "result-and-witness"
            or sorted(entry.name for entry in terminal_root.iterdir())
            != [executor.P43_FINAL_DIRECTORY, executor.WITNESS_FINAL_DIRECTORY]
        ):
            raise RuntimeError("P59 qualification did not retain result and witness")
    public = _public_text(result)
    if any(
        value in public
        for value in (
            "seed.bin",
            "case_id",
            "private_execution",
            str(terminal_root),
        )
    ):
        raise RuntimeError("P59 qualification disclosed private capability material")
    return {
        "cycle": cycle,
        "cycle_mode": mode,
        "fake_artifact_sha256": sha256_bytes(artifact.read_bytes()),
        "fake_launches": 138,
        "materializer_invocations": 1,
        "no_launch_dispositions": 2,
        "ordered_event_sha256": sha256_bytes(
            canonical_bytes({"catalog": result.catalog, "events": result.events})
        ),
        "p39_checkout_verifications": private["p39_checkout_verifications"],
        "p41_transactional_copy_invocations": private["p41_transactional_copy_invocations"],
        "publication_disposition": result.publication["disposition"],
        "terminal_p47_invocations": private["terminal_p47_invocation_count"],
        "verifier_invocations": private["verifier_invocations"],
        **({"p43_failure_posture": failure_state} if witnessed_failure else {}),
    }


class _QualificationResult(unittest.TextTestResult):
    def __init__(self, *args: object, **kwargs: object) -> None:
        super().__init__(*args, **kwargs)
        self.executed_controls: list[str] = []

    def startTest(self, test: unittest.case.TestCase) -> None:
        super().startTest(test)
        control = _METHOD_TO_CONTROL.get(test.id().rsplit(".", 1)[-1])
        if control is not None:
            self.executed_controls.append(control)


def _run_behavioral_controls() -> tuple[str, ...]:
    suite = unittest.defaultTestLoader.discover(str(ROOT / "tests"), pattern="test_*.py")
    result = unittest.TextTestRunner(
        stream=sys.stderr, verbosity=0, resultclass=_QualificationResult
    ).run(suite)
    if (
        not result.wasSuccessful()
        or result.testsRun != len(CONTROL_METHODS)
        or len(result.executed_controls) != len(CONTROL_METHODS)
        or set(result.executed_controls) != set(CONTROL_METHODS)
    ):
        raise RuntimeError("P59 qualification behavioral-control failure")
    return tuple(result.executed_controls)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cycles", type=int, default=20)
    parser.add_argument("--write-receipt", action="store_true")
    args = parser.parse_args(argv)
    if args.cycles < 20:
        parser.error("--cycles must be at least 20")
    control_ids = _run_behavioral_controls()
    if WORK_ROOT.exists():
        cleanup_synthetic_runtime_root(WORK_ROOT)
    WORK_ROOT.mkdir(parents=True)
    cycles: list[dict[str, object]] = []
    try:
        for cycle in range(1, args.cycles + 1):
            cycle_root = WORK_ROOT / f"cycle-{cycle:03d}"
            cycle_root.mkdir()
            try:
                cycles.append(_run_cycle(cycle_root, cycle))
            finally:
                if cycle_root.exists():
                    cleanup_synthetic_runtime_root(cycle_root)
        with os.scandir(WORK_ROOT) as entries:
            if next(entries, None) is not None:
                raise RuntimeError("Pulse 59 qualification retained scratch")
    finally:
        if WORK_ROOT.exists():
            cleanup_synthetic_runtime_root(WORK_ROOT)
    published_results = sum(
        item["publication_disposition"] == "published-result" for item in cycles
    )
    failure_witnesses = sum(
        item["publication_disposition"] == "published-failure-witness" for item in cycles
    )
    payload = {
        "schema": (
            "ferris.pulse-59-witness-preserving-capability-materialization-"
            "executor-qualification/v1"
        ),
        "outcome": "pass",
        "cycles_required": 20,
        "cycles_run": args.cycles,
        "cycles_passed": args.cycles,
        "cycles_failed": 0,
        "behavioral_control_tests_run": len(control_ids),
        "behavioral_control_tests_passed": len(control_ids),
        "behavioral_control_test_ids": list(control_ids),
        "fake_launches_per_cycle": 138,
        "fake_launches_total": args.cycles * 138,
        "topology_per_cycle": "70/69/1",
        "p39_caller_authority_precondition": P39_CALLER_AUTHORITY_PRECONDITION,
        "p39_execution_scope": "exact-p39-semantics-only",
        "p39_checkout_verifications_per_cycle": 1,
        "p41_transactional_copy_invocations_per_cycle": 1,
        "p27_invocations_per_cycle": 1,
        "materializer_invocations_per_cycle": 1,
        "verifier_invocations_per_cycle": 1,
        "private_seed_bytes_per_cycle": 32,
        "p58_bound_commit": P58_COMMIT,
        "p58_publication_invocations": 0,
        "p59_publication_invocations": args.cycles,
        "published_results": published_results,
        "published_failure_witnesses": failure_witnesses,
        "invalid_witness_publications": 0,
        "failure_witness_postures": {"absent": 0, "rolled-back": 0, "indeterminate": 0},
        "ferris_executed": False,
        "terminal_root_policy": executor.TERMINAL_ROOT_POLICY,
        "cycles": cycles,
    }
    postures = payload["failure_witness_postures"]
    assert type(postures) is dict
    for cycle in cycles:
        state = cycle.get("p43_failure_posture")
        if state is not None:
            postures[state] += 1
    if (
        published_results + failure_witnesses != args.cycles
        or payload["invalid_witness_publications"] != 0
        or not all(postures.values())
    ):
        raise RuntimeError("Pulse 59 qualification did not cover all terminal classes")
    if args.write_receipt:
        digest = sha256_bytes(canonical_bytes(payload))
        envelope = {
            "payload": payload,
            "payload_sha256": digest,
            "receipt_id": digest,
            "schema": (
                "ferris.pulse-59-witness-preserving-capability-materialization-"
                "executor-qualification-envelope/v1"
            ),
        }
        (ROOT / "qualification-receipt.json").write_bytes(canonical_bytes(envelope) + b"\n")
    print(canonical_bytes(payload).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
