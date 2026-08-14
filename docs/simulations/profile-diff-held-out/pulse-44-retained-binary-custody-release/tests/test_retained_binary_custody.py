from __future__ import annotations

import json
import os
import shutil
import stat
import sys
import unittest
import uuid
from pathlib import Path
from unittest import mock


RELEASE = Path(__file__).resolve().parents[1]
ROOT = Path(__file__).resolve().parents[5]
sys.path.insert(0, str(RELEASE))
import retained_binary_custody as custody  # noqa: E402


def synced(_: Path) -> custody.SyncPosture:
    return custody.SyncPosture("synced", custody.SYNC_MECHANISM, None)


def write_synthetic_output(
    output: Path,
    *,
    artifact_name: str | None = None,
    artifact_platform: str = "windows-x86_64",
    artifact_cutoff: str = custody.P33_CUTOFF,
    retained: bool = True,
    artifact_size: int | None = None,
    artifact_sha256: str | None = None,
    include_executable: bool = True,
) -> None:
    output.mkdir()
    filename = artifact_name or custody._logical_filename("windows-x86_64", custody.P33_CUTOFF)
    executable = output / filename
    binary = b"bounded synthetic retained executable\n"
    if include_executable:
        executable.write_bytes(binary)
    digest = custody.sha256_bytes(binary)
    payload = {
        "artifact": {
            "discovery": "cargo-compiler-artifact-json",
            "logical_filename": filename,
            "retained_in_public_bundle": retained,
            "sha256": artifact_sha256 or digest,
            "size": len(binary) if artifact_size is None else artifact_size,
        },
        "build": {
            "binary": "ferris",
            "command": [
                "cargo",
                "build",
                "--locked",
                "--release",
                "--package",
                "ferris-cli",
                "--bin",
                "ferris",
            ],
            "package": "ferris-cli",
            "profile": "release",
        },
        "checkout": {
            "core_autocrlf": False,
            "exact_commit": True,
            "tracked_files_clean": True,
        },
        "cutoff": artifact_cutoff,
        "platform": artifact_platform,
        "safety": {"diagnostic_execution": False, "product_files_modified": False},
        "schema": "ferris.public-build-freeze-receipt/v1",
    }
    receipt = {
        "payload": payload,
        "payload_sha256": custody.sha256_bytes(custody.canonical_bytes(payload)),
        "schema": "ferris.public-build-freeze-envelope/v1",
    }
    (output / f"{filename}.receipt.json").write_bytes(
        json.dumps(receipt, sort_keys=True, indent=2).encode("utf-8") + b"\n"
    )


class RetainedBinaryCustodyTests(unittest.TestCase):
    def setUp(self) -> None:
        self.runtime = ROOT / "target" / f"pulse-44-python-{uuid.uuid4().hex}"
        self.runtime.mkdir(parents=True)
        self.calls: list[bool] = []

    def tearDown(self) -> None:
        if self.runtime.exists() or os.path.lexists(self.runtime):
            shutil.rmtree(self.runtime, ignore_errors=True)

    def roots(self) -> tuple[Path, Path]:
        return self.runtime / "work", self.runtime / "final"

    def builder(self, output_writer=write_synthetic_output):
        def build(
            repo: Path, cutoff: str, platform: str, output: Path, *, retain_executable: bool
        ) -> dict[str, object]:
            self.assertEqual(repo, ROOT)
            self.assertEqual(cutoff, custody.P33_CUTOFF)
            self.assertEqual(platform, "windows-x86_64")
            self.calls.append(retain_executable)
            output_writer(output)
            return {}

        return build

    def invoke(self, **kwargs: object) -> dict[str, object]:
        work, final = self.roots()
        options: dict[str, object] = {
            "builder": self.builder(),
            "synchronizer": synced,
        }
        options.update(kwargs)
        return custody.retain_binary_custody(
            ROOT,
            custody.P33_CUTOFF,
            "windows-x86_64",
            work,
            final,
            **options,  # type: ignore[arg-type]
        )

    def assert_failure(self, report: dict[str, object], code: str, posture: str = "absent") -> None:
        self.assertEqual(report["outcome"], "failed")
        self.assertEqual(report["failure_code"], code)
        self.assertEqual(report["custody"]["state"], posture)
        self.assertFalse(report["custody"]["final_files_present"])
        self.assertEqual(
            report["ordered_execution_event"],
            {
                "classification": "ordered-execution",
                "event_kind": "terminal-stop",
                "gate_id": "retained-binary-custody",
                "outcome": "failed",
                "schema": "ferris.pulse-43-ordered-result-event/v1",
            },
        )

    def test_success_copies_exact_pair_with_one_build_and_one_rename(self) -> None:
        emitted: list[dict[str, str]] = []
        report = self.invoke(event_emitter=emitted.append)
        _, final = self.roots()
        self.assertEqual(self.calls, [True])
        self.assertEqual(report["outcome"], "published")
        self.assertEqual(report["custody"]["files"], "2/2")
        self.assertEqual(report["custody"]["work_verified"], "2/2")
        self.assertEqual(report["custody"]["stage_verified"], "2/2")
        self.assertEqual(report["custody"]["final_verified"], "2/2")
        self.assertEqual(report["custody"]["rename_attempts"], 1)
        self.assertEqual(report["custody"]["retries"], 0)
        self.assertEqual(len(list(final.iterdir())), 2)
        self.assertFalse(self.roots()[0].exists())
        self.assertEqual(emitted, [report["ordered_execution_event"]])

    def test_success_event_is_not_emitted_before_final_verification(self) -> None:
        seen: list[bool] = []
        _, final = self.roots()
        report = self.invoke(event_emitter=lambda _: seen.append(final.is_dir()))
        self.assertEqual(report["outcome"], "published")
        self.assertEqual(seen, [True])

    def test_missing_retain_flag_is_rejected(self) -> None:
        def writer(output: Path) -> None:
            write_synthetic_output(output, retained=False)

        work, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(writer), synchronizer=synced,
        )
        self.assert_failure(report, "P44-WORK-VERIFY-FAILURE")
        self.assertEqual(self.calls, [True])

    def test_missing_executable_is_rejected(self) -> None:
        def writer(output: Path) -> None:
            write_synthetic_output(output, include_executable=False)

        work, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(writer), synchronizer=synced,
        )
        self.assert_failure(report, "P44-WORK-VERIFY-FAILURE")

    def test_filename_mismatch_is_rejected(self) -> None:
        def writer(output: Path) -> None:
            write_synthetic_output(output, artifact_name="unexpected.exe")

        work, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(writer), synchronizer=synced,
        )
        self.assert_failure(report, "P44-WORK-VERIFY-FAILURE")

    def test_platform_mismatch_is_rejected(self) -> None:
        def writer(output: Path) -> None:
            write_synthetic_output(output, artifact_platform="ubuntu-24.04-x86_64")

        work, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(writer), synchronizer=synced,
        )
        self.assert_failure(report, "P44-WORK-VERIFY-FAILURE")

    def test_cutoff_mismatch_is_rejected(self) -> None:
        def writer(output: Path) -> None:
            write_synthetic_output(output, artifact_cutoff="0" * 40)

        work, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(writer), synchronizer=synced,
        )
        self.assert_failure(report, "P44-WORK-VERIFY-FAILURE")

    def test_hash_mismatch_is_rejected(self) -> None:
        def writer(output: Path) -> None:
            write_synthetic_output(output, artifact_sha256="sha256:" + "0" * 64)

        work, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(writer), synchronizer=synced,
        )
        self.assert_failure(report, "P44-WORK-VERIFY-FAILURE")

    def test_size_mismatch_is_rejected(self) -> None:
        def writer(output: Path) -> None:
            write_synthetic_output(output, artifact_size=1)

        work, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(writer), synchronizer=synced,
        )
        self.assert_failure(report, "P44-WORK-VERIFY-FAILURE")

    def test_stale_work_root_is_rejected_without_build(self) -> None:
        work, final = self.roots()
        work.mkdir()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(), synchronizer=synced,
        )
        self.assert_failure(report, "P44-ROOT-EXISTS")
        self.assertEqual(self.calls, [])

    def test_stale_final_root_is_rejected_without_build(self) -> None:
        work, final = self.roots()
        final.mkdir()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(), synchronizer=synced,
        )
        self.assert_failure(report, "P44-ROOT-EXISTS")
        self.assertEqual(self.calls, [])

    def test_stale_stage_is_rejected_without_build(self) -> None:
        work, final = self.roots()
        (self.runtime / ".final.pulse-44-stage").mkdir()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(), synchronizer=synced,
        )
        self.assert_failure(report, "P44-STAGING-EXISTS")
        self.assertEqual(self.calls, [])

    def test_non_regular_executable_is_rejected(self) -> None:
        def writer(output: Path) -> None:
            write_synthetic_output(output)
            executable = output / custody._logical_filename("windows-x86_64", custody.P33_CUTOFF)
            executable.unlink()
            executable.mkdir()

        work, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", work, final,
            builder=self.builder(writer), synchronizer=synced,
        )
        self.assert_failure(report, "P44-WORK-VERIFY-FAILURE")

    def test_symlink_executable_is_rejected(self) -> None:
        synthetic_link = os.stat_result((stat.S_IFLNK, 0, 0, 0, 0, 0, 0, 0, 0, 0))
        with mock.patch.object(custody.os, "lstat", return_value=synthetic_link):
            with self.assertRaises(custody.CustodyFailure) as raised:
                custody._safe_regular_digest(
                    self.runtime / "synthetic-link", "P44-WORK-VERIFY-FAILURE"
                )
        self.assertEqual(str(raised.exception), "P44-WORK-VERIFY-FAILURE")

    def test_copy_failure_cleans_all_runtime_roots(self) -> None:
        report = self.invoke(copier=lambda _source, _destination: (_ for _ in ()).throw(OSError()))
        self.assert_failure(report, "P44-STAGE-COPY-FAILURE")
        self.assertFalse(self.roots()[0].exists())
        self.assertFalse(self.roots()[1].exists())

    def test_file_sync_failure_cleans_all_runtime_roots(self) -> None:
        report = self.invoke(file_synchronizer=lambda _descriptor: (_ for _ in ()).throw(OSError()))
        self.assert_failure(report, "P44-STAGE-COPY-FAILURE")

    def test_stage_verification_failure_cleans_all_runtime_roots(self) -> None:
        report = self.invoke(copier=lambda _source, destination: destination.write_bytes(b"tampered"))
        self.assert_failure(report, "P44-STAGE-VERIFY-FAILURE")

    def test_stage_sync_failure_cleans_all_runtime_roots(self) -> None:
        report = self.invoke(synchronizer=lambda _path: (_ for _ in ()).throw(OSError()))
        self.assert_failure(report, "P44-STAGE-SYNC-FAILURE")

    def test_rename_failure_has_absent_posture(self) -> None:
        report = self.invoke(renamer=lambda _source, _destination: (_ for _ in ()).throw(OSError()))
        self.assert_failure(report, "P44-RENAME-FAILURE")
        self.assertEqual(report["custody"]["rename_attempts"], 1)

    def test_partial_rename_failure_rolls_back(self) -> None:
        def partial(source: Path, destination: Path) -> None:
            os.replace(source, destination)
            raise OSError()

        report = self.invoke(renamer=partial)
        self.assert_failure(report, "P44-RENAME-FAILURE", "rolled-back")
        self.assertFalse(self.roots()[1].exists())

    def test_final_verify_failure_rolls_back(self) -> None:
        def corrupt(final: Path) -> None:
            (final / custody._logical_filename("windows-x86_64", custody.P33_CUTOFF)).write_bytes(
                b"tampered"
            )

        report = self.invoke(post_rename=corrupt)
        self.assert_failure(report, "P44-FINAL-VERIFY-FAILURE", "rolled-back")

    def test_final_parent_sync_failure_rolls_back(self) -> None:
        calls = 0

        def fail_second(_: Path) -> custody.SyncPosture:
            nonlocal calls
            calls += 1
            if calls == 2:
                raise OSError()
            return synced(_)

        report = self.invoke(synchronizer=fail_second)
        self.assert_failure(report, "P44-FINAL-SYNC-FAILURE", "rolled-back")

    def test_rollback_removal_failure_is_indeterminate(self) -> None:
        _, final = self.roots()

        def corrupt(path: Path) -> None:
            (path / custody._logical_filename("windows-x86_64", custody.P33_CUTOFF)).write_bytes(b"x")

        def fail_final_removal(path: Path) -> None:
            if path == final:
                raise OSError()
            custody.remove_tree(path)

        report = self.invoke(post_rename=corrupt, remover=fail_final_removal)
        self.assert_failure(report, "P44-INDETERMINATE-PUBLICATION", "indeterminate")

    def test_rollback_sync_failure_is_indeterminate(self) -> None:
        calls = 0

        def corrupt(path: Path) -> None:
            (path / custody._logical_filename("windows-x86_64", custody.P33_CUTOFF)).write_bytes(b"x")

        def fail_rollback(_: Path) -> custody.SyncPosture:
            nonlocal calls
            calls += 1
            if calls == 2:
                raise OSError()
            return synced(_)

        report = self.invoke(post_rename=corrupt, synchronizer=fail_rollback)
        self.assert_failure(report, "P44-INDETERMINATE-PUBLICATION", "indeterminate")

    def test_event_emission_failure_rolls_back(self) -> None:
        report = self.invoke(event_emitter=lambda _event: (_ for _ in ()).throw(OSError()))
        self.assert_failure(report, "P44-EVENT-EMIT-FAILURE", "rolled-back")

    def test_invalid_cutoff_does_not_call_builder(self) -> None:
        work, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, "0" * 40, "windows-x86_64", work, final,
            builder=self.builder(), synchronizer=synced,
        )
        self.assert_failure(report, "P44-BUILD-IDENTITY")
        self.assertEqual(self.calls, [])

    def test_relative_root_does_not_call_builder(self) -> None:
        _, final = self.roots()
        report = custody.retain_binary_custody(
            ROOT, custody.P33_CUTOFF, "windows-x86_64", "relative", final,
            builder=self.builder(), synchronizer=synced,
        )
        self.assert_failure(report, "P44-ROOT-INVALID")
        self.assertEqual(self.calls, [])

    def test_failure_summary_is_path_free_and_deterministic(self) -> None:
        first = self.invoke(copier=lambda _source, _destination: (_ for _ in ()).throw(OSError()))
        serialized = custody.canonical_bytes(first).decode("ascii")
        self.assertNotIn(str(self.runtime), serialized)
        self.assertNotIn("executable", serialized)
        self.assertEqual(first["custody"]["retries"], 0)

    def test_pulse_33_identity_is_pinned(self) -> None:
        source = custody._verify_p33_identity(custody._p33_release_root())
        self.assertEqual(source.name, "build_freeze.py")


if __name__ == "__main__":
    unittest.main()
