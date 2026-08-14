from __future__ import annotations

import argparse
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent
COLLECTOR_ROOT = ROOT / "collector"
if str(COLLECTOR_ROOT) not in sys.path:
    sys.path.insert(0, str(COLLECTOR_ROOT))

from durability import canonical_json  # noqa: E402
from sealed_store import verify_store  # noqa: E402


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", required=True)
    parser.add_argument("--count", required=True, type=int)
    arguments = parser.parse_args()
    root = Path(arguments.root)
    result = {
        "windows": verify_store(root / "windows", "windows", arguments.count),
        "ubuntu": verify_store(root / "ubuntu", "ubuntu", arguments.count),
        "pairs": verify_store(root / "pairs", "pair", arguments.count),
    }
    print(canonical_json(result).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
