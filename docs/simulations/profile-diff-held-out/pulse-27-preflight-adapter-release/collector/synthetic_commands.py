from __future__ import annotations

import hashlib
import platform
import subprocess
import sys

from durability import canonical_json


def environment_record(shell_name: str) -> dict:
    value = {
        "system": platform.system(),
        "release": platform.release(),
        "version": platform.version(),
        "machine": platform.machine(),
        "python_implementation": platform.python_implementation(),
        "python_version": platform.python_version(),
        "shell": shell_name,
    }
    value["environment_sha256"] = (
        "sha256:" + hashlib.sha256(canonical_json(value)).hexdigest()
    )
    return value


def expected_observation(platform_name: str, index: int) -> dict:
    route = index % 4
    prefix = "WINDOWS" if platform_name == "windows" else "UBUNTU"
    stdout = ""
    stderr = ""
    exit_code = 0
    if route == 0:
        stdout = f"{prefix}-PAIR-{index:03d}-STDOUT"
    elif route == 1:
        stderr = f"{prefix}-PAIR-{index:03d}-STDERR"
        exit_code = 3
    elif route == 2:
        stdout = f"{prefix}-PAIR-{index:03d}-STDOUT"
        stderr = f"{prefix}-PAIR-{index:03d}-STDERR"
    else:
        stdout = f"{prefix}-PAIR-{index:03d}-STDOUT"
        stderr = f"{prefix}-PAIR-{index:03d}-STDERR"
        exit_code = 7
    return {
        "stdout": stdout,
        "stderr": stderr,
        "expected_exit": exit_code,
        "route": ["stdout", "stderr", "both-success", "both-nonzero"][route],
    }


def _powershell_literal(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def run_windows(index: int) -> dict:
    expected = expected_observation("windows", index)
    script = (
        f"[Console]::Out.Write({_powershell_literal(expected['stdout'])});"
        f"[Console]::Error.Write({_powershell_literal(expected['stderr'])});"
        f"exit {expected['expected_exit']}"
    )
    process = subprocess.run(
        [
            "powershell.exe",
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            script,
        ],
        capture_output=True,
        check=False,
    )
    stdout = process.stdout.decode("ascii")
    stderr = process.stderr.decode("ascii")
    if (
        stdout != expected["stdout"]
        or stderr != expected["stderr"]
        or process.returncode != expected["expected_exit"]
    ):
        raise RuntimeError(f"Windows synthetic command mismatch at pair {index}")
    return {
        "schema": "collector-synthetic-observation-v1",
        "platform": "windows",
        "index": index,
        "route": expected["route"],
        "expected_exit": expected["expected_exit"],
        "exit_code": process.returncode,
        "stdout": stdout,
        "stderr": stderr,
        "environment": environment_record("powershell"),
    }


def run_ubuntu(index: int) -> dict:
    expected = expected_observation("ubuntu", index)
    script = "printf '%s' \"$1\"; printf '%s' \"$2\" >&2; exit \"$3\""
    process = subprocess.run(
        [
            "/bin/sh",
            "-c",
            script,
            "collector-synthetic",
            expected["stdout"],
            expected["stderr"],
            str(expected["expected_exit"]),
        ],
        capture_output=True,
        check=False,
    )
    stdout = process.stdout.decode("ascii")
    stderr = process.stderr.decode("ascii")
    if (
        stdout != expected["stdout"]
        or stderr != expected["stderr"]
        or process.returncode != expected["expected_exit"]
    ):
        raise RuntimeError(f"Ubuntu synthetic command mismatch at pair {index}")
    return {
        "schema": "collector-synthetic-observation-v1",
        "platform": "ubuntu",
        "index": index,
        "route": expected["route"],
        "expected_exit": expected["expected_exit"],
        "exit_code": process.returncode,
        "stdout": stdout,
        "stderr": stderr,
        "environment": environment_record("/bin/sh"),
    }
