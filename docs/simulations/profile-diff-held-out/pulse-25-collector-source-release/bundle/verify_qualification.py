from __future__ import annotations

import json
from pathlib import Path

from durability import canonical_json, find_residue
from sealed_store import read_sealed_json, verify_store


ROOT = Path(__file__).resolve().parent
DATA_ROOT = ROOT / "qualification-data"


def main() -> int:
    before = (DATA_ROOT / "qualification-seal.json").read_bytes()
    qualification, seal_digest = read_sealed_json(
        DATA_ROOT / "qualification-seal.json"
    )
    count = qualification["pair_count"]
    results = {
        "windows": verify_store(DATA_ROOT / "windows", "windows", count),
        "ubuntu": verify_store(DATA_ROOT / "ubuntu", "ubuntu", count),
        "pairs": verify_store(DATA_ROOT / "pairs", "pair", count),
    }
    after = (DATA_ROOT / "qualification-seal.json").read_bytes()
    if before != after:
        raise ValueError("qualification verification modified the seal")
    residue_count = len(find_residue(DATA_ROOT))
    if residue_count:
        raise ValueError("qualification residue detected")
    result = {
        "outcome": "pass",
        "qualification_seal_file_sha256": seal_digest,
        "pair_count": count,
        "residue_count": 0,
        "idempotent_read_only": True,
        "stores": results,
    }
    print(canonical_json(result).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
