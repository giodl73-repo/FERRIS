from __future__ import annotations

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
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = ROOT.parents[3]
PROFILE_DIFF_ROOT = REPO_ROOT / "docs" / "simulations" / "profile-diff-held-out"
for cache in PROFILE_DIFF_ROOT.rglob("__pycache__"):
    shutil.rmtree(cache, ignore_errors=True)
sys.dont_write_bytecode = True
sys.path.insert(0, str(ROOT))

import ordered_capability_materialization_executor as executor  # noqa: E402
import generate_release  # noqa: E402
import qualify  # noqa: E402
import sealed_dependencies as sealed  # noqa: E402
from fixtures.fake_p56 import FakeReleaseFailure  # noqa: E402


executor._bind_local_sealed_lock_manager_module(sys.modules["sealed_dependencies"])


class OrderedCapabilityMaterializationExecutorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.work = REPO_ROOT / "target" / f"pulse-81-test-{uuid.uuid4().hex}"
        self.work.mkdir(parents=True)
        self.repo_root = qualify.exact_repo_root()
        self.addCleanup(lambda: shutil.rmtree(self.work, ignore_errors=True))

    def _write_subprocess_script(self, name: str, body: str) -> Path:
        script = self.work / name
        script.write_text(
            "\n".join(
                (
                    "from __future__ import annotations",
                    "",
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
                    "import ordered_capability_materialization_executor as executor",
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

    def _run(
        self,
        name: str,
        *,
        cycle: int = 1,
        p27_runner: object = qualify._p27_success,
        on_launch: object = None,
        stack: tuple[object, object, object] | None = None,
        p39_p41: tuple[object, object] | None = None,
        p52: object | None = None,
    ) -> tuple[object, qualify.FakeP56, list[qualify._InProcessWsl], Path, Path]:
        root = self.work / name
        root.mkdir()
        runtime = root / "runtime"
        runtime.mkdir()
        artifact = qualify._fake_release(root, "alpha" if cycle % 2 else "beta")
        p78, p51, p56_real = stack or sealed.load_exact_p78_stack(self.repo_root)
        fake = qualify.FakeP56(
            artifact,
            on_launch=on_launch,
            runner=qualify.QualificationProcessRunner(runtime, p51),
            runtime_root=runtime,
            p51=p51,
        )
        sessions: list[qualify._InProcessWsl] = []

        def open_wsl(_repo: Path, _parent: str, api: object) -> qualify._InProcessWsl:
            self.assertIs(api, p51)
            session = qualify._InProcessWsl(fake, p51)
            sessions.append(session)
            session.bind(fake.publish_retained_build_and_custody("ubuntu-24.04-x86_64", root))
            return session

        p39_checkout = qualify._synthetic_p39_checkout(root)
        patches = [
            patch.object(
                executor,
                "load_exact_p78_stack",
                return_value=(p78, p51, p56_real),
            )
        ]
        if p39_p41 is not None:
            patches.append(
                patch.object(executor, "load_exact_p39_and_p41", return_value=p39_p41)
            )
        if p52 is not None:
            patches.append(
                patch.object(executor, "load_exact_p52_stage_reader", return_value=p52)
            )
        with patches[0]:
            with patches[1] if len(patches) > 1 else _null_context():
                with patches[2] if len(patches) > 2 else _null_context():
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
                    )
        return result, fake, sessions, runtime, p39_checkout

    def _copy_exact_repo_root(self, name: str) -> Path:
        return qualify.copy_exact_repo_root(self.work / name)

    def _p35_release_root(self, repo_root: Path) -> Path:
        return (
            repo_root
            / "docs"
            / "simulations"
            / "profile-diff-held-out"
            / "pulse-35-corpus-materializer-release"
        )

    def _exact_p39_p41(self) -> tuple[object, object]:
        return sealed.load_exact_p39_and_p41(self.repo_root)

    def _load_complete_exact_graph(self, binder: object) -> tuple[object, ...]:
        p39, p41 = binder.load_exact_p39_and_p41(self.repo_root)
        p52 = binder.load_exact_p52_stage_reader(self.repo_root)
        materializer, verifier = binder.load_exact_p35_materializer_and_verifier(self.repo_root)
        p78, p51, p56 = binder.load_exact_p78_stack(self.repo_root)
        return p39, p41, p52, materializer, verifier, p78, p51, p56

    def test_production_surface_rejects_injection(self) -> None:
        with self.assertRaises(TypeError):
            executor.run_ordered_capability_materialization_executor(
                self.repo_root,
                self.work / "runtime",
                self.work / "cycle",
                self.work / "p39",
                self.work / "p41",
                "/home/x",
                seed=b"x" * 32,
            )

    def test_exact_p78_binding_and_local_loader(self) -> None:
        p78, p51, p56 = sealed.load_exact_p78_stack(self.repo_root)
        self.assertTrue(callable(p78.run_capability_bound_diagnostic_executor))
        self.assertTrue(callable(p51.validate_descriptor_root))
        self.assertTrue(callable(p56.publish_retained_build_and_custody))
        source = (ROOT / "ordered_capability_materialization_executor.py").read_text(encoding="utf-8")
        self.assertIn("_load_local_sealed_dependencies()", source)
        self.assertIn("load_exact_p78_stack", source)
        self.assertNotIn("from sealed_dependencies import", source)
        sealed_source = (ROOT / "sealed_dependencies.py").read_text(encoding="utf-8")
        self.assertIn("P35 = ReleaseIdentity(", sealed_source)
        self.assertIn("P35_VERIFIER_SOURCE_SHA256", sealed_source)
        self.assertNotIn("P35_MATERIALIZER_HASHES", sealed_source)
        self.assertNotIn("P35_VERIFIER_HASHES", sealed_source)

    def test_p35_source_matching_historical_alternate_digest_is_rejected(self) -> None:
        repo_root = self._copy_exact_repo_root("p35-old-alternate")
        release = self._p35_release_root(repo_root)
        materializer = release / "corpus_materializer.py"
        current = materializer.read_bytes()
        self.assertEqual(
            sealed.sha256_bytes(current),
            "sha256:7f74a642ce27f5742e87870e4d39d375cfa9223a40f92d253916db81260db6ba",
        )
        tampered = current.replace(b"\n", b"\r\n")
        materializer.write_bytes(tampered)
        self.assertEqual(
            sealed.sha256_bytes(tampered),
            "sha256:f531028a10127e7bc5f989eeffee45f89ffcfbe74660b3aa9eb4e8913aa3f73a",
        )
        with self.assertRaisesRegex(sealed.SealedDependencyFailure, "P81-P35-IDENTITY"):
            sealed.load_exact_p35_materializer_and_verifier(repo_root)

    def test_p35_receipt_tamper_is_rejected(self) -> None:
        repo_root = self._copy_exact_repo_root("p35-receipt-tamper")
        receipt = self._p35_release_root(repo_root) / "qualification-receipt.json"
        payload = json.loads(receipt.read_bytes())
        payload["tampered"] = True
        receipt.write_bytes(sealed.canonical_bytes(payload) + b"\n")
        with self.assertRaisesRegex(sealed.SealedDependencyFailure, "P81-P35-IDENTITY"):
            sealed.load_exact_p35_materializer_and_verifier(repo_root)

    def test_p35_seal_tamper_is_rejected(self) -> None:
        repo_root = self._copy_exact_repo_root("p35-seal-tamper")
        seal_path = self._p35_release_root(repo_root) / "release-seal.json"
        payload = json.loads(seal_path.read_bytes())
        payload["tampered"] = True
        seal_path.write_bytes(sealed.canonical_bytes(payload) + b"\n")
        with self.assertRaisesRegex(sealed.SealedDependencyFailure, "P81-P35-IDENTITY"):
            sealed.load_exact_p35_materializer_and_verifier(repo_root)

    def test_p35_extra_tree_file_is_rejected(self) -> None:
        repo_root = self._copy_exact_repo_root("p35-extra-tree-file")
        extra = self._p35_release_root(repo_root) / "extra.txt"
        extra.write_text("unexpected\n", encoding="ascii", newline="\n")
        with self.assertRaisesRegex(sealed.SealedDependencyFailure, "P81-P35-IDENTITY"):
            sealed.load_exact_p35_materializer_and_verifier(repo_root)

    def test_local_loader_ignores_ambient_state_and_returns_fresh_modules(self) -> None:
        fake = types.SimpleNamespace(marker="ambient")
        previous = sys.modules.get("sealed_dependencies")
        sys.modules["sealed_dependencies"] = fake  # type: ignore[assignment]
        try:
            first = executor._load_local_sealed_dependencies()
            setattr(first, "mutated_marker", True)
            second = executor._load_local_sealed_dependencies()
        finally:
            if previous is None:
                sys.modules.pop("sealed_dependencies", None)
            else:
                sys.modules["sealed_dependencies"] = previous
        self.assertIsNot(first, fake)
        self.assertIsNot(first, second)
        self.assertNotEqual(first.__name__, second.__name__)
        self.assertFalse(hasattr(second, "mutated_marker"))
        self.assertEqual(Path(first.__file__).resolve(), ROOT / "sealed_dependencies.py")

    def test_p39_failure_is_terminal_and_cleaned(self) -> None:
        p39, p41 = self._exact_p39_p41()

        def fail(*_args: object) -> object:
            raise p39.PublicFailure("P39-TEST-FAILURE")

        p39.verify = fail
        result, _fake, _sessions, runtime, _checkout = self._run(
            "p39-failure", p39_p41=(p39, p41)
        )
        self.assertEqual(result.private_record["failure_code"], "P52-P41-P39-PRELAUNCH")
        self.assertEqual(result.events[-1]["outcome"], "failed")
        self.assertFalse(runtime.exists())

    def test_p41_failure_is_terminal_and_cleaned(self) -> None:
        p39, p41 = self._exact_p39_p41()

        def fail(*_args: object, **_kwargs: object) -> object:
            raise p41.PublicFailure("P41-TEST-FAILURE")

        p41.copy_release = fail
        result, _fake, _sessions, runtime, _checkout = self._run(
            "p41-failure", p39_p41=(p39, p41)
        )
        self.assertEqual(result.private_record["failure_code"], "P52-P41-P39-PRELAUNCH")
        self.assertEqual(result.events[-1]["outcome"], "failed")
        self.assertFalse(runtime.exists())

    def test_public_failure_has_zero_seed(self) -> None:
        p39, p41 = self._exact_p39_p41()
        p39.verify = lambda *_args: (_ for _ in ()).throw(p39.PublicFailure("P39-FAIL"))
        with patch.object(executor.secrets, "token_bytes") as token:
            result, _fake, _sessions, runtime, _checkout = self._run(
                "zero-seed", p39_p41=(p39, p41)
            )
        token.assert_not_called()
        self.assertEqual(result.private_record["seed_calls"], 0)
        self.assertFalse(runtime.exists())

    def test_p27_bounded_failure_is_terminal_and_cleaned(self) -> None:
        _p69, p51, _p56 = sealed.load_exact_p78_stack(self.repo_root)

        def fail(_path: Path) -> dict[str, object]:
            raise p51.ExecutorFailure("P27-TEST-FAILURE")

        result, _fake, _sessions, runtime, _checkout = self._run(
            "p27-failure", p27_runner=fail
        )
        self.assertEqual(result.private_record["failure_code"], "P51-P27-EXECUTION")
        self.assertFalse(runtime.exists())

    def test_unknown_fault_reraises_after_cleanup(self) -> None:
        with patch.object(
            executor, "_execute_p57_semantics", side_effect=AssertionError("unknown")
        ):
            with self.assertRaisesRegex(AssertionError, "unknown"):
                self._run("unknown-reraise")
        self.assertFalse((self.work / "unknown-reraise" / "runtime").exists())

    def test_unknown_fault_cleanup_indeterminate_wins(self) -> None:
        p52 = sealed.load_exact_p52_stage_reader(self.repo_root)
        _p69, p51, _p56 = sealed.load_exact_p78_stack(self.repo_root)

        def fail_cleanup(*_args: object, **_kwargs: object) -> None:
            raise p51.ExecutorFailure("P58-TEST-CLEANUP")

        p52._remove_private_tree = fail_cleanup
        with patch.object(
            executor, "_execute_p57_semantics", side_effect=AssertionError("unknown")
        ):
            with self.assertRaisesRegex(
                executor.IndeterminateCleanup, "P58-INDETERMINATE-CLEANUP"
            ):
                self._run("unknown-indeterminate", p52=p52)

    def test_second_seed_is_prohibited_behaviorally(self) -> None:
        result, _fake, _sessions, _runtime, _checkout = self._run("one-seed")
        self.assertEqual(result.private_record["seed_calls"], 1)
        self.assertEqual(result.private_record["seed_byte_count"], 32)
        self.assertEqual(result.private_record["seed_cleanup"], "removed-after-verification")

    def test_second_materialization_is_prohibited_behaviorally(self) -> None:
        result, _fake, _sessions, _runtime, _checkout = self._run("one-materialization")
        self.assertEqual(result.private_record["materializer_invocations"], 1)
        self.assertEqual(result.private_record["verifier_invocations"], 1)

    def test_capabilities_are_not_rebuilt_after_seed(self) -> None:
        result, fake, _sessions, _runtime, _checkout = self._run("capability-order")
        self.assertEqual(result.private_record["seed_calls"], 1)
        self.assertEqual(fake.publishes, ["windows-x86_64", "ubuntu-24.04-x86_64"])

    def test_ordinal_69_ubuntu_failure_preserves_failure_and_windows_expiry(self) -> None:
        def fail_ubuntu(platform: str, _argv: tuple[str, ...], count: int) -> None:
            if platform == "ubuntu-24.04-x86_64" and count == 138:
                raise FakeReleaseFailure("P56-ORDINAL-69")

        result, fake, sessions, runtime, _checkout = self._run(
            "ordinal-69-ubuntu-failure", on_launch=fail_ubuntu
        )
        self.assertEqual(result.private_record["failure_code"], "P56-ORDINAL-69")
        self.assertEqual(result.private_record["process_counts"]["windows-x86_64"], 69)
        self.assertEqual(result.private_record["process_counts"]["ubuntu-24.04-x86_64"], 68)
        self.assertEqual(fake.closes, ["ubuntu-24.04-x86_64"])
        self.assertEqual(len(sessions[0].requests), 68)
        self.assertFalse(runtime.exists())

    def test_ordinal_69_semantic_mismatch_preserves_failure_and_windows_expiry(self) -> None:
        p78, p51, p56 = sealed.load_exact_p78_stack(self.repo_root)
        original = p78._normalize_result
        calls = 0

        def mismatch(api: object, frozen: object, capture: object) -> dict[str, object]:
            nonlocal calls
            calls += 1
            normalized = original(api, frozen, capture)
            if calls == 138:
                return {**normalized, "semantic_projection": {"forced": "mismatch"}}
            return normalized

        p78._normalize_result = mismatch
        result, fake, sessions, runtime, _checkout = self._run(
            "ordinal-69-mismatch", stack=(p78, p51, p56)
        )
        self.assertEqual(result.private_record["failure_code"], "P57-FIRST-TARGET-MISMATCH")
        self.assertEqual(result.private_record["first_mismatch_ordinal"], 69)
        self.assertEqual(fake.closes, [])
        self.assertEqual(len(sessions[0].requests), 69)
        self.assertFalse(runtime.exists())

    def test_directory_symlink_and_wsl_no_follow_are_rejected(self) -> None:
        real = self.work / "directory-real"
        link = self.work / "directory-link"
        real.mkdir()
        try:
            os.symlink(real, link, target_is_directory=True)
        except OSError as error:
            if getattr(error, "winerror", None) != 1314:
                raise
        else:
            with self.assertRaises(executor.P58Failure):
                executor._directory_identity(link)

        wsl_root = _to_wsl(self.work / "wsl-no-follow")
        wsl_release = _to_wsl(ROOT)
        program = "\n".join(
            (
                "import os",
                "import pathlib",
                "import sys",
                "sys.path.insert(0, sys.argv[1])",
                "import ordered_capability_materialization_executor as e",
                "root = pathlib.Path(sys.argv[2])",
                "root.mkdir()",
                "real = root / 'real'",
                "real.mkdir()",
                "link = root / 'link'",
                "os.symlink(real, link)",
                "try:",
                "    e._directory_identity(link)",
                "except e.P58Failure:",
                "    pass",
                "else:",
                "    raise SystemExit(17)",
            )
        )
        completed = subprocess.run(
            [
                "wsl.exe",
                "--distribution",
                "Ubuntu-24.04",
                "--exec",
                "/usr/bin/python3",
                "-I",
                "-S",
                "-B",
                "-c",
                program,
                wsl_release,
                wsl_root,
            ],
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        self.assertEqual(
            completed.returncode,
            0,
            completed.stderr.decode("utf-8", "replace"),
        )

    def test_directory_substitution_race_is_rejected(self) -> None:
        root = self.work / "directory-substitution"
        root.mkdir()
        directory = root / "directory"
        replacement = root / "replacement"
        directory.mkdir()
        identity = executor._directory_identity(directory)
        replacement.mkdir()
        os.rmdir(directory)
        os.replace(replacement, directory)
        with self.assertRaisesRegex(executor.P58Failure, "P57-P51-INPUT-SUBSTITUTION"):
            executor._same_lexical_identity(root, identity)

    def test_worker_replay_is_rejected(self) -> None:
        artifact = qualify._fake_release(self.work, "alpha")
        fake = qualify.FakeP56(artifact)
        session = qualify._InProcessWsl(fake, object())
        session.bind(fake.publish_retained_build_and_custody("ubuntu-24.04-x86_64", self.work))
        with self.assertRaisesRegex(RuntimeError, "protocol"):
            session.launch(
                2,
                ("profile-diff", "--before", "x", "--after", "y", "--format", "json"),
            )

    def test_first_semantic_mismatch_stops_execution(self) -> None:
        p78, p51, p56 = sealed.load_exact_p78_stack(self.repo_root)
        original = p78._normalize_result
        calls = 0

        def mismatch(api: object, frozen: object, capture: object) -> dict[str, object]:
            nonlocal calls
            calls += 1
            normalized = original(api, frozen, capture)
            if calls == 2:
                return {**normalized, "semantic_projection": {"forced": "mismatch"}}
            return normalized

        p78._normalize_result = mismatch
        result, fake, _sessions, runtime, _checkout = self._run(
            "first-mismatch", stack=(p78, p51, p56)
        )
        self.assertEqual(result.private_record["first_mismatch_ordinal"], 1)
        self.assertEqual(len(fake.launches), 2)
        self.assertFalse(runtime.exists())

    def test_no_launch_topology_is_recorded(self) -> None:
        result, fake, _sessions, _runtime, _checkout = self._run("no-launch")
        self.assertEqual(result.private_record["process_counts"], {
            "windows-x86_64": 69,
            "ubuntu-24.04-x86_64": 69,
        })
        self.assertEqual(len(result.private_record["no_launch_records"]), 2)
        self.assertEqual(len(fake.launches), 138)

    def test_qualification_uses_synthetic_p39_root(self) -> None:
        checkout = qualify._synthetic_p39_checkout(self.work / "p39-only")
        self.assertNotEqual(checkout, self.repo_root)
        p39, _p41 = self._exact_p39_p41()
        summary = p39.verify(str(checkout), p39.PULSE_25_ROOT, p39.PULSE_27_ROOT)
        self.assertEqual(summary["status"], "pass")

    def test_final_cleanup_removes_private_runtime(self) -> None:
        result, _fake, _sessions, runtime, _checkout = self._run("final-cleanup")
        self.assertEqual(result.private_record["private_root_cleanup"], "removed-and-verified")
        self.assertEqual(result.private_record["descriptor_cleanup"], "removed-and-verified")
        self.assertFalse(runtime.exists())

    def test_concurrent_100_exact_p78_loads_are_serialized(self) -> None:
        binder = executor._load_local_sealed_dependencies()
        barrier = threading.Barrier(100)
        errors: list[str] = []

        def worker(index: int) -> None:
            try:
                barrier.wait(timeout=30)
                p39, p41, p52, materializer, verifier, p78, p51, p56 = (
                    self._load_complete_exact_graph(binder)
                )
                if not callable(p39.verify):
                    raise AssertionError("missing exact Pulse 39 verifier")
                if not callable(p41.copy_release):
                    raise AssertionError("missing exact Pulse 41 copier")
                if not callable(p52._verify_public_prelaunch_custody):
                    raise AssertionError("missing exact Pulse 52 stage reader")
                if not callable(materializer.materialize):
                    raise AssertionError("missing exact Pulse 35 materializer")
                if not callable(verifier.verify):
                    raise AssertionError("missing exact Pulse 35 verifier")
                if not callable(p78.run_capability_bound_diagnostic_executor):
                    raise AssertionError("missing exact Pulse 78 executor")
                if not callable(p51.validate_descriptor_root):
                    raise AssertionError("missing exact Pulse 51 validator")
                if not callable(p56.publish_retained_build_and_custody):
                    raise AssertionError("missing exact Pulse 56 publisher")
                time.sleep(0.005)
            except BaseException as error:
                errors.append(f"{index}:{error!r}")

        threads = [threading.Thread(target=worker, args=(index,)) for index in range(100)]
        for thread in threads:
            thread.start()
        for thread in threads:
            thread.join(timeout=120)
        self.assertFalse(any(thread.is_alive() for thread in threads), "thread load deadlock")
        self.assertEqual(errors, [])

    def test_kernel_lock_process_stress_serializes_subprocesses(self) -> None:
        script = self._write_subprocess_script(
            "kernel_lock_process_stress.py",
            """
            start = Path(sys.argv[1])
            marker = Path(sys.argv[2])
            repo_root = Path(sys.argv[3])
            index = sys.argv[4]
            binder = executor._load_local_sealed_dependencies()
            deadline = time.monotonic() + 15.0
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
                p39, p41 = binder.load_exact_p39_and_p41(repo_root)
                p52 = binder.load_exact_p52_stage_reader(repo_root)
                materializer, verifier = binder.load_exact_p35_materializer_and_verifier(
                    repo_root
                )
                p78, p51, p56 = binder.load_exact_p78_stack(repo_root)
                if not callable(p39.verify):
                    raise AssertionError("missing exact Pulse 39 verifier")
                if not callable(p41.copy_release):
                    raise AssertionError("missing exact Pulse 41 copier")
                if not callable(p52._verify_public_prelaunch_custody):
                    raise AssertionError("missing exact Pulse 52 stage reader")
                if not callable(materializer.materialize):
                    raise AssertionError("missing exact Pulse 35 materializer")
                if not callable(verifier.verify):
                    raise AssertionError("missing exact Pulse 35 verifier")
                if not callable(p78.run_capability_bound_diagnostic_executor):
                    raise AssertionError("missing exact Pulse 78 executor")
                if not callable(p51.validate_descriptor_root):
                    raise AssertionError("missing exact Pulse 51 validator")
                if not callable(p56.publish_retained_build_and_custody):
                    raise AssertionError("missing exact Pulse 56 publisher")
                time.sleep(0.02)
                marker.unlink()
            """,
        )
        start = self.work / "process-stress-start.flag"
        marker = self.work / "process-stress-marker"
        processes = [
            subprocess.Popen(
                [
                    sys.executable,
                    "-B",
                    os.fspath(script),
                    os.fspath(start),
                    os.fspath(marker),
                    os.fspath(self.repo_root),
                    str(index),
                ],
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
            stdout, stderr = process.communicate(timeout=60)
            if process.returncode != 0:
                failures.append((index, process.returncode, stdout, stderr))
        self.assertFalse(marker.exists())
        if failures:
            self.fail(f"kernel lock subprocess stress failures: {failures}")

    def test_release_generator_rejects_python_cache_residue(self) -> None:
        residue = ROOT / "__pycache__"
        residue.mkdir()
        cache = residue / "p58-control.pyc"
        cache.write_bytes(b"not-a-bytecode-cache")
        try:
            with self.assertRaisesRegex(RuntimeError, "Python cache residue"):
                generate_release.public_files()
        finally:
            cache.unlink(missing_ok=True)
            residue.rmdir()


class _null_context:
    def __enter__(self) -> None:
        return None

    def __exit__(self, *_args: object) -> bool:
        return False


def _to_wsl(path: Path) -> str:
    value = os.fspath(path.absolute()).replace("\\", "/")
    if len(value) < 3 or value[1] != ":":
        raise AssertionError("expected a Windows drive path")
    return "/mnt/" + value[0].lower() + value[2:]


if __name__ == "__main__":
    unittest.main()
