# Pulse 04 Pure Data Family Validation

Date: 2026-08-12
Implementation cutoff: `c76007894aa07f391dc60c82cedc2b0b427a6c31`
Disposition: Windows and Unix development validation passed
Evidence class: Controlled PLATFORM-001 family

## Scope

This receipt validates two exact zero-dependency pure-data consumers and
their deterministic in-memory `ferris.platform-profile/v1` records.

The owner operation normalizes one ASCII record key:

- `r1` trims and lowercases, but rejects internal ASCII whitespace; and
- `r2` additionally accepts internal ASCII whitespace and collapses each run
  to `-`.

Both revisions reject empty and non-ASCII input.

## Exact identities

| Revision | Package | Source-tree digest | Canonical profile digest |
|---|---|---|---|
| `r1` | `ferris-profile-pure-data@0.1.0` | `sha256:862d028f8c034e9fee92de1992a89d479a526c6c6855e4e0f0ab8b5aca84790f` | `sha256:f095f19beb0f8fa2dfeb249dd54e0b7e7969e286187f7f35d219920d3660388a` |
| `r2` | `ferris-profile-pure-data@0.2.0` | `sha256:82eb8ef91701e63f07f7789b4a0a3e293e4072705b5dd010f24a16ac40fc0d6f` | `sha256:49e9bed3231fa52f26133fea9d7f17936a9b6a1b64d0919c932f5e8d3909d328` |

The source digest uses the documented test-only
`ferris.fixture-tree/v1` framing over sorted relative files and directories.
The profile digest uses the frozen `ferris.platform-profile/v1` frame.

## Owner commands

Each revision ran with `CARGO_NET_OFFLINE=true`,
`RUSTUP_AUTO_INSTALL=0`, and a separate external target directory per
command:

```console
cargo metadata --format-version 1 --no-deps --locked --offline
cargo check --locked --offline
cargo build --locked --offline
cargo clippy --locked --offline -- -D warnings
cargo test --lib --locked --offline
cargo test --doc --locked --offline
cargo package --locked --offline --allow-dirty --no-verify
```

Metadata identified exactly one package at the expected version. Every command
left the complete consumer source and lock tree byte-identical.

## Profile stage matrix

| Stage | State |
|---|---|
| resolve, check, lint, build, link, unit-test, doctest, contract-conformance, package | `pass` |
| execute, integration-test, deploy, operational-validation | `unsupported` |
| sign-attest, rollback | `not-observed` |

The link state refers only to the host Rust library artifact and explicitly
does not establish native ABI evidence. Package pass does not establish
installation. Lifecycle renewal, substitution, emergency, rollback, and
removal remain planned rather than executed.

## Windows evidence

- operating system: Microsoft Windows 11 Enterprise Insider Preview;
- build: 26310, x64;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`; and
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`.

The full workspace reported 68 passing tests, 2 ignored bounded-command
helpers, and 0 failures. Formatting, Clippy with warnings denied, and Git diff
validation passed.

## Unix evidence

- distribution: Ubuntu 24.04.4 LTS under WSL2;
- kernel: `6.6.87.2-microsoft-standard-WSL2`;
- architecture: x86-64;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`;
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`; and
- workspace target directory: `/tmp/ferris-p04-rust195-target`.

The full workspace reported 68 passing tests, 2 ignored bounded-command
helpers, and 0 failures. Formatting and Clippy with warnings denied passed.

## Claim boundary

This completes one controlled pure-data family. It does not establish:

- another PLATFORM-001 family;
- external dependency, registry, feature, build-script, macro, unsafe, native,
  provider, runtime, target, service, deployment, or credential behavior;
- native Linux support beyond the recorded WSL development path;
- performance, security, safety, compatibility, support, approval, or
  certification;
- executed renewal, substitution, emergency, rollback, or removal; or
- held-out evidence or PLATFORM-001 Proposed status.
