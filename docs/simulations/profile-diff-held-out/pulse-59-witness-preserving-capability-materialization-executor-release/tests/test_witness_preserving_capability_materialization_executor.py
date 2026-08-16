from __future__ import annotations

import atexit
import importlib.util
import inspect
import json
import os
import shutil
import sys
import types
import unittest
import uuid
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = ROOT.parents[3]
RUN_ROOT = REPO_ROOT / "target" / "pulse-59-test-runtime"
_LOADED_TEST_MODULES: set[str] = set()
sys.dont_write_bytecode = True
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import generate_release  # noqa: E402
import qualify  # noqa: E402
import witness_preserving_capability_materialization_executor as executor  # noqa: E402
from sealed_dependencies import P58_COMMIT, P58_RELEASE_ROOT, load_pulse58  # noqa: E402


def _clean_release_python_residue() -> None:
    for path in sorted(
        ROOT.rglob("__pycache__"), key=lambda item: len(item.parts), reverse=True
    ):
        shutil.rmtree(path, ignore_errors=True)


def _clean_loaded_test_modules() -> None:
    for name in tuple(_LOADED_TEST_MODULES):
        sys.modules.pop(name, None)
        _LOADED_TEST_MODULES.discard(name)


_clean_release_python_residue()
_clean_loaded_test_modules()
atexit.register(_clean_release_python_residue)
atexit.register(_clean_loaded_test_modules)


def _public_result_text(result: object) -> str:
    return repr(
        {
            "catalog": result.catalog,
            "events": result.events,
            "publication": result.publication,
            "transfer_descriptor": result.transfer_descriptor,
        }
    )


def _load_fresh_release_module(relative: str) -> object:
    path = ROOT / relative
    name = f"pulse59_test_{path.stem}_{uuid.uuid4().hex}"
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise AssertionError(f"missing module spec for {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    _LOADED_TEST_MODULES.add(name)
    spec.loader.exec_module(module)
    return module


class WitnessPreservingCapabilityMaterializationExecutorTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        _clean_release_python_residue()
        _clean_loaded_test_modules()
        if RUN_ROOT.exists():
            shutil.rmtree(RUN_ROOT, ignore_errors=True)
        RUN_ROOT.mkdir(parents=True)

    @classmethod
    def tearDownClass(cls) -> None:
        if RUN_ROOT.exists():
            shutil.rmtree(RUN_ROOT, ignore_errors=True)
        _clean_loaded_test_modules()
        _clean_release_python_residue()

    def setUp(self) -> None:
        self.sandbox = RUN_ROOT / uuid.uuid4().hex
        self.sandbox.mkdir()
        self.p58, self.p52, self.p57, self.p51, self.p43, self.p47 = load_pulse58(
            REPO_ROOT
        )

    def tearDown(self) -> None:
        if self.sandbox.exists():
            shutil.rmtree(self.sandbox, ignore_errors=True)

    def _sealed_stub(self, modules: tuple[object, ...] | None = None) -> object:
        class StubSealedDependencyFailure(RuntimeError):
            pass

        bound = (
            modules
            if modules is not None
            else (self.p58, self.p52, self.p57, self.p51, self.p43, self.p47)
        )
        return types.SimpleNamespace(
            SealedDependencyFailure=StubSealedDependencyFailure,
            load_pulse58=lambda _repo_root: bound,
        )

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
            sync = {
                name: self._sync("not-attempted")
                for name in ("final_parent", "rollback_parent", "stage")
            }
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

        def terminal(terminal_object: object, result: object, p43_root: Path, witness_root: Path):
            calls.append((terminal_object, p43_root, witness_root))
            return self.p47.witness_pulse_43(
                result.catalog,
                result.events,
                p43_root,
                witness_root,
                invoker=lambda *_arguments: failure,
            )

        return terminal

    def _run(
        self,
        name: str,
        *,
        cycle: int = 1,
        p27_runner: object = qualify._p27_success,
        on_launch: object = None,
        terminal_call: object = None,
    ) -> tuple[object, qualify.FakeP56, list[qualify._InProcessWsl], Path, Path]:
        root = self.sandbox / name
        root.mkdir()
        runtime = root / "runtime"
        runtime.mkdir()
        artifact = qualify._fake_release(root, "alpha" if cycle % 2 else "beta")
        fake = qualify.FakeP56(
            artifact,
            on_launch=on_launch,
            runner=qualify.QualificationProcessRunner(runtime, self.p51),
            runtime_root=runtime,
            p51=self.p51,
        )
        sessions: list[qualify._InProcessWsl] = []

        def open_wsl(_repo: Path, _parent: str, api: object) -> qualify._InProcessWsl:
            session = qualify._InProcessWsl(fake, api)
            sessions.append(session)
            session.bind(fake.publish_retained_build_and_custody("ubuntu-24.04-x86_64", root))
            return session

        p39_checkout = qualify._synthetic_p39_checkout(root)
        with patch.object(
            executor,
            "_load_local_sealed_dependencies",
            return_value=self._sealed_stub(),
        ):
            result = executor._run_qualification_executor(
                REPO_ROOT,
                runtime,
                runtime / "p27-cycle",
                p39_checkout,
                root / "p41-public-custody",
                seed_bytes=qualify.synthetic_seed(cycle),
                p27_runner=p27_runner,
                p56=fake,
                open_wsl=open_wsl,
                terminal_call=terminal_call,
            )
        terminal_root = root / f"runtime{executor.TERMINAL_ROOT_SUFFIX}"
        return result, fake, sessions, runtime, terminal_root

    def _assert_completed_execution(
        self,
        result: object,
        fake: qualify.FakeP56,
        sessions: list[qualify._InProcessWsl],
        runtime: Path,
        terminal_root: Path,
        *,
        terminal_root_exists: bool = True,
    ) -> None:
        self.assertEqual(result.private_record["p58_execution_outcome"], "completed")
        self.assertEqual(result.private_record["process_counts"], {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69})
        self.assertEqual(len(result.private_record["no_launch_records"]), 2)
        self.assertEqual(fake.publishes, ["windows-x86_64", "ubuntu-24.04-x86_64"])
        self.assertEqual(len(fake.launches), 138)
        self.assertEqual(len(sessions), 1)
        self.assertEqual(len(sessions[0].requests), 69)
        self.assertFalse(runtime.exists())
        self.assertEqual(terminal_root.is_dir(), terminal_root_exists)
        self.assertTrue(result.private_record["terminal_runtime_absence_verified"])
        self.assertEqual(result.private_record["terminal_p47_invocation_count"], 1)
        self.assertEqual(result.events[-1]["event_kind"], "terminal-stop")
        self.assertEqual(result.events[-1]["outcome"], "completed")

    def test_sealed_binding_and_production_signature_match_pulse58(self) -> None:
        self.assertEqual(P58_COMMIT, "7c66d70800edd06642274ed4f2e4aee224b7583e")
        self.assertEqual(
            P58_RELEASE_ROOT,
            "docs/simulations/profile-diff-held-out/pulse-58-ordered-capability-materialization-executor-release",
        )
        self.assertEqual(
            tuple(
                inspect.signature(
                    executor.run_witness_preserving_capability_materialization_executor
                ).parameters
            ),
            (
                "repo_root",
                "private_runtime_root",
                "p27_cycle_root",
                "p39_checkout_root",
                "p41_final_root",
                "ubuntu_runtime_parent",
            ),
        )
        self.assertEqual(
            executor.__all__,
            [
                "P59Failure",
                "TerminalPublicationCleanupIndeterminate",
                "WitnessPreservingCapabilityMaterializationResult",
                "run_witness_preserving_capability_materialization_executor",
            ],
        )
        source = (ROOT / "witness_preserving_capability_materialization_executor.py").read_text(
            encoding="utf-8"
        )
        self.assertNotIn("from sealed_dependencies import", source)
        self.assertIn("def _load_local_sealed_dependencies()", source)
        sealed_source = (ROOT / "sealed_dependencies.py").read_text(encoding="utf-8")
        self.assertNotIn("_P58_MODULE", sealed_source)

    def test_production_surface_rejects_injection(self) -> None:
        with self.assertRaises(TypeError):
            executor.run_witness_preserving_capability_materialization_executor(
                REPO_ROOT,
                self.sandbox / "runtime",
                self.sandbox / "cycle",
                self.sandbox / "p39",
                self.sandbox / "p41",
                "/home/pulse59",
                seed=b"x" * 32,
            )

    def test_executor_import_ignores_external_sealed_dependencies(self) -> None:
        attacker = self.sandbox / "attacker"
        attacker.mkdir()
        attacker_file = attacker / "sealed_dependencies.py"
        attacker_file.write_text(
            "raise RuntimeError('P59 attack path resolution succeeded')\n",
            encoding="utf-8",
            newline="\n",
        )

        def hijacked(_repo_root: Path) -> object:
            raise AssertionError("P59 attack sys.modules resolution succeeded")

        malicious = types.SimpleNamespace(
            P58_GATE_IDS=("hijacked",),
            SealedDependencyFailure=RuntimeError,
            load_pulse58=hijacked,
            __file__=os.fspath(attacker_file),
        )
        previous = sys.modules.get("sealed_dependencies")
        sys.modules["sealed_dependencies"] = malicious
        sys.path.insert(0, os.fspath(attacker))
        try:
            fresh_executor = _load_fresh_release_module(
                "witness_preserving_capability_materialization_executor.py"
            )
            sealed = fresh_executor._load_local_sealed_dependencies()
            self.assertEqual(
                fresh_executor._local_sealed_dependencies_path().resolve(),
                (ROOT / "sealed_dependencies.py").resolve(),
            )
            self.assertEqual(
                Path(sealed.__file__).resolve(),
                (ROOT / "sealed_dependencies.py").resolve(),
            )
            p58, _p52, _p57, _p51, _p43, _p47 = sealed.load_pulse58(REPO_ROOT)
        finally:
            sys.path.remove(os.fspath(attacker))
            if previous is None:
                sys.modules.pop("sealed_dependencies", None)
            else:
                sys.modules["sealed_dependencies"] = previous
        self.assertTrue(callable(p58.run_ordered_capability_materialization_executor))
        self.assertEqual(tuple(p58.P58_GATE_IDS), executor.P58_GATE_IDS)

    def test_sealed_dependency_loader_ignores_cache_preseed_and_mutation(self) -> None:
        fresh = _load_fresh_release_module("sealed_dependencies.py")
        sentinels = tuple(types.SimpleNamespace(marker=index) for index in range(6))
        for name, sentinel in zip(
            (
                "_P58_MODULE",
                "_P52_MODULE",
                "_P57_MODULE",
                "_P51_MODULE",
                "_P43_MODULE",
                "_P47_MODULE",
            ),
            sentinels,
        ):
            setattr(fresh, name, sentinel)
        first = fresh.load_pulse58(REPO_ROOT)
        self.assertEqual(len(first), 6)
        for actual, sentinel in zip(first, sentinels):
            self.assertIsNot(actual, sentinel)
        first[0].P58_GATE_IDS = ("tampered",)
        first[3].TerminalPulse47Once = object()
        second = fresh.load_pulse58(REPO_ROOT)
        self.assertIsNot(second[0], first[0])
        self.assertEqual(tuple(second[0].P58_GATE_IDS), executor.P58_GATE_IDS)
        self.assertIsInstance(second[3].TerminalPulse47Once, type)

    def test_qualification_delegates_to_exact_p58_executor(self) -> None:
        with patch.object(
            self.p58,
            "_run_qualification_executor",
            wraps=self.p58._run_qualification_executor,
        ) as wrapped:
            result, fake, sessions, runtime, terminal_root = self._run("delegates")
        wrapped.assert_called_once()
        self._assert_completed_execution(result, fake, sessions, runtime, terminal_root)
        self.assertEqual(result.publication["disposition"], "published-result")

    def test_published_result_retains_verified_result_and_witness_after_p58_cleanup(self) -> None:
        result, fake, sessions, runtime, terminal_root = self._run("published")
        self._assert_completed_execution(result, fake, sessions, runtime, terminal_root)
        self.assertEqual(result.publication["disposition"], "published-result")
        self.assertEqual(result.private_record["outcome"], "published-result")
        self.assertEqual(result.private_record["terminal_publication_cleanup"], "retained-published-result")
        self.assertEqual(result.transfer_descriptor["expected_public_tree_kind"], "result-and-witness")
        self.assertEqual(result.transfer_descriptor["exact_file_counts"], {"result": 2, "witness": 2, "total": 4})
        self.assertEqual(sorted(entry.name for entry in terminal_root.iterdir()), [executor.P43_FINAL_DIRECTORY, executor.WITNESS_FINAL_DIRECTORY])
        self.assertTrue((terminal_root / executor.P43_FINAL_DIRECTORY / "public-result.json").is_file())
        self.assertTrue((terminal_root / executor.WITNESS_FINAL_DIRECTORY / "publication-witness.json").is_file())

    def test_absent_p43_failure_is_retained_as_failure_witness(self) -> None:
        calls: list[object] = []
        result, fake, sessions, runtime, terminal_root = self._run(
            "absent-failure",
            cycle=2,
            terminal_call=self._witnessed_failure_terminal("absent", calls),
        )
        self._assert_completed_execution(result, fake, sessions, runtime, terminal_root)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result.publication["disposition"], "published-failure-witness")
        self.assertEqual(result.publication["posture"]["publication"]["state"], "absent")
        self.assertEqual(sorted(entry.name for entry in terminal_root.iterdir()), [executor.WITNESS_FINAL_DIRECTORY])

    def test_rolled_back_p43_failure_is_retained_as_failure_witness(self) -> None:
        calls: list[object] = []
        result, fake, sessions, runtime, terminal_root = self._run(
            "rolled-back-failure",
            cycle=4,
            terminal_call=self._witnessed_failure_terminal("rolled-back", calls),
        )
        self._assert_completed_execution(result, fake, sessions, runtime, terminal_root)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result.publication["disposition"], "published-failure-witness")
        self.assertEqual(result.publication["posture"]["publication"]["state"], "rolled-back")

    def test_indeterminate_p43_failure_is_retained_as_failure_witness(self) -> None:
        calls: list[object] = []
        result, fake, sessions, runtime, terminal_root = self._run(
            "indeterminate-failure",
            cycle=6,
            terminal_call=self._witnessed_failure_terminal("indeterminate", calls),
        )
        self._assert_completed_execution(result, fake, sessions, runtime, terminal_root)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result.publication["disposition"], "published-failure-witness")
        self.assertEqual(result.publication["posture"]["publication"]["state"], "indeterminate")

    def test_malformed_hash_mismatch_and_result_residue_clean_invalid_publication(self) -> None:
        failure = self._p43_failure("absent")

        def malformed(_terminal: object, _result: object, _p43_root: Path, _witness_root: Path):
            return {"schema": self.p47.SUMMARY_SCHEMA}

        def mismatched(_terminal: object, result: object, p43_root: Path, witness_root: Path):
            summary = self.p47.witness_pulse_43(
                result.catalog,
                result.events,
                p43_root,
                witness_root,
                invoker=lambda *_arguments: failure,
            )
            summary["witness_publication"]["raw_hashes"]["witness_raw_sha256"] = "sha256:" + "f" * 64
            return summary

        def residue(_terminal: object, result: object, p43_root: Path, witness_root: Path):
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

        for name, terminal in (
            ("malformed-summary", malformed),
            ("witness-hash-mismatch", mismatched),
            ("p43-residue", residue),
        ):
            with self.subTest(case=name):
                result, fake, sessions, runtime, terminal_root = self._run(
                    name, cycle=8, terminal_call=terminal
                )
                self._assert_completed_execution(
                    result,
                    fake,
                    sessions,
                    runtime,
                    terminal_root,
                    terminal_root_exists=False,
                )
                self.assertEqual(result.publication["disposition"], "invalid-witness-publication")
                self.assertIsNone(result.transfer_descriptor)
                self.assertEqual(result.private_record["terminal_publication_cleanup"], "removed-and-verified")
                self.assertFalse(terminal_root.exists())

    def test_terminal_seam_is_called_once_without_retry(self) -> None:
        calls: list[object] = []
        result, fake, sessions, runtime, terminal_root = self._run(
            "single-terminal-call",
            cycle=10,
            terminal_call=self._witnessed_failure_terminal("absent", calls),
        )
        self._assert_completed_execution(result, fake, sessions, runtime, terminal_root)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result.private_record["terminal_p47_invocation_count"], 1)

    def test_transfer_descriptor_is_path_free(self) -> None:
        success, _fake, _sessions, _runtime, success_terminal_root = self._run("path-free-success")
        failure_calls: list[object] = []
        failure, _fake2, _sessions2, _runtime2, failure_terminal_root = self._run(
            "path-free-failure",
            cycle=12,
            terminal_call=self._witnessed_failure_terminal("rolled-back", failure_calls),
        )
        for result, terminal_root in (
            (success, success_terminal_root),
            (failure, failure_terminal_root),
        ):
            with self.subTest(disposition=result.publication["disposition"]):
                text = json.dumps(result.transfer_descriptor, sort_keys=True)
                public = _public_result_text(result)
                for forbidden in (
                    str(terminal_root),
                    executor.P43_FINAL_DIRECTORY,
                    executor.WITNESS_FINAL_DIRECTORY,
                    "private_record",
                    "case_id",
                    "seed",
                ):
                    self.assertNotIn(forbidden, text)
                    self.assertNotIn(forbidden, public)

    def test_p58_prelaunch_failure_remains_publication_not_attempted(self) -> None:
        p39, p41 = self.p58.load_exact_p39_and_p41(REPO_ROOT)

        def fail(*_args: object) -> object:
            raise p39.PublicFailure("P39-TEST-FAILURE")

        with (
            patch.object(self.p58, "load_exact_p39_and_p41", return_value=(p39, p41)),
            patch.object(p39, "verify", new=fail),
        ):
            result, fake, _sessions, runtime, terminal_root = self._run("p39-failure")
        self.assertEqual(result.private_record["failure_code"], "P52-P41-P39-PRELAUNCH")
        self.assertEqual(result.publication["disposition"], "not-attempted")
        self.assertEqual(result.private_record["publication_disposition"], "not-attempted")
        self.assertFalse(runtime.exists())
        self.assertFalse(terminal_root.exists())
        self.assertEqual(fake.launches, [])

    def test_cleanup_indeterminate_takes_precedence(self) -> None:
        failure = self._p43_failure("absent")
        expected_terminal = self.sandbox / "cleanup-indeterminate" / f"runtime{executor.TERMINAL_ROOT_SUFFIX}"
        original_remove = self.p52._remove_private_tree

        def invalid_summary(
            _terminal: object, result: object, p43_root: Path, witness_root: Path
        ) -> object:
            return self.p47.witness_pulse_43(
                result.catalog,
                result.events,
                p43_root,
                witness_root,
                invoker=lambda *_arguments: failure,
            ) | {"outcome": "malformed"}

        def locked_remove(p51: object, path: Path, code: str = "P52-PRIVATE-CLEANUP") -> None:
            if path == expected_terminal:
                raise PermissionError("terminal locked")
            original_remove(p51, path, code)

        with (
            patch.object(self.p52, "_remove_private_tree", new=locked_remove),
            patch.object(self.p52.time, "sleep", return_value=None),
            self.assertRaises(executor.TerminalPublicationCleanupIndeterminate) as raised,
        ):
            self._run("cleanup-indeterminate", cycle=14, terminal_call=invalid_summary)
        self.assertEqual(str(raised.exception), "terminal-publication-cleanup-indeterminate")
        self.assertEqual(raised.exception.public_posture["cleanup_posture"], "unresolved")
        if expected_terminal.exists():
            shutil.rmtree(expected_terminal, ignore_errors=True)

    def test_preexisting_terminal_root_is_rejected_before_p58_execution(self) -> None:
        root = self.sandbox / "preexisting-terminal"
        root.mkdir()
        runtime = root / "runtime"
        runtime.mkdir()
        terminal_root = root / f"runtime{executor.TERMINAL_ROOT_SUFFIX}"
        terminal_root.mkdir()
        artifact = qualify._fake_release(root, "alpha")
        fake = qualify.FakeP56(
            artifact,
            runner=qualify.QualificationProcessRunner(runtime, self.p51),
            runtime_root=runtime,
            p51=self.p51,
        )

        def open_wsl(_repo: Path, _parent: str, _api: object) -> object:
            raise AssertionError("P58 must not build WSL capability when terminal root exists")

        with (
            patch.object(
                executor,
                "_load_local_sealed_dependencies",
                return_value=self._sealed_stub(),
            ),
            patch.object(
                self.p58,
                "_run_qualification_executor",
                side_effect=AssertionError("P58 must not run"),
            ),
        ):
            result = executor._run_qualification_executor(
                REPO_ROOT,
                runtime,
                runtime / "p27-cycle",
                qualify._synthetic_p39_checkout(root),
                root / "p41-public-custody",
                seed_bytes=qualify.synthetic_seed(1),
                p27_runner=qualify._p27_success,
                p56=fake,
                open_wsl=open_wsl,
            )
        self.assertEqual(result.private_record["failure_code"], "P59-TERMINAL-ROOT-NOT-FRESH")
        self.assertEqual(result.publication["disposition"], "not-attempted")
        self.assertEqual(fake.launches, [])

    def test_release_generator_rejects_python_cache_residue(self) -> None:
        residue = ROOT / "__pycache__"
        residue.mkdir(exist_ok=True)
        cache = residue / "p59-control.pyc"
        cache.write_bytes(b"not-a-bytecode-cache")
        try:
            with self.assertRaisesRegex(RuntimeError, "Python cache residue"):
                generate_release.public_files()
        finally:
            cache.unlink(missing_ok=True)
            residue.rmdir()


if __name__ == "__main__":
    unittest.main()
