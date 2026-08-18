from __future__ import annotations

import atexit
import contextlib
import contextvars
import hashlib
import importlib.util
import inspect
import json
import os
import shutil
import subprocess
import sys
import textwrap
import threading
import time
import types
import unittest
import uuid
from pathlib import Path
from typing import Mapping
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = ROOT.parents[3]
RUN_ROOT = REPO_ROOT / "target" / f"pulse-88-test-runtime-{os.getpid()}"
_LOADED_TEST_MODULES: set[str] = set()
_DYNAMIC_RELEASE_PREFIXES = (
    "p88_exact_",
    "p87_exact_",
    "p86_exact_",
    "p78_exact_",
    "p56_exact_",
    "p52_exact_",
    "p51_exact_",
    "p47_exact_",
    "p45_exact_",
    "p43_exact_",
    "p41_exact_",
    "p39_exact_",
    "p35_exact_",
)
_LOCAL_SEALED_MODULE_PREFIX = "_ferris_p88_local_sealed_dependencies_v1_"
sys.dont_write_bytecode = True
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import generate_release  # noqa: E402
import qualify  # noqa: E402
import witness_preserving_capability_materialization_executor as executor  # noqa: E402
from sealed_dependencies import P87_COMMIT, P87_RELEASE_ROOT, load_pulse87  # noqa: E402

executor._bind_local_sealed_lock_manager_module(sys.modules["sealed_dependencies"])


def _clean_release_python_residue() -> None:
    for path in sorted(
        ROOT.rglob("__pycache__"), key=lambda item: len(item.parts), reverse=True
    ):
        shutil.rmtree(path, ignore_errors=True)


def _clean_loaded_test_modules() -> None:
    for name in tuple(_LOADED_TEST_MODULES):
        sys.modules.pop(name, None)
        _LOADED_TEST_MODULES.discard(name)
    for name in tuple(sys.modules):
        if name.startswith(_DYNAMIC_RELEASE_PREFIXES) or name.startswith(
            (
                _LOCAL_SEALED_MODULE_PREFIX,
                "ferris.pulse-88.local-sealed-dependencies.runtime",
            )
        ):
            sys.modules.pop(name, None)


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
    name = f"pulse88_test_{path.stem}_{uuid.uuid4().hex}"
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise AssertionError(f"missing module spec for {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    _LOADED_TEST_MODULES.add(name)
    spec.loader.exec_module(module)
    return module


def _old_private_binder_key(module: object) -> str:
    path = module._local_sealed_dependencies_path()
    return (
        f"{_LOCAL_SEALED_MODULE_PREFIX}"
        f"{hashlib.sha256(os.fsencode(os.fspath(path))).hexdigest()}"
    )


def _old_registry_key(module: object) -> str:
    return f"{_old_private_binder_key(module)}_registry"


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
        self.repo_root = qualify.exact_repo_root()
        self.p87, self.p52, self.p86, self.p51, self.p43, self.p47 = load_pulse87(
            self.repo_root
        )

    def tearDown(self) -> None:
        if self.sandbox.exists():
            shutil.rmtree(self.sandbox, ignore_errors=True)

    def _write_subprocess_script(self, name: str, body: str) -> Path:
        script = self.sandbox / name
        script.write_text(
            "\n".join(
                (
                    "from __future__ import annotations",
                    "",
                    "import json",
                    "import os",
                    "import sys",
                    "import threading",
                    "import time",
                    "from pathlib import Path",
                    "",
                    f"ROOT = Path({os.fspath(ROOT)!r})",
                    "if str(ROOT) not in sys.path:",
                    "    sys.path.insert(0, str(ROOT))",
                    "",
                    "import witness_preserving_capability_materialization_executor as executor",
                    "",
                    textwrap.dedent(body).strip(),
                    "",
                )
            ),
            encoding="utf-8",
            newline="\n",
        )
        return script

    def _run_subprocess_script(
        self, script: Path, *arguments: str, timeout: int = 30
    ) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, "-B", os.fspath(script), *arguments],
            capture_output=True,
            check=False,
            cwd=ROOT,
            encoding="utf-8",
            env=self._subprocess_environment(),
            timeout=timeout,
        )

    def _subprocess_environment(self) -> dict[str, str]:
        environment = os.environ.copy()
        environment["PYTHONDONTWRITEBYTECODE"] = "1"
        pythonpath = environment.get("PYTHONPATH")
        environment["PYTHONPATH"] = (
            os.fspath(ROOT)
            if not pythonpath
            else os.pathsep.join((os.fspath(ROOT), pythonpath))
        )
        return environment

    def _sealed_stub(self, modules: tuple[object, ...] | None = None) -> object:
        class StubSealedDependencyFailure(RuntimeError):
            pass

        bound = (
            modules
            if modules is not None
            else (self.p87, self.p52, self.p86, self.p51, self.p43, self.p47)
        )
        return types.SimpleNamespace(
            SealedDependencyFailure=StubSealedDependencyFailure,
            load_pulse87=lambda _repo_root: bound,
        )

    def _assert_exact_loaded_stack(self, modules: tuple[object, ...]) -> None:
        p87, p52, p86, p51, p43, p47 = modules
        self.assertEqual(tuple(p87.P58_GATE_IDS), executor.P58_GATE_IDS)
        self.assertTrue(callable(p87.run_ordered_capability_materialization_executor))
        self.assertTrue(callable(p52._cleanup_terminal_publication))
        self.assertTrue(callable(p86.run_capability_bound_diagnostic_executor))
        self.assertIsInstance(p51.TerminalPulse47Once, type)
        self.assertTrue(callable(p43.verify_publication_directory))
        self.assertTrue(callable(p47.verify_witness_directory))

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
        p86_stack: tuple[object, object, object] | None = None,
        wsl_failure: BaseException | None = None,
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
            if wsl_failure is not None:
                raise wsl_failure
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
            with (
                patch.object(
                    self.p87,
                    "load_exact_p86_stack",
                    return_value=p86_stack,
                )
                if p86_stack is not None
                else contextlib.nullcontext()
            ):
                result = executor._run_qualification_executor(
                    self.repo_root,
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
        self.assertEqual(result.private_record["p87_execution_outcome"], "completed")
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

    def test_sealed_binding_and_production_signature_match_pulse87(self) -> None:
        self.assertEqual(P87_COMMIT, "efa98a4e5b2fc138458c1ead45dbb7796cf00290")
        self.assertEqual(
            P87_RELEASE_ROOT,
            "docs/simulations/profile-diff-held-out/pulse-87-ordered-capability-materialization-executor-wsl-parent-owner-binding-successor-release",
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
                "P88Failure",
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
        self.assertIn("LOCAL_SEALED_DEPENDENCIES_SHA256", source)
        self.assertIn("class _Pulse88LinuxLockManager", source)
        self.assertIn("register_at_fork", source)
        self.assertIn("_CROSS_INSTANCE_REENTRY_STATE_KEY", source)
        self.assertIn("advisory_conflict", source)
        self.assertIn("_bind_local_sealed_lock_manager_module", source)
        self.assertNotIn("_LOCAL_SEALED_MODULE_PREFIX", source)
        self.assertNotIn("_process_local_sealed_registry", source)
        sealed_source = (ROOT / "sealed_dependencies.py").read_text(encoding="utf-8")
        self.assertNotIn("_P58_MODULE", sealed_source)
        self.assertIn("CreateMutexW", sealed_source)
        self.assertIn("WaitForSingleObject", sealed_source)
        self.assertIn("socket.AF_UNIX", sealed_source)
        self.assertIn('_KERNEL_LOCK_NAMESPACE_PREFIX = "ferris-p88"', sealed_source)
        self.assertIn('return f"\\0{_KERNEL_LOCK_NAMESPACE_PREFIX}-{value}"', sealed_source)
        self.assertIn("_kernel_lock_name", sealed_source)
        self.assertIn("_current_pid", sealed_source)
        self.assertIn("_current_thread_id", sealed_source)
        self.assertIn("_ACTIVE_SEALED_LOADING_LOCK", sealed_source)
        self.assertIn("_normalize_active_loading_lock", sealed_source)
        self.assertIn("owner_thread_id", sealed_source)
        self.assertIn("owner_token.live = False", sealed_source)
        self.assertIn("_P88_INTERNAL_LOCK_MANAGER", sealed_source)
        self.assertIn("_bind_internal_lock_manager", sealed_source)
        self.assertIn("P88-SEALED-LOCK-CROSS-INSTANCE-REENTRY", sealed_source)
        self.assertIn("_WINDOWS_WAIT_ABANDONED", sealed_source)
        self.assertNotIn("register_at_fork", sealed_source)
        self.assertNotIn("pulse-88-sealed-loader-locks", sealed_source)
        self.assertNotIn("_lock_file_path", sealed_source)
        self.assertNotIn("sem_open", sealed_source)
        self.assertNotIn("threading.RLock", sealed_source)
        p87_source = (
            self.repo_root
            / "docs"
            / "simulations"
            / "profile-diff-held-out"
            / P87_RELEASE_ROOT.removeprefix(
                "docs/simulations/profile-diff-held-out/"
            )
            / "ordered_capability_materialization_executor.py"
        ).read_text(encoding="utf-8")
        self.assertIn("load_exact_p86_stack", p87_source)
        p86_source = Path(self.p86.__file__).read_text(encoding="utf-8")
        self.assertIn('"--user"', p86_source)
        self.assertIn("P86-WSL-OWNER", p86_source)
        self.assertIn("P86-INDETERMINATE-STAGE-CLEANUP", p86_source)

    def test_p86_stage_indeterminate_remains_publication_not_attempted(self) -> None:
        p56 = self.p86.load_exact_p56(self.repo_root)
        result, fake, sessions, runtime, terminal_root = self._run(
            "p86-stage-indeterminate",
            p86_stack=(self.p86, self.p51, p56),
            wsl_failure=self.p86.ExecutorFailure(
                "P86-INDETERMINATE-STAGE-CLEANUP"
            ),
        )
        self.assertEqual(
            result.private_record["failure_code"],
            "P86-INDETERMINATE-STAGE-CLEANUP",
        )
        self.assertEqual(result.private_record["p87_execution_outcome"], "failed")
        self.assertEqual(result.private_record["seed_calls"], 0)
        self.assertEqual(result.publication["disposition"], "not-attempted")
        self.assertIsNone(result.transfer_descriptor)
        self.assertEqual(sessions, [])
        self.assertFalse(runtime.exists())
        self.assertFalse(terminal_root.exists())
        self.assertEqual(len(fake.launches), 0)

    def test_production_surface_rejects_injection(self) -> None:
        with self.assertRaises(TypeError):
            executor.run_witness_preserving_capability_materialization_executor(
                self.repo_root,
                self.sandbox / "runtime",
                self.sandbox / "cycle",
                self.sandbox / "p39",
                self.sandbox / "p41",
                "/home/pulse88",
                seed=b"x" * 32,
            )

    def test_executor_import_ignores_external_sealed_dependencies(self) -> None:
        attacker = self.sandbox / "attacker"
        attacker.mkdir()
        attacker_file = attacker / "sealed_dependencies.py"
        attacker_file.write_text(
            "raise RuntimeError('P88 attack path resolution succeeded')\n",
            encoding="utf-8",
            newline="\n",
        )

        def hijacked(_repo_root: Path) -> object:
            raise AssertionError("P88 attack sys.modules resolution succeeded")

        malicious = types.SimpleNamespace(
            P58_GATE_IDS=("hijacked",),
            SealedDependencyFailure=RuntimeError,
            load_pulse87=hijacked,
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
            p87, _p52, _p86, _p51, _p43, _p47 = sealed.load_pulse87(self.repo_root)
        finally:
            sys.path.remove(os.fspath(attacker))
            if previous is None:
                sys.modules.pop("sealed_dependencies", None)
            else:
                sys.modules["sealed_dependencies"] = previous
        self.assertTrue(callable(p87.run_ordered_capability_materialization_executor))
        self.assertEqual(tuple(p87.P58_GATE_IDS), executor.P58_GATE_IDS)

    def test_sealed_dependency_loader_ignores_cache_preseed_and_mutation(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        first = fresh_executor._load_local_sealed_dependencies()
        first.load_pulse87 = lambda _repo_root: (_ for _ in ()).throw(
            AssertionError("mutated prior binder was reused")
        )
        first.SealedDependencyFailure = RuntimeError
        second = fresh_executor._load_local_sealed_dependencies()
        self.assertIsNot(first, second)
        self.assertTrue(callable(second.load_pulse87))
        self.assertIsNot(second.SealedDependencyFailure, RuntimeError)
        self._assert_exact_loaded_stack(second.load_pulse87(self.repo_root))

    def test_concurrent_load_pulse87_serializes_and_restores_foreign_sentinel(self) -> None:
        fresh_a = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        fresh_b = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder_a = fresh_a._load_local_sealed_dependencies()
        binder_b = fresh_b._load_local_sealed_dependencies()
        sentinel = types.ModuleType("foreign_sealed_dependencies")
        sentinel.marker = "foreign"
        missing = object()
        previous = sys.modules.get("sealed_dependencies", missing)
        sys.modules["sealed_dependencies"] = sentinel
        first_holding = threading.Event()
        release_first = threading.Event()
        results: list[tuple[object, ...]] = []
        failures: list[BaseException] = []
        seen_first = False
        original_exec_a = binder_a._exec_bound_module
        original_exec_b = binder_b._exec_bound_module

        def gated_exec(
            original: object,
            *arguments: object,
            **kwargs: object,
        ) -> object:
            nonlocal seen_first
            name, source, content, code = arguments[:4]
            if (
                type(name) is not str
                or not isinstance(source, Path)
                or type(content) is not bytes
                or type(code) is not str
            ):
                raise AssertionError("unexpected _exec_bound_module signature")
            if name == "p88_exact_p87" and not seen_first:
                seen_first = True
                first_holding.set()
                self.assertIsNot(sys.modules["sealed_dependencies"], sentinel)
                if not release_first.wait(5):
                    raise AssertionError("timed out waiting for concurrent loader")
            return original(*arguments, **kwargs)

        def worker(binder: object) -> None:
            try:
                results.append(binder.load_pulse87(self.repo_root))
            except BaseException as error:  # pragma: no cover - asserted below
                failures.append(error)

        try:
            with (
                patch.object(
                    binder_a,
                    "_exec_bound_module",
                    new=lambda *args, **kwargs: gated_exec(
                        original_exec_a, *args, **kwargs
                    ),
                ),
                patch.object(
                    binder_b,
                    "_exec_bound_module",
                    new=lambda *args, **kwargs: gated_exec(
                        original_exec_b, *args, **kwargs
                    ),
                ),
            ):
                first = threading.Thread(target=worker, args=(binder_a,))
                second = threading.Thread(target=worker, args=(binder_b,))
                first.start()
                self.assertTrue(first_holding.wait(5))
                second.start()
                self.assertTrue(second.is_alive())
                release_first.set()
                first.join(10)
                second.join(10)
                self.assertFalse(first.is_alive())
                self.assertFalse(second.is_alive())
            if failures:
                raise failures[0]
            self.assertEqual(len(results), 2)
            for modules in results:
                self._assert_exact_loaded_stack(modules)
            self.assertIs(sys.modules["sealed_dependencies"], sentinel)
        finally:
            if previous is missing:
                sys.modules.pop("sealed_dependencies", None)
            else:
                sys.modules["sealed_dependencies"] = previous

    def test_load_pulse87_import_exception_restores_generic_slot(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        sentinel = types.ModuleType("foreign_sealed_dependencies")
        missing = object()
        previous = sys.modules.get("sealed_dependencies", missing)
        sys.modules["sealed_dependencies"] = sentinel
        original_exec = binder._exec_bound_module

        def failing_exec(name: str, source: Path, content: bytes, code: str) -> object:
            if name == "p88_exact_p87":
                raise RuntimeError("forced import failure")
            return original_exec(name, source, content, code)

        try:
            with patch.object(binder, "_exec_bound_module", new=failing_exec):
                with self.assertRaises(RuntimeError):
                    binder.load_pulse87(self.repo_root)
            self.assertIs(sys.modules["sealed_dependencies"], sentinel)
        finally:
            if previous is missing:
                sys.modules.pop("sealed_dependencies", None)
            else:
                sys.modules["sealed_dependencies"] = previous

    def test_stress_concurrent_legitimate_binder_pairs_complete_without_failure(self) -> None:
        fresh_a = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        fresh_b = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder_a = fresh_a._load_local_sealed_dependencies()
        binder_b = fresh_b._load_local_sealed_dependencies()
        expected_paths = tuple(
            Path(module.__file__).resolve()
            for module in (self.p87, self.p52, self.p86, self.p51, self.p43, self.p47)
        )
        expected_slot = sys.modules.get("sealed_dependencies")
        start = threading.Event()
        results: list[tuple[str, int, tuple[object, ...]]] = []
        failures: list[tuple[str, int, BaseException]] = []
        results_lock = threading.Lock()
        threads: list[threading.Thread] = []

        def worker(label: str, binder: object, pair_index: int) -> None:
            try:
                if not start.wait(30):
                    raise AssertionError("stress start barrier timed out")
                modules = binder.load_pulse87(self.repo_root)
                with results_lock:
                    results.append((label, pair_index, modules))
            except BaseException as error:  # pragma: no cover - asserted below
                with results_lock:
                    failures.append((label, pair_index, error))

        for pair_index in range(100):
            threads.append(
                threading.Thread(
                    target=worker,
                    args=("first", binder_a, pair_index),
                    name=f"p59-stress-a-{pair_index}",
                )
            )
            threads.append(
                threading.Thread(
                    target=worker,
                    args=("second", binder_b, pair_index),
                    name=f"p59-stress-b-{pair_index}",
                )
            )

        for thread in threads:
            thread.start()
        start.set()
        for thread in threads:
            thread.join(300)
            self.assertFalse(thread.is_alive(), f"{thread.name} hung")

        if failures:
            self.fail(
                "concurrent legitimate load failures: "
                f"{[(label, pair_index, str(error)) for label, pair_index, error in failures]}"
            )
        self.assertEqual(len(results), 200)
        for _label, _pair_index, modules in results:
            self._assert_exact_loaded_stack(modules)
            self.assertEqual(
                tuple(Path(module.__file__).resolve() for module in modules),
                expected_paths,
            )
        self.assertIs(sys.modules.get("sealed_dependencies"), expected_slot)

    def test_kernel_lock_name_is_stable_across_fresh_binders(self) -> None:
        fresh_a = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        fresh_b = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder_a = fresh_a._load_local_sealed_dependencies()
        binder_b = fresh_b._load_local_sealed_dependencies()
        first = binder_a._kernel_lock_name()
        second = binder_b._kernel_lock_name()
        self.assertEqual(first, second)
        self.assertNotIn("pulse-88-sealed-loader-locks", first)
        self.assertTrue(
            first.startswith("Local\\")
            if os.name == "nt"
            else first.startswith("\0ferris-p88-")
        )

    def test_kernel_lock_rejects_unsupported_posix_platform(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        with (
            patch.object(binder.os, "name", "posix"),
            patch.object(binder.sys, "platform", "darwin"),
        ):
            with self.assertRaises(binder.SealedDependencyFailure) as raised:
                binder._open_kernel_lock("\0ferris-p88-unsupported")
        self.assertEqual(str(raised.exception), "P88-SEALED-LOCK-OPEN")

    def test_kernel_lock_reentrant_same_pid_tracks_depth_and_single_acquisition(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        opens: list[object] = []
        acquires: list[object] = []
        closes: list[tuple[object, bool]] = []

        def open_kernel_lock(name: str) -> object:
            state = binder._KernelLockState("test-lock", name, object())
            opens.append(state)
            return state

        def acquire_kernel_lock(lock_state: object) -> None:
            acquires.append(lock_state)

        def close_kernel_lock(lock_state: object, *, acquired: bool) -> None:
            if getattr(lock_state, "handle", None) is None:
                return
            closes.append((lock_state, acquired))
            lock_state.handle = None

        with (
            patch.object(binder, "_kernel_lock_name", return_value="test-lock"),
            patch.object(binder, "_open_kernel_lock", side_effect=open_kernel_lock),
            patch.object(binder, "_acquire_kernel_lock", new=acquire_kernel_lock),
            patch.object(binder, "_close_kernel_lock", new=close_kernel_lock),
        ):
            with binder._sealed_loading_lock() as outer:
                self.assertEqual(outer["depth"], 1)
                self.assertEqual(outer["owner_pid"], os.getpid())
                self.assertEqual(outer["owner_thread_id"], threading.get_ident())
                with binder._sealed_loading_lock() as inner:
                    self.assertEqual(inner["depth"], 2)
                    self.assertEqual(inner["owner_pid"], outer["owner_pid"])
                    self.assertEqual(inner["owner_thread_id"], outer["owner_thread_id"])
                    self.assertEqual(inner["name"], outer["name"])
        self.assertEqual(len(opens), 1)
        self.assertEqual(acquires, opens)
        self.assertEqual(closes, [(opens[0], True)])

    def test_kernel_lock_context_copy_thread_blocks_until_owner_release(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        attempting = threading.Event()
        acquired = threading.Event()
        failures: list[BaseException] = []
        worker_records: list[tuple[int, Mapping[str, object]]] = []

        def worker() -> None:
            try:
                attempting.set()
                with binder._sealed_loading_lock() as state:
                    worker_records.append((threading.get_ident(), dict(state)))
                    acquired.set()
            except BaseException as error:  # pragma: no cover - asserted below
                failures.append(error)

        with binder._sealed_loading_lock() as owner_state:
            copied = contextvars.copy_context()
            worker_thread = threading.Thread(
                target=lambda: copied.run(worker),
                name="p59-context-copy-worker",
            )
            worker_thread.start()
            self.assertTrue(attempting.wait(5))
            self.assertFalse(
                acquired.wait(0.2),
                "copied context bypassed kernel lock across threads",
            )
            self.assertEqual(owner_state["owner_thread_id"], threading.get_ident())
        worker_thread.join(10)
        self.assertFalse(worker_thread.is_alive())
        if failures:
            raise failures[0]
        self.assertTrue(acquired.is_set())
        self.assertEqual(len(worker_records), 1)
        worker_thread_id, worker_state = worker_records[0]
        self.assertEqual(worker_state["owner_pid"], os.getpid())
        self.assertEqual(worker_state["owner_thread_id"], worker_thread_id)
        self.assertNotEqual(worker_state["owner_thread_id"], threading.get_ident())
        self.assertEqual(worker_state["depth"], 1)

    def test_kernel_lock_context_replay_blocks_until_other_thread_releases(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        copied: contextvars.Context | None = None
        with binder._sealed_loading_lock():
            copied = contextvars.copy_context()

        holder_ready = threading.Event()
        release_holder = threading.Event()
        failures: list[BaseException] = []

        def holder() -> None:
            try:
                with binder._sealed_loading_lock():
                    holder_ready.set()
                    if not release_holder.wait(5):
                        raise AssertionError("timed out waiting to release holder")
            except BaseException as error:  # pragma: no cover - asserted below
                failures.append(error)

        holder_thread = threading.Thread(target=holder, name="p59-replay-holder")
        holder_thread.start()
        self.assertTrue(holder_ready.wait(5))

        def delayed_release() -> None:
            time.sleep(0.2)
            release_holder.set()

        releaser = threading.Thread(target=delayed_release, name="p59-replay-release")
        releaser.start()
        self.assertIsNotNone(copied)
        started = time.monotonic()

        def replay() -> Mapping[str, object]:
            with binder._sealed_loading_lock() as state:
                return dict(state)

        replay_state = copied.run(replay)
        elapsed = time.monotonic() - started
        releaser.join(5)
        holder_thread.join(10)
        self.assertFalse(holder_thread.is_alive())
        if failures:
            raise failures[0]
        self.assertGreaterEqual(
            elapsed,
            0.18,
            "replayed copied context bypassed invalidated lock ownership",
        )
        self.assertEqual(replay_state["owner_pid"], os.getpid())
        self.assertEqual(replay_state["owner_thread_id"], threading.get_ident())
        self.assertEqual(replay_state["depth"], 1)

    def test_kernel_lock_cross_instance_same_thread_fails_closed_quickly_and_clears_marker(
        self,
    ) -> None:
        fresh_a = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        fresh_b = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder_a = fresh_a._load_local_sealed_dependencies()
        binder_b = fresh_b._load_local_sealed_dependencies()
        manager_a = fresh_a._P88_INTERNAL_LOCK_MANAGER
        manager_b = fresh_b._P88_INTERNAL_LOCK_MANAGER
        self.assertEqual(manager_a.advisory_snapshot(), ())
        with binder_a._sealed_loading_lock() as owner_state:
            expected_key = ((owner_state["name"], os.getpid(), threading.get_ident()),)
            self.assertEqual(manager_a.advisory_snapshot(), expected_key)
            started = time.monotonic()
            with self.assertRaises(binder_b.SealedDependencyFailure) as raised:
                with binder_b._sealed_loading_lock():
                    raise AssertionError("cross-instance reentry unexpectedly acquired")
            elapsed = time.monotonic() - started
            self.assertEqual(
                str(raised.exception), "P88-SEALED-LOCK-CROSS-INSTANCE-REENTRY"
            )
            self.assertLess(elapsed, 0.5, "cross-instance reentry did not fail promptly")
            self.assertEqual(manager_b.advisory_snapshot(), expected_key)
        self.assertEqual(manager_a.advisory_snapshot(), ())
        with binder_b._sealed_loading_lock() as recovered:
            self.assertEqual(recovered["depth"], 1)
        self.assertEqual(manager_b.advisory_snapshot(), ())

    @unittest.skipUnless(
        os.name == "posix" and sys.platform.startswith("linux"),
        "Linux executor at-fork manager regression",
    )
    def test_kernel_lock_at_fork_registration_is_idempotent_per_binder(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        manager = fresh_executor._P88_INTERNAL_LOCK_MANAGER
        self.assertEqual(manager._fork_hook_registrations, 1)
        manager_ids: set[int] = set()
        for _ in range(25):
            binder = fresh_executor._load_local_sealed_dependencies()
            manager_ids.add(id(binder._P88_INTERNAL_LOCK_MANAGER))
            self.assertIs(binder._P88_INTERNAL_LOCK_MANAGER, manager)
            self.assertEqual(
                binder._P88_INTERNAL_LOCK_MANAGER._fork_hook_registrations, 1
            )
        self.assertEqual(manager_ids, {id(manager)})

    def test_kernel_lock_pid_mismatch_closes_inherited_handle_before_reacquire(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        current_pid = {"value": 101}
        opens: list[object] = []
        closes: list[tuple[object, bool]] = []

        def open_kernel_lock(name: str) -> object:
            state = binder._KernelLockState("test-lock", name, object())
            opens.append(state)
            return state

        def close_kernel_lock(lock_state: object, *, acquired: bool) -> None:
            if getattr(lock_state, "handle", None) is None:
                return
            closes.append((lock_state, acquired))
            lock_state.handle = None

        with (
            patch.object(binder, "_current_pid", new=lambda: current_pid["value"]),
            patch.object(binder, "_kernel_lock_name", return_value="test-lock"),
            patch.object(binder, "_open_kernel_lock", side_effect=open_kernel_lock),
            patch.object(binder, "_acquire_kernel_lock", new=lambda _state: None),
            patch.object(binder, "_close_kernel_lock", new=close_kernel_lock),
        ):
            with binder._sealed_loading_lock() as parent_state:
                self.assertEqual(parent_state["owner_pid"], 101)
                self.assertEqual(parent_state["owner_thread_id"], threading.get_ident())
                self.assertEqual(parent_state["depth"], 1)
                current_pid["value"] = 202
                with binder._sealed_loading_lock() as child_state:
                    self.assertEqual(child_state["owner_pid"], 202)
                    self.assertEqual(child_state["owner_thread_id"], threading.get_ident())
                    self.assertEqual(child_state["depth"], 1)
                    self.assertEqual(len(opens), 2)
                    self.assertEqual(closes, [(opens[0], False)])
        self.assertEqual(closes, [(opens[0], False), (opens[1], True)])

    def test_kernel_lock_does_not_create_path_artifacts(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        artifact_root = REPO_ROOT / "target" / "pulse-88-sealed-loader-locks"
        if artifact_root.exists():
            shutil.rmtree(artifact_root, ignore_errors=True)
        with binder._sealed_loading_lock():
            self.assertFalse(artifact_root.exists())
        self.assertFalse(artifact_root.exists())

    def test_kernel_lock_acquire_failure_cleans_up(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        kernel_lock = binder._KernelLockState("test-lock", "p59-test-lock", object())
        close_calls: list[tuple[object, bool]] = []

        def close_kernel_lock(lock_state: object, *, acquired: bool) -> None:
            close_calls.append((lock_state, acquired))

        with (
            patch.object(binder, "_open_kernel_lock", return_value=kernel_lock),
            patch.object(
                binder,
                "_acquire_kernel_lock",
                side_effect=binder.SealedDependencyFailure("P88-SEALED-LOCK-ACQUIRE"),
            ),
            patch.object(binder, "_close_kernel_lock", new=close_kernel_lock),
        ):
            with self.assertRaises(binder.SealedDependencyFailure) as raised:
                with binder._sealed_loading_lock():
                    raise AssertionError("kernel lock acquisition failure entered context")
        self.assertEqual(str(raised.exception), "P88-SEALED-LOCK-ACQUIRE")
        self.assertEqual(close_calls, [(kernel_lock, False)])

    def test_kernel_lock_releases_after_exception(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        manager = fresh_executor._P88_INTERNAL_LOCK_MANAGER
        with self.assertRaises(RuntimeError):
            with binder._sealed_loading_lock():
                self.assertEqual(
                    manager.advisory_snapshot(),
                    ((binder._kernel_lock_name(), os.getpid(), threading.get_ident()),),
                )
                raise RuntimeError("forced kernel lock exception")
        self.assertEqual(manager.advisory_snapshot(), ())
        with binder._sealed_loading_lock() as state:
            self.assertEqual(state["name"], binder._kernel_lock_name())

    def test_kernel_lock_wait_abandoned_is_treated_as_acquired_and_released(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        kernel_lock = binder._KernelLockState("windows-mutex", "Local\\p59-test", object())

        class FakeKernel32:
            def __init__(self) -> None:
                self.waits: list[tuple[object, int]] = []
                self.releases = 0
                self.closes = 0

            def WaitForSingleObject(self, handle: object, timeout: int) -> int:
                self.waits.append((handle, timeout))
                return binder._WINDOWS_WAIT_ABANDONED

            def ReleaseMutex(self, handle: object) -> int:
                self.releases += 1
                return 1

            def CloseHandle(self, handle: object) -> int:
                self.closes += 1
                return 1

        library = FakeKernel32()
        with (
            patch.object(binder, "_kernel_lock_name", return_value="Local\\p59-test"),
            patch.object(binder, "_open_kernel_lock", return_value=kernel_lock),
            patch.object(binder, "_windows_kernel32", return_value=library),
        ):
            with binder._sealed_loading_lock() as state:
                self.assertEqual(state["kind"], "windows-mutex")
                self.assertEqual(state["name"], "Local\\p59-test")
        self.assertTrue(library.waits)
        self.assertEqual(library.releases, 1)
        self.assertEqual(library.closes, 1)

    def test_kernel_lock_crash_recovery_reacquires_after_subprocess_exit(self) -> None:
        script = self._write_subprocess_script(
            "kernel_lock_crash_recovery.py",
            """
            ready = Path(sys.argv[1])
            binder = executor._load_local_sealed_dependencies()
            with binder._sealed_loading_lock():
                ready.write_text(str(os.getpid()), encoding="ascii", newline="\\n")
                os._exit(23)
            """,
        )
        ready = self.sandbox / "crash-ready.txt"
        completed = self._run_subprocess_script(script, os.fspath(ready), timeout=15)
        self.assertEqual(completed.returncode, 23, completed.stderr)
        self.assertTrue(ready.is_file())
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        with binder._sealed_loading_lock() as state:
            self.assertEqual(state["name"], binder._kernel_lock_name())

    def test_kernel_lock_process_stress_serializes_subprocesses(self) -> None:
        script = self._write_subprocess_script(
            "kernel_lock_process_stress.py",
            """
            start = Path(sys.argv[1])
            marker = Path(sys.argv[2])
            index = sys.argv[3]
            binder = executor._load_local_sealed_dependencies()
            deadline = time.monotonic() + 10.0
            while not start.exists():
                if time.monotonic() >= deadline:
                    raise SystemExit("timed out waiting for process stress start")
                time.sleep(0.01)
            with binder._sealed_loading_lock():
                descriptor = os.open(
                    marker,
                    os.O_CREAT | os.O_EXCL | os.O_WRONLY,
                    0o600,
                )
                try:
                    os.write(descriptor, f"{index}:{os.getpid()}".encode("ascii"))
                finally:
                    os.close(descriptor)
                time.sleep(0.05)
                marker.unlink()
            """,
        )
        start = self.sandbox / "process-stress-start.flag"
        marker = self.sandbox / "process-stress-marker"
        processes = [
            subprocess.Popen(
                [sys.executable, "-B", os.fspath(script), os.fspath(start), os.fspath(marker), str(index)],
                cwd=ROOT,
                env=self._subprocess_environment(),
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            for index in range(8)
        ]
        start.write_text("go\n", encoding="ascii", newline="\n")
        failures: list[tuple[int, int, str, str]] = []
        for index, process in enumerate(processes):
            stdout, stderr = process.communicate(timeout=30)
            if process.returncode != 0:
                failures.append((index, process.returncode, stdout, stderr))
        self.assertFalse(marker.exists())
        if failures:
            self.fail(f"kernel lock subprocess stress failures: {failures}")

    @unittest.skipUnless(
        os.name == "posix" and sys.platform.startswith("linux"),
        "Linux abstract AF_UNIX fork regression",
    )
    def test_kernel_lock_fork_inside_lock_reacquires_without_count_inflation(self) -> None:
        script = self._write_subprocess_script(
            "kernel_lock_fork_reacquire.py",
            """
            import select

            result_path = Path(sys.argv[1])
            binder = executor._load_local_sealed_dependencies()
            read_fd, write_fd = os.pipe()
            child_pid = -1
            try:
                with binder._sealed_loading_lock():
                    child_pid = os.fork()
                    if child_pid == 0:
                        os.close(read_fd)
                        try:
                            with binder._sealed_loading_lock():
                                os.write(write_fd, b"A")
                                time.sleep(0.2)
                            os.write(write_fd, b"R")
                            os._exit(0)
                        except BaseException:
                            os._exit(1)
                    os.close(write_fd)
                    blocked_before_release = not select.select([read_fd], [], [], 0.1)[0]
                data = bytearray()
                deadline = time.monotonic() + 5.0
                while len(data) < 2 and time.monotonic() < deadline:
                    ready, _, _ = select.select([read_fd], [], [], 0.1)
                    if ready:
                        chunk = os.read(read_fd, 2 - len(data))
                        if chunk:
                            data.extend(chunk)
                waited_pid, status = os.waitpid(child_pid, 0)
                with binder._sealed_loading_lock():
                    reacquired = True
                result_path.write_text(
                    json.dumps(
                        {
                            "blocked_before_release": blocked_before_release,
                            "child_messages": data.decode("ascii"),
                            "child_pid": waited_pid,
                            "exit_code": os.waitstatus_to_exitcode(status),
                            "reacquired": reacquired,
                        }
                    )
                    + "\\n",
                    encoding="utf-8",
                    newline="\\n",
                )
            finally:
                for descriptor in (read_fd, write_fd):
                    try:
                        os.close(descriptor)
                    except OSError:
                        pass
            """,
        )
        result_path = self.sandbox / "fork-reacquire.json"
        completed = self._run_subprocess_script(script, os.fspath(result_path), timeout=20)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        result = json.loads(result_path.read_text(encoding="utf-8"))
        self.assertTrue(result["blocked_before_release"])
        self.assertEqual(result["child_messages"], "AR")
        self.assertEqual(result["exit_code"], 0)
        self.assertTrue(result["reacquired"])

    @unittest.skipUnless(
        os.name == "posix" and sys.platform.startswith("linux"),
        "Linux abstract AF_UNIX at-fork cleanup regression",
    )
    def test_kernel_lock_fork_child_cleanup_allows_parent_reacquire_before_child_reentry(
        self,
    ) -> None:
        script = self._write_subprocess_script(
            "kernel_lock_fork_child_cleanup.py",
            """
            import select

            result_path = Path(sys.argv[1])
            binder = executor._load_local_sealed_dependencies()
            read_fd, write_fd = os.pipe()
            child_pid = -1
            try:
                with binder._sealed_loading_lock():
                    child_pid = os.fork()
                    if child_pid == 0:
                        os.close(read_fd)
                        try:
                            time.sleep(0.5)
                            with binder._sealed_loading_lock():
                                os.write(write_fd, b"A")
                            os.write(write_fd, b"R")
                            os._exit(0)
                        except BaseException:
                            os._exit(1)
                    os.close(write_fd)
                reacquire_started = time.monotonic()
                with binder._sealed_loading_lock():
                    reacquire_delay = time.monotonic() - reacquire_started
                data = bytearray()
                deadline = time.monotonic() + 5.0
                while len(data) < 2 and time.monotonic() < deadline:
                    ready, _, _ = select.select([read_fd], [], [], 0.1)
                    if ready:
                        chunk = os.read(read_fd, 2 - len(data))
                        if chunk:
                            data.extend(chunk)
                waited_pid, status = os.waitpid(child_pid, 0)
                result_path.write_text(
                    json.dumps(
                        {
                            "child_messages": data.decode("ascii"),
                            "child_pid": waited_pid,
                            "exit_code": os.waitstatus_to_exitcode(status),
                            "reacquire_delay_seconds": reacquire_delay,
                        }
                    )
                    + "\\n",
                    encoding="utf-8",
                    newline="\\n",
                )
            finally:
                for descriptor in (read_fd, write_fd):
                    try:
                        os.close(descriptor)
                    except OSError:
                        pass
            """,
        )
        result_path = self.sandbox / "fork-child-cleanup.json"
        completed = self._run_subprocess_script(script, os.fspath(result_path), timeout=20)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        result = json.loads(result_path.read_text(encoding="utf-8"))
        self.assertEqual(result["child_messages"], "AR")
        self.assertEqual(result["exit_code"], 0)
        self.assertLess(result["reacquire_delay_seconds"], 0.25)

    def test_old_private_binder_key_is_ignored(self) -> None:
        _clean_loaded_test_modules()
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        sentinel = types.ModuleType("old_private_binder_key")
        key = _old_private_binder_key(fresh_executor)
        sys.modules[key] = sentinel
        try:
            binder = fresh_executor._load_local_sealed_dependencies()
            self.assertIs(sys.modules[key], sentinel)
            self.assertTrue(callable(binder.load_pulse87))
            self._assert_exact_loaded_stack(binder.load_pulse87(self.repo_root))
        finally:
            _clean_loaded_test_modules()

    def test_old_registry_key_is_ignored(self) -> None:
        _clean_loaded_test_modules()
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        sentinel = types.ModuleType("old_registry_key")
        key = _old_registry_key(fresh_executor)
        sys.modules[key] = sentinel
        try:
            binder = fresh_executor._load_local_sealed_dependencies()
            self.assertIs(sys.modules[key], sentinel)
            self.assertTrue(callable(binder.load_pulse87))
            self._assert_exact_loaded_stack(binder.load_pulse87(self.repo_root))
        finally:
            _clean_loaded_test_modules()

    def test_two_executor_instances_load_fresh_binders_without_cached_state(self) -> None:
        _clean_loaded_test_modules()
        fresh_a = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        fresh_b = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        first_holding = threading.Event()
        release_first = threading.Event()
        results: list[ModuleType] = []
        failures: list[BaseException] = []
        original_exec_a = fresh_a._exec_local_sealed_module
        original_exec_b = fresh_b._exec_local_sealed_module

        def gated_exec(original: object, path: Path, content: bytes) -> ModuleType:
            if not first_holding.is_set():
                first_holding.set()
                if not release_first.wait(5):
                    raise AssertionError("timed out waiting for fresh binder load")
            return original(path, content)

        def worker(target: object) -> None:
            try:
                results.append(target._load_local_sealed_dependencies())
            except BaseException as error:  # pragma: no cover - asserted below
                failures.append(error)

        try:
            with (
                patch.object(
                    fresh_a,
                    "_exec_local_sealed_module",
                    new=lambda path, content: gated_exec(original_exec_a, path, content),
                ),
                patch.object(
                    fresh_b,
                    "_exec_local_sealed_module",
                    new=lambda path, content: gated_exec(original_exec_b, path, content),
                ),
            ):
                first = threading.Thread(target=worker, args=(fresh_a,))
                second = threading.Thread(target=worker, args=(fresh_b,))
                first.start()
                self.assertTrue(first_holding.wait(5))
                second.start()
                self.assertTrue(second.is_alive())
                release_first.set()
                first.join(10)
                second.join(10)
                self.assertFalse(first.is_alive())
                self.assertFalse(second.is_alive())
            if failures:
                raise failures[0]
            self.assertEqual(len(results), 2)
            self.assertIsNot(results[0], results[1])
            self.assertEqual(
                Path(results[0].__file__).resolve(),
                (ROOT / "sealed_dependencies.py").resolve(),
            )
            self.assertEqual(
                Path(results[1].__file__).resolve(),
                (ROOT / "sealed_dependencies.py").resolve(),
            )
            self.assertNotEqual(results[0].__name__, results[1].__name__)
            self.assertFalse(
                any(
                    name.startswith(
                        "ferris.pulse-88.local-sealed-dependencies.runtime"
                    )
                    for name in sys.modules
                )
            )
        finally:
            _clean_loaded_test_modules()

    def test_local_sealed_binder_exception_cleanup_leaves_no_stale_runtime_slot(self) -> None:
        _clean_loaded_test_modules()
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        runtime_prefix = fresh_executor._LOCAL_SEALED_DEPENDENCIES_RUNTIME_PREFIX

        def failing_exec(*_args: object, **_kwargs: object) -> None:
            raise RuntimeError("forced local sealed bootstrap failure")

        try:
            with patch.object(fresh_executor, "exec", new=failing_exec, create=True):
                with self.assertRaises(fresh_executor._LocalSealedBootstrapFailure) as raised:
                    fresh_executor._load_local_sealed_dependencies()
            self.assertEqual(str(raised.exception), "P88-LOCAL-SEALED-IMPORT")
            self.assertFalse(
                any(name.startswith(runtime_prefix) for name in sys.modules)
            )
        finally:
            _clean_loaded_test_modules()

    def test_qualification_delegates_to_exact_p87_executor(self) -> None:
        with patch.object(
            self.p87,
            "_run_qualification_executor",
            wraps=self.p87._run_qualification_executor,
        ) as wrapped:
            result, fake, sessions, runtime, terminal_root = self._run("delegates")
        wrapped.assert_called_once()
        self._assert_completed_execution(result, fake, sessions, runtime, terminal_root)
        self.assertEqual(result.publication["disposition"], "published-result")

    def test_published_result_retains_verified_result_and_witness_after_p87_cleanup(self) -> None:
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

    def test_p87_prelaunch_failure_remains_publication_not_attempted(self) -> None:
        p39, p41 = self.p87.load_exact_p39_and_p41(self.repo_root)

        def fail(*_args: object) -> object:
            raise p39.PublicFailure("P39-TEST-FAILURE")

        with (
            patch.object(self.p87, "load_exact_p39_and_p41", return_value=(p39, p41)),
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

    def test_preexisting_terminal_root_is_rejected_before_p87_execution(self) -> None:
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
            raise AssertionError("P87 must not build WSL capability when terminal root exists")

        with (
            patch.object(
                executor,
                "_load_local_sealed_dependencies",
                return_value=self._sealed_stub(),
            ),
            patch.object(
                self.p87,
                "_run_qualification_executor",
                side_effect=AssertionError("P87 must not run"),
            ),
        ):
            result = executor._run_qualification_executor(
                self.repo_root,
                runtime,
                runtime / "p27-cycle",
                qualify._synthetic_p39_checkout(root),
                root / "p41-public-custody",
                seed_bytes=qualify.synthetic_seed(1),
                p27_runner=qualify._p27_success,
                p56=fake,
                open_wsl=open_wsl,
            )
        self.assertEqual(result.private_record["failure_code"], "P88-TERMINAL-ROOT-NOT-FRESH")
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
