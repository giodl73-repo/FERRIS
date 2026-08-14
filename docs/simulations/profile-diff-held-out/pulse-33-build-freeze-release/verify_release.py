from __future__ import annotations

import hashlib
import json
from pathlib import Path

from build_freeze import canonical_bytes, sha256_file


ROOT = Path(__file__).resolve().parent


def verify_envelope(path: Path) -> None:
    envelope = json.loads(path.read_text(encoding="utf-8"))
    expected = f"sha256:{hashlib.sha256(canonical_bytes(envelope['payload'])).hexdigest()}"
    if envelope.get("payload_sha256") != expected:
        raise RuntimeError(f"payload digest mismatch: {path.name}")


def aggregate(entries: list[dict[str, object]]) -> str:
    digest = hashlib.sha256()
    for entry in entries:
        digest.update(str(entry["size"]).encode("ascii"))
        digest.update(b"\0")
        digest.update(str(entry["path"]).encode("utf-8"))
        digest.update(b"\0")
        digest.update(str(entry["sha256"]).removeprefix("sha256:").encode("ascii"))
        digest.update(b"\n")
    return f"sha256:{digest.hexdigest()}"


def verify_manifest() -> int:
    manifest = json.loads((ROOT / "public-manifest.json").read_text(encoding="utf-8"))
    entries = manifest["files"]
    for entry in entries:
        path = ROOT / str(entry["path"])
        if not path.is_file():
            raise RuntimeError(f"manifest file missing: {entry['path']}")
        if path.stat().st_size != entry["size"]:
            raise RuntimeError(f"manifest size mismatch: {entry['path']}")
        if f"sha256:{sha256_file(path)}" != entry["sha256"]:
            raise RuntimeError(f"manifest digest mismatch: {entry['path']}")
    if manifest["aggregate"] != aggregate(entries):
        raise RuntimeError("manifest aggregate mismatch")
    return len(entries)


def verify_seal() -> None:
    seal_path = ROOT / "release-seal.json"
    verify_envelope(seal_path)
    seal = json.loads(seal_path.read_text(encoding="utf-8"))["payload"]
    bindings = {
        ROOT / "public-manifest.json": seal["manifest"],
        ROOT / "qualification-receipt.json": seal["artifacts"]["qualification_receipt"],
        ROOT / "root-cause-report.json": seal["artifacts"]["root_cause_report"],
    }
    for path, binding in bindings.items():
        if path.stat().st_size != binding["size"]:
            raise RuntimeError(f"seal size mismatch: {path.name}")
        if f"sha256:{sha256_file(path)}" != binding["sha256"]:
            raise RuntimeError(f"seal digest mismatch: {path.name}")


def verify_public_paths() -> None:
    forbidden = [
        b"c:" + b"\\src\\ferris",
        b"/home/" + b"root",
        b"c:" + b"\\users\\",
        b"app" + b"data",
        b"custody" + b"-",
    ]
    for path in ROOT.rglob("*"):
        if not path.is_file() or ".work" in path.parts or "__pycache__" in path.parts:
            continue
        content = path.read_bytes().lower()
        for token in forbidden:
            if token in content or token.decode("ascii").encode("utf-16le") in content:
                raise RuntimeError(f"non-public path token in {path.relative_to(ROOT)}")


def main() -> None:
    count = verify_manifest()
    for relative in [
        "qualification-receipt.json",
        "root-cause-report.json",
        "synthetic-checks.json",
        "frozen/ferris-ubuntu-24.04-x86_64-29517d732db13cc2ffa304684b344f3538ab587d.receipt.json",
        "frozen/ferris-windows-x86_64-29517d732db13cc2ffa304684b344f3538ab587d.exe.receipt.json",
    ]:
        verify_envelope(ROOT / relative)
    verify_seal()
    verify_public_paths()
    print(json.dumps({"manifest_files_verified": count, "outcome": "pass"}))


if __name__ == "__main__":
    main()
