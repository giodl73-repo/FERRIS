from __future__ import annotations

import os
import shutil
import unittest
import uuid
from pathlib import Path

from durability import temporary_path
from sealed_store import (
    read_sealed_json,
    seal_store,
    tree_state,
    verify_store,
    write_record,
)


ROOT = Path(__file__).resolve().parents[1]
RUN_ROOT = ROOT / "test-runs"


class SealedStoreTests(unittest.TestCase):
    def setUp(self) -> None:
        self.sandbox = RUN_ROOT / f"store-{os.name}-{uuid.uuid4().hex}"
        (self.sandbox / "records").mkdir(parents=True)

    def tearDown(self) -> None:
        shutil.rmtree(self.sandbox, ignore_errors=True)
        if RUN_ROOT.exists() and not any(RUN_ROOT.iterdir()):
            RUN_ROOT.rmdir()

    def make_store(self) -> None:
        for index in range(3):
            write_record(
                self.sandbox,
                {
                    "platform": "windows",
                    "index": index,
                    "expected_exit": 0 if index != 1 else 3,
                    "exit_code": 0 if index != 1 else 3,
                    "stdout": "fixed" if index != 1 else "",
                    "stderr": "fixed" if index == 1 else "",
                },
            )
        seal_store(self.sandbox, "windows", 3)

    def test_verification_is_idempotent_and_read_only(self) -> None:
        self.make_store()
        before = tree_state(self.sandbox)
        first = verify_store(self.sandbox, "windows", 3)
        second = verify_store(self.sandbox, "windows", 3)
        after = tree_state(self.sandbox)
        self.assertEqual(first, second)
        self.assertEqual(before, after)
        self.assertTrue(first["idempotent_read_only"])

    def test_verification_rejects_crash_residue(self) -> None:
        self.make_store()
        residue = temporary_path(self.sandbox / "manifest.json", "crash")
        residue.write_bytes(b"partial")
        with self.assertRaisesRegex(ValueError, "residue"):
            verify_store(self.sandbox, "windows", 3)

    def test_verification_rejects_wrong_cardinality(self) -> None:
        self.make_store()
        (self.sandbox / "records" / "pair-002.json").unlink()
        with self.assertRaisesRegex(ValueError, "cardinality|naming"):
            verify_store(self.sandbox, "windows", 3)

    def test_seal_detects_tampering(self) -> None:
        self.make_store()
        path = self.sandbox / "records" / "pair-000.json"
        content = path.read_bytes()
        path.write_bytes(content.replace(b"fixed", b"altered", 1))
        with self.assertRaisesRegex(ValueError, "digest mismatch"):
            read_sealed_json(path)


if __name__ == "__main__":
    unittest.main()
