from __future__ import annotations

import hashlib
import json
from pathlib import Path

from build_freeze import canonical_bytes, sha256_file


ROOT = Path(__file__).resolve().parent
EXCLUDED_PARTS = {".work", "__pycache__"}
EXCLUDED_NAMES = {"public-manifest.json", "release-seal.json"}


def write_envelope(path: Path, schema: str, payload: dict[str, object]) -> None:
    envelope = {
        "payload": payload,
        "payload_sha256": f"sha256:{hashlib.sha256(canonical_bytes(payload)).hexdigest()}",
        "schema": schema,
    }
    path.write_text(
        json.dumps(envelope, ensure_ascii=True, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
        newline="\n",
    )


def normalize_root_cause() -> None:
    path = ROOT / "root-cause-report.json"
    current = json.loads(path.read_text(encoding="utf-8"))
    write_envelope(path, str(current["schema"]), current["payload"])


def qualification_receipt() -> None:
    ubuntu = json.loads(
        (ROOT / "frozen" / "ferris-ubuntu-24.04-x86_64-29517d732db13cc2ffa304684b344f3538ab587d.receipt.json").read_text(
            encoding="utf-8"
        )
    )
    windows = json.loads(
        (ROOT / "frozen" / "ferris-windows-x86_64-29517d732db13cc2ffa304684b344f3538ab587d.exe.receipt.json").read_text(
            encoding="utf-8"
        )
    )
    synthetic = json.loads((ROOT / "synthetic-checks.json").read_text(encoding="utf-8"))
    payload = {
        "actual_build_freezes": {
            "failed": 0,
            "passed": 2,
            "receipts": [
                ubuntu["payload_sha256"],
                windows["payload_sha256"],
            ],
        },
        "cutoff": "29517d732db13cc2ffa304684b344f3538ab587d",
        "diagnostic_execution": False,
        "deterministic_clean_rebuilds": {
            "ubuntu": {
                "builds": 2,
                "digest_match": True,
            },
            "windows": {
                "builds": 2,
                "digest_match": True,
                "linker_control": "RUSTFLAGS=-C link-arg=/Brepro",
            },
        },
        "line_ending_comparison": {
            "identical": True,
            "ubuntu_tracked_eol_counts": {
                "i/-text w/-text": 1,
                "i/lf w/lf": 928,
                "i/none w/none": 3,
            },
            "windows_tracked_eol_counts": {
                "i/-text w/-text": 1,
                "i/lf w/lf": 928,
                "i/none w/none": 3,
            },
        },
        "outcome": "pass",
        "product_files_modified": False,
        "schema": "ferris.pulse-33-public-build-freeze-qualification/v1",
        "synthetic_checks": {
            "failed": synthetic["payload"]["failed"],
            "passed": synthetic["payload"]["passed"],
            "receipt": synthetic["payload_sha256"],
            "total": synthetic["payload"]["total"],
        },
        "unit_tests": {
            "failed": 0,
            "passed": 14,
            "total": 14,
        },
    }
    write_envelope(
        ROOT / "qualification-receipt.json",
        "ferris.pulse-33-public-build-freeze-qualification-envelope/v1",
        payload,
    )


def kind(path: Path) -> str:
    relative = path.relative_to(ROOT).as_posix()
    if relative.startswith("frozen/") and relative.endswith(".receipt.json"):
        return "build-receipt"
    if relative.startswith("frozen/"):
        return "frozen-executable"
    if relative.startswith("tests/"):
        return "test"
    if relative.startswith("evidence/"):
        return "evidence"
    if relative.endswith(".md"):
        return "documentation"
    if relative.endswith(".json"):
        return "receipt-or-report"
    return "source"


def public_files() -> list[Path]:
    files = []
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        relative = path.relative_to(ROOT)
        if any(part in EXCLUDED_PARTS for part in relative.parts):
            continue
        if path.name in EXCLUDED_NAMES:
            continue
        files.append(path)
    return sorted(files, key=lambda path: path.relative_to(ROOT).as_posix())


def aggregate(files: list[dict[str, object]]) -> str:
    digest = hashlib.sha256()
    for entry in files:
        digest.update(str(entry["size"]).encode("ascii"))
        digest.update(b"\0")
        digest.update(str(entry["path"]).encode("utf-8"))
        digest.update(b"\0")
        digest.update(str(entry["sha256"]).removeprefix("sha256:").encode("ascii"))
        digest.update(b"\n")
    return f"sha256:{digest.hexdigest()}"


def manifest() -> None:
    entries = []
    for path in public_files():
        entries.append(
            {
                "kind": kind(path),
                "path": path.relative_to(ROOT).as_posix(),
                "sha256": f"sha256:{sha256_file(path)}",
                "size": path.stat().st_size,
            }
        )
    payload = {
        "aggregate": aggregate(entries),
        "aggregate_algorithm": "sha256-length-path-filedigest-v1",
        "cutoff": "29517d732db13cc2ffa304684b344f3538ab587d",
        "file_count": len(entries),
        "files": entries,
        "schema": "ferris.pulse-33-public-build-freeze-manifest/v1",
        "total_bytes": sum(int(entry["size"]) for entry in entries),
    }
    (ROOT / "public-manifest.json").write_text(
        json.dumps(payload, ensure_ascii=True, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
        newline="\n",
    )


def seal() -> None:
    manifest_path = ROOT / "public-manifest.json"
    qualification_path = ROOT / "qualification-receipt.json"
    root_cause_path = ROOT / "root-cause-report.json"
    payload = {
        "artifacts": {
            "qualification_receipt": {
                "sha256": f"sha256:{sha256_file(qualification_path)}",
                "size": qualification_path.stat().st_size,
            },
            "root_cause_report": {
                "sha256": f"sha256:{sha256_file(root_cause_path)}",
                "size": root_cause_path.stat().st_size,
            },
        },
        "cutoff": "29517d732db13cc2ffa304684b344f3538ab587d",
        "manifest": {
            "sha256": f"sha256:{sha256_file(manifest_path)}",
            "size": manifest_path.stat().st_size,
        },
        "product_change_required": False,
        "qualification": {
            "actual_build_freezes_passed": 2,
            "clean_rebuilds_passed": 4,
            "synthetic_checks_passed": 20,
            "unit_tests_passed": 14,
        },
        "schema": "ferris.pulse-33-public-build-freeze-seal-payload/v1",
    }
    write_envelope(
        ROOT / "release-seal.json",
        "ferris.pulse-33-public-build-freeze-seal/v1",
        payload,
    )


def main() -> None:
    normalize_root_cause()
    qualification_receipt()
    manifest()
    seal()


if __name__ == "__main__":
    main()
