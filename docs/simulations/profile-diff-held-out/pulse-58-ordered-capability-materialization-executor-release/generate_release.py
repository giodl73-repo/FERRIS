#!/usr/bin/env python3
"""Generate Pulse 58's complete-tree manifest and release seal."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

from sealed_dependencies import canonical_bytes, release_identities, sha256_bytes


ROOT = Path(__file__).resolve().parent
EXCLUDED = {"public-manifest.json", "release-seal.json"}


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(65_536), b""):
            digest.update(chunk)
    return "sha256:" + digest.hexdigest()


def public_files() -> list[Path]:
    residue = [
        path
        for path in ROOT.rglob("*")
        if "__pycache__" in path.parts or path.suffix == ".pyc"
    ]
    if residue:
        raise RuntimeError(
            "P58 release tree contains Python cache residue: "
            + ", ".join(path.relative_to(ROOT).as_posix() for path in residue)
        )
    return sorted(
        (
            path for path in ROOT.rglob("*")
            if path.is_file() and path.name not in EXCLUDED
            and not any(part.startswith(".") for part in path.relative_to(ROOT).parts)
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
    return "sha256:" + digest.hexdigest()


def write(path: Path, value: object) -> None:
    path.write_bytes(canonical_bytes(value) + b"\n")


def main() -> None:
    entries = [{"path": path.relative_to(ROOT).as_posix(), "sha256": sha256_file(path), "size": path.stat().st_size} for path in public_files()]
    manifest = {
        "aggregate": aggregate(entries),
        "aggregate_algorithm": "sha256-path-length-path-digest-v1",
        "file_count": len(entries),
        "files": entries,
        "release_tree_file_count": len(entries) + 2,
        "schema": "ferris.pulse-58-ordered-capability-materialization-executor-manifest/v1",
        "scope": "Ordered P39/P41/P35/P56/P57 capability-materialization infrastructure; fake-only qualification and no diagnostic authority or publication.",
        "total_bytes": sum(int(entry["size"]) for entry in entries),
    }
    write(ROOT / "public-manifest.json", manifest)
    receipt = json.loads((ROOT / "qualification-receipt.json").read_bytes())
    payload = {
        "manifest": {"aggregate": manifest["aggregate"], "raw_sha256": sha256_file(ROOT / "public-manifest.json")},
        "predecessors": release_identities(),
        "qualification_receipt": {"payload_sha256": receipt["payload_sha256"], "raw_sha256": sha256_file(ROOT / "qualification-receipt.json")},
        "root_cause_report": {"raw_sha256": sha256_file(ROOT / "root-cause-report.md")},
        "schema": "ferris.pulse-58-ordered-capability-materialization-executor-seal-payload/v1",
        "scope": {
            "diagnostic_authority": False,
            "ferris_executed_in_qualification": False,
            "p44_p45_execution": False,
            "public_publication": False,
            "pulse_39_pulse_41_executed_in_fake_qualification": True,
            "pulse_56_live_capabilities": True,
        },
    }
    identity = sha256_bytes(canonical_bytes(payload))
    write(ROOT / "release-seal.json", {"payload": payload, "payload_sha256": identity, "seal_id": identity, "schema": "ferris.pulse-58-ordered-capability-materialization-executor-seal/v1"})


if __name__ == "__main__":
    main()
