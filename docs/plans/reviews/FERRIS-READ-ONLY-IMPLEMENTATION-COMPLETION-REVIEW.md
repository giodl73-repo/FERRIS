# Ferris Read-Only Implementation Completion Review

Date: 2026-08-11
Scope: Pulse 01 local `plan` and `explain`
Disposition: Complete on Windows and Unix; applicable held-out fixtures passed
Implementation authority: No expansion

## Measured result

The pulse implemented two Rust crates:

- `ferris-core` for typed planning, explanation, diagnostics, process classes,
  Cargo metadata normalization, and portable plan identity; and
- `ferris-cli` for the `plan` and `explain` adapters.

The implementation invokes only:

```console
cargo metadata --format-version 1 --no-deps --offline --locked --manifest-path <Cargo.toml>
```

It emits non-executable `v0` records, uses workspace-relative paths, preserves
ordinary Cargo as fallback, and does not discover sibling workspaces.

## Validation

Windows environment:

- `rustc 1.95.0 (59807616e 2026-04-14)`;
- `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`; and
- `stable-x86_64-pc-windows-msvc`.

Commands:

```console
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Results:

- 8 core tests passed;
- 6 CLI tests passed;
- no lint warning;
- no `unsafe`;
- invalid manifest mapped to exit 2;
- unsupported metadata mapped to exit 4;
- incomplete metadata mapped to exit 5;
- unavailable or blocked Cargo metadata mapped to exit 7;
- malformed owner output mapped to exit 11;
- plan ID remained stable across different checkout roots;
- public output contained no checkout-absolute path;
- selected workspace did not include its adjacent sibling; and
- all 12 held-out packages have independently verified public-safe SHA-256
  receipts without oracle disclosure.

Unix renewal environment:

- Ubuntu 24.04.4 LTS under WSL2 kernel
  `6.6.87.2-microsoft-standard-WSL2`;
- `rustc 1.95.0 (59807616e 2026-04-14)`;
- `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`; and
- `stable-x86_64-unknown-linux-gnu`.

The same format, test, and lint commands passed. The simple-workspace JSON
record produced the same portable plan identity on Windows and Unix:

```text
plan:ebfe9ac89f0dd76c8f2a2d04da888a4d99e2361a88a2c004f15b6c09e1e0f444
```

Independent scoring froze commit
`0cc01df0835f7651a66dd884321325e8a316775c`. The custodian classified all 12
fixtures before execution:

- FHIF-009 passed `P01-LOCAL-PLAN`;
- FHIF-012 passed `P01-LOCAL-PARITY`;
- no applicable fixture failed, blocked, or was invalid; and
- ten fixtures were outside Pulse 01 and were not executed.

The
[public-safe score receipt](../../simulations/held-out/PUBLIC_SAFE_SCORE_RECEIPT_001.md)
contains result and output digests without hidden inputs or oracles.

## Role dispositions

### Rust Safety Steward

Accept. The implementation is safe Rust, contains no `unsafe`, executes no
owner code, and makes no memory-safety or correctness claim.

### Compiler Performance Engineer

Accept with no performance claim. The pulse does not measure or reduce builds,
checks, tests, links, or iteration time.

### Interop Boundary Auditor

Accept. Cargo metadata remains the only runtime boundary. Workspace-relative
normalization does not claim ABI, native, source, or runtime semantics.

### AI Assurance Skeptic

Accept. Runtime behavior contains no model. Held-out edit and oracle packages
remained outside the repository and implementation context, and scoring
returned only public-safe dispositions and digests.

### Ecosystem Strategist

Accept. Cargo owns workspace and package discovery; Ferris neither resolves
dependencies nor creates a parallel manifest.

### Rust Maintainer

Accept. Human output uses package and workspace vocabulary, names unknowns and
limitations, and preserves direct Cargo operation and removal.

### Native Platform Adopter

Accept the Windows and Unix renewal result. Native ABI, SDK, deployment,
service, and broader distribution support remain unclaimed.

### Scope Keeper

Accept. The pulse contains two crates, two commands, development fixtures, and
no action, connector, MCP, AI, approval, or remote-evidence expansion.

### Validation Checker

Accept the cross-platform pulse. Formatting, tests, lint, negative controls,
stable identity, fixed process codes, custody receipts, and applicable
held-out passes are present on the recorded Windows and Unix environments.

## Remaining gates

- approve a new pulse before adding any command or capability.

## Decision

Pulse 01 is complete on the recorded Windows and Unix environments and remains
the maximum implementation authority. No Proposed status or action capability
is granted.
