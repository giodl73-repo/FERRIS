# EXP-01: Codegen-Unit Partition and Reuse Matrix

Date: 2026-08-09
Question: PERF-Q25
Environment: Windows 11 Enterprise Insider Preview 10.0.26310, NTFS
CPU: Intel Core i7-12800HX, 16 cores, 24 logical processors
Memory: 31.7 GiB
Rust: rustc 1.99.0-nightly (1a98b1e13 2026-08-07)
LLVM: 23.1.0

## Purpose

This experiment separates questions that one aggregate build duration cannot:

1. How many CGUs are requested, initially formed, and actually emitted?
2. How does inline-copy placement grow as more CGUs survive merging?
3. Which CGU work products survive an unchanged build, a local body edit, a
   new generic reference, or a new module?
4. Do more CGUs reduce cold makespan, and what CPU and memory do they consume?
5. How do local ThinLTO, disabled local ThinLTO, and explicit whole-graph
   ThinLTO change compile cost, executable size, and runtime?
6. Does a small source edit preserve unrelated item-to-CGU assignments?

The fixture is synthetic. It is intentionally module-heavy and exposes
partition behavior; it does not establish a portfolio-wide profile setting.

## Fixture

The generated executable contained:

- 32 source modules;
- 48 non-generic leaf functions per ordinary module;
- one four-times-larger module;
- one generic instance per module;
- one shared `#[inline(always)]` helper;
- 1,860 development mono items in the baseline;
- an externally timed compile and an internally timed runtime kernel.

Controlled edits were:

| Edit | Change |
|---|---|
| Unchanged | Compile identical source into the same incremental directory |
| Local | Change one constant in one leaf function in module 16 |
| Generic | Add 64 root-level concrete calls to the shared generic function |
| Module add | Add one new ordinary source module |

The generic and module-add edits deliberately tested partition-shape changes,
not only body invalidation.

## Evidence surfaces

The harness used:

- repeated external wall time, CPU time, and process-family peak RSS;
- `-Ztime-passes -Ztime-passes-format=json`;
- `-Zprint-mono-items=yes`;
- `-Zhuman-readable-cgu-names=yes`;
- `-Csave-temps=yes`;
- object and executable byte counts;
- incremental work-product content hashes across rustc session generations;
- 15 rotating-order executable runtime samples per configuration.

Representative partition diagnostic:

```powershell
rustc +nightly fixture.rs --edition=2024 --crate-name cgu_fixture `
  --emit=link --out-dir out -Csave-temps=yes `
  -Copt-level=0 -Ccodegen-units=16 -Cdebuginfo=0 `
  -Csymbol-mangling-version=v0 `
  -Zhuman-readable-cgu-names=yes `
  -Zprint-mono-items=yes `
  -Ztime-passes=yes -Ztime-passes-format=json
```

Representative incremental build:

```powershell
rustc +nightly fixture.rs --edition=2024 --crate-name cgu_fixture `
  -Copt-level=0 -Ccodegen-units=16 -Cdebuginfo=0 `
  -Cincremental=<isolated-cache> `
  -Ztime-passes=yes -Ztime-passes-format=json `
  -o fixture.exe
```

Diagnostic runs were separate from primary timing. A pilot using `--emit=obj`
with no explicit CGU count silently selected one CGU because object output is
incompatible with multiple default CGUs as one output. The final diagnostic
used link output plus saved temporaries. This is a measurement constraint, not
a compiler-performance finding.

## Current partitioning control

The pinned compiler defaults to a maximum of 16 CGUs without incremental
compilation and 256 with incremental compilation. These are maxima, not
promises.

Current partitioning:

1. places non-generic items by source module;
2. places generic items in separate volatile module partitions only for
   incremental compilation;
3. copies reachable `#[inline]` items into every consuming CGU;
4. merges until the requested maximum is met, preferring overlap with the
   smallest retained large CGU;
5. for non-incremental defaults only, keeps merging CGUs below an estimated
   size of 1,800;
6. internalizes symbols where cross-CGU use permits it;
7. uses source-derived composite names for merged incremental CGUs and
   size-ranked numeric names for non-incremental CGUs.

An incremental CGU is the backend work-product reuse unit. It is opaque after
formation: a changed CGU is regenerated rather than patched function by
function.

## Requested maximum versus actual CGUs

| Configuration | Requested maximum | Actual CGUs | Mono placements | Shared inline placements | Saved object KiB |
|---|---:|---:|---:|---:|---:|
| Dev default, non-incremental | 16 default | 4 | 1,872 | 4 | 852.3 |
| Dev explicit 1 | 1 | 1 | 1,860 | 1 | 840.2 |
| Dev explicit 4 | 4 | 4 | 1,872 | 4 | 852.5 |
| Dev explicit 16 | 16 | 16 | 1,920 | 16 | 865.0 |
| Dev explicit 32 | 32 | 32 | 1,983 | 32 | 878.5 |
| Dev explicit 64 | 64 | 63 | 1,987 | 33 | 888.9 |
| Dev incremental default | 256 default | 68 | 1,987 | 33 | 891.5 |
| Dev incremental explicit 16 | 16 | 16 | 1,920 | 16 | 865.9 |
| Dev incremental explicit 64 | 64 | 64 | 1,987 | 33 | 890.3 |
| Release default | 16 default | 5 | 1,830 | 0 | 394.6 |
| Release explicit 1 | 1 | 1 | 1,826 | 0 | 351.1 |
| Release explicit 16 | 16 | 16 | 1,844 | 0 | 411.1 |
| Release explicit 32 | 32 | 32 | 1,860 | 0 | 416.3 |

The non-incremental default collapsed to four development CGUs and five
release CGUs because its initial module partitions were small enough for the
minimum-size merge. The incremental default retained 68 CGUs because the
minimum-size rule does not apply there and stable/volatile partitions are
separate.

Explicit 64 produced only 63 non-incremental CGUs because only 63 initial
partitions existed. A requested count cannot manufacture additional useful
partitions.

From explicit one to explicit 64 development CGUs, placement count increased
6.8% and saved object bytes increased 5.8%. The four duplicated item identities
included the shared inline path; each could be present in many CGUs.

## Cold compile matrix

Every row has five samples. Wall time is the primary distribution. Diagnostic
phase values were collected in the same run and remain secondary.

### Development, optimization level 0

| Requested CGUs | Wall median, ms | MAD, ms | CPU median, ms | Peak RSS median, MiB |
|---:|---:|---:|---:|---:|
| Default | 688.2 | 69.7 | 671.9 | 128.6 |
| 1 | 719.1 | 3.6 | 593.8 | 127.0 |
| 2 | 690.1 | 23.9 | 609.4 | 127.1 |
| 4 | 1,042.5 | 63.0 | 734.4 | 128.9 |
| 8 | 799.3 | 11.6 | 906.2 | 129.6 |
| 16 | 870.4 | 32.5 | 1,031.2 | 128.2 |
| 32 | 896.8 | 31.6 | 1,046.9 | 131.1 |
| 64 | 667.1 | 40.2 | 1,000.0 | 129.8 |

The direction was not monotonic. More CGUs generally consumed more total CPU,
but wall time moved irregularly. The explicit-four samples were bimodal and
the default row had relative MAD above 10%; neither supports a promoted
fine-grained ranking.

### Release and LTO controls

| Mode | CGUs | Wall median, ms | MAD, ms | CPU median, ms | Peak RSS, MiB | Executable bytes |
|---|---:|---:|---:|---:|---:|---:|
| Automatic local ThinLTO when applicable | Default | 841.7 | 6.6 | 1,515.6 | 136.8 | 240,640 |
| Automatic local ThinLTO when applicable | 1 | 1,723.1 | 314.5 | 1,468.8 | 132.6 | 239,616 |
| Automatic local ThinLTO when applicable | 4 | 919.4 | 65.6 | 1,546.9 | 138.2 | 240,640 |
| Automatic local ThinLTO when applicable | 16 | 778.8 | 28.2 | 2,390.6 | 141.0 | 240,640 |
| Automatic local ThinLTO when applicable | 32 | 797.0 | 11.7 | 2,531.2 | 145.8 | 240,640 |
| Local ThinLTO disabled | 1 | 1,405.3 | 107.8 | 1,343.8 | 134.0 | 239,616 |
| Local ThinLTO disabled | 16 | 804.3 | 51.2 | 1,890.6 | 142.5 | 242,176 |
| Local ThinLTO disabled | 32 | 741.5 | 12.1 | 2,250.0 | 171.7 | 242,176 |
| Explicit ThinLTO | 1 | 1,724.8 | 71.0 | 3,250.0 | 157.5 | 238,080 |
| Explicit ThinLTO | 16 | 2,319.9 | 497.0 | 6,218.8 | 165.6 | 238,592 |

Multiple release CGUs shortened the measured wall path while consuming more
aggregate CPU. The one-CGU automatic-mode row was unstable and cannot support
an exact percentage claim.

Explicit whole-graph ThinLTO was a different workload. Its 16-CGU median was
2.98 times the automatic 16-CGU median, but its relative MAD exceeded 20%.
The stable conclusion is only that explicit ThinLTO dominated this small
fixture and must not be treated as the same mode as local ThinLTO.

With local ThinLTO disabled, 32 CGUs used 28.2% more peak RSS than one CGU.
Explicit ThinLTO also raised peak RSS. More backend parallelism is therefore a
memory-budget decision as well as a makespan decision.

## Incremental work-product reuse

Each row is the median of three fresh baseline-plus-edit pairs. `Reused` counts
new-session object-like work products whose bytes matched a prior-session work
product. It is content evidence, not rustc's currently incomplete
post-ThinLTO reuse report.

### Unchanged and one-function edit

| CGUs | Unchanged wall, ms | Unchanged reused | Local wall, ms | Local reused |
|---:|---:|---:|---:|---:|
| Default, actual 68 | 1,438.5 | 68/68 | 1,430.9 | 67/68 |
| 1 | 584.2 | 1/1 | 771.4 | 0/1 |
| 4 | 645.1 | 4/4 | 652.1 | 3/4 |
| 8 | 704.2 | 8/8 | 1,033.7 | 7/8 |
| 16 | 733.2 | 16/16 | 788.7 | 15/16 |
| 32 | 836.7 | 32/32 | 790.4 | 31/32 |
| 64 | 926.5 | 64/64 | 891.0 | 63/64 |

The one-function edit preserved every unrelated placement in separate
diagnostic maps. Every explicit multi-CGU configuration regenerated exactly
one work product.

Reuse fraction alone did not predict latency. The incremental default reused
67 of 68 work products after the local edit but took 2.19 times the four-CGU
median. Loading, proving, copying, linking, and managing many work products
outweighed the finer invalidation boundary for this fixture.

The unchanged default was 2.46 times the one-CGU unchanged median despite
reusing every work product. The best cache hit is not a zero-cost event.

### Partition-shape edits

| CGUs | Generic wall, ms | Generic reused | Module-add wall, ms | Module-add reused |
|---:|---:|---:|---:|---:|
| Default, actual 68 | 984.7 | 34/68 | 948.6 | 66/69 |
| 1 | 886.1 | 0/1 | 831.6 | 0/1 |
| 4 | 794.5 | 0/4 | 759.7 | 0/4 |
| 8 | 862.6 | 0/8 | 748.1 | 0/8 |
| 16 | 1,039.0 | 0/16 | 769.6 | 1/16 |
| 32 | 1,104.7 | 0/32 | 1,255.2 | 1/32 |
| 64 | 1,111.8 | 30/64 | 1,209.1 | 57/64 |

Adding generic references enlarged a volatile root partition. At 4 and 8
CGUs, every composite merged name changed. At 16 and 32, 89.2% of unrelated
common mono items changed placement names. All 4 through 32 work products were
new even though most source bodies were untouched.

Adding one module changed 100% of unrelated placements at 4 and 8 CGUs and
89.6% at 16 and 32. At 64, most initial module partitions survived without
merging, so 57 of 64 prior work products remained reusable.

This is a discontinuity: merging reduces the number of work products and can
reduce duplicate inline copies, but a small topology change can rename and
recompose otherwise unrelated merged units.

## Runtime control

All binaries produced the same checksum. Fifteen samples rotated and reversed
execution order.

| Mode | CGUs | Runtime median, ms | MAD, ms | Executable bytes |
|---|---:|---:|---:|---:|
| Automatic local ThinLTO when applicable | Default | 4.728 | 0.194 | 240,640 |
| Automatic local ThinLTO when applicable | 1 | 4.906 | 0.132 | 239,616 |
| Automatic local ThinLTO when applicable | 4 | 4.744 | 0.156 | 240,640 |
| Automatic local ThinLTO when applicable | 16 | 4.832 | 0.144 | 240,640 |
| Automatic local ThinLTO when applicable | 32 | 4.771 | 0.082 | 240,640 |
| Local ThinLTO disabled | 1 | 4.813 | 0.109 | 239,616 |
| Local ThinLTO disabled | 16 | 4.669 | 0.086 | 242,176 |
| Local ThinLTO disabled | 32 | 4.776 | 0.194 | 242,176 |
| Explicit ThinLTO | 1 | 4.833 | 0.153 | 238,080 |
| Explicit ThinLTO | 16 | 4.931 | 0.263 | 238,592 |

The distributions overlap. This fixture does not establish that one CGU,
local ThinLTO, or explicit ThinLTO improves runtime. It also disproves the
shortcut that one CGU must be faster at runtime.

Executable size ranged from 238,080 to 242,176 bytes, a 1.7% span. That small
range does not justify the much larger compile-cost differences.

## External controls

The current compiler source and tests establish:

- CGU partitioning is primarily an incremental-compilation mechanism;
- mono items are grouped by source module and then merged;
- generic instances are volatile only in incremental mode;
- `#[inline]` local copies can appear in every consuming CGU;
- merged incremental names encode their source partition composition;
- ThinLTO imports can invalidate a consumer CGU after the source CGU changes;
- rustc's post-ThinLTO reuse test reporting remains inaccurate in issue
  `#119076`.

Upstream optimization attempts also show that intuitive heuristics are not
enough:

- a 2023 greedy balancing PR made estimated CGU sizes more even but regressed
  primary rustc-perf instruction counts by 2.8% on average;
- a 2025 shim-partitioning experiment produced primary regressions averaging
  4.6% despite improvements on some workloads;
- a 2026 inline-copy deduplication prototype improved a synthetic stress case
  and `syn`, but had no effect on most crates and was closed for more research;
- the open scheduling issue notes that the current size estimate is not an
  accurate proxy for LLVM time.

## Limitations

- One synthetic executable on x86_64-pc-windows-msvc cannot determine Cargo
  profile defaults for real repositories.
- Antivirus, indexing, power behavior, and unrelated system load were not
  fully controlled. Several rows were bimodal or exceeded 10% relative MAD.
- Process-family RSS is sampled and may miss short-lived peaks.
- Saved object bytes include mode-specific temporary artifacts and are
  comparable only within a like-for-like row family.
- Time-pass values are diagnostic aggregates, not the primary benchmark.
- The runtime kernel does not represent all inlining, vectorization, cache,
  branch, or code-layout-sensitive applications.
- Direct rustc excludes Cargo graph scheduling and simultaneous crate builds.
- Content-identical copied work products provide conservative reuse evidence;
  they do not distinguish pre-LTO from post-LTO reuse.

## Reproduction boundary

The retained harness regenerates the fixture, runs the partition, cold,
incremental, stability, and runtime matrices, writes summarized JSON, and
removes disposable sources and compiler outputs.

The result is reproducible only with the recorded toolchain and host class.
Nightly diagnostic names and output formats are not stable APIs.
