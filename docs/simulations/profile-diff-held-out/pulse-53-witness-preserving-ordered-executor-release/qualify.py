"""Run deterministic fake-only Pulse 53 qualification cycles."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import sys
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).resolve().parent
REPO_ROOT = ROOT.parents[3]
WORK_ROOT = REPO_ROOT / "target" / "pulse-53-qualification-runtime"
sys.dont_write_bytecode = True
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import witness_preserving_ordered_executor as executor
from sealed_dependencies import load_pulse52
from synthetic_fixture import (
    QualificationProcessRunner,
    cleanup_synthetic_runtime_root,
    create_synthetic_custodies,
    synthetic_seed,
)


def _canonical_json(value: object) -> bytes:
    return json.dumps(value, ensure_ascii=True, separators=(",", ":"), sort_keys=True).encode(
        "ascii"
    )


def _clean_sealed_python_residue() -> None:
    for path in sorted(ROOT.rglob("__pycache__"), key=lambda item: len(item.parts), reverse=True):
        cleanup_synthetic_runtime_root(path)


def _clean_prelaunch_python_residue() -> None:
    for name in (
        "pulse-25-collector-source-release",
        "pulse-27-preflight-adapter-release",
    ):
        release = REPO_ROOT / "docs" / "simulations" / "profile-diff-held-out" / name
        for path in sorted(release.rglob("__pycache__"), key=lambda item: len(item.parts), reverse=True):
            cleanup_synthetic_runtime_root(path)


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
        sync = {name: _sync("not-attempted") for name in ("final_parent", "rollback_parent", "stage")}
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


def _run_cycle(cycle_root: Path, cycle: int) -> dict[str, object]:
    runtime_root = cycle_root / "runtime"
    runtime_root.mkdir()
    p52, p51 = load_pulse52(REPO_ROOT)
    p43, _p45, p47 = p51.load_terminal_dependencies(REPO_ROOT)
    _clean_prelaunch_python_residue()
    custodies, expectations = create_synthetic_custodies(runtime_root, p51)
    runner = QualificationProcessRunner(runtime_root, p51)
    witnessed_failure = cycle % 2 == 0
    failure_state = ("absent", "rolled-back", "indeterminate")[(cycle // 2 - 1) % 3]

    def call() -> object:
        return executor._run_qualification_executor(
            REPO_ROOT,
            runtime_root,
            runtime_root / "p27-cycle",
            REPO_ROOT,
            cycle_root / "p41-public-custody",
            custodies,
            seed_bytes=synthetic_seed(cycle),
            process_runner=runner,
            expectations=expectations,
        )

    if witnessed_failure:
        failure = _p43_failure(p43, failure_state)
        calls: list[object] = []

        def terminal(_terminal: object, result: object, p43_root: Path, witness_root: Path):
            calls.append(_terminal)
            return p47.witness_pulse_43(
                result.catalog,
                result.events,
                p43_root,
                witness_root,
                invoker=lambda *_arguments: failure,
            )

        with mock.patch.object(p51, "invoke_terminal_pulse47_once", new=terminal):
            result = call()
        if len(calls) != 1:
            raise RuntimeError("Pulse 53 retried or skipped the P47 witness seam")
        expected_disposition = "published-failure-witness"
        expected_kind = "failure-witness-only"
        expected_counts = {"witness": 2, "total": 2}
        terminal_entries = [executor.WITNESS_FINAL_DIRECTORY]
        if (
            result.publication["posture"].get("source") != "pulse-43"
            or result.publication["posture"]["publication"]["state"] != failure_state
            or (runtime_root / executor.TERMINAL_DIRECTORY / executor.P43_FINAL_DIRECTORY).exists()
        ):
            raise RuntimeError("Pulse 53 did not retain only the valid P47 failure witness")
    else:
        result = call()
        expected_disposition = "published-result"
        expected_kind = "result-and-witness"
        expected_counts = {"result": 2, "witness": 2, "total": 4}
        terminal_entries = [executor.P43_FINAL_DIRECTORY, executor.WITNESS_FINAL_DIRECTORY]

    private = result.private_record
    terminal_root = runtime_root / executor.TERMINAL_DIRECTORY
    if (
        private["execution_outcome"] != "completed"
        or private["publication_disposition"] != expected_disposition
        or result.publication["disposition"] != expected_disposition
        or result.publication["product_conclusion"] is not None
        or result.publication["category_conclusion"] is not None
        or result.publication["fix_conclusion"] is not None
        or result.transfer_descriptor is None
        or result.transfer_descriptor["expected_public_tree_kind"] != expected_kind
        or result.transfer_descriptor["exact_file_counts"] != expected_counts
        or private["private_launch_started"] is not True
        or private["prelaunch_private_namespace_absence_checks"] != 7
        or private["p39_checkout_verifications"] != 1
        or private["p41_transactional_copy_invocations"] != 1
        or private["p41_post_copy_binding"] != "8/8"
        or private["p27_invocations"] != 1
        or private["materializer_invocations"] != 1
        or private["verifier_invocations"] != 1
        or private["seed_byte_count"] != 32
        or private["seed_cleanup"] != "removed-after-verification"
        or private["descriptor_cleanup"] != "removed"
        or private["process_counts"] != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
        or len(private["no_launch_records"]) != 2
        or runner.counts != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
        or len(runner.dispatches) != 138
        or private["terminal_p47_invocation_count"] != 1
        or result.events[-1]
        != {
            "classification": "ordered-execution",
            "event_kind": "terminal-stop",
            "gate_id": "bounded-process-exit-search",
            "outcome": "completed",
            "schema": "ferris.pulse-43-ordered-result-event/v1",
        }
        or (runtime_root / executor.PRIVATE_LAUNCH_DIRECTORY).exists()
        or not terminal_root.is_dir()
        or sorted(entry.name for entry in terminal_root.iterdir()) != terminal_entries
    ):
        raise RuntimeError("Pulse 53 qualification topology, terminal class, or cleanup failed")

    public_text = _canonical_json(
        {
            "catalog": result.catalog,
            "events": result.events,
            "publication": result.publication,
            "transfer_descriptor": result.transfer_descriptor,
        }
    ).decode("ascii")
    if any(value in public_text for value in ("seed", "private-launch", "p27-cycle", str(terminal_root))):
        raise RuntimeError("Pulse 53 qualification exposed private material publicly")
    return {
        "cycle": cycle,
        "fake_dispatches": 138,
        "materializer_invocations": 1,
        "no_launch_dispositions": 2,
        "ordered_event_count": len(result.events),
        "p39_checkout_verifications": 1,
        "p41_post_copy_binding": "8/8",
        "p41_transactional_copy_invocations": 1,
        "p27_invocations": 1,
        "prelaunch_absence_checks": 7,
        "publication_disposition": expected_disposition,
        "terminal_p47_invocations": 1,
        "verifier_invocations": 1,
        **({"p43_failure_posture": failure_state} if witnessed_failure else {}),
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cycles", type=int, default=20)
    parser.add_argument("--write-receipt", action="store_true")
    arguments = parser.parse_args(argv)
    if arguments.cycles < 20:
        parser.error("--cycles must be at least 20")
    _clean_sealed_python_residue()
    _clean_prelaunch_python_residue()
    if WORK_ROOT.exists():
        cleanup_synthetic_runtime_root(WORK_ROOT)
    WORK_ROOT.mkdir(parents=True)
    cycles: list[dict[str, object]] = []
    try:
        for cycle in range(1, arguments.cycles + 1):
            cycle_root = WORK_ROOT / f"cycle-{cycle:03d}"
            cycle_root.mkdir()
            try:
                cycles.append(_run_cycle(cycle_root, cycle))
            finally:
                if cycle_root.exists():
                    cleanup_synthetic_runtime_root(cycle_root)
        with os.scandir(WORK_ROOT) as entries:
            if next(entries, None) is not None:
                raise RuntimeError("Pulse 53 qualification retained scratch")
    finally:
        if WORK_ROOT.exists():
            cleanup_synthetic_runtime_root(WORK_ROOT)
        _clean_sealed_python_residue()
        _clean_prelaunch_python_residue()
    published_results = sum(item["publication_disposition"] == "published-result" for item in cycles)
    failure_witnesses = sum(
        item["publication_disposition"] == "published-failure-witness" for item in cycles
    )
    payload = {
        "schema": "ferris.pulse-53-witness-preserving-ordered-executor-qualification/v1",
        "outcome": "pass",
        "cycles_required": 20,
        "cycles_run": arguments.cycles,
        "cycles_passed": arguments.cycles,
        "cycles_failed": 0,
        "fake_dispatches_per_cycle": 138,
        "fake_dispatches_total": arguments.cycles * 138,
        "topology_per_cycle": "70/69/1",
        "p39_checkout_verifications_per_cycle": 1,
        "p41_transactional_copy_invocations_per_cycle": 1,
        "p41_post_copy_binding_per_cycle": "8/8",
        "p27_invocations_per_cycle": 1,
        "materializer_invocations_per_cycle": 1,
        "verifier_invocations_per_cycle": 1,
        "private_seed_bytes_per_cycle": 32,
        "seed_values_disclosed": False,
        "descriptor_paths_disclosed": False,
        "prelaunch_private_namespace_absence_checks_per_cycle": 7,
        "p43_terminal_publication_invocations": arguments.cycles,
        "p47_terminal_publication_invocations": arguments.cycles,
        "published_results": published_results,
        "published_failure_witnesses": failure_witnesses,
        "invalid_witness_publications": 0,
        "failure_witness_postures": {"absent": 0, "rolled-back": 0, "indeterminate": 0},
        "ferris_executed": False,
        "failure_boundary_hardening": {
            "exact_predecessor_public_failures": "bounded-prelaunch",
            "p47_witness_of_exact_p43_failure": "published-failure-witness-retained",
            "invalid_witness_publication": "bounded-verified-cleanup",
            "programmer_faults": "propagate",
        },
        "synthetic_scratch_cleanup": {
            "retry_delays_seconds": [0.02, 0.05, 0.1, 0.2],
            "retryable_errors": ["PermissionError", "WinError32"],
            "root_absence_verified": True,
        },
        "cycles": cycles,
    }
    postures = payload["failure_witness_postures"]
    assert type(postures) is dict
    for cycle in cycles:
        state = cycle.get("p43_failure_posture")
        if state is not None:
            postures[state] += 1
    if published_results + failure_witnesses != arguments.cycles or not all(postures.values()):
        raise RuntimeError("Pulse 53 qualification did not cover all terminal classes")
    if arguments.write_receipt:
        digest = "sha256:" + hashlib.sha256(_canonical_json(payload)).hexdigest()
        envelope = {
            "payload": payload,
            "payload_sha256": digest,
            "receipt_id": digest,
            "schema": "ferris.pulse-53-witness-preserving-ordered-executor-qualification-envelope/v1",
        }
        (ROOT / "qualification-receipt.json").write_bytes(_canonical_json(envelope) + b"\n")
    print(_canonical_json(payload).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
