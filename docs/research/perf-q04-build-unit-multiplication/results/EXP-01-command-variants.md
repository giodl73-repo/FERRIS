# EXP-01: Cargo command and unit variants

Date: 2026-08-08
Question: PERF-Q04
Status: Complete

## Purpose

Inventory planned Cargo units across common commands and validate which
apparently repeated variants:

- are selected by target or validation coverage;
- differ by mode, effective profile, platform, feature, or dependency role;
- are actually reused across sequential commands;
- remain unexplained by the unstable unit-graph schema.

## Environment

- Windows 11 Enterprise Insider Preview `10.0.26310`
- 12th Gen Intel Core i7-12800HX
- 24 logical processors
- 31.7 GiB memory
- NTFS
- stable Cargo `1.95.0 (f2d3ce0bd 2026-03-21)`
- stable rustc `1.95.0 (59807616e 2026-04-14)`
- stable LLVM `22.1.2`
- nightly Cargo `1.99.0-nightly (c79e8f894 2026-08-04)`
- nightly rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`
- nightly LLVM `23.1.0`
- host `x86_64-pc-windows-msvc`
- Cargo source reviewed at
  `21c2a90636b4a1991eacd14eca439e7e308c1af4`

## Fixtures

| Fixture | Revision | Lockfile SHA-256 | Shape |
| --- | --- | --- | --- |
| METIS-CORE | `78ae34090e043e79a206f2daffaa3889389b4790` | `1CAC404E926E148B0471233D14629D31BA66086F10903269E46B48109D7D6CAE` | One library, integration tests, property tests, benchmark dependencies |
| RUNE | `194449444624fb10add4137cb0da8d0327164fa7` | `094B2ADC226091513605CD5072F5F4805CE9E236B4F17FC90BC6E243753EB030` | Six-member workspace, proc macro, CLI, examples, trybuild and integration tests |
| PARLOR | `0975fad880cb3bda0b911cd8eb4fc58edbbfaf29` | `F77065A0BFB7C479F2A835C5502DEEF33D2FDC7F0C76ABFBEB296B07617FDE73` | Six-member pure workspace with five libraries and one CLI |
| Feature control | Disposable | `53B8682E6DBA1241EA979CF1399FF84B5CEC10FBC116BDD03867600400C96527` | One dependency used as normal, build, and dev dependency |

RUNE and PARLOR were clean public working trees. Raw unit graphs, Cargo JSON,
stderr, and disposable targets were retained outside the repository.

The first offline RUNE unit-graph attempt exited `101` because `trybuild` was
not in the local Cargo registry cache. Dependencies were fetched with the
committed lockfile before the offline inventory. The failed attempt was not
converted into a successful sample.

## Commands

Representative planned-unit inventory:

```powershell
cargo +nightly <command> --workspace `
  --locked --offline `
  --manifest-path <fixture>\Cargo.toml `
  --unit-graph -Zunstable-options
```

Commands:

- `check`
- `build`
- `test --no-run`
- `bench --no-run`
- `build --release`
- `check --all-targets`
- `check --target x86_64-pc-windows-msvc`
- `clippy --all-targets`

METIS-CORE omitted `--workspace` because it is one package.

Stable observed-artifact sequences used new target directories and:

```powershell
cargo <command> --workspace `
  --locked --offline `
  --manifest-path <fixture>\Cargo.toml `
  --target-dir <new-target> `
  --message-format=json-render-diagnostics
```

Unit counts describe planned graph shape, not measured duration. Each stable
sequence was run once to validate freshness behavior and is not a performance
benchmark.

## Planned unit graph inventory

Each cell is `units / roots`.

| Fixture | Check | Build | Test | Bench | All targets | Clippy all targets | Release | Explicit host target |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| METIS-CORE | 16 / 1 | 16 / 1 | 114 / 9 | 107 / 2 | 114 / 10 | 114 / 10 | 16 / 1 | 16 / 1 |
| RUNE | 34 / 6 | 29 / 6 | 72 / 28 | 58 / 6 | 72 / 29 | 72 / 29 | 30 / 6 | 34 / 6 |
| PARLOR | 6 / 6 | 6 / 6 | 16 / 11 | 11 / 6 | 12 / 12 | 12 / 12 | 6 / 6 | 6 / 6 |

For all three fixtures, the Clippy all-target unit-graph JSON was byte-identical
to the check all-target JSON.

### Workspace-owned units

| Fixture and command | Workspace packages | Workspace units |
| --- | ---: | ---: |
| METIS check | 1 | 1 |
| METIS test | 1 | 10 |
| RUNE check | 6 | 7 |
| RUNE test | 6 | 34 |
| RUNE all targets | 6 | 30 |
| PARLOR check | 6 | 6 |
| PARLOR test | 6 | 16 |
| PARLOR all targets | 6 | 12 |

### Mode composition

| Fixture and command | Modes |
| --- | --- |
| METIS check | 8 build, 5 check, 3 build-script run |
| METIS test | 90 build, 8 test, 1 doctest, 15 build-script run |
| RUNE check | 12 build, 16 check, 6 build-script run |
| RUNE test | 37 build, 23 test, 5 doctest, 7 build-script run |
| PARLOR check | 6 check |
| PARLOR test | 5 build, 6 test, 5 doctest |

## Test and all-target expansion

### PARLOR

The five library packages each formed:

- one ordinary build unit;
- one test-harness unit;
- one doctest unit.

The CLI formed one test unit. This produced 16 units and 11 roots.

All-target check produced two check units for every workspace target. For
PARLOR, both serialized variants had the same package, target, profile,
platform, mode, features, and dependencies. Cargo internally distinguishes
check from check-test, but that boolean is not visible in the serialized mode.

### RUNE

The full test graph included:

- five workspace library or proc-macro targets in build, test, and doctest
  roles;
- the CLI in build and test roles;
- 18 named integration-test targets;
- dev-only `trybuild` and related dependencies.

### METIS-CORE

The single workspace package expanded to:

- one ordinary library build;
- one test-harness library;
- one doctest;
- seven integration tests.

Dev dependencies expanded the package graph from 10 packages in check to 75 in
test.

## Package-version duplicate check

```powershell
cargo tree --duplicates --locked --offline --workspace
```

- RUNE: no package-version duplicates.
- PARLOR: no package-version duplicates.
- METIS-CORE: duplicate versions of `getrandom`, `rand`, `rand_core`, and `syn`.

RUNE and PARLOR still had substantial target and mode multiplication.

## Feature resolver control

The dependency `feature-base` defined:

- `runtime`;
- `buildtime`;
- `devsupport`.

The application requested them from normal, build, and dev dependency roles.

### Resolver 2

| Command | Total units | `feature-base` variants |
| --- | ---: | --- |
| Check | 6 | build/buildtime; check/runtime |
| Build | 6 | build/buildtime; build/runtime |
| Test | 8 | build/buildtime; build/runtime+devsupport |
| Explicit target build | 6 | host/buildtime; target/runtime |

Ordinary program output:

```text
runtime=runtime build=buildtime
```

### Resolver 1

| Command | Total units | `feature-base` variants |
| --- | ---: | --- |
| Check | 6 | build/all features; check/all features |
| Build | 5 | one build unit with all features |
| Test | 7 | one build unit with all features |
| Explicit target build | 6 | host/all features; target/all features |

Ordinary program output:

```text
runtime=devsupport build=devsupport
```

Resolver 1 reduced build and test by one unit only by changing which code was
compiled in each role.

## Stable observed-artifact sequences

### PARLOR test composition

| Sequence | Step | Observed artifacts | Fresh | Dirty |
| --- | --- | ---: | ---: | ---: |
| Test alone | Test | 11 | 0 | 11 |
| Check, then test | Check | 6 | 0 | 6 |
| Check, then test | Test | 11 | 0 | 11 |
| Build, then test | Build | 6 | 0 | 6 |
| Build, then test | Test | 11 | 5 | 6 |
| All-targets check, then test | Check | 12 | 0 | 12 |
| All-targets check, then test | Test | 11 | 0 | 11 |

The unit graph had 16 planned test units, while stable Cargo JSON emitted 11
compiler artifacts. Build-script, rustdoc, doctest, and compiler-artifact
surfaces must not be assumed to have one-to-one counts.

### Check and Clippy

| Step | Observed artifacts | Fresh | Dirty |
| --- | ---: | ---: | ---: |
| Check all targets | 12 | 0 | 12 |
| Clippy all targets | 12 | 0 | 12 |

The planned unit graphs were identical; observed artifact reuse was zero.

### Dev and release

| Step | Observed artifacts | Fresh | Dirty |
| --- | ---: | ---: | ---: |
| Dev build | 6 | 0 | 6 |
| Release build | 6 | 0 | 6 |

### Implicit and explicit host target

| Step | Observed artifacts | Fresh | Dirty |
| --- | ---: | ---: | ---: |
| Implicit-host check | 6 | 0 | 6 |
| Explicit host-triple check | 6 | 0 | 6 |

## Interpretation

1. Package-version duplicates and build-unit variants are different problems.
2. Test, bench, and all-target commands expand selected coverage and modes.
3. Resolver 2 feature duplication can preserve semantics that resolver 1
   changes.
4. Effective profile fields and observed freshness matter more than profile
   labels.
5. Unit-graph equality cannot prove artifact compatibility across tools.
6. Check artifacts do not substitute for test or Clippy artifacts.
7. Explicit target selection creates a separate namespace even for the host
   triple.

## Limitations

- Three public fixtures and one synthetic control.
- Windows host only.
- Unit graph required nightly and remains unstable.
- Stable artifact sequences were single validation runs, not timing samples.
- No cross-compilation to a non-host target.
- No custom profiles, artifact dependencies, native libraries, examples, or
  target-specific dependency control.
- No CI matrix or cross-job cache measurement.
- No attempt to remove coverage or change public fixture manifests.
- Cargo JSON compiler-artifact counts do not cover every planned unit type.

