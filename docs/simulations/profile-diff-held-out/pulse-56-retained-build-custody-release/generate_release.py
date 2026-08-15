#!/usr/bin/env python3
"""Recompute the Pulse 56 manifest and seal from the complete release tree."""

from __future__ import annotations

import hashlib
from pathlib import Path

from retained_build_custody import canonical_bytes, sha256_bytes


ROOT = Path(__file__).resolve().parent
EXCLUDED = {"public-manifest.json", "release-seal.json"}


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(65_536), b""):
            digest.update(chunk)
    return f"sha256:{digest.hexdigest()}"


def public_files() -> list[Path]:
    return sorted(
        (
            path
            for path in ROOT.rglob("*")
            if path.is_file()
            and path.name not in EXCLUDED
            and "__pycache__" not in path.parts
            and not any(part.startswith(".test-work-") for part in path.parts)
            and not any(part.startswith(".qualification-work-") for part in path.parts)
        ),
        key=lambda path: path.relative_to(ROOT).as_posix(),
    )


def aggregate(entries: list[dict[str, object]]) -> str:
    digest = hashlib.sha256()
    for entry in entries:
        path = str(entry["path"]).encode("utf-8")
        digest.update(len(path).to_bytes(8, "big"))
        digest.update(path)
        digest.update(bytes.fromhex(str(entry["sha256"]).removeprefix("sha256:")))
    return f"sha256:{digest.hexdigest()}"


def write(path: Path, value: object) -> None:
    path.write_bytes(canonical_bytes(value) + b"\n")


def main() -> None:
    entries = [
        {
            "path": path.relative_to(ROOT).as_posix(),
            "sha256": sha256_file(path),
            "size": path.stat().st_size,
        }
        for path in public_files()
    ]
    manifest = {
        "aggregate": aggregate(entries),
        "aggregate_algorithm": "sha256-path-length-path-digest-v1",
        "file_count": len(entries),
        "files": entries,
        "release_tree_file_count": len(entries) + 2,
        "schema": "ferris.pulse-56-retained-build-custody-manifest/v1",
        "total_bytes": sum(int(entry["size"]) for entry in entries),
    }
    write(ROOT / "public-manifest.json", manifest)
    receipt = ROOT / "qualification-receipt.json"
    root_cause = ROOT / "root-cause-report.md"
    payload = {
        "manifest": {
            "aggregate": manifest["aggregate"],
            "raw_sha256": sha256_file(ROOT / "public-manifest.json"),
        },
        "qualification_receipt": {
            "payload_sha256": __import__("json").loads(receipt.read_bytes())["payload_sha256"],
            "raw_sha256": sha256_file(receipt),
        },
        "root_cause_report": {"raw_sha256": sha256_file(root_cause)},
        "schema": "ferris.pulse-56-retained-build-custody-seal-payload/v1",
        "scope": {
            "diagnostic_executor": False,
            "ferris_executed": False,
            "retained_build_and_custody": True,
        },
    }
    identity = sha256_bytes(canonical_bytes(payload))
    write(
        ROOT / "release-seal.json",
        {
            "payload": payload,
            "payload_sha256": identity,
            "seal_id": identity,
            "schema": "ferris.pulse-56-retained-build-custody-seal/v1",
        },
    )


if __name__ == "__main__":
    main()
