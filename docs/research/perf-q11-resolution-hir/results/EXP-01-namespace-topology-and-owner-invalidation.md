# EXP-01: Namespace Topology and Owner Invalidation

Date: 2026-08-08
Question: PERF-Q11
Status: Complete

## Purpose

Measure name-resolution and HIR-lowering cost across:

1. flat item definitions;
2. local bindings and body paths;
3. named imports;
4. qualified item paths;
5. module count;
6. glob re-export depth and propagated bindings;
7. frontend job count;
8. body, import, visibility, module, and macro edits;
9. ambiguous, private, and unresolved names.

The experiment separates stable complete metadata compilation from nightly
root, no-analysis, time-pass, self-profile, and input-stat diagnostics.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- local NTFS workspace;
- stable rustc `1.95.0 (59807616e 2026-04-14)`;
- stable LLVM `22.1.2`;
- nightly rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`;
- nightly LLVM `23.1.0`;
- host `x86_64-pc-windows-msvc`.

All promoted distributions are warm operating-system-cache results using the
direct toolchain compiler executable.

## Fixture families

| Fixture | Shape | Source bytes |
| --- | --- | ---: |
| Tiny | One function | 28 |
| Flat items | 10,000 flat unit structs | 208,890 |
| Body bindings | One function with 10,000 local bindings and paths | 307,820 |
| Named imports | 5,000 definitions and 5,000 public named imports | 306,687 |
| Qualified paths | 10,000 constants with two qualified paths each | 499,197 |
| Modules | 100 inline modules with 100 definitions each | 260,090 |
| Glob chain | 1,000 definitions re-exported through 100 glob layers | 28,591 |

The glob-chain fixture intentionally models pathological namespace
propagation. It is not a recommended public API.

## Stable complete compilation

Stable rustc emitted metadata. Each fixture had three warm-ups and 30
round-robin repetitions.

| Fixture | Median | MAD | Relative MAD |
| --- | ---: | ---: | ---: |
| Tiny | 84.44 ms | 12.06 ms | 14.3% |
| Flat items | 253.55 ms | 16.72 ms | 6.6% |
| Body bindings | 210.40 ms | 15.03 ms | 7.1% |
| Named imports | 221.57 ms | 16.91 ms | 7.6% |
| Qualified paths | 529.31 ms | 46.43 ms | 8.8% |
| Modules | 261.25 ms | 9.85 ms | 3.8% |
| Glob chain | 3,055.14 ms | 193.61 ms | 6.3% |

The tiny series is retained as a noisy lower boundary rather than a promoted
comparison. Every non-tiny stable series remained below the contract's 10%
relative-MAD threshold.

The 28.6 KB glob chain was roughly 5.8 times slower than the 499 KB qualified
path fixture and roughly 12 times slower than the similarly item-sized flat
and module fixtures. Source size and AST item count did not explain its cost.

## Root and no-analysis boundaries

Nightly root and no-analysis boundaries used 30 repetitions.

| Fixture | Root parse | No analysis |
| --- | ---: | ---: |
| Tiny | 55.97 ms | 89.30 ms |
| Flat items | 60.18 ms | 132.13 ms |
| Body bindings | 65.53 ms | 110.70 ms |
| Named imports | 64.10 ms | 145.87 ms |
| Qualified paths | 74.97 ms | 148.22 ms |
| Modules | 62.20 ms | 134.06 ms |
| Glob chain | 59.57 ms | 1,923.45 ms |

Several root series and the glob no-analysis series exceeded 10% relative MAD
and remain diagnostic. The decisive boundary is still visible: the glob
fixture parsed like the other roots, then became expensive during expansion
and resolution work before semantic analysis.

`-Z no-analysis` is not a pure expansion timer. The current
`configure_and_expand` pipeline invokes `resolver.resolve_crate` after macro
expansion, so import finalization, effective visibility computation, and late
resolution occur before the no-analysis stop.

## Resolution subphases

Thirty nightly time-pass repetitions produced these diagnostic medians:

| Fixture | `finalize_imports` | Effective visibility | Late resolution | `resolve_crate` |
| --- | ---: | ---: | ---: | ---: |
| Tiny | 0.01 ms | 0.01 ms | 0.06 ms | 0.56 ms |
| Flat items | 1.38 ms | 1.51 ms | 8.52 ms | 13.01 ms |
| Body bindings | 0.01 ms | 0.01 ms | 7.26 ms | 8.73 ms |
| Named imports | 5.27 ms | 2.78 ms | 6.24 ms | 17.80 ms |
| Qualified paths | 0.61 ms | 1.09 ms | 21.48 ms | 26.04 ms |
| Modules | 1.14 ms | 2.93 ms | 8.66 ms | 15.39 ms |
| Glob chain | 343.71 ms | 1,130.96 ms | 0.74 ms | 1,665.31 ms |

Time-pass wall series were more variable and slower than uninstrumented stable
compilation. These values identify causal regions; they are not optimization
claims.

For the glob chain, effective visibility traversal dominated import
finalization, and late expression/type/path resolution was negligible. The
same `resolve_crate` label therefore covers materially different algorithms.

## Self-profile and HIR topology

Five full-event self-profiles per fixture were summarized. They are
observer-affected diagnostics.

| Fixture | Late-resolution self time | `lower_to_hir` self time | `lower_to_hir` misses |
| --- | ---: | ---: | ---: |
| Tiny | 0.07 ms | 0.20 ms | 4 |
| Flat items | 9.32 ms | 32.47 ms | 20,003 |
| Body bindings | 7.00 ms | 9.35 ms | 4 |
| Named imports | 8.73 ms | 26.28 ms | 15,004 |
| Qualified paths | 21.02 ms | 34.99 ms | 10,204 |
| Modules | 10.27 ms | 29.54 ms | 20,103 |
| Glob chain | 1.08 ms | 2.88 ms | 2,204 |

`-Z input-stats` reported:

| Fixture | AST records | HIR records |
| --- | ---: | ---: |
| Flat items | 10,008 | 20,008 |
| Body bindings | 50,015 | 60,018 |
| Named imports | 20,009 | 30,010 |
| Qualified paths | 70,109 | 110,210 |
| Modules | 10,108 | 20,208 |
| Glob chain | 1,409 | 2,610 |

The body fixture had three times as many HIR records as the flat-item fixture
but only four `lower_to_hir` misses and less than one third of its lowering
self time. HIR records, item count, and owner-query count are distinct cost
dimensions.

The qualified-path fixture had the highest late-resolution and HIR-lowering
times among the ordinary-size fixtures. Its 20,000 qualified paths and 10,101
item owners combined namespace lookup with per-owner lowering and later
semantic work.

## Glob propagation scaling

Scaling fixtures varied chain depth and base bindings. Stable and no-analysis
measurements used three warm-ups and 20 repetitions.

### Fixed 1,000 bindings

| Layers | Propagated bindings | Stable median | No-analysis median |
| ---: | ---: | ---: | ---: |
| 10 | 10,000 | 143.71 ms | 95.92 ms |
| 25 | 25,000 | 211.07 ms | 158.26 ms |
| 50 | 50,000 | 709.92 ms | 414.34 ms |
| 75 | 75,000 | 1,849.16 ms | 904.88 ms |
| 100 | 100,000 | 3,435.13 ms | 1,582.88 ms |

### Fixed 100 layers

| Base bindings | Propagated bindings | Stable median | No-analysis median |
| ---: | ---: | ---: | ---: |
| 100 | 10,000 | 215.49 ms | 139.05 ms |
| 250 | 25,000 | 624.34 ms | 287.87 ms |
| 500 | 50,000 | 899.01 ms | 634.94 ms |
| 1,000 | 100,000 | 3,435.13 ms | 1,582.88 ms |

Stable series were below 10% relative MAD except the 25-layer case at 13.9%.
No-analysis series were below 10% except 25 layers at 10.5% and 500 bindings
at 11.4%.

Cost grew faster than linearly over the measured upper range. Propagated
binding count was useful but not sufficient: 10 layers times 1,000 bindings
and 100 layers times 100 bindings both propagate 10,000 bindings, yet their
stable medians were 143.71 and 215.49 ms. Dependency depth and repeated
visibility/import processing both matter.

## Frontend jobs

Nightly metadata runs compared default frontend jobs with eight jobs. Thirty
repetitions produced:

| Fixture | Default | Eight jobs |
| --- | ---: | ---: |
| Flat items | 304.30 ms | 317.70 ms |
| Body bindings | 212.55 ms | 228.27 ms |
| Glob chain | 1,650.91 ms | 1,574.21 ms |

The flat and body series remained below 10% relative MAD and showed no
speedup. Glob series remained above 10% relative MAD, so their small apparent
difference is inconclusive.

Separate time-pass runs also showed no speedup:

| Fixture | Jobs | Wall | Import finalization | Effective visibility | Resolution |
| --- | ---: | ---: | ---: | ---: | ---: |
| Named imports | 1 | 987.55 ms | 8.87 ms | 4.82 ms | 29.20 ms |
| Named imports | 8 | 1,061.70 ms | 9.08 ms | 5.00 ms | 30.17 ms |
| Glob chain | 1 | 2,541.78 ms | 422.60 ms | 1,530.31 ms | 2,076.12 ms |
| Glob chain | 8 | 3,086.05 ms | 449.79 ms | 1,660.43 ms | 2,375.96 ms |

The named default series remained below 10% relative MAD; named eight-job and
both glob series did not. The result supports only a negative conclusion:
eight jobs did not accelerate these controls.

The compiler source already uses a parallel slice traversal within each batch
of import fixed-point resolution. That does not make dependent import batches,
effective visibility traversal, or late crate walking fully parallel.

## Incremental edit matrix

The incremental fixture contained:

- 3,000 type definitions;
- 1,001 function owners;
- three outline modules;
- one named import;
- one public re-export;
- one macro-generated function.

Each scenario used an independent reused incremental directory. Primary wall
time used 15 repetitions; time passes used 30; self-profile query summaries
used five.

| Scenario | Wall median | MAD | `resolve_crate` | `lower_to_hir` self time |
| --- | ---: | ---: | ---: | ---: |
| Fresh directory | 615.04 ms | 40.30 ms | 6.10 ms | 14.50 ms |
| Untouched source | 622.26 ms | 24.14 ms | 5.98 ms | 14.68 ms |
| Identical root rewrite | 615.15 ms | 24.97 ms | 5.89 ms | 16.11 ms |
| One body literal | 668.67 ms | 48.56 ms | 5.87 ms | 13.18 ms |
| One import target | 704.49 ms | 67.54 ms | 6.08 ms | 14.41 ms |
| One visibility | 732.74 ms | 61.83 ms | 5.90 ms | 13.38 ms |
| One module target | 705.59 ms | 51.05 ms | 5.97 ms | 15.08 ms |
| Macro invocation value | 679.74 ms | 42.18 ms | 5.96 ms | 13.62 ms |

All primary wall series remained below 10% relative MAD.

Every profile reported 7,017 `lower_to_hir` misses, including untouched and
identically rewritten sessions. `resolver_for_lowering_raw`, `index_ast`, and
`lower_to_hir` are `eval_always` queries in the pinned compiler source.
Incremental compilation therefore did not skip resolution, AST indexing, or
owner lowering once rustc ran.

The feedable `hir_owner` boundary still mattered. Aggregate `hir_owner` time
was 26.60 ms for a fresh directory and 1.23 ms for untouched source. The
frontend reconstructed and hashed owners, but unchanged owner results allowed
later query work to remain reusable.

The complete wall medians diverged after edits even though resolution and HIR
lowering remained approximately flat. Visibility, module, import, macro, and
body edits therefore need downstream query and metadata attribution before
their total differences are assigned to name resolution.

Incremental cache loading was material in the reused controls:
`incr_comp_load_dep_graph` and `incr_comp_load_query_result_cache` each took
roughly 20–40 ms in the profiles. This fixture does not demonstrate that a
reused incremental directory is always faster than a fresh one.

## Expected failures

Nightly metadata compilation retained exit status 1 and complete stderr:

| Failure | Median | MAD | Stderr |
| --- | ---: | ---: | ---: |
| Ambiguous glob name | 75.72 ms | 4.57 ms | 2,267 bytes |
| Private re-export | 73.71 ms | 4.28 ms | 615 bytes |
| Unresolved path | 95.01 ms | 4.11 ms | 506 bytes |

These are correctness and diagnostic controls, not fast paths. Replacing
globs, imports, or visibility boundaries can change ambiguity, privacy, lint,
and public API behavior.

## Diagnostics assessment

Current nightly diagnostics expose complementary layers:

- `parse-crate-root-only` gives a source-loading and root-parse boundary;
- `no-analysis` includes expansion and crate resolution but excludes later
  analysis;
- time passes expose import finalization, effective visibility, late
  resolution, and aggregate resolution;
- self-profile exposes `resolver_for_lowering_raw`, `index_ast`,
  `lower_to_hir`, `hir_owner`, HIR item queries, and incremental cache work;
- `input-stats` reports AST and HIR record counts.

They do not provide a durable machine-readable explanation of:

- which import or re-export chain propagated how many bindings;
- fixed-point batch count and unresolved work per batch;
- per-module effective visibility work;
- late-resolution paths or rib depth by owner;
- owner lowering cost joined to source identity;
- whether an edit changed resolver output, owner hashes, or only later work.

FERRIUM can join current diagnostics externally. Any compiler event or
structured-statistics contribution requires maintainer guidance and explicit
approval before upstream activity.

## Limitations

- Synthetic namespace extremes are not representative application APIs.
- Windows scheduling, filesystem, antivirus, and indexing remain host-local
  influences.
- Linux and macOS have not been measured.
- Stable complete and nightly diagnostic runs use different compiler
  revisions.
- Time passes and self-profile materially affect wall time.
- Five-profile medians are diagnostic, not primary performance distributions.
- No dep-graph dump was interpreted as a complete invalidation proof.
- Frontend job controls do not generalize to all module or import shapes.
- The glob scaling matrix is not a formal complexity proof.
- Failed compilations can stop before complete successful work.

## Reproduction evidence

The retained session evidence includes:

- deterministic fixture generators;
- stable, parse, and no-analysis distributions;
- 30-sample time-pass distributions;
- five-run self-profile summaries;
- AST and HIR input statistics;
- glob depth and binding scaling;
- frontend-job controls;
- incremental edit distributions;
- expected failure output;
- exact source sizes, toolchains, commands, and compiler revisions.
