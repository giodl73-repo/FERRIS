from __future__ import annotations

import argparse
import hashlib
import json
import shutil
from pathlib import Path
from typing import Callable

from build_freeze import (
    BuildFreezeError,
    discover_executable,
    freeze_filename,
    sha256_file,
)


PACKAGE_ID = "path+file:///checkout#ferris-cli@0.1.0"
CUTOFF = "29517d732db13cc2ffa304684b344f3538ab587d"


def event(
    executable: str | None,
    *,
    package_id: str = PACKAGE_ID,
    target_name: str = "ferris",
    kind: list[str] | None = None,
    reason: str = "compiler-artifact",
) -> str:
    return json.dumps(
        {
            "executable": executable,
            "package_id": package_id,
            "reason": reason,
            "target": {"kind": kind or ["bin"], "name": target_name},
        }
    )


def expect_error(action: Callable[[], object], text: str) -> None:
    try:
        action()
    except BuildFreezeError as exc:
        if text not in str(exc):
            raise AssertionError(f"expected {text!r}, received {str(exc)!r}") from exc
        return
    raise AssertionError(f"expected BuildFreezeError containing {text!r}")


def run_checks(work: Path) -> list[dict[str, object]]:
    existing: set[str] = set()

    def exists(path: str) -> bool:
        return path in existing

    def select(lines: list[str], platform: str) -> str:
        return discover_executable(
            lines,
            PACKAGE_ID,
            "ferris",
            platform,
            exists=exists,
        )

    checks: list[tuple[str, Callable[[], None]]] = []

    def add(name: str, action: Callable[[], None]) -> None:
        checks.append((name, action))

    linux = "/checkout/target/release/ferris"
    windows = r"C:\checkout\target\release\ferris.exe"
    existing.update({linux, windows})

    add("ubuntu-release-artifact", lambda: select([event(linux)], "ubuntu-24.04-x86_64") == linux or (_ for _ in ()).throw(AssertionError()))
    add("windows-release-artifact", lambda: select([event(windows)], "windows-x86_64") == windows or (_ for _ in ()).throw(AssertionError()))

    custom_linux = "/cache/custom-target/release/ferris"
    custom_windows = r"D:\cache\custom-target\release\ferris.exe"
    existing.update({custom_linux, custom_windows})
    add("ubuntu-custom-target", lambda: select([event(custom_linux)], "ubuntu-24.04-x86_64") == custom_linux or (_ for _ in ()).throw(AssertionError()))
    add("windows-custom-target", lambda: select([event(custom_windows)], "windows-x86_64") == custom_windows or (_ for _ in ()).throw(AssertionError()))

    spaced_linux = "/checkout with spaces/target/release/ferris"
    spaced_windows = r"C:\checkout with spaces\target\release\ferris.exe"
    existing.update({spaced_linux, spaced_windows})
    add("ubuntu-spaced-path", lambda: select([event(spaced_linux)], "ubuntu-24.04-x86_64") == spaced_linux or (_ for _ in ()).throw(AssertionError()))
    add("windows-spaced-path", lambda: select([event(spaced_windows)], "windows-x86_64") == spaced_windows or (_ for _ in ()).throw(AssertionError()))

    add("non-json-lines-ignored", lambda: select(["Compiling ferris-cli", event(linux)], "ubuntu-24.04-x86_64") == linux or (_ for _ in ()).throw(AssertionError()))
    add("build-script-artifact-ignored", lambda: select([event("/wrong", kind=["custom-build"]), event(linux)], "ubuntu-24.04-x86_64") == linux or (_ for _ in ()).throw(AssertionError()))
    add("library-artifact-ignored", lambda: select([event("/wrong", kind=["lib"]), event(linux)], "ubuntu-24.04-x86_64") == linux or (_ for _ in ()).throw(AssertionError()))
    add("wrong-package-ignored", lambda: select([event("/wrong", package_id="other"), event(linux)], "ubuntu-24.04-x86_64") == linux or (_ for _ in ()).throw(AssertionError()))
    add("wrong-binary-ignored", lambda: select([event("/wrong", target_name="other"), event(linux)], "ubuntu-24.04-x86_64") == linux or (_ for _ in ()).throw(AssertionError()))
    add("null-executable-ignored", lambda: select([event(None), event(linux)], "ubuntu-24.04-x86_64") == linux or (_ for _ in ()).throw(AssertionError()))
    add("duplicate-artifact-deduplicated", lambda: select([event(linux), event(linux)], "ubuntu-24.04-x86_64") == linux or (_ for _ in ()).throw(AssertionError()))
    add("conflicting-artifacts-rejected", lambda: expect_error(lambda: select([event(linux), event(custom_linux)], "ubuntu-24.04-x86_64"), "conflicting"))
    add("missing-artifact-rejected", lambda: expect_error(lambda: select(["not-json"], "ubuntu-24.04-x86_64"), "no executable"))
    add("windows-suffix-enforced", lambda: expect_error(lambda: select([event(linux)], "windows-x86_64"), ".exe"))
    add("ubuntu-suffix-enforced", lambda: expect_error(lambda: select([event(windows)], "ubuntu-24.04-x86_64"), ".exe"))
    add("reported-file-must-exist", lambda: expect_error(lambda: discover_executable([event("/missing/ferris")], PACKAGE_ID, "ferris", "ubuntu-24.04-x86_64", exists=lambda _: False), "does not exist"))

    sample = work / "sample.bin"

    def deterministic_hash() -> None:
        sample.write_bytes(b"abc")
        expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        assert sha256_file(sample) == expected
        assert sha256_file(sample) == expected

    add("sha256-is-deterministic", deterministic_hash)

    def deterministic_names() -> None:
        assert freeze_filename("ubuntu-24.04-x86_64", CUTOFF) == f"ferris-ubuntu-24.04-x86_64-{CUTOFF}"
        assert freeze_filename("windows-x86_64", CUTOFF) == f"ferris-windows-x86_64-{CUTOFF}.exe"

    add("freeze-names-are-deterministic", deterministic_names)

    if len(checks) != 20:
        raise AssertionError(f"expected 20 checks, found {len(checks)}")

    results: list[dict[str, object]] = []
    for index, (name, action) in enumerate(checks, start=1):
        try:
            action()
        except Exception as exc:
            results.append({"check": index, "name": name, "outcome": "fail", "error": str(exc)})
        else:
            results.append({"check": index, "name": name, "outcome": "pass"})
    return results


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work-dir", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    if args.work_dir.exists():
        shutil.rmtree(args.work_dir)
    args.work_dir.mkdir(parents=True)
    try:
        results = run_checks(args.work_dir)
    finally:
        shutil.rmtree(args.work_dir)

    passed = sum(result["outcome"] == "pass" for result in results)
    payload = {
        "checks": results,
        "diagnostic_execution": False,
        "failed": len(results) - passed,
        "passed": passed,
        "schema": "ferris.public-build-freeze-synthetic-checks/v1",
        "total": len(results),
    }
    envelope = {
        "payload": payload,
        "payload_sha256": f"sha256:{hashlib.sha256(json.dumps(payload, ensure_ascii=True, separators=(',', ':'), sort_keys=True).encode('utf-8')).hexdigest()}",
        "schema": "ferris.public-build-freeze-synthetic-envelope/v1",
    }
    args.output.write_text(
        json.dumps(envelope, ensure_ascii=True, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
        newline="\n",
    )
    print(json.dumps({"failed": len(results) - passed, "passed": passed, "total": len(results)}))
    return 0 if passed == len(results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
