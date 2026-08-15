"""Git-clean custody binding for the complete Pulse 35 tree and machine schema."""

from __future__ import annotations

import hashlib
import os
import stat
import subprocess
from dataclasses import dataclass
from pathlib import Path


P35_RELEASE_ROOT = "docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release"
P35_SCHEMA_PATH = "docs/simulations/profile-diff-held-out/schemas/ferris.pulse-35-corpus-materializer.v1.schema.json"
P37_RECEIPT_PATH = (
    "docs/simulations/profile-diff-held-out/pulse-37-checkout-normalization/"
    "PULSE-37-CHECKOUT-NORMALIZATION-RECEIPT.json"
)
P37_RECEIPT_RAW = "sha256:9c6f61340af9d6e7bcd4d294c7916d34c16c226d0c4ccf7d28c812465658bff6"


@dataclass(frozen=True)
class Identity:
    size: int
    sha256: str
    cr_bytes: int
    lf_bytes: int


@dataclass(frozen=True)
class Binding:
    path: str
    checkout_variants: tuple[Identity, ...]
    canonical_lf: Identity


def _identity(data: bytes) -> Identity:
    return Identity(
        size=len(data),
        sha256="sha256:" + hashlib.sha256(data).hexdigest(),
        cr_bytes=data.count(b"\r"),
        lf_bytes=data.count(b"\n"),
    )


def _binding(
    path: str,
    checkout_size: int,
    checkout_sha256: str,
    cr_bytes: int,
    lf_bytes: int,
    canonical_size: int,
    canonical_sha256: str,
) -> Binding:
    checkout = Identity(checkout_size, checkout_sha256, cr_bytes, lf_bytes)
    canonical = Identity(canonical_size, canonical_sha256, 0, lf_bytes)
    variants = (checkout,) if checkout == canonical else (checkout, canonical)
    return Binding(path, variants, canonical)


BINDINGS = (
    _binding(
        P35_RELEASE_ROOT + "/README.md",
        4610,
        "sha256:f594c79a972fffc30f664e4eaf9ba4eccd331f1d31f8421e39fa26c7087c1bde",
        91,
        91,
        4519,
        "sha256:7ef2d0405eb77fb84c912644385e202815c3ccb9ba92c22a0a405965b00bcab7",
    ),
    _binding(
        P35_RELEASE_ROOT + "/corpus_materializer.py",
        56034,
        "sha256:f531028a10127e7bc5f989eeffee45f89ffcfbe74660b3aa9eb4e8913aa3f73a",
        970,
        970,
        55064,
        "sha256:7f74a642ce27f5742e87870e4d39d375cfa9223a40f92d253916db81260db6ba",
    ),
    _binding(
        P35_RELEASE_ROOT + "/qualification-receipt.json",
        283334,
        "sha256:4c4f4ad1d9fa437e23f655083eb74c754114c5bea43ae111d2127fc7f051a037",
        0,
        1,
        283334,
        "sha256:4c4f4ad1d9fa437e23f655083eb74c754114c5bea43ae111d2127fc7f051a037",
    ),
    _binding(
        P35_RELEASE_ROOT + "/qualify.py",
        8693,
        "sha256:6e89cac5917419cdf26b9e2181f84f02f082b386b45b8f0cdae9de1776f33251",
        188,
        188,
        8505,
        "sha256:6f3b2ab330792e32606624367f7b9607d0d6f665f461dd3e8a255d4186012f8c",
    ),
    _binding(
        P35_RELEASE_ROOT + "/root-cause-report.json",
        666,
        "sha256:02f3a34195858b1f82acd4b9c2ea9abc42413306e40caea3b9594ed0492b6ffe",
        0,
        1,
        666,
        "sha256:02f3a34195858b1f82acd4b9c2ea9abc42413306e40caea3b9594ed0492b6ffe",
    ),
    _binding(
        P35_RELEASE_ROOT + "/root-cause-report.md",
        523,
        "sha256:6ebd42a6aee56e9b76f65bd90905b0414eed9aae2a1f17bb7b8c42ec34afbedf",
        10,
        10,
        513,
        "sha256:07b4c7e695f134878c92e55582d148037af021571265f197f2141b391eaa4d7c",
    ),
    _binding(
        P35_RELEASE_ROOT + "/tests/test_materializer.py",
        10605,
        "sha256:5b86747b9f1e9b5a37161cc564a949d260421fc0858f9ef56afc6ed80d6f34a2",
        203,
        203,
        10402,
        "sha256:a2d40c7048a0ad77bedecfa137a06ef0a487ecd1b4aad5f14247072ba71a54f7",
    ),
    _binding(
        P35_RELEASE_ROOT + "/verify_materialization.py",
        40949,
        "sha256:911fb069627a0c0bf657d7af974271f50b827cab34f326f7e09bff8045815221",
        636,
        636,
        40313,
        "sha256:352d35202c0bef1a2294daa21bc4f6151db8f86a1bc1a0465914474981c1e301",
    ),
    _binding(
        P35_RELEASE_ROOT + "/public-manifest.json",
        4620,
        "sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1",
        0,
        1,
        4620,
        "sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1",
    ),
    _binding(
        P35_RELEASE_ROOT + "/release-seal.json",
        1642,
        "sha256:17459123c674f2664d7d09ea03c00dcba72129bb1cf532cfe11f8cf4edeffd23",
        0,
        1,
        1642,
        "sha256:17459123c674f2664d7d09ea03c00dcba72129bb1cf532cfe11f8cf4edeffd23",
    ),
    _binding(
        P35_SCHEMA_PATH,
        10367,
        "sha256:d85cea956a2cf82d0bf360cbccda2d19c25705c3c17f8d2a255a8dc11852825b",
        204,
        204,
        10163,
        "sha256:3543c1d83815e0d6b2fcaee3ee14bca4ec13f1a9ef02102993ffa9edbb7c08f9",
    ),
)


class CustodyFailure(RuntimeError):
    """A fail-closed P35/P37 custody error without a private path."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


def canonical_lf(data: bytes) -> bytes:
    """Normalize only CRLF pairs and reject bare carriage-return bytes."""

    without_pairs = data.replace(b"\r\n", b"")
    if b"\r" in without_pairs:
        raise CustodyFailure("P51-P35-BARE-CR")
    return data.replace(b"\r\n", b"\n")


def _safe_regular_bytes(path: Path) -> bytes:
    try:
        initial = os.lstat(path)
        if stat.S_ISLNK(initial.st_mode) or not stat.S_ISREG(initial.st_mode):
            raise CustodyFailure("P51-P35-CUSTODY-TYPE")
        descriptor = os.open(
            path,
            os.O_RDONLY | getattr(os, "O_BINARY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except CustodyFailure:
        raise
    except OSError as error:
        raise CustodyFailure("P51-P35-CUSTODY-UNAVAILABLE") from error
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or (initial.st_dev, initial.st_ino) != (
            opened.st_dev,
            opened.st_ino,
        ):
            raise CustodyFailure("P51-P35-CUSTODY-TYPE")
        chunks: list[bytes] = []
        while chunk := os.read(descriptor, 65_536):
            chunks.append(chunk)
        return b"".join(chunks)
    except OSError as error:
        raise CustodyFailure("P51-P35-CUSTODY-UNAVAILABLE") from error
    finally:
        os.close(descriptor)


def _repo_path(repo_root: Path, relative: str) -> Path:
    try:
        root = repo_root.resolve(strict=True)
    except OSError as error:
        raise CustodyFailure("P51-P35-CUSTODY-UNAVAILABLE") from error
    path = root.joinpath(*relative.split("/"))
    try:
        path.relative_to(root)
    except ValueError as error:
        raise CustodyFailure("P51-P35-CUSTODY-PATH") from error
    return path


def _git_clean_bytes(repo_root: Path, relative: str, git: str) -> bytes:
    try:
        completed = subprocess.run(
            [git, "-C", str(repo_root), "show", "HEAD:" + relative],
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except (OSError, subprocess.SubprocessError) as error:
        raise CustodyFailure("P51-P35-GIT-CLEAN-UNAVAILABLE") from error
    if completed.returncode != 0:
        raise CustodyFailure("P51-P35-GIT-CLEAN-UNAVAILABLE")
    return completed.stdout


def verify_p35_p37_custody(repo_root: Path, *, git: str = "git") -> dict[str, int]:
    """Bind every P35 release-tree file and the formerly omitted machine schema."""

    try:
        root = repo_root.resolve(strict=True)
    except OSError as error:
        raise CustodyFailure("P51-P35-CUSTODY-UNAVAILABLE") from error
    for binding in BINDINGS:
        checkout = _safe_regular_bytes(_repo_path(root, binding.path))
        checkout_identity = _identity(checkout)
        if checkout_identity not in binding.checkout_variants:
            raise CustodyFailure("P51-P35-RAW-CHECKOUT-IDENTITY")
        normalized = canonical_lf(checkout)
        if _identity(normalized) != binding.canonical_lf:
            raise CustodyFailure("P51-P35-CANONICAL-LF-IDENTITY")
        git_clean = _git_clean_bytes(root, binding.path, git)
        if git_clean != normalized or _identity(git_clean) != binding.canonical_lf:
            raise CustodyFailure("P51-P35-GIT-CLEAN-IDENTITY")

    receipt = _safe_regular_bytes(_repo_path(root, P37_RECEIPT_PATH))
    if "sha256:" + hashlib.sha256(receipt).hexdigest() != P37_RECEIPT_RAW:
        raise CustodyFailure("P51-P37-RECEIPT-IDENTITY")
    return {
        "bound_file_count": len(BINDINGS),
        "p35_release_tree_file_count": 10,
        "machine_schema_count": 1,
        "canonical_lf_file_count": len(BINDINGS),
        "git_clean_checks": len(BINDINGS),
    }
