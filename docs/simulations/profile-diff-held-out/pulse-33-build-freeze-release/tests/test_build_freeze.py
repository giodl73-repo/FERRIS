from __future__ import annotations

import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from build_freeze import (  # noqa: E402
    BuildFreezeError,
    canonical_bytes,
    discover_executable,
    freeze_filename,
    package_metadata,
    sha256_file,
    validate_cutoff,
)


CUTOFF = "29517d732db13cc2ffa304684b344f3538ab587d"
PACKAGE_ID = "path+file:///checkout#ferris-cli@0.1.0"


def artifact(
    executable: str | None,
    *,
    package_id: str = PACKAGE_ID,
    name: str = "ferris",
    kind: list[str] | None = None,
) -> str:
    return json.dumps(
        {
            "executable": executable,
            "package_id": package_id,
            "reason": "compiler-artifact",
            "target": {"kind": kind or ["bin"], "name": name},
        }
    )


class BuildFreezeTests(unittest.TestCase):
    def test_cutoff_accepts_lowercase_sha(self) -> None:
        self.assertEqual(validate_cutoff(CUTOFF), CUTOFF)

    def test_cutoff_rejects_uppercase(self) -> None:
        with self.assertRaises(BuildFreezeError):
            validate_cutoff(CUTOFF.upper())

    def test_cutoff_rejects_short_value(self) -> None:
        with self.assertRaises(BuildFreezeError):
            validate_cutoff("29517d7")

    def test_freeze_filename_linux(self) -> None:
        self.assertEqual(
            freeze_filename("ubuntu-24.04-x86_64", CUTOFF),
            f"ferris-ubuntu-24.04-x86_64-{CUTOFF}",
        )

    def test_freeze_filename_windows(self) -> None:
        self.assertEqual(
            freeze_filename("windows-x86_64", CUTOFF),
            f"ferris-windows-x86_64-{CUTOFF}.exe",
        )

    def test_canonical_bytes_are_order_independent(self) -> None:
        self.assertEqual(canonical_bytes({"b": 2, "a": 1}), canonical_bytes({"a": 1, "b": 2}))

    def test_package_metadata_selects_ferris_binary(self) -> None:
        metadata = {
            "packages": [
                {
                    "id": PACKAGE_ID,
                    "name": "ferris-cli",
                    "targets": [{"kind": ["bin"], "name": "ferris"}],
                }
            ]
        }
        self.assertEqual(package_metadata(metadata), (PACKAGE_ID, "ferris"))

    def test_package_metadata_rejects_missing_package(self) -> None:
        with self.assertRaises(BuildFreezeError):
            package_metadata({"packages": []})

    def test_discovery_uses_cargo_artifact_path(self) -> None:
        path = "/custom/target/release/ferris"
        actual = discover_executable(
            [artifact(path)],
            PACKAGE_ID,
            "ferris",
            "ubuntu-24.04-x86_64",
            exists=lambda candidate: candidate == path,
        )
        self.assertEqual(actual, path)

    def test_discovery_ignores_unrelated_messages(self) -> None:
        path = r"C:\custom\target\release\ferris.exe"
        actual = discover_executable(
            [
                "Compiling ferris-cli",
                artifact("/wrong", package_id="other"),
                artifact(path),
            ],
            PACKAGE_ID,
            "ferris",
            "windows-x86_64",
            exists=lambda candidate: candidate == path,
        )
        self.assertEqual(actual, path)

    def test_discovery_rejects_conflicts(self) -> None:
        with self.assertRaisesRegex(BuildFreezeError, "conflicting"):
            discover_executable(
                [artifact("/a/ferris"), artifact("/b/ferris")],
                PACKAGE_ID,
                "ferris",
                "ubuntu-24.04-x86_64",
                exists=lambda _: True,
            )

    def test_discovery_rejects_missing_file(self) -> None:
        with self.assertRaisesRegex(BuildFreezeError, "does not exist"):
            discover_executable(
                [artifact("/missing/ferris")],
                PACKAGE_ID,
                "ferris",
                "ubuntu-24.04-x86_64",
                exists=lambda _: False,
            )

    def test_discovery_enforces_windows_suffix(self) -> None:
        with self.assertRaisesRegex(BuildFreezeError, ".exe"):
            discover_executable(
                [artifact("/target/release/ferris")],
                PACKAGE_ID,
                "ferris",
                "windows-x86_64",
                exists=lambda _: True,
            )

    def test_sha256_file(self) -> None:
        with tempfile.TemporaryDirectory(dir=ROOT) as directory:
            sample = Path(directory) / "sample"
            sample.write_bytes(b"abc")
            self.assertEqual(
                sha256_file(sample),
                hashlib.sha256(b"abc").hexdigest(),
            )


if __name__ == "__main__":
    unittest.main()
