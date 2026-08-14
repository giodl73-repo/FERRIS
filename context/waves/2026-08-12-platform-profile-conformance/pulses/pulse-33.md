# Pulse 33: Public Build-Freeze Release

Status: Complete; public build-freeze release recorded
Implementation authority: Governance, public release evidence, review, and
test-only validation only

## Goal

Close the external build-custody blocker exposed by Pulse 32 without
executing a diagnostic, changing FERRIS product code, reopening Pulse 32, or
granting a new diagnostic authority.

The public release is at
[`pulse-33-build-freeze-release/`](../../../../docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/README.md).
Its adapter verifies an exact clean cutoff, resolves the Cargo executable,
builds locked release binaries, discovers the `ferris` artifact from Cargo
`compiler-artifact` JSON, hashes it, and emits path-free receipts. It never
runs the executable.

The immutable build cutoff is
`29517d732db13cc2ffa304684b344f3538ab587d`, the same cutoff used by Pulse
32.

## Pulse 32 root cause

Pulse 32 stopped `invalid` at `cutoff-build-freeze` because its custodian
could not obtain the required Ubuntu executable. The public root-cause report
records the initiating failure as exit `127`, `cargo: command not found`, in
an Ubuntu 24.04 WSL2 non-login shell.

The same shell had an executable rustup-managed Cargo installation at
`$HOME/.cargo/bin/cargo`, but that directory was absent from `PATH`. The same
exact cutoff compiled successfully in a login shell and in the non-login
shell when Cargo was addressed explicitly. Checkout translation,
dependencies, executable naming, line endings, and actual compilation were
excluded. No FERRIS product change is required.

The adapter now checks `PATH` and then the ordinary rustup Cargo location. It
uses Cargo JSON artifact output rather than guessing a target-directory path.

## Deterministic platform builds

Both exact-cutoff locked release builds passed without diagnostic execution:

| Platform | Artifact | Size | SHA-256 |
|---|---|---:|---|
| Ubuntu 24.04 WSL2 x86_64 | `ferris` | 1,945,448 | `sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4` |
| Windows x86_64 MSVC | `ferris.exe` | 1,436,672 | `sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8` |

Two clean Ubuntu builds matched with `CARGO_INCREMENTAL=0`. An ordinary
Windows rebuild exposed the PE timestamp as build-freeze variability; two
clean builds with `RUSTFLAGS=-C link-arg=/Brepro` then matched. These are four
clean rebuilds total. The public release retains receipts, sizes, and hashes,
not executables that may contain local toolchain paths.

## Qualification and release identity

Qualification passed:

- 14 of 14 adapter unit tests;
- 20 of 20 synthetic artifact-discovery, naming, and hashing checks;
- two actual platform build freezes;
- four clean rebuilds with deterministic platform digests;
- identical tracked line-ending counts across Windows and Ubuntu checkouts;
  and
- all 37 manifest files, totaling 59,895 bytes.

The complete public identities are:

- manifest raw SHA-256:
  `sha256:9082bb18ab72e5e5ced2ec43811ecc5ce14ac43c9cd2878e4db0f10bf7a741fd`;
- 37-file aggregate:
  `sha256:07df7bc02cab288adccbdc0f87e45f3fd52939ebc9d22c8d064f773843e861a4`;
- qualification receipt raw SHA-256:
  `sha256:84c09348fe1af7c639510d4ca175bdde0eed51a27a3e2e6f2b80414c80fc10a0`;
- qualification payload SHA-256:
  `sha256:0e64090a6fa7cddfa44e63f7a6be7963498dfc9f34ef15fa1c290fa73dbac48e`;
- root-cause report raw SHA-256:
  `sha256:9c299af5548a5df004676c1dd79108d76ea0774861f8bc4d0758d44fd7a1e16b`;
- root-cause payload SHA-256:
  `sha256:e72921f8433d2a787c9142ad056bc5beff05f71836a0ab38b7fad90797d2babc`;
- synthetic receipt payload SHA-256:
  `sha256:8ca82fee60c484c9b18113ee5aa6dd9326a9f29d8c33982891a435403c32914a`;
- Ubuntu build receipt payload SHA-256:
  `sha256:b01e45259e340309772e1d5d5c947cff163ada8dad8b9ddf19775f0a537c4cae`;
- Windows build receipt payload SHA-256:
  `sha256:b1d42470ca709406c5869bae9e677334539e745faf8e0f400e4ae93f34cf7d7a`;
- release-seal raw SHA-256:
  `sha256:057f6dea59665401331b29ad984e203cca474143d7576a6617588922bf678cbd`;
  and
- release-seal payload SHA-256:
  `sha256:7ebb70ddc2a610b8c7638f30d03d0707b7d00c3eabe56ab679f085d7035f109a`.

The copied release is byte-preserved with a release-root `binary` attribute
because its sealed manifest already binds the supplied platform-native line
endings. No normalization or regeneration is authorized by this record.

## Evidence

- [Public release README](../../../../docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/README.md)
- [Public manifest](../../../../docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/public-manifest.json)
- [Root-cause report](../../../../docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/root-cause-report.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/qualification-receipt.json)
- [Release seal](../../../../docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/release-seal.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-33-PUBLIC-BUILD-FREEZE-RELEASE-ROLE-REVIEW.md)
- [Rust release validator](../../../../crates/ferris-cli/tests/pulse_33_public_build_freeze_release.rs)

## Decision

Pulse 33 is a public build-freeze release only. It establishes that the Pulse
32 Ubuntu blocker was WSL non-login Cargo discovery, that the exact cutoff
compiles on both platforms, and that deterministic hashes can be frozen by
the external adapter. It grants no diagnostic execution, product change,
score, certification, fix, support, or PLATFORM-001 authority. In particular,
there is no product change in this pulse.

## Required next step for Pulse 34

A new diagnostic authority requires an immutable cutoff that contains this
complete Pulse 33 release and predates the authority. That cutoff can only be
the future Pulse 33 commit, which does not exist while this work remains
uncommitted.

Therefore Pulse 34 authority is not implemented here. After the future Pulse
33 commit exists, a separate governance/test-only change may cite its exact
40-character commit ID as the Pulse 34 execution cutoff and prove the Pulse
34 authority is absent from that cutoff.

No placeholder or self-containing cutoff is permitted.

## Stop conditions

Stop rather than widen if work would execute a diagnostic, run a FERRIS
binary, modify product code, alter or regenerate the sealed release, reopen
Pulse 32, claim native Linux support from WSL evidence, create Pulse 34
authority before the future Pulse 33 commit exists, use a placeholder or
self-containing cutoff, or change PLATFORM-001 status.
