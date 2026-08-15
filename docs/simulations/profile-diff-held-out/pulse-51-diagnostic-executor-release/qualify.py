"""Run twenty or more isolated private synthetic Pulse 51 qualification cycles."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path


sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent
REPO_ROOT = ROOT.parents[3]
WORK_ROOT = REPO_ROOT / "target" / "pulse-51-qualification-runtime"
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from diagnostic_executor import (
    Dispatch,
    LaunchCapture,
    _canonical_json,
    _execution_event,
    _run_qualification_executor,
    resolve_python_launcher,
    windows_to_wsl_absolute,
)
from synthetic_fixture import (
    SCRATCH_CLEANUP_DELAYS,
    cleanup_synthetic_runtime_root,
    create_descriptor_root,
    create_synthetic_custodies,
)


def _clean_sealed_python_residue() -> None:
    for path in (ROOT / ".qualification-work", ROOT / "tests" / ".run"):
        if path.exists():
            cleanup_synthetic_runtime_root(path)
    for path in sorted(ROOT.rglob("__pycache__"), key=lambda value: len(value.parts), reverse=True):
        cleanup_synthetic_runtime_root(path)


class QualificationProcessRunner:
    """Asserts complete dispatch construction, then replaces only process execution."""

    def __init__(self, runtime_root: Path) -> None:
        self.runtime_root = runtime_root
        self.counts = {"windows-x86_64": 0, "ubuntu-24.04-x86_64": 0}
        self.dispatch_hashes: list[str] = []

    def __call__(self, dispatch: Dispatch) -> LaunchCapture:
        self._assert_dispatch(dispatch)
        interpreter = resolve_python_launcher(
            dispatch.platform,
            which=lambda name: sys.executable if name in {"python", "python3"} else None,
        )
        completed = subprocess.run(
            [
                *interpreter,
                str(ROOT / "fixtures" / "fake_ferris.py"),
                "--synthetic-platform",
                dispatch.platform,
                *dispatch.application_argv,
            ],
            cwd=dispatch.host_cwd,
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
        )
        self.counts[dispatch.platform] += 1
        self.dispatch_hashes.append(
            "sha256:" + hashlib.sha256(_canonical_json(list(dispatch.command))).hexdigest()
        )
        return LaunchCapture(completed.returncode, completed.stdout, completed.stderr)

    def _assert_dispatch(self, dispatch: Dispatch) -> None:
        if (
            len(dispatch.application_argv) != 7
            or dispatch.application_argv[0] != "profile-diff"
            or dispatch.application_argv[1] != "--before"
            or dispatch.application_argv[3] != "--after"
            or dispatch.application_argv[5] != "--format"
            or dispatch.application_argv[6] not in {"json", "human"}
            or dispatch.host_cwd != self.runtime_root
        ):
            raise RuntimeError("qualification did not receive exact profile-diff dispatch")
        for path in (
            dispatch.executable,
            Path(dispatch.application_argv[2]),
            Path(dispatch.application_argv[4]),
        ):
            path.relative_to(self.runtime_root)
        if dispatch.platform == "windows-x86_64":
            if dispatch.wsl_cwd is not None or dispatch.command != (
                str(dispatch.executable),
                *dispatch.application_argv,
            ):
                raise RuntimeError("qualification Windows native dispatch mismatch")
            return
        expected = (
            "wsl.exe",
            "--distribution",
            "Ubuntu-24.04",
            "--cd",
            windows_to_wsl_absolute(self.runtime_root, self.runtime_root),
            "--exec",
            windows_to_wsl_absolute(dispatch.executable, self.runtime_root),
            "profile-diff",
            "--before",
            windows_to_wsl_absolute(Path(dispatch.application_argv[2]), self.runtime_root),
            "--after",
            windows_to_wsl_absolute(Path(dispatch.application_argv[4]), self.runtime_root),
            "--format",
            dispatch.application_argv[6],
        )
        if dispatch.command != expected or dispatch.wsl_cwd != expected[4]:
            raise RuntimeError("qualification WSL dispatch or translation mismatch")


def _p27_success(path: Path) -> dict[str, object]:
    if path.exists():
        raise RuntimeError("synthetic P27 root must be absent")
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


def _run_cycle(root: Path) -> dict[str, object]:
    descriptor_root = create_descriptor_root(root / "descriptors")
    custodies, expectations = create_synthetic_custodies(
        root / "custodies", ROOT / "fixtures" / "fake_ferris.py"
    )
    runner = QualificationProcessRunner(root)
    result = _run_qualification_executor(
        REPO_ROOT,
        descriptor_root,
        root,
        root / "p27-cycle",
        custodies,
        p27_runner=_p27_success,
        process_runner=runner,
        expectations=expectations,
    )
    if (
        result.private_record["outcome"] != "completed"
        or result.private_record["process_counts"]
        != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
        or len(result.private_record["no_launch_records"]) != 2
        or not (root / "p27-cycle").is_dir()
        or result.events[-1]
        != _execution_event("bounded-process-exit-search", "terminal-stop", "completed")
    ):
        raise RuntimeError("synthetic executor topology qualification failed")
    records = result.private_record["platform_records"]
    if any(
        windows["result"]["semantic_projection_sha256"]
        != ubuntu["result"]["semantic_projection_sha256"]
        for windows, ubuntu in zip(
            records["windows-x86_64"], records["ubuntu-24.04-x86_64"], strict=True
        )
    ):
        raise RuntimeError("synthetic dispatch changed a semantic projection")
    return {
        "fake_launches": 138,
        "no_launch_dispositions": 2,
        "ordered_event_count": len(result.events),
        "dispatch_construction_count": len(runner.dispatch_hashes),
        "public_event_digest": "sha256:"
        + hashlib.sha256(_canonical_json({"catalog": result.catalog, "events": result.events})).hexdigest(),
        "semantic_projection_digest": "sha256:"
        + hashlib.sha256(
            _canonical_json(
                [
                    record["result"]["semantic_projection_sha256"]
                    for record in records["windows-x86_64"]
                ]
            )
        ).hexdigest(),
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cycles", type=int, default=20)
    parser.add_argument("--write-receipt", action="store_true")
    args = parser.parse_args(argv)
    if args.cycles < 20:
        parser.error("--cycles must be at least 20")
    _clean_sealed_python_residue()
    work_root = WORK_ROOT
    if work_root.exists():
        cleanup_synthetic_runtime_root(work_root)
    work_root.mkdir(parents=True)
    cycles: list[dict[str, object]] = []
    try:
        for cycle in range(1, args.cycles + 1):
            cycle_root = work_root / f"cycle-{cycle:03d}"
            cycle_root.mkdir()
            receipt = _run_cycle(cycle_root)
            receipt["cycle"] = cycle
            cycles.append(receipt)
            cleanup_synthetic_runtime_root(cycle_root)
        with os.scandir(work_root) as directory:
            if next(directory, None) is not None:
                raise RuntimeError("synthetic qualification residue remains")
    finally:
        if work_root.exists():
            cleanup_synthetic_runtime_root(work_root)
        _clean_sealed_python_residue()
    payload = {
        "schema": "ferris.pulse-51-diagnostic-executor-qualification/v2",
        "outcome": "pass",
        "cycles_required": 20,
        "cycles_run": args.cycles,
        "cycles_passed": args.cycles,
        "cycles_failed": 0,
        "fake_launches_per_platform_per_cycle": 69,
        "fake_launches_total": args.cycles * 138,
        "no_launch_per_platform_per_cycle": 1,
        "no_launch_total": args.cycles * 2,
        "p27_invocations_per_cycle": 1,
        "p27_successful_cycle_retention": "private-until-cycle-cleanup",
        "p43_terminal_publication_invocations": 0,
        "p44_p45_bridges_per_platform_per_cycle": 1,
        "p47_terminal_publication_invocations": 0,
        "private_seed_created": False,
        "descriptor_materializer_invocations": 0,
        "ferris_executed": False,
        "synthetic_scratch_cleanup": {
            "retry_delays_seconds": list(SCRATCH_CLEANUP_DELAYS),
            "retryable_errors": ["PermissionError", "WinError32"],
            "root_absence_verified": True,
        },
        "windows_native_dispatches_per_cycle": 69,
        "ubuntu_wsl_dispatches_per_cycle": 69,
        "wsl_distribution": "Ubuntu-24.04",
        "cycles": cycles,
    }
    if args.write_receipt:
        digest = "sha256:" + hashlib.sha256(_canonical_json(payload)).hexdigest()
        envelope = {
            "payload": payload,
            "payload_sha256": digest,
            "receipt_id": digest,
            "schema": "ferris.pulse-51-diagnostic-executor-qualification-envelope/v2",
        }
        (ROOT / "qualification-receipt.json").write_bytes(_canonical_json(envelope) + b"\n")
    print(_canonical_json(payload).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
