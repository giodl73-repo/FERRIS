# EXP-01: rustc Invocation Floor and Metadata Demand

Date: 2026-08-08
Question: PERF-Q08
Status: Complete

## Purpose

Separate:

1. executable and process-launch cost;
2. compiler session, crate-root parsing, expansion, and analysis;
3. sysroot metadata registration;
4. unused, selectively used, and broadly enumerated dependency metadata;
5. dependency-count cost;
6. metadata-only output from minimal code generation.

The experiment uses direct rustc invocations rather than Cargo so Cargo graph,
freshness, scheduling, and lock time do not enter the measured process.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- NTFS local workspace;
- stable rustc `1.95.0 (59807616e 2026-04-14)`;
- stable LLVM `22.1.2`;
- nightly rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`;
- nightly LLVM `23.1.0`;
- host `x86_64-pc-windows-msvc`.

The reported distributions are warm operating-system-cache results. They do
not represent first invocation after boot, cold executable pages, or another
platform.

## Measurement correction

An initial pilot passed:

```text
-C incremental=off
```

That did not disable incremental compilation. `-C incremental` takes a
directory path, so rustc created and maintained a directory named `off`. The
pilot showed approximately 54 ms in
`incr_comp_finalize_session_directory` for the tiny crate.

The pilot was discarded. Final direct-rustc commands omit `-C incremental`,
which leaves incremental compilation disabled by default. Cargo experiments
should use `CARGO_INCREMENTAL=0` when they need an explicit override.

## Fixture

The generated source set contains:

- one tiny ordinary library;
- the same library with `#![no_std]`;
- one 10,000-item dependency whose `.rmeta` is 10,949,700 bytes;
- a consumer that receives the large dependency through `--extern` but does
  not reference it;
- a consumer that references one type from the large dependency;
- a consumer that publicly glob-reexports the large dependency;
- 32 dependencies of approximately 2,030 bytes each;
- consumers that reference 1, 8, or 32 of those tiny dependencies.

The many-dependency metadata totals approximately 64,950 bytes. It is designed
to separate per-crate registration and lookup from total metadata bytes.

## Stable primary distribution

A Node.js harness launched the compiler with `child_process.spawn` and measured
from immediately before spawn until process close. Each scenario had three
declared warm-ups and 30 measured repetitions. Scenarios were rotated between
rounds. Outputs were overwritten at fixed paths.

The compiler path was obtained with:

```powershell
rustup which rustc
```

A temporary `RUSTC_WRAPPER` confirmed that Cargo passes the direct toolchain
compiler path to wrappers:

```text
<rustup-home>\toolchains\stable-x86_64-pc-windows-msvc\bin\rustc.exe
```

Therefore the direct toolchain results represent Cargo's rustc process more
closely than launching the `rustc` rustup proxy from a terminal.

| Scenario | Median | MAD | Relative MAD |
| --- | ---: | ---: | ---: |
| rustup proxy `rustc --version` | 94.44 ms | 4.92 ms | 5.2% |
| direct rustc `--version` | 54.04 ms | 2.08 ms | 3.8% |
| direct `--print target-libdir` | 58.29 ms | 3.10 ms | 5.3% |
| tiny std, metadata only | 84.30 ms | 3.98 ms | 4.7% |
| identical std repeat control | 85.78 ms | 5.17 ms | 6.0% |
| tiny `no_std`, metadata only | 79.84 ms | 4.01 ms | 5.0% |
| tiny std, rlib output | 99.84 ms | 6.52 ms | 6.5% |
| large extern unused | 88.32 ms | 5.67 ms | 6.4% |
| one large-dependency item used | 101.05 ms | 4.90 ms | 4.8% |
| large dependency glob-reexported | 128.29 ms | 4.61 ms | 3.6% |
| one tiny dependency | 83.92 ms | 3.74 ms | 4.5% |
| eight tiny dependencies | 93.20 ms | 6.38 ms | 6.8% |
| 32 tiny dependencies | 107.20 ms | 6.33 ms | 5.9% |

Every final series remained below the contract's 10% relative-MAD threshold.

### Interpretation

- The direct `--version` path took 54.04 ms before parsing or analysis.
- A complete tiny metadata-only std compilation took 84.30 ms. The executable
  and early-command lower bound was therefore a material share of the tiny
  invocation, but it is not a complete attribution of compiler startup.
- The rustup proxy added about 40 ms to manual `rustc --version` on this host.
  Cargo did not pay that proxy cost for each crate.
- The two identical std controls differed by 1.48 ms in their medians. Deltas
  of that scale are treated as local noise.
- Adding an unused 10.95 MB metadata file through `--extern` did not produce a
  size-proportional cost.
- Referencing one item increased the median by about 13 ms over the unused
  case. Glob-reexporting the dependency increased it by another 27 ms.
- Moving from one to 32 tiny dependencies added about 23 ms despite only about
  65 KB of generated dependency metadata.
- Emitting a tiny rlib added about 16 ms over metadata-only output. This did
  not invoke an external executable linker and is not a binary-link result.

## Nightly phase boundaries

The direct nightly compiler path was measured with three warm-ups and 15
repetitions.

| Boundary | Median | MAD |
| --- | ---: | ---: |
| `--version` | 53.70 ms | 3.27 ms |
| `-Z parse-crate-root-only` | 57.76 ms | 4.10 ms |
| `-Z no-analysis` | 71.10 ms | 3.84 ms |
| std metadata | 80.77 ms | 4.09 ms |
| `no_std` metadata | 79.86 ms | 5.38 ms |
| one large-dependency item | 99.65 ms | 8.36 ms |
| large dependency glob | 126.27 ms | 7.06 ms |

On this warmed Windows host:

- opening and parsing the tiny crate root added about 4 ms over the early
  version path;
- expansion without analysis added about 13 ms over parse-root-only;
- full metadata output added about 10 ms over expansion-only.

These are boundary differences, not a claim that each interval maps to one
exclusive internal phase.

## Nightly diagnostic profiles

Separate one-run diagnostics used:

```powershell
rustc +nightly <input> -Z time-passes -Z time-passes-format=json
rustc +nightly <input> -Z self-profile=<path> `
  -Z self-profile-events=default
summarize summarize <profile>.mm_profdata
```

Query arguments were deliberately excluded. A pilot with
`self-profile-events=default,args` caused material metadata decoding to format
query keys and was discarded as an observer-effect risk.

Selected self-profile results:

| Scenario | Profiled CPU time | Selected evidence |
| --- | ---: | --- |
| Parse root only | 1.14 ms | `parse_crate` 0.59 ms |
| Expand without analysis | 13.16 ms | 13 crates registered; `metadata_register_crate` 6.19 ms |
| Std metadata | 23.63 ms | 13 crates registered; `metadata_register_crate` 6.08 ms |
| `no_std` metadata | 20.25 ms | 2 crates registered; `metadata_register_crate` 0.13 ms |
| One large item | 53.86 ms | metadata registration and module-child decoding visible |
| Large glob | 74.61 ms | metadata decode, expansion, and metadata writing visible |

The self-profile totals are not end-to-end wall time. The parse-root boundary
had a 57.76 ms external median while its separate self-profile run contained
1.14 ms of profiled CPU events. Process creation, image loading, early option
and session work, profiler setup and output, uninstrumented compiler work, and
operating-system scheduling cannot be assigned from that difference alone.

## Source correspondence

The nightly source revision
`1a98b1e135b254f209c67d447b6d8bcd56a859e0` shows:

- `rustc_driver_impl::run_compiler` expands and parses arguments, builds
  session options, constructs the interface configuration, and only then
  enters the compiler;
- the driver supports explicit stop boundaries after crate-root parsing and
  after expansion;
- crate metadata uses `LazyValue`, `LazyArray`, and `LazyTable`;
- lazy tables provide random access without eagerly decoding every value;
- crate location recursively discovers transitive metadata because public
  types can expose transitive dependency types.

Sources:

- [`rustc_driver_impl`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_driver_impl/src/lib.rs)
- [`rmeta` lazy structures](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/rmeta/mod.rs)
- [metadata decoder](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/rmeta/decoder.rs)
- [crate reader](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/creader.rs)
- [crate locator](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/locator.rs)

## Limitations

- One Windows host and warm filesystem/executable state.
- The version path is an early-process lower bound, not a direct measurement
  of every initialization step used by compilation.
- The synthetic metadata shape is not representative of all real crates.
- Generated public items can stress export enumeration differently from
  traits, macros, generics, MIR, and proc-macro metadata.
- Stable and nightly compilers are different revisions and LLVM versions.
- Self-profile and time-passes are separate diagnostic runs with substantial
  observer effects on these sub-150-ms workloads.
- No native process profiler, page-fault counter, CPU counter, or cold-boot
  experiment was used.
- No Linux or macOS result is available.
- The rlib control does not measure executable linking.
- The experiment did not modify rustc, Cargo, rustup, or upstream fixtures.

## Retained evidence

The private experiment record retains:

- source and metadata generators;
- 30-sample stable distributions;
- 15-sample nightly boundary distributions;
- exact commands and compiler paths;
- generated artifact sizes;
- time-passes JSON;
- raw measureme profiles and `summarize` output;
- the Cargo wrapper compiler-path check;
- the discarded incremental and query-argument pilots.

No upstream issue, comment, branch, or pull request was created.
