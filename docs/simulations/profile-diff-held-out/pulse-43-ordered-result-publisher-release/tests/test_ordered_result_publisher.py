"""Qualification for the public Pulse 43 ordered-result publisher."""

from __future__ import annotations

import importlib.util
import json
import os
import shutil
import sys
import unittest
from copy import deepcopy
from pathlib import Path


sys.dont_write_bytecode = True
HERE = Path(__file__).resolve()
RELEASE_ROOT = HERE.parents[1]
REPOSITORY_ROOT = HERE.parents[5]
PUBLISHER_PATH = RELEASE_ROOT / "ordered_result_publisher.py"
SPEC = importlib.util.spec_from_file_location("pulse_43_publisher", PUBLISHER_PATH)
assert SPEC is not None and SPEC.loader is not None
publisher = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = publisher
SPEC.loader.exec_module(publisher)


def synced(_: Path) -> publisher.SyncPosture:
    return publisher.SyncPosture("synced", "test-directory-sync-v1", None)


class OrderedResultPublisherTests(unittest.TestCase):
    def setUp(self) -> None:
        self.work = (
            REPOSITORY_ROOT
            / "target"
            / f"pulse-43-python-{os.getpid()}-{self._testMethodName}"
        )
        if self.work.exists():
            shutil.rmtree(self.work)
        self.work.mkdir(parents=True)
        self.catalog = json.loads(
            (RELEASE_ROOT / "fixtures" / "public-gate-catalog.json").read_text("utf-8")
        )
        self.events = json.loads(
            (RELEASE_ROOT / "fixtures" / "complete-events.json").read_text("utf-8")
        )

    def tearDown(self) -> None:
        if self.work.exists():
            shutil.rmtree(self.work)

    def final(self, name: str = "final") -> Path:
        return self.work / name

    @staticmethod
    def stage(final: Path) -> Path:
        return final.parent / f".{final.name}.pulse-43-stage"

    def assert_absent(self, final: Path) -> None:
        self.assertFalse(final.exists(), final)
        self.assertFalse(self.stage(final).exists(), self.stage(final))

    def publish(
        self, events: object | None = None, final: Path | None = None, **kwargs: object
    ) -> tuple[dict[str, object], Path]:
        output = final or self.final()
        return (
            publisher.publish_result(
                self.catalog, self.events if events is None else events, output, **kwargs
            ),
            output,
        )

    def test_complete_catalog_publishes_and_recomputes_both_hashes(self) -> None:
        report, final = self.publish(synchronizer=synced)
        self.assertEqual(report["publication"]["state"], "published")
        self.assertEqual(report["publication"]["files"], "2/2")
        self.assertEqual(report["publication"]["rename_attempts"], 1)
        self.assertEqual(report["publication"]["retries"], 0)
        self.assertTrue(report["publication"]["final_files_present"])
        self.assertEqual(report["ordered_execution"]["completed_gate_count"], 3)
        self.assertEqual(report["ordered_execution"]["attempted_gate_count"], 3)
        self.assertEqual(report["ordered_execution"]["terminal"]["outcome"], "completed")
        self.assertEqual(report["public_self_validation"]["completed_checks"], 47)
        self.assertEqual(report["public_self_validation"]["expected_checks"], 47)
        hashes = publisher.verify_publication_directory(final)
        self.assertEqual(report["publication"]["raw_hashes"], hashes)
        self.assertEqual(set(path.name for path in final.iterdir()), set(publisher.OUTPUT_FILES))

    def test_early_pulse33_stop_cannot_coexist_with_later_p31_or_p35_execution(self) -> None:
        events = [
            self.events[0],
            {
                "classification": "ordered-execution",
                "event_kind": "terminal-stop",
                "gate_id": "pulse-33-freeze",
                "outcome": "stopped",
                "schema": publisher.EVENT_SCHEMA,
            },
            self.events[3],
            self.events[4],
        ]
        report, final = self.publish(events)
        self.assertEqual(report["failure_code"], "P43-ORDERED-AFTER-TERMINAL")
        self.assertEqual(report["publication"]["state"], "absent")
        self.assert_absent(final)

    def test_stopped_terminal_publishes_only_its_derived_execution_counts(self) -> None:
        events = [
            self.events[0],
            {
                "classification": "ordered-execution",
                "event_kind": "terminal-stop",
                "gate_id": "pulse-33-freeze",
                "outcome": "stopped",
                "schema": publisher.EVENT_SCHEMA,
            },
        ]
        report, final = self.publish(events, synchronizer=synced)
        self.assertEqual(report["publication"]["state"], "published")
        self.assertEqual(report["ordered_execution"]["attempted_gate_count"], 1)
        self.assertEqual(report["ordered_execution"]["completed_gate_count"], 0)
        self.assertEqual(report["public_self_validation"]["completed_checks"], 39)
        result = json.loads((final / "public-result.json").read_text("utf-8"))
        self.assertEqual(
            result["payload"]["ordered_execution"],
            {
                "attempted_gate_count": 1,
                "catalog_gate_count": 3,
                "completed_gate_count": 0,
                "terminal": {
                    "event_kind": "terminal-stop",
                    "gate_id": "pulse-33-freeze",
                    "outcome": "stopped",
                },
            },
        )
        self.assertNotIn("ordered_gate_counts", result["payload"])

    def test_self_validation_cannot_satisfy_or_advance_execution(self) -> None:
        events = [
            self.events[0],
            self.events[2],
            {
                "classification": "ordered-execution",
                "event_kind": "terminal-stop",
                "gate_id": "pulse-33-freeze",
                "outcome": "failed",
                "schema": publisher.EVENT_SCHEMA,
            },
        ]
        report, _ = self.publish(events, synchronizer=synced)
        self.assertEqual(report["publication"]["state"], "published")
        self.assertEqual(report["public_self_validation"]["completed_checks"], 47)
        self.assertEqual(report["ordered_execution"]["attempted_gate_count"], 1)
        self.assertEqual(report["ordered_execution"]["completed_gate_count"], 0)

    def test_rejects_missing_prior_duplicate_and_missing_terminal_execution_gates(self) -> None:
        missing_prior = [deepcopy(self.events[3]), deepcopy(self.events[4])]
        report, _ = self.publish(missing_prior)
        self.assertEqual(report["failure_code"], "P43-MISSING-PRIOR-EXECUTION-GATE")

        duplicate = [
            deepcopy(self.events[1]),
            deepcopy(self.events[1]),
            deepcopy(self.events[4]),
        ]
        report, _ = self.publish(duplicate)
        self.assertEqual(report["failure_code"], "P43-DUPLICATE-EXECUTION-GATE")

        missing_terminal = [deepcopy(self.events[1]), deepcopy(self.events[3])]
        report, _ = self.publish(missing_terminal)
        self.assertEqual(report["failure_code"], "P43-MISSING-TERMINAL")

    def test_rejects_early_completed_terminal_and_unknown_execution_gate(self) -> None:
        early_completed = [
            {
                "classification": "ordered-execution",
                "event_kind": "terminal-stop",
                "gate_id": "pulse-33-freeze",
                "outcome": "completed",
                "schema": publisher.EVENT_SCHEMA,
            }
        ]
        report, _ = self.publish(early_completed)
        self.assertEqual(report["failure_code"], "P43-INCOMPLETE-CATALOG")
        unknown = deepcopy(self.events)
        unknown[1]["gate_id"] = "public-gate-four"
        report, _ = self.publish(unknown)
        self.assertEqual(report["failure_code"], "P43-UNKNOWN-EXECUTION-GATE")

    def test_rejects_unknown_fields_privacy_bearing_ids_and_unbounded_counts(self) -> None:
        unknown = deepcopy(self.events)
        unknown[0]["path"] = "C:\\not-public"
        report, _ = self.publish(unknown)
        self.assertEqual(report["failure_code"], "P43-EVENT-SCHEMA")

        private = deepcopy(self.events)
        private[0]["validation_id"] = "private-record"
        report, _ = self.publish(private)
        self.assertEqual(report["failure_code"], "P43-PRIVACY-BEARING-IDENTIFIER")

        unbounded = deepcopy(self.events)
        unbounded[0]["expected_checks"] = publisher.MAX_VALIDATION_COUNT + 1
        report, _ = self.publish(unbounded)
        self.assertEqual(report["failure_code"], "P43-VALIDATION-COUNT")

    def test_rejects_unknown_catalog_fields_duplicate_ids_and_private_catalog_ids(self) -> None:
        unknown = deepcopy(self.catalog)
        unknown["path"] = "C:\\not-public"
        self.assertEqual(
            publisher.publish_result(unknown, self.events, self.final("unknown"))["failure_code"],
            "P43-CATALOG-SCHEMA",
        )
        duplicate = deepcopy(self.catalog)
        duplicate["gate_ids"][1] = duplicate["gate_ids"][0]
        self.assertEqual(
            publisher.publish_result(duplicate, self.events, self.final("duplicate"))["failure_code"],
            "P43-DUPLICATE-CATALOG-GATE",
        )
        private = deepcopy(self.catalog)
        private["gate_ids"][0] = "private-gate"
        self.assertEqual(
            publisher.publish_result(private, self.events, self.final("private"))["failure_code"],
            "P43-PRIVACY-BEARING-IDENTIFIER",
        )

    def test_duplicate_json_members_are_rejected_before_publication(self) -> None:
        source = self.work / "duplicate.json"
        source.write_text(
            '{"schema":"ferris.pulse-43-ordered-gate-catalog/v1","schema":"x","gate_ids":["a"]}',
            encoding="utf-8",
        )
        with self.assertRaisesRegex(publisher.PublicFailure, "P43-DUPLICATE-JSON-MEMBER"):
            publisher.load_public_json(source)

    def test_output_is_deterministic_and_does_not_include_input_paths(self) -> None:
        reports = []
        for index in range(2):
            final = self.final(f"deterministic-{index}")
            report, _ = self.publish(final=final)
            rendered = publisher.public_json(report)
            self.assertNotIn(str(final), rendered)
            self.assertNotIn(str(RELEASE_ROOT), rendered)
            reports.append(rendered)
            publisher.remove_tree(final)
        self.assertEqual(reports[0], reports[1])

    def test_twenty_isolated_publication_cycles_leave_no_residue(self) -> None:
        reports = []
        for index in range(20):
            final = self.final(f"cycle-{index}")
            report, _ = self.publish(final=final, synchronizer=synced)
            self.assertEqual(report["publication"]["state"], "published")
            reports.append(publisher.public_json(report))
            publisher.remove_tree(final)
            self.assertFalse(self.stage(final).exists())
        self.assertEqual(len(set(reports)), 1)
        self.assertEqual(list(self.work.iterdir()), [])

    def test_injected_copy_failure_is_absent_and_never_success_shaped(self) -> None:
        def failed_writer(_: Path, __: bytes) -> None:
            raise OSError("injected")

        report, final = self.publish(writer=failed_writer)
        self.assertEqual(report["failure_code"], "P43-STAGE-COPY-FAILURE")
        self.assertEqual(report["publication"]["state"], "absent")
        self.assertNotIn("ordered_execution", report)
        self.assert_absent(final)

    def test_injected_stage_sync_failure_is_absent(self) -> None:
        def failed_sync(_: Path) -> publisher.SyncPosture:
            raise OSError("injected")

        report, final = self.publish(synchronizer=failed_sync)
        self.assertEqual(report["failure_code"], "P43-STAGE-SYNC-FAILURE")
        self.assertEqual(report["publication"]["state"], "absent")
        self.assertEqual(report["publication"]["sync"]["stage"]["status"], "failed")
        self.assert_absent(final)

    def test_injected_rename_failure_has_one_attempt_zero_retries_and_is_absent(self) -> None:
        def failed_rename(_: Path, __: Path) -> None:
            raise OSError("injected")

        report, final = self.publish(renamer=failed_rename, synchronizer=synced)
        self.assertEqual(report["failure_code"], "P43-RENAME-FAILURE")
        self.assertEqual(report["publication"]["rename_attempts"], 1)
        self.assertEqual(report["publication"]["retries"], 0)
        self.assertEqual(report["publication"]["state"], "absent")
        self.assert_absent(final)

    def test_missing_final_directory_after_rename_is_absent(self) -> None:
        def no_rename(_: Path, __: Path) -> None:
            return None

        report, final = self.publish(renamer=no_rename, synchronizer=synced)
        self.assertEqual(report["failure_code"], "P43-FINAL-DIRECTORY-MISSING")
        self.assertEqual(report["publication"]["state"], "absent")
        self.assert_absent(final)

    def test_final_verification_failure_rolls_back_to_absent(self) -> None:
        def tamper(final: Path) -> None:
            (final / "public-result.json").write_bytes(b"tampered\n")

        report, final = self.publish(post_rename=tamper, synchronizer=synced)
        self.assertEqual(report["failure_code"], "P43-FINAL-VERIFY-FAILURE")
        self.assertEqual(report["publication"]["state"], "rolled-back")
        self.assertEqual(report["publication"]["sync"]["rollback_parent"]["status"], "synced")
        self.assert_absent(final)

    def test_final_sync_failure_rolls_back_with_explicit_unsupported_posture(self) -> None:
        calls = 0

        def sync_then_fail(_: Path) -> publisher.SyncPosture:
            nonlocal calls
            calls += 1
            if calls == 1:
                return synced(_)
            if calls == 2:
                raise OSError("injected")
            return publisher.SyncPosture(
                "unsupported",
                "test-directory-sync-v1",
                "unsupported-by-platform-or-filesystem",
            )

        report, final = self.publish(synchronizer=sync_then_fail)
        self.assertEqual(report["failure_code"], "P43-FINAL-SYNC-FAILURE")
        self.assertEqual(report["publication"]["state"], "rolled-back")
        self.assertEqual(
            report["publication"]["sync"]["rollback_parent"]["status"], "unsupported"
        )
        self.assert_absent(final)

    def test_rollback_removal_or_sync_failure_is_indeterminate_never_success(self) -> None:
        def tamper(final: Path) -> None:
            (final / "public-result.json").write_bytes(b"tampered\n")

        def failed_remover(_: Path) -> None:
            raise OSError("injected")

        report, final = self.publish(
            post_rename=tamper, remover=failed_remover, synchronizer=synced
        )
        self.assertEqual(report["failure_code"], "P43-INDETERMINATE-PUBLICATION")
        self.assertEqual(report["publication"]["state"], "indeterminate")
        self.assertTrue(final.exists())

        calls = 0

        def sync_then_rollback_fails(_: Path) -> publisher.SyncPosture:
            nonlocal calls
            calls += 1
            if calls == 1:
                return synced(_)
            raise OSError("injected")

        report, final = self.publish(
            post_rename=tamper,
            final=self.final("rollback-sync"),
            synchronizer=sync_then_rollback_fails,
        )
        self.assertEqual(report["failure_code"], "P43-INDETERMINATE-PUBLICATION")
        self.assertEqual(report["publication"]["state"], "indeterminate")
        self.assertFalse(final.exists())


if __name__ == "__main__":
    unittest.main()
