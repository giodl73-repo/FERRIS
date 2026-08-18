#!/usr/bin/env python3
"""Run Pulse 87's sealed fake-only qualification cycles."""
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
PROFILE_DIFF_ROOT = REPO_ROOT / "docs" / "simulations" / "profile-diff-held-out"
for cache in PROFILE_DIFF_ROOT.rglob("__pycache__"):
    shutil.rmtree(cache, ignore_errors=True)
P51_ROOT = PROFILE_DIFF_ROOT / "pulse-51-diagnostic-executor-release"
WORK_ROOT = REPO_ROOT / "target" / "pulse-87-qualification-runtime"
EXACT_REPO_ROOT = REPO_ROOT / "target" / "pulse-87-exact-p35-release-tree-root"
EXACT_RELEASE_DIRECTORIES = (
    "pulse-35-corpus-materializer-release",
    "pulse-39-checkout-verifier-release",
    "pulse-41-transactional-copy-release",
    "pulse-43-ordered-result-publisher-release",
    "pulse-45-binary-custody-event-bridge-release",
    "pulse-47-publication-outcome-witness-release",
    "pulse-51-diagnostic-executor-release",
    "pulse-52-ordered-materialization-executor-release",
    "pulse-56-retained-build-custody-release",
    "pulse-57-capability-bound-diagnostic-executor-release",
    "pulse-69-capability-bound-diagnostic-executor-successor-release",
    "pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-release",
    "pulse-75-capability-bound-diagnostic-executor-stage-bootstrap-worker-identity-successor-release",
    "pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor-release",
    "pulse-86-wsl-parent-owner-binding-capability-executor-successor-release",
)
sys.path.insert(0, str(ROOT))

import ordered_capability_materialization_executor as executor  # noqa: E402
from fixtures.fake_p56 import FakeP56  # noqa: E402
from fixtures.p52_synthetic_fixture import (  # noqa: E402
    QualificationProcessRunner,
    cleanup_synthetic_runtime_root,
    synthetic_seed,
)
from sealed_dependencies import canonical_bytes, load_exact_p86_stack, sha256_bytes  # noqa: E402


executor._bind_local_sealed_lock_manager_module(sys.modules["sealed_dependencies"])


CONTROL_METHODS = {
    "production-surface-rejects-injection": "test_production_surface_rejects_injection",
    "exact-p86-binding-and-local-loader": "test_exact_p86_binding_and_local_loader",
    "ordered-terminal-preserves-p86-stage-indeterminate": "test_ordered_terminal_preserves_p86_stage_indeterminate",
    "p35-old-alternate-digest-rejected": "test_p35_source_matching_historical_alternate_digest_is_rejected",
    "p35-receipt-tamper-rejected": "test_p35_receipt_tamper_is_rejected",
    "p35-seal-tamper-rejected": "test_p35_seal_tamper_is_rejected",
    "p35-extra-tree-file-rejected": "test_p35_extra_tree_file_is_rejected",
    "local-loader-fresh-modules": "test_local_loader_ignores_ambient_state_and_returns_fresh_modules",
    "concurrent-100-complete-load-graph-serialized": "test_concurrent_100_exact_p86_loads_are_serialized",
    "process-stress-complete-load-graph-serialized": "test_kernel_lock_process_stress_serializes_subprocesses",
    "p39-failure-terminal-cleanup": "test_p39_failure_is_terminal_and_cleaned",
    "p41-failure-terminal-cleanup": "test_p41_failure_is_terminal_and_cleaned",
    "seed-zero-public-failure": "test_public_failure_has_zero_seed",
    "p27-bounded-failure-cleanup": "test_p27_bounded_failure_is_terminal_and_cleaned",
    "unknown-fault-cleanup-reraise": "test_unknown_fault_reraises_after_cleanup",
    "unknown-fault-cleanup-indeterminate": "test_unknown_fault_cleanup_indeterminate_wins",
    "single-seed-per-cycle": "test_second_seed_is_prohibited_behaviorally",
    "single-materialization-per-cycle": "test_second_materialization_is_prohibited_behaviorally",
    "capability-built-before-seed": "test_capabilities_are_not_rebuilt_after_seed",
    "ordinal-69-ubuntu-failure-cleanup": "test_ordinal_69_ubuntu_failure_preserves_failure_and_windows_expiry",
    "ordinal-69-semantic-mismatch-cleanup": "test_ordinal_69_semantic_mismatch_preserves_failure_and_windows_expiry",
    "directory-symlink-and-wsl-no-follow": "test_directory_symlink_and_wsl_no_follow_are_rejected",
    "directory-substitution-rejected": "test_directory_substitution_race_is_rejected",
    "worker-protocol-replay-rejected": "test_worker_replay_is_rejected",
    "first-semantic-mismatch-stop": "test_first_semantic_mismatch_stops_execution",
    "no-launch-topology": "test_no_launch_topology_is_recorded",
    "synthetic-p39-root-only": "test_qualification_uses_synthetic_p39_root",
    "final-cleanup": "test_final_cleanup_removes_private_runtime",
    "release-generator-rejects-cache-residue": "test_release_generator_rejects_python_cache_residue",
}
NEGATIVE_CONTROL_TESTS = tuple(CONTROL_METHODS)
_METHOD_TO_CONTROL = {method: control for control, method in CONTROL_METHODS.items()}


def _p27_success(path: Path) -> dict[str, object]:
    if path.exists():
        raise RuntimeError("P58 fake P27 root must be absent")
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
            raise RuntimeError("P58 fake worker protocol rejection")
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
    source_root = PROFILE_DIFF_ROOT
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
        raise RuntimeError("P58 synthetic P39 checkout initialization failed")
    return checkout


def _normalize_exact_p35_release(root: Path) -> None:
    manifest_path = root / "public-manifest.json"
    manifest = json.loads(manifest_path.read_bytes())
    if manifest.get("file_count") != 8:
        raise RuntimeError("P87 exact Pulse 35 manifest file count mismatch")
    for entry in manifest["files"]:
        path = root.joinpath(*str(entry["path"]).split("/"))
        expected_size = int(entry["size"])
        expected_sha256 = str(entry["sha256"])
        content = path.read_bytes()
        if sha256_bytes(content) == expected_sha256 and len(content) == expected_size:
            continue
        normalized = content.replace(b"\r\n", b"\n")
        if (
            normalized == content
            or sha256_bytes(normalized) != expected_sha256
            or len(normalized) != expected_size
        ):
            raise RuntimeError(
                f"P87 exact Pulse 35 normalization failed for {path.name}"
            )
        path.write_bytes(normalized)
    if sum(1 for path in root.rglob("*") if path.is_file()) != 10:
        raise RuntimeError("P87 exact Pulse 35 release tree file count mismatch")


def _build_exact_repo_root(destination: Path) -> Path:
    if destination.exists():
        shutil.rmtree(destination, ignore_errors=True)
    profile_root = destination / "docs" / "simulations" / "profile-diff-held-out"
    profile_root.parent.mkdir(parents=True)
    shutil.copytree(PROFILE_DIFF_ROOT, profile_root)
    _normalize_exact_p35_release(profile_root / "pulse-35-corpus-materializer-release")
    return destination


def exact_repo_root() -> Path:
    profile_root = EXACT_REPO_ROOT / "docs" / "simulations" / "profile-diff-held-out"
    pulse35_root = (
        EXACT_REPO_ROOT
        / "docs"
        / "simulations"
        / "profile-diff-held-out"
        / "pulse-35-corpus-materializer-release"
    )
    if (
        not pulse35_root.exists()
        or any(not (profile_root / name).exists() for name in EXACT_RELEASE_DIRECTORIES)
        or not (profile_root / "INPUT_PROFILE_EVIDENCE.md").exists()
    ):
        _build_exact_repo_root(EXACT_REPO_ROOT)
    return EXACT_REPO_ROOT


def copy_exact_repo_root(destination: Path) -> Path:
    return _build_exact_repo_root(destination)


def _fake_release(root: Path, mode: str) -> Path:
    release = root / f"fake-release-{mode}"
    (release / "fixtures").mkdir(parents=True)
    for name in ("frozen_profile_diff.py", "p31_contract_verifier.py"):
        shutil.copyfile(P51_ROOT / name, release / name)
    artifact = release / "fixtures" / "fake_ferris.py"
    artifact.write_bytes(
        (P51_ROOT / "fixtures" / "fake_ferris.py").read_bytes()
        + f"\n# p87-{mode}\n".encode("ascii")
    )
    return artifact


def _run_cycle(root: Path, cycle: int) -> dict[str, object]:
    runtime = root / "runtime"
    runtime.mkdir()
    mode = "alpha" if cycle % 2 else "beta"
    artifact = _fake_release(root, mode)
    repo_root = exact_repo_root()
    _p86, p51, _p56 = load_exact_p86_stack(repo_root)
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

    p39_checkout_root = _synthetic_p39_checkout(root)
    result = executor._run_qualification_executor(
        repo_root,
        runtime,
        runtime / "p27-cycle",
        p39_checkout_root,
        root / "p41-public-custody",
        seed_bytes=synthetic_seed(cycle),
        p27_runner=_p27_success,
        p56=fake,
        open_wsl=open_wsl,
    )
    private = result.private_record
    if (
        private["outcome"] != "completed"
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
    ):
        raise RuntimeError("P58 qualification topology, cleanup, or ordering failure")
    public = canonical_bytes({"catalog": result.catalog, "events": result.events}).decode("ascii")
    if any(value in public for value in ("seed.bin", "case_id", "private_execution", "receipt_payload")):
        raise RuntimeError("P58 qualification disclosed private capability material")
    return {
        "cycle": cycle,
        "cycle_mode": mode,
        "fake_artifact_sha256": sha256_bytes(artifact.read_bytes()),
        "fake_launches": 138,
        "no_launch_dispositions": 2,
        "p39_checkout_verifications": private["p39_checkout_verifications"],
        "p41_transactional_copy_invocations": private["p41_transactional_copy_invocations"],
        "materializer_invocations": private["materializer_invocations"],
        "verifier_invocations": private["verifier_invocations"],
        "ordered_event_sha256": sha256_bytes(canonical_bytes({"catalog": result.catalog, "events": result.events})),
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


def _run_negative_controls() -> tuple[str, ...]:
    suite = unittest.defaultTestLoader.discover(str(ROOT / "tests"), pattern="test_*.py")
    result = unittest.TextTestRunner(
        stream=sys.stderr, verbosity=0, resultclass=_QualificationResult
    ).run(suite)
    if (
        not result.wasSuccessful()
        or result.testsRun != len(CONTROL_METHODS)
        or len(result.executed_controls) != len(CONTROL_METHODS)
        or set(result.executed_controls) != set(NEGATIVE_CONTROL_TESTS)
    ):
        raise RuntimeError("P58 qualification negative-control failure")
    return tuple(result.executed_controls)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cycles", type=int, default=20)
    parser.add_argument("--write-receipt", action="store_true")
    args = parser.parse_args(argv)
    if args.cycles < 20:
        parser.error("--cycles must be at least 20")
    negative_control_ids = _run_negative_controls()
    if WORK_ROOT.exists():
        cleanup_synthetic_runtime_root(WORK_ROOT)
    WORK_ROOT.mkdir(parents=True)
    cycles: list[dict[str, object]] = []
    try:
        for cycle in range(1, args.cycles + 1):
            root = WORK_ROOT / f"cycle-{cycle:03d}"
            root.mkdir()
            try:
                cycles.append(_run_cycle(root, cycle))
            finally:
                if root.exists():
                    cleanup_synthetic_runtime_root(root)
        if any(WORK_ROOT.iterdir()):
            raise RuntimeError("P58 qualification residue")
    finally:
        if WORK_ROOT.exists():
            cleanup_synthetic_runtime_root(WORK_ROOT)
    payload = {
        "schema": "ferris.pulse-87-ordered-capability-materialization-executor-wsl-parent-owner-binding-successor-qualification/v1",
        "outcome": "pass",
        "cycles_required": 20,
        "cycles_run": args.cycles,
        "cycles_passed": args.cycles,
        "cycles_failed": 0,
        "fake_launches_per_cycle": 138,
        "fake_launches_total": args.cycles * 138,
        "topology_per_cycle": "70/69/1",
        "p39_checkout_verifications_per_cycle": 1,
        "p41_transactional_copy_invocations_per_cycle": 1,
        "materializer_invocations_per_cycle": 1,
        "verifier_invocations_per_cycle": 1,
        "seed_calls_per_cycle": 1,
        "private_seed_bytes_per_cycle": 32,
        "capability_publishes_per_cycle": 2,
        "exact_p86_binding_verified": True,
        "ordered_terminal_preserves_p86_stage_indeterminate": True,
        "exact_p35_complete_release_tree_verified": True,
        "local_loader_explicit_binding_verified": True,
        "fresh_module_loading_verified": True,
        "transitive_sealed_loading_serialization_verified": True,
        "kernel_lock_cross_process_serialization_verified": True,
        "p44_p45_execution_invocations": 0,
        "publication_invocations": 0,
        "ferris_executed": False,
        "negative_control_tests_run": len(negative_control_ids),
        "negative_control_tests_passed": len(negative_control_ids),
        "negative_control_test_ids": list(negative_control_ids),
        "p39_caller_authority_precondition": executor.P39_CALLER_AUTHORITY_PRECONDITION,
        "p39_execution_scope": "exact-p39-semantics-only",
        "private_material_disclosed": False,
        "cycles": cycles,
    }
    if args.write_receipt:
        digest = sha256_bytes(canonical_bytes(payload))
        envelope = {
            "payload": payload,
            "payload_sha256": digest,
            "receipt_id": digest,
            "schema": "ferris.pulse-87-ordered-capability-materialization-executor-wsl-parent-owner-binding-successor-qualification-envelope/v1",
        }
        (ROOT / "qualification-receipt.json").write_bytes(canonical_bytes(envelope) + b"\n")
    print(canonical_bytes(payload).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
