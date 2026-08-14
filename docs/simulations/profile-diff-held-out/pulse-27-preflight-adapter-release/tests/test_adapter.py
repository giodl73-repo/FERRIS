from __future__ import annotations

import hashlib
import shutil
import sys
import unittest
import uuid
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from adapter import PAIR_IDS, tree_state, verify_exact_two


COLLECTOR_ROOT = ROOT / "collector"
if str(COLLECTOR_ROOT) not in sys.path:
    sys.path.insert(0, str(COLLECTOR_ROOT))

from durability import canonical_json, temporary_path
from sealed_store import read_sealed_json, write_record, write_sealed_json
from synthetic_commands import expected_observation


RUN_ROOT = ROOT / "test-runs"


def fixture_environment(platform_name: str) -> dict:
    value = {
        "system": platform_name,
        "release": "public-fixture",
        "version": "1",
        "machine": "x86_64",
        "python_implementation": "fixture",
        "python_version": "3",
        "shell": "fixture-shell",
    }
    value["environment_sha256"] = (
        "sha256:" + hashlib.sha256(canonical_json(value)).hexdigest()
    )
    return value


def process_payload(platform_name: str, index: int) -> dict:
    expected = expected_observation(platform_name, index)
    return {
        "schema": "collector-synthetic-observation-v1",
        "platform": platform_name,
        "index": index,
        "route": expected["route"],
        "expected_exit": expected["expected_exit"],
        "exit_code": expected["expected_exit"],
        "stdout": expected["stdout"],
        "stderr": expected["stderr"],
        "environment": fixture_environment(platform_name),
    }


def make_fixture(root: Path, count: int = 2) -> None:
    for store_name in ("windows", "ubuntu", "pairs"):
        (root / store_name / "records").mkdir(parents=True)
    for index in range(count):
        windows_receipt = write_record(
            root / "windows", process_payload("windows", index)
        )
        ubuntu_receipt = write_record(
            root / "ubuntu", process_payload("ubuntu", index)
        )
        write_record(
            root / "pairs",
            {
                "schema": "exact-two-preflight-pair-seal-v1",
                "platform": "pair",
                "index": index,
                "pair_id": PAIR_IDS[index] if index < 2 else "preflight-pair-002",
                "windows_record_sha256": windows_receipt["record_sha256"],
                "ubuntu_record_sha256": ubuntu_receipt["record_sha256"],
            },
        )


class ExactTwoAdapterTests(unittest.TestCase):
    def setUp(self) -> None:
        self.sandbox = RUN_ROOT / f"adapter-{uuid.uuid4().hex}"
        self.sandbox.mkdir(parents=True)

    def tearDown(self) -> None:
        shutil.rmtree(self.sandbox, ignore_errors=True)
        if RUN_ROOT.exists() and not any(RUN_ROOT.iterdir()):
            RUN_ROOT.rmdir()

    def test_exact_two_pairs_pass(self) -> None:
        make_fixture(self.sandbox)
        result = verify_exact_two(self.sandbox)
        self.assertEqual(result["pair_ids"], list(PAIR_IDS))
        self.assertEqual(result["process_record_count"], 4)
        self.assertEqual(result["pair_seal_count"], 2)

    def test_one_pair_only_is_rejected(self) -> None:
        make_fixture(self.sandbox, 1)
        with self.assertRaisesRegex(ValueError, "cardinality|naming"):
            verify_exact_two(self.sandbox)

    def test_extra_third_pair_is_rejected(self) -> None:
        make_fixture(self.sandbox, 3)
        with self.assertRaisesRegex(ValueError, "cardinality|naming"):
            verify_exact_two(self.sandbox)

    def test_duplicate_row_is_rejected(self) -> None:
        make_fixture(self.sandbox)
        duplicate = process_payload("windows", 0)
        write_sealed_json(
            self.sandbox / "windows" / "records" / "pair-001.json",
            duplicate,
        )
        with self.assertRaisesRegex(ValueError, "duplicate-index|identity"):
            verify_exact_two(self.sandbox)

    def test_missing_partner_is_rejected(self) -> None:
        make_fixture(self.sandbox)
        (self.sandbox / "ubuntu" / "records" / "pair-001.json").unlink()
        with self.assertRaisesRegex(ValueError, "cardinality|naming"):
            verify_exact_two(self.sandbox)

    def test_partial_seal_is_rejected(self) -> None:
        make_fixture(self.sandbox)
        (self.sandbox / "pairs" / "records" / "pair-001.json").unlink()
        with self.assertRaisesRegex(ValueError, "cardinality|naming"):
            verify_exact_two(self.sandbox)

    def test_tampering_is_rejected(self) -> None:
        make_fixture(self.sandbox)
        path = self.sandbox / "windows" / "records" / "pair-000.json"
        content = path.read_bytes()
        path.write_bytes(content.replace(b"WINDOWS", b"TAMPER!", 1))
        with self.assertRaisesRegex(ValueError, "digest mismatch"):
            verify_exact_two(self.sandbox)

    def test_atomic_write_residue_is_rejected(self) -> None:
        make_fixture(self.sandbox)
        temporary_path(
            self.sandbox / "pairs" / "records" / "pair-001.json", "partial"
        ).write_bytes(b"partial")
        with self.assertRaisesRegex(ValueError, "residue"):
            verify_exact_two(self.sandbox)

    def test_reload_is_idempotent_and_read_only(self) -> None:
        make_fixture(self.sandbox)
        before = tree_state(self.sandbox)
        first = verify_exact_two(self.sandbox)
        second = verify_exact_two(self.sandbox)
        after = tree_state(self.sandbox)
        self.assertEqual(first, second)
        self.assertEqual(before, after)
        self.assertTrue(first["idempotent_read_only"])


if __name__ == "__main__":
    unittest.main()
