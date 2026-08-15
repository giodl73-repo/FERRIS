"""Run twenty or more isolated fake-only Pulse 52 qualification cycles."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent
REPO_ROOT = ROOT.parents[3]
WORK_ROOT = REPO_ROOT / "target" / "pulse-52-qualification-runtime"
sys.dont_write_bytecode = True
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import ordered_materialization_executor as executor
from sealed_dependencies import load_pulse51
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
    for release in (
        REPO_ROOT
        / "docs"
        / "simulations"
        / "profile-diff-held-out"
        / "pulse-25-collector-source-release",
        REPO_ROOT
        / "docs"
        / "simulations"
        / "profile-diff-held-out"
        / "pulse-27-preflight-adapter-release",
    ):
        for path in sorted(
            release.rglob("__pycache__"), key=lambda item: len(item.parts), reverse=True
        ):
            cleanup_synthetic_runtime_root(path)


def _run_cycle(cycle_root: Path, cycle: int) -> dict[str, object]:
    _clean_prelaunch_python_residue()
    runtime_root = cycle_root / "runtime"
    runtime_root.mkdir()
    p51 = load_pulse51(REPO_ROOT)
    custodies, expectations = create_synthetic_custodies(runtime_root, p51)
    runner = QualificationProcessRunner(runtime_root, p51)
    result = executor._run_qualification_executor(
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
    private = result.private_record
    if (
        private["outcome"] != "published"
        or private["execution_outcome"] != "completed"
        or private["publication_disposition"] != "published"
        or result.publication
        != {
            "schema": "ferris.pulse-52-terminal-publication-disposition/v1",
            "disposition": "published",
            "product_conclusion": None,
            "category_conclusion": None,
            "fix_conclusion": None,
            "posture": {
                "p43_result": "published-and-verified",
                "p47_witness": "published-and-verified",
            },
        }
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
        or private["process_counts"]
        != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
        or len(private["no_launch_records"]) != 2
        or runner.counts != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
        or len(runner.dispatches) != 138
        or private["terminal_p47_invocation_count"] != 1
        or private["terminal_publication_cleanup"] != "retained-published"
        or result.events[-1]
        != {
            "classification": "ordered-execution",
            "event_kind": "terminal-stop",
            "gate_id": "bounded-process-exit-search",
            "outcome": "completed",
            "schema": "ferris.pulse-43-ordered-result-event/v1",
        }
        or (runtime_root / executor.PRIVATE_LAUNCH_DIRECTORY).exists()
        or not (
            runtime_root
            / executor.TERMINAL_DIRECTORY
            / executor.P43_FINAL_DIRECTORY
            / "public-result.json"
        ).is_file()
        or not (
            runtime_root
            / executor.TERMINAL_DIRECTORY
            / executor.WITNESS_FINAL_DIRECTORY
            / "publication-witness.json"
        ).is_file()
    ):
        raise RuntimeError("Pulse 52 qualification topology or cleanup failed")
    public_text = _canonical_json({"catalog": result.catalog, "events": result.events}).decode(
        "ascii"
    )
    if any(value in public_text for value in ("seed", "descriptor", "private", "case_id")):
        raise RuntimeError("Pulse 52 qualification exposed private material publicly")
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
        "publication_disposition": "published",
        "terminal_p47_invocations": 1,
        "verifier_invocations": 1,
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
                raise RuntimeError("Pulse 52 qualification retained scratch")
    finally:
        if WORK_ROOT.exists():
            cleanup_synthetic_runtime_root(WORK_ROOT)
        _clean_sealed_python_residue()
        _clean_prelaunch_python_residue()
    payload = {
        "schema": "ferris.pulse-52-ordered-materialization-executor-qualification/v1",
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
        "terminal_publication_successes": arguments.cycles,
        "terminal_publication_failures": 0,
        "failure_boundary_hardening": {
            "exact_predecessor_public_failures": "bounded-prelaunch",
            "exact_terminal_publication_failures": "invalid-publication-integrity",
            "programmer_faults": "propagate",
        },
        "ferris_executed": False,
        "synthetic_scratch_cleanup": {
            "retry_delays_seconds": [0.02, 0.05, 0.10, 0.20],
            "retryable_errors": ["PermissionError", "WinError32"],
            "root_absence_verified": True,
        },
        "cycles": cycles,
    }
    if arguments.write_receipt:
        digest = "sha256:" + hashlib.sha256(_canonical_json(payload)).hexdigest()
        envelope = {
            "payload": payload,
            "payload_sha256": digest,
            "receipt_id": digest,
            "schema": "ferris.pulse-52-ordered-materialization-executor-qualification-envelope/v1",
        }
        (ROOT / "qualification-receipt.json").write_bytes(_canonical_json(envelope) + b"\n")
    print(_canonical_json(payload).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
