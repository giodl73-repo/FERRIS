#!/usr/bin/env python3
"""Generate Pulse 72's complete-tree manifest and release seal."""

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
    "pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-release"
)
SCHEMA_PATH = (
    "schemas/"
    "ferris.pulse-72-capability-bound-diagnostic-executor-stage-identity-successor.v1.schema.json"
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
            "P72 release tree contains Python cache residue: "
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
        digest.update((len(path)).to_bytes(8, "big"))
        digest.update(path)
        digest.update(bytes.fromhex(str(entry["sha256"]).removeprefix("sha256:")))
    return "sha256:" + digest.hexdigest()


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
        "predecessors": release_identities(),
        "release_root": RELEASE_ROOT,
        "release_tree_file_count": len(entries) + 2,
        "schema": "ferris.pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-manifest/v1",
        "scope": (
            "Stage-identity exact Pulse 69 successor preserving exact P57/P56/P51 "
            "semantics while byte-binding a local sibling sealed loader, capturing "
            "canonical staged root and parent identity at stage time, revalidating "
            "that identity before launch, and failing closed on substitution during "
            "cleanup."
        ),
        "total_bytes": sum(int(entry["size"]) for entry in entries),
    }
    write(ROOT / "public-manifest.json", manifest)
    receipt = json.loads((ROOT / "qualification-receipt.json").read_bytes())
    payload = {
        "schema": "ferris.pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-seal-payload/v1",
        "manifest": {
            "aggregate": manifest["aggregate"],
            "file_count": manifest["file_count"],
            "raw_sha256": sha256_file(ROOT / "public-manifest.json"),
            "release_tree_file_count": manifest["release_tree_file_count"],
        },
        "predecessors": release_identities(),
        "qualification_receipt": {
            "fake_launches_total": receipt["payload"]["fake_launches_total"],
            "owned_bundle_cleanup_total": receipt["payload"]["owned_bundle_cleanup_total"],
            "staged_identity_revalidation_total": receipt["payload"]["staged_identity_revalidation_total"],
            "payload_sha256": receipt["payload_sha256"],
            "raw_sha256": sha256_file(ROOT / "qualification-receipt.json"),
        },
        "root_cause_report": {"raw_sha256": sha256_file(ROOT / "root-cause-report.md")},
        "schema_definition": {"raw_sha256": sha256_file(ROOT / SCHEMA_PATH)},
        "scope": {
            "diagnostic_authority": False,
            "exact_p69_execution": True,
            "exact_p57_p56_p51_binding": True,
            "ferris_executed_in_qualification": False,
            "local_sibling_sealed_loader": True,
            "prelaunch_stage_identity_revalidation": True,
            "stage_time_parent_identity_captured": True,
            "stage_time_root_identity_captured": True,
        },
        "limits": {
            "ambient_sealed_dependency_resolution": False,
            "cleanup_substitution_disposition": "P57-INDETERMINATE-CLEANUP",
            "exact_tree_cleanup_required": True,
            "parent_or_root_replacement_deletion": False,
            "prelaunch_identity_equality_required": True,
            "real_ferris_execution": False,
            "worker_launch_without_identity_match": False,
        },
    }
    identity = sha256_bytes(canonical_bytes(payload))
    write(
        ROOT / "release-seal.json",
        {
            "payload": payload,
            "payload_sha256": identity,
            "schema": "ferris.pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-seal/v1",
            "seal_id": identity,
        },
    )


if __name__ == "__main__":
    main()
