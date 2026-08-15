#!/usr/bin/env python3
"""Run Pulse 56 synthetic controls and real non-executing two-build probes."""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import uuid
import re
from pathlib import Path

from retained_build_custody import canonical_bytes, sha256_bytes


ROOT = Path(__file__).resolve().parent


def _run(command: list[str], *, env: dict[str, str] | None = None) -> bytes:
    completed = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, check=False)
    if completed.returncode != 0:
        raise RuntimeError(
            f"qualification command failed: {command!r}\n{completed.stderr.decode('utf-8', 'replace')}"
        )
    return completed.stdout


def synthetic_controls() -> dict[str, object]:
    windows = subprocess.run(
        [sys.executable, "-B", "-m", "unittest", "discover", "-s", "tests", "-v"],
        cwd=ROOT,
        env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
        capture_output=True,
        check=False,
    )
    if windows.returncode != 0:
        raise RuntimeError(windows.stderr.decode("utf-8", "replace"))
    wsl_tests = _wsl_path(ROOT / "tests")
    ubuntu = subprocess.run(
        [
            "wsl.exe",
            "-d",
            "Ubuntu-24.04",
            "--exec",
            "/usr/bin/env",
            "-i",
            "HOME=/root",
            "PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
            "PYTHONDONTWRITEBYTECODE=1",
            "python3",
            "-B",
            "-m",
            "unittest",
            "discover",
            "-s",
            wsl_tests,
            "-v",
        ],
        cwd=ROOT,
        capture_output=True,
        check=False,
    )
    if ubuntu.returncode != 0:
        raise RuntimeError(ubuntu.stderr.decode("utf-8", "replace"))
    def result(output: bytes, name: str) -> dict[str, int]:
        text = output.decode("utf-8", "replace")
        match = re.search(r"Ran (\d+) tests?.*?(?:OK(?: \(skipped=(\d+)\))?)", text, re.DOTALL)
        if match is None:
            raise RuntimeError(f"{name} test count was not parseable")
        skipped = int(match.group(2) or "0")
        return {"passed": int(match.group(1)) - skipped, "skipped": skipped}

    output = windows.stdout + windows.stderr + ubuntu.stdout + ubuntu.stderr
    return {
        "negative_controls": [
            "forged-or-recomputed-public-receipt-root",
            "copied-or-forged-live-handle",
            "receipt-artifact-disagreement",
            "extra-file",
            "binary-replacement",
            "fresh-root-and-one-rename",
            "single-descriptor-receipt-hash-and-parse",
            "lazy-argument-iterable-before-side-effects",
            "controlled-child-environment",
            "atomic-handle-use-accounting-and-exhaustion",
            "early-close-exact-owned-root-cleanup",
            "active-close-refusal",
            "concurrent-last-use-cleanup-exactly-once",
            "fatal-cleanup-failure-is-not-completed-process",
            "substituted-runtime-root-refusal",
            "windows-os-handle-single-ownership",
            "profile-diff-shaped-arguments-and-byte-capture",
            "native-linux-fd-inode-launch-after-path-mutation",
            "production-api-no-root-callback-or-launch-parent",
        ],
        "result_sha256": sha256_bytes(output),
        "native_wsl_launch_path_qualified": True,
        "ubuntu_24_04_wsl": result(ubuntu.stdout + ubuntu.stderr, "Ubuntu"),
        "windows": result(windows.stdout + windows.stderr, "Windows"),
    }


def _windows_probe(scratch: Path) -> dict[str, object]:
    scratch.mkdir()
    output = _run(
        [
            sys.executable,
            "-B",
            "retained_build_custody.py",
            "--platform",
            "windows-x86_64",
            "--runtime-parent",
            os.fspath(scratch),
            "--include-receipt",
        ],
        env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
    )
    return json.loads(output)


def _wsl_path(path: Path) -> str:
    completed = subprocess.run(
        ["wsl.exe", "-d", "Ubuntu-24.04", "--exec", "wslpath", "-a", os.fspath(path)],
        capture_output=True,
        check=False,
    )
    if completed.returncode != 0:
        raise RuntimeError("Ubuntu-24.04 WSL path conversion failed")
    return completed.stdout.decode("utf-8").strip()


def _ubuntu_probe() -> dict[str, object]:
    script = _wsl_path(ROOT / "retained_build_custody.py")
    runtime = f"/root/.ferris-p56-qualification-{uuid.uuid4().hex}"
    created = subprocess.run(
        ["wsl.exe", "-d", "Ubuntu-24.04", "--exec", "/bin/mkdir", "-m", "700", runtime],
        capture_output=True,
        check=False,
    )
    if created.returncode != 0:
        raise RuntimeError("Ubuntu-24.04 qualification root creation failed")
    command = [
        "wsl.exe",
        "-d",
        "Ubuntu-24.04",
        "--exec",
        "/usr/bin/env",
        "-i",
        "HOME=/root",
        "PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
        "CARGO_HOME=/root/.cargo",
        "RUSTUP_HOME=/root/.rustup",
        "PYTHONDONTWRITEBYTECODE=1",
        "python3",
        "-B",
        script,
        "--platform",
        "ubuntu-24.04-x86_64",
        "--runtime-parent",
        runtime,
        "--include-receipt",
    ]
    try:
        completed = subprocess.run(command, capture_output=True, check=False)
        if completed.returncode != 0:
            raise RuntimeError(
                completed.stdout.decode("utf-8", "replace")
                + completed.stderr.decode("utf-8", "replace")
            )
        return json.loads(completed.stdout)
    finally:
        subprocess.run(
            [
                "wsl.exe",
                "-d",
                "Ubuntu-24.04",
                "--exec",
                "/bin/rm",
                "-rf",
                runtime,
            ],
            capture_output=True,
            check=False,
        )


def actual_probes() -> dict[str, object]:
    if os.name != "nt":
        raise RuntimeError("cross-platform qualification must be initiated from Windows")
    scratch = ROOT.parents[4] / f".p56-qualification-{uuid.uuid4().hex}"
    try:
        windows = _windows_probe(scratch)
        ubuntu = _ubuntu_probe()
    finally:
        shutil.rmtree(scratch, ignore_errors=True)
    for expected, result in (
        ("windows-x86_64", windows),
        ("ubuntu-24.04-x86_64", ubuntu),
    ):
        summary = result.get("summary")
        receipt = result.get("qualification_receipt")
        if not isinstance(summary, dict) or not isinstance(receipt, dict):
            raise RuntimeError(f"{expected} did not return a retained receipt")
        if summary["custody"]["rename_attempts"] != 1 or summary["custody"]["retries"] != 0:
            raise RuntimeError(f"{expected} custody was not one-rename/zero-retry")
        if receipt["payload"]["platform"] != expected:
            raise RuntimeError(f"{expected} platform binding mismatch")
    return {
        "ubuntu_24_04_wsl": ubuntu,
        "windows": windows,
    }


def envelope(payload: dict[str, object]) -> dict[str, object]:
    identity = sha256_bytes(canonical_bytes(payload))
    return {
        "payload": payload,
        "payload_sha256": identity,
        "receipt_id": identity,
        "schema": "ferris.pulse-56-retained-build-custody-qualification-envelope/v1",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--actual", action="store_true")
    parser.add_argument("--write-receipt", action="store_true")
    parsed = parser.parse_args()
    synthetic = synthetic_controls()
    payload: dict[str, object] = {
        "actual_probes": actual_probes() if parsed.actual else "not-run",
        "ferris_executed": False,
        "outcome": "pass",
        "schema": "ferris.pulse-56-retained-build-custody-qualification/v1",
        "synthetic": synthetic,
    }
    result = envelope(payload)
    if parsed.write_receipt:
        if not parsed.actual:
            raise RuntimeError("--write-receipt requires --actual")
        (ROOT / "qualification-receipt.json").write_bytes(canonical_bytes(result) + b"\n")
    print(canonical_bytes(result).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
