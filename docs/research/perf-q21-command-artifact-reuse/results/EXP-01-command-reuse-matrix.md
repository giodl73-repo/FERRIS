# EXP-01: Cross-Command Reuse Matrix

Date: 2026-08-08
Question: PERF-Q21
Status: Complete

## Purpose

Measure which current Cargo artifacts are reused across aligned check, build,
Clippy, test, documentation, and doctest command sequences.

The experiment distinguishes:

1. exact Cargo artifact reuse;
2. required target or mode-specific compilation;
3. tool-specific rustc, Clippy, and rustdoc invocations;
4. repeated command freshness;
5. the future opportunity to share compiler stages even when whole artifacts
   remain incompatible.

## Fixture

The disposable workspace contained:

```text
dep -> corelib -> app
```

- `dep` contained 2,000 generated public functions;
- `corelib` contained a public API, unit test, integration test, example, and
  one documented code example;
- `app` contained one binary;
- the development profile used incremental compilation and `debug = 0`;
- no external dependencies, build scripts, proc macros, or native libraries
  were present.

Every pair used a new workspace and three exploratory repetitions:

1. generate the lockfile;
2. run the first command from an empty target directory;
3. run the second command in the same target directory;
4. repeat the second command;
5. retain Cargo JSON artifacts, verbose compiler commands, freshness, and wall
   time.

Shared command options:

```text
-vv
-Z checksum-freshness
--locked
--offline
--message-format=json-render-diagnostics
```

Commands:

| Name | Command intent |
|---|---|
| `check` | `cargo +nightly check --workspace` |
| `checkAll` | `cargo +nightly check --workspace --all-targets` |
| `build` | `cargo +nightly build --workspace` |
| `clippy` | `cargo +nightly clippy --workspace --all-targets -- -D warnings` |
| `test` | `cargo +nightly test --workspace --no-run` |
| `doc` | `cargo +nightly doc --workspace --no-deps` |
| `doctest` | `cargo +nightly test --workspace --doc` |

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- local NTFS storage;
- rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`;
- Cargo `1.99.0-nightly (c79e8f894 2026-08-04)`;
- Clippy `0.1.99 (1a98b1e135 2026-08-07)`;
- rustdoc `1.99.0-nightly (1a98b1e13 2026-08-07)`;
- host `x86_64-pc-windows-msvc`.

The installed rustc exposed no target-stage or cross-activity incremental
option.

## Results

Every structural result below was consistent across all three repetitions.
Times are exploratory medians, not promoted optimization claims.

| Sequence | Second-command median | Fresh artifacts | Dirty artifacts | Second-command compiler invocations |
|---|---:|---:|---:|---|
| Check then build | 1,067.15 ms | 0 | 3 | 3 rustc |
| Build then check | 575.65 ms | 0 | 3 | 3 rustc |
| All-target check then Clippy | 1,077.36 ms | 0 | 8 | 8 Clippy |
| Clippy then all-target check | 859.42 ms | 0 | 8 | 8 rustc |
| All-target check then test no-run | 1,227.14 ms | 0 | 7 | 7 rustc |
| Build then test no-run | 795.05 ms | 2 | 5 | 5 rustc |
| Test no-run then build | 481.03 ms | 2 | 1 | 1 rustc |
| Build then documentation | 2,329.15 ms | 0 | 5 | 2 rustc, 3 rustdoc |
| Documentation then build | 1,085.32 ms | 0 | 3 | 3 rustc |
| Documentation then doctest | 2,750.79 ms | 0 | 2 | 2 rustc, 2 rustdoc tests |
| Doctest then documentation | 2,264.72 ms | 0 | 5 | 2 rustc, 3 rustdoc |
| Doctest then doctest | 2,040.18 ms | 2 | 0 | 2 rustdoc tests |
| Test no-run then test no-run | 167.83 ms | 7 | 0 | None |

### Check and build

Aligned workspace check and build commands reused no artifacts in either
direction.

Check emitted metadata-only outputs:

```text
libdep-fd2bf849b707af64.rmeta
libcorelib-c93609092f22ee18.rmeta
libapp-981c46ab698456e8.rmeta
```

Build emitted different metadata identities plus linkable outputs:

```text
libdep-ace6bb9cb3743711.rmeta and .rlib
libcorelib-3466010da3add07c.rmeta and .rlib
app.exe
```

For the first repetition:

| Package | Check `.rmeta` | Build `.rmeta` | Equal |
|---|---:|---:|---|
| `dep` | 326,804 bytes | 342,690 bytes | No |
| `corelib` | 2,333 bytes | 2,341 bytes | No |

The target contained two incremental namespaces for each package after the
pair. Check and build therefore did not share a current incremental base merely
because they used the same target directory.

### Check and Clippy

All-target check and Clippy selected the same eight fixture targets, but Clippy
created distinct metadata identities and ran eight `clippy-driver`
compilations. Running check after Clippy likewise ran eight rustc compilations.

This is not evidence that Clippy can be skipped. Clippy performs additional
lint analysis. It is evidence that the common frontend and semantic base is not
shared across the two current activities.

### Build and test

Build and test demonstrated real compatible reuse:

- build then test reused the ordinary `dep` and `corelib` library artifacts;
- test still compiled five test, integration, example, and binary-test roots;
- test then build reused the same two ordinary libraries;
- build still compiled the ordinary `app` binary because the test command had
  produced a test-harness binary, not the normal application executable.

All-target check then test reused nothing. Metadata-only check artifacts did
not substitute for linkable libraries or test-harness outputs.

### Documentation and doctests

Build and documentation reused no compiler artifacts in either direction.
Documentation compiled metadata-only dependency units, then ran rustdoc for all
three packages.

Documentation and doctest also reused no artifacts in either direction.
Doctest required linkable libraries, while documentation had produced
metadata-only dependency units.

Repeated doctest did reuse the two linkable library dependencies. It still ran
two rustdoc test invocations every time. Cargo source explains the boundary:
doctests are built in temporary directories and deleted, and Cargo records no
persistent doctest output.

Repeated test no-run made all seven Cargo compiler artifacts fresh and launched
no compiler. Test execution is a separate workload; this experiment used
`--no-run`.

## Failed pilot

The first Clippy run failed because the generated fixture included
`input + 0`, and `-D warnings` promoted Clippy's `identity_op` lint to an
error. The fixture was corrected to avoid the lint and the complete matrix was
rerun.

The failure is useful evidence: Clippy is not simply another spelling of check.
It can reject code accepted by rustc and therefore has required tool-specific
work even when a future common compiler base is available.

## Interpretation

Current reuse falls into four classes:

| Class | Example | Current result |
|---|---|---|
| Exact same activity | test no-run then test no-run | Cargo artifacts fresh |
| Compatible dependency artifact | build then test | Ordinary libraries reused |
| Distinct activity over a common semantic prefix | check/build, check/Clippy | No current artifact or incremental-base reuse |
| Ephemeral activity output | doctest snippets | Dependencies reused; snippet compilation repeats |

Whole-artifact equality is too coarse for the third class. Build must continue
past checking into codegen and linking. Clippy must run lint-specific analysis.
Test must compile `cfg(test)`, harness, dev-dependency, and selected target work.
Documentation and doctest use rustdoc and have different output contracts.

The opportunity is compiler-stage reuse, not command equivalence.

## Limitations

- One synthetic workspace and one Windows host.
- Three repetitions support behavior discovery, not a promoted timing claim.
- No source edit was applied between commands.
- No external dependencies, build scripts, proc macros, native libraries,
  dynamic libraries, custom profiles, release mode, LTO, cross-compilation, or
  custom rustflags.
- `cargo test --no-run` measures compilation reuse but not repeated test
  execution.
- Cargo JSON does not represent temporary doctest binaries as persistent
  compiler artifacts.
- The experiment observes the current nightly; it does not implement target
  stages or shared incremental bases.

## Result

Cargo already reuses exact compatible build artifacts where identities match,
as demonstrated by ordinary libraries shared with test. The major remaining
gap is below whole-artifact identity: compatible compiler stages are not shared
across check, build, Clippy, documentation, or doctest activities.

Doctest adds a separate gap because its generated test crates are temporary and
recompiled on every invocation even when library dependencies are fresh.
