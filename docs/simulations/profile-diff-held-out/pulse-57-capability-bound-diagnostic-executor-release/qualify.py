#!/usr/bin/env python3
"""Run the fake-only Pulse 57 qualification cycles."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
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
WORK_ROOT = REPO_ROOT / "target" / "pulse-57-qualification-runtime"
sys.path.insert(0, str(ROOT))

from capability_bound_executor import (  # noqa: E402
    WSL_PLATFORM,
    _Controls,
    _run_qualification_executor,
)
from fixtures.fake_p56 import FakeP56  # noqa: E402
from sealed_dependencies import (  # noqa: E402
    canonical_bytes,
    load_exact_p51,
    load_p51_synthetic_fixture,
    sha256_bytes,
)
from wsl_session_worker import WorkerProtocol, _request_id  # noqa: E402


NEGATIVE_CONTROL_TESTS = (
    "capability-exhaustion",
    "final-cleanup-after-138-launches",
    "lazy-path-substitution",
    "prelaunch-semantics-substitution",
    "profile-result-mismatch",
    "worker-injection-replay-order-extra-output",
    "worker-unknown-programmer-fault-reraises",
    "worker-startup-io-orphan-termination",
    "worker-close-timeout",
    "first-stop",
    "forged-predecessor",
    "public-evidence-capability-forgery",
    "unknown-programmer-fault-reraises",
    "bound-byte-import-substitution-pycache",
    "worker-bound-byte-import-substitution-pycache",
    "dependency-failure-terminal-classification",
    "p31-failure-classification",
    "p35-p37-failure-classification",
    "p51-executor-failure-classification",
    "loaded-terminal-failure-classification",
    "worker-unknown-cleanup-precedence",
    "parent-unknown-cleanup-precedence",
)


def _run_negative_controls() -> int:
    suite = unittest.defaultTestLoader.discover(str(ROOT / "tests"), pattern="test_*.py")
    result = unittest.TextTestRunner(stream=sys.stderr, verbosity=0).run(suite)
    if not result.wasSuccessful() or result.testsRun != len(NEGATIVE_CONTROL_TESTS):
        raise RuntimeError("P57 qualification negative-control failure")
    return result.testsRun


def _p27_success(path: Path) -> dict[str, object]:
    if path.exists():
        raise RuntimeError("P57 synthetic P27 root must be absent")
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
    """Exercise the sealed worker protocol without a real WSL or FERRIS run."""

    def __init__(self, p56: FakeP56, p51: object, runtime_root: Path) -> None:
        self._protocol = WorkerProtocol(p56, runtime_root)
        self._p51 = p51
        self._runtime_root = runtime_root
        self._closed = False
        ready = json.loads(self._protocol.ready())
        if ready["count"] != 69 or ready["platform"] != WSL_PLATFORM:
            raise RuntimeError("fake worker readiness mismatch")

    def _windows_path(self, value: str) -> str:
        if not value.startswith("/mnt/c/"):
            return value
        return "C:/" + value.removeprefix("/mnt/c/")

    def launch(self, ordinal: int, arguments: tuple[str, ...]) -> object:
        translated = list(arguments)
        translated[2] = self._windows_path(translated[2])
        translated[4] = self._windows_path(translated[4])
        request_id = _request_id(ordinal, translated)
        request = {
            "arguments": translated,
            "ordinal": ordinal,
            "platform": WSL_PLATFORM,
            "request_id": request_id,
            "schema": "ferris.pulse-57-wsl-capability-session/v1",
            "type": "launch",
        }
        raw = self._protocol.consume(canonical_bytes(request) + b"\n")
        if raw is None:
            raise RuntimeError("fake worker returned no result")
        response = json.loads(raw)
        import base64

        return self._p51.LaunchCapture(
            response["returncode"],
            base64.b64decode(response["stdout_b64"]),
            base64.b64decode(response["stderr_b64"]),
        )

    def close(self) -> None:
        if not self._closed:
            self._closed = True
            self._protocol.consume(
                canonical_bytes(
                    {"schema": "ferris.pulse-57-wsl-capability-session/v1", "type": "close"}
                )
                + b"\n"
            )


def _run_cycle(root: Path, cycle: int, p51: object, fixture: object) -> dict[str, object]:
    descriptors = fixture.create_descriptor_root(root / "descriptors")
    mode = "alpha" if cycle % 2 else "beta"
    fake_release = root / f"fake-release-{mode}"
    (fake_release / "fixtures").mkdir(parents=True)
    for dependency in ("frozen_profile_diff.py", "p31_contract_verifier.py"):
        shutil.copyfile(P51_ROOT / dependency, fake_release / dependency)
    fake_artifact = fake_release / "fixtures" / "fake_ferris.py"
    fake_artifact.write_bytes(
        (P51_ROOT / "fixtures" / "fake_ferris.py").read_bytes()
        + f"\n# pulse-57-{mode}-fake-artifact\n".encode("ascii")
    )
    fake = FakeP56(fake_artifact)
    sessions: list[_InProcessWsl] = []

    def open_wsl(_repo: Path, _native_parent: str, api: object) -> _InProcessWsl:
        session = _InProcessWsl(fake, api, root)
        sessions.append(session)
        return session

    result = _run_qualification_executor(
        REPO_ROOT,
        descriptors,
        root,
        root / "p27-cycle",
        _Controls(p51, fake, _p27_success, open_wsl),
    )
    if (
        result.private_record["outcome"] != "completed"
        or result.private_record["process_counts"]
        != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
        or len(result.private_record["no_launch_records"]) != 2
        or fake.publishes != ["windows-x86_64", "ubuntu-24.04-x86_64"]
        or len(fake.launches) != 138
        or len(sessions) != 1
    ):
        raise RuntimeError("P57 fake cycle did not preserve the exact topology")
    windows = result.private_record["platform_records"]["windows-x86_64"]
    ubuntu = result.private_record["platform_records"]["ubuntu-24.04-x86_64"]
    if any(
        left["result"]["semantic_projection_sha256"]
        != right["result"]["semantic_projection_sha256"]
        for left, right in zip(windows, ubuntu, strict=True)
    ):
        raise RuntimeError("P57 fake cross-platform semantic identity mismatch")
    return {
        "cycle": cycle,
        "cycle_mode": mode,
        "fake_artifact_sha256": sha256_bytes(fake_artifact.read_bytes()),
        "fake_launches": 138,
        "no_launch_dispositions": 2,
        "event_sha256": sha256_bytes(canonical_bytes({"catalog": result.catalog, "events": result.events})),
        "projection_sha256": sha256_bytes(
            canonical_bytes([record["result"]["semantic_projection_sha256"] for record in windows])
        ),
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cycles", type=int, default=20)
    parser.add_argument("--write-receipt", action="store_true")
    args = parser.parse_args(argv)
    if args.cycles < 20:
        parser.error("--cycles must be at least 20")
    negative_controls_run = _run_negative_controls()
    if WORK_ROOT.exists():
        shutil.rmtree(WORK_ROOT)
    WORK_ROOT.mkdir(parents=True)
    cycles: list[dict[str, object]] = []
    p51 = load_exact_p51(REPO_ROOT)
    fixture = load_p51_synthetic_fixture(REPO_ROOT, p51)
    try:
        for cycle in range(1, args.cycles + 1):
            cycle_root = WORK_ROOT / f"cycle-{cycle:03d}"
            cycle_root.mkdir()
            cycles.append(_run_cycle(cycle_root, cycle, p51, fixture))
            fixture.cleanup_synthetic_runtime_root(cycle_root)
        if any(WORK_ROOT.iterdir()):
            raise RuntimeError("P57 synthetic runtime residue")
    finally:
        if WORK_ROOT.exists():
            shutil.rmtree(WORK_ROOT)
    payload = {
        "schema": "ferris.pulse-57-capability-bound-diagnostic-executor-qualification/v1",
        "outcome": "pass",
        "cycles_required": 20,
        "cycles_run": args.cycles,
        "cycles_passed": args.cycles,
        "cycles_failed": 0,
        "fake_launches_per_platform_per_cycle": 69,
        "fake_launches_total": args.cycles * 138,
        "no_launch_per_platform_per_cycle": 1,
        "no_launch_total": args.cycles * 2,
        "windows_p56_capability_publishes_per_cycle": 1,
        "ubuntu_p56_capability_publishes_per_cycle": 1,
        "ubuntu_session_requests_per_cycle": 69,
        "alternating_cycle_modes": ["alpha", "beta"],
        "p44_p45_execution_invocations": 0,
        "ferris_executed": False,
        "private_seed_created": False,
        "descriptor_materializer_invocations": 0,
        "synthetic_root_absence_verified": True,
        "negative_control_tests_run": negative_controls_run,
        "negative_control_tests_passed": negative_controls_run,
        "negative_control_test_ids": list(NEGATIVE_CONTROL_TESTS),
        "cycles": cycles,
    }
    if args.write_receipt:
        digest = sha256_bytes(canonical_bytes(payload))
        (ROOT / "qualification-receipt.json").write_bytes(
            canonical_bytes(
                {
                    "payload": payload,
                    "payload_sha256": digest,
                    "receipt_id": digest,
                    "schema": "ferris.pulse-57-capability-bound-diagnostic-executor-qualification-envelope/v1",
                }
            )
            + b"\n"
        )
    print(canonical_bytes(payload).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
