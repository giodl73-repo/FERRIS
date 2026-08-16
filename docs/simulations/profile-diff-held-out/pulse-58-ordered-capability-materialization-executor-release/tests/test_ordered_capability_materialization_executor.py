from __future__ import annotations

import os
import shutil
import subprocess
import sys
import unittest
import uuid
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = ROOT.parents[3]
sys.dont_write_bytecode = True
sys.path.insert(0, str(ROOT))

import ordered_capability_materialization_executor as executor  # noqa: E402
import generate_release  # noqa: E402
import qualify  # noqa: E402
import sealed_dependencies as sealed  # noqa: E402
from fixtures.fake_p56 import FakeReleaseFailure  # noqa: E402


class OrderedCapabilityMaterializationExecutorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.work = REPO_ROOT / "target" / f"pulse-58-test-{uuid.uuid4().hex}"
        self.work.mkdir(parents=True)
        self.addCleanup(lambda: shutil.rmtree(self.work, ignore_errors=True))

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
        p57, p51, p56_real = stack or sealed.load_exact_p57_stack(REPO_ROOT)
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
                "load_exact_p57_stack",
                return_value=(p57, p51, p56_real),
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
                        REPO_ROOT,
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

    def _exact_p39_p41(self) -> tuple[object, object]:
        return sealed.load_exact_p39_and_p41(REPO_ROOT)

    def test_production_surface_rejects_injection(self) -> None:
        with self.assertRaises(TypeError):
            executor.run_ordered_capability_materialization_executor(
                REPO_ROOT,
                self.work / "runtime",
                self.work / "cycle",
                self.work / "p39",
                self.work / "p41",
                "/home/x",
                seed=b"x" * 32,
            )

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
        _p57, p51, _p56 = sealed.load_exact_p57_stack(REPO_ROOT)

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
        p52 = sealed.load_exact_p52_stage_reader(REPO_ROOT)
        _p57, p51, _p56 = sealed.load_exact_p57_stack(REPO_ROOT)

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
        p57, p51, p56 = sealed.load_exact_p57_stack(REPO_ROOT)
        original = p57._normalize_result
        calls = 0

        def mismatch(api: object, frozen: object, capture: object) -> dict[str, object]:
            nonlocal calls
            calls += 1
            normalized = original(api, frozen, capture)
            if calls == 138:
                return {**normalized, "semantic_projection": {"forced": "mismatch"}}
            return normalized

        p57._normalize_result = mismatch
        result, fake, sessions, runtime, _checkout = self._run(
            "ordinal-69-mismatch", stack=(p57, p51, p56)
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
        p57, p51, p56 = sealed.load_exact_p57_stack(REPO_ROOT)
        original = p57._normalize_result
        calls = 0

        def mismatch(api: object, frozen: object, capture: object) -> dict[str, object]:
            nonlocal calls
            calls += 1
            normalized = original(api, frozen, capture)
            if calls == 2:
                return {**normalized, "semantic_projection": {"forced": "mismatch"}}
            return normalized

        p57._normalize_result = mismatch
        result, fake, _sessions, runtime, _checkout = self._run(
            "first-mismatch", stack=(p57, p51, p56)
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
        self.assertNotEqual(checkout, REPO_ROOT)
        p39, _p41 = self._exact_p39_p41()
        summary = p39.verify(str(checkout), p39.PULSE_25_ROOT, p39.PULSE_27_ROOT)
        self.assertEqual(summary["status"], "pass")

    def test_final_cleanup_removes_private_runtime(self) -> None:
        result, _fake, _sessions, runtime, _checkout = self._run("final-cleanup")
        self.assertEqual(result.private_record["private_root_cleanup"], "removed-and-verified")
        self.assertEqual(result.private_record["descriptor_cleanup"], "removed-and-verified")
        self.assertFalse(runtime.exists())

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
