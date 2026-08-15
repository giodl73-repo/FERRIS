from __future__ import annotations

import base64
import inspect
import io
import json
import shutil
import subprocess
import sys
import unittest
import uuid
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = ROOT.parents[3]
P51_ROOT = (
    REPO_ROOT
    / "docs"
    / "simulations"
    / "profile-diff-held-out"
    / "pulse-51-diagnostic-executor-release"
)
sys.dont_write_bytecode = True
sys.path.insert(0, str(ROOT))

import capability_bound_executor as executor  # noqa: E402
import sealed_dependencies as sealed  # noqa: E402
import wsl_session_worker as worker  # noqa: E402
from fixtures.fake_p56 import FakeP56  # noqa: E402
from sealed_dependencies import (  # noqa: E402
    P56,
    SealedDependencyFailure,
    load_exact_p51,
    load_exact_p56,
    load_p51_synthetic_fixture,
    verify_release,
)
from wsl_session_worker import PLATFORM, SCHEMA, WorkerFailure, WorkerProtocol, _line_value, _request_id  # noqa: E402


def p27_success(path: Path) -> dict[str, object]:
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


class InProcessSession:
    """Test-only protocol endpoint; production cannot supply this seam."""

    def __init__(self, fake: FakeP56, p51: object, root: Path, *, fail_close: bool = False) -> None:
        self.protocol = WorkerProtocol(fake, root)
        self.p51 = p51
        self.closed = False
        self.fail_close = fail_close

    @staticmethod
    def _from_wsl(value: str) -> str:
        if value.startswith("/mnt/c/"):
            return "C:/" + value.removeprefix("/mnt/c/")
        return value

    def launch(self, ordinal: int, arguments: tuple[str, ...]) -> object:
        converted = [self._from_wsl(value) for value in arguments]
        request = {
            "arguments": converted,
            "ordinal": ordinal,
            "platform": PLATFORM,
            "request_id": _request_id(ordinal, converted),
            "schema": SCHEMA,
            "type": "launch",
        }
        raw = self.protocol.consume(
            json.dumps(request, separators=(",", ":"), sort_keys=True).encode("ascii") + b"\n"
        )
        assert raw is not None
        response = json.loads(raw)
        return self.p51.LaunchCapture(
            response["returncode"],
            base64.b64decode(response["stdout_b64"]),
            base64.b64decode(response["stderr_b64"]),
        )

    def close(self) -> None:
        if self.closed:
            return
        self.closed = True
        self.protocol.consume(
            b'{"schema":"ferris.pulse-57-wsl-capability-session/v1","type":"close"}\n'
        )
        if self.fail_close:
            raise executor.ExecutorFailure("P57-WSL-CLEANUP")


class _StartupIoProcess:
    def __init__(self) -> None:
        self.stdin = io.BytesIO()
        self.stdout = self
        self.stderr = io.BytesIO()
        self.terminated = False

    def readline(self, _maximum: int) -> bytes:
        raise OSError("startup read failed")

    def read(self, _maximum: int) -> bytes:
        return b""

    def wait(self, timeout: int) -> int:
        if not self.terminated:
            raise subprocess.TimeoutExpired("worker", timeout)
        return -15

    def terminate(self) -> None:
        self.terminated = True

    def kill(self) -> None:
        self.terminated = True


class _CloseTimeoutProcess:
    def __init__(self, ready: bytes) -> None:
        self.stdin = io.BytesIO()
        self.stdout = io.BytesIO(ready)
        self.stderr = io.BytesIO()
        self.terminated = False
        self.killed = False

    def wait(self, timeout: int) -> int:
        if self.killed:
            return -9
        raise subprocess.TimeoutExpired("worker", timeout)

    def terminate(self) -> None:
        self.terminated = True

    def kill(self) -> None:
        self.killed = True


class CapabilityBoundExecutorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.work = REPO_ROOT / "target" / f"pulse-57-test-{uuid.uuid4().hex}"
        self.work.mkdir(parents=True)
        self.addCleanup(lambda: shutil.rmtree(self.work) if self.work.exists() else None)

    def _controls(
        self, fake: FakeP56, p51: object, *, fail_close: bool = False
    ) -> executor._Controls:
        def open_wsl(_repo: Path, _parent: str, api: object) -> InProcessSession:
            return InProcessSession(fake, api, self.work, fail_close=fail_close)

        return executor._Controls(p51, fake, p27_success, open_wsl)

    def _fixture_root(self, p51: object) -> Path:
        fixture = load_p51_synthetic_fixture(REPO_ROOT, p51)
        return fixture.create_descriptor_root(self.work / "descriptors")

    def test_predecessors_are_complete_exact_sealed_releases(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)
        p56 = load_exact_p56(REPO_ROOT)
        self.assertTrue(callable(p51.validate_descriptor_root))
        self.assertTrue(callable(p56.publish_retained_build_and_custody))
        source = (ROOT / "capability_bound_executor.py").read_text(encoding="utf-8")
        parameters = inspect.signature(executor.run_capability_bound_diagnostic_executor).parameters
        self.assertNotIn("retained_custodies", parameters)
        self.assertNotIn("process_runner", parameters)
        self.assertNotIn("environment", parameters)
        self.assertNotIn("pulse-41-pulse-39-public-custody", executor.P57_GATE_IDS)
        self.assertEqual(executor.P57_GATE_IDS[0], "sealed-predecessor-binding")
        self.assertIn("launch_verified", source)
        self.assertIn("close_custody", source)

    def test_forged_p56_release_is_rejected_before_import(self) -> None:
        copied = self.work / "repository"
        source = (
            REPO_ROOT
            / "docs"
            / "simulations"
            / "profile-diff-held-out"
            / P56.directory
        )
        target = (
            copied
            / "docs"
            / "simulations"
            / "profile-diff-held-out"
            / P56.directory
        )
        target.parent.mkdir(parents=True)
        shutil.copytree(source, target)
        (target / "retained_build_custody.py").write_text("forged\n", encoding="utf-8")
        with self.assertRaisesRegex(SealedDependencyFailure, "P57-SEALED-IDENTITY"):
            verify_release(copied, P56)

    def test_bound_import_executes_verified_bytes_without_path_reopen_or_pycache(self) -> None:
        copied = self.work / "repository"
        source = (
            REPO_ROOT
            / "docs"
            / "simulations"
            / "profile-diff-held-out"
            / P56.directory
        )
        target = (
            copied
            / "docs"
            / "simulations"
            / "profile-diff-held-out"
            / P56.directory
        )
        target.parent.mkdir(parents=True)
        shutil.copytree(source, target)
        original = sealed._safe_regular
        reads = 0

        def read_then_substitute(path: Path) -> bytes:
            nonlocal reads
            content = original(path)
            if path == target / P56.source:
                reads += 1
                path.write_bytes(b'raise RuntimeError("substituted after verification")\n')
            return content

        with patch.object(sealed, "_safe_regular", side_effect=read_then_substitute):
            p56 = sealed.load_exact_p56(copied)
        self.assertTrue(callable(p56.launch_verified))
        self.assertEqual(reads, 1)
        self.assertFalse(list(target.rglob("__pycache__")))

    def test_worker_loader_executes_verified_helper_bytes_without_pycache(self) -> None:
        bundle = self.work / "bundle"
        worker_root = bundle / "worker"
        worker_root.mkdir(parents=True)
        helper = worker_root / "sealed_dependencies.py"
        shutil.copyfile(ROOT / "sealed_dependencies.py", helper)
        original = worker._safe_regular

        def read_then_substitute(path: Path, code: str) -> bytes:
            content = original(path, code)
            if path == helper:
                path.write_bytes(b'raise RuntimeError("substituted after verification")\n')
            return content

        with patch.object(worker, "_safe_regular", side_effect=read_then_substitute):
            dependencies = worker._load_sealed_dependencies(bundle)
        self.assertTrue(callable(dependencies.load_exact_p56))
        self.assertFalse(list(bundle.rglob("__pycache__")))

    def test_public_evidence_cannot_become_capability(self) -> None:
        p56 = load_exact_p56(REPO_ROOT)
        evidence_root = self.work / "public-evidence"
        evidence_root.mkdir()
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-FORGERY"):
            p56.launch_verified(evidence_root, "windows-x86_64", [])  # type: ignore[arg-type]
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-FORGERY"):
            p56.close_custody(evidence_root)  # type: ignore[arg-type]

    def test_final_cleanup_failure_has_one_failed_terminal_after_138_launches(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)
        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py")
        result = executor._run_qualification_executor(
            REPO_ROOT,
            self._fixture_root(p51),
            self.work,
            self.work / "p27",
            self._controls(fake, p51, fail_close=True),
        )
        terminal = [event for event in result.events if event["event_kind"] == "terminal-stop"]
        self.assertEqual(len(fake.launches), 138)
        self.assertEqual(len(terminal), 1)
        self.assertEqual(terminal[0]["outcome"], "failed")
        self.assertEqual(result.private_record["failure_code"], "P57-INDETERMINATE-CLEANUP")
        self.assertNotIn("P57-P43-CATALOG", result.private_record.values())

    def test_freezes_all_input_identities_and_stops_lazy_substitution(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)
        descriptors = self._fixture_root(p51)
        _, validated = p51.validate_descriptor_root(descriptors, self.work)
        target = validated[1].before
        assert target is not None

        def substitute(_platform: str, _arguments: tuple[str, ...], count: int) -> None:
            if count == 1:
                target.write_bytes(b"{}")

        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py", on_launch=substitute)
        result = executor._run_qualification_executor(
            REPO_ROOT, descriptors, self.work, self.work / "p27", self._controls(fake, p51)
        )
        self.assertEqual(result.private_record["failure_code"], "P57-P51-INPUT-SUBSTITUTION")
        self.assertEqual(len(fake.launches), 2, "substitution stops before the affected descriptor")

    def test_prelaunch_semantics_substitution_uses_frozen_semantics(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)
        descriptors = self._fixture_root(p51)
        _, validated = p51.validate_descriptor_root(descriptors, self.work)
        target = validated[0].after
        assert target is not None

        def substitute(_platform: str, _arguments: tuple[str, ...], count: int) -> None:
            if count == 1:
                target.write_bytes(b'{"schema":"ferris.platform-profile/v1"}')

        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py", on_launch=substitute)
        result = executor._run_qualification_executor(
            REPO_ROOT, descriptors, self.work, self.work / "p27", self._controls(fake, p51)
        )
        self.assertEqual(result.private_record["failure_code"], "P57-P51-INPUT-SUBSTITUTION")
        self.assertEqual(len(fake.launches), 1)

    def test_profile_result_mismatch_stops_at_first_launch(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)

        def mismatch(completed: subprocess.CompletedProcess[bytes]) -> subprocess.CompletedProcess[bytes]:
            return subprocess.CompletedProcess(completed.args, completed.returncode, b"not-a-result\n", b"")

        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py", result_mutator=mismatch)
        result = executor._run_qualification_executor(
            REPO_ROOT,
            self._fixture_root(p51),
            self.work,
            self.work / "p27",
            self._controls(fake, p51),
        )
        self.assertTrue(result.private_record["failure_code"].startswith("P51-"))
        self.assertEqual(len(fake.launches), 1)

    def test_dependency_failure_at_terminal_dependency_call_is_bounded(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)
        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py")
        with patch.object(
            p51,
            "load_terminal_dependencies",
            side_effect=p51.DependencyFailure("P51-DEPENDENCY-NEGATIVE"),
        ):
            result = executor._run_qualification_executor(
                REPO_ROOT,
                self._fixture_root(p51),
                self.work,
                self.work / "p27",
                self._controls(fake, p51),
            )
        self.assertEqual(result.private_record["failure_code"], "P51-DEPENDENCY-NEGATIVE")
        self.assertEqual(
            len([event for event in result.events if event["event_kind"] == "terminal-stop"]), 1
        )

    def test_p31_failure_at_contract_call_is_bounded(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)
        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py")
        with patch.object(
            p51, "verify_bound_contract", side_effect=p51.P31Failure("P31-NEGATIVE")
        ):
            result = executor._run_qualification_executor(
                REPO_ROOT,
                self._fixture_root(p51),
                self.work,
                self.work / "p27",
                self._controls(fake, p51),
            )
        self.assertEqual(result.private_record["failure_code"], "P31-NEGATIVE")
        self.assertEqual(
            len([event for event in result.events if event["event_kind"] == "terminal-stop"]), 1
        )

    def test_custody_failure_at_p35_p37_call_is_bounded(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)
        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py")
        with patch.object(
            p51, "verify_p35_p37_custody", side_effect=p51.CustodyFailure("P35-NEGATIVE")
        ):
            result = executor._run_qualification_executor(
                REPO_ROOT,
                self._fixture_root(p51),
                self.work,
                self.work / "p27",
                self._controls(fake, p51),
            )
        self.assertEqual(result.private_record["failure_code"], "P35-NEGATIVE")
        self.assertEqual(
            len([event for event in result.events if event["event_kind"] == "terminal-stop"]), 1
        )

    def test_executor_failure_at_descriptor_call_is_bounded(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)
        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py")
        with patch.object(
            p51,
            "validate_descriptor_root",
            side_effect=p51.ExecutorFailure("P51-EXECUTOR-NEGATIVE"),
        ):
            result = executor._run_qualification_executor(
                REPO_ROOT,
                self._fixture_root(p51),
                self.work,
                self.work / "p27",
                self._controls(fake, p51),
            )
        self.assertEqual(result.private_record["failure_code"], "P51-EXECUTOR-NEGATIVE")
        self.assertEqual(
            len([event for event in result.events if event["event_kind"] == "terminal-stop"]), 1
        )

    def test_loaded_terminal_failure_classes_are_bounded(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)
        p43, p45, p47 = p51.load_terminal_dependencies(REPO_ROOT)
        controls = executor._with_terminal_failure_types(
            self._controls(FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py"), p51),
            p43,
            p45,
            p47,
        )
        failures = (
            p51._TerminalPreconditionFailure("P51-TERMINAL-NEGATIVE"),
            p43.PublicFailure("P43-NEGATIVE"),
            p45.BridgeFailure("P45-NEGATIVE"),
            p45.SummaryMalformed(),
            p47.WitnessFailure("P47-NEGATIVE"),
            p47.SummaryMalformed(),
        )
        for failure in failures:
            with self.subTest(failure=type(failure).__name__):
                self.assertTrue(executor._known_failure(failure, controls))

    def test_capability_exhaustion_is_rejected(self) -> None:
        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py", uses=1)
        handle = fake.publish_retained_build_and_custody("windows-x86_64", self.work)
        fake.launch_verified(handle, "windows-x86_64", ("profile-diff",))
        with self.assertRaisesRegex(fake.ReleaseFailure, "P56-HANDLE-EXPIRED"):
            fake.launch_verified(handle, "windows-x86_64", ("profile-diff",))

    def test_worker_rejects_injection_replay_order_and_extra_output(self) -> None:
        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py")
        protocol = WorkerProtocol(fake, self.work)
        injected = {
            "arguments": ["profile-diff", "--before", "x", "--after", "y", "--format", "json"],
            "environment": {"PATH": "attacker"},
            "ordinal": 1,
            "platform": PLATFORM,
            "request_id": "sha256:" + "0" * 64,
            "schema": SCHEMA,
            "type": "launch",
        }
        with self.assertRaisesRegex(WorkerFailure, "P57-WSL-PROTOCOL"):
            protocol.consume(json.dumps(injected, separators=(",", ":"), sort_keys=True).encode("ascii") + b"\n")
        arguments = ["profile-diff", "--before", "x", "--after", "y", "--format", "json"]
        request = {
            "arguments": arguments,
            "ordinal": 1,
            "platform": PLATFORM,
            "request_id": _request_id(1, arguments),
            "schema": SCHEMA,
            "type": "launch",
        }
        raw = json.dumps(request, separators=(",", ":"), sort_keys=True).encode("ascii") + b"\n"
        self.assertIsNotNone(protocol.consume(raw))
        with self.assertRaisesRegex(WorkerFailure, "P57-WSL-PROTOCOL"):
            protocol.consume(raw)
        out_of_order = dict(request, ordinal=3, request_id=_request_id(3, arguments))
        with self.assertRaisesRegex(WorkerFailure, "P57-WSL-PROTOCOL"):
            protocol.consume(
                json.dumps(out_of_order, separators=(",", ":"), sort_keys=True).encode("ascii") + b"\n"
            )
        with self.assertRaisesRegex(WorkerFailure, "P57-WSL-PROTOCOL"):
            _line_value(b'{"schema":"ferris.pulse-57-wsl-capability-session/v1"}\nextra', 100)
        protocol.close_after_failure()

    def test_worker_unknown_fault_cleans_up_then_reraises(self) -> None:
        class FaultingP56:
            ReleaseFailure = RuntimeError

            def publish_retained_build_and_custody(self, _platform: str, _parent: Path) -> object:
                return object()

            def close_custody(self, _handle: object) -> None:
                return None

            def launch_verified(
                self, _handle: object, _platform: str, _arguments: list[str]
            ) -> object:
                raise AssertionError("worker programmer fault")

        protocol = WorkerProtocol(FaultingP56(), self.work)
        arguments = ["profile-diff", "--before", "x", "--after", "y", "--format", "json"]
        request = {
            "arguments": arguments,
            "ordinal": 1,
            "platform": PLATFORM,
            "request_id": _request_id(1, arguments),
            "schema": SCHEMA,
            "type": "launch",
        }
        with self.assertRaisesRegex(AssertionError, "worker programmer fault"):
            protocol.consume(json.dumps(request, separators=(",", ":"), sort_keys=True).encode("ascii") + b"\n")

    def test_worker_unknown_fault_with_cleanup_failure_is_indeterminate(self) -> None:
        class FaultingP56:
            class ReleaseFailure(RuntimeError):
                pass

            def publish_retained_build_and_custody(self, _platform: str, _parent: Path) -> object:
                return object()

            def close_custody(self, _handle: object) -> None:
                raise self.ReleaseFailure("P56-cleanup")

            def launch_verified(
                self, _handle: object, _platform: str, _arguments: list[str]
            ) -> object:
                raise AssertionError("worker programmer fault")

        protocol = WorkerProtocol(FaultingP56(), self.work)
        arguments = ["profile-diff", "--before", "x", "--after", "y", "--format", "json"]
        request = {
            "arguments": arguments,
            "ordinal": 1,
            "platform": PLATFORM,
            "request_id": _request_id(1, arguments),
            "schema": SCHEMA,
            "type": "launch",
        }
        with self.assertRaisesRegex(WorkerFailure, "P57-INDETERMINATE-CLEANUP") as raised:
            protocol.consume(json.dumps(request, separators=(",", ":"), sort_keys=True).encode("ascii") + b"\n")
        self.assertIsNotNone(raised.exception.__cause__)

    def test_worker_startup_io_failure_terminates_orphan(self) -> None:
        process = _StartupIoProcess()
        staged = executor._StagedBundle("/home/p57-test", {"executable": "/usr/bin/python3", "version": [3, 12, 0]})
        with (
            patch.object(executor, "_stage_wsl_bundle", return_value=staged),
            patch.object(executor.subprocess, "Popen", return_value=process),
        ):
            with self.assertRaisesRegex(executor.ExecutorFailure, "P57-WSL-PROTOCOL"):
                executor._NativeWslSession(REPO_ROOT, "/home/runtime", object())
        self.assertTrue(process.terminated)

    def test_close_timeout_kills_worker_and_reports_cleanup_failure(self) -> None:
        identity = {"executable": "/usr/bin/python3", "version": [3, 12, 0]}
        ready = executor._canonical_line(
            {
                "count": 69,
                "platform": PLATFORM,
                "python": identity,
                "schema": SCHEMA,
                "type": "ready",
            }
        )
        process = _CloseTimeoutProcess(ready)
        staged = executor._StagedBundle("/home/p57-test", identity)
        with (
            patch.object(executor, "_stage_wsl_bundle", return_value=staged),
            patch.object(executor.subprocess, "Popen", return_value=process),
        ):
            session = executor._NativeWslSession(REPO_ROOT, "/home/runtime", object())
            with self.assertRaisesRegex(executor.ExecutorFailure, "P57-WSL-CLEANUP"):
                session.close()
        self.assertTrue(process.terminated)
        self.assertTrue(process.killed)

    def test_unknown_programmer_fault_cleans_up_then_reraises(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)

        def fault(_platform: str, _arguments: tuple[str, ...], _count: int) -> None:
            raise AssertionError("programmer fault")

        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py", on_launch=fault)
        with self.assertRaisesRegex(AssertionError, "programmer fault"):
            executor._run_qualification_executor(
                REPO_ROOT,
                self._fixture_root(p51),
                self.work,
                self.work / "p27",
                self._controls(fake, p51),
            )

    def test_unknown_programmer_fault_with_cleanup_failure_is_indeterminate(self) -> None:
        p51 = load_exact_p51(REPO_ROOT)

        def fault(_platform: str, _arguments: tuple[str, ...], _count: int) -> None:
            raise AssertionError("programmer fault")

        fake = FakeP56(P51_ROOT / "fixtures" / "fake_ferris.py", on_launch=fault)
        with self.assertRaisesRegex(executor.ExecutorFailure, "P57-INDETERMINATE-CLEANUP") as raised:
            executor._run_qualification_executor(
                REPO_ROOT,
                self._fixture_root(p51),
                self.work,
                self.work / "p27",
                self._controls(fake, p51, fail_close=True),
            )
        self.assertIsNotNone(raised.exception.__cause__)


if __name__ == "__main__":
    unittest.main()
