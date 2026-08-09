# EXP-01: LLVM Pass Cost Matrix

Date: 2026-08-09
Question: PERF-Q26
Status: Complete

## Decision tested

Determine whether FERRIUM can explain which LLVM stages, passes, and Rust IR
shapes dominate development and release compilation without turning unstable
profiling output into a benchmark or recommending an unmeasured profile change.

## Environment

- Windows 11 Enterprise Insider Preview 10.0.26310
- NTFS local workspace
- Intel Core i7-12800HX
- 16 physical cores, 24 logical processors
- 31.7 GiB memory
- `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`
- LLVM 23.1.0
- target `x86_64-pc-windows-msvc`
- default MSVC linker path selected by rustc

No antivirus or indexing interference was directly observed. Power mode was
not captured and remains unknown.

## Evidence tiers

### Tier 0 synthetic control

The generated 2,712-line fixture contains four named shapes:

- 96 non-inlined scalar arithmetic functions;
- 20 nested floating-point loop functions over a 2,048-element slice;
- 48 branch- and match-heavy functions;
- 64 generic seed types consumed through an always-inlined generic function.

Every runtime row executes all four shapes and checks one common checksum.

### Tier 1 public control

[METIS-CORE](https://github.com/giodl73-repo/METIS-CORE) was measured at:

```text
78ae34090e043e79a206f2daffaa3889389b4790
```

The repository did not contain a committed lockfile. A disposable clone
generated one offline before measurement:

```text
SHA-256 1f4fd21fe5fb1ffd141cc6fe5956e71837159c5b5b4309b30c34c0d9f3577cdd
```

Dependencies were built once with ordinary Cargo. The exact final-crate rustc
command exposed by `cargo rustc -vv` was then replayed with unique metadata and
output directories. This isolates repeat root-crate compilation while
preserving Cargo-selected dependency metadata. It is a diagnostic control, not
a replacement for the ordinary Cargo workflow.

## Configuration matrix

The synthetic matrix used:

```text
rustc +nightly fixture.rs --edition=2024 --crate-name llvm_fixture \
  -Csymbol-mangling-version=v0 -Cdebuginfo=0 -o <output> <configuration>
```

| Configuration | Additional flags |
|---|---|
| `o0-cgu1` | `-Copt-level=0 -Ccodegen-units=1 -Zthinlto=no` |
| `o1-cgu1` | `-Copt-level=1 -Ccodegen-units=1 -Zthinlto=no` |
| `o2-cgu1` | `-Copt-level=2 -Ccodegen-units=1 -Zthinlto=no` |
| `o3-cgu1` | `-Copt-level=3 -Ccodegen-units=1 -Zthinlto=no` |
| `os-cgu1` | `-Copt-level=s -Ccodegen-units=1 -Zthinlto=no` |
| `oz-cgu1` | `-Copt-level=z -Ccodegen-units=1 -Zthinlto=no` |
| `o3-cgu16-auto` | `-Copt-level=3 -Ccodegen-units=16` |
| `o3-cgu16-no-thin` | `-Copt-level=3 -Ccodegen-units=16 -Zthinlto=no` |
| `o3-cgu16-thin-lto` | `-Copt-level=3 -Ccodegen-units=16 -Clto=thin -Cembed-bitcode=yes` |
| `o3-cgu1-fat-lto` | `-Copt-level=3 -Ccodegen-units=1 -Clto=fat -Cembed-bitcode=yes` |
| `o3-cgu1-debug2` | `-Copt-level=3 -Ccodegen-units=1 -Zthinlto=no -Cdebuginfo=2` |

Minimally instrumented compilation used one declared warm-up and five measured
repetitions. The order reversed on alternating rounds. Diagnostic compilation
used three repetitions with:

```text
-Zllvm-time-trace=yes
-Ztime-passes=yes
-Ztime-passes-format=json
```

Runtime used 15 alternating repetitions of 80 complete fixture iterations.

## Primary synthetic compilation

| Configuration | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB | Executable KiB | All emitted KiB |
|---|---:|---:|---:|---:|---:|---:|
| O0, one CGU | 385.3 | 5.3 | 343.8 | 73.6 | 231.0 | 1,571.0 |
| O1, one CGU | 919.1 | 1.7 | 843.8 | 74.0 | 214.0 | 1,530.0 |
| O2, one CGU | 950.5 | 32.1 | 875.0 | 74.6 | 196.0 | 1,520.0 |
| O3, one CGU | 959.8 | 35.4 | 921.9 | 73.1 | 196.5 | 1,520.5 |
| Os, one CGU | 663.2 | 2.1 | 578.1 | 74.0 | 157.5 | 1,473.5 |
| Oz, one CGU | 644.5 | 27.5 | 671.9 | 74.6 | 157.0 | 1,473.0 |
| O3, 16 CGUs, automatic mode | 794.4 | 14.0 | 1,171.9 | 78.7 | 196.5 | 1,536.5 |
| O3, 16 CGUs, local ThinLTO disabled | 756.0 | 5.4 | 1,078.1 | 95.7 | 198.5 | 1,522.5 |
| O3, 16 CGUs, explicit ThinLTO | 1,412.7 | 14.6 | 3,531.2 | 137.6 | 194.0 | 1,294.0 |
| O3, one CGU, fat LTO | 2,218.3 | 35.0 | 2,156.2 | 119.0 | 184.5 | 1,092.5 |
| O3, one CGU, debuginfo 2 | 1,148.9 | 17.7 | 1,062.5 | 85.6 | 196.5 | 1,792.5 |

Relative MAD remained below 10% for every primary row.

Compared with one-CGU O3:

- O0 shortened compilation 59.9% but increased runtime 227.2%.
- O2 changed wall time by only -1.0% and runtime by +1.2%.
- Os shortened compilation 30.9%, reduced executable bytes 19.8%, and changed
  runtime by +2.0%.
- Oz shortened compilation 32.8%, reduced executable bytes 20.1%, and changed
  runtime by +3.9%.
- 16 automatic CGUs shortened wall time 17.2% while consuming 27.1% more CPU.
- explicit ThinLTO added 47.2% wall time, 283.1% CPU, and 88.1% peak RSS.
- fat LTO added 131.1% wall time and reduced executable bytes 6.1%.
- full debuginfo added 19.7% wall time and 17.9% emitted bytes.

These are fixture results, not profile recommendations.

## Runtime control

| Configuration | Runtime median ms | MAD ms | Minimum ms | Maximum ms |
|---|---:|---:|---:|---:|
| O0, one CGU | 75.770 | 3.280 | 67.830 | 86.330 |
| O1, one CGU | 23.750 | 1.120 | 21.450 | 31.350 |
| O2, one CGU | 23.430 | 1.160 | 21.280 | 26.930 |
| O3, one CGU | 23.150 | 1.330 | 20.770 | 25.730 |
| Os, one CGU | 23.610 | 0.930 | 21.360 | 31.740 |
| Oz, one CGU | 24.050 | 1.180 | 21.680 | 31.430 |
| O3, 16 CGUs, automatic mode | 23.040 | 1.790 | 21.020 | 28.290 |
| O3, 16 CGUs, local ThinLTO disabled | 23.070 | 1.100 | 20.850 | 29.530 |
| O3, 16 CGUs, explicit ThinLTO | 22.950 | 0.800 | 20.770 | 28.860 |
| O3, one CGU, fat LTO | 22.900 | 1.500 | 21.000 | 27.470 |
| O3, one CGU, debuginfo 2 | 23.500 | 1.840 | 21.220 | 30.130 |

Every configuration produced the same checksum. O1 through O3, size modes,
multi-CGU modes, LTO modes, and debuginfo distributions overlapped. The fixture
supports the large O0 runtime loss but does not support an exact runtime win
among the optimized rows.

## LLVM trace model

The trace contains nested events:

- `OptModule` contains module optimization;
- `OptFunction` contains function optimization;
- wrapper and adaptor events contain child pass events;
- named new-pass-manager events include `InstCombinePass`,
  `LoopVectorizePass`, and `SLPVectorizerPass`;
- `RunPass` events name machine passes such as instruction selection and
  register allocation.

Parent and child durations overlap. They must not be added into a synthetic
"total LLVM time."

### Selected synthetic pass medians

All values are milliseconds from separately instrumented diagnostic runs.

| Configuration | Inliner wrapper | InstCombine | Loop vectorizer | SLP vectorizer | X86 instruction selection | Greedy register allocation |
|---|---:|---:|---:|---:|---:|---:|
| O0, one CGU | 0.0 | 0.0 | 0.0 | 0.0 | 22.0 | 0.0 |
| O1, one CGU | 165.0 | 61.9 | 0.5 | 0.0 | 89.0 | 9.7 |
| O2, one CGU | 181.7 | 71.0 | 57.6 | 0.0 | 85.6 | 8.2 |
| O3, one CGU | 171.6 | 71.7 | 56.5 | 19.3 | 80.1 | 7.6 |
| Os, one CGU | 190.9 | 49.8 | 0.4 | 0.0 | 29.1 | 4.1 |
| Oz, one CGU | 169.8 | 48.8 | 0.4 | 0.0 | 26.7 | 4.1 |
| O3, 16 CGUs, explicit ThinLTO | 929.0 | 340.5 | 83.4 | 66.6 | 363.7 | 46.7 |
| O3, one CGU, fat LTO | 241.9 | 178.3 | 69.5 | 64.6 | 313.2 | 41.1 |
| O3, one CGU, debuginfo 2 | 247.2 | 108.6 | 76.3 | 25.1 | 119.6 | 9.7 |

`ModuleInlinerWrapperPass` is inclusive of nested call-graph and function work.
Its duration cannot be added to `InstCombine`, vectorization, or inlining
children.

### Function-shape attribution

`OptFunction` event names were grouped by generated module name.

| Configuration | Scalar ms | Loop ms | Branch ms | Generic ms | Other ms |
|---|---:|---:|---:|---:|---:|
| O0, one CGU | 8.9 | 2.5 | 18.0 | 0.6 | 18.6 |
| O1, one CGU | 39.7 | 170.6 | 71.3 | 0.0 | 22.6 |
| O2, one CGU | 41.8 | 169.1 | 33.1 | 0.0 | 23.0 |
| O3, one CGU | 39.1 | 155.9 | 30.9 | 0.0 | 22.4 |
| Os, one CGU | 39.9 | 29.6 | 25.7 | 0.0 | 22.4 |
| Oz, one CGU | 38.7 | 19.5 | 25.0 | 0.0 | 24.3 |
| O3, 16 CGUs, explicit ThinLTO | 60.2 | 180.6 | 42.6 | 7.7 | 817.6 |
| O3, one CGU, fat LTO | 38.6 | 189.0 | 36.9 | 0.0 | 700.2 |

The loop family was the dominant named fixture family under O1 through O3.
The generic family disappeared as an independent optimized function under
one-CGU optimized modes because its always-inlined bodies were absorbed into
consumers. LTO introduced large "other" time from dependency and runtime
functions outside the four generated families.

## Public METIS-CORE control

Five minimally instrumented direct root-crate compilations were measured.

| Configuration | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB | Rlib KiB |
|---|---:|---:|---:|---:|---:|
| O2, one CGU | 5,199.6 | 368.4 | 4,578.1 | 136.9 | 1,170.2 |
| O3, one CGU | 5,285.5 | 310.1 | 5,031.2 | 136.7 | 1,181.4 |
| Os, one CGU | 3,324.8 | 121.5 | 3,156.2 | 133.8 | 1,554.2 |
| Oz, one CGU | 2,833.8 | 89.7 | 2,625.0 | 134.7 | 1,578.2 |

The Rlib is an intermediate archive, not a final binary-size control. Os and
Oz compiled faster while producing larger archives. This is a direct negative
control against treating one intermediate artifact size as final code quality.

One diagnostic trace per public configuration showed:

| Configuration | `LLVM_passes` ms | Inliner wrapper ms | InstCombine ms | SLP ms | Instruction selection ms | Trace MiB |
|---|---:|---:|---:|---:|---:|---:|
| O2, one CGU | 3,963.2 | 2,294.3 | 505.3 | 0.0 | 290.0 | 65.8 |
| O3, one CGU | 4,090.6 | 2,296.8 | 503.9 | 88.7 | 305.3 | 68.9 |
| Os, one CGU | 3,030.7 | 1,597.0 | 316.8 | 0.0 | 290.1 | 99.2 |
| Oz, one CGU | 2,354.9 | 1,161.0 | 228.4 | 0.0 | 233.7 | 99.9 |

The O2 and Os traces contained the same 87 named IR pass classes. Under LLVM
23, rustc maps Os and Oz to LLVM's O2 pipeline while size attributes carry the
per-function policy. Equal pass-class sets did not mean equal work.

## Observer effect

| Synthetic configuration | Primary wall ms | Diagnostic wall ms | Change | Trace MiB |
|---|---:|---:|---:|---:|
| O0, one CGU | 385.3 | 441.4 | +14.6% | 3.1 |
| O3, one CGU | 959.8 | 1,092.7 | +13.8% | 11.9 |
| Os, one CGU | 663.2 | 855.7 | +29.0% | 12.2 |
| O3, 16 CGUs, explicit ThinLTO | 1,412.7 | 1,763.4 | +24.8% | 43.5 |
| O3, one CGU, debuginfo 2 | 1,148.9 | 1,645.1 | +43.2% | 11.9 |

The diagnostic is not a benchmark. The combined trace and time-pass mode
changed wall time substantially. Its trace file reached 43.5 MiB for the
synthetic fixture and 99.9 MiB for one public-crate compile.

The JSON `-Ztime-passes` event named `LLVM_passes` also did not equal the sum
of nested trace events. Under multi-CGU and LTO modes it was especially
unsuitable as a total-pass-work estimate. It remains a coarse region timer.

## Upstream controls

- rustc maps `PreLinkNoLTO`, `PreLinkThinLTO`, `PreLinkFatLTO`, `ThinLTO`, and
  `FatLTO` to distinct LLVM pipeline stages.
- LLVM 23 removed separate Os and Oz pipelines. Rustc maps both to O2 and
  relies on `optsize` or `minsize` attributes.
- `-Zself-profile` can register LLVM pass and analysis callbacks with pass and
  IR names, while rustc separately records module optimization, codegen, and
  object-emission regions.
- The rustc developer guide treats `-Ztime-llvm-passes`,
  `-Zllvm-time-trace`, raw IR, `opt`, `llc`, native profilers, and rustc-perf
  as complementary tools.
- `cargo-llvm-lines` measures LLVM IR volume and generic copies but does not
  measure pass cost.
- rust-lang/rust issue `#157302` recorded an LLVM 22 SLP regression from
  minutes to hours on one x86 AVX-512 shape. Later comments report it fixed in
  LLVM 22.1.6. The incident supports exact-version fixtures, not disabling SLP
  generally.
- rust-lang/rust issue `#102709` records a case where vectorized, heavily
  unrolled output was slower than a scalar size-optimized loop.

## Limitations

- Primary evidence is one Windows MSVC system.
- The synthetic fixture is intentionally shaped and cannot establish
  ecosystem prevalence.
- METIS-CORE is one public library and had no final executable runtime control
  in this experiment.
- Direct root-crate replay excludes Cargo scheduling and dependency
  compilation by design.
- Trace events are unstable LLVM internals, nested, and observer-affected.
- Function names may disappear through inlining or change with mangling and
  toolchain revision.
- Peak RSS samples the process tree at intervals and is not a full allocator
  profile.
- No profile-guided optimization, sanitizer, target-feature, or non-x86 target
  was measured.
- Linker cost remains PERF-Q29. Debug and object emission remain PERF-Q28.
- A pass timing identifies cost, not whether the transformation improved
  runtime, size, or downstream code quality.
