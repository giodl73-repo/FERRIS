"""Qualification for the public Pulse 41 transactional copier."""

from __future__ import annotations

import importlib.util
import json
import os
import shutil
import stat
import subprocess
import sys
import unittest
from pathlib import Path
from unittest import mock


sys.dont_write_bytecode = True
HERE = Path(__file__).resolve()
RELEASE_ROOT = HERE.parents[1]
REPOSITORY_ROOT = HERE.parents[5]
PULSE_39_ROOT = (
    REPOSITORY_ROOT
    / "docs"
    / "simulations"
    / "profile-diff-held-out"
    / "pulse-39-checkout-verifier-release"
)
ADAPTER_PATH = RELEASE_ROOT / "transactional_copy.py"
SPEC = importlib.util.spec_from_file_location("pulse_41_transactional_copy", ADAPTER_PATH)
assert SPEC is not None and SPEC.loader is not None
adapter = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = adapter
SPEC.loader.exec_module(adapter)


def synced(_: Path) -> adapter.SyncPosture:
    return adapter.SyncPosture("synced", "test-directory-sync-v1", None)


class TransactionalCopyTests(unittest.TestCase):
    def setUp(self) -> None:
        self.work = (
            REPOSITORY_ROOT
            / "target"
            / f"pulse-41-python-{os.getpid()}-{self._testMethodName}"
        )
        if self.work.exists():
            shutil.rmtree(self.work)
        self.work.mkdir(parents=True)

    def tearDown(self) -> None:
        if self.work.exists():
            shutil.rmtree(self.work)

    def source(self, name: str = "source") -> Path:
        root = self.work / name
        root.mkdir(parents=True)
        for binding in adapter.CANONICAL_FILES:
            original = PULSE_39_ROOT.joinpath(*binding.path.split("/"))
            destination = root.joinpath(*binding.path.split("/"))
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(original, destination)
        adapter.verify_bound_tree(root, "SOURCE")
        return root

    def final(self, name: str = "final") -> Path:
        return self.work / name

    @staticmethod
    def stage(final_root: Path) -> Path:
        return final_root.parent / f".{final_root.name}.pulse-41-stage"

    def assert_no_residue(self, final_root: Path) -> None:
        self.assertFalse(final_root.exists(), final_root)
        self.assertFalse(self.stage(final_root).exists(), self.stage(final_root))

    def test_source_passes_exact_committed_pulse_39_bindings(self) -> None:
        source = self.source()
        final_root = self.final()
        result = adapter.copy_release(source, final_root, synchronizer=synced)
        self.assertEqual(result["status"], "pass")
        self.assertEqual(result["failure_code"], None)
        self.assertEqual(result["counts"], {"source": "8/8", "stage": "8/8", "final": "8/8"})
        self.assertEqual(result["rename_attempts"], 1)
        self.assertEqual(result["retries"], 0)
        self.assertFalse(result["rollback_attempted"])
        self.assertFalse(result["indeterminate_publication"])
        self.assertEqual(result["sync"]["staging"]["status"], "synced")
        self.assertEqual(result["sync"]["staging"]["directories"], 2)
        self.assertEqual(result["sync"]["staging"]["attempts"], 2)
        self.assertEqual(result["sync"]["staging"]["synced"], 2)
        self.assertEqual(result["sync"]["staging"]["unsupported"], 0)
        self.assertEqual(result["sync"]["final_parent"]["status"], "synced")
        self.assertFalse(result["sync"]["rollback_parent"]["attempted"])
        self.assertTrue(final_root.is_dir())
        self.assertFalse(self.stage(final_root).exists())
        adapter.verify_bound_tree(final_root, "FINAL")

    def test_rejects_missing_extra_and_symlink_source_entries(self) -> None:
        missing = self.source("missing")
        missing.joinpath(*adapter.EXPECTED_PATHS[0].split("/")).unlink()
        result = adapter.copy_release(missing, self.final("missing-final"), synchronizer=synced)
        self.assertEqual(result["failure_code"], "P41-SOURCE-PATH-SET")

        extra = self.source("extra")
        (extra / "unexpected.txt").write_bytes(b"extra\n")
        result = adapter.copy_release(extra, self.final("extra-final"), synchronizer=synced)
        self.assertEqual(result["failure_code"], "P41-SOURCE-PATH-SET")

        symlinked = self.source("symlink")
        target = symlinked / "README-target"
        target.write_bytes(b"public\n")
        original_bytes = (symlinked / "README.md").read_bytes()
        (symlinked / "README.md").unlink()
        try:
            os.symlink(target, symlinked / "README.md")
        except OSError as error:
            (symlinked / "README.md").write_bytes(original_bytes)
            original_lstat = adapter.os.lstat

            def symlink_lstat(path: str | os.PathLike[str]) -> os.stat_result:
                if Path(path) == symlinked / "README.md":
                    return os.stat_result((stat.S_IFLNK, 0, 0, 0, 0, 0, 0, 0, 0, 0))
                return original_lstat(path)

            with mock.patch.object(adapter.os, "lstat", side_effect=symlink_lstat):
                result = adapter.copy_release(
                    symlinked, self.final("symlink-final"), synchronizer=synced
                )
        else:
            result = adapter.copy_release(
                symlinked, self.final("symlink-final"), synchronizer=synced
            )
        self.assertEqual(result["failure_code"], "P41-SOURCE-TREE-UNSAFE")

    def test_rejects_traversal_overlap_existing_final_and_unsafe_parent(self) -> None:
        source = self.source()
        relative = adapter.copy_release("relative", self.final("relative"), synchronizer=synced)
        self.assertEqual(relative["failure_code"], "P41-ARGUMENT")

        traversal = self.work / "path" / ".." / "traversal-final"
        result = adapter.copy_release(source, traversal, synchronizer=synced)
        self.assertEqual(result["failure_code"], "P41-ARGUMENT")

        result = adapter.copy_release(source, source / "overlap", synchronizer=synced)
        self.assertEqual(result["failure_code"], "P41-ROOTS-OVERLAP")

        existing = self.final("existing")
        existing.mkdir()
        result = adapter.copy_release(source, existing, synchronizer=synced)
        self.assertEqual(result["failure_code"], "P41-FINAL-EXISTS")

        unsafe_parent = self.work / "unsafe-parent"
        unsafe_parent.write_bytes(b"not a directory")
        result = adapter.copy_release(
            source, unsafe_parent / "final", synchronizer=synced
        )
        self.assertEqual(result["failure_code"], "P41-FINAL-PARENT-UNSAFE")

    def test_rejects_duplicated_or_omitted_release_root_layout(self) -> None:
        source = self.source()
        duplicated = self.work / "duplicated"
        shutil.copytree(source, duplicated / "pulse-39-checkout-verifier-release")
        result = adapter.copy_release(
            duplicated, self.final("duplicated-final"), synchronizer=synced
        )
        self.assertEqual(result["failure_code"], "P41-SOURCE-PATH-SET")

        omitted = self.work / "omitted"
        shutil.copytree(source, omitted / "actual-release-root")
        result = adapter.copy_release(omitted, self.final("omitted-final"), synchronizer=synced)
        self.assertEqual(result["failure_code"], "P41-SOURCE-PATH-SET")

    def test_reconstructs_final_paths_instead_of_using_stale_staging_paths(self) -> None:
        source = self.source()
        final_root = self.final()
        stale_stage = self.stage(final_root)

        def after_rename(published_root: Path) -> None:
            self.assertEqual(published_root, final_root.resolve())
            self.assertFalse(stale_stage.exists())

        result = adapter.copy_release(
            source, final_root, synchronizer=synced, post_rename=after_rename
        )
        self.assertEqual(result["status"], "pass")
        self.assertEqual(result["counts"]["final"], "8/8")
        self.assertFalse(stale_stage.exists())
        adapter.verify_bound_tree(final_root, "FINAL")

    def test_is_independent_of_the_current_working_directory(self) -> None:
        source = self.source()
        final_root = self.final()
        nested = self.work / "nested" / "cwd"
        nested.mkdir(parents=True)
        result = subprocess.run(
            [
                sys.executable,
                str(ADAPTER_PATH),
                "--source-root",
                str(source),
                "--final-root",
                str(final_root),
            ],
            cwd=nested,
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertEqual(report["status"], "pass")
        self.assertEqual(report["counts"], {"final": "8/8", "source": "8/8", "stage": "8/8"})
        self.assertNotIn(str(source), result.stdout)
        self.assertNotIn(str(final_root), result.stdout)

    def test_partial_stage_copy_cleans_up_before_publication(self) -> None:
        source = self.source()
        final_root = self.final()

        def partial_copy(original: Path, destination: Path) -> None:
            destination.write_bytes(original.read_bytes()[:1])

        result = adapter.copy_release(
            source, final_root, synchronizer=synced, copier=partial_copy
        )
        self.assertEqual(result["failure_code"], "P41-STAGE-VERIFY-FAILURE")
        self.assertEqual(result["rename_attempts"], 0)
        self.assert_no_residue(final_root)

    def test_destination_file_fsync_failure_cleans_up_before_publication(self) -> None:
        source = self.source()
        final_root = self.final()

        with mock.patch.object(adapter.os, "fsync", side_effect=OSError("injected")):
            result = adapter.copy_release(source, final_root, synchronizer=synced)

        self.assertEqual(result["failure_code"], "P41-STAGE-COPY-FAILURE")
        self.assertEqual(result["rename_attempts"], 0)
        self.assertEqual(result["sync"]["staging"]["status"], "not-attempted")
        self.assert_no_residue(final_root)

    def test_staging_directories_sync_bottom_up_before_rename(self) -> None:
        source = self.source()
        final_root = self.final()
        observed: list[Path] = []

        def record_sync(path: Path) -> adapter.SyncPosture:
            observed.append(path)
            return synced(path)

        result = adapter.copy_release(source, final_root, synchronizer=record_sync)

        staging_root = self.stage(final_root)
        self.assertEqual(observed[:2], [staging_root / "tests", staging_root])
        self.assertEqual(observed[2:], [final_root.parent])
        self.assertEqual(result["sync"]["staging"]["directories"], 2)
        self.assertEqual(result["sync"]["staging"]["attempts"], 2)
        self.assertEqual(result["sync"]["staging"]["synced"], 2)
        self.assertEqual(result["sync"]["staging"]["unsupported"], 0)

    def test_rename_failure_has_one_attempt_no_retry_and_no_residue(self) -> None:
        source = self.source()
        final_root = self.final()

        def failed_rename(_: Path, __: Path) -> None:
            raise OSError("injected")

        result = adapter.copy_release(
            source, final_root, synchronizer=synced, renamer=failed_rename
        )
        self.assertEqual(result["failure_code"], "P41-RENAME-FAILURE")
        self.assertEqual(result["rename_attempts"], 1)
        self.assertEqual(result["retries"], 0)
        self.assert_no_residue(final_root)

    def test_stage_sync_failure_cleans_up_before_publication(self) -> None:
        source = self.source()
        final_root = self.final()

        def failed_sync(_: Path) -> adapter.SyncPosture:
            raise OSError("injected")

        result = adapter.copy_release(source, final_root, synchronizer=failed_sync)
        self.assertEqual(result["failure_code"], "P41-STAGE-SYNC-FAILURE")
        self.assertEqual(result["rename_attempts"], 0)
        self.assertEqual(result["sync"]["staging"]["status"], "failed")
        self.assertEqual(result["sync"]["staging"]["error_category"], "sync-operation-failed")
        self.assertEqual(result["sync"]["staging"]["directories"], 2)
        self.assertEqual(result["sync"]["staging"]["attempts"], 1)
        self.assertEqual(result["sync"]["staging"]["operational_failures"], 1)
        self.assert_no_residue(final_root)

    def test_final_verification_tamper_rolls_back_to_absent(self) -> None:
        source = self.source()
        final_root = self.final()

        def tamper(published_root: Path) -> None:
            (published_root / "README.md").write_bytes(b"tampered\n")

        result = adapter.copy_release(
            source, final_root, synchronizer=synced, post_rename=tamper
        )
        self.assertEqual(result["failure_code"], "P41-FINAL-VERIFY-FAILURE")
        self.assertTrue(result["rollback_attempted"])
        self.assertTrue(result["rollback_path_absent"])
        self.assertTrue(result["rollback_verified_absent"])
        self.assertFalse(result["indeterminate_publication"])
        self.assertEqual(result["sync"]["rollback_parent"]["status"], "synced")
        self.assert_no_residue(final_root)

    def test_final_sync_failure_rolls_back_with_unsupported_parent_sync(self) -> None:
        source = self.source()
        final_root = self.final()
        calls = 0

        def sync_then_fail(_: Path) -> adapter.SyncPosture:
            nonlocal calls
            calls += 1
            if calls <= 2:
                return synced(_)
            if calls == 3:
                raise OSError("injected")
            return adapter.SyncPosture(
                "unsupported",
                "test-directory-sync-v1",
                "unsupported-by-platform-or-filesystem",
            )

        result = adapter.copy_release(source, final_root, synchronizer=sync_then_fail)
        self.assertEqual(result["failure_code"], "P41-FINAL-SYNC-FAILURE")
        self.assertEqual(result["counts"]["final"], "8/8")
        self.assertTrue(result["rollback_attempted"])
        self.assertTrue(result["rollback_path_absent"])
        self.assertTrue(result["rollback_verified_absent"])
        self.assertEqual(result["sync"]["rollback_parent"]["status"], "unsupported")
        self.assert_no_residue(final_root)

    def test_unproven_rollback_is_indeterminate_never_success(self) -> None:
        source = self.source()
        final_root = self.final()

        def tamper(published_root: Path) -> None:
            (published_root / "README.md").write_bytes(b"tampered\n")

        def failed_remover(_: Path) -> None:
            raise OSError("injected")

        result = adapter.copy_release(
            source,
            final_root,
            synchronizer=synced,
            post_rename=tamper,
            remover=failed_remover,
        )
        self.assertEqual(result["status"], "fail")
        self.assertEqual(result["failure_code"], "P41-INDETERMINATE-PUBLICATION")
        self.assertTrue(result["rollback_attempted"])
        self.assertFalse(result["rollback_path_absent"])
        self.assertFalse(result["rollback_verified_absent"])
        self.assertTrue(result["indeterminate_publication"])
        self.assertFalse(result["sync"]["rollback_parent"]["attempted"])
        self.assertTrue(final_root.exists())

    def test_rollback_parent_sync_failure_is_indeterminate(self) -> None:
        source = self.source()
        final_root = self.final()
        calls = 0

        def tamper(published_root: Path) -> None:
            (published_root / "README.md").write_bytes(b"tampered\n")

        def sync_then_rollback_fails(_: Path) -> adapter.SyncPosture:
            nonlocal calls
            calls += 1
            if calls <= 2:
                return synced(_)
            raise OSError("injected")

        result = adapter.copy_release(
            source,
            final_root,
            synchronizer=sync_then_rollback_fails,
            post_rename=tamper,
        )
        self.assertEqual(result["status"], "fail")
        self.assertEqual(result["failure_code"], "P41-INDETERMINATE-PUBLICATION")
        self.assertTrue(result["rollback_attempted"])
        self.assertTrue(result["rollback_path_absent"])
        self.assertFalse(result["rollback_verified_absent"])
        self.assertTrue(result["indeterminate_publication"])
        self.assertEqual(
            result["sync"]["rollback_parent"]["error_category"], "sync-operation-failed"
        )
        self.assert_no_residue(final_root)

    def test_output_is_deterministic_and_has_no_private_paths(self) -> None:
        reports = []
        for index in range(2):
            source = self.source(f"deterministic-source-{index}")
            final_root = self.final(f"deterministic-final-{index}")
            result = adapter.copy_release(source, final_root)
            self.assertNotIn(str(source), public := adapter.public_json(result))
            self.assertNotIn(str(final_root), public)
            reports.append(public)
            adapter.remove_tree(final_root)
            adapter.remove_tree(source)
            self.assertFalse(self.stage(final_root).exists())
        self.assertEqual(reports[0], reports[1])

    def test_twenty_isolated_exact_source_success_cycles_leave_no_residue(self) -> None:
        reports = []
        for index in range(20):
            source = self.source(f"cycle-source-{index}")
            final_root = self.final(f"cycle-final-{index}")
            result = adapter.copy_release(source, final_root, synchronizer=synced)
            self.assertEqual(result["status"], "pass", index)
            self.assertEqual(result["counts"], {"source": "8/8", "stage": "8/8", "final": "8/8"})
            self.assertEqual(result["rename_attempts"], 1)
            self.assertEqual(result["retries"], 0)
            reports.append(adapter.public_json(result))
            adapter.remove_tree(final_root)
            adapter.remove_tree(source)
            self.assertFalse(self.stage(final_root).exists())
        self.assertEqual(len(set(reports)), 1)
        self.assertEqual(list(self.work.iterdir()), [])


if __name__ == "__main__":
    unittest.main()
