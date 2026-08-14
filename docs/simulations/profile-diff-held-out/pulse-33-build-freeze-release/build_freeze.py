from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Callable, Iterable


CUTOFF_PATTERN = re.compile(r"^[0-9a-f]{40}$")
PLATFORM_SUFFIXES = {
    "ubuntu-24.04-x86_64": "",
    "windows-x86_64": ".exe",
}


class BuildFreezeError(RuntimeError):
    pass


def canonical_bytes(value: object) -> bytes:
    return json.dumps(
        value,
        ensure_ascii=True,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")


def validate_cutoff(cutoff: str) -> str:
    if not CUTOFF_PATTERN.fullmatch(cutoff):
        raise BuildFreezeError("cutoff must be a lowercase 40-character Git object ID")
    return cutoff


def freeze_filename(platform: str, cutoff: str) -> str:
    validate_cutoff(cutoff)
    try:
        suffix = PLATFORM_SUFFIXES[platform]
    except KeyError as exc:
        raise BuildFreezeError(f"unsupported platform: {platform}") from exc
    return f"ferris-{platform}-{cutoff}{suffix}"


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _run(
    command: list[str],
    cwd: Path,
    *,
    allow_failure: bool = False,
    env: dict[str, str] | None = None,
) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        command,
        cwd=cwd,
        check=False,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        env=env,
    )
    if completed.returncode and not allow_failure:
        detail = completed.stderr.strip() or completed.stdout.strip()
        raise BuildFreezeError(
            f"command failed with exit {completed.returncode}: {command[0]}: {detail}"
        )
    return completed


def resolve_tool(name: str, explicit: str | None = None) -> str:
    if explicit:
        candidate = Path(explicit).expanduser()
        if candidate.is_file():
            return str(candidate)
        resolved = shutil.which(explicit)
        if resolved:
            return resolved
        raise BuildFreezeError(f"tool is unavailable: {explicit}")

    resolved = shutil.which(name)
    if resolved:
        return resolved

    if name in {"cargo", "rustc"}:
        suffix = ".exe" if os.name == "nt" else ""
        fallback = Path.home() / ".cargo" / "bin" / f"{name}{suffix}"
        if fallback.is_file():
            return str(fallback)
    raise BuildFreezeError(f"tool is unavailable: {name}")


def verify_platform(platform: str) -> None:
    if platform not in PLATFORM_SUFFIXES:
        raise BuildFreezeError(f"unsupported platform: {platform}")
    if platform.startswith("windows-") and os.name != "nt":
        raise BuildFreezeError("Windows freeze must run on Windows")
    if platform.startswith("ubuntu-") and not sys.platform.startswith("linux"):
        raise BuildFreezeError("Ubuntu freeze must run on Linux")


def verify_checkout(repo: Path, cutoff: str, git: str) -> dict[str, object]:
    actual = _run([git, "rev-parse", "HEAD"], repo).stdout.strip()
    if actual != cutoff:
        raise BuildFreezeError(f"checkout HEAD {actual} does not match cutoff {cutoff}")

    object_type = _run([git, "cat-file", "-t", cutoff], repo).stdout.strip()
    if object_type != "commit":
        raise BuildFreezeError("cutoff is not a commit")

    autocrlf = _run(
        [git, "config", "--get", "core.autocrlf"],
        repo,
        allow_failure=True,
    ).stdout.strip().lower()
    if autocrlf != "false":
        raise BuildFreezeError("core.autocrlf must be false")

    status = _run(
        [git, "status", "--porcelain", "--untracked-files=no"],
        repo,
    ).stdout.strip()
    if status:
        raise BuildFreezeError("checkout has tracked modifications")

    _run([git, "diff", "--check"], repo)
    return {
        "core_autocrlf": False,
        "exact_commit": True,
        "tracked_files_clean": True,
    }


def package_metadata(metadata: dict[str, object]) -> tuple[str, str]:
    packages = [
        package
        for package in metadata.get("packages", [])
        if package.get("name") == "ferris-cli"
    ]
    if len(packages) != 1:
        raise BuildFreezeError("Cargo metadata must contain exactly one ferris-cli package")

    package = packages[0]
    targets = [
        target
        for target in package.get("targets", [])
        if target.get("name") == "ferris" and "bin" in target.get("kind", [])
    ]
    if len(targets) != 1:
        raise BuildFreezeError("ferris-cli must expose exactly one ferris binary target")
    return str(package["id"]), str(targets[0]["name"])


def discover_executable(
    messages: Iterable[str],
    package_id: str,
    binary_name: str,
    platform: str,
    *,
    exists: Callable[[str], bool] = os.path.isfile,
) -> str:
    if platform not in PLATFORM_SUFFIXES:
        raise BuildFreezeError(f"unsupported platform: {platform}")

    candidates: list[str] = []
    for line in messages:
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        target = message.get("target") or {}
        if (
            message.get("reason") != "compiler-artifact"
            or message.get("package_id") != package_id
            or target.get("name") != binary_name
            or "bin" not in target.get("kind", [])
            or not message.get("executable")
        ):
            continue
        candidate = str(message["executable"])
        if candidate not in candidates:
            candidates.append(candidate)

    if not candidates:
        raise BuildFreezeError("Cargo emitted no executable artifact for ferris-cli/ferris")
    if len(candidates) != 1:
        raise BuildFreezeError("Cargo emitted conflicting executable artifacts")

    executable = candidates[0]
    suffix = PLATFORM_SUFFIXES[platform]
    if suffix and not executable.lower().endswith(suffix):
        raise BuildFreezeError("Windows executable does not end in .exe")
    if not suffix and executable.lower().endswith(".exe"):
        raise BuildFreezeError("Ubuntu executable unexpectedly ends in .exe")
    if not exists(executable):
        raise BuildFreezeError("Cargo-reported executable does not exist")
    return executable


def _tool_version(tool: str, repo: Path) -> str:
    return _run([tool, "--version"], repo).stdout.strip()


def _rustc_host(rustc: str, repo: Path) -> str:
    output = _run([rustc, "-vV"], repo).stdout.splitlines()
    hosts = [line.removeprefix("host: ").strip() for line in output if line.startswith("host: ")]
    if len(hosts) != 1:
        raise BuildFreezeError("rustc did not report exactly one host triple")
    return hosts[0]


def build_and_freeze(
    repo: Path,
    cutoff: str,
    platform: str,
    output: Path,
    *,
    cargo_arg: str | None = None,
    git_arg: str | None = None,
    rustc_arg: str | None = None,
    retain_executable: bool = False,
) -> dict[str, object]:
    cutoff = validate_cutoff(cutoff)
    verify_platform(platform)
    repo = repo.resolve(strict=True)
    output.mkdir(parents=True, exist_ok=True)

    cargo = resolve_tool("cargo", cargo_arg)
    git = resolve_tool("git", git_arg)
    rustc = resolve_tool("rustc", rustc_arg)
    checkout = verify_checkout(repo, cutoff, git)

    metadata_result = _run(
        [cargo, "metadata", "--locked", "--no-deps", "--format-version", "1"],
        repo,
    )
    metadata = json.loads(metadata_result.stdout)
    package_id, binary_name = package_metadata(metadata)

    command = [
        cargo,
        "build",
        "--locked",
        "--release",
        "--package",
        "ferris-cli",
        "--bin",
        binary_name,
        "--message-format=json-render-diagnostics",
    ]
    build_env = os.environ.copy()
    build_env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    build_env["CARGO_INCREMENTAL"] = "0"
    reproducibility_controls = ["CARGO_INCREMENTAL=0"]
    if platform.startswith("windows-"):
        build_env["RUSTFLAGS"] = "-C link-arg=/Brepro"
        reproducibility_controls.append("RUSTFLAGS=-C link-arg=/Brepro")
    else:
        build_env.pop("RUSTFLAGS", None)
    build = _run(command, repo, env=build_env)
    executable = Path(
        discover_executable(
            build.stdout.splitlines(),
            package_id,
            binary_name,
            platform,
        )
    )

    filename = freeze_filename(platform, cutoff)
    destination = output / filename
    artifact_size = executable.stat().st_size
    artifact_sha256 = sha256_file(executable)
    if retain_executable:
        partial = output / f"{filename}.partial"
        shutil.copyfile(executable, partial)
        os.replace(partial, destination)
    elif destination.exists():
        destination.unlink()

    payload = {
        "artifact": {
            "discovery": "cargo-compiler-artifact-json",
            "logical_filename": filename,
            "retained_in_public_bundle": retain_executable,
            "sha256": f"sha256:{artifact_sha256}",
            "size": artifact_size,
        },
        "build": {
            "binary": binary_name,
            "cargo_version": _tool_version(cargo, repo),
            "command": [
                "cargo",
                "build",
                "--locked",
                "--release",
                "--package",
                "ferris-cli",
                "--bin",
                "ferris",
                "--message-format=json-render-diagnostics",
            ],
            "package": "ferris-cli",
            "profile": "release",
            "reproducibility_controls": reproducibility_controls,
            "rustc_host": _rustc_host(rustc, repo),
            "rustc_version": _tool_version(rustc, repo),
        },
        "checkout": checkout,
        "cutoff": cutoff,
        "platform": platform,
        "safety": {
            "diagnostic_execution": False,
            "product_files_modified": False,
        },
        "schema": "ferris.public-build-freeze-receipt/v1",
    }
    receipt = {
        "payload": payload,
        "payload_sha256": f"sha256:{hashlib.sha256(canonical_bytes(payload)).hexdigest()}",
        "schema": "ferris.public-build-freeze-envelope/v1",
    }
    receipt_path = output / f"{filename}.receipt.json"
    receipt_path.write_text(
        json.dumps(receipt, ensure_ascii=True, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
        newline="\n",
    )
    return receipt


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build, discover, hash, and freeze the public FERRIS CLI cutoff."
    )
    parser.add_argument("--repo", required=True, type=Path)
    parser.add_argument("--cutoff", required=True)
    parser.add_argument("--platform", required=True, choices=sorted(PLATFORM_SUFFIXES))
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--cargo")
    parser.add_argument("--git")
    parser.add_argument("--rustc")
    parser.add_argument("--retain-executable", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        receipt = build_and_freeze(
            args.repo,
            args.cutoff,
            args.platform,
            args.output,
            cargo_arg=args.cargo,
            git_arg=args.git,
            rustc_arg=args.rustc,
            retain_executable=args.retain_executable,
        )
    except (BuildFreezeError, OSError, json.JSONDecodeError) as exc:
        print(f"build-freeze: {exc}", file=sys.stderr)
        return 1
    print(json.dumps(receipt, ensure_ascii=True, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
