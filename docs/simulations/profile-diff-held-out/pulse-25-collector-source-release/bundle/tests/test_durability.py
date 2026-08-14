from __future__ import annotations

import os
import shutil
import unittest
import uuid
from pathlib import Path
from unittest import mock

import durability


ROOT = Path(__file__).resolve().parents[1]
RUN_ROOT = ROOT / "test-runs"


class DurabilityTests(unittest.TestCase):
    def setUp(self) -> None:
        self.sandbox = RUN_ROOT / f"{os.name}-{uuid.uuid4().hex}"
        self.sandbox.mkdir(parents=True)

    def tearDown(self) -> None:
        shutil.rmtree(self.sandbox, ignore_errors=True)
        if RUN_ROOT.exists() and not any(RUN_ROOT.iterdir()):
            RUN_ROOT.rmdir()

    def test_file_descriptor_is_live_and_writable_during_fsync(self) -> None:
        target = self.sandbox / "value.bin"
        real_open = durability.os.open
        real_fsync = durability.os.fsync
        opened_flags = []

        def checked_open(path, flags, mode=0o777):
            descriptor = real_open(path, flags, mode)
            if durability.is_temporary_path(Path(path)):
                opened_flags.append(flags)
            return descriptor

        def checked_fsync(descriptor):
            os.fstat(descriptor)
            return real_fsync(descriptor)

        with mock.patch.object(durability.os, "open", side_effect=checked_open):
            with mock.patch.object(durability.os, "fsync", side_effect=checked_fsync):
                durability.atomic_write_bytes(target, b"live-handle")

        self.assertEqual(target.read_bytes(), b"live-handle")
        self.assertEqual(len(opened_flags), 1)
        self.assertTrue(opened_flags[0] & os.O_WRONLY)

    def test_flush_sync_replace_and_directory_ordering(self) -> None:
        events = []
        target = self.sandbox / "ordered.bin"
        receipt = durability.atomic_write_bytes(
            target, b"ordered", observer=events.append, token="ordering"
        )
        required = [
            "temp-open",
            "userspace-write",
            "userspace-flush",
            "file-sync",
            "temp-close",
            "atomic-replace",
            "directory-open",
            "directory-close",
            "complete",
        ]
        positions = [events.index(event) for event in required]
        self.assertEqual(positions, sorted(positions))
        if receipt.directory_sync.state == "synced":
            self.assertLess(
                events.index("directory-open"),
                events.index("directory-sync"),
            )
            self.assertLess(
                events.index("directory-sync"),
                events.index("directory-close"),
            )
        else:
            self.assertEqual(receipt.directory_sync.state, "unsupported")

    def test_temp_is_removed_when_replace_fails(self) -> None:
        target = self.sandbox / "failed.bin"
        with mock.patch.object(
            durability.os, "replace", side_effect=OSError("injected replace failure")
        ):
            with self.assertRaises(OSError):
                durability.atomic_write_bytes(
                    target, b"not-committed", token="replace-failure"
                )
        self.assertFalse(target.exists())
        self.assertEqual(durability.find_residue(self.sandbox), [])

    def test_existing_file_is_atomically_replaced(self) -> None:
        target = self.sandbox / "replace.bin"
        target.write_bytes(b"old")
        receipt = durability.atomic_write_bytes(target, b"new", token="replace")
        self.assertTrue(receipt.replaced_existing)
        self.assertEqual(target.read_bytes(), b"new")
        self.assertEqual(durability.find_residue(self.sandbox), [])

    def test_interruption_residue_is_detected(self) -> None:
        target = self.sandbox / "record.json"
        residue = durability.temporary_path(target, "interrupted")
        residue.write_bytes(b"partial")
        self.assertEqual(durability.find_residue(self.sandbox), [residue])

    def test_directory_sync_status_is_explicit(self) -> None:
        status = durability.sync_directory(self.sandbox)
        self.assertIn(status.state, {"synced", "unsupported"})
        self.assertTrue(status.mechanism)
        self.assertTrue(status.detail)


if __name__ == "__main__":
    unittest.main()
