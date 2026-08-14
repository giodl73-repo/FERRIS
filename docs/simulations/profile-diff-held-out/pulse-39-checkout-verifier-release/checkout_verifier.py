#!/usr/bin/env python3
"""Public checkout verifier with explicit Git-process accounting."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path, PurePosixPath, PureWindowsPath
from typing import Iterable


PULSE_25_ROOT = (
    "docs/simulations/profile-diff-held-out/pulse-25-collector-source-release"
)
PULSE_27_ROOT = (
    "docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release"
)
EXPECTED_CARDINALITY = 36

_PULSE_25_FILES = (
    "README.md",
    "bundle/durability.py",
    "bundle/qualification.py",
    "bundle/sealed_store.py",
    "bundle/synthetic_commands.py",
    "bundle/tests/test_durability.py",
    "bundle/tests/test_sealed_store.py",
    "bundle/ubuntu_worker.py",
    "bundle/verify_qualification.py",
    "bundle/verify_store.py",
    "public-manifest.json",
    "qualification-report.json",
    "release-receipt.json",
    "release-seal.json",
)
_PULSE_27_FILES = (
    "README.md",
    "adapter.py",
    "audit-report.json",
    "collector/durability.py",
    "collector/qualification.py",
    "collector/sealed_store.py",
    "collector/synthetic_commands.py",
    "collector/tests/test_durability.py",
    "collector/tests/test_sealed_store.py",
    "collector/ubuntu_worker.py",
    "collector/verify_qualification.py",
    "collector/verify_store.py",
    "fresh_verify.py",
    "legacy_verify.py",
    "public-manifest.json",
    "qualification-receipt.json",
    "qualify.py",
    "release-seal.json",
    "reproduce_cardinality_failure.py",
    "reproduction-receipt.json",
    "root-cause-report.json",
    "tests/test_adapter.py",
)
EXPECTED_PATHS = tuple(
    sorted(
        f"{release_root}/{path}"
        for release_root, paths in (
            (PULSE_25_ROOT, _PULSE_25_FILES),
            (PULSE_27_ROOT, _PULSE_27_FILES),
        )
        for path in paths
    )
)


class PublicFailure(Exception):
    """A deliberately non-sensitive failure category."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


class PublicArgumentParser(argparse.ArgumentParser):
    def error(self, message: str) -> None:
        del message
        raise PublicFailure("P39-PATH-INVALID")


def public_json(value: dict[str, object]) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def failure_json(code: str) -> str:
    return public_json({"code": code, "status": "fail"})


def validate_release_root(value: str, expected: str) -> str:
    """Accept exactly one canonical repository-relative release root."""

    if "\\" in value or value != expected:
        raise PublicFailure("P39-PATH-INVALID")
    posix = PurePosixPath(value)
    windows = PureWindowsPath(value)
    if (
        posix.is_absolute()
        or windows.is_absolute()
        or ".." in posix.parts
        or "." in posix.parts
        or posix.as_posix() != value
    ):
        raise PublicFailure("P39-PATH-INVALID")
    return value


def resolve_checkout_root(value: str) -> Path:
    root = Path(value).resolve()
    if not root.is_dir():
        raise PublicFailure("P39-ROOT-INVALID")
    return root


def is_within(path: Path, root: Path) -> bool:
    try:
        path.relative_to(root)
    except ValueError:
        return False
    return True


def enumerate_release_paths(checkout_root: Path, release_roots: Iterable[str]) -> dict[str, Path]:
    found: dict[str, Path] = {}
    for release_root in release_roots:
        release_path = checkout_root.joinpath(*PurePosixPath(release_root).parts)
        if not release_path.is_dir():
            raise PublicFailure("P39-PATH-MISSING")
        for directory, directories, files in os.walk(release_path, followlinks=False):
            directory_path = Path(directory)
            for name in directories:
                if (directory_path / name).is_symlink():
                    raise PublicFailure("P39-PATH-OUT-OF-ROOT")
            for name in files:
                path = directory_path / name
                if path.is_symlink() or not path.is_file():
                    raise PublicFailure("P39-PATH-OUT-OF-ROOT")
                resolved = path.resolve()
                if not is_within(resolved, checkout_root):
                    raise PublicFailure("P39-PATH-OUT-OF-ROOT")
                relative = resolved.relative_to(checkout_root).as_posix()
                if relative in found:
                    raise PublicFailure("P39-PATH-DUPLICATE")
                found[relative] = resolved

    if len(found) != EXPECTED_CARDINALITY:
        raise PublicFailure("P39-CARDINALITY-MISMATCH")
    expected = set(EXPECTED_PATHS)
    actual = set(found)
    missing = expected - actual
    unexpected = actual - expected
    if missing:
        raise PublicFailure("P39-PATH-MISSING")
    if unexpected:
        raise PublicFailure("P39-PATH-UNEXPECTED")
    return {path: found[path] for path in EXPECTED_PATHS}


def parse_check_attr_z(raw: bytes, expected_paths: Iterable[str]) -> None:
    """Require exactly one NUL-framed text/eol result for each expected path."""

    expected = tuple(expected_paths)
    if not raw or not raw.endswith(b"\0"):
        raise PublicFailure("P39-ATTR-OUTPUT-MALFORMED")
    fields = raw[:-1].split(b"\0")
    if len(fields) % 3:
        raise PublicFailure("P39-ATTR-OUTPUT-MALFORMED")

    values: dict[str, dict[str, str]] = {}
    for offset in range(0, len(fields), 3):
        try:
            path = fields[offset].decode("utf-8")
            attribute = fields[offset + 1].decode("ascii")
            value = fields[offset + 2].decode("ascii")
        except UnicodeDecodeError as error:
            raise PublicFailure("P39-ATTR-OUTPUT-MALFORMED") from error
        if path not in expected:
            raise PublicFailure("P39-ATTR-UNEXPECTED-PATH")
        if attribute not in {"text", "eol"}:
            raise PublicFailure("P39-ATTR-OUTPUT-MALFORMED")
        if value == "unspecified":
            raise PublicFailure("P39-ATTR-UNSPECIFIED")
        path_values = values.setdefault(path, {})
        if attribute in path_values:
            raise PublicFailure("P39-ATTR-DUPLICATE")
        path_values[attribute] = value

    for path in expected:
        path_values = values.get(path)
        if path_values is None or set(path_values) != {"text", "eol"}:
            raise PublicFailure("P39-ATTR-MISSING")
        if path_values["text"] != "set" or path_values["eol"] != "lf":
            raise PublicFailure("P39-ATTR-MISMATCH")


def checked_git(
    git: str, checkout_root: Path, arguments: list[str], stdin: bytes = b""
) -> subprocess.CompletedProcess[bytes]:
    try:
        return subprocess.run(
            [git, "-C", str(checkout_root), *arguments],
            input=stdin,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
            timeout=30,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        raise PublicFailure("P39-GIT-ERROR") from error


def git_version(git: str, checkout_root: Path) -> str:
    result = checked_git(git, checkout_root, ["--version"])
    if result.returncode != 0:
        raise PublicFailure("P39-GIT-VERSION-ERROR")
    try:
        version = result.stdout.decode("ascii").strip()
    except UnicodeDecodeError as error:
        raise PublicFailure("P39-GIT-VERSION-ERROR") from error
    if (
        not version.startswith("git version ")
        or len(version) > 128
        or any(character in version for character in "\r\n")
    ):
        raise PublicFailure("P39-GIT-VERSION-ERROR")
    return version


def verify(
    checkout_root_value: str,
    pulse_25_root: str,
    pulse_27_root: str,
    git: str = "git",
) -> dict[str, object]:
    checkout_root = resolve_checkout_root(checkout_root_value)
    roots = (
        validate_release_root(pulse_25_root, PULSE_25_ROOT),
        validate_release_root(pulse_27_root, PULSE_27_ROOT),
    )
    paths = enumerate_release_paths(checkout_root, roots)
    stdin = b"".join(path.encode("utf-8") + b"\0" for path in paths)
    result = checked_git(
        git, checkout_root, ["check-attr", "-z", "--stdin", "text", "eol"], stdin
    )
    if result.returncode != 0:
        raise PublicFailure("P39-GIT-ERROR")
    parse_check_attr_z(result.stdout, paths)
    for path in paths.values():
        if b"\r" in path.read_bytes():
            raise PublicFailure("P39-CR-BYTES")

    return {
        "attribute_files": EXPECTED_CARDINALITY,
        "count": EXPECTED_CARDINALITY,
        "files": list(paths),
        "git_version": git_version(git, checkout_root),
        "lf_files": EXPECTED_CARDINALITY,
        "status": "pass",
        "zero_cr_files": EXPECTED_CARDINALITY,
    }


def arguments(argv: list[str] | None) -> argparse.Namespace:
    parser = PublicArgumentParser(description=__doc__)
    parser.add_argument("--checkout-root", required=True)
    parser.add_argument("--pulse25-root", required=True)
    parser.add_argument("--pulse27-root", required=True)
    parser.add_argument("--git", default="git")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    try:
        parsed = arguments(argv)
        report = verify(
            parsed.checkout_root,
            parsed.pulse25_root,
            parsed.pulse27_root,
            parsed.git,
        )
    except PublicFailure as error:
        print(failure_json(error.code))
        return 1
    except Exception:
        print(failure_json("P39-INTERNAL-ERROR"))
        return 1
    print(public_json(report))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
