from __future__ import annotations

import argparse
import json
from pathlib import Path

from durability import canonical_json
from sealed_store import verify_store


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", required=True)
    parser.add_argument("--platform", required=True)
    parser.add_argument("--count", required=True, type=int)
    arguments = parser.parse_args()
    result = verify_store(
        Path(arguments.root), arguments.platform, arguments.count
    )
    print(canonical_json(result).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
