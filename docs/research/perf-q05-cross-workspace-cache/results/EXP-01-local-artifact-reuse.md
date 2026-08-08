# EXP-01: Local Cross-Workspace Artifact Reuse

Date: 2026-08-08
Question: PERF-Q05
Status: Complete

## Purpose

Test whether ordinary Cargo artifacts can be reused between unrelated local
workspaces, which metadata is required, how mismatches and damage behave, and
whether a shared writable target directory preserves path-package provenance.

This was a diagnostic experiment, not a latency benchmark. The fixture was
intentionally tiny and each sequence was run once. Durations are retained only
to confirm the expected cold, reused, and failed paths.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310
- Intel Core i7-12800HX, 16 cores and 24 logical processors
- 32 GiB memory
- Cargo `1.95.0 (f2d3ce0bd 2026-03-21)`
- rustc `1.95.0 (59807616e 2026-04-14)`
- host `x86_64-pc-windows-msvc`
- LLVM `22.1.2`
- global registry source cache retained
- every target directory was disposable and outside the FERRIUM repository

All build commands used `--locked --offline --message-format=json`.

## Fixture

Three unrelated applications tested immutable registry reuse:

| Workspace | Root package | Dependency |
| --- | --- | --- |
| A | `registry-a 0.1.0` | `itoa = 1.0.15` |
| B | `registry-b 0.1.0` | `itoa = 1.0.15` |
| C | `registry-c 0.1.0` | `itoa = 1.0.14` |

Two additional workspaces reproduced the path-package collision:

- both roots were named `collision-app 0.1.0`;
- both contained `collision-dep 0.1.0` at the same relative path;
- corresponding source files had equal lengths and controlled equal
  modification times;
- workspace A returned `ALPHA`;
- workspace B returned `BRAVO`.

The collision fixture is disposable and exists only to test the failure
described by Cargo issue
[#12516](https://github.com/rust-lang/cargo/issues/12516).

## Commands

Representative registry-reuse command:

```powershell
$env:CARGO_TARGET_DIR = "<disposable-target>"
cargo check `
  --manifest-path "<workspace>\Cargo.toml" `
  --locked `
  --offline `
  --message-format=json
```

The rustflag mismatch added:

```powershell
$env:RUSTFLAGS = "-Cdebuginfo=1"
```

The collision sequence used `cargo build` for workspace A and then workspace B
with one shared disposable target directory. It executed the produced binary
after each build, then ran `cargo clean` from workspace B and rebuilt B.

## Registry artifact results

| Sequence | `itoa` result | Root result | Status | Target bytes |
| --- | --- | --- | ---: | ---: |
| A in empty target | Dirty | Dirty | 0 | 83,166 |
| B in separate empty target | Dirty | Dirty | 0 | 83,165 |
| A in shared target | Dirty | Dirty | 0 | 83,166 |
| B in shared target | **Fresh** | Dirty | 0 | 128,819 |
| C with `itoa 1.0.14` | Dirty | Dirty | 0 | 210,864 |
| B with changed `RUSTFLAGS` | Dirty, new filename | Dirty, new filename | 0 | 293,527 |
| B after removing `RUSTFLAGS` | **Fresh** | **Fresh** | 0 | 293,527 |

Cargo reused `itoa 1.0.15` across unrelated workspaces A and B when the
package, source, mode, profile, target, features, toolchain, and flags matched.
The different version and rustflags intentionally missed. Returning to the
original flags recovered the previous artifact rather than overwriting it.

The target grew from 83,166 bytes after A to 293,527 bytes after retaining:

- two local root packages;
- two versions of `itoa`;
- one additional rustflag variant of `itoa` and workspace B.

This small fixture confirms coexistence and growth mechanics, not a useful
storage ratio for real projects.

## Import and damage results

| Case | Cargo behavior | Result |
| --- | --- | --- |
| Complete target snapshot from A, then build B | `itoa` fresh; B dirty | Success |
| Copy only `itoa` output files into an empty target | `itoa` dirty; B dirty | Success |
| Delete `itoa` output but retain Cargo metadata | Cargo rebuilt `itoa` and B | Success |
| Replace `itoa` metadata output with corrupt bytes | Cargo reported `itoa` fresh; rustc rejected it while compiling B | Failure 101 |

The corruption failure was:

```text
found invalid metadata files for crate `itoa`
```

Artifact bytes alone were not a reusable cache entry. Cargo also needed its
fingerprints and associated build-directory state. A complete target snapshot
contained enough state for the immutable dependency to be reused, but it also
carried unrelated local artifacts and internal layout assumptions.

Missing output failed closed by rebuilding. Corrupted output was not detected
by Cargo's freshness decision; Cargo emitted a fresh artifact message and the
consumer rustc process detected invalid metadata. This establishes a separate
integrity requirement for any import boundary.

## Path-package collision result

| Step | Cargo artifact state | Executed output |
| --- | --- | --- |
| Build workspace A | dependency dirty; root dirty | `ALPHA` |
| Build workspace B in the same target | dependency **fresh**; root **fresh** | `ALPHA` |
| `cargo clean` from workspace B, then rebuild B | dependency dirty; root dirty | `BRAVO` |

Workspace B built successfully but executed workspace A's code. Both compiler
artifacts were reported fresh even though the JSON package IDs named
workspace B's absolute paths.

This is stronger than a performance miss: the shared writable target directory
lost source provenance and returned a successful but wrong program.

`cargo clean` from workspace B reduced the shared target from 3,093,660 bytes
to zero. Cleanup restored correctness only by deleting the entire shared
state, including artifacts produced for workspace A.

## Public fixture overlap

The PERF-Q04 `cargo check` unit graphs provide a small real-corpus overlap
sample:

| Fixture | Distinct registry package IDs |
| --- | ---: |
| METIS-CORE | 9 |
| RUNE | 11 |
| PARLOR | 0 |

METIS-CORE and RUNE shared exactly one registry package ID and one visible unit
signature:

```text
unicode-ident 1.0.24
```

No exact registry package ID appeared in all three fixtures. This is an upper
bound, not a hit-rate prediction: dependency-unit identities and hidden Cargo
inputs can still prevent reuse, while other commands and portfolio fixtures
may expose more overlap.

## Interpretation

1. Immutable registry dependencies are technically reusable across unrelated
   workspaces when their complete effective build identity matches.
2. Version and compiler-flag divergence correctly create separate artifacts.
3. Cargo-managed metadata is required; copying compiler outputs is not enough.
4. Freshness is not artifact integrity. Imported entries require verification
   before Cargo treats them as usable.
5. Local path packages cannot safely share one writable target namespace
   across unrelated workspaces.
6. Whole-target snapshots have excessive provenance, cleanup, locking, and
   layout blast radius.
7. Real value depends on exact version and unit alignment, which was sparse in
   the first three public check graphs.

## Limitations

- The registry fixture used one small dependency and one diagnostic run per
  state.
- It did not test build scripts, proc macros, native dependencies, incremental
  state, rustdoc, linking, concurrent writers, or cross-platform transfer.
- The corruption case proved rejection by rustc, not every possible corrupted
  artifact behavior.
- The overlap inventory covered three pinned check graphs, not the complete
  portfolio or command matrix.
- Remote transport, signing, producer trust, and revocation belong to
  PERF-Q30.

## Retained evidence

The private session record retains:

- Cargo JSON messages for every sequence;
- stderr and exit status;
- target byte counts;
- the exact disposable fixtures;
- the corruption diagnostic;
- collision outputs before and after cleanup; and
- the public unit-overlap summary.

No upstream issue, comment, branch, or pull request was created.
