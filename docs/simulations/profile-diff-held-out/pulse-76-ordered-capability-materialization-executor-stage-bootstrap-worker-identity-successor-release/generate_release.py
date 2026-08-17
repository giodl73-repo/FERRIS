#!/usr/bin/env python3
"""Generate Pulse 76's complete-tree manifest and release seal."""
from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

sys.dont_write_bytecode = True

from sealed_dependencies import canonical_bytes, release_identities, sha256_bytes


ROOT = Path(__file__).resolve().parent
RELEASE_ROOT = (
    "docs/simulations/profile-diff-held-out/"
    "pulse-76-ordered-capability-materialization-executor-stage-bootstrap-worker-identity-successor-release"
)
SCHEMA_PATH = (
    "schemas/"
    "ferris.pulse-76-ordered-capability-materialization-executor-stage-bootstrap-worker-identity-successor.v1.schema.json"
)
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
            "P76 release tree contains Python cache residue: "
            + ", ".join(path.relative_to(ROOT).as_posix() for path in residue)
        )
    return sorted(
        (
            path
            for path in ROOT.rglob("*")
            if path.is_file()
            and path.name not in EXCLUDED
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
    entries = [
        {"path": path.relative_to(ROOT).as_posix(), "sha256": sha256_file(path), "size": path.stat().st_size}
        for path in public_files()
    ]
    manifest = {
        "aggregate": aggregate(entries),
        "aggregate_algorithm": "sha256-path-length-path-digest-v1",
        "file_count": len(entries),
        "files": entries,
        "predecessors": release_identities(),
        "release_root": RELEASE_ROOT,
        "release_tree_file_count": len(entries) + 2,
        "schema": "ferris.pulse-76-ordered-capability-materialization-executor-stage-bootstrap-worker-identity-successor-manifest/v1",
        "scope": (
            "Ordered capability/materialization successor preserving exact Pulse 70/P58 "
            "behavior while loading only a local sibling sealed binder, serializing "
            "every local and transitive exact Pulse 39/P41/P52/P35/P75 load with the "
            "final Pulse 74/P59 kernel-lock discipline, and explicitly binding exact "
            "Pulse 75 for capability execution."
        ),
        "total_bytes": sum(int(entry["size"]) for entry in entries),
    }
    write(ROOT / "public-manifest.json", manifest)
    receipt = json.loads((ROOT / "qualification-receipt.json").read_bytes())
    payload = {
        "schema": "ferris.pulse-76-ordered-capability-materialization-executor-stage-bootstrap-worker-identity-successor-seal-payload/v1",
        "manifest": {
            "aggregate": manifest["aggregate"],
            "file_count": manifest["file_count"],
            "raw_sha256": sha256_file(ROOT / "public-manifest.json"),
            "release_tree_file_count": manifest["release_tree_file_count"],
        },
        "predecessors": release_identities(),
        "qualification_receipt": {
            "fake_launches_total": receipt["payload"]["fake_launches_total"],
            "payload_sha256": receipt["payload_sha256"],
            "raw_sha256": sha256_file(ROOT / "qualification-receipt.json"),
        },
        "root_cause_report": {"raw_sha256": sha256_file(ROOT / "root-cause-report.md")},
        "schema_definition": {"raw_sha256": sha256_file(ROOT / SCHEMA_PATH)},
        "scope": {
            "diagnostic_authority": False,
            "exact_p35_p39_p41_p52_ordering": True,
            "exact_p75_capability_binding": True,
            "ferris_executed_in_qualification": False,
            "kernel_lock_cross_process_serialization": True,
            "local_sibling_sealed_loader": True,
            "public_publication": False,
            "transitive_sealed_loading_serialization": True,
        },
        "limits": {
            "ambient_sealed_dependency_resolution": False,
            "p44_p45_execution": False,
            "public_publication": False,
            "real_ferris_execution": False,
            "topology_per_platform": "70/69/1",
        },
    }
    identity = sha256_bytes(canonical_bytes(payload))
    write(
        ROOT / "release-seal.json",
        {
            "payload": payload,
            "payload_sha256": identity,
            "seal_id": identity,
            "schema": "ferris.pulse-76-ordered-capability-materialization-executor-stage-bootstrap-worker-identity-successor-seal/v1",
        },
    )


if __name__ == "__main__":
    main()
