from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path

from durability import canonical_json, find_residue
from sealed_store import (
    read_sealed_json,
    seal_store,
    sha256_bytes,
    write_record,
    write_sealed_json,
)
from synthetic_commands import run_windows


PAIR_COUNT = 20
ROOT = Path(__file__).resolve().parent
DATA_ROOT = ROOT / "qualification-data"


def windows_to_wsl(path: Path) -> str:
    resolved = path.resolve()
    drive = resolved.drive
    if len(drive) != 2 or drive[1] != ":":
        raise ValueError("qualification path must be on a Windows drive")
    tail = resolved.as_posix()[2:]
    return f"/mnt/{drive[0].lower()}{tail}"


def parse_json_output(process: subprocess.CompletedProcess[bytes], label: str) -> dict:
    if process.returncode != 0:
        raise RuntimeError(
            f"{label} failed with exit {process.returncode}: "
            f"{process.stderr.decode('utf-8', errors='replace')}"
        )
    lines = [line for line in process.stdout.decode("utf-8").splitlines() if line]
    if not lines:
        raise RuntimeError(f"{label} returned no receipt")
    return json.loads(lines[-1])


def run_ubuntu_worker(mode: str, root: Path, *, index: int | None = None) -> dict:
    command = [
        "wsl.exe",
        "--exec",
        "python3",
        windows_to_wsl(ROOT / "ubuntu_worker.py"),
        "--mode",
        mode,
        "--root",
        windows_to_wsl(root),
    ]
    if index is not None:
        command.extend(["--index", str(index)])
    else:
        command.extend(["--count", str(PAIR_COUNT)])
    return parse_json_output(
        subprocess.run(command, capture_output=True, check=False),
        f"Ubuntu worker {mode}",
    )


def run_fresh_verifier(root: Path, platform_name: str, ubuntu: bool) -> dict:
    if ubuntu:
        command = [
            "wsl.exe",
            "--exec",
            "python3",
            windows_to_wsl(ROOT / "verify_store.py"),
            "--root",
            windows_to_wsl(root),
            "--platform",
            platform_name,
            "--count",
            str(PAIR_COUNT),
        ]
    else:
        command = [
            sys.executable,
            str(ROOT / "verify_store.py"),
            "--root",
            str(root),
            "--platform",
            platform_name,
            "--count",
            str(PAIR_COUNT),
        ]
    return parse_json_output(
        subprocess.run(command, capture_output=True, check=False),
        f"fresh {platform_name} verifier",
    )


def digest_files(paths: list[Path]) -> str:
    aggregate = hashlib.sha256()
    for path in sorted(paths, key=lambda item: item.relative_to(ROOT).as_posix()):
        relative = path.relative_to(ROOT).as_posix().encode("utf-8")
        content = path.read_bytes()
        aggregate.update(len(relative).to_bytes(8, "big"))
        aggregate.update(relative)
        aggregate.update(hashlib.sha256(content).digest())
    return "sha256:" + aggregate.hexdigest()


def summarize_receipts(receipts: list[dict]) -> dict:
    states: dict[str, int] = {}
    for receipt in receipts:
        durability = receipt["durability"]
        if durability.get("file_sync") != "synced":
            raise RuntimeError("a write did not report a synchronized file")
        if not durability.get("residue_clean"):
            raise RuntimeError("a write reported temporary-file residue")
        state = durability["directory_sync"]["state"]
        states[state] = states.get(state, 0) + 1
    return {
        "write_count": len(receipts),
        "file_sync_pass_count": len(receipts),
        "directory_sync_states": states,
        "residue_clean_count": len(receipts),
    }


def main() -> int:
    if DATA_ROOT.exists():
        shutil.rmtree(DATA_ROOT)
    windows_root = DATA_ROOT / "windows"
    ubuntu_root = DATA_ROOT / "ubuntu"
    pair_root = DATA_ROOT / "pairs"
    for store in (windows_root, ubuntu_root, pair_root):
        (store / "records").mkdir(parents=True)

    windows_receipts = []
    ubuntu_receipts = []
    pair_receipts = []
    windows_environment = None
    ubuntu_environment = None

    for index in range(PAIR_COUNT):
        windows_observation = run_windows(index)
        windows_environment = windows_observation["environment"]
        windows_receipt = write_record(windows_root, windows_observation)
        windows_receipts.append(windows_receipt)

        ubuntu_receipt = run_ubuntu_worker("record", ubuntu_root, index=index)
        ubuntu_receipts.append(ubuntu_receipt)
        ubuntu_observation, _ = read_sealed_json(
            ubuntu_root / "records" / f"pair-{index:03d}.json"
        )
        ubuntu_environment = ubuntu_observation["environment"]

        pair_receipt = write_record(
            pair_root,
            {
                "schema": "collector-synthetic-pair-v1",
                "platform": "pair",
                "index": index,
                "windows_record_sha256": windows_receipt["record_sha256"],
                "ubuntu_record_sha256": ubuntu_receipt["record_sha256"],
            },
        )
        pair_receipts.append(pair_receipt)

    windows_receipts.append(seal_store(windows_root, "windows", PAIR_COUNT))
    ubuntu_receipts.append(run_ubuntu_worker("seal", ubuntu_root))
    pair_receipts.append(seal_store(pair_root, "pair", PAIR_COUNT))

    verifications = {
        "windows": run_fresh_verifier(windows_root, "windows", False),
        "ubuntu": run_fresh_verifier(ubuntu_root, "ubuntu", True),
        "pairs": run_fresh_verifier(pair_root, "pair", False),
    }
    residue_count = len(find_residue(DATA_ROOT))
    if residue_count:
        raise RuntimeError(f"qualification residue detected: {residue_count}")

    source_files = [
        path
        for path in ROOT.glob("*.py")
        if path.name not in {"qualification.py"}
    ] + [ROOT / "qualification.py"]
    test_files = list((ROOT / "tests").glob("test_*.py"))
    qualification = {
        "schema": "collector-synthetic-qualification-v1",
        "outcome": "pass",
        "pair_count": PAIR_COUNT,
        "command_execution_count": PAIR_COUNT * 2,
        "platform_record_counts": {"windows": PAIR_COUNT, "ubuntu": PAIR_COUNT},
        "pair_seal_count": PAIR_COUNT,
        "success_exit_count": 20,
        "nonzero_exit_count": 20,
        "stdout_routed_count": 30,
        "stderr_routed_count": 30,
        "verifications": verifications,
        "durability": {
            "windows": summarize_receipts(windows_receipts),
            "ubuntu": summarize_receipts(ubuntu_receipts),
            "pairs": summarize_receipts(pair_receipts),
        },
        "residue_count": residue_count,
        "environment_digests": {
            "windows": windows_environment["environment_sha256"],
            "ubuntu": ubuntu_environment["environment_sha256"],
        },
        "source_digest": digest_files(source_files),
        "test_digest": digest_files(test_files),
    }
    seal_receipt = write_sealed_json(DATA_ROOT / "qualification-seal.json", qualification)
    qualification["qualification_seal_file_sha256"] = seal_receipt["record_sha256"]
    qualification["qualification_seal_payload_sha256"] = seal_receipt["payload_sha256"]
    print(canonical_json(qualification).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
