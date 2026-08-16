from __future__ import annotations

import contextlib
import inspect
import shutil
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

import capability_bound_diagnostic_executor_successor as executor  # noqa: E402
from fixtures.fake_p56 import FakeP56  # noqa: E402
from fixtures.p69_fake_native_wsl import (  # noqa: E402
    FakeBundleManager,
    FakeWorkerProcessFactory,
)
from sealed_dependencies import load_exact_p57_stack, load_p51_synthetic_fixture  # noqa: E402


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


class CapabilityBoundDiagnosticExecutorSuccessorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.work = REPO_ROOT / "target" / f"pulse-69-test-{uuid.uuid4().hex}"
        self.work.mkdir(parents=True)
        self.addCleanup(lambda: shutil.rmtree(self.work) if self.work.exists() else None)

    def _fixture_root(self, p51: object) -> Path:
        fixture = load_p51_synthetic_fixture(REPO_ROOT, p51)
        return fixture.create_descriptor_root(self.work / "descriptors")

    def _fake_artifact(self, label: str) -> Path:
        release = self.work / f"fake-release-{label}"
        (release / "fixtures").mkdir(parents=True)
        for dependency in ("frozen_profile_diff.py", "p31_contract_verifier.py"):
            shutil.copyfile(P51_ROOT / dependency, release / dependency)
        artifact = release / "fixtures" / "fake_ferris.py"
        artifact.write_bytes(
            (P51_ROOT / "fixtures" / "fake_ferris.py").read_bytes()
            + f"\n# pulse-69-{label}\n".encode("ascii")
        )
        return artifact

    def _controls(self, fake: FakeP56, p51: object) -> executor._Controls:
        return executor._Controls(
            p51,
            fake,
            p27_success,
            lambda root, parent, api: executor._NativeWslSession(root, parent, api),
        )

    def _patch_native(
        self,
        manager: FakeBundleManager,
        process_factory: FakeWorkerProcessFactory,
    ) -> contextlib.ExitStack:
        stack = contextlib.ExitStack()
        stack.enter_context(patch.object(executor, "_stage_owned_bundle", manager.stage))
        stack.enter_context(patch.object(executor, "_cleanup_owned_bundle", manager.cleanup))
        stack.enter_context(patch.object(executor, "_spawn_wsl_worker", process_factory))
        return stack

    def test_exact_p57_binding_and_production_signature_match(self) -> None:
        p57, p51, p56 = load_exact_p57_stack(REPO_ROOT)
        self.assertTrue(callable(p51.validate_descriptor_root))
        self.assertTrue(callable(p56.publish_retained_build_and_custody))
        self.assertEqual(executor.ExecutorFailure.__name__, p57.ExecutorFailure.__name__)
        self.assertEqual(executor.REQUEST_COUNT, p57.REQUEST_COUNT)
        self.assertEqual(
            tuple(inspect.signature(executor.run_capability_bound_diagnostic_executor).parameters),
            (
                "repo_root",
                "descriptor_root",
                "private_runtime_root",
                "p27_cycle_root",
                "ubuntu_runtime_parent",
            ),
        )
        source = (
            ROOT / "capability_bound_diagnostic_executor_successor.py"
        ).read_text(encoding="utf-8")
        self.assertIn("load_exact_p57_stack(REPO_ROOT)", source)
        self.assertIn("_stage_owned_bundle", source)
        self.assertIn("_cleanup_owned_bundle", source)
        self.assertNotIn("retained_custodies", source)
        self.assertNotIn("process_runner", source)

    def test_cleanup_bootstrap_uses_no_follow_identity_and_parent_sync(self) -> None:
        source = (
            ROOT / "capability_bound_diagnostic_executor_successor.py"
        ).read_text(encoding="utf-8")
        for snippet in (
            "O_NOFOLLOW",
            "dir_fd=parent_fd",
            "expected_children",
            "os.fsync",
            "root_inode",
            "parent_inode",
            "bundle_absent",
            "stat.S_ISLNK",
        ):
            self.assertIn(snippet, source)

    def test_full_fake_cycle_retains_bundle_until_close_and_leaves_no_residue(self) -> None:
        _p57, p51, _p56 = load_exact_p57_stack(REPO_ROOT)
        fake = FakeP56(self._fake_artifact("alpha"))
        manager = FakeBundleManager(executor, self.work / "bundles")
        process_factory = FakeWorkerProcessFactory(executor, fake, self.work)
        with self._patch_native(manager, process_factory):
            result = executor._run_qualification_executor(
                REPO_ROOT,
                self._fixture_root(p51),
                self.work,
                self.work / "p27-cycle",
                self._controls(fake, p51),
            )
        self.assertEqual(result.private_record["outcome"], "completed")
        self.assertEqual(
            result.private_record["process_counts"],
            {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69},
        )
        self.assertEqual(len(result.private_record["no_launch_records"]), 2)
        self.assertEqual(len(fake.launches), 138)
        self.assertEqual(len(manager.records), 1)
        self.assertEqual(manager.records[0].cleanup_calls, 1)
        self.assertFalse(manager.residue())
        self.assertEqual(len(process_factory.processes), 1)
        self.assertEqual(len(process_factory.processes[0].requests), 69)

    def test_startup_failure_removes_staged_bundle(self) -> None:
        _p57, p51, _p56 = load_exact_p57_stack(REPO_ROOT)
        fake = FakeP56(self._fake_artifact("startup"))
        manager = FakeBundleManager(executor, self.work / "bundles")
        process_factory = FakeWorkerProcessFactory(
            executor,
            fake,
            self.work,
            startup_error=OSError("startup read failed"),
        )
        with self._patch_native(manager, process_factory):
            with self.assertRaisesRegex(executor.ExecutorFailure, "P57-WSL-PROTOCOL"):
                executor._NativeWslSession(REPO_ROOT, "/home/runtime", p51)
        self.assertEqual(len(manager.records), 1)
        self.assertEqual(manager.records[0].cleanup_calls, 1)
        self.assertFalse(manager.residue())

    def test_close_timeout_kills_worker_and_removes_bundle(self) -> None:
        _p57, p51, _p56 = load_exact_p57_stack(REPO_ROOT)
        fake = FakeP56(self._fake_artifact("timeout"))
        manager = FakeBundleManager(executor, self.work / "bundles")
        process_factory = FakeWorkerProcessFactory(
            executor,
            fake,
            self.work,
            close_timeout=True,
        )
        with self._patch_native(manager, process_factory):
            session = executor._NativeWslSession(REPO_ROOT, "/home/runtime", p51)
            with self.assertRaisesRegex(executor.ExecutorFailure, "P57-WSL-CLEANUP"):
                session.close()
        self.assertTrue(process_factory.processes[0].terminated)
        self.assertTrue(process_factory.processes[0].killed)
        self.assertEqual(manager.records[0].cleanup_calls, 1)
        self.assertFalse(manager.residue())

    def test_root_substitution_is_rejected_before_cleanup(self) -> None:
        _p57, p51, _p56 = load_exact_p57_stack(REPO_ROOT)
        fake = FakeP56(self._fake_artifact("substitution"))
        manager = FakeBundleManager(executor, self.work / "bundles")
        process_factory = FakeWorkerProcessFactory(executor, fake, self.work)

        stale_paths: list[Path] = []

        def substitute(path: Path, _bundle: object, _record: object) -> None:
            stale = path.with_name(path.name + "-stale")
            path.rename(stale)
            path.mkdir()
            stale_paths.append(stale)

        manager.before_cleanup = substitute
        with self._patch_native(manager, process_factory):
            session = executor._NativeWslSession(REPO_ROOT, "/home/runtime", p51)
            with self.assertRaisesRegex(executor.ExecutorFailure, "P57-WSL-CLEANUP"):
                session.close()
        self.assertTrue(Path(manager.records[0].root).exists())
        self.assertTrue(stale_paths[0].exists())

    def test_cleanup_failure_takes_precedence_over_protocol_failure(self) -> None:
        _p57, p51, _p56 = load_exact_p57_stack(REPO_ROOT)
        fake = FakeP56(self._fake_artifact("precedence"))
        manager = FakeBundleManager(executor, self.work / "bundles")
        manager.cleanup_failure = executor.ExecutorFailure("P57-WSL-CLEANUP")
        process_factory = FakeWorkerProcessFactory(
            executor,
            fake,
            self.work,
            launch_mutator=lambda _request, _raw: b"{}\n",
        )
        with self._patch_native(manager, process_factory):
            result = executor._run_qualification_executor(
                REPO_ROOT,
                self._fixture_root(p51),
                self.work,
                self.work / "p27-cycle",
                self._controls(fake, p51),
            )
        self.assertEqual(result.private_record["failure_code"], "P57-INDETERMINATE-CLEANUP")
        self.assertEqual(manager.records[0].cleanup_calls, 1)

    def test_two_sessions_remove_only_their_owned_bundles(self) -> None:
        _p57, p51, _p56 = load_exact_p57_stack(REPO_ROOT)
        fake = FakeP56(self._fake_artifact("concurrent"))
        manager = FakeBundleManager(executor, self.work / "bundles")
        process_factory = FakeWorkerProcessFactory(executor, fake, self.work)
        with self._patch_native(manager, process_factory):
            first = executor._NativeWslSession(REPO_ROOT, "/home/runtime", p51)
            second = executor._NativeWslSession(REPO_ROOT, "/home/runtime", p51)
            first.close()
            second.close()
        self.assertEqual([record.cleanup_calls for record in manager.records], [1, 1])
        self.assertFalse(manager.residue())


if __name__ == "__main__":
    unittest.main()
