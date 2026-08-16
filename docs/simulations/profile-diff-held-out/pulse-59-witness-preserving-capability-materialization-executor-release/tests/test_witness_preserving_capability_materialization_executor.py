from __future__ import annotations

import atexit
import hashlib
import importlib.util
import inspect
import json
import os
import shutil
import subprocess
import sys
import threading
import types
import unittest
import uuid
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = ROOT.parents[3]
RUN_ROOT = REPO_ROOT / "target" / "pulse-59-test-runtime"
_LOADED_TEST_MODULES: set[str] = set()
_DYNAMIC_RELEASE_PREFIXES = (
    "p59_exact_",
    "p58_exact_",
    "p57_exact_",
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
_LOCAL_SEALED_MODULE_PREFIX = "_ferris_p59_local_sealed_dependencies_v1_"
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
    for name in tuple(sys.modules):
        if name.startswith(_DYNAMIC_RELEASE_PREFIXES) or name.startswith(
            (
                _LOCAL_SEALED_MODULE_PREFIX,
                "ferris.pulse-59.local-sealed-dependencies.runtime",
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
    name = f"pulse59_test_{path.stem}_{uuid.uuid4().hex}"
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


def _make_directory_link(link: Path, target: Path) -> str:
    if os.name != "nt":
        os.symlink(target, link)
        return "symlink"
    try:
        os.symlink(target, link, target_is_directory=True)
        return "symlink"
    except (NotImplementedError, OSError) as symlink_error:
        command = [
            os.environ.get("ComSpec", r"C:\Windows\System32\cmd.exe"),
            "/c",
            "mklink",
            "/J",
            os.fspath(link),
            os.fspath(target),
        ]
        completed = subprocess.run(
            command, check=False, capture_output=True, text=True
        )
        if completed.returncode == 0:
            return "junction"
        raise unittest.SkipTest(
            "directory link creation unavailable: "
            f"{symlink_error}; {completed.stdout}{completed.stderr}"
        )


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

    def _fake_lock_anchor(self, name: str) -> Path:
        release = (
            self.sandbox
            / name
            / "docs"
            / "simulations"
            / "profile-diff-held-out"
            / ROOT.name
        )
        release.mkdir(parents=True)
        path = release / "sealed_dependencies.py"
        path.write_text("# synthetic lock anchor\n", encoding="utf-8", newline="\n")
        return path.resolve(strict=True)

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

    def _assert_exact_loaded_stack(self, modules: tuple[object, ...]) -> None:
        p58, p52, p57, p51, p43, p47 = modules
        self.assertEqual(tuple(p58.P58_GATE_IDS), executor.P58_GATE_IDS)
        self.assertTrue(callable(p58.run_ordered_capability_materialization_executor))
        self.assertTrue(callable(p52._cleanup_terminal_publication))
        self.assertTrue(callable(p57.run_capability_bound_diagnostic_executor))
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
        self.assertIn("LOCAL_SEALED_DEPENDENCIES_SHA256", source)
        self.assertNotIn("_LOCAL_SEALED_MODULE_PREFIX", source)
        self.assertNotIn("_process_local_sealed_registry", source)
        sealed_source = (ROOT / "sealed_dependencies.py").read_text(encoding="utf-8")
        self.assertNotIn("_P58_MODULE", sealed_source)
        self.assertIn("msvcrt.locking", sealed_source)
        self.assertIn("fcntl.flock", sealed_source)
        self.assertIn("st_file_attributes", sealed_source)
        self.assertIn("os.O_EXCL", sealed_source)
        self.assertIn("_reverify_lock_ancestors", sealed_source)
        self.assertIn("_revalidate_locked_path", sealed_source)
        self.assertIn("_ACTIVE_SEALED_LOADING_LOCK", sealed_source)
        self.assertNotIn("threading.RLock", sealed_source)

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
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        first = fresh_executor._load_local_sealed_dependencies()
        first.load_pulse58 = lambda _repo_root: (_ for _ in ()).throw(
            AssertionError("mutated prior binder was reused")
        )
        first.SealedDependencyFailure = RuntimeError
        second = fresh_executor._load_local_sealed_dependencies()
        self.assertIsNot(first, second)
        self.assertTrue(callable(second.load_pulse58))
        self.assertIsNot(second.SealedDependencyFailure, RuntimeError)
        self._assert_exact_loaded_stack(second.load_pulse58(REPO_ROOT))

    def test_concurrent_load_pulse58_serializes_and_restores_foreign_sentinel(self) -> None:
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
            if name == "p59_exact_p58" and not seen_first:
                seen_first = True
                first_holding.set()
                self.assertIsNot(sys.modules["sealed_dependencies"], sentinel)
                if not release_first.wait(5):
                    raise AssertionError("timed out waiting for concurrent loader")
            return original(*arguments, **kwargs)

        def worker(binder: object) -> None:
            try:
                results.append(binder.load_pulse58(REPO_ROOT))
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

    def test_load_pulse58_import_exception_restores_generic_slot(self) -> None:
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
            if name == "p59_exact_p58":
                raise RuntimeError("forced import failure")
            return original_exec(name, source, content, code)

        try:
            with patch.object(binder, "_exec_bound_module", new=failing_exec):
                with self.assertRaises(RuntimeError):
                    binder.load_pulse58(REPO_ROOT)
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
            for module in (self.p58, self.p52, self.p57, self.p51, self.p43, self.p47)
        )
        expected_slot = sys.modules.get("sealed_dependencies")
        original_signature_a = binder_a._signature
        completed = 0

        for round_index in range(100):
            ready = threading.Event()
            release = threading.Event()
            results: list[tuple[str, tuple[object, ...]]] = []
            failures: list[tuple[str, BaseException]] = []
            gate_fired = False

            def gated_signature(
                module: ModuleType,
                name: str,
                parameters: tuple[str, ...],
                code: str,
            ) -> None:
                nonlocal gate_fired
                original_signature_a(module, name, parameters, code)
                if module.__name__ == "p59_exact_p58" and name == "_terminal" and not gate_fired:
                    gate_fired = True
                    ready.set()
                    if not release.wait(10):
                        raise AssertionError("timed out widening transitive load window")

            def worker(label: str, binder: object) -> None:
                try:
                    modules = binder.load_pulse58(REPO_ROOT)
                    results.append((label, modules))
                except BaseException as error:  # pragma: no cover - asserted below
                    failures.append((label, error))

            with patch.object(binder_a, "_signature", new=gated_signature):
                first = threading.Thread(
                    target=worker, args=("first", binder_a), name=f"p59-stress-a-{round_index}"
                )
                second = threading.Thread(
                    target=worker, args=("second", binder_b), name=f"p59-stress-b-{round_index}"
                )
                first.start()
                self.assertTrue(ready.wait(10), f"round {round_index} did not reach transitive window")
                second.start()
                self.assertTrue(second.is_alive(), f"round {round_index} second loader never blocked")
                release.set()
                first.join(20)
                second.join(20)

            self.assertFalse(first.is_alive(), f"round {round_index} first loader hung")
            self.assertFalse(second.is_alive(), f"round {round_index} second loader hung")
            if failures:
                self.fail(
                    f"round {round_index} concurrent legitimate load failures: "
                    f"{[(label, str(error)) for label, error in failures]}"
                )
            self.assertEqual(len(results), 2, f"round {round_index} result count")
            for _label, modules in results:
                self._assert_exact_loaded_stack(modules)
                self.assertEqual(
                    tuple(Path(module.__file__).resolve() for module in modules),
                    expected_paths,
                )
            self.assertIs(sys.modules.get("sealed_dependencies"), expected_slot)
            completed += len(results)

        self.assertEqual(completed, 200)

    def test_lock_file_path_is_stable_across_fresh_binders(self) -> None:
        fresh_a = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        fresh_b = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder_a = fresh_a._load_local_sealed_dependencies()
        binder_b = fresh_b._load_local_sealed_dependencies()
        expected_source = (ROOT / "sealed_dependencies.py").resolve(strict=True)
        expected = (
            REPO_ROOT
            / "target"
            / "pulse-59-sealed-loader-locks"
            / f"{hashlib.sha256(os.fsencode(os.fspath(expected_source))).hexdigest()}.lock"
        )
        self.assertEqual(binder_a._lock_file_path(), expected)
        self.assertEqual(binder_b._lock_file_path(), expected)

    def test_sealed_loading_lock_rejects_linked_target_ancestor(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        fake_self = self._fake_lock_anchor("linked-target")
        fake_repo = fake_self.parents[4]
        linked_target_destination = self.sandbox / "linked-target-destination"
        linked_target_destination.mkdir()
        linked_target = fake_repo / "target"
        link_kind = _make_directory_link(linked_target, linked_target_destination)
        try:
            with patch.object(binder, "_self_path", return_value=fake_self):
                with self.assertRaises(binder.SealedDependencyFailure) as raised:
                    with binder._sealed_loading_lock():
                        raise AssertionError(
                            "lock should reject linked target ancestor"
                        )
            self.assertEqual(str(raised.exception), "P59-SEALED-LOCK-PATH")
            self.assertFalse(
                (linked_target_destination / "pulse-59-sealed-loader-locks").exists(),
                f"linked {link_kind} target was followed",
            )
        finally:
            if linked_target.exists() or os.path.lexists(linked_target):
                if linked_target.is_dir() and not linked_target.is_symlink():
                    os.rmdir(linked_target)
                else:
                    linked_target.unlink()

    def test_sealed_loading_lock_closes_descriptor_on_acquire_failure(self) -> None:
        fresh_executor = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder = fresh_executor._load_local_sealed_dependencies()
        lock_file = self.sandbox / "acquire-failure.lock"
        opened: list[int] = []
        closed: list[int] = []
        original_close = os.close

        def open_descriptor(_path: Path) -> int:
            descriptor = os.open(
                lock_file,
                os.O_RDWR | os.O_CREAT | getattr(os, "O_BINARY", 0),
                0o600,
            )
            opened.append(descriptor)
            return descriptor

        def tracked_close(descriptor: int) -> None:
            closed.append(descriptor)
            original_close(descriptor)

        try:
            with (
                patch.object(binder, "_lock_file_path", return_value=lock_file),
                patch.object(binder, "_open_lock_descriptor", side_effect=open_descriptor),
                patch.object(
                    binder,
                    "_acquire_descriptor_lock",
                    side_effect=binder.SealedDependencyFailure(
                        "P59-SEALED-LOCK-ACQUIRE"
                    ),
                ),
                patch.object(binder.os, "close", new=tracked_close),
            ):
                for _attempt in range(3):
                    with self.assertRaises(binder.SealedDependencyFailure) as raised:
                        with binder._sealed_loading_lock():
                            raise AssertionError(
                                "lock acquisition failure should not enter context"
                            )
                    self.assertEqual(str(raised.exception), "P59-SEALED-LOCK-ACQUIRE")
            self.assertEqual(closed, opened)
            self.assertEqual(len(closed), 3)
            for descriptor in opened:
                with self.assertRaises(OSError):
                    os.fstat(descriptor)
        finally:
            for descriptor in opened:
                try:
                    original_close(descriptor)
                except OSError:
                    pass

    def test_locked_path_revalidation_prevents_distinct_inode_critical_sections(self) -> None:
        fresh_a = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        fresh_b = _load_fresh_release_module(
            "witness_preserving_capability_materialization_executor.py"
        )
        binder_a = fresh_a._load_local_sealed_dependencies()
        binder_b = fresh_b._load_local_sealed_dependencies()
        lock_repo = self.sandbox / "lock-revalidation"
        lock_path = (
            lock_repo
            / "target"
            / "pulse-59-sealed-loader-locks"
            / "shared.lock"
        )
        entered: list[str] = []
        failures: list[tuple[str, BaseException]] = []
        active = 0
        max_active = 0
        swap_complete = threading.Event()
        second_entered = threading.Event()
        first_done = threading.Event()
        original_acquire_a = binder_a._acquire_descriptor_lock
        original_revalidate_a = binder_a._revalidate_locked_path

        def swap_lock_target(mode: str) -> None:
            if mode == "file":
                moved = lock_path.with_name("shared-original.lock")
                os.replace(lock_path, moved)
                lock_path.write_bytes(b"replacement lock\n")
                return
            if mode == "directory":
                moved_root = lock_path.parent.with_name("pulse-59-sealed-loader-locks-moved")
                os.replace(lock_path.parent, moved_root)
                lock_path.parent.mkdir()
                lock_path.write_bytes(b"replacement lock\n")
                return
            raise AssertionError(mode)

        def run_case(mode: str) -> None:
            nonlocal active, max_active
            entered.clear()
            failures.clear()
            active = 0
            max_active = 0
            swap_complete.clear()
            second_entered.clear()
            first_done.clear()

            def first_acquire(descriptor: int) -> None:
                if os.name != "nt":
                    swap_lock_target(mode)
                swap_complete.set()
                original_acquire_a(descriptor)

            def first_revalidate(path: Path, descriptor: int) -> None:
                if os.name != "nt":
                    if not second_entered.wait(10):
                        raise AssertionError("second critical section did not enter")
                else:
                    raise binder_a.SealedDependencyFailure("P59-SEALED-LOCK-PATH")
                original_revalidate_a(path, descriptor)

            def first_worker() -> None:
                try:
                    with binder_a._sealed_loading_lock():
                        entered.append("first")
                except BaseException as error:  # pragma: no cover - asserted below
                    failures.append(("first", error))
                finally:
                    first_done.set()

            def second_worker() -> None:
                nonlocal active, max_active
                try:
                    if not swap_complete.wait(10):
                        raise AssertionError("swap did not complete")
                    with binder_b._sealed_loading_lock():
                        active += 1
                        max_active = max(max_active, active)
                        entered.append("second")
                        second_entered.set()
                        if not first_done.wait(10):
                            raise AssertionError("first worker did not complete")
                        active -= 1
                except BaseException as error:  # pragma: no cover - asserted below
                    failures.append(("second", error))

            if lock_repo.exists():
                shutil.rmtree(lock_repo, ignore_errors=True)
            lock_repo.mkdir()
            first = threading.Thread(target=first_worker, name=f"p59-first-{mode}")
            second = threading.Thread(target=second_worker, name=f"p59-second-{mode}")
            with (
                patch.object(binder_a, "_lock_file_path", return_value=lock_path),
                patch.object(binder_b, "_lock_file_path", return_value=lock_path),
                patch.object(binder_a, "_acquire_descriptor_lock", new=first_acquire),
                patch.object(binder_a, "_revalidate_locked_path", new=first_revalidate),
            ):
                first.start()
                second.start()
                first.join(20)
                second.join(20)
            self.assertFalse(first.is_alive())
            self.assertFalse(second.is_alive())
            self.assertEqual(
                [
                    (name, str(error))
                    for name, error in failures
                ],
                [("first", "P59-SEALED-LOCK-PATH")],
            )
            self.assertEqual(entered, ["second"])
            self.assertEqual(max_active, 1)

        if os.name == "nt":
            run_case("windows-mocked")
            return

        for mode in ("file", "directory"):
            with self.subTest(mode=mode):
                run_case(mode)

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
            self.assertTrue(callable(binder.load_pulse58))
            self._assert_exact_loaded_stack(binder.load_pulse58(REPO_ROOT))
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
            self.assertTrue(callable(binder.load_pulse58))
            self._assert_exact_loaded_stack(binder.load_pulse58(REPO_ROOT))
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
                        "ferris.pulse-59.local-sealed-dependencies.runtime"
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
            self.assertEqual(str(raised.exception), "P59-LOCAL-SEALED-IMPORT")
            self.assertFalse(
                any(name.startswith(runtime_prefix) for name in sys.modules)
            )
        finally:
            _clean_loaded_test_modules()

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
