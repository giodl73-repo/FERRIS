from __future__ import annotations

import hashlib
import json
from pathlib import Path

from durability import atomic_write_bytes, canonical_json, find_residue


SEALED_SCHEMA = "collector-sealed-json-v1"


def sha256_bytes(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def sealed_document(payload: dict) -> dict:
    return {
        "schema": SEALED_SCHEMA,
        "payload": payload,
        "payload_sha256": sha256_bytes(canonical_json(payload)),
    }


def write_sealed_json(path: Path, payload: dict) -> dict:
    document = sealed_document(payload)
    encoded = canonical_json(document) + b"\n"
    receipt = atomic_write_bytes(path, encoded)
    return {
        "record_sha256": sha256_bytes(encoded),
        "payload_sha256": document["payload_sha256"],
        "durability": receipt.to_dict(),
    }


def read_sealed_json(path: Path) -> tuple[dict, str]:
    encoded = path.read_bytes()
    document = json.loads(encoded)
    if document.get("schema") != SEALED_SCHEMA:
        raise ValueError(f"unsupported sealed document schema: {path.name}")
    payload = document.get("payload")
    if not isinstance(payload, dict):
        raise ValueError(f"sealed payload is not an object: {path.name}")
    expected = sha256_bytes(canonical_json(payload))
    if document.get("payload_sha256") != expected:
        raise ValueError(f"sealed payload digest mismatch: {path.name}")
    return payload, sha256_bytes(encoded)


def record_name(index: int) -> str:
    return f"pair-{index:03d}.json"


def write_record(store_root: Path, payload: dict) -> dict:
    index = payload["index"]
    return write_sealed_json(store_root / "records" / record_name(index), payload)


def seal_store(store_root: Path, platform_name: str, count: int) -> dict:
    records = []
    for index in range(count):
        payload, digest = read_sealed_json(
            store_root / "records" / record_name(index)
        )
        if payload.get("index") != index:
            raise ValueError("record index mismatch")
        if payload.get("platform") != platform_name:
            raise ValueError("record platform mismatch")
        records.append({"index": index, "record_sha256": digest})
    manifest = {
        "schema": "collector-store-manifest-v1",
        "platform": platform_name,
        "record_count": count,
        "records": records,
    }
    return write_sealed_json(store_root / "manifest.json", manifest)


def tree_state(root: Path) -> list[dict]:
    state = []
    for path in sorted(
        (item for item in root.rglob("*") if item.is_file()),
        key=lambda item: item.relative_to(root).as_posix(),
    ):
        if path.name == "closed-workspace-baseline.json":
            continue
        stat_result = path.stat()
        state.append(
            {
                "path": path.relative_to(root).as_posix(),
                "size": stat_result.st_size,
                "mtime_ns": stat_result.st_mtime_ns,
                "sha256": sha256_bytes(path.read_bytes()),
            }
        )
    return state


def verify_store(store_root: Path, platform_name: str, count: int) -> dict:
    before = tree_state(store_root)
    residue = find_residue(store_root)
    if residue:
        raise ValueError(f"atomic-write residue detected: {len(residue)}")

    record_directory = store_root / "records"
    actual_names = sorted(path.name for path in record_directory.glob("*.json"))
    expected_names = [record_name(index) for index in range(count)]
    if actual_names != expected_names:
        raise ValueError("record cardinality or naming mismatch")

    manifest, _ = read_sealed_json(store_root / "manifest.json")
    if manifest.get("platform") != platform_name:
        raise ValueError("manifest platform mismatch")
    if manifest.get("record_count") != count:
        raise ValueError("manifest cardinality mismatch")
    entries = manifest.get("records")
    if not isinstance(entries, list) or len(entries) != count:
        raise ValueError("manifest record list mismatch")

    success = 0
    nonzero = 0
    stdout_records = 0
    stderr_records = 0
    for index, entry in enumerate(entries):
        payload, digest = read_sealed_json(record_directory / record_name(index))
        if payload.get("index") != index or payload.get("platform") != platform_name:
            raise ValueError("record identity mismatch")
        if entry != {"index": index, "record_sha256": digest}:
            raise ValueError("manifest record digest mismatch")
        if platform_name in {"windows", "ubuntu"}:
            expected_exit = payload.get("expected_exit")
            if payload.get("exit_code") != expected_exit:
                raise ValueError("command exit mismatch")
            if expected_exit == 0:
                success += 1
            else:
                nonzero += 1
            if payload.get("stdout"):
                stdout_records += 1
            if payload.get("stderr"):
                stderr_records += 1

    after_first = tree_state(store_root)
    verify_again = [
        read_sealed_json(record_directory / record_name(index))[1]
        for index in range(count)
    ]
    if len(verify_again) != count:
        raise AssertionError("second verification did not reload every record")
    after_second = tree_state(store_root)
    if before != after_first or after_first != after_second:
        raise ValueError("verification changed the sealed store")

    return {
        "platform": platform_name,
        "record_count": count,
        "success_count": success,
        "nonzero_count": nonzero,
        "stdout_record_count": stdout_records,
        "stderr_record_count": stderr_records,
        "residue_count": 0,
        "idempotent_read_only": True,
    }
