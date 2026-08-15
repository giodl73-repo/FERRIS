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
RUN_ROOT = REPO_ROOT / "target" / "pulse-52-test-runtime"
sys.dont_write_bytecode = True
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import ordered_materialization_executor as executor
import sealed_dependencies
from sealed_dependencies import (
    PULSE39_MANIFEST_RAW_SHA256,
    PULSE39_RECEIPT_RAW_SHA256,
    PULSE39_SEAL_RAW_SHA256,
    PULSE39_SOURCE_SHA256,
    PULSE41_MANIFEST_RAW_SHA256,
    PULSE41_RECEIPT_RAW_SHA256,
    PULSE41_SEAL_RAW_SHA256,
    PULSE41_SOURCE_SHA256,
    PULSE51_MANIFEST_RAW_SHA256,
    PULSE51_SOURCE_SHA256,
    SealedDependencyFailure,
    load_p39_and_p41,
    load_pulse51,
)
from synthetic_fixture import (
    QualificationProcessRunner,
    cleanup_synthetic_runtime_root,
    create_synthetic_custodies,
    public_result_text,
    synthetic_seed,
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


_clean_sealed_python_residue()
_clean_prelaunch_python_residue()
atexit.register(_clean_sealed_python_residue)
atexit.register(_clean_prelaunch_python_residue)


class OrderedMaterializationExecutorTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        _clean_sealed_python_residue()
        _clean_prelaunch_python_residue()
        if RUN_ROOT.exists():
            cleanup_synthetic_runtime_root(RUN_ROOT)
        RUN_ROOT.mkdir(parents=True)

    @classmethod
    def tearDownClass(cls) -> None:
        if RUN_ROOT.exists():
            cleanup_synthetic_runtime_root(RUN_ROOT)
        _clean_sealed_python_residue()
        _clean_prelaunch_python_residue()

    def setUp(self) -> None:
        _clean_prelaunch_python_residue()
        self.sandbox = RUN_ROOT / uuid.uuid4().hex
        self.sandbox.mkdir()
        self.runtime_root = self.sandbox / "runtime"
        self.runtime_root.mkdir()
        self.p51 = load_pulse51(REPO_ROOT)
        self.p39, self.p41 = load_p39_and_p41(REPO_ROOT)
        self.custodies, self.expectations = create_synthetic_custodies(
            self.runtime_root, self.p51
        )
        self.p27_cycle_root = self.runtime_root / "p27-cycle"
        self.p43, self.p45, self.p47 = self.p51.load_terminal_dependencies(REPO_ROOT)
        self.p41_copy_index = 0

    def tearDown(self) -> None:
        if self.sandbox.exists():
            cleanup_synthetic_runtime_root(self.sandbox)
        _clean_prelaunch_python_residue()

    def _run(
        self,
        *,
        p39_checkout_root: object = REPO_ROOT,
        p41_final_root: Path | None = None,
        p27_cycle_root: Path | None = None,
        force_materializer_destination_conflict: bool = False,
    ):
        _clean_prelaunch_python_residue()
        if p41_final_root is None:
            self.p41_copy_index += 1
            p41_final_root = self.sandbox / f"p41-final-{self.p41_copy_index}"
        runner = QualificationProcessRunner(self.runtime_root, self.p51)
        result = executor._run_qualification_executor(
            REPO_ROOT,
            self.runtime_root,
            self.p27_cycle_root if p27_cycle_root is None else p27_cycle_root,
            p39_checkout_root,
            p41_final_root,
            self.custodies,
            seed_bytes=synthetic_seed(1),
            process_runner=runner,
            expectations=self.expectations,
            force_materializer_destination_conflict=force_materializer_destination_conflict,
        )
        return result, runner

    @staticmethod
    def _sync(status: str) -> dict[str, object]:
        if status == "not-attempted":
            return {
                "attempted": False,
                "error_category": "not-attempted",
                "mechanism": "not-attempted",
                "status": status,
            }
        return {
            "attempted": True,
            "error_category": None,
            "mechanism": "os.open+os.fsync-directory-v1",
            "status": status,
        }

    @classmethod
    def _failed_posture(cls) -> dict[str, object]:
        return {
            "final_files_present": False,
            "rename_attempts": 0,
            "retries": 0,
            "state": "absent",
            "sync": {
                key: cls._sync("not-attempted")
                for key in ("final_parent", "rollback_parent", "stage")
            },
        }

    @classmethod
    def _published_witness_posture(cls) -> dict[str, object]:
        digest = "sha256:" + "0" * 64
        return {
            "files": "2/2",
            "final_files_present": True,
            "raw_hashes": {
                "receipt_payload_sha256": digest,
                "receipt_raw_sha256": digest,
                "witness_payload_sha256": digest,
                "witness_raw_sha256": digest,
            },
            "rename_attempts": 1,
            "retries": 0,
            "state": "published",
            "sync": {
                "final_parent": cls._sync("synced"),
                "rollback_parent": cls._sync("not-attempted"),
                "stage": cls._sync("synced"),
            },
        }

    def _assert_invalid_publication(self, result: object) -> None:
        self.assertEqual(result.private_record["execution_outcome"], "completed")
        self.assertEqual(result.private_record["outcome"], "invalid-publication-integrity")
        self.assertEqual(
            result.private_record["publication_disposition"],
            "invalid-publication-integrity",
        )
        self.assertEqual(result.publication["disposition"], "invalid-publication-integrity")
        self.assertIsNone(result.publication["product_conclusion"])
        self.assertIsNone(result.publication["category_conclusion"])
        self.assertIsNone(result.publication["fix_conclusion"])
        self.assertEqual(result.events[-1]["outcome"], "completed")
        self.assertEqual(len(result.events), 10)
        self.assertFalse((self.runtime_root / executor.TERMINAL_DIRECTORY).exists())
        self.assertEqual(
            result.private_record["terminal_publication_cleanup"], "removed-and-verified"
        )

    def _assert_prelaunch_rejected(self, **arguments: object) -> None:
        with mock.patch.object(
            executor.secrets,
            "token_bytes",
            side_effect=AssertionError("CSPRNG must not run before P39/P41 custody"),
        ) as token_bytes:
            result, runner = self._run(**arguments)
        token_bytes.assert_not_called()
        self.assertEqual(result.private_record["outcome"], "failed")
        self.assertEqual(result.private_record["execution_outcome"], "failed")
        self.assertFalse(result.private_record["private_launch_started"])
        self.assertEqual(result.private_record["p27_invocations"], 0)
        self.assertEqual(result.private_record["materializer_invocations"], 0)
        self.assertEqual(result.private_record["verifier_invocations"], 0)
        self.assertEqual(result.private_record["failure_code"], "P52-P41-P39-PRELAUNCH")
        self.assertEqual(result.events[-1]["gate_id"], "pulse-41-pulse-39-public-custody")
        self.assertEqual(runner.dispatches, [])

    def _clean_fault_residue(self, p27_cycle_root: Path) -> None:
        for path in (
            self.runtime_root / executor.PRIVATE_LAUNCH_DIRECTORY,
            self.runtime_root / executor.TERMINAL_DIRECTORY,
            p27_cycle_root,
        ):
            if path.exists():
                cleanup_synthetic_runtime_root(path)

    def test_executes_ordered_fake_cycle_then_cleans_private_material(self) -> None:
        result, runner = self._run()
        self.assertEqual(result.private_record["outcome"], "published")
        self.assertEqual(result.private_record["execution_outcome"], "completed")
        self.assertEqual(result.private_record["publication_disposition"], "published")
        self.assertEqual(result.publication["disposition"], "published")
        self.assertEqual(
            result.publication["posture"],
            {
                "p43_result": "published-and-verified",
                "p47_witness": "published-and-verified",
            },
        )
        self.assertIsNone(result.publication["product_conclusion"])
        self.assertIsNone(result.publication["category_conclusion"])
        self.assertIsNone(result.publication["fix_conclusion"])
        self.assertTrue(result.private_record["private_launch_started"])
        self.assertEqual(result.private_record["prelaunch_private_namespace_absence_checks"], 7)
        self.assertEqual(result.private_record["p39_checkout_verifications"], 1)
        self.assertEqual(result.private_record["p41_transactional_copy_invocations"], 1)
        self.assertEqual(result.private_record["p41_post_copy_binding"], "8/8")
        self.assertEqual(result.private_record["p27_invocations"], 1)
        self.assertEqual(result.private_record["materializer_invocations"], 1)
        self.assertEqual(result.private_record["verifier_invocations"], 1)
        self.assertEqual(result.private_record["seed_byte_count"], 32)
        self.assertEqual(result.private_record["seed_cleanup"], "removed-after-verification")
        self.assertEqual(result.private_record["descriptor_cleanup"], "removed")
        self.assertEqual(result.private_record["private_launch_cleanup"], "removed-and-verified")
        self.assertEqual(
            result.private_record["process_counts"],
            {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69},
        )
        self.assertEqual(len(result.private_record["no_launch_records"]), 2)
        self.assertEqual(runner.counts, {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69})
        self.assertEqual(len(runner.dispatches), 138)
        self.assertEqual(
            result.events[-1],
            {
                "classification": "ordered-execution",
                "event_kind": "terminal-stop",
                "gate_id": "bounded-process-exit-search",
                "outcome": "completed",
                "schema": "ferris.pulse-43-ordered-result-event/v1",
            },
        )
        self.assertFalse((self.runtime_root / executor.PRIVATE_LAUNCH_DIRECTORY).exists())
        terminal = self.runtime_root / executor.TERMINAL_DIRECTORY
        self.assertTrue((terminal / executor.P43_FINAL_DIRECTORY / "public-result.json").is_file())
        self.assertTrue(
            (terminal / executor.WITNESS_FINAL_DIRECTORY / "publication-witness.json").is_file()
        )
        self.assertEqual(result.private_record["terminal_p47_invocation_count"], 1)
        self.assertEqual(
            result.private_record["terminal_publication_cleanup"], "retained-published"
        )
        public_text = public_result_text(result)
        for forbidden in (
            "seed",
            "descriptors",
            "private-launch",
            "case_id",
            "ordinal",
            "p27-cycle",
        ):
            self.assertNotIn(forbidden, public_text)
        private_text = json.dumps(result.private_record, sort_keys=True)
        self.assertNotIn(synthetic_seed(1).hex(), private_text)

    def test_p43_failure_witnessed_by_p47_is_invalid_publication_integrity(self) -> None:
        calls: list[tuple[object, ...]] = []

        def terminal_failure(
            terminal: object,
            result: object,
            p43_final_root: Path,
            witness_final_root: Path,
        ) -> dict[str, object]:
            calls.append((terminal, result, p43_final_root, witness_final_root))
            return {
                "outcome": "published",
                "publication_outcome": {
                    "failure_code": "P43-STAGE-COPY-FAILURE",
                    "kind": "failed",
                    "publication": self._failed_posture(),
                },
                "schema": self.p47.SUMMARY_SCHEMA,
                "witness_publication": self._published_witness_posture(),
            }

        with mock.patch.object(
            self.p51, "invoke_terminal_pulse47_once", new=terminal_failure
        ):
            result, runner = self._run()

        self.assertEqual(len(calls), 1)
        self.assertEqual(len(runner.dispatches), 138)
        self._assert_invalid_publication(result)
        self.assertEqual(result.publication["posture"]["source"], "pulse-43")
        self.assertEqual(
            result.publication["posture"]["failure_code"], "P43-STAGE-COPY-FAILURE"
        )
        self.assertNotIn("ordered_execution", repr(result.publication["posture"]))

    def test_p47_witness_failure_is_invalid_publication_integrity(self) -> None:
        calls: list[tuple[object, ...]] = []

        def terminal_failure(
            terminal: object,
            result: object,
            p43_final_root: Path,
            witness_final_root: Path,
        ) -> dict[str, object]:
            calls.append((terminal, result, p43_final_root, witness_final_root))
            return {
                "failure_code": "P47-WITNESS-FINAL-VERIFY-FAILURE",
                "schema": self.p47.SUMMARY_SCHEMA,
                "witness_publication": self._failed_posture(),
            }

        with mock.patch.object(
            self.p51, "invoke_terminal_pulse47_once", new=terminal_failure
        ):
            result, _runner = self._run()

        self.assertEqual(len(calls), 1)
        self._assert_invalid_publication(result)
        self.assertEqual(result.publication["posture"]["source"], "pulse-47")
        self.assertEqual(
            result.publication["posture"]["failure_code"],
            "P47-WITNESS-FINAL-VERIFY-FAILURE",
        )

    def test_terminal_verifier_programmer_faults_propagate(self) -> None:
        for name, module, attribute in (
            ("p43", self.p43, "verify_publication_directory"),
            ("p47", self.p47, "verify_witness_directory"),
        ):
            for error_type in (TypeError, AssertionError):
                with self.subTest(verifier=name, error=error_type.__name__):
                    p27_cycle_root = self.runtime_root / (
                        f"p27-verifier-{name}-{error_type.__name__}"
                    )

                    def broken_verifier(*arguments: object, _error: type[Exception] = error_type):
                        del arguments
                        raise _error(f"{name} verifier programmer fault")

                    try:
                        with (
                            mock.patch.object(
                                executor, "load_pulse51", return_value=self.p51
                            ),
                            mock.patch.object(
                                self.p51,
                                "load_terminal_dependencies",
                                return_value=(self.p43, self.p45, self.p47),
                            ),
                            mock.patch.object(module, attribute, new=broken_verifier),
                            self.assertRaises(error_type),
                        ):
                            self._run(p27_cycle_root=p27_cycle_root)
                    finally:
                        self._clean_fault_residue(p27_cycle_root)

    def test_terminal_verifier_public_failures_remain_bounded(self) -> None:
        for name, module, attribute, error_type in (
            (
                "p43",
                self.p43,
                "verify_publication_directory",
                self.p43.PublicFailure,
            ),
            (
                "p47",
                self.p47,
                "verify_witness_directory",
                self.p47.WitnessFailure,
            ),
        ):
            with self.subTest(verifier=name):
                p27_cycle_root = self.runtime_root / f"p27-verifier-public-{name}"

                def public_failure(*arguments: object, _error: type[Exception] = error_type):
                    del arguments
                    raise _error("P52-TEST-TERMINAL-VERIFIER")

                with mock.patch.object(module, attribute, new=public_failure):
                    with (
                        mock.patch.object(executor, "load_pulse51", return_value=self.p51),
                        mock.patch.object(
                            self.p51,
                            "load_terminal_dependencies",
                            return_value=(self.p43, self.p45, self.p47),
                        ),
                    ):
                        result, _runner = self._run(p27_cycle_root=p27_cycle_root)

                self._assert_invalid_publication(result)
                self._clean_fault_residue(p27_cycle_root)

    def test_terminal_invocation_programmer_faults_propagate(self) -> None:
        for error_type in (TypeError, AssertionError):
            with self.subTest(error=error_type.__name__):
                p27_cycle_root = self.runtime_root / f"p27-invocation-{error_type.__name__}"

                def broken_terminal(*arguments: object, _error: type[Exception] = error_type):
                    del arguments
                    raise _error("terminal invocation programmer fault")

                try:
                    with (
                        mock.patch.object(
                            executor, "load_pulse51", return_value=self.p51
                        ),
                        mock.patch.object(
                            self.p51,
                            "load_terminal_dependencies",
                            return_value=(self.p43, self.p45, self.p47),
                        ),
                        mock.patch.object(
                            self.p51, "invoke_terminal_pulse47_once", new=broken_terminal
                        ),
                        self.assertRaises(error_type),
                    ):
                        self._run(p27_cycle_root=p27_cycle_root)
                finally:
                    self._clean_fault_residue(p27_cycle_root)

    def test_terminal_invocation_public_failures_remain_bounded(self) -> None:
        for name, error in (
            ("p43", self.p43.PublicFailure("P43-TEST-TERMINAL-INVOCATION")),
            ("p47", self.p47.WitnessFailure("P47-TEST-TERMINAL-INVOCATION")),
        ):
            with self.subTest(source=name):
                p27_cycle_root = self.runtime_root / f"p27-invocation-public-{name}"

                def public_failure(*arguments: object, _error: Exception = error):
                    del arguments
                    raise _error

                with (
                    mock.patch.object(executor, "load_pulse51", return_value=self.p51),
                    mock.patch.object(
                        self.p51,
                        "load_terminal_dependencies",
                        return_value=(self.p43, self.p45, self.p47),
                    ),
                    mock.patch.object(
                        self.p51, "invoke_terminal_pulse47_once", new=public_failure
                    ),
                ):
                    result, _runner = self._run(p27_cycle_root=p27_cycle_root)

                self._assert_invalid_publication(result)
                self._clean_fault_residue(p27_cycle_root)

    def test_terminal_cleanup_retries_a_transient_permission_failure(self) -> None:
        terminal_calls: list[tuple[object, ...]] = []
        cleanup_calls = 0
        original_remove = executor._remove_private_tree

        def terminal_failure(
            terminal: object,
            result: object,
            p43_final_root: Path,
            witness_final_root: Path,
        ) -> dict[str, object]:
            terminal_calls.append((terminal, result, p43_final_root, witness_final_root))
            return {
                "failure_code": "P47-WITNESS-FINAL-VERIFY-FAILURE",
                "schema": self.p47.SUMMARY_SCHEMA,
                "witness_publication": self._failed_posture(),
            }

        def transient_remove(p51: object, path: Path, code: str = "P52-PRIVATE-CLEANUP") -> None:
            nonlocal cleanup_calls
            if path.name == executor.TERMINAL_DIRECTORY:
                cleanup_calls += 1
                if cleanup_calls == 1:
                    raise PermissionError("transient terminal lock")
            original_remove(p51, path, code)

        with (
            mock.patch.object(executor, "load_pulse51", return_value=self.p51),
            mock.patch.object(
                self.p51, "invoke_terminal_pulse47_once", new=terminal_failure
            ),
            mock.patch.object(executor, "_remove_private_tree", new=transient_remove),
        ):
            result, _runner = self._run()

        self.assertEqual(len(terminal_calls), 1)
        self.assertEqual(cleanup_calls, 2)
        self._assert_invalid_publication(result)

    def test_terminal_cleanup_permanent_permission_failure_has_no_normal_return(self) -> None:
        terminal_calls: list[tuple[object, ...]] = []
        cleanup_calls = 0
        original_remove = executor._remove_private_tree

        def terminal_failure(
            terminal: object,
            result: object,
            p43_final_root: Path,
            witness_final_root: Path,
        ) -> dict[str, object]:
            terminal_calls.append((terminal, result, p43_final_root, witness_final_root))
            return {
                "failure_code": "P47-WITNESS-FINAL-VERIFY-FAILURE",
                "schema": self.p47.SUMMARY_SCHEMA,
                "witness_publication": self._failed_posture(),
            }

        def permanently_locked(
            p51: object, path: Path, code: str = "P52-PRIVATE-CLEANUP"
        ) -> None:
            nonlocal cleanup_calls
            if path.name == executor.TERMINAL_DIRECTORY:
                cleanup_calls += 1
                raise PermissionError("permanent terminal lock")
            original_remove(p51, path, code)

        with (
            mock.patch.object(
                self.p51, "invoke_terminal_pulse47_once", new=terminal_failure
            ),
            mock.patch.object(executor, "_remove_private_tree", new=permanently_locked),
            self.assertRaises(executor.TerminalPublicationCleanupIndeterminate) as raised,
        ):
            self._run()

        self.assertEqual(len(terminal_calls), 1)
        self.assertEqual(cleanup_calls, len(executor.TERMINAL_CLEANUP_DELAYS) + 1)
        self.assertEqual(
            raised.exception.public_posture,
            {
                "schema": executor.TERMINAL_CLEANUP_FATAL_SCHEMA,
                "state": "terminal-publication-cleanup-indeterminate",
                "cleanup_owner": "caller-public-custodian",
                "cleanup_posture": "unresolved",
            },
        )
        self.assertEqual(str(raised.exception), "terminal-publication-cleanup-indeterminate")

    def test_terminal_cleanup_programmer_faults_propagate(self) -> None:
        original_remove = executor._remove_private_tree

        def terminal_failure(
            terminal: object,
            result: object,
            p43_final_root: Path,
            witness_final_root: Path,
        ) -> dict[str, object]:
            del terminal, result, p43_final_root, witness_final_root
            return {
                "failure_code": "P47-WITNESS-FINAL-VERIFY-FAILURE",
                "schema": self.p47.SUMMARY_SCHEMA,
                "witness_publication": self._failed_posture(),
            }

        for error_type in (TypeError, AssertionError):
            with self.subTest(error=error_type.__name__):
                p27_cycle_root = self.runtime_root / f"p27-cleanup-{error_type.__name__}"

                def broken_remove(
                    p51: object,
                    path: Path,
                    code: str = "P52-PRIVATE-CLEANUP",
                    _error: type[Exception] = error_type,
                ) -> None:
                    if path.name == executor.TERMINAL_DIRECTORY:
                        raise _error("terminal cleanup programmer fault")
                    original_remove(p51, path, code)

                try:
                    with (
                        mock.patch.object(
                            executor, "load_pulse51", return_value=self.p51
                        ),
                        mock.patch.object(
                            self.p51,
                            "load_terminal_dependencies",
                            return_value=(self.p43, self.p45, self.p47),
                        ),
                        mock.patch.object(
                            self.p51, "invoke_terminal_pulse47_once", new=terminal_failure
                        ),
                        mock.patch.object(executor, "_remove_private_tree", new=broken_remove),
                        self.assertRaises(error_type),
                    ):
                        self._run(p27_cycle_root=p27_cycle_root)
                finally:
                    self._clean_fault_residue(p27_cycle_root)

    def test_same_and_nested_terminal_roots_close_out_without_republication(self) -> None:
        original_prepare = executor._prepare_terminal
        for mode in ("same", "nested"):
            with self.subTest(mode=mode):
                def overlapping_prepare(*arguments: object):
                    terminal, p43_root, witness_root, parent = original_prepare(*arguments)
                    del witness_root
                    return (
                        terminal,
                        p43_root,
                        p43_root if mode == "same" else p43_root / "nested",
                        parent,
                    )

                with mock.patch.object(
                    executor, "_prepare_terminal", side_effect=overlapping_prepare
                ):
                    result, _runner = self._run()

                self._assert_invalid_publication(result)
                self.assertEqual(result.private_record["terminal_p47_invocation_count"], 1)
                self.assertEqual(
                    result.publication["posture"]["failure_code"],
                    "P47-WITNESS-ROOTS-OVERLAP",
                )
                if self.p27_cycle_root.exists():
                    cleanup_synthetic_runtime_root(self.p27_cycle_root)

    def test_copied_prelaunch_fixture_without_verified_roots_fails_before_csprng(self) -> None:
        with mock.patch.object(
            executor.secrets,
            "token_bytes",
            side_effect=AssertionError("CSPRNG must not run before P39/P41 custody"),
        ) as token_bytes:
            result, runner = self._run(
                p39_checkout_root={
                    "classification": "ordered-execution",
                    "event_kind": "gate-complete",
                    "gate_id": "pulse-41-pulse-39-public-custody",
                    "outcome": "passed",
                    "schema": "ferris.pulse-43-ordered-result-event/v1",
                }
            )

        token_bytes.assert_not_called()
        self.assertEqual(result.private_record["outcome"], "failed")
        self.assertFalse(result.private_record["private_launch_started"])
        self.assertEqual(result.private_record["p27_invocations"], 0)
        self.assertEqual(result.private_record["materializer_invocations"], 0)
        self.assertEqual(result.private_record["verifier_invocations"], 0)
        self.assertEqual(result.private_record["seed_cleanup"], "not-created")
        self.assertEqual(result.private_record["failure_code"], "P52-P41-P39-PRELAUNCH")
        self.assertEqual(result.events[-1]["gate_id"], "pulse-41-pulse-39-public-custody")
        self.assertFalse((self.runtime_root / executor.PRIVATE_LAUNCH_DIRECTORY).exists())
        self.assertEqual(runner.dispatches, [])

    def test_p39_p41_programmer_faults_propagate_before_private_launch(self) -> None:
        for name, module, attribute in (
            ("p39", self.p39, "verify"),
            ("p41", self.p41, "copy_release"),
        ):
            for error_type in (TypeError, AssertionError):
                with self.subTest(predecessor=name, error=error_type.__name__):
                    def broken_predecessor(
                        *arguments: object, _error: type[Exception] = error_type
                    ) -> None:
                        del arguments
                        raise _error(f"{name} predecessor programmer fault")

                    with (
                        mock.patch.object(executor, "load_pulse51", return_value=self.p51),
                        mock.patch.object(
                            executor,
                            "load_p39_and_p41",
                            return_value=(self.p39, self.p41),
                        ),
                        mock.patch.object(
                            executor.secrets,
                            "token_bytes",
                            side_effect=AssertionError(
                                "CSPRNG must not run before P39/P41 custody"
                            ),
                        ) as token_bytes,
                        mock.patch.object(module, attribute, new=broken_predecessor),
                        self.assertRaises(error_type),
                    ):
                        self._run()
                    token_bytes.assert_not_called()

    def test_p39_p41_public_failures_remain_bounded_prelaunch(self) -> None:
        for name, module, attribute, error_type in (
            ("p39", self.p39, "verify", self.p39.PublicFailure),
            ("p41", self.p41, "copy_release", self.p41.PublicFailure),
        ):
            with self.subTest(predecessor=name):
                def public_failure(
                    *arguments: object, _error: type[Exception] = error_type
                ) -> None:
                    del arguments
                    raise _error("P52-TEST-PREDECESSOR-PUBLIC-FAILURE")

                with (
                    mock.patch.object(executor, "load_pulse51", return_value=self.p51),
                    mock.patch.object(
                        executor,
                        "load_p39_and_p41",
                        return_value=(self.p39, self.p41),
                    ),
                    mock.patch.object(module, attribute, new=public_failure),
                ):
                    self._assert_prelaunch_rejected()

    def test_exported_copied_fixture_fails_before_production_csprng(self) -> None:
        def fixed_binary(custody: object, expectation: object, runtime_root: Path) -> Path:
            del runtime_root
            return Path(custody.final_root) / expectation.logical_filename

        copied_fixture = {
            "classification": "ordered-execution",
            "event_kind": "gate-complete",
            "gate_id": "pulse-41-pulse-39-public-custody",
            "outcome": "passed",
            "schema": "ferris.pulse-43-ordered-result-event/v1",
        }
        with (
            mock.patch.object(self.p51, "_verify_custody_binary", new=fixed_binary),
            mock.patch.object(
                executor.secrets,
                "token_bytes",
                side_effect=AssertionError("CSPRNG must not run before P39/P41 custody"),
            ) as token_bytes,
        ):
            result = executor.run_ordered_materialization_executor(
                REPO_ROOT,
                self.runtime_root,
                self.p27_cycle_root,
                copied_fixture,
                self.sandbox / "production-copied-fixture-p41-final",
                self.custodies,
            )

        token_bytes.assert_not_called()
        self.assertEqual(result.private_record["failure_code"], "P52-P41-P39-PRELAUNCH")
        self.assertFalse(result.private_record["private_launch_started"])
        self.assertEqual(result.private_record["p27_invocations"], 0)

    def test_p39_summary_path_and_checkout_root_mutations_fail_before_p27(self) -> None:
        real_verify = self.p39.verify

        for name, mutate in (
            ("count", lambda summary: summary.__setitem__("count", 35)),
            ("path", lambda summary: summary.__setitem__("files", summary["files"][:-1])),
        ):
            with self.subTest(name=name):
                def mutated_verify(
                    checkout_root_value: str,
                    pulse_25_root: str,
                    pulse_27_root: str,
                    git: str = "git",
                ) -> dict[str, object]:
                    summary = real_verify(
                        checkout_root_value, pulse_25_root, pulse_27_root, git
                    )
                    mutate(summary)
                    return summary

                with (
                    mock.patch.object(
                        executor,
                        "load_p39_and_p41",
                        return_value=(self.p39, self.p41),
                    ),
                    mock.patch.object(self.p39, "verify", new=mutated_verify),
                ):
                    self._assert_prelaunch_rejected()

        self._assert_prelaunch_rejected(
            p39_checkout_root=self.sandbox / "missing-p39-checkout"
        )

    def test_p41_root_summary_sync_count_and_retry_mutations_fail_before_p27(self) -> None:
        existing = self.sandbox / "existing-p41-final"
        existing.mkdir()
        self._assert_prelaunch_rejected(p41_final_root=existing)

        real_copy = self.p41.copy_release
        for name, mutate in (
            (
                "sync",
                lambda summary: summary["sync"]["final_parent"].update(
                    {
                        "error_category": "sync-operation-failed",
                        "status": "failed",
                    }
                ),
            ),
            ("count", lambda summary: summary["counts"].__setitem__("final", "7/8")),
            ("retries", lambda summary: summary.__setitem__("retries", 1)),
        ):
            with self.subTest(name=name):
                def mutated_copy(
                    source_root_value: str | Path, final_root_value: str | Path
                ) -> dict[str, object]:
                    summary = real_copy(source_root_value, final_root_value)
                    mutable = json.loads(json.dumps(summary))
                    mutate(mutable)
                    return mutable

                with (
                    mock.patch.object(
                        executor,
                        "load_p39_and_p41",
                        return_value=(self.p39, self.p41),
                    ),
                    mock.patch.object(self.p41, "copy_release", new=mutated_copy),
                ):
                    self._assert_prelaunch_rejected()

    def test_p41_final_path_file_and_hash_mutations_fail_before_p27(self) -> None:
        real_copy = self.p41.copy_release

        for name, mutate in (
            (
                "path",
                lambda root: (root / "unexpected-public-file.txt").write_text(
                    "unexpected\n", encoding="utf-8"
                ),
            ),
            (
                "file-and-hash",
                lambda root: (root / "README.md").write_text(
                    "tampered\n", encoding="utf-8"
                ),
            ),
        ):
            with self.subTest(name=name):
                def corrupted_copy(
                    source_root_value: str | Path, final_root_value: str | Path
                ) -> dict[str, object]:
                    summary = real_copy(source_root_value, final_root_value)
                    mutate(Path(final_root_value))
                    return summary

                with (
                    mock.patch.object(
                        executor,
                        "load_p39_and_p41",
                        return_value=(self.p39, self.p41),
                    ),
                    mock.patch.object(self.p41, "copy_release", new=corrupted_copy),
                ):
                    self._assert_prelaunch_rejected()

    def test_p39_and_p41_sealed_receipt_mutations_fail_before_p27(self) -> None:
        original_safe_regular = sealed_dependencies._safe_regular
        releases = {
            "pulse39": "pulse-39-checkout-verifier-release",
            "pulse41": "pulse-41-transactional-copy-release",
        }
        expected_codes = {
            "pulse39": "P52-P39-IDENTITY",
            "pulse41": "P52-P41-IDENTITY",
        }
        for name, release in releases.items():
            with self.subTest(name=name):
                def corrupted_receipt(
                    path: Path, code: str, maximum: int = 4_194_304
                ) -> bytes:
                    raw = original_safe_regular(path, code, maximum)
                    if (
                        path.parent.name == release
                        and path.name == "qualification-receipt.json"
                    ):
                        return raw + b" "
                    return raw

                with mock.patch.object(
                    sealed_dependencies, "_safe_regular", new=corrupted_receipt
                ):
                    with mock.patch.object(
                        executor.secrets,
                        "token_bytes",
                        side_effect=AssertionError(
                            "CSPRNG must not run before sealed P39/P41 receipt binding"
                        ),
                    ) as token_bytes:
                        result, runner = self._run()
                token_bytes.assert_not_called()
                self.assertEqual(result.private_record["p27_invocations"], 0)
                self.assertFalse(result.private_record["private_launch_started"])
                self.assertEqual(result.private_record["failure_code"], expected_codes[name])
                self.assertEqual(runner.dispatches, [])

    def test_materializer_failure_consumes_private_launch_and_terminalizes(self) -> None:
        result, runner = self._run(force_materializer_destination_conflict=True)
        self.assertEqual(result.private_record["outcome"], "failed")
        self.assertTrue(result.private_record["private_launch_started"])
        self.assertEqual(result.private_record["materializer_invocations"], 1)
        self.assertEqual(result.private_record["verifier_invocations"], 0)
        self.assertEqual(result.private_record["failure_code"], "P52-MATERIALIZATION")
        self.assertEqual(result.events[-1]["gate_id"], "bounded-materialization")
        self.assertFalse((self.runtime_root / executor.PRIVATE_LAUNCH_DIRECTORY).exists())
        self.assertEqual(runner.dispatches, [])

    def test_production_signature_has_no_injection_surface(self) -> None:
        self.assertEqual(
            tuple(inspect.signature(executor.run_ordered_materialization_executor).parameters),
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
                "OrderedMaterializationResult",
                "TerminalPublicationCleanupIndeterminate",
                "run_ordered_materialization_executor",
            ],
        )
        forbidden = {
            "seed",
            "descriptor",
            "generator",
            "launcher",
            "runner",
            "expectations",
            "trust",
            "event",
        }
        self.assertFalse(forbidden.intersection(inspect.signature(executor.run_ordered_materialization_executor).parameters))

    def test_exported_production_path_uses_fixed_sealed_wiring(self) -> None:
        """Patch only sealed operations; call the injection-free export itself."""

        fixed_expectations: list[object] = []
        fixed_runners: list[object] = []
        p35_modules: list[tuple[object, object]] = []
        production_runner = self.p51._subprocess_process_runner

        def fixed_binary(custody: object, expectation: object, runtime_root: Path) -> Path:
            del runtime_root
            fixed_expectations.append(expectation)
            self.assertIs(
                expectation, self.p51.P33_EXPECTATIONS[custody.platform]
            )
            return Path(custody.final_root) / expectation.logical_filename

        def fixed_bridge(
            p45: object,
            repo_root: Path,
            custody: object,
            platform: str,
            runtime_root: Path,
        ) -> None:
            del p45, repo_root, custody, platform, runtime_root

        def fixed_p27(
            runtime_root: Path, cycle_root_value: Path, runner: object
        ) -> None:
            del runtime_root, cycle_root_value, runner

        def fixed_p31(repo_root: Path) -> dict[str, object]:
            del repo_root
            return {
                "artifact_count": 9,
                "positive_fixture_count": 6,
                "mutation_control_count": 33,
                "public_input_checks": 39,
            }

        def fixed_p35_custody(
            repo_root: Path, git: str = "git"
        ) -> dict[str, object]:
            del repo_root, git
            return {
                "bound_file_count": 11,
                "p35_release_tree_file_count": 10,
                "machine_schema_count": 1,
                "canonical_lf_file_count": 11,
                "git_clean_checks": 11,
            }

        def fixed_dispatch(
            descriptor: object,
            platform: str,
            executable: Path,
            runtime_root: Path,
            process_runner: object,
        ) -> dict[str, object]:
            del descriptor, platform, executable, runtime_root
            fixed_runners.append(process_runner)
            self.assertIs(process_runner, production_runner)
            return {"result": {"semantic_projection": {"fixed": "production-runner"}}}

        real_load_p35 = executor.load_p35_materializer_and_verifier

        def exact_p35(repo_root: Path) -> tuple[object, object]:
            modules = real_load_p35(repo_root)
            p35_modules.append(modules)
            self.assertEqual(modules[0].__name__, "pulse52_exact_p35_materializer")
            self.assertEqual(modules[1].__name__, "pulse52_exact_p35_verifier")
            return modules

        with (
            mock.patch.object(self.p51, "_verify_custody_binary", new=fixed_binary),
            mock.patch.object(self.p51, "_bridge_p44_once", new=fixed_bridge),
            mock.patch.object(self.p51, "_run_p27_once", new=fixed_p27),
            mock.patch.object(self.p51, "verify_bound_contract", new=fixed_p31),
            mock.patch.object(self.p51, "verify_p35_p37_custody", new=fixed_p35_custody),
            mock.patch.object(self.p51, "_run_descriptor", new=fixed_dispatch),
            mock.patch.object(
                executor, "load_p35_materializer_and_verifier", side_effect=exact_p35
            ),
            mock.patch.object(
                executor.secrets, "token_bytes", wraps=executor.secrets.token_bytes
            ) as token_bytes,
        ):
            result = executor.run_ordered_materialization_executor(
                REPO_ROOT,
                self.runtime_root,
                self.p27_cycle_root,
                REPO_ROOT,
                self.sandbox / "production-p41-final",
                self.custodies,
            )

        token_bytes.assert_called_once_with(32)
        self.assertEqual(len(fixed_expectations), 2)
        self.assertEqual(len(fixed_runners), 138)
        self.assertTrue(all(runner is production_runner for runner in fixed_runners))
        self.assertEqual(len(p35_modules), 1)
        self.assertEqual(result.private_record["p39_checkout_verifications"], 1)
        self.assertEqual(result.private_record["p41_transactional_copy_invocations"], 1)
        self.assertEqual(result.private_record["p41_post_copy_binding"], "8/8")
        self.assertEqual(result.private_record["outcome"], "published")
        self.assertEqual(result.publication["disposition"], "published")

    def test_exported_fixed_binary_failure_does_not_call_csprng(self) -> None:
        def reject_fixed_binary(
            custody: object, expectation: object, runtime_root: Path
        ) -> Path:
            del custody, expectation, runtime_root
            raise self.p51.ExecutorFailure("P52-TEST-FIXED-BINARY")

        with (
            mock.patch.object(
                self.p51, "_verify_custody_binary", new=reject_fixed_binary
            ),
            mock.patch.object(
                executor.secrets,
                "token_bytes",
                side_effect=AssertionError("CSPRNG must not run before fixed binary custody"),
            ) as token_bytes,
        ):
            result = executor.run_ordered_materialization_executor(
                REPO_ROOT,
                self.runtime_root,
                self.p27_cycle_root,
                REPO_ROOT,
                self.sandbox / "binary-failure-p41-final",
                self.custodies,
            )

        token_bytes.assert_not_called()
        self.assertEqual(result.private_record["outcome"], "failed")
        self.assertEqual(result.private_record["execution_outcome"], "failed")
        self.assertEqual(result.private_record["failure_code"], "P52-TEST-FIXED-BINARY")
        self.assertEqual(result.publication["disposition"], "not-attempted")

    def test_exact_pulse39_pulse41_pulse51_identities_and_signatures_are_bound(self) -> None:
        self.assertEqual(
            PULSE39_MANIFEST_RAW_SHA256,
            "sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c",
        )
        self.assertEqual(
            PULSE39_SOURCE_SHA256,
            "sha256:783283fd127170460ce52106a7a1158054cdc2608475e53899ff45a7a6a31d12",
        )
        self.assertEqual(
            PULSE41_MANIFEST_RAW_SHA256,
            "sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8",
        )
        self.assertEqual(
            PULSE41_SOURCE_SHA256,
            "sha256:900a89de3401f78558970d896214568f851ca644def28639476e66154235c8cf",
        )
        self.assertTrue(PULSE39_RECEIPT_RAW_SHA256.startswith("sha256:"))
        self.assertTrue(PULSE39_SEAL_RAW_SHA256.startswith("sha256:"))
        self.assertTrue(PULSE41_RECEIPT_RAW_SHA256.startswith("sha256:"))
        self.assertTrue(PULSE41_SEAL_RAW_SHA256.startswith("sha256:"))
        self.assertEqual(
            tuple(inspect.signature(self.p39.verify).parameters),
            ("checkout_root_value", "pulse_25_root", "pulse_27_root", "git"),
        )
        self.assertEqual(
            tuple(inspect.signature(self.p41.copy_release).parameters),
            (
                "source_root_value",
                "final_root_value",
                "synchronizer",
                "copier",
                "renamer",
                "remover",
                "post_rename",
            ),
        )
        self.assertEqual(
            PULSE51_MANIFEST_RAW_SHA256,
            "sha256:5799416be367b9293c85fbbc3db0a340184045a1a33e75c9d7a4351d5741efbc",
        )
        self.assertEqual(
            PULSE51_SOURCE_SHA256,
            "sha256:97c404dbf29d387561878772403c7fbd2672e97283b0620e838e7126ecbdd637",
        )
        self.assertEqual(
            tuple(inspect.signature(self.p51.run_diagnostic_executor).parameters),
            (
                "repo_root",
                "descriptor_root",
                "private_runtime_root",
                "p27_cycle_root",
                "retained_custodies",
            ),
        )
        self.assertEqual(self.p51.P50_GATE_IDS[6], "bounded-materialization")
        self.assertEqual(self.p51.P50_GATE_IDS[7], "bounded-process-exit-search")

    def test_runtime_and_terminal_roots_are_one_use(self) -> None:
        result, _runner = self._run()
        self.assertEqual(result.private_record["terminal_p47_invocation_count"], 1)
        second, second_runner = self._run()
        self.assertEqual(second.private_record["outcome"], "failed")
        self.assertFalse(second.private_record["private_launch_started"])
        self.assertEqual(second.private_record["failure_code"], "P52-P27-CYCLE-ROOT")
        self.assertEqual(second_runner.dispatches, [])


if __name__ == "__main__":
    unittest.main()
