from __future__ import annotations

import base64
import contextlib
import hashlib
import inspect
import json
import os
import shutil
import subprocess
import sys
import types
import unittest
import uuid
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = ROOT.parents[3]
PROFILE_DIFF_ROOT = (
    REPO_ROOT
    / "docs"
    / "simulations"
    / "profile-diff-held-out"
)
for cache in PROFILE_DIFF_ROOT.rglob("__pycache__"):
    shutil.rmtree(cache, ignore_errors=True)

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
from fixtures.p75_fake_native_wsl import (  # noqa: E402
    FakeBundleManager,
    FakeWorkerProcessFactory,
)
import sealed_dependencies as local_sealed  # noqa: E402

executor._bind_local_sealed_lock_manager_module(local_sealed)
load_exact_p72_stack = local_sealed.load_exact_p72_stack
load_p51_synthetic_fixture = local_sealed.load_p51_synthetic_fixture


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


class CapabilityBoundDiagnosticExecutorStageBootstrapWorkerIdentitySuccessorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.work = REPO_ROOT / "target" / f"pulse-75-test-{uuid.uuid4().hex}"
        self.work.mkdir(parents=True)
        self.addCleanup(lambda: shutil.rmtree(self.work) if self.work.exists() else None)
        self._wsl_paths: list[str] = []
        self.addCleanup(self._cleanup_wsl_paths)

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
            + f"\n# pulse-75-{label}\n".encode("ascii")
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
        stack.enter_context(patch.object(executor, "_revalidate_staged_bundle", manager.revalidate))
        stack.enter_context(patch.object(executor, "_cleanup_owned_bundle", manager.cleanup))
        stack.enter_context(patch.object(executor, "_spawn_wsl_worker", process_factory))
        return stack

    def _wsl_run(
        self,
        script: str,
        *arguments: str,
        input_bytes: bytes | None = None,
        timeout: int = 120,
    ) -> subprocess.CompletedProcess[bytes]:
        return subprocess.run(
            [
                executor._P57._wsl_executable(),
                "--distribution",
                "Ubuntu-24.04",
                "--exec",
                "/usr/bin/python3",
                "-I",
                "-S",
                "-B",
                "-c",
                script,
                *arguments,
            ],
            input=input_bytes,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
            env=executor._P57._wsl_environment(),
            timeout=timeout,
        )

    def _wsl_parent(self, label: str) -> str:
        completed = self._wsl_run(
            "\n".join(
                (
                    "import pathlib,sys,uuid",
                    "root = pathlib.Path.home() / '.pulse75-tests'",
                    "root.mkdir(exist_ok=True)",
                    "parent = root / (sys.argv[1] + '-' + uuid.uuid4().hex)",
                    "parent.mkdir()",
                    "sys.stdout.buffer.write(parent.as_posix().encode('utf-8'))",
                )
            ),
            label,
        )
        self.assertEqual(
            completed.returncode,
            0,
            completed.stderr.decode("utf-8", "replace"),
        )
        parent = completed.stdout.decode("utf-8")
        self._wsl_paths.append(parent)
        return parent

    def _cleanup_wsl_paths(self) -> None:
        for path in reversed(self._wsl_paths):
            try:
                self._wsl_run(
                    "import shutil,sys; shutil.rmtree(sys.argv[1], ignore_errors=True)",
                    path,
                    timeout=30,
                )
            except BaseException:
                pass

    def _create_wsl_bundle(
        self,
        parent: str,
        *,
        worker_source: bytes,
        dependency_source: bytes,
    ) -> dict[str, object]:
        payload = executor._P57.canonical_bytes(
            {
                "dependency_b64": base64.b64encode(dependency_source).decode("ascii"),
                "worker_b64": base64.b64encode(worker_source).decode("ascii"),
            }
        )
        completed = self._wsl_run(
            "\n".join(
                (
                    "import base64,hashlib,json,pathlib,sys",
                    "request = json.loads(sys.stdin.buffer.read())",
                    "parent = pathlib.Path(sys.argv[1])",
                    "root = parent / '.p57-bundle'",
                    "(root / 'worker').mkdir(parents=True)",
                    "worker = base64.b64decode(request['worker_b64'])",
                    "dependency = base64.b64decode(request['dependency_b64'])",
                    "(root / 'worker' / 'wsl_session_worker.py').write_bytes(worker)",
                    "(root / 'worker' / 'sealed_dependencies.py').write_bytes(dependency)",
                    "parent_stat = parent.stat()",
                    "root_stat = root.stat()",
                    "response = {",
                    "    'bundle_root': root.as_posix(),",
                    "    'dependency_sha256': 'sha256:' + hashlib.sha256(dependency).hexdigest(),",
                    "    'name': root.name,",
                    "    'parent_device': parent_stat.st_dev,",
                    "    'parent_inode': parent_stat.st_ino,",
                    "    'runtime_parent': parent.as_posix(),",
                    "    'root_device': root_stat.st_dev,",
                    "    'root_inode': root_stat.st_ino,",
                    "    'worker_sha256': 'sha256:' + hashlib.sha256(worker).hexdigest(),",
                    "}",
                    "sys.stdout.buffer.write(json.dumps(response, sort_keys=True).encode('utf-8'))",
                )
            ),
            parent,
            input_bytes=payload,
        )
        self.assertEqual(
            completed.returncode,
            0,
            completed.stderr.decode("utf-8", "replace"),
        )
        return json.loads(completed.stdout)

    def test_exact_p72_binding_and_production_signature_match(self) -> None:
        p72, p57, p51, p56 = load_exact_p72_stack(REPO_ROOT)
        self.assertTrue(callable(p72.run_capability_bound_diagnostic_executor))
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
        source = (ROOT / "capability_bound_diagnostic_executor_successor.py").read_text(
            encoding="utf-8"
        )
        self.assertIn("class _Pulse75LinuxLockManager", source)
        self.assertIn("LOCAL_SEALED_DEPENDENCIES_SHA256", source)
        self.assertIn("_SEALED.load_exact_p72_stack(REPO_ROOT)", source)
        self.assertIn("_revalidate_staged_bundle", source)
        self.assertNotIn("from sealed_dependencies import", source)
        self.assertNotIn("retained_custodies", source)
        self.assertNotIn("process_runner", source)

    def test_local_loader_reads_sibling_module_not_ambient_state(self) -> None:
        fake = types.SimpleNamespace(marker="ambient")
        previous = sys.modules.get("sealed_dependencies")
        sys.modules["sealed_dependencies"] = fake  # type: ignore[assignment]
        try:
            loaded = executor._load_local_sealed_dependencies()
        finally:
            if previous is None:
                sys.modules.pop("sealed_dependencies", None)
            else:
                sys.modules["sealed_dependencies"] = previous
        self.assertIsNot(loaded, fake)
        self.assertEqual(
            Path(loaded.__file__).resolve(),
            ROOT / "sealed_dependencies.py",
        )
        self.assertTrue(callable(loaded.load_exact_p72_stack))

    def test_local_loader_returns_fresh_module_each_time(self) -> None:
        first = executor._load_local_sealed_dependencies()
        setattr(first, "mutated_marker", True)
        second = executor._load_local_sealed_dependencies()
        self.assertIsNot(first, second)
        self.assertNotEqual(first.__name__, second.__name__)
        self.assertFalse(hasattr(second, "mutated_marker"))

    def test_stage_bootstrap_binds_identity_and_revalidates_before_spawn(self) -> None:
        source = (ROOT / "capability_bound_diagnostic_executor_successor.py").read_text(
            encoding="utf-8"
        )
        for snippet in (
            "_WSL_BUNDLE_STAGE_BOOTSTRAP",
            "_WSL_BUNDLE_REVALIDATION_BOOTSTRAP",
            "_WSL_WORKER_BOOTSTRAP",
            "_cleanup_created_root",
            "root_type",
            "parent_type",
            "WORKER_SEALED_DEPENDENCIES_SHA256",
            "worker_sealed_dependencies.py",
            "--expected-sealed-dependencies-sha256",
            "dir_fd=current_fd",
            "os.mkdir(name,0o700,dir_fd=parent_fd)",
            "_revalidate_staged_bundle(self._bundle)",
        ):
            self.assertIn(snippet, source)

    def test_stage_post_create_failure_cleanup_is_owned_inside_bootstrap(self) -> None:
        parent = self._wsl_parent("post-create-failure")
        payload = executor._P57.canonical_bytes(
            {
                "files": [
                    {"bytes_b64": base64.b64encode(b"a\n").decode("ascii"), "path": "worker/a.py"},
                    {"bytes_b64": base64.b64encode(b"b\n").decode("ascii"), "path": "worker/a.py"},
                ],
                "schema": executor._P57.BUNDLE_SCHEMA,
            }
        )
        completed = self._wsl_run(
            executor._WSL_BUNDLE_STAGE_BOOTSTRAP,
            parent,
            ".p57-stage-failure",
            input_bytes=payload,
        )
        self.assertEqual(completed.returncode, 0, completed.stderr.decode("utf-8", "replace"))
        response = executor._P57._parse_line(completed.stdout, executor.MAX_PROTOCOL_BYTES)
        self.assertEqual(
            response,
            {
                "bundle_absent_verified": True,
                "bundle_root": parent + "/.p57-stage-failure",
                "cleanup_posture": "removed",
                "schema": executor._BUNDLE_STAGE_SCHEMA,
                "status": "failed",
            },
        )
        exists = self._wsl_run(
            "import os,sys; sys.stdout.buffer.write(b'1' if os.path.exists(sys.argv[1]) else b'0')",
            response["bundle_root"],
        )
        self.assertEqual(exists.stdout, b"0")

    def test_stage_failure_cleanup_indeterminate_takes_precedence(self) -> None:
        expected_root = "/home/runtime/.p57-test"
        response = executor._P57._canonical_line(
            {
                "bundle_absent_verified": False,
                "bundle_root": expected_root,
                "cleanup_posture": "indeterminate",
                "schema": executor._BUNDLE_STAGE_SCHEMA,
                "status": "failed",
            }
        )
        completed = types.SimpleNamespace(returncode=0, stderr=b"", stdout=response)
        with (
            patch.object(executor.subprocess, "run", return_value=completed),
            patch.object(executor._P57.secrets, "token_hex", return_value="test"),
        ):
            with self.assertRaisesRegex(executor.ExecutorFailure, "P57-INDETERMINATE-CLEANUP"):
                executor._stage_owned_bundle(REPO_ROOT, "/home/runtime")

    def test_worker_bootstrap_rejects_root_swap_after_revalidation(self) -> None:
        worker = b"raise SystemExit(0)\n"
        dependency = b"pass\n"
        parent = self._wsl_parent("root-swap")
        bundle = self._create_wsl_bundle(
            parent,
            worker_source=worker,
            dependency_source=dependency,
        )
        completed = self._wsl_run(
            "\n".join(
                (
                    "import pathlib,sys",
                    "root = pathlib.Path(sys.argv[1])",
                    "stale = root.with_name(root.name + '-stale')",
                    "root.rename(stale)",
                    "(root / 'worker').mkdir(parents=True)",
                    "(root / 'worker' / 'wsl_session_worker.py').write_bytes((stale / 'worker' / 'wsl_session_worker.py').read_bytes())",
                    "(root / 'worker' / 'sealed_dependencies.py').write_bytes((stale / 'worker' / 'sealed_dependencies.py').read_bytes())",
                )
            ),
            str(bundle["bundle_root"]),
        )
        self.assertEqual(completed.returncode, 0, completed.stderr.decode("utf-8", "replace"))
        launched = self._wsl_run(
            executor._WSL_WORKER_BOOTSTRAP,
            str(bundle["runtime_parent"]),
            str(bundle["name"]),
            str(bundle["bundle_root"]),
            str(bundle["parent_device"]),
            str(bundle["parent_inode"]),
            str(bundle["root_device"]),
            str(bundle["root_inode"]),
            "worker/wsl_session_worker.py",
            str(bundle["worker_sha256"]),
            "worker/sealed_dependencies.py",
            str(bundle["dependency_sha256"]),
        )
        self.assertEqual(launched.returncode, 2)
        self.assertEqual(launched.stdout, b"")
        self.assertEqual(launched.stderr, b"")

    def test_worker_bootstrap_rejects_worker_path_swap(self) -> None:
        worker = b"raise SystemExit(0)\n"
        dependency = b"pass\n"
        parent = self._wsl_parent("worker-swap")
        bundle = self._create_wsl_bundle(
            parent,
            worker_source=worker,
            dependency_source=dependency,
        )
        completed = self._wsl_run(
            "import pathlib,sys; pathlib.Path(sys.argv[1]).write_bytes(b\"raise SystemExit(1)\\n\")",
            str(bundle["bundle_root"]) + "/worker/wsl_session_worker.py",
        )
        self.assertEqual(completed.returncode, 0, completed.stderr.decode("utf-8", "replace"))
        launched = self._wsl_run(
            executor._WSL_WORKER_BOOTSTRAP,
            str(bundle["runtime_parent"]),
            str(bundle["name"]),
            str(bundle["bundle_root"]),
            str(bundle["parent_device"]),
            str(bundle["parent_inode"]),
            str(bundle["root_device"]),
            str(bundle["root_inode"]),
            "worker/wsl_session_worker.py",
            str(bundle["worker_sha256"]),
            "worker/sealed_dependencies.py",
            str(bundle["dependency_sha256"]),
        )
        self.assertEqual(launched.returncode, 2)
        self.assertEqual(launched.stdout, b"")
        self.assertEqual(launched.stderr, b"")

    def test_full_fake_cycle_retains_bundle_until_close_and_leaves_no_residue(self) -> None:
        _p69, _p57, p51, _p56 = load_exact_p72_stack(REPO_ROOT)
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
        self.assertEqual(manager.records[0].revalidate_calls, 1)
        self.assertEqual(manager.records[0].cleanup_calls, 1)
        self.assertFalse(manager.residue())
        self.assertEqual(len(process_factory.processes), 1)
        self.assertEqual(len(process_factory.processes[0].requests), 69)

    def test_startup_failure_removes_staged_bundle(self) -> None:
        _p69, _p57, p51, _p56 = load_exact_p72_stack(REPO_ROOT)
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
        self.assertEqual(manager.records[0].revalidate_calls, 1)
        self.assertEqual(manager.records[0].cleanup_calls, 1)
        self.assertFalse(manager.residue())

    def test_close_timeout_kills_worker_and_removes_bundle(self) -> None:
        _p69, _p57, p51, _p56 = load_exact_p72_stack(REPO_ROOT)
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

    def test_prelaunch_root_substitution_causes_indeterminate_cleanup(self) -> None:
        _p69, _p57, p51, _p56 = load_exact_p72_stack(REPO_ROOT)
        fake = FakeP56(self._fake_artifact("prelaunch-root"))
        manager = FakeBundleManager(executor, self.work / "bundles")
        process_factory = FakeWorkerProcessFactory(executor, fake, self.work)
        stale_paths: list[Path] = []

        def substitute(path: Path, _bundle: object, _record: object) -> None:
            stale = path.with_name(path.name + "-stale")
            path.rename(stale)
            path.mkdir()
            stale_paths.append(stale)

        manager.before_revalidate = substitute
        with self._patch_native(manager, process_factory):
            with self.assertRaisesRegex(executor.ExecutorFailure, "P57-INDETERMINATE-CLEANUP"):
                executor._NativeWslSession(REPO_ROOT, "/home/runtime", p51)
        self.assertEqual(manager.records[0].revalidate_calls, 1)
        self.assertEqual(manager.records[0].cleanup_calls, 1)
        self.assertTrue(Path(manager.records[0].root).exists())
        self.assertTrue(stale_paths[0].exists())

    def test_prelaunch_parent_substitution_causes_indeterminate_cleanup(self) -> None:
        _p69, _p57, p51, _p56 = load_exact_p72_stack(REPO_ROOT)
        fake = FakeP56(self._fake_artifact("prelaunch-parent"))
        manager = FakeBundleManager(executor, self.work / "bundles")
        process_factory = FakeWorkerProcessFactory(executor, fake, self.work)
        stale_roots: list[Path] = []

        def substitute(path: Path, _bundle: object, _record: object) -> None:
            parent = path.parent
            stale_parent = parent.with_name(parent.name + "-stale")
            parent.rename(stale_parent)
            parent.mkdir()
            replacement = parent / path.name
            replacement.mkdir()
            stale_roots.append(stale_parent / path.name)

        manager.before_revalidate = substitute
        with self._patch_native(manager, process_factory):
            with self.assertRaisesRegex(executor.ExecutorFailure, "P57-INDETERMINATE-CLEANUP"):
                executor._NativeWslSession(REPO_ROOT, "/home/runtime", p51)
        self.assertEqual(manager.records[0].revalidate_calls, 1)
        self.assertEqual(manager.records[0].cleanup_calls, 1)
        self.assertTrue(Path(manager.records[0].root).exists())
        self.assertTrue(stale_roots[0].exists())

    def test_cleanup_failure_takes_precedence_over_protocol_failure(self) -> None:
        _p69, _p57, p51, _p56 = load_exact_p72_stack(REPO_ROOT)
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
        _p69, _p57, p51, _p56 = load_exact_p72_stack(REPO_ROOT)
        fake = FakeP56(self._fake_artifact("concurrent"))
        manager = FakeBundleManager(executor, self.work / "bundles")
        process_factory = FakeWorkerProcessFactory(executor, fake, self.work)
        with self._patch_native(manager, process_factory):
            first = executor._NativeWslSession(REPO_ROOT, "/home/runtime", p51)
            second = executor._NativeWslSession(REPO_ROOT, "/home/runtime", p51)
            first.close()
            second.close()
        self.assertEqual([record.revalidate_calls for record in manager.records], [1, 1])
        self.assertEqual([record.cleanup_calls for record in manager.records], [1, 1])
        self.assertFalse(manager.residue())


if __name__ == "__main__":
    unittest.main()
