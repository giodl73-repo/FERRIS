from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import sys
import unittest
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import corpus_materializer as materializer  # noqa: E402
from corpus_materializer import MaterializationError, REQUIRED_CASE_COUNT, materialize  # noqa: E402
from qualify import qualify  # noqa: E402
from verify_materialization import verify  # noqa: E402


RUN_ROOT = ROOT / ".test-runs"


def tree_digest(root: Path) -> str:
    aggregate = hashlib.sha256()
    for path in sorted(root.rglob("*")):
        if path.is_file():
            relative = path.relative_to(root).as_posix().encode("utf-8")
            aggregate.update(len(relative).to_bytes(8, "big"))
            aggregate.update(relative)
            aggregate.update(hashlib.sha256(path.read_bytes()).digest())
    return aggregate.hexdigest()


class Pulse35MaterializerTests(unittest.TestCase):
    def setUp(self) -> None:
        self.sandbox = RUN_ROOT / f"{self._testMethodName}-{os.getpid()}"
        shutil.rmtree(self.sandbox, ignore_errors=True)
        self.sandbox.mkdir(parents=True)

    def tearDown(self) -> None:
        shutil.rmtree(self.sandbox, ignore_errors=True)
        if RUN_ROOT.exists() and not any(RUN_ROOT.iterdir()):
            RUN_ROOT.rmdir()

    def seed(self, name: str, value: bytes | None = None) -> Path:
        path = self.sandbox / name
        path.write_bytes(hashlib.sha256(b"pulse-35-test-seed").digest() if value is None else value)
        return path

    def test_complete_materialization_derives_every_public_value_and_interaction(self) -> None:
        output = self.sandbox / "corpus"
        summary = materialize(self.seed("seed.bin"), output)
        self.assertEqual(summary["case_count"], REQUIRED_CASE_COUNT)
        self.assertEqual(summary["coverage_domains_closed"], "18/18")
        self.assertEqual(summary["coverage_interactions_closed"], "8/8")
        self.assertEqual(summary["logical_retries"], 0)
        verified = verify(output, self.sandbox / "seed.bin")
        self.assertTrue(verified["fresh_process_reload"])
        self.assertEqual(verified["residue_count"], 0)
        coverage = json.loads((output / "coverage-manifest.json").read_bytes())
        self.assertEqual(len(coverage["derived_catalog"]["domains"]), 18)
        self.assertEqual(len(coverage["derived_catalog"]["interactions"]), 8)
        self.assertTrue(all(ids for domain in coverage["derived_catalog"]["domains"] for ids in domain["witness_case_ids"]))
        self.assertTrue(all(entry["case_ids"] for interaction in coverage["derived_catalog"]["interactions"] for entry in interaction["tuples"]))
        self.assertIn(summary["directory_sync_posture"], {"synced", "unsupported"})
        self.assertTrue(all(record["state"] in {"synced", "unsupported"} for record in summary["directory_sync_records"]))
        fresh = subprocess.run([sys.executable, str(ROOT / "verify_materialization.py"), "--output", str(output), "--seed-file", str(self.sandbox / "seed.bin")], capture_output=True, check=False)
        self.assertEqual(fresh.returncode, 0, fresh.stderr.decode("utf-8"))

    def test_same_seed_identity_and_different_seed_divergence(self) -> None:
        seed = self.seed("seed.bin")
        first, second, third = self.sandbox / "first", self.sandbox / "second", self.sandbox / "third"
        materialize(seed, first)
        materialize(seed, second)
        materialize(self.seed("different.bin", hashlib.sha256(b"different").digest()), third)
        self.assertEqual(tree_digest(first), tree_digest(second))
        self.assertNotEqual(tree_digest(first), tree_digest(third))
        first_manifest = json.loads((first / "case-manifest.json").read_bytes())
        third_manifest = json.loads((third / "case-manifest.json").read_bytes())
        self.assertNotEqual(first_manifest["seed_commitment_sha256"], third_manifest["seed_commitment_sha256"])
        self.assertEqual(first_manifest["seed_commitment_algorithm"], "sha256(ferris-p35-seed-commitment-v1\\0 || seed)")
        self.assertEqual(first_manifest["derivation"], "hmac-sha256-seed-key-domain-purpose-counter-v1")
        with self.assertRaisesRegex(ValueError, "commitment"):
            verify(first, self.sandbox / "different.bin")

    def test_one_case_seed_length_replay_residue_and_extra_controls_are_rejected(self) -> None:
        with self.assertRaisesRegex(MaterializationError, "unavailable"):
            materialize(self.sandbox / "missing.bin", self.sandbox / "missing-output")
        directory = self.sandbox / "directory-seed"
        directory.mkdir()
        with self.assertRaisesRegex(MaterializationError, "regular"):
            materialize(directory, self.sandbox / "directory-output")
        for length in (0, 31, 33):
            with self.assertRaisesRegex(MaterializationError, "exactly 32"):
                materialize(self.seed(f"length-{length}.bin", b"x" * length), self.sandbox / f"length-{length}")
        seed = self.seed("seed.bin")
        with self.assertRaisesRegex(MaterializationError, "exactly 70 complete"):
            materialize(seed, self.sandbox / "one-case", 1)
        output = self.sandbox / "corpus"
        materialize(seed, output)
        with self.assertRaisesRegex(MaterializationError, "already exists"):
            materialize(seed, output)
        (output / "unexpected.txt").write_text("extra", encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "missing or extra"):
            verify(output, self.sandbox / "seed.bin")
        residue = self.sandbox / ".residue-output.partial-control"
        residue.mkdir()
        with self.assertRaisesRegex(MaterializationError, "residue"):
            materialize(seed, self.sandbox / "residue-output")

    def test_semantic_witness_tamper_cannot_fake_coverage(self) -> None:
        output = self.sandbox / "corpus"
        materialize(self.seed("seed.bin"), output)
        manifest_path = output / "case-manifest.json"
        coverage_path = output / "coverage-manifest.json"
        manifest = json.loads(manifest_path.read_bytes())
        coverage = json.loads(coverage_path.read_bytes())
        manifest["cases"][3]["semantic_witnesses"]["before"]["json_value_kinds"] = []
        manifest_path.write_bytes(materializer.canonical_json(manifest))
        coverage["case_manifest_sha256"] = materializer.sha256(manifest_path.read_bytes())
        coverage_path.write_bytes(materializer.canonical_json(coverage))
        with self.assertRaisesRegex(ValueError, "semantic witnesses"):
            verify(output, self.sandbox / "seed.bin")

    def test_publish_has_one_attempt_and_final_sync_rolls_back_for_reentry(self) -> None:
        seed = self.seed("seed.bin")
        output = self.sandbox / "corpus"
        real_replace = materializer.os.replace
        attempts = 0

        def fail_final_directory_rename(source: Path, destination: Path) -> None:
            nonlocal attempts
            if source.is_dir():
                attempts += 1
                raise PermissionError("injected final rename failure")
            real_replace(source, destination)

        with patch.object(materializer.os, "replace", side_effect=fail_final_directory_rename):
            with self.assertRaises(PermissionError):
                materialize(seed, output)
        self.assertEqual(attempts, 1)
        self.assertFalse(output.exists())

        real_sync = materializer._sync_directory

        def fail_published_parent(directory: Path) -> None:
            if directory == output.parent and output.exists():
                raise OSError("injected final parent sync failure")
            return real_sync(directory)

        with patch.object(materializer, "_sync_directory", side_effect=fail_published_parent):
            with self.assertRaisesRegex(MaterializationError, "rolled back"):
                materialize(seed, output)
        self.assertFalse(output.exists())
        materialize(seed, output)
        self.assertEqual(verify(output, seed)["logical_retries"], 0)

    def test_prepare_destination_cleans_residue_when_its_post_creation_sync_fails(self) -> None:
        seed = self.seed("seed.bin")
        output = self.sandbox / "corpus"
        real_sync = materializer._sync_directory
        calls = 0

        def fail_once(directory: Path):
            nonlocal calls
            if directory == output.parent and calls == 0:
                calls += 1
                raise OSError("injected stage-parent sync failure")
            return real_sync(directory)

        with patch.object(materializer, "_sync_directory", side_effect=fail_once):
            with self.assertRaisesRegex(MaterializationError, "staging residue was rolled back"):
                materialize(seed, output)
        self.assertFalse(output.exists())
        self.assertFalse(any(".partial-" in path.name for path in self.sandbox.iterdir()))
        materialize(seed, output)

    def test_unc_normalization_preserves_server_share_authority(self) -> None:
        normalized, steps = materializer.normalize_lexical_path(r"\\server\share\folder\..\artifact.bin")
        self.assertEqual(normalized, "//server/share/artifact.bin")
        self.assertIn("unc-authority-preserve", steps)
        self.assertNotIn("repeated-separator-collapse", steps)

    def test_qualification_runs_twenty_complete_cycles_and_no_seed_bytes_publish(self) -> None:
        receipt = qualify(20, self.sandbox / "qualification-work", self.sandbox / "qualification-receipt.json")
        payload = receipt["payload"]
        self.assertEqual(payload["cycles_run"], 20)
        self.assertEqual(payload["cycles_passed"], 20)
        self.assertEqual(payload["case_count_per_cycle"], REQUIRED_CASE_COUNT)
        self.assertEqual(payload["semantic_fake_coverage_rejections"], 20)
        self.assertEqual(payload["seed_length_rejections"], 3)
        self.assertEqual(payload["logical_retries"], 0)
        self.assertFalse((self.sandbox / "qualification-work").exists())
        seed_bytes = hashlib.sha256(b"pulse-35-synthetic-qualification-seed-v1").digest()
        self.assertNotIn(seed_bytes, (self.sandbox / "qualification-receipt.json").read_bytes())


if __name__ == "__main__":
    unittest.main()
