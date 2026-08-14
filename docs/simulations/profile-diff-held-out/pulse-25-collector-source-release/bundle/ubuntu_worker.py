from __future__ import annotations

import argparse
from pathlib import Path

from durability import canonical_json
from sealed_store import seal_store, write_record
from synthetic_commands import run_ubuntu


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=("record", "seal"), required=True)
    parser.add_argument("--root", required=True)
    parser.add_argument("--index", type=int)
    parser.add_argument("--count", type=int)
    arguments = parser.parse_args()
    root = Path(arguments.root)

    if arguments.mode == "record":
        if arguments.index is None:
            parser.error("--index is required for record mode")
        result = write_record(root, run_ubuntu(arguments.index))
    else:
        if arguments.count is None:
            parser.error("--count is required for seal mode")
        result = seal_store(root, "ubuntu", arguments.count)

    print(canonical_json(result).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
