# EXP-01: Development Backend Matrix

Date: 2026-08-09
Question: PERF-Q27
Status: Complete

## Decision tested

Determine when the nightly Cranelift backend improves trustworthy Rust
development iteration, which workflows remain frontend- or freshness-bound,
and which target, panic, test, optimization, runtime, and artifact boundaries
prevent a universal backend recommendation.

## Environment

- Windows 11 Enterprise Insider Preview 10.0.26310
- NTFS local workspace
- Intel Core i7-12800HX
- 16 physical cores, 24 logical processors
- 31.7 GiB memory
- `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`
- LLVM 23.1.0
- target `x86_64-pc-windows-msvc`
- rustup component `rustc-codegen-cranelift-preview`
- Cranelift backend DLL SHA-256
  `bdd5a3d18751d0bb10a218b1f34152d213f6e32ac376a31352658f99975a20fc`

Power mode was not captured and remains unknown.

## Backend selection

LLVM used ordinary nightly Cargo. Cranelift used the distributed nightly
component and Cargo's unstable profile backend surface:

```text
CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift
cargo +nightly <operation> -Zcodegen-backend
```

The component was installed with:

```text
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

Every backend used a separate target directory. Artifacts and incremental
state were not shared across backends. CPU is summed process-tree CPU time;
peak RSS is the largest sampled sum across the active process tree.

## Evidence tiers

### Tier 0 synthetic control

The 2,712-line shaped fixture from PERF-Q26 contains scalar, floating-point
loop, branch, match, and generic code. Cargo used the default unoptimized
development policy with full debuginfo and incremental compilation enabled.

Five repetitions were collected for clean check, clean build, clean test
compilation, warm no-op build, and a semantically equivalent body edit.
Backend order reversed on alternating rounds. Runtime used 15 alternating
repetitions of 80 complete fixture iterations.

### Tier 1 public control

[METIS-CORE](https://github.com/giodl73-repo/METIS-CORE) was measured at:

```text
78ae34090e043e79a206f2daffaa3889389b4790
```

The repository did not contain a committed lockfile. A disposable clone
generated one offline:

```text
SHA-256 1f4fd21fe5fb1ffd141cc6fe5956e71837159c5b5b4309b30c34c0d9f3577cdd
```

Five isolated clean debug builds, five clean test compilations, and five warm
no-op builds were measured for each backend.

## Synthetic clean workflows

| Workflow | Backend | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB |
|---|---|---:|---:|---:|---:|
| Check | LLVM | 524.4 | 34.6 | 296.9 | 90.5 |
| Check | Cranelift | 511.2 | 32.8 | 312.5 | 87.3 |
| Build | LLVM | 993.1 | 25.5 | 1,000.0 | 154.9 |
| Build | Cranelift | 964.0 | 58.1 | 843.8 | 136.6 |
| Test compile | LLVM | 728.7 | 78.5 | 578.1 | 152.5 |
| Test compile | Cranelift | 705.3 | 65.1 | 500.0 | 144.4 |

The check medians differed by 2.5%, inside the observed distributions. This is
expected because check does not perform ordinary machine-code generation.

Cranelift shortened the synthetic clean build by 2.9%, reduced CPU by 15.6%,
and reduced peak RSS by 11.9%. The wall-time result was much smaller than the
backend CPU difference because Cargo startup, rustc frontend work, metadata,
linking, and filesystem work remained.

The test-compilation row had relative MAD above 10% for LLVM and is retained as
exploratory. The binary fixture's test harness also does not codegen the
ordinary `main` workload, so it is not a substitute for a repository test
graph.

## Warm and incremental synthetic workflows

| Workflow | Backend | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB |
|---|---|---:|---:|---:|---:|
| Warm no-op build | LLVM | 98.7 | 1.9 | 46.9 | 25.4 |
| Warm no-op build | Cranelift | 99.5 | 0.5 | 62.5 | 25.4 |
| Equivalent body edit | LLVM | 838.3 | 21.9 | 578.1 | 145.2 |
| Equivalent body edit | Cranelift | 858.5 | 108.1 | 546.9 | 131.7 |

Warm no-op builds were the same because Cargo freshness avoided rustc codegen.
The body-edit Cranelift median was 2.4% slower, but its relative MAD was 12.6%.
This experiment does not establish an incremental wall-time advantage for
either backend.

Backend selection therefore cannot be inferred from a clean-build result. The
expected edit mix and the fraction of time reaching codegen are required.

## Synthetic runtime control

| Backend | Runtime median ms | MAD ms | Minimum ms | Maximum ms | Executable KiB |
|---|---:|---:|---:|---:|---:|
| LLVM | 128.995 | 4.164 | 121.464 | 483.626 | 264.5 |
| Cranelift | 152.432 | 6.808 | 141.806 | 181.803 | 241.5 |

Both executables produced the same checksum. Cranelift output ran 18.2% slower
on this development-profile workload. The executable was smaller, but one
synthetic executable is not a code-quality or deployment-size conclusion.
One LLVM repetition was a 483.626 ms system outlier; the reported median and
MAD are robust to it, and removing it does not reverse the backend ordering.

The backend's purpose is faster code generation, not release optimization.
Runtime-sensitive development commands need their own acceptance threshold.

## Public METIS-CORE workflows

### Clean debug build

| Backend | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB |
|---|---:|---:|---:|---:|
| LLVM | 13,773.7 | 733.9 | 9,703.1 | 416.6 |
| Cranelift | 10,765.5 | 379.1 | 7,250.0 | 415.3 |

Cranelift shortened the public clean debug build by 21.8% and reduced CPU by
25.3%. Peak RSS differed by less than 1%.

The METIS root Rlib was 11,868,736 bytes with LLVM and 3,158,400 bytes with
Cranelift. Rlib bytes are an intermediate representation and are not accepted
as final binary-size or runtime evidence.

### Clean test compilation

| Backend | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB |
|---|---:|---:|---:|---:|
| LLVM | 36,768.4 | 1,013.5 | 109,437.5 | 1,592.7 |
| Cranelift | 33,741.8 | 584.3 | 70,734.4 | 1,529.6 |

Cranelift shortened clean `cargo test --no-run` wall time by 8.2%, reduced CPU
by 35.4%, and reduced peak RSS by 4.0%. Test compilation included the broader
development dependency and target graph, so frontend, proc-macro, metadata,
and linker work diluted the codegen gain.

### Warm no-op build

| Backend | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB |
|---|---:|---:|---:|---:|
| LLVM | 333.3 | 15.0 | 359.4 | 42.9 |
| Cranelift | 330.8 | 8.8 | 312.5 | 43.2 |

The warm wall medians differed by less than 1%. Backend choice did not improve
a workflow that Cargo classified as fresh.

## Behavior and failure controls

Positive controls succeeded:

- direct LLVM and Cranelift executables produced the same result;
- the synthetic runtime produced one common checksum;
- a small Cranelift unit-test crate passed;
- 236 METIS tests completed successfully under both backends before the public
  suite reached environment-dependent failure cases.

Failure behavior did not remain equivalent on Windows.

The METIS parity suite depends on an external `gpmetis` executable that was not
available. LLVM reported three ordinary failed assertions and exited through
the test harness. Cranelift reached the same failure boundary but the test
process exited abnormally with Windows code `0xe06d7363`.

An isolated one-test crate made the distinction reproducible:

| Backend | Intentional failing test |
|---|---|
| LLVM | Reported the panic, named the failed test, summarized `0 passed; 1 failed`, and exited 101 |
| Cranelift | Printed `running 1 test`, then failed to complete within 15 seconds and was terminated |

A direct `catch_unwind` probe compiled with
`-Zcodegen-backend=cranelift -Cpanic=unwind`, printed the panic, and did not
complete within five seconds.

This matches the current Cranelift README: panic unwinding remains experimental
and is not supported on Windows or macOS. Passing happy-path tests do not close
the failure-path compatibility gap.

## Unsupported and unstable controls

### Stable Cargo

Stable Cargo rejected `-Zcodegen-backend`. Backend selection remains a nightly
and unstable Cargo feature.

### LTO

Cranelift rejected the direct ThinLTO control:

```text
error: LTO is not supported by rustc_codegen_cranelift
```

Release and LTO workflows remain LLVM-owned in this decision.

### Platform and instruction coverage

The current upstream support matrix lists:

- Linux x86_64 and AArch64 as supported and tested;
- macOS x86_64 and AArch64 as supported and tested;
- Windows x86_64 as supported and tested;
- Windows AArch64 as unsupported;
- additional Linux architectures with narrower distribution availability.

The current unsupported list also retains incomplete SIMD coverage, including
partial `std::arch` support. Exact target, target features, intrinsics, inline
assembly, ABI, panic strategy, and debuginfo behavior remain eligibility
dimensions.

## Other backends

`rustc_codegen_gcc` is a work-in-progress libgccjit backend. Its primary goal
is target coverage where LLVM is unavailable; runtime optimization is
secondary. It requires a patched libgccjit and reports expected test failures.
It is not a distributed, low-friction development-speed substitute for this
Windows fixture.

No other backend in the reviewed set matched Cranelift's combination of
nightly rustup distribution, Cargo profile integration, and explicit
debug-compilation goal.

## Limitations

- Primary measurements use one x86_64 Windows MSVC system.
- Cranelift support and behavior can change with every nightly.
- The synthetic fixture is intentionally codegen-heavy and is not ecosystem
  prevalence evidence.
- METIS-CORE is one public library and its complete test suite includes an
  unavailable external executable.
- The body-edit control changed one function expression and cannot represent
  every incremental edit topology.
- Source-level debugger quality was not exercised.
- SIMD, inline assembly, sanitizers, dynamic libraries, FFI unwind, profilers,
  coverage, and non-x86 targets were not measured.
- Runtime evidence is one shaped workload, not a release benchmark.
- Rlib and executable bytes are not a complete size or deployment comparison.
- Linker behavior remains PERF-Q29 and debug emission remains PERF-Q28.
