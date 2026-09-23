# Owner Executable Staging Adoption

Date: 2026-09-23

Status: Complete controlled Windows adopter evaluation and Ubuntu WSL validation

## Question

Can Ferris replace adopter-owned force-copy staging with one bounded command
while retaining explicit tool selection, repository-local executable identity,
and the existing no-approval/no-execution boundary?

## Control Record

- Product outcome: remove silent executable replacement and custom copy
  semantics from owner entrypoint onboarding.
- Ferris base: `1dbb9dde2a4c0d89f974ef37a448e31ff4474ad5`.
- Consumer: `giodl73-repo/ferris-synthetic-chain` at
  `089343a8f73ceca7e885e4b151b371444736a88d`.
- Toolchain: Cargo and rustc 1.95.0 on Windows and Ubuntu WSL.
- Maximum effort: one command, one emitted receipt, focused core/CLI tests,
  documentation, and one replay of the existing four-case synthetic chain.
- Completion: explicit-source staging, bounded content verification, atomic
  no-clobber publication, path-private output, direct entrypoint binding, and
  unchanged failure-policy results.
- Abandonment: stop if staging requires `PATH` lookup, download or installation,
  retained source paths, directory-tree ownership, approval, execution, or an
  Action Plan schema change.
- Opening Product Value Governor disposition: `continue-within-budget`.

## Implemented Boundary

`ferris stage-owner-executable --source <FILE> --destination <RELATIVE_FILE>`
accepts one explicit source and one portable repository-relative destination
whose parent already exists. The source must be a non-empty regular file no
larger than 512 MiB and, on Unix, have executable permission.

Ferris streams source bytes into a unique sibling temporary file, computes
SHA-256, preserves ordinary Unix permission bits, synchronizes the file,
rehashes source and staged bytes, rechecks source size and permissions, and
uses atomic hard-link publication without overwrite. An identical destination
with the expected ordinary Unix permissions is reused without mutation;
different content or permissions fail closed.

The emitted `ferris.owner-executable-staging-receipt/v1` contains only the
destination, byte identity and length, deterministic staging identity, and
`source_path_retained: false`. The explicit source remains visible to the
invoking process and operating system as a CLI argument but is absent from
Ferris success output and diagnostics. The command does not search `PATH` or
`PATHEXT`, create directories, download, install, approve, or execute.

## Adopter Result

The synthetic adapter replaced `Copy-Item -Force` with
`stage-owner-executable`. Its physical size increased from 124 to 128 lines;
the value is centralized verification and fail-closed replacement semantics,
not reduced adapter size. The adopter still explicitly selects Cargo and owns
creation of ignored `.ferris/runtime/bin` state.

The selected Windows Cargo binary was 12,814,336 bytes with identity
`sha256:86478e53f769379d7f0ebfa7c9aa97cb76ca92233f79aa2cc0dbee2efaac73c7`.
Initial and repeated staging emitted the same staging identity
`owner-executable-stage:650313d11b1e3891b0dd935431b3d6d7296878958a9cdbeae5bf593b5167977a`.
The clean adopter revision emitted declaration
`sha256:f0ec1f0334b976623cab149330faab05ce6e7350e4ce96e320739cbff4f3204f`
and unsigned validation plan
`sha256:3cac2a718b327d22c773b1eccecf5560054119246984403d4581cd6f95feaee4`.

| Case | Cargo exit | Stderr bytes | Classification | Disposition | Plan |
|---|---:|---:|---|---|---|
| Missing path dependency | 101 | 415 | `dependency` | `route` to `owner.dependencies.review` | none |
| Locked new path dependency | 101 | 237 | `lockfile` | `route` to `owner.lockfile.review` | none |
| Unavailable registry package offline | 101 | 381 | `offline_policy` | `prepare_action` for `owner.cargo-cache.populate` | unsigned V1 |
| Invalid Rust source | 101 | 553 | `unclassified` | `halt` at `owner.failure.manual-review` | none |

The offline case emitted unsigned failure plan
`sha256:fdea3071e996a8cf8073d2c16f553a8980b8ddfadbd550ccbbe425d6e031738c`.
The adopter `cargo test --workspace` oracle passed four unit tests and both
zero-test doc-test targets. No approval, execution, or execution receipt was
created. The tracked adopter remained clean, and all disposable replay roots
were removed.

## Validation

Windows validation:

```console
cargo test -p ferris-cli --test owner_executable_staging --test owner_entrypoint_binding --test contracts
cargo check -p ferris-cli
cargo clippy -p ferris-core -p ferris-cli --all-targets -- -D warnings
```

Ubuntu WSL validation used an isolated target directory and passed 23 focused
integration tests; Windows passed 22 because one permission-drift case is
Unix-only:

```console
CARGO_TARGET_DIR=/tmp/ferris-owner-executable-staging-target cargo test -p ferris-cli --test owner_executable_staging --test owner_entrypoint_binding --test contracts
```

Eight staging tests cover exact bytes, deterministic idempotent receipts,
source-path omission, checked-in schema shape, Unix permission preservation,
existing-content rejection, portable and escaped destinations, prepared-parent
requirements, missing, empty, directory and oversized sources, competing
publication, temporary-file cleanup, and direct binder consumption.

## Role Review

- Product Value Governor: the measured force-copy gap is replaced within the
  one-command budget. Closeout disposition: `stop-value-exhausted`; this result
  authorizes no successor.
- Scope Keeper: pass. Ferris stages only one caller-selected file and does not
  own tool discovery, installation, runtime-directory policy, approval, or
  execution.
- Validation Checker: pass for Windows and Ubuntu WSL. macOS, network filesystems,
  production repositories, and executable formats other than the selected
  Cargo binary remain unproven.
- Rust Maintainer: pass. The implementation is isolated in one core module,
  has no new dependency, and exposes typed diagnostics and a strict receipt.
- Native Platform Adopter: pass with limitation. Silent replacement is removed,
  but adopter code is four lines longer and changing the selected tool requires
  explicit cleanup of ignored runtime state.
- Ecosystem Strategist: pass. Ferris materializes exact caller-selected bytes;
  it does not become a package manager or replace Cargo or rustup.
- Autonomy Supervisor: one implementation pulse and no corrective successor
  were consumed. Stop after validation, adoption evidence, commit, and push.

## Decision

Bounded explicit executable staging is implemented and adopted by the
controlled synthetic chain. It closes the last measured custom staging
semantic without weakening executable content identity or changing Action Plan
V1. The evidence supports an incubation usability claim across Windows and
Ubuntu WSL only; it does not establish production, macOS, network-filesystem,
or package-installation support.
