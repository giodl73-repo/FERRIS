# Cargo Offline Policy Classification

Status: Complete bounded Windows probe; synthetic matrix abandoned before observation

## Outcome Contract

The user outcome was for Ferris to distinguish a Cargo failure caused by an
explicit offline policy from an ordinary dependency failure, so the next action
addresses source availability instead of suggesting an unnecessary dependency
edit. The budget allowed one isolated real-Cargo probe and one classifier
correction. Completion required a reproducible diagnostic and focused regression
coverage. A new architectural layer or validation harness larger than the
behavior under test was an abandonment condition.

Product Value Governor disposition: `continue-within-budget`.

## Probe

The probe used Cargo `1.95.0 (1.95.0-ms-20260618.5+ed80dadd6a)` on
`x86_64-pc-windows-msvc`. A disposable crate outside every Cargo workspace was
given a cached `serde` dependency with the normal Cargo home. Its lockfile was
removed, then `cargo check --offline` ran with an empty temporary `CARGO_HOME`.

Cargo exited nonzero with one 362-byte diagnostic containing both:

- `no matching package named` dependency-resolution language; and
- an explicit `offline mode (--offline)` policy reminder.

The raw diagnostic stayed in process memory and was not added to the repository.
Before the correction, `ferris diagnose-cargo --stderr -` classified this input
as `dependency` with `FERRIS-CARGO-DEPENDENCY-BLOCKED`. This directed the owner
toward repairing a valid dependency declaration even though the actionable
constraint was the cold offline source cache.

## Correction

Explicit offline-policy indicators now take precedence over generic dependency
resolution indicators. Replaying the same diagnostic produced:

| Field | Result |
|---|---|
| Classification | `offline_policy` |
| Diagnostic | `FERRIS-CARGO-OFFLINE-BLOCKED` |
| Classified | `true` |
| Raw output retained | `false` |
| Executable | `false` |
| Input complete | `true` |

The focused classifier suite covers explicit offline policy, dependency,
lockfile, unknown fallback, and path-fragment false positives.

## Validation

Run from the Ferris repository with Cargo 1.95.0:

| Command | Result |
|---|---|
| `cargo fmt -p ferris-core -- --check` | passed |
| `cargo test -p ferris-core cargo_failure_classification_` | 7 passed |
| `cargo test -p ferris-core --lib` | 131 passed, 2 ignored |
| `cargo test -p ferris-cli --bin ferris` | 10 passed |
| `cargo test -p ferris-cli --test cargo_failure` | 5 passed |
| `cargo test -p ferris-cli --test cli` | 62 passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | passed |

`cargo fmt --all -- --check` remains blocked by pre-existing formatting
differences in historical CLI test fixtures under the current formatter. Those
unrelated files were not rewritten for this pulse.

## Discarded Matrix

A follow-on harness was started to evaluate lockfile and offline-policy failures
across the boundaries, chain, fanout, federated workspace 00, and scale
repositories. It produced no Cargo scenario observations. The first setup failed
because Git's MSYS `tar.exe` interpreted a Windows drive path as a remote host.
After pinning the Windows system tar, the second setup failed because the
disposable empty Cargo home already existed.

Ferris autonomy rules require stopping after two consecutive invalid attempts,
so the matrix was abandoned rather than repaired again. All five source
repositories remained clean, and no matrix result is claimed.

## Decision

The real diagnostic demonstrates a user-visible precedence defect and justifies
the bounded classifier correction. This is controlled Windows evidence only. It
does not establish failure prevalence, owner adoption, cross-platform Cargo
wording, repair correctness, or five-topology coverage.
