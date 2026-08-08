# EXP-01: Trait Topology and Solver Reuse

Date: 2026-08-08
Question: PERF-Q13
Status: Complete

## Purpose

Measure trait-solving cost and reuse across:

1. repeated versus unique marker goals;
2. repeated versus unique associated-type projections;
3. supertrait and projection depth;
4. same-named method-candidate width;
5. method-call count;
6. old and globally enabled next solver modes;
7. untouched, rewritten, caller-body, impl-body, impl-set, and shared-bound
   incremental edits;
8. unsatisfied, ambiguous, and recursive-overflow failures.

Stable complete metadata compilation remains primary. Nightly no-analysis,
time-pass, self-profile, solver-mode, incremental, and failure runs are
diagnostic boundaries.

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
direct toolchain compiler executable. Complete compilations emitted metadata.

## Solver modes

The pinned nightly defines:

- default: next solver for coherence, old solver elsewhere;
- `-Znext-solver=no`: old solver everywhere;
- `-Znext-solver=coherence`: explicit default behavior;
- `-Znext-solver=globally`: next solver everywhere;
- bare `-Znext-solver`: equivalent to `globally`.

The same-nightly solver comparison uses explicit `no` and `globally`
endpoints. Stable default is the production-facing complete-time series.

The old solver exposes canonical `evaluate_obligation` query events. The
global next solver uses search-graph caches and fulfillment fast paths; its
goal work is not expected to appear under that old query.

## Primary fixtures

| Fixture | Shape | Source bytes |
| --- | --- | ---: |
| Tiny | One constant function | 49 |
| Repeated marker | 10,000 calls to one concrete `T: Marker` goal | 230,130 |
| Unique marker | 5,000 types, impls, and concrete goals | 356,761 |
| Repeated projection | 10,000 uses of one concrete associated type | 230,179 |
| Unique projection | 2,000 types with unique associated-type uses | 263,462 |
| Supertrait depth | One 200-trait inheritance chain | 33,606 |
| Projection depth | One 128-level associated-type chain | 24,105 |
| Method candidates | 1,000 same-named traits and 1,000 calls | 67,050 |

Generated method results were consumed as expression statements. Warnings
were disabled so diagnostic rendering did not dominate.

## Primary fixture matrix

Each stable, no-analysis, old-solver, and global-next series had three warm-ups
and 30 round-robin repetitions.

| Fixture | Stable | No analysis | Old solver | Global next |
| --- | ---: | ---: | ---: | ---: |
| Tiny | 64.83 ms | 53.44 ms | 64.89 ms | 64.55 ms |
| Repeated marker | 199.39 ms | 82.91 ms | 187.45 ms | 200.26 ms |
| Unique marker | 288.95 ms | 102.70 ms | 282.65 ms | 290.77 ms |
| Repeated projection | 206.52 ms | 84.32 ms | 192.20 ms | 207.05 ms |
| Unique projection | 228.79 ms | 86.78 ms | 232.41 ms | 246.72 ms |
| Supertrait depth 200 | 128.20 ms | 55.52 ms | 126.91 ms | 130.35 ms |
| Projection depth 128 | 113.62 ms | 56.60 ms | 107.86 ms | 110.28 ms |
| Method candidates 1,000 | 2,664.59 ms | 84.68 ms | 2,249.66 ms | 3,048.94 ms |

All primary relative MAD values remained below 5.8% except stable unique
projection at 6.2%, old supertrait depth at 5.9%, and global-next repeated
projection at 5.2%. All remained below the contract's 10% threshold.

No-analysis excludes semantic analysis but is not subtracted and relabeled as
trait-solving time. Stable and nightly use different revisions.

## Time-pass attribution

Thirty observer-affected time-pass repetitions produced:

| Fixture | Old `type_check_crate` | Global-next `type_check_crate` |
| --- | ---: | ---: |
| Tiny | 1.09 ms | 1.05 ms |
| Repeated marker | 17.95 ms | 22.82 ms |
| Unique marker | 191.69 ms | 197.90 ms |
| Repeated projection | 19.41 ms | 26.08 ms |
| Unique projection | 157.80 ms | 175.71 ms |
| Supertrait depth 200 | 25.16 ms | 28.56 ms |
| Projection depth 128 | 7.48 ms | 8.39 ms |
| Method candidates 1,000 | 2,238.11 ms | 3,245.68 ms |

The unique fixtures include item collection, well-formedness, coherence, and
body work for thousands of generated definitions. The pass is not a pure
solver timer.

## Self-profile attribution

Five full-event self-profiles per fixture produced:

| Fixture | Solver | `typeck_root` self | Old obligation hits / misses | Next proof-tree hits / misses |
| --- | --- | ---: | ---: | ---: |
| Repeated marker | Old | 41.75 ms | 2 / 1 | - |
| Unique marker | Old | 27.02 ms | 10,000 / 5,000 | - |
| Repeated projection | Old | 41.08 ms | 3 / 2 | - |
| Unique projection | Old | 17.60 ms | 6,000 / 4,000 | - |
| Supertrait depth 200 | Old | 3.85 ms | 20,102 / 201 | - |
| Projection depth 128 | Old | 4.31 ms | 2 / 3 | - |
| Method candidates 1,000 | Old | 3,526.83 ms | 1,000,003 / 3,003 | - |
| Repeated marker | Global next | 59.14 ms | - | no row |
| Unique marker | Global next | 33.96 ms | - | no row |
| Repeated projection | Global next | 64.72 ms | - | no row |
| Unique projection | Global next | 34.85 ms | - | no row |
| Supertrait depth 200 | Global next | 6.02 ms | - | no row |
| Projection depth 128 | Global next | 10.19 ms | - | no row |
| Method candidates 1,000 | Global next | 4,541.28 ms | - | 998,001 / 999 |

The old method fixture invoked `evaluate_obligation` 1,003,006 times but its
event self time was 11.36 ms. The next method fixture invoked
`evaluate_root_goal_for_proof_tree_raw` 999,000 times but its event self time
was 1.72 ms.

These values demonstrate cache access and query visibility. They do not
capture complete method probing, candidate assembly, fulfillment, inference,
or solver work. Self-profile also increased wall time substantially.

## Structural scaling

Stable metadata compilation used three warm-ups and 30 repetitions.

| Supertrait depth | Median | MAD |
| ---: | ---: | ---: |
| 10 | 73.72 ms | 2.29 ms |
| 25 | 101.31 ms | 5.65 ms |
| 50 | 103.32 ms | 4.08 ms |
| 100 | 107.35 ms | 7.45 ms |
| 200 | 137.09 ms | 4.52 ms |
| 400 | 191.95 ms | 7.80 ms |

| Projection depth | Median | MAD |
| ---: | ---: | ---: |
| 8 | 75.34 ms | 2.55 ms |
| 16 | 76.31 ms | 1.28 ms |
| 32 | 76.26 ms | 2.57 ms |
| 64 | 102.10 ms | 7.24 ms |
| 128 | 110.86 ms | 6.27 ms |
| 256 | 124.28 ms | 6.50 ms |

All series remained below 7.1% relative MAD. The chains were successful,
acyclic, and concrete; cyclic ambiguity and overflow are separate controls.

## Method-candidate width and call count

Each stable, old-solver, and global-next series used three warm-ups and 30
repetitions.

### Candidate width with 1,000 calls

| Candidates | Stable | Old solver | Global next |
| ---: | ---: | ---: | ---: |
| 10 | 98.78 ms | 96.85 ms | 100.77 ms |
| 50 | 181.20 ms | 162.76 ms | 191.00 ms |
| 100 | 247.10 ms | 229.64 ms | 285.87 ms |
| 250 | 555.41 ms | 491.18 ms | 628.90 ms |
| 500 | 1,108.91 ms | 1,025.56 ms | 1,279.87 ms |
| 1,000 | 2,384.21 ms | 2,185.89 ms | 2,744.73 ms |

### Call count with 1,000 candidates

| Calls | Stable | Old solver | Global next |
| ---: | ---: | ---: | ---: |
| 1 | 138.09 ms | 137.16 ms | 144.71 ms |
| 10 | 147.04 ms | 145.51 ms | 161.71 ms |
| 100 | 292.70 ms | 276.21 ms | 339.39 ms |
| 250 | 582.52 ms | 544.22 ms | 729.73 ms |
| 500 | 1,128.36 ms | 1,043.95 ms | 1,379.14 ms |
| 1,000 | 2,403.72 ms | 2,195.45 ms | 2,783.46 ms |

All method series remained below 4.9% relative MAD. The fixture controls the
number of in-scope traits with one applicable implementation. It does not
represent inherent methods, blanket impls, autoderef chains, generic
receivers, specialization, negative reasoning, or real crate import patterns.

## Solver comparison

Global-next differed modestly on ordinary primary controls and materially on
the method topology. In the orthogonal 1,000-by-1,000 scaling controls:

- width sweep: 2,744.73 versus 2,185.89 ms, 25.6% slower;
- call sweep: 2,783.46 versus 2,195.45 ms, 26.8% slower.

The primary method series measured a larger 35.5% difference. Different
round-robin sets, code generation, and host state explain why the precise
ratios are not identical.

No conclusion is drawn about overall solver superiority. Existing rustc-perf
next-solver pairs cover large real and synthetic programs with different
topologies.

## Incremental edit matrix

The fixture contained:

- one `Marker` trait with one method;
- one `Extra` trait;
- `Leaf` and `Other` types;
- one shared `require<T: Marker>()` helper;
- 1,000 caller body owners with the same concrete `Leaf: Marker` goal.

Each reused scenario had an independent incremental directory. Primary wall
time used 15 repetitions; query summaries used five profiles.

| Solver | Scenario | Wall median | MAD | `typeck_root` misses | Old obligation misses |
| --- | --- | ---: | ---: | ---: | ---: |
| Old | Fresh directory | 143.93 ms | 7.39 ms | 1,002 | 1 |
| Old | Untouched source | 145.64 ms | 13.21 ms | 0 | 0 |
| Old | Identical rewrite | 157.78 ms | 10.39 ms | 0 | 0 |
| Old | One caller body | 201.94 ms | 7.99 ms | 1 | 1 |
| Old | Impl method body | 206.90 ms | 10.37 ms | 1 | 0 |
| Old | Unrelated same-trait impl | 207.44 ms | 3.21 ms | 0 | 1 |
| Old | Shared bound | 219.59 ms | 12.52 ms | 1,001 | 1 |
| Global next | Fresh directory | 143.71 ms | 7.29 ms | 1,002 | - |
| Global next | Untouched source | 138.72 ms | 4.81 ms | 0 | - |
| Global next | Identical rewrite | 168.57 ms | 8.31 ms | 0 | - |
| Global next | One caller body | 202.76 ms | 8.50 ms | 1 | - |
| Global next | Impl method body | 206.32 ms | 10.99 ms | 1 | - |
| Global next | Unrelated same-trait impl | 220.42 ms | 8.10 ms | 1,000 | - |
| Global next | Shared bound | 223.66 ms | 10.80 ms | 1,001 | - |

All wall series remained below 10% relative MAD. Query miss counts remained
identical across all five profiles. Wall differences among small reused
scenarios are not promoted as speedups.

The impl-set edit alternated the presence of `impl Marker for Other`, while all
callers continued to prove `Leaf: Marker`. The old solver re-evaluated one
canonical obligation query and reused caller `TypeckResults`. The globally
enabled next solver re-ran all 1,000 caller roots.

This exact dependency difference is a candidate regression fixture. It does
not establish behavior for coherence-only default mode, cross-crate edits,
blanket impls, generic goals, or future solver revisions.

## Expected failures

Each nightly mode used three warm-ups and 30 repetitions. All fixtures retained
exit status 1 and complete stderr.

| Failure | Old median | Global-next median | Diagnostic |
| --- | ---: | ---: | --- |
| Unsatisfied `Leaf: Marker` | 62.46 ms | 65.95 ms | trait bound not satisfied |
| Ambiguous same-name method | 64.77 ms | 66.49 ms | multiple applicable items |
| Recursive blanket impl | 63.14 ms | 64.64 ms | overflow evaluating requirement |

The recursive-overflow stderr differed by three bytes between modes while
preserving the same failure class. Failure time is not successful solver
throughput.

## Diagnostics assessment

Current nightly exposes:

- effective solver configuration through compiler options;
- `type_check_crate` and per-owner `typeck_root`;
- old-solver `evaluate_obligation` cache hits and misses;
- `trait_impls_of`, coherence, impl-header, normalization, and related queries;
- selected new-solver proof-tree query events;
- incremental owner-result hits and misses;
- debug logs and proof-tree inspection for targeted diagnosis.

It does not provide one stable machine-readable view combining:

- canonical goal identity and repetition;
- method candidate enumeration and applicability;
- old and new solver cache behavior;
- cycles, provisional entries, stalled goals, and fixpoint iterations;
- per-owner solver versus method-probe versus inference cost;
- impl-set dependency and invalidation breadth.

FERRIUM can join source topology, solver mode, query evidence, and controlled
edits externally. Compiler event or rustc-perf contributions require
maintainer guidance and explicit approval.

## Limitations

- Synthetic goals do not represent complete ecosystem APIs.
- The candidate fixture uses one applicable impl and same-named imported
  traits.
- Stable complete and nightly diagnostic runs use different compiler
  revisions.
- Nightly default coherence-only mode was not a third same-nightly wall
  distribution; explicit old/global endpoints isolate body solver behavior.
- Time passes and self-profile materially affect wall time.
- Five-profile medians are diagnostic, not primary distributions.
- The new solver's internal cache is not fully represented by query events.
- The impl-set invalidation result is one local-crate concrete-goal fixture.
- Windows scheduling, filesystem, antivirus, and indexing remain host-local
  influences.
- Linux and macOS have not been measured.
- No rustc-perf collector run was executed.
- Failed compilations stop before complete successful work.

## Reproduction evidence

The retained evidence includes:

- deterministic fixture generators;
- stable, no-analysis, old-solver, and global-next distributions;
- 30-sample structural and method scaling sweeps;
- 30-sample time-pass distributions;
- five-run full-event self-profile summaries;
- two-solver incremental edit distributions and query summaries;
- expected failure output;
- pinned rustc and rustc-perf source evidence;
- exact source sizes, toolchains, commands, and compiler revisions.
