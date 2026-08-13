# Pulse 19 Ordinary Cargo Preservation Validation

Date: 2026-08-12
Implementation cutoff: `e1b9e9d427b8bfcca7f21ce7f177fd31d6cf8960`
Disposition: Windows and Unix development validation passed
Evidence class: Representative development control

## Scope

This receipt validates one locked, zero-dependency Rust consumer around the
existing local `profile-diff` command.

The fixture contains:

- one edition-2024 library package;
- an exact `Cargo.lock`;
- no external dependencies, build scripts, procedural macros, native
  libraries, providers, or generated code; and
- one owner unit test asserting the fixture's baseline behavior.

The Ferris product does not invoke Cargo. The integration harness invokes
ordinary owner-native Cargo before and after Ferris.

## Test sequence

The public integration test
`profile_diff_preserves_ordinary_cargo_workflow`:

1. snapshots every consumer file and directory;
2. runs locked offline `cargo metadata --no-deps`;
3. requires valid JSON and empty Cargo metadata stderr;
4. runs the owner unit test with a first external target directory;
5. requires the owner test result to report one passing test;
6. runs `ferris profile-diff` from the consumer workspace;
7. requires typed difference exit 1 with stderr empty;
8. reruns identical locked offline Cargo metadata;
9. requires exact parsed metadata equality and unchanged stderr;
10. reruns the owner unit test with a second external target directory; and
11. requires the complete consumer snapshot to remain unchanged after every
    step.

Both Cargo test runs use separate target directories outside the consumer
workspace so the preservation assertion does not depend on reused build
artifacts.

## Owner commands

```console
cargo metadata --format-version 1 --no-deps --locked --offline --manifest-path <CONSUMER>/Cargo.toml
cargo test --quiet --locked --offline --manifest-path <CONSUMER>/Cargo.toml --target-dir <EXTERNAL_TARGET>
```

The harness also sets:

```text
CARGO_NET_OFFLINE=true
RUSTUP_AUTO_INSTALL=0
```

## Windows evidence

- OS: Windows 10 Enterprise build 26310, 64-bit;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`;
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`; and
- source cutoff:
  `e1b9e9d427b8bfcca7f21ce7f177fd31d6cf8960`.

The full workspace gate passed with 65 tests, 2 ignored bounded-command
helpers, 0 failures, formatting clean, Clippy warnings denied, and Windows
Git diff validation clean.

## Unix evidence

- distribution: Ubuntu 24.04.4 LTS under WSL2;
- kernel:
  `6.6.87.2-microsoft-standard-WSL2`;
- architecture: x86-64;
- rustc: `rustc 1.95.0 (59807616e 2026-04-14)`;
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`;
- outer workspace target directory: `/tmp/ferris-p19-rust195-target`;
- Cargo network mode: offline; and
- source cutoff:
  `e1b9e9d427b8bfcca7f21ce7f177fd31d6cf8960`.

The full workspace gate passed with 65 tests, 2 ignored bounded-command
helpers, 0 failures, formatting clean, and Clippy warnings denied.

## What the evidence establishes

At the recorded cutoff and environments, the representative consumer's
owner-native metadata is exactly equal before and after `profile-diff`, its
owner unit behavior passes before and after, and its complete source and lock
workspace remains byte-identical.

This demonstrates that the bounded command does not require a hidden
resolver, manifest, lock change, registry, feature change, source change, or
workspace-local build state for this representative control.

## What the evidence does not establish

This is not completion of PLATFORM-001 Phase 7 or a PRODUCT-001 Removal
Record. It does not cover:

- external dependencies or registry resolution;
- multi-package workspaces;
- requested or effective feature variation;
- build scripts, procedural macros, generated code, unsafe, or native inputs;
- alternate targets, linkers, runners, providers, or deployment;
- failure, unsupported, stale, or unavailable owner states;
- actual profile adoption metadata or automation;
- removal of a deployed Ferris integration; or
- hidden held-out inputs.

Each real adoption and removal case still requires its own owner-native before,
during, after, and cleanup evidence.
