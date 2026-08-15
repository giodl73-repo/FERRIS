from __future__ import annotations

import atexit
import inspect
import json
import os
import sys
import unittest
import uuid
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = ROOT.parents[3]
RUN_ROOT = REPO_ROOT / "target" / "pulse-53-test-runtime"
sys.dont_write_bytecode = True
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import witness_preserving_ordered_executor as executor
from sealed_dependencies import (
    PULSE51_COMMIT,
    PULSE51_SOURCE_SHA256,
    PULSE52_COMMIT,
    PULSE52_MANIFEST_AGGREGATE,
    PULSE52_MANIFEST_RAW_SHA256,
    PULSE52_SOURCE_SHA256,
    load_pulse52,
)
from synthetic_fixture import (
    QualificationProcessRunner,
    cleanup_synthetic_runtime_root,
    create_synthetic_custodies,
    public_result_text,
    synthetic_seed,
)


def _clean_release_python_residue() -> None:
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


_clean_release_python_residue()
_clean_prelaunch_python_residue()
atexit.register(_clean_release_python_residue)
atexit.register(_clean_prelaunch_python_residue)


class WitnessPreservingOrderedExecutorTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        _clean_release_python_residue()
        _clean_prelaunch_python_residue()
        if RUN_ROOT.exists():
            cleanup_synthetic_runtime_root(RUN_ROOT)
        RUN_ROOT.mkdir(parents=True)

    @classmethod
    def tearDownClass(cls) -> None:
        if RUN_ROOT.exists():
            cleanup_synthetic_runtime_root(RUN_ROOT)
        _clean_release_python_residue()
        _clean_prelaunch_python_residue()

    def setUp(self) -> None:
        self.sandbox = RUN_ROOT / uuid.uuid4().hex
        self.sandbox.mkdir()
        self.runtime_root = self.sandbox / "runtime"
        self.runtime_root.mkdir()
        self.p52, self.p51 = load_pulse52(REPO_ROOT)
        self.p43, self.p45, self.p47 = self.p51.load_terminal_dependencies(REPO_ROOT)
        _clean_prelaunch_python_residue()
        self.custodies, self.expectations = create_synthetic_custodies(
            self.runtime_root, self.p51
        )
        self.p41_copy_index = 0

    def tearDown(self) -> None:
        if self.sandbox.exists():
            cleanup_synthetic_runtime_root(self.sandbox)
        _clean_prelaunch_python_residue()

    def _run(
        self,
        *,
        p39_checkout_root: object = REPO_ROOT,
        terminal: object | None = None,
        seed_index: int = 1,
    ):
        _clean_prelaunch_python_residue()
        self.p41_copy_index += 1
        runner = QualificationProcessRunner(self.runtime_root, self.p51)
        call = lambda: executor._run_qualification_executor(
            REPO_ROOT,
            self.runtime_root,
            self.runtime_root / "p27-cycle",
            p39_checkout_root,
            self.sandbox / f"p41-final-{self.p41_copy_index}",
            self.custodies,
            seed_bytes=synthetic_seed(seed_index),
            process_runner=runner,
            expectations=self.expectations,
        )
        if terminal is None:
            return call(), runner
        with mock.patch.object(self.p51, "invoke_terminal_pulse47_once", new=terminal):
            return call(), runner

    def _run_loaded_for_fault_test(self, terminal: object, *, seed_index: int):
        """Use the sealed loaded phase engine after a fake-only lower-level fault patch."""

        _clean_prelaunch_python_residue()
        self.p41_copy_index += 1
        p39, p41 = self.p52.load_p39_and_p41(REPO_ROOT)
        runner = QualificationProcessRunner(self.runtime_root, self.p51)
        controls = self.p52._QualificationControls(
            synthetic_seed(seed_index),
            runner,
            self.expectations,
        )
        with mock.patch.object(self.p51, "invoke_terminal_pulse47_once", new=terminal):
            result = executor._run_loaded(
                self.p52,
                self.p51,
                p39,
                p41,
                REPO_ROOT,
                self.runtime_root,
                self.runtime_root / "p27-cycle",
                REPO_ROOT,
                self.sandbox / f"p41-final-{self.p41_copy_index}",
                self.custodies,
                controls,
            )
        return result, runner

    @staticmethod
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

    def _p43_failure(self, state: str) -> dict[str, object]:
        if state == "absent":
            rename_attempts = 0
            sync = {name: self._sync("not-attempted") for name in ("final_parent", "rollback_parent", "stage")}
        elif state == "rolled-back":
            rename_attempts = 1
            sync = {
                "final_parent": self._sync("synced"),
                "rollback_parent": self._sync("synced"),
                "stage": self._sync("synced"),
            }
        elif state == "indeterminate":
            rename_attempts = 1
            sync = {
                "final_parent": self._sync("failed"),
                "rollback_parent": self._sync("failed"),
                "stage": self._sync("failed"),
            }
        else:
            raise AssertionError(state)
        return {
            "schema": self.p43.SUMMARY_SCHEMA,
            "failure_code": "P43-STAGE-COPY-FAILURE",
            "publication": {
                "final_files_present": False,
                "rename_attempts": rename_attempts,
                "retries": 0,
                "state": state,
                "sync": sync,
            },
        }

    def _witnessed_failure_terminal(self, state: str, calls: list[object]):
        failure = self._p43_failure(state)

        def terminal(_terminal: object, result: object, p43_root: Path, witness_root: Path):
            calls.append((_terminal, result, p43_root, witness_root))
            return self.p47.witness_pulse_43(
                result.catalog,
                result.events,
                p43_root,
                witness_root,
                invoker=lambda *_arguments: failure,
            )

        return terminal

    def _assert_completed_execution(self, result: object, runner: QualificationProcessRunner) -> None:
        self.assertEqual(result.private_record["execution_outcome"], "completed")
        self.assertEqual(result.private_record["process_counts"], {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69})
        self.assertEqual(runner.counts, {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69})
        self.assertEqual(len(runner.dispatches), 138)
        self.assertEqual(result.private_record["terminal_p47_invocation_count"], 1)
        self.assertEqual(len(result.events), 10)
        self.assertEqual(result.events[-1]["event_kind"], "terminal-stop")
        self.assertEqual(result.events[-1]["outcome"], "completed")

    def _assert_failure_witness(self, state: str, rename_attempts: int) -> None:
        calls: list[object] = []
        result, runner = self._run(
            terminal=self._witnessed_failure_terminal(state, calls), seed_index=3
        )
        self._assert_completed_execution(result, runner)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result.publication["disposition"], "published-failure-witness")
        self.assertEqual(result.private_record["outcome"], "published-failure-witness")
        self.assertEqual(result.private_record["terminal_publication_cleanup"], "retained-failure-witness")
        self.assertEqual(result.publication["product_conclusion"], None)
        self.assertEqual(result.publication["category_conclusion"], None)
        self.assertEqual(result.publication["fix_conclusion"], None)
        posture = result.publication["posture"]
        self.assertEqual(posture["source"], "pulse-43")
        self.assertEqual(posture["failure_code"], "P43-STAGE-COPY-FAILURE")
        self.assertEqual(posture["publication"]["state"], state)
        self.assertEqual(posture["publication"]["rename_attempts"], rename_attempts)
        self.assertEqual(
            result.transfer_descriptor["expected_public_tree_kind"], "failure-witness-only"
        )
        self.assertEqual(result.transfer_descriptor["exact_file_counts"], {"witness": 2, "total": 2})
        self.assertEqual(set(result.transfer_descriptor["verified_raw_payload_hashes"]), {"witness"})
        terminal_root = self.runtime_root / executor.TERMINAL_DIRECTORY
        self.assertEqual([entry.name for entry in terminal_root.iterdir()], [executor.WITNESS_FINAL_DIRECTORY])
        witness_root = terminal_root / executor.WITNESS_FINAL_DIRECTORY
        self.assertEqual(sorted(entry.name for entry in witness_root.iterdir()), ["publication-witness.json", "release-receipt.json"])
        self.assertFalse((terminal_root / executor.P43_FINAL_DIRECTORY).exists())
        self.assertFalse((terminal_root / f".{executor.P43_FINAL_DIRECTORY}.pulse-43-stage").exists())
        descriptor_text = json.dumps(result.transfer_descriptor, sort_keys=True)
        self.assertNotIn(str(terminal_root), descriptor_text)
        self.assertNotIn(executor.P43_FINAL_DIRECTORY, descriptor_text)
        self.assertNotIn(executor.WITNESS_FINAL_DIRECTORY, descriptor_text)
        public_text = public_result_text(result)
        for forbidden in ("seed", "private-launch", "p27-cycle", "case_id", str(terminal_root)):
            self.assertNotIn(forbidden, public_text)

    def _invalid_terminal(self, summary: object, calls: list[object]):
        def terminal(_terminal: object, _result: object, _p43_root: Path, _witness_root: Path):
            calls.append(_terminal)
            return summary

        return terminal

    def _assert_invalid_cleanup(self, terminal: object) -> None:
        calls: list[object] = []
        result, runner = self._run(terminal=terminal(calls), seed_index=4)
        self._assert_completed_execution(result, runner)
        self.assertEqual(result.publication["disposition"], "invalid-witness-publication")
        self.assertIsNone(result.transfer_descriptor)
        self.assertEqual(result.private_record["outcome"], "invalid-witness-publication")
        self.assertEqual(result.private_record["terminal_publication_cleanup"], "removed-and-verified")
        self.assertFalse((self.runtime_root / executor.TERMINAL_DIRECTORY).exists())

    def test_sealed_production_wiring_binds_exact_pulse51_and_pulse52(self) -> None:
        self.assertEqual(PULSE51_COMMIT, "d09c923c1e2cd2be003026597f4ad2a0e2d3764f")
        self.assertEqual(PULSE52_COMMIT, "e4ef9617f227670f3911be42ca63df4b2e66d24f")
        self.assertEqual(PULSE51_SOURCE_SHA256, "sha256:97c404dbf29d387561878772403c7fbd2672e97283b0620e838e7126ecbdd637")
        self.assertEqual(PULSE52_MANIFEST_RAW_SHA256, "sha256:e585d6baaf83783ff1a1c65e1d3f281ce1d3afd9806f9cb9811b328eff9811da")
        self.assertEqual(PULSE52_MANIFEST_AGGREGATE, "sha256:3da8401a52d020ead7b9c6854461da5f28dfb9d1117385cd6943592f74e8aaec")
        self.assertEqual(PULSE52_SOURCE_SHA256, "sha256:768f4dc3af1009515e2e28ebc211af76215f434cee209b547d7be923a1bcec73")
        self.assertIs(self.p52, load_pulse52(REPO_ROOT)[0])
        self.assertEqual(
            tuple(inspect.signature(executor.run_witness_preserving_ordered_executor).parameters),
            (
                "repo_root",
                "private_runtime_root",
                "p27_cycle_root",
                "p39_checkout_root",
                "p41_final_root",
                "retained_custodies",
            ),
        )
        self.assertEqual(
            executor.__all__,
            [
                "TerminalPublicationCleanupIndeterminate",
                "WitnessPreservingOrderedResult",
                "run_witness_preserving_ordered_executor",
            ],
        )

    def test_published_result_retains_verified_result_and_witness(self) -> None:
        result, runner = self._run()
        self._assert_completed_execution(result, runner)
        self.assertEqual(result.publication["disposition"], "published-result")
        self.assertEqual(result.private_record["outcome"], "published-result")
        self.assertEqual(result.private_record["terminal_publication_cleanup"], "retained-published-result")
        self.assertEqual(result.transfer_descriptor["expected_public_tree_kind"], "result-and-witness")
        self.assertEqual(result.transfer_descriptor["exact_file_counts"], {"result": 2, "witness": 2, "total": 4})
        self.assertEqual(set(result.transfer_descriptor["verified_raw_payload_hashes"]), {"result", "witness"})
        terminal_root = self.runtime_root / executor.TERMINAL_DIRECTORY
        self.assertEqual(sorted(entry.name for entry in terminal_root.iterdir()), [executor.P43_FINAL_DIRECTORY, executor.WITNESS_FINAL_DIRECTORY])
        self.assertTrue((terminal_root / executor.P43_FINAL_DIRECTORY / "public-result.json").is_file())
        self.assertTrue((terminal_root / executor.WITNESS_FINAL_DIRECTORY / "publication-witness.json").is_file())
        self.assertEqual(result.events[-1]["outcome"], "completed")

    def test_absent_p43_failure_is_retained_as_failure_witness(self) -> None:
        self._assert_failure_witness("absent", 0)

    def test_rolled_back_p43_failure_is_retained_as_failure_witness(self) -> None:
        self._assert_failure_witness("rolled-back", 1)

    def test_indeterminate_p43_failure_is_retained_as_failure_witness(self) -> None:
        self._assert_failure_witness("indeterminate", 1)

    def test_p47_failure_malformed_and_hash_mismatch_clean_terminal_residue(self) -> None:
        failure = {
            "schema": self.p47.SUMMARY_SCHEMA,
            "failure_code": "P47-WITNESS-FINAL-VERIFY-FAILURE",
            "witness_publication": {
                "final_files_present": False,
                "rename_attempts": 0,
                "retries": 0,
                "state": "absent",
                "sync": {name: self._sync("not-attempted") for name in ("final_parent", "rollback_parent", "stage")},
            },
        }
        self._assert_invalid_cleanup(lambda calls: self._invalid_terminal(failure, calls))

    def test_malformed_terminal_summary_cleans_without_republication(self) -> None:
        self._assert_invalid_cleanup(
            lambda calls: self._invalid_terminal({"schema": self.p47.SUMMARY_SCHEMA}, calls)
        )

    def test_witness_hash_mismatch_cleans_verified_residue(self) -> None:
        calls: list[object] = []
        failure = self._p43_failure("absent")

        def terminal(_terminal: object, result: object, p43_root: Path, witness_root: Path):
            calls.append(_terminal)
            summary = self.p47.witness_pulse_43(
                result.catalog,
                result.events,
                p43_root,
                witness_root,
                invoker=lambda *_arguments: failure,
            )
            summary["witness_publication"]["raw_hashes"]["witness_raw_sha256"] = "sha256:" + "f" * 64
            return summary

        self._assert_invalid_cleanup(lambda _calls: terminal)
        self.assertEqual(len(calls), 1)

    def test_p43_result_residue_prevents_failure_witness_retention(self) -> None:
        calls: list[object] = []
        failure = self._p43_failure("absent")

        def terminal(_terminal: object, result: object, p43_root: Path, witness_root: Path):
            calls.append(_terminal)
            summary = self.p47.witness_pulse_43(
                result.catalog,
                result.events,
                p43_root,
                witness_root,
                invoker=lambda *_arguments: failure,
            )
            p43_root.mkdir()
            (p43_root / "public-result.json").write_text("residue\n", encoding="utf-8", newline="\n")
            return summary

        self._assert_invalid_cleanup(lambda _calls: terminal)
        self.assertEqual(len(calls), 1)

    def test_prelaunch_failure_does_not_call_csprng_or_start_private_launch(self) -> None:
        with mock.patch.object(
            self.p52.secrets,
            "token_bytes",
            side_effect=AssertionError("CSPRNG must not run before public custody"),
        ) as token_bytes:
            result, runner = self._run(p39_checkout_root=Path("relative"))
        token_bytes.assert_not_called()
        self.assertEqual(result.private_record["failure_code"], "P52-P41-P39-PRELAUNCH")
        self.assertFalse(result.private_record["private_launch_started"])
        self.assertEqual(result.private_record["materializer_invocations"], 0)
        self.assertEqual(result.private_record["terminal_p47_invocation_count"], 0)
        self.assertEqual(result.publication["disposition"], "not-attempted")
        self.assertEqual(runner.dispatches, [])

    def test_cleanup_fatal_and_programmer_faults_propagate(self) -> None:
        summary = {
            "schema": self.p47.SUMMARY_SCHEMA,
            "failure_code": "P47-WITNESS-FINAL-VERIFY-FAILURE",
            "witness_publication": {
                "final_files_present": False,
                "rename_attempts": 0,
                "retries": 0,
                "state": "absent",
                "sync": {name: self._sync("not-attempted") for name in ("final_parent", "rollback_parent", "stage")},
            },
        }
        original_remove = self.p52._remove_private_tree

        def permanent_lock(p51: object, path: Path, code: str = "P52-PRIVATE-CLEANUP") -> None:
            if path.name == executor.TERMINAL_DIRECTORY:
                raise PermissionError("terminal locked")
            original_remove(p51, path, code)

        with (
            mock.patch.object(self.p52, "_remove_private_tree", new=permanent_lock),
            mock.patch.object(self.p52.time, "sleep", return_value=None),
            self.assertRaises(executor.TerminalPublicationCleanupIndeterminate) as raised,
        ):
            self._run_loaded_for_fault_test(
                self._invalid_terminal(summary, []), seed_index=5
            )
        self.assertEqual(str(raised.exception), "terminal-publication-cleanup-indeterminate")
        self.assertEqual(raised.exception.public_posture["cleanup_posture"], "unresolved")
        for path in (
            self.runtime_root / executor.PRIVATE_LAUNCH_DIRECTORY,
            self.runtime_root / executor.TERMINAL_DIRECTORY,
            self.runtime_root / "p27-cycle",
        ):
            if path.exists():
                cleanup_synthetic_runtime_root(path)

        for error_type in (TypeError, AssertionError):
            with self.subTest(cleanup_error=error_type.__name__):
                def programmer_fault(
                    p51: object,
                    path: Path,
                    code: str = "P52-PRIVATE-CLEANUP",
                    _error: type[Exception] = error_type,
                ) -> None:
                    if path.name == executor.TERMINAL_DIRECTORY:
                        raise _error("cleanup programmer fault")
                    original_remove(p51, path, code)

                try:
                    with (
                        mock.patch.object(self.p52, "_remove_private_tree", new=programmer_fault),
                        self.assertRaises(error_type),
                    ):
                        self._run_loaded_for_fault_test(
                            self._invalid_terminal(summary, []), seed_index=7
                        )
                finally:
                    for path in (
                        self.runtime_root / executor.PRIVATE_LAUNCH_DIRECTORY,
                        self.runtime_root / executor.TERMINAL_DIRECTORY,
                        self.runtime_root / "p27-cycle",
                    ):
                        if path.exists():
                            cleanup_synthetic_runtime_root(path)

    def test_terminal_programmer_faults_propagate(self) -> None:
        for error_type in (TypeError, AssertionError):
            with self.subTest(error=error_type.__name__):
                def broken_terminal(*_arguments: object, _error: type[Exception] = error_type):
                    raise _error("programmer fault")

                try:
                    with self.assertRaises(error_type):
                        self._run(terminal=broken_terminal, seed_index=6)
                finally:
                    for path in (
                        self.runtime_root / executor.PRIVATE_LAUNCH_DIRECTORY,
                        self.runtime_root / executor.TERMINAL_DIRECTORY,
                        self.runtime_root / "p27-cycle",
                    ):
                        if path.exists():
                            cleanup_synthetic_runtime_root(path)
