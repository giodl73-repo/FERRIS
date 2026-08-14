from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path

from adapter import (
    PAIR_IDS,
    ROOT,
    canonical_json,
    find_residue,
    require_durable,
    run_ubuntu_record,
    run_windows,
)


COLLECTOR_ROOT = ROOT / "collector"
if str(COLLECTOR_ROOT) not in sys.path:
    sys.path.insert(0, str(COLLECTOR_ROOT))

from sealed_store import seal_store, write_record, write_sealed_json  # noqa: E402


WORK_ROOT = ROOT / "reproduction-work"
RECEIPT_PATH = ROOT / "reproduction-receipt.json"


def run_legacy_reload() -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        [
            sys.executable,
            str(ROOT / "legacy_verify.py"),
            "--root",
            str(WORK_ROOT),
            "--count",
            "1",
        ],
        capture_output=True,
        check=False,
    )


def write_process_pair(index: int) -> tuple[dict, dict]:
    windows_receipt = write_record(
        WORK_ROOT / "windows", run_windows(index)
    )
    require_durable(windows_receipt, f"reproduction Windows record {index}")
    ubuntu_receipt = run_ubuntu_record(WORK_ROOT / "ubuntu", index)
    require_durable(ubuntu_receipt, f"reproduction Ubuntu record {index}")
    return windows_receipt, ubuntu_receipt


def main() -> int:
    if WORK_ROOT.exists():
        shutil.rmtree(WORK_ROOT)
    for store_name in ("windows", "ubuntu", "pairs"):
        (WORK_ROOT / store_name / "records").mkdir(parents=True)

    windows_zero, ubuntu_zero = write_process_pair(0)
    pair_receipt = write_record(
        WORK_ROOT / "pairs",
        {
            "schema": "exact-two-preflight-pair-seal-v1",
            "platform": "pair",
            "index": 0,
            "pair_id": PAIR_IDS[0],
            "windows_record_sha256": windows_zero["record_sha256"],
            "ubuntu_record_sha256": ubuntu_zero["record_sha256"],
        },
    )
    require_durable(pair_receipt, "reproduction pair seal 0")
    for store_name, platform_name in (
        ("windows", "windows"),
        ("ubuntu", "ubuntu"),
        ("pairs", "pair"),
    ):
        manifest_receipt = seal_store(
            WORK_ROOT / store_name, platform_name, 1
        )
        require_durable(
            manifest_receipt, f"reproduction {store_name} one-row manifest"
        )

    first_reload = run_legacy_reload()
    if first_reload.returncode != 0:
        raise RuntimeError("the one-pair control reload did not pass")

    write_process_pair(1)
    second_reload = run_legacy_reload()
    failure_text = (
        second_reload.stdout + second_reload.stderr
    ).decode("utf-8", errors="replace")
    if second_reload.returncode == 0:
        raise RuntimeError("the accumulating-store cardinality failure was not reproduced")
    if "record cardinality or naming mismatch" not in failure_text:
        raise RuntimeError("the reproduced failure was not the expected cardinality guard")

    counts = {
        store_name: len(
            list((WORK_ROOT / store_name / "records").glob("*.json"))
        )
        for store_name in ("windows", "ubuntu", "pairs")
    }
    residue_count = len(find_residue(WORK_ROOT))
    if counts != {"windows": 2, "ubuntu": 2, "pairs": 1}:
        raise RuntimeError(f"unexpected reproduction counts: {counts}")
    if residue_count:
        raise RuntimeError("reproduction left atomic-write residue")

    shutil.rmtree(WORK_ROOT)
    payload = {
        "schema": "exact-two-cardinality-reproduction-v1",
        "outcome": "pass",
        "blocker": "preflight-cardinality-reload-failure",
        "fixed_synthetic_commands_only": True,
        "first_pair_fresh_reload": "pass",
        "second_pair_fresh_reload": "rejected-as-expected",
        "generic_trigger": (
            "A growing two-row store was reloaded with a pair-local expected "
            "count of one. The collector correctly rejected the extra row."
        ),
        "durable_process_records": 4,
        "durable_pair_seals": 1,
        "successful_fresh_process_reloads": 1,
        "failed_fresh_process_reloads": 1,
        "retries": 0,
        "residue_count": 0,
        "disposable_store_remaining": False,
    }
    receipt = write_sealed_json(RECEIPT_PATH, payload)
    require_durable(receipt, "reproduction receipt")
    print(canonical_json(payload).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
