# EXP-01: Cargo identity dimensions

Date: 2026-08-07
Question: PERF-Q02
Status: Complete

## Purpose

Change one Cargo identity or freshness dimension at a time and observe:

- unit-graph shape;
- artifact filename;
- artifact freshness;
- rebuild propagation;
- workspace relocation behavior;
- current nightly structured analysis coverage.

## Environment

- Windows 11 Enterprise Insider Preview `10.0.26310`
- 12th Gen Intel Core i7-12800HX
- 24 logical processors
- 31.7 GiB memory
- NTFS
- stable baseline: Cargo `1.95.0`, rustc `1.95.0`
- diagnostic nightly: Cargo and rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`
- Cargo source reviewed at
  `21c2a90636b4a1991eacd14eca439e7e308c1af4`

## Fixtures

### Public fixture

METIS-CORE revision:
`78ae34090e043e79a206f2daffaa3889389b4790`.

Generated lockfile SHA-256:
`1CAC404E926E148B0471233D14629D31BA66086F10903269E46B48109D7D6CAE`.

### Synthetic fixture

A two-member resolver-v2 workspace:

- `identity-app`, with one binary and one build script;
- `identity-dep`, used as both a normal dependency with feature `runtime` and a
  build dependency with feature `build-time`;
- root feature `alpha`;
- build script declares `IDENTITY_INPUT` with
  `cargo::rerun-if-env-changed`.

Generated lockfile SHA-256:
`AB7CF80D5596397EF90A00DD98B671AD3D0B8E9EC08B3FB4E6DDD6E600EEE9F7`.

The synthetic fixture and raw outputs were retained outside the repository.
No product source code was added.

## Commands

Representative unit-graph command:

```powershell
cargo +nightly check -p identity-app `
  --locked --offline `
  --manifest-path <fixture>\Cargo.toml `
  --unit-graph -Z unstable-options
```

Representative observed-build command:

```powershell
cargo +nightly check -p identity-app `
  --locked --offline `
  --manifest-path <fixture>\Cargo.toml `
  --target-dir <isolated-target> `
  --message-format=json-render-diagnostics
```

Structured analysis was enabled with:

```toml
[unstable]
build-analysis = true

[build.analysis]
enabled = true
```

## Results

### Public fixture command multiplication

| Command shape | Units | Roots | Modes |
| --- | ---: | ---: | --- |
| Check | 16 | 1 | build, check, run-custom-build |
| Build | 16 | 1 | build, run-custom-build |
| Release build | 16 | 1 | build, run-custom-build |
| Test without running | 114 | 9 | build, doctest, run-custom-build, test |
| Check with explicit host target | 16 | 1 | build, check, run-custom-build |

The explicit target graph contained host units and
`x86_64-pc-windows-msvc` target units.

### Synthetic baseline graph

The baseline `cargo check` contained five units:

| Package | Target | Mode | Features | Debug info |
| --- | --- | --- | --- | ---: |
| identity-app | binary | check | default | 2 |
| identity-app | build script | build | default | 0 |
| identity-app | build script | run-custom-build | default | 2 |
| identity-dep | library | build | build-time, default | 0 |
| identity-dep | library | check | runtime, default | 2 |

The same dependency therefore formed distinct build-time and runtime units.

### Command dimension changes

| Scenario | Units | Main identity change |
| --- | ---: | --- |
| Baseline check | 5 | Baseline |
| Enable `alpha` | 5 | Root and build-script feature sets |
| Release check | 5 | Profile fields |
| Test without running | 5 | Root/runtime dependency mode and test profile |
| Explicit host target | 5 | Runtime units target; compile-time units remain host |

### Artifact and freshness changes

| Scenario | Fresh compiler artifacts | Dirty compiler artifacts | Artifact namespace result |
| --- | ---: | ---: | --- |
| Initial baseline | 0 | 4 | Baseline artifacts created |
| Warm baseline | 4 | 0 | All baseline artifacts reused |
| Enable `alpha` | 2 | 2 | New app and build-script filenames |
| `RUSTFLAGS=-Copt-level=1` | 0 | 4 | New filenames for all compiler artifacts |
| Return to baseline flags | 4 | 0 | Original artifact set reused |
| Set declared build-script env input | 3 | 1 | Same app filename rebuilt |
| Unset declared build-script env input | 3 | 1 | Same app filename rebuilt |
| Source body edit | 3 | 1 | Same app filename rebuilt |
| Source revert | 3 | 1 | Same app filename rebuilt |
| Relocate complete workspace | 4 | 0 | Original artifact set reused |

The baseline application artifact was
`libidentity_app-767476edf30699b6.rmeta`.

With feature `alpha`, it became
`libidentity_app-75ac8d5e3503b282.rmeta`.

With `RUSTFLAGS=-Copt-level=1`, it became
`libidentity_app-d2c7a51ab6550d56.rmeta`.

The source edit, source revert, and build-script input changes continued to use
the baseline artifact filename.

### Structured rebuild cause

Changing `IDENTITY_INPUT` produced:

```text
Root rebuild:
  build-script run: environment variable changed

Cascading rebuild:
  identity-app check unit
```

The build-analysis JSONL contained:

- build invocation and toolchain identity;
- resolution and unit-graph timing;
- five unit registrations and dependency indexes;
- three fresh and two dirty fingerprint results;
- `env-var-changed` as the root cause;
- unit start, metadata completion, finish, and unblocking events.

## Interpretation

1. Unit identity, artifact namespace, and freshness are distinct.
2. Feature and compiler-flag changes may coexist as different artifact sets.
3. Source and declared build-script input changes normally overwrite the same
   artifact identity.
4. Dependency role, mode, profile, and target side can multiply one package
   into several units.
5. Relocation compatibility works for the controlled fixture.
6. Relocation-compatible local identity is not sufficient provenance for
   unrelated workspaces sharing one writable target directory.
7. Build analysis is the strongest observed structured causal interface, but
   it is nightly and unstable.

## Limitations

- One small synthetic workspace and one small public package.
- Windows host only.
- Unit graph and build analysis required nightly.
- No symlink, custom target JSON, native dependency, artifact dependency,
  wrapper, LTO, or remap-path-prefix experiment.
- No attempt to reproduce the known unrelated-workspace collision.
- No instrumentation-overhead claim.
