#!/usr/bin/env python3
"""Qualification controls for the Pulse 47 public outcome witness."""

from __future__ import annotations

import copy
import json
import os
import shutil
import sys
import unittest
from pathlib import Path


RELEASE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(RELEASE))
import publication_outcome_witness as witness  # noqa: E402


def synced(_: Path) -> witness.SyncPosture:
    return witness.SyncPosture("synced", witness.SYNC_MECHANISM, None)


class PublicationOutcomeWitnessTests(unittest.TestCase):
    def setUp(self) -> None:
        self.runtime = Path(__file__).parent / f".pulse-47-runtime-{os.getpid()}"
        shutil.rmtree(self.runtime, ignore_errors=True)
        self.runtime.mkdir()
        fixtures = RELEASE / "fixtures"
        self.published = json.loads(
            (fixtures / "pulse-43-published-summary.json").read_text(encoding="utf-8")
        )
        self.indeterminate = json.loads(
            (fixtures / "pulse-43-indeterminate-summary.json").read_text(encoding="utf-8")
        )
        self.catalog = {"schema": "ferris.pulse-43-ordered-gate-catalog/v1"}
        self.events = [{"schema": "ferris.pulse-43-ordered-result-event/v1"}]

    def tearDown(self) -> None:
        shutil.rmtree(self.runtime, ignore_errors=True)

    def roots(self, name: str) -> tuple[Path, Path]:
        root = self.runtime / name
        root.mkdir()
        return root / "p43-final", root / "witness-final"

    def invoke(self, summary: object, calls: list[tuple[object, object, object]]):
        def call(catalog: object, events: object, p43_final: object) -> object:
            calls.append((catalog, events, p43_final))
            return copy.deepcopy(summary)

        return call

    def assert_witness_failure(self, result: dict[str, object], code: str) -> None:
        self.assertEqual(
            result,
            {
                "failure_code": code,
                "schema": witness.SUMMARY_SCHEMA,
                "witness_publication": result["witness_publication"],
            },
        )
        self.assertFalse(result["witness_publication"]["final_files_present"])
        self.assertNotIn("publication_outcome", result)
        self.assertNotIn("pulse_43", result)

    def test_published_witness_is_path_free_rehashed_and_once_only(self) -> None:
        p43_final, witness_final = self.roots("published")
        calls: list[tuple[object, object, object]] = []
        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=self.invoke(self.published, calls),
            synchronizer=synced,
        )
        self.assertEqual(len(calls), 1)
        self.assertEqual(calls[0], (self.catalog, self.events, p43_final))
        self.assertEqual(result["outcome"], "published")
        self.assertEqual(result["witness_publication"]["files"], "2/2")
        self.assertTrue(result["witness_publication"]["final_files_present"])
        self.assertEqual(result["witness_publication"]["rename_attempts"], 1)
        self.assertEqual(result["witness_publication"]["retries"], 0)
        self.assertEqual(
            {entry.name for entry in witness_final.iterdir()},
            {"publication-witness.json", "release-receipt.json"},
        )
        hashes = witness.verify_witness_directory(witness_final)
        self.assertEqual(result["witness_publication"]["raw_hashes"], hashes)
        payload = json.loads(
            (witness_final / "publication-witness.json").read_text(encoding="ascii")
        )["payload"]
        self.assertEqual(payload["pulse_43"]["identities"], witness._p43_identities())
        outcome = payload["publication_outcome"]
        self.assertEqual(outcome["publication"]["final_files"], "2/2")
        self.assertEqual(outcome["ordered_execution"]["completed_gate_count"], 3)
        self.assertNotIn("gate_id", json.dumps(outcome, sort_keys=True))
        rendered = witness.canonical_bytes(result).decode("ascii")
        self.assertNotIn(str(p43_final), rendered)
        self.assertNotIn(str(witness_final), rendered)

    def test_indeterminate_p43_failure_witnesses_only_approved_posture(self) -> None:
        p43_final, witness_final = self.roots("indeterminate")
        calls: list[tuple[object, object, object]] = []
        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=self.invoke(self.indeterminate, calls),
            synchronizer=synced,
        )
        self.assertEqual(len(calls), 1)
        outcome = result["publication_outcome"]
        self.assertEqual(outcome["kind"], "failed")
        self.assertEqual(outcome["failure_code"], "P43-INDETERMINATE-PUBLICATION")
        self.assertEqual(outcome["publication"]["state"], "indeterminate")
        self.assertFalse(outcome["publication"]["final_files_present"])
        self.assertEqual(outcome["publication"]["rename_attempts"], 1)
        self.assertEqual(
            outcome["publication"]["sync"]["rollback_parent"]["status"], "failed"
        )
        self.assertNotIn("ordered_execution", outcome)
        self.assertNotIn("public_self_validation", outcome)
        self.assertFalse(p43_final.exists())
        self.assertTrue(witness_final.exists())

    def test_absent_and_rolled_back_p43_failures_preserve_only_posture(self) -> None:
        for state, code, rename_attempts, sync in (
            (
                "absent",
                "P43-STAGE-COPY-FAILURE",
                0,
                {
                    name: witness._not_attempted_sync().public()
                    for name in ("stage", "final_parent", "rollback_parent")
                },
            ),
            (
                "rolled-back",
                "P43-FINAL-VERIFY-FAILURE",
                1,
                {
                    "stage": synced(Path()).public(),
                    "final_parent": witness._not_attempted_sync().public(),
                    "rollback_parent": synced(Path()).public(),
                },
            ),
        ):
            p43_final, witness_final = self.roots(f"p43-{state}")
            summary = copy.deepcopy(self.indeterminate)
            summary["failure_code"] = code
            summary["publication"]["state"] = state
            summary["publication"]["rename_attempts"] = rename_attempts
            summary["publication"]["sync"] = sync
            result = witness.witness_pulse_43(
                self.catalog,
                self.events,
                p43_final,
                witness_final,
                invoker=lambda *_: summary,
                synchronizer=synced,
            )
            outcome = result["publication_outcome"]
            self.assertEqual(outcome["kind"], "failed")
            self.assertEqual(outcome["failure_code"], code)
            self.assertEqual(outcome["publication"]["state"], state)
            self.assertEqual(outcome["publication"]["rename_attempts"], rename_attempts)
            self.assertNotIn("ordered_execution", outcome)
            self.assertNotIn("public_self_validation", outcome)

    def test_malformed_p43_summary_is_not_published(self) -> None:
        p43_final, witness_final = self.roots("malformed")
        calls: list[tuple[object, object, object]] = []
        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=self.invoke({"schema": witness.P43_SUMMARY_SCHEMA}, calls),
            synchronizer=synced,
        )
        self.assertEqual(len(calls), 1)
        self.assert_witness_failure(result, "P47-P43-SUMMARY-MALFORMED")
        self.assertFalse(witness_final.exists())

    def test_success_shaped_partial_p43_summary_is_not_published(self) -> None:
        p43_final, witness_final = self.roots("partial")
        partial = copy.deepcopy(self.published)
        del partial["publication"]["raw_hashes"]
        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=lambda *_: partial,
            synchronizer=synced,
        )
        self.assert_witness_failure(result, "P47-P43-SUMMARY-MALFORMED")
        self.assertFalse(witness_final.exists())

    def test_thrown_p43_invocation_is_not_published(self) -> None:
        p43_final, witness_final = self.roots("thrown")

        def throw(*_: object) -> object:
            raise RuntimeError("hidden invocation detail")

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=throw,
            synchronizer=synced,
        )
        self.assert_witness_failure(result, "P47-P43-INVOCATION-FAILURE")
        self.assertNotIn("hidden", witness.canonical_bytes(result).decode("ascii"))

    def test_invalid_witness_root_does_not_invoke_p43(self) -> None:
        p43_final, _ = self.roots("invalid-root")
        calls: list[tuple[object, object, object]] = []
        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            "relative-witness-root",
            invoker=self.invoke(self.published, calls),
            synchronizer=synced,
        )
        self.assertEqual(calls, [])
        self.assert_witness_failure(result, "P47-WITNESS-FINAL-ROOT-INVALID")

    def test_stage_copy_or_fsync_failure_omits_p43_details(self) -> None:
        p43_final, witness_final = self.roots("copy-failure")
        calls: list[tuple[object, object, object]] = []

        def fail_writer(_: Path, __: bytes) -> None:
            raise OSError("write")

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=self.invoke(self.published, calls),
            writer=fail_writer,
            synchronizer=synced,
        )
        self.assertEqual(len(calls), 1)
        self.assert_witness_failure(result, "P47-WITNESS-STAGE-COPY-FAILURE")
        self.assertFalse(witness_final.exists())

    def test_stage_verify_failure_omits_p43_details(self) -> None:
        p43_final, witness_final = self.roots("stage-verify")

        def fail_verify(_: Path) -> dict[str, str]:
            raise witness.WitnessFailure("injected")

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=lambda *_: copy.deepcopy(self.published),
            verifier=fail_verify,
            synchronizer=synced,
        )
        self.assert_witness_failure(result, "P47-WITNESS-STAGE-VERIFY-FAILURE")

    def test_stage_sync_failure_omits_p43_details(self) -> None:
        p43_final, witness_final = self.roots("stage-sync")

        def fail_sync(_: Path) -> witness.SyncPosture:
            raise OSError("sync")

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=lambda *_: copy.deepcopy(self.published),
            synchronizer=fail_sync,
        )
        self.assert_witness_failure(result, "P47-WITNESS-STAGE-SYNC-FAILURE")
        self.assertEqual(result["witness_publication"]["sync"]["stage"]["status"], "failed")

    def test_rename_failure_omits_p43_details(self) -> None:
        p43_final, witness_final = self.roots("rename")

        def fail_rename(_: Path, __: Path) -> None:
            raise OSError("rename")

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=lambda *_: copy.deepcopy(self.published),
            renamer=fail_rename,
            synchronizer=synced,
        )
        self.assert_witness_failure(result, "P47-WITNESS-RENAME-FAILURE")
        self.assertEqual(result["witness_publication"]["rename_attempts"], 1)

    def test_partial_rename_rolls_back_without_p43_details(self) -> None:
        p43_final, witness_final = self.roots("partial-rename")

        def partial_rename(stage: Path, final: Path) -> None:
            os.replace(stage, final)
            raise OSError("after rename")

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=lambda *_: copy.deepcopy(self.published),
            renamer=partial_rename,
            synchronizer=synced,
        )
        self.assert_witness_failure(result, "P47-WITNESS-RENAME-FAILURE")
        self.assertEqual(result["witness_publication"]["state"], "rolled-back")
        self.assertFalse(witness_final.exists())

    def test_final_verify_failure_rolls_back_without_p43_details(self) -> None:
        p43_final, witness_final = self.roots("final-verify")

        def corrupt(final: Path) -> None:
            (final / "extra.json").write_text("{}\n", encoding="ascii")

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=lambda *_: copy.deepcopy(self.published),
            synchronizer=synced,
            post_rename=corrupt,
        )
        self.assert_witness_failure(result, "P47-WITNESS-FINAL-VERIFY-FAILURE")
        self.assertEqual(result["witness_publication"]["state"], "rolled-back")

    def test_final_sync_failure_rolls_back_without_p43_details(self) -> None:
        p43_final, witness_final = self.roots("final-sync")
        sync_calls = 0

        def fail_final_sync(_: Path) -> witness.SyncPosture:
            nonlocal sync_calls
            sync_calls += 1
            if sync_calls == 2:
                raise OSError("final parent")
            return synced(Path())

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=lambda *_: copy.deepcopy(self.published),
            synchronizer=fail_final_sync,
        )
        self.assert_witness_failure(result, "P47-WITNESS-FINAL-SYNC-FAILURE")
        self.assertEqual(result["witness_publication"]["state"], "rolled-back")
        self.assertEqual(result["witness_publication"]["sync"]["final_parent"]["status"], "failed")

    def test_rollback_removal_failure_is_indeterminate(self) -> None:
        p43_final, witness_final = self.roots("rollback-removal")

        def corrupt(final: Path) -> None:
            (final / "extra.json").write_text("{}\n", encoding="ascii")

        def fail_remove(path: Path) -> None:
            if path == witness_final:
                raise OSError("remove")
            witness.remove_tree(path)

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=lambda *_: copy.deepcopy(self.published),
            remover=fail_remove,
            synchronizer=synced,
            post_rename=corrupt,
        )
        self.assert_witness_failure(result, "P47-INDETERMINATE-WITNESS-PUBLICATION")
        self.assertEqual(result["witness_publication"]["state"], "indeterminate")

    def test_rollback_sync_failure_is_indeterminate(self) -> None:
        p43_final, witness_final = self.roots("rollback-sync")
        sync_calls = 0

        def corrupt(final: Path) -> None:
            (final / "extra.json").write_text("{}\n", encoding="ascii")

        def fail_rollback_sync(_: Path) -> witness.SyncPosture:
            nonlocal sync_calls
            sync_calls += 1
            if sync_calls == 2:
                raise OSError("rollback")
            return synced(Path())

        result = witness.witness_pulse_43(
            self.catalog,
            self.events,
            p43_final,
            witness_final,
            invoker=lambda *_: copy.deepcopy(self.published),
            synchronizer=fail_rollback_sync,
            post_rename=corrupt,
        )
        self.assert_witness_failure(result, "P47-INDETERMINATE-WITNESS-PUBLICATION")
        self.assertEqual(
            result["witness_publication"]["sync"]["rollback_parent"]["status"], "failed"
        )

    def test_duplicate_json_members_are_rejected_on_final_rehash(self) -> None:
        p43_final, witness_final = self.roots("duplicate-json")
        witness_dir = witness_final
        witness_dir.mkdir()
        (witness_dir / "publication-witness.json").write_text(
            '{"schema":"one","schema":"two"}\n', encoding="ascii"
        )
        (witness_dir / "release-receipt.json").write_text("{}\n", encoding="ascii")
        with self.assertRaises(witness.WitnessFailure) as raised:
            witness.verify_witness_directory(witness_dir)
        self.assertEqual(raised.exception.code, "P47-WITNESS-VERIFY-FAILURE")


if __name__ == "__main__":
    unittest.main()
