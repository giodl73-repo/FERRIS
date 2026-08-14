from __future__ import annotations

import argparse
from pathlib import Path

from adapter import verify_exact_two


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", required=True)
    parser.add_argument(
        "--verifier-platform", choices=("windows", "ubuntu"), required=True
    )
    arguments = parser.parse_args()
    result = verify_exact_two(Path(arguments.root))
    result["verifier_platform"] = arguments.verifier_platform

    from adapter import canonical_json

    print(canonical_json(result).decode("ascii"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
