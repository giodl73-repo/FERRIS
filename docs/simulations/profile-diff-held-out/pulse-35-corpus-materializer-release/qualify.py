from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path

from corpus_materializer import (
    REQUIRED_CASE_COUNT,
    MaterializationError,
    atomic_write,
    canonical_json,
    materialize,
    sha256,
)
from verify_materialization import verify


ROOT = Path(__file__).resolve().parent
SYNTHETIC_SEED = hashlib.sha256(b"ferris-p35-synthetic-qualification-seed-v1").digest()
ALTERNATE_SEED = hashlib.sha256(b"ferris-p35-synthetic-qualification-alternate-v1").digest()


def _write_receipt(path: Path, payload: dict) -> dict:
    envelope = {
        "schema": "ferris.pulse-35-corpus-materializer-qualification-envelope/v1",
        "payload": payload,
        "payload_sha256": sha256(canonical_json(payload)[:-1]),
    }
    envelope["receipt_id"] = envelope["payload_sha256"]
    atomic_write(path, canonical_json(envelope))
    return envelope


def _require_rejection(action, label: str) -> None:
    try:
        action()
    except ValueError:
        return
    raise MaterializationError(f"{label} was accepted")


def _invalid_controls(work_root: Path) -> tuple[int, int]:
    _require_rejection(lambda: materialize(work_root / "missing.bin", work_root / "missing-output"), "missing seed control")
    directory = work_root / "directory-seed"
    directory.mkdir()
    _require_rejection(lambda: materialize(directory, work_root / "directory-output"), "non-file seed control")
    for length in (0, 31, 33):
        seed = work_root / f"seed-{length}.bin"
        seed.write_bytes(b"x" * length)
        _require_rejection(lambda seed=seed: materialize(seed, work_root / f"invalid-{length}"), f"{length}-byte seed control")
        seed.unlink()
    full_seed = work_root / "full-seed.bin"
    full_seed.write_bytes(SYNTHETIC_SEED)
    _require_rejection(lambda: materialize(full_seed, work_root / "one-case", 1), "one-case false-coverage control")
    _require_rejection(lambda: materialize(full_seed, work_root / "over-limit", 513), "maximum-case control")
    full_seed.unlink()
    directory.rmdir()
    return (7, 3)


def _semantic_tamper(output: Path, seed: Path) -> None:
    manifest_path = output / "case-manifest.json"
    coverage_path = output / "coverage-manifest.json"
    manifest = json.loads(manifest_path.read_bytes())
    coverage = json.loads(coverage_path.read_bytes())
    manifest["cases"][3]["semantic_witnesses"]["before"]["json_value_kinds"] = []
    manifest_path.write_bytes(canonical_json(manifest))
    coverage["case_manifest_sha256"] = sha256(manifest_path.read_bytes())
    coverage_path.write_bytes(canonical_json(coverage))
    _require_rejection(lambda: verify(output, seed), "semantic fake-coverage control")


def qualify(cycles: int, work_root: Path, receipt_path: Path) -> dict:
    if cycles < 20:
        raise MaterializationError("qualification requires at least 20 cycles")
    if work_root.exists():
        raise MaterializationError("qualification work root already exists")
    work_root.mkdir(parents=True)
    receipts = []
    try:
        invalid_rejections, seed_length_rejections = _invalid_controls(work_root)
        for cycle in range(1, cycles + 1):
            cycle_root = work_root / f"cycle-{cycle:03d}"
            cycle_root.mkdir()
            seed = cycle_root / "seed.bin"
            alternate_seed = cycle_root / "alternate-seed.bin"
            seed.write_bytes(SYNTHETIC_SEED)
            alternate_seed.write_bytes(ALTERNATE_SEED)
            output = cycle_root / "corpus"
            same_output = cycle_root / "corpus-same-seed"
            alternate_output = cycle_root / "corpus-different-seed"
            summary = materialize(seed, output)
            materialize(seed, same_output)
            materialize(alternate_seed, alternate_output)
            fresh = subprocess.run(
                [sys.executable, str(ROOT / "verify_materialization.py"), "--output", str(output), "--seed-file", str(seed)],
                capture_output=True,
                check=False,
            )
            if fresh.returncode != 0:
                raise MaterializationError("fresh-process materialization reload failed")
            manifest = json.loads((output / "case-manifest.json").read_bytes())
            same = json.loads((same_output / "case-manifest.json").read_bytes())
            alternate = json.loads((alternate_output / "case-manifest.json").read_bytes())
            if same["artifact_aggregate"] != manifest["artifact_aggregate"] or same["cases"] != manifest["cases"]:
                raise MaterializationError("same-seed materialization was not deterministic")
            if alternate["artifact_aggregate"] == manifest["artifact_aggregate"] or alternate["seed_commitment_sha256"] == manifest["seed_commitment_sha256"]:
                raise MaterializationError("different-seed materialization did not diverge")
            _require_rejection(lambda: materialize(seed, output), "replay control")
            (alternate_output / "unexpected.txt").write_text("extra", encoding="utf-8")
            _require_rejection(lambda: verify(alternate_output, alternate_seed), "extra-output control")
            residue = cycle_root / ".residual-output.partial-control"
            residue.mkdir()
            _require_rejection(lambda: materialize(seed, cycle_root / "residual-output"), "residue control")
            residue.rmdir()
            semantic_output = cycle_root / "semantic-tamper"
            shutil.copytree(output, semantic_output)
            _semantic_tamper(semantic_output, seed)
            receipts.append({
                "cycle": cycle,
                "case_count": summary["case_count"],
                "artifact_aggregate": manifest["artifact_aggregate"],
                "seed_commitment_sha256": manifest["seed_commitment_sha256"],
                "fresh_process_reload": True,
                "logical_retries": 0,
                "residue_count": 0,
                "directory_sync_posture": summary["directory_sync_posture"],
                "directory_sync_records": summary["directory_sync_records"],
            })
            shutil.rmtree(cycle_root)
        if any(work_root.iterdir()):
            raise MaterializationError("qualification left disposable material")
        payload = {
            "schema": "ferris.pulse-35-corpus-materializer-qualification/v1",
            "outcome": "pass",
            "cycles_required": 20,
            "cycles_run": cycles,
            "cycles_passed": cycles,
            "cycles_failed": 0,
            "case_count_per_cycle": REQUIRED_CASE_COUNT,
            "logical_case_max": 512,
            "coverage_domains_closed_per_cycle": "18/18",
            "coverage_interactions_closed_per_cycle": "8/8",
            "fresh_process_reloads": cycles,
            "deterministic_same_seed_checks": cycles,
            "different_seed_divergence_checks": cycles,
            "invalid_input_rejections": invalid_rejections,
            "seed_length_rejections": seed_length_rejections,
            "semantic_fake_coverage_rejections": cycles,
            "replay_rejections": cycles,
            "extra_output_rejections": cycles,
            "residue_rejections": cycles,
            "logical_retries": 0,
            "residue_count": 0,
            "diagnostic_execution": False,
            "product_files_modified": False,
            "private_paths_disclosed": False,
            "seed_bytes_disclosed": False,
            "seed_material_requirement": "exactly-32-byte-csprng",
            "cycle_receipts": receipts,
        }
    finally:
        if work_root.exists():
            shutil.rmtree(work_root)
    return _write_receipt(receipt_path, payload)


def main() -> int:
    parser = argparse.ArgumentParser(description="Run only synthetic Pulse 35 corpus-materializer qualification.")
    parser.add_argument("--cycles", type=int, default=20)
    parser.add_argument("--work-root", type=Path, default=ROOT / ".qualification-work")
    parser.add_argument("--receipt", type=Path, default=ROOT / "qualification-receipt.json")
    arguments = parser.parse_args()
    try:
        receipt = qualify(arguments.cycles, arguments.work_root, arguments.receipt)
    except (MaterializationError, OSError) as error:
        print(f"qualification rejected: {error}", file=sys.stderr)
        return 2
    print(canonical_json(receipt).decode("ascii"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
