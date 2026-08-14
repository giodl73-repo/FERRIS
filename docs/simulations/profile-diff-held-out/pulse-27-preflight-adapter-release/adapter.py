from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent
COLLECTOR_ROOT = ROOT / "collector"
if str(COLLECTOR_ROOT) not in sys.path:
    sys.path.insert(0, str(COLLECTOR_ROOT))

from durability import canonical_json, find_residue  # noqa: E402
from sealed_store import (  # noqa: E402
    read_sealed_json,
    sha256_bytes,
    tree_state,
    write_record,
)
from synthetic_commands import expected_observation, run_windows  # noqa: E402


PAIR_COUNT = 2
PAIR_IDS = tuple(f"preflight-pair-{index:03d}" for index in range(PAIR_COUNT))
RECORD_NAMES = tuple(f"pair-{index:03d}.json" for index in range(PAIR_COUNT))


def windows_to_wsl(path: Path) -> str:
    resolved = path.resolve()
    drive = resolved.drive
    if len(drive) != 2 or drive[1] != ":":
        raise ValueError("adapter path must be on a Windows drive")
    return f"/mnt/{drive[0].lower()}{resolved.as_posix()[2:]}"


def parse_json_output(
    process: subprocess.CompletedProcess[bytes], label: str
) -> dict:
    if process.returncode != 0:
        detail = process.stderr.decode("utf-8", errors="replace").strip()
        raise RuntimeError(f"{label} failed with exit {process.returncode}: {detail}")
    lines = [line for line in process.stdout.decode("utf-8").splitlines() if line]
    if not lines:
        raise RuntimeError(f"{label} returned no receipt")
    return json.loads(lines[-1])


def require_durable(receipt: dict, label: str) -> None:
    durability = receipt.get("durability")
    if not isinstance(durability, dict):
        raise RuntimeError(f"{label} returned no durability receipt")
    if durability.get("file_sync") != "synced":
        raise RuntimeError(f"{label} did not synchronize the file")
    directory = durability.get("directory_sync")
    if not isinstance(directory, dict) or directory.get("state") != "synced":
        raise RuntimeError(f"{label} did not synchronize the containing directory")
    if durability.get("residue_clean") is not True:
        raise RuntimeError(f"{label} reported atomic-write residue")


def run_ubuntu_record(store_root: Path, index: int) -> dict:
    command = [
        "wsl.exe",
        "--exec",
        "python3",
        windows_to_wsl(COLLECTOR_ROOT / "ubuntu_worker.py"),
        "--mode",
        "record",
        "--root",
        windows_to_wsl(store_root),
        "--index",
        str(index),
    ]
    return parse_json_output(
        subprocess.run(command, capture_output=True, check=False),
        f"Ubuntu record {index}",
    )


def run_fresh_verifier(cycle_root: Path, verifier_platform: str) -> dict:
    if verifier_platform == "windows":
        command = [
            sys.executable,
            str(ROOT / "fresh_verify.py"),
            "--root",
            str(cycle_root),
            "--verifier-platform",
            "windows",
        ]
    elif verifier_platform == "ubuntu":
        command = [
            "wsl.exe",
            "--exec",
            "python3",
            windows_to_wsl(ROOT / "fresh_verify.py"),
            "--root",
            windows_to_wsl(cycle_root),
            "--verifier-platform",
            "ubuntu",
        ]
    else:
        raise ValueError(f"unsupported verifier platform: {verifier_platform}")
    return parse_json_output(
        subprocess.run(command, capture_output=True, check=False),
        f"fresh {verifier_platform} verifier",
    )


def _environment_digest_is_valid(environment: object) -> bool:
    if not isinstance(environment, dict):
        return False
    claimed = environment.get("environment_sha256")
    if not isinstance(claimed, str):
        return False
    unsigned = dict(environment)
    del unsigned["environment_sha256"]
    return claimed == sha256_bytes(canonical_json(unsigned))


def _load_process_record(
    path: Path, platform_name: str, index: int
) -> tuple[dict, str]:
    payload, digest = read_sealed_json(path)
    if payload.get("schema") != "collector-synthetic-observation-v1":
        raise ValueError("process record schema mismatch")
    if payload.get("platform") != platform_name or payload.get("index") != index:
        raise ValueError("process record identity or duplicate-index mismatch")
    expected = expected_observation(platform_name, index)
    for key in ("route", "expected_exit", "stdout", "stderr"):
        if payload.get(key) != expected[key]:
            raise ValueError(f"process record {key} mismatch")
    if payload.get("exit_code") != expected["expected_exit"]:
        raise ValueError("process record exit mismatch")
    if not _environment_digest_is_valid(payload.get("environment")):
        raise ValueError("process environment digest mismatch")
    return payload, digest


def _exact_file_set(cycle_root: Path) -> list[str]:
    return sorted(
        path.relative_to(cycle_root).as_posix()
        for path in cycle_root.rglob("*")
        if path.is_file()
    )


def _load_exact_two(cycle_root: Path) -> dict:
    expected_files = sorted(
        f"{store}/records/{name}"
        for store in ("windows", "ubuntu", "pairs")
        for name in RECORD_NAMES
    )
    if _exact_file_set(cycle_root) != expected_files:
        raise ValueError("exact-two file cardinality or naming mismatch")

    process_digests: dict[tuple[str, int], str] = {}
    environment_digests: dict[str, str] = {}
    for platform_name in ("windows", "ubuntu"):
        seen_indexes = []
        for index, name in enumerate(RECORD_NAMES):
            payload, digest = _load_process_record(
                cycle_root / platform_name / "records" / name,
                platform_name,
                index,
            )
            seen_indexes.append(payload["index"])
            process_digests[(platform_name, index)] = digest
            environment_digests[platform_name] = payload["environment"][
                "environment_sha256"
            ]
        if seen_indexes != [0, 1] or len(set(seen_indexes)) != PAIR_COUNT:
            raise ValueError("process record duplicate or missing index")

    pair_ids = []
    for index, name in enumerate(RECORD_NAMES):
        payload, _ = read_sealed_json(cycle_root / "pairs" / "records" / name)
        expected_pair_id = PAIR_IDS[index]
        if (
            payload.get("schema") != "exact-two-preflight-pair-seal-v1"
            or payload.get("platform") != "pair"
            or payload.get("index") != index
            or payload.get("pair_id") != expected_pair_id
        ):
            raise ValueError("pair seal identity mismatch")
        if payload.get("windows_record_sha256") != process_digests[
            ("windows", index)
        ]:
            raise ValueError("pair seal Windows join mismatch")
        if payload.get("ubuntu_record_sha256") != process_digests[
            ("ubuntu", index)
        ]:
            raise ValueError("pair seal Ubuntu join mismatch")
        pair_ids.append(payload["pair_id"])
    if pair_ids != list(PAIR_IDS) or len(set(pair_ids)) != PAIR_COUNT:
        raise ValueError("pair seal duplicate or missing pair ID")

    return {
        "pair_ids": pair_ids,
        "pair_count": PAIR_COUNT,
        "windows_record_count": PAIR_COUNT,
        "ubuntu_record_count": PAIR_COUNT,
        "process_record_count": PAIR_COUNT * 2,
        "pair_seal_count": PAIR_COUNT,
        "environment_digests": environment_digests,
    }


def verify_exact_two(cycle_root: Path) -> dict:
    root = Path(cycle_root)
    if find_residue(root):
        raise ValueError("atomic-write residue detected")
    before = tree_state(root)
    first = _load_exact_two(root)
    middle = tree_state(root)
    second = _load_exact_two(root)
    after = tree_state(root)
    if first != second:
        raise ValueError("idempotent reload mismatch")
    if before != middle or middle != after:
        raise ValueError("read-only reload changed the store")
    result = dict(first)
    result.update(
        {
            "residue_count": 0,
            "idempotent_read_only": True,
            "file_count": len(before),
        }
    )
    return result


def run_exact_two_cycle(cycle_root: Path) -> dict:
    root = Path(cycle_root)
    if root.exists():
        raise FileExistsError(f"cycle store already exists: {root.name}")
    for store_name in ("windows", "ubuntu", "pairs"):
        (root / store_name / "records").mkdir(parents=True)

    write_receipts = []
    for index in range(PAIR_COUNT):
        windows_receipt = write_record(
            root / "windows", run_windows(index)
        )
        require_durable(windows_receipt, f"Windows record {index}")
        write_receipts.append(windows_receipt)

        ubuntu_receipt = run_ubuntu_record(root / "ubuntu", index)
        require_durable(ubuntu_receipt, f"Ubuntu record {index}")
        write_receipts.append(ubuntu_receipt)

        pair_receipt = write_record(
            root / "pairs",
            {
                "schema": "exact-two-preflight-pair-seal-v1",
                "platform": "pair",
                "index": index,
                "pair_id": PAIR_IDS[index],
                "windows_record_sha256": windows_receipt["record_sha256"],
                "ubuntu_record_sha256": ubuntu_receipt["record_sha256"],
            },
        )
        require_durable(pair_receipt, f"pair seal {index}")
        write_receipts.append(pair_receipt)

    windows_verification = run_fresh_verifier(root, "windows")
    ubuntu_verification = run_fresh_verifier(root, "ubuntu")
    if windows_verification != ubuntu_verification:
        comparable_windows = dict(windows_verification)
        comparable_ubuntu = dict(ubuntu_verification)
        comparable_windows.pop("verifier_platform", None)
        comparable_ubuntu.pop("verifier_platform", None)
        if comparable_windows != comparable_ubuntu:
            raise RuntimeError("cross-platform fresh verifier mismatch")
    if find_residue(root):
        raise RuntimeError("cycle left atomic-write residue")

    return {
        "schema": "exact-two-preflight-cycle-v1",
        "outcome": "pass",
        "pair_ids": list(PAIR_IDS),
        "pair_count": PAIR_COUNT,
        "windows_record_count": PAIR_COUNT,
        "ubuntu_record_count": PAIR_COUNT,
        "process_record_count": PAIR_COUNT * 2,
        "pair_seal_count": PAIR_COUNT,
        "durable_write_count": len(write_receipts),
        "fresh_process_reload_count": 2,
        "fresh_verifiers": {
            "windows": windows_verification,
            "ubuntu": ubuntu_verification,
        },
        "residue_count": 0,
        "retries": 0,
    }


def cycle_aggregate(cycle_root: Path) -> str:
    aggregate = hashlib.sha256()
    for path in sorted(
        (item for item in cycle_root.rglob("*") if item.is_file()),
        key=lambda item: item.relative_to(cycle_root).as_posix(),
    ):
        relative = path.relative_to(cycle_root).as_posix().encode("utf-8")
        data = path.read_bytes()
        aggregate.update(len(relative).to_bytes(8, "big"))
        aggregate.update(relative)
        aggregate.update(hashlib.sha256(data).digest())
    return "sha256:" + aggregate.hexdigest()


def remove_store(path: Path) -> None:
    shutil.rmtree(path)
    if path.exists():
        raise RuntimeError(f"failed to remove disposable store: {path.name}")
