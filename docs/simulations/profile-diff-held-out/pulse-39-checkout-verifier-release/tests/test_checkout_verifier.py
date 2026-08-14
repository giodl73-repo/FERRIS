"""Unit and disposable-checkout tests for the public Pulse 39 verifier."""

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


HERE = Path(__file__).resolve()
RELEASE_ROOT = HERE.parents[1]
REPOSITORY_ROOT = HERE.parents[5]
VERIFIER_PATH = RELEASE_ROOT / "checkout_verifier.py"
SPEC = importlib.util.spec_from_file_location("pulse_39_verifier", VERIFIER_PATH)
assert SPEC is not None and SPEC.loader is not None
verifier = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = verifier
SPEC.loader.exec_module(verifier)


def run(command: list[str], cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=cwd,
        check=True,
        capture_output=True,
        text=True,
    )


def remove_tree(path: Path) -> None:
    def remove_readonly(function: object, failed_path: str, exception: object) -> None:
        del exception
        os.chmod(failed_path, stat.S_IWRITE)
        function(failed_path)

    if path.exists():
        shutil.rmtree(path, onerror=remove_readonly)


class CheckoutVerifierTests(unittest.TestCase):
    def setUp(self) -> None:
        self.work = (
            REPOSITORY_ROOT
            / "target"
            / f"pulse-39-python-{os.getpid()}-{self._testMethodName}"
        )
        remove_tree(self.work)
        self.work.mkdir(parents=True)
        self.checkout = self.work / "checkout"
        self._create_checkout()
        self.fake_python = self.work / "fake_git.py"
        self.fake_command = self.work / (
            "fake_git.cmd" if os.name == "nt" else "fake_git"
        )
        self.fake_python.write_text(
            """
import json
import os
import sys

args = sys.argv[1:]
record = os.environ.get("P39_FAKE_ARGS")
if record:
    with open(record, "a", encoding="utf-8", newline="\\n") as handle:
        handle.write(json.dumps(args, separators=(",", ":")) + "\\n")
if "--version" in args:
    print("git version 2.55.0.windows.3")
    raise SystemExit(0)
if os.environ.get("P39_FAKE_MODE") == "git-error":
    raise SystemExit(7)
paths = sys.stdin.buffer.read().split(b"\\0")[:-1]
mode = os.environ.get("P39_FAKE_MODE")
if mode == "malformed":
    sys.stdout.buffer.write(b"broken\\0text\\0")
    raise SystemExit(0)
out = bytearray()
for index, path in enumerate(paths):
    text = b"unspecified" if mode == "unspecified" and index == 0 else b"set"
    out.extend(path + b"\\0text\\0" + text + b"\\0")
    out.extend(path + b"\\0eol\\0lf\\0")
sys.stdout.buffer.write(out)
""".lstrip(),
            encoding="utf-8",
            newline="\n",
        )
        if os.name == "nt":
            self.fake_command.write_text(
                f'@echo off\r\n"{sys.executable}" "{self.fake_python}" %*\r\n',
                encoding="utf-8",
                newline="",
            )
        else:
            self.fake_command.write_text(
                "#!/usr/bin/env python3\n"
                "import os\n"
                "import sys\n"
                f"os.execv({str(sys.executable)!r}, "
                f"[{str(sys.executable)!r}, {str(self.fake_python)!r}, *sys.argv[1:]])\n",
                encoding="utf-8",
                newline="\n",
            )
            self.fake_command.chmod(
                self.fake_command.stat().st_mode | stat.S_IXUSR
            )

    def tearDown(self) -> None:
        remove_tree(self.work)

    def _create_checkout(self) -> None:
        for relative in verifier.EXPECTED_PATHS:
            path = self.checkout.joinpath(*relative.split("/"))
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"public\n")

    def _arguments(self, **overrides: str) -> list[str]:
        values = {
            "checkout_root": str(self.checkout),
            "pulse25_root": verifier.PULSE_25_ROOT,
            "pulse27_root": verifier.PULSE_27_ROOT,
            "git": str(self.fake_command),
        }
        values.update(overrides)
        return [
            "--checkout-root",
            values["checkout_root"],
            "--pulse25-root",
            values["pulse25_root"],
            "--pulse27-root",
            values["pulse27_root"],
            "--git",
            values["git"],
        ]

    def _invoke(self, **overrides: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(VERIFIER_PATH), *self._arguments(**overrides)],
            cwd=self.work,
            capture_output=True,
            text=True,
            check=False,
        )

    def test_parses_precisely_nul_framed_attributes(self) -> None:
        raw = b"".join(
            path.encode() + b"\0text\0set\0" + path.encode() + b"\0eol\0lf\0"
            for path in verifier.EXPECTED_PATHS
        )
        verifier.parse_check_attr_z(raw, verifier.EXPECTED_PATHS)

    def test_rejects_malformed_and_unspecified_attributes(self) -> None:
        with self.assertRaisesRegex(verifier.PublicFailure, "P39-ATTR-OUTPUT-MALFORMED"):
            verifier.parse_check_attr_z(b"only\0two\0", verifier.EXPECTED_PATHS)
        with self.assertRaisesRegex(verifier.PublicFailure, "P39-ATTR-UNSPECIFIED"):
            verifier.parse_check_attr_z(
                verifier.EXPECTED_PATHS[0].encode() + b"\0text\0unspecified\0",
                verifier.EXPECTED_PATHS,
            )

    def test_rejects_duplicate_missing_and_unexpected_attribute_records(self) -> None:
        path = verifier.EXPECTED_PATHS[0].encode()
        with self.assertRaisesRegex(verifier.PublicFailure, "P39-ATTR-DUPLICATE"):
            verifier.parse_check_attr_z(
                path + b"\0text\0set\0" + path + b"\0text\0set\0",
                verifier.EXPECTED_PATHS,
            )
        with self.assertRaisesRegex(verifier.PublicFailure, "P39-ATTR-MISSING"):
            verifier.parse_check_attr_z(path + b"\0text\0set\0", verifier.EXPECTED_PATHS)
        with self.assertRaisesRegex(verifier.PublicFailure, "P39-ATTR-UNEXPECTED-PATH"):
            verifier.parse_check_attr_z(
                b"unexpected\0text\0set\0",
                verifier.EXPECTED_PATHS,
            )

    def test_rejects_absolute_and_traversal_release_roots(self) -> None:
        for path in (
            "/absolute",
            r"C:\absolute",
            "../pulse-25-collector-source-release",
            f"{verifier.PULSE_25_ROOT}/../other",
        ):
            with self.assertRaisesRegex(verifier.PublicFailure, "P39-PATH-INVALID"):
                verifier.validate_release_root(path, verifier.PULSE_25_ROOT)

    def test_rejects_cardinality_mismatch(self) -> None:
        self.checkout.joinpath(*verifier.EXPECTED_PATHS[0].split("/")).unlink()
        result = self._invoke()
        self.assertEqual(result.returncode, 1)
        self.assertEqual(json.loads(result.stdout)["code"], "P39-CARDINALITY-MISMATCH")

    def test_rejects_cr_bytes(self) -> None:
        path = self.checkout.joinpath(*verifier.EXPECTED_PATHS[0].split("/"))
        path.write_bytes(b"public\r\n")
        result = self._invoke()
        self.assertEqual(result.returncode, 1)
        self.assertEqual(json.loads(result.stdout)["code"], "P39-CR-BYTES")

    def test_rejects_git_failure(self) -> None:
        previous = os.environ.get("P39_FAKE_MODE")
        os.environ["P39_FAKE_MODE"] = "git-error"
        try:
            result = self._invoke()
        finally:
            if previous is None:
                os.environ.pop("P39_FAKE_MODE", None)
            else:
                os.environ["P39_FAKE_MODE"] = previous
        self.assertEqual(result.returncode, 1)
        self.assertEqual(json.loads(result.stdout)["code"], "P39-GIT-ERROR")

    def test_cwd_independence_and_deterministic_public_output(self) -> None:
        captured = self.work / "arguments.json"
        previous = os.environ.get("P39_FAKE_ARGS")
        os.environ["P39_FAKE_ARGS"] = str(captured)
        try:
            first = self._invoke()
            second = self._invoke()
        finally:
            if previous is None:
                os.environ.pop("P39_FAKE_ARGS", None)
            else:
                os.environ["P39_FAKE_ARGS"] = previous
        self.assertEqual(first.returncode, 0, first.stderr)
        self.assertEqual(first.stdout, second.stdout)
        report = json.loads(first.stdout)
        self.assertEqual(report["status"], "pass")
        self.assertEqual(report["count"], 36)
        self.assertEqual(report["attribute_files"], 36)
        self.assertEqual(report["lf_files"], 36)
        self.assertEqual(report["zero_cr_files"], 36)
        self.assertEqual(report["files"], list(verifier.EXPECTED_PATHS))
        self.assertTrue(all(not Path(path).is_absolute() for path in report["files"]))
        calls = [
            json.loads(line)
            for line in captured.read_text(encoding="utf-8").splitlines()
        ]
        check_attr = [
            "-C",
            str(self.checkout),
            "check-attr",
            "-z",
            "--stdin",
            "text",
            "eol",
        ]
        version_probe = ["-C", str(self.checkout), "--version"]
        self.assertEqual(
            calls,
            [check_attr, version_probe, check_attr, version_probe],
        )

    def test_fake_git_reports_malformed_and_unspecified_output(self) -> None:
        for mode, code in (
            ("malformed", "P39-ATTR-OUTPUT-MALFORMED"),
            ("unspecified", "P39-ATTR-UNSPECIFIED"),
        ):
            previous = os.environ.get("P39_FAKE_MODE")
            os.environ["P39_FAKE_MODE"] = mode
            try:
                result = self._invoke()
            finally:
                if previous is None:
                    os.environ.pop("P39_FAKE_MODE", None)
                else:
                    os.environ["P39_FAKE_MODE"] = previous
            self.assertEqual(result.returncode, 1)
            self.assertEqual(json.loads(result.stdout)["code"], code)

    def test_real_cutoff_checkout_is_root_anchored_from_nested_cwd(self) -> None:
        clone = self.work / "cutoff"
        run(
            [
                "git",
                "clone",
                "--no-local",
                "--no-checkout",
                str(REPOSITORY_ROOT),
                str(clone),
            ]
        )
        run(["git", "-C", str(clone), "config", "core.autocrlf", "true"])
        run(
            [
                "git",
                "-C",
                str(clone),
                "checkout",
                "--force",
                "6807bd68aa01cbf0c819198765b7d6b5aa443328",
            ]
        )
        nested = clone / "docs" / "simulations"
        result = subprocess.run(
            [
                sys.executable,
                str(VERIFIER_PATH),
                "--checkout-root",
                str(clone),
                "--pulse25-root",
                verifier.PULSE_25_ROOT,
                "--pulse27-root",
                verifier.PULSE_27_ROOT,
            ],
            cwd=nested,
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertTrue(report["git_version"].startswith("git version "))
        self.assertLessEqual(len(report["git_version"]), 128)
        self.assertNotIn("\r", report["git_version"])
        self.assertNotIn("\n", report["git_version"])
        self.assertEqual(
            {
                "count": report["count"],
                "attribute_files": report["attribute_files"],
                "lf_files": report["lf_files"],
                "zero_cr_files": report["zero_cr_files"],
            },
            {"count": 36, "attribute_files": 36, "lf_files": 36, "zero_cr_files": 36},
        )


if __name__ == "__main__":
    unittest.main()
