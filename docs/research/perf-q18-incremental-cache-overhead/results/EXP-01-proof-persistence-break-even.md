# EXP-01: Incremental Proof, Persistence, and Reuse Break-Even

Date: 2026-08-08
Question: PERF-Q18
Status: Complete

## Purpose

Measure when rustc's incremental dependency tracking, stable hashing,
serialization, cache loading, cache promotion, and backend work-product reuse
cost more or less than the compiler work they avoid.

The experiment separates four boundaries:

1. direct rustc with incremental compilation disabled;
2. direct rustc metadata compilation with query persistence enabled;
3. direct rustc codegen with backend work-product caching enabled or disabled;
4. Cargo freshness, which can skip rustc before any incremental cache is read.

It also measures cache topology and controlled recovery after failed
compilation or deliberate cache-file damage.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- local NTFS workspace;
- nightly rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`, LLVM `23.1.0`;
- Cargo `1.95.0 (f2d3ce0bd 2026-03-21)`;
- host `x86_64-pc-windows-msvc`;
- rustc-perf source review pinned at
  `58b05b9a296dfb148aa54c9ec61e8890c65a4223`.

Primary latency runs were minimally instrumented. Self-profile runs were
separate diagnostic observations and are not used as the wall-time baseline.

## Fixtures

### Trivial owner scaling

Generated metadata-only crates contained one shared scalar alias and 1, 100,
1,000, 5,000, or 10,000 independent trivial function owners.

Each shape measured:

- incremental disabled on the baseline source;
- incremental cold on the baseline source;
- incremental warm and unchanged;
- incremental disabled and enabled after one equal-width local body edit;
- incremental disabled and enabled after a shared alias type edit.

A separate one-owner fixture placed 5,000 arithmetic operations in one body.
Its local edit changed one operation without changing the following source
offsets.

### Semantic owner scaling

The second matrix made each reusable owner more expensive:

- 1,000 and 5,000 trait-obligation owners;
- 5,000 borrow-checking owners.

Local edits changed one owner. Broad edits changed a shared bound or type used
by every owner.

### Backend work-product control

A 5,000-function crate distributed functions across 16 modules and compiled an
rlib with:

```powershell
rustc fixture.rs --crate-name backend_cache_fixture --crate-type lib `
  --emit link -Ccodegen-units=16 -Cdebuginfo=0 -Copt-level=0
```

Incremental runs were repeated with ordinary backend caching and with the
diagnostic control:

```powershell
-Zdisable-incr-comp-backend-caching=yes
```

The unchanged case reused all codegen units. The local case changed one
equal-width literal in one module. The broad case changed the shared scalar
type.

### Cargo boundary

Two disposable Cargo packages were measured with incremental compilation
enabled and disabled:

- a tiny crate;
- the 5,000-owner trait fixture.

Each seven-run distribution measured cold, true no-op, local-edit, and broad-
edit states. Cargo JSON messages recorded fresh and rebuilt artifacts.

## Protocol

The direct metadata command was:

```powershell
rustc fixture.rs --crate-name consumer --crate-type lib --edition 2024 `
  --emit metadata -o output.rmeta
```

Incremental variants added:

```powershell
-Cincremental=<isolated-scenario-cache>
```

Every edit scenario started from a fresh baseline cache. Disabled edit
controls compiled the edited source without an incremental directory. Primary
and semantic matrices used nine repetitions. Cargo used seven repetitions.
The backend control used five repetitions.

Latency tables report medians. Every retained result also recorded every
sample, MAD, CPU time, peak working set, output hash, and logical and unique
cache size. A wall result with relative MAD above 10% is treated as unstable
and is not used alone for a promoted optimization claim.

## Trivial-owner break-even

Metadata wall medians:

| Shape | Disabled baseline | Incremental cold | Warm unchanged | Disabled local | Incremental local | Disabled broad | Incremental broad |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1 owner | 88.09 ms | 105.14 ms | 120.65 ms | 94.47 ms | 134.45 ms | 91.77 ms | 121.81 ms |
| 100 owners | 117.74 | 122.56 | 150.06 | 118.86 | 175.06 | 120.27 | 180.74 |
| 1,000 owners | 198.34 | 222.04 | 194.11 | 184.55 | 216.62 | 173.22 | 276.36 |
| 5,000 owners | 351.58 | 426.33 | 292.95 | 344.55 | 372.42 | 351.58 | 544.94 |
| 10,000 owners | 575.72 | 725.90 | 481.51 | 555.50 | 629.60 | 625.20 | 1,037.87 |
| One body, 5,000 operations | 211.85 | 230.75 | 185.53 | 209.24 | 282.06 | 230.14 | 266.43 |

Cold incremental medians added 4.1% to 26.1% across the owner-count controls
and 8.9% in the large-body control. Several smaller distributions exceeded
the 10% relative-MAD gate; the stable 5,000- and 10,000-owner controls added
21.3% and 26.1%. The cold runs had to construct and persist the dependency
graph and query cache without receiving prior-session reuse.

Warm unchanged compilation regressed the 1- and 100-owner controls. It reached
approximate parity at 1,000 trivial owners and was 16.7% and 16.4% faster at
5,000 and 10,000 owners. The one-large-body unchanged case was 12.4% faster.

One local edit to a trivial owner did not break even at 5,000 owners and was
still slower at 10,000 owners. The 10,000-owner incremental-local
distribution had relative MAD of 11.9%, so its exact ratio is not promoted.
The direction remained consistent with the 5,000-owner control: proving and
persisting reuse can cost more than recomputing trivial bodies.

The one-large-body local edit was 34.8% slower with incremental compilation.
Owner-granular invalidation required the whole body to run again while still
paying cache overhead.

Broad shared-type edits were slower with incremental compilation because the
semantic work reran and the new graph and cache still had to be written.

## Reusable work per owner determines value

Metadata wall medians for semantic owners:

| Shape | Warm unchanged | Disabled local | Incremental local | Local result | Disabled broad | Incremental broad |
|---|---:|---:|---:|---|---:|---:|
| Trait owners, 1,000 | 196.47 ms | 221.41 ms | 243.84 ms | 10.1% slower | 208.52 ms | 294.88 ms |
| Trait owners, 5,000 | 345.85 | 489.53 | 419.66 | 14.3% faster | 505.98 | 741.67 |
| Borrow owners, 5,000 | 395.40 | 613.77 | 500.30 | 18.5% faster | 612.10 | 874.47 |

The 5,000-owner local edits became worthwhile only when each untouched owner
avoided enough type, trait, MIR, or borrow work. Owner count alone did not
predict the break-even point.

Broad edits remained 42.9% to 46.6% slower because they removed the reusable
semantic frontier while retaining incremental bookkeeping.

## Backend work-product reuse is a separate layer

Five-run rlib medians:

| Edit state | Incremental disabled | Incremental, backend cache | Incremental, backend cache disabled |
|---|---:|---:|---:|
| Unchanged | 1,020.71 ms | 627.90 ms | 853.70 ms |
| One module-local body | 955.84 | 752.36 | 934.01 |
| Shared type | 1,005.72 | 1,371.10 | 1,277.89 |

The unchanged backend-cache distribution had relative MAD of 10.2%, but every
enabled sample remained below every disabled sample. Query and frontend reuse
alone improved the unchanged median by 16.4%; reusing compiled codegen units
increased the median improvement to 38.5%.

For the one-module edit, disabling backend work-product caching left only a
2.3% median improvement. Enabling it improved the median by 21.3%. Backend
reuse therefore accounted for most of the local codegen benefit in this
fixture.

The shared-type edit invalidated the reusable work. Incremental compilation
was 27.1% slower with backend caching disabled and 36.3% slower with it
enabled. Persisting changed work products is not free.

The warm cache was about 44.6 MiB without backend objects and 48.3 MiB with
them. Sixteen object-file identities accounted for the difference in this
fixture.

## Self-profile attribution

One separately profiled 10,000-owner metadata run reported:

| Scenario | Stable result hashing | Dep-graph load | Query-cache file load | Query-result deserialization | Cache promotion | Dep-graph encoding |
|---|---:|---:|---:|---:|---:|---:|
| Incremental cold | 111.13 ms | - | - | - | - | 50.38 ms |
| Warm unchanged | 11.02 | 42.27 ms | 28.17 ms | 42.42 ms | 16.17 ms | 47.06 ms |
| Warm local | 17.28 | 39.40 | 32.10 | 58.03 | <5 ms | 52.53 |
| Warm broad | 67.56 | 52.17 | 29.53 | 45.81 | <5 ms | 62.58 |

The dependency graph contained approximately 530,388 nodes and 2,650,627
edges. Encoding recorded approximately one event per node.

An unchanged compile still loaded the previous graph, memory-mapped the query
cache, deserialized requested values, promoted some green values, constructed
the current graph, and serialized state for the next session. "No provider
work changed" does not mean "the incremental subsystem did no work."

These self-profile wall values are observer-affected and support attribution
only.

## Cache topology and generations

| Shape | Source bytes | Graph nodes | Graph edges | Cold cache | Warm cache, logical | Warm cache, unique |
|---|---:|---:|---:|---:|---:|---:|
| 1 owner | 93 | 441 | 892 | 0.02 MiB | 0.04 MiB | 0.04 MiB |
| 1,000 owners | 52,932 | 53,388 | 265,627 | 2.93 MiB | 5.87 MiB | 5.71 MiB |
| 10,000 owners | 538,932 | 530,388 | 2,650,627 | 31.35 MiB | 62.71 MiB | 61.15 MiB |
| One body, 5,000 operations | 80,093 | 764 | 1,477 | 0.17 MiB | 0.34 MiB | 0.33 MiB |

Owner and query topology predicted cache size much better than source bytes.
The 10,000-owner source was only about 6.7 times larger than the one-body
source, but its cold cache was about 184 times larger.

After the second compile, rustc retained the previous finalized session and
the newly finalized session. A third compile removed the oldest generation
and again left two. Four files were hard-linked forward, but the warm unique
size remained close to twice the cold size.

At 10,000 owners, two-generation logical component totals were approximately:

- `dep-graph.bin`: 52.61 MiB;
- `query-cache.bin`: 6.99 MiB;
- `metadata.rmeta`: 3.11 MiB.

The dependency graph accounted for about 84% of the warm logical cache.

In the steady-state third compile, the 1-, 1,000-, and 10,000-owner fixtures'
two query-cache files had identical content hashes while their dependency-
graph hashes differed. The current implementation still creates the next
result cache through cache promotion and serialization; content sharing is
not assumed to be free or correct without a compiler design and benchmark.

## Cargo freshness boundary

Cargo JSON reported:

| Shape | Incremental | Cold | True no-op | Local edit | Broad edit |
|---|---|---:|---:|---:|---:|
| Tiny | Disabled | 474.47 ms | 183.69 ms | 256.30 ms | 256.83 ms |
| Tiny | Enabled | 473.60 | 147.45 | 289.19 | 321.30 |
| Trait, 5,000 owners | Disabled | 1,728.49 | 241.90 | 755.33 | 801.63 |
| Trait, 5,000 owners | Enabled | 1,617.45 | 205.58 | 695.64 | 1,080.16 |

Every true no-op run reported one fresh artifact and zero rebuilt artifacts.
Cargo did not invoke rustc, so no incremental graph or query cache was loaded.
The no-op wall differences are Cargo-process noise, not incremental compiler
speedups.

Several Cargo edit distributions exceeded 10% relative MAD. Their exact
latency ratios are therefore directional only. The supported conclusions are:

- tiny edit medians did not benefit from incremental compilation;
- the 5,000-owner local median moved in the beneficial direction and the broad
  median moved in the harmful direction, consistent with the stable direct-
  rustc semantic controls;
- enabled target storage grew from 18.36 MiB after cold compilation to
  35.95 MiB after the edited second generation, versus about 0.78 MiB with
  incremental disabled.

## Failure and recovery controls

### Failed compilation

A valid 1,000-owner baseline created the incremental cache. A deliberately
invalid edit exited 1. Restoring the valid source:

- exited successfully;
- produced the same within-mode metadata hash;
- reported no `typeck_root`, `mir_built`, or `mir_borrowck` misses.

The failed compile did not replace the last good finalized generation.

### Truncated dependency graph

Truncating `dep-graph.bin` caused a safe full recomputation:

- exit status 0;
- 1,000 type-check misses;
- 1,000 MIR-construction misses;
- 1,000 borrow-check misses;
- the same within-mode metadata hash.

The loader treated the dep graph as out of date.

### Missing or truncated query cache

Deleting or truncating only `query-cache.bin` while preserving a valid
dependency graph exited 101 on the tested nightly:

```text
failed to load disk-cached value for green node
externally_implementable_items(...)
```

The dep graph had marked a cacheable value green, while the deliberately
damaged result cache no longer contained the required value. Removing the
whole incremental cache restored successful compilation and the expected
within-mode output hash.

This was deliberate mutation of unstable internal files, not an ordinary
workflow failure. The operational recovery boundary is the whole incremental
cache directory, not an individual internal file.

## Artifact identity control

Incremental and non-incremental metadata files had different hashes even when
compiled from semantically equivalent source. The `-Cincremental` setting is
part of the compiler configuration and artifact identity.

Output equality is therefore checked within one mode across unchanged,
failed-then-restored, and recovery scenarios. Cross-mode byte equality is not
used as a correctness oracle.

## Persistence source model

Pinned rustc source:

- [dep-graph and work-product loading](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/load.rs#L44-L118);
- [query-cache loading](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/load.rs#L121-L150);
- [cache promotion and result-cache persistence](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/save.rs#L20-L89);
- [read-only memory-mapped cache files](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/file_format.rs#L83-L150);
- [query cache promotion](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/dep_graph/graph.rs#L1062-L1091);
- [successful-session finalization](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/fs.rs#L293-L340);
- [session generation garbage collection](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_incremental/src/persist/fs.rs#L589-L835);
- [green-node missing-value invariant](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_query_impl/src/plumbing.rs#L155-L178).

The source confirms that:

- dep graph and query-result cache are separate but coordinated persisted
  structures;
- query data is memory-mapped and loaded selectively;
- green disk-cached values may be promoted before the next result cache is
  serialized;
- the save-path comment already identifies promotion work in unchanged builds
  as a potential optimization area;
- successful compilation publishes a finalized session directory;
- garbage collection keeps the most recent prior finalized generation while
  the current session is active.

## Cargo and rustc-perf boundaries

Cargo documents that the profile `incremental` setting maps to
`-C incremental`, stores extra data in `target`, and applies to workspace
members and path dependencies. The development profile defaults to enabled;
the release profile defaults to disabled. `CARGO_INCREMENTAL` and
`build.incremental` can override the profile.

Sources:

- [Cargo profile incremental setting](https://doc.rust-lang.org/cargo/reference/profiles.html#incremental);
- [Cargo profile defaults](https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles);
- [`CARGO_INCREMENTAL`](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-reads);
- [`build.incremental`](https://doc.rust-lang.org/cargo/reference/config.html#buildincremental).

Rustc-perf defines `Full`, `IncrFull`, `IncrUnchanged`, and `IncrPatched`
scenarios. Its collector creates the cold incremental state before unchanged
or patched runs, and benchmark-specific patches usually model a small edit.
This is the correct upstream regression framework.

At the pinned revision, the detailed self-profile API retains cache hits,
cache misses, and incremental query-load time. The reviewed API and storage
model did not retain stable-result hashing time as a first-class detailed
query metric, even though current self-profile data can expose it locally.

Sources:

- [rustc-perf scenarios](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/scenario.rs);
- [scenario execution](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/mod.rs);
- [patch guidance](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md);
- [detailed query fields](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/site/src/self_profile.rs).

## Limitations

- Synthetic fixtures isolate mechanisms but do not predict every real crate.
- Direct metadata compilation excludes linking and most backend work; the
  separate rlib control covers codegen reuse but not a full workspace link.
- Windows process CPU values are coarse and are secondary to wall
  distributions.
- Filesystem cache state, antivirus, and scheduler noise were not fully
  controlled.
- Self-profile wall time is observer-affected.
- The cache corruption probes intentionally violated an unstable internal
  invariant and are not evidence of ordinary cache failure frequency.
- No compression, format change, compiler patch, rustc-perf run, upstream
  issue, or pull request was attempted.
