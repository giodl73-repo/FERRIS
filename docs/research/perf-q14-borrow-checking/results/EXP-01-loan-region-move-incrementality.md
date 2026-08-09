# EXP-01: Loan, Region, Move, and Incremental Topology

Date: 2026-08-08
Question: PERF-Q14
Status: Complete

## Purpose

Measure borrow-check cost and reuse across:

1. immediate versus simultaneously live shared loans;
2. mutable reborrow depth;
3. one large body versus many borrow-check roots;
4. CFG joins and mutable accesses;
5. partial non-`Copy` moves;
6. nested closure captures;
7. mutable borrows across await points;
8. ordinary NLL, legacy Polonius, and Polonius-next modes;
9. untouched, rewritten, localized, helper-body, and shared-type edits;
10. move, conflict, and lifetime failures.

Stable complete metadata compilation remains primary. Nightly no-analysis,
time-pass, self-profile, Polonius, incremental, and failure runs are
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

## Compiler boundaries

The pinned compiler defines:

- `thir_body` for typed high-level IR;
- `mir_built` for THIR-to-MIR lowering;
- `mir_drops_elaborated_and_const_checked` for adjacent MIR work;
- `mir_promoted` as the promoted input to borrow checking;
- `mir_borrowck` for each type-check root;
- `optimized_mir` for later codegen-ready MIR.

`mir_borrowck` has no `cache_on_disk` declaration. `typeck_root` and
`optimized_mir` do. Borrow checking can be skipped when the incremental
dependency graph keeps an ensured query green, but rustc does not deserialize
a saved borrow set, move data, region solution, or diagnostics.

The provider gathers nested closures and coroutines under the root. It clones
MIR, renumbers regions, gathers move paths and loans, MIR-type-checks,
constructs liveness and outlives constraints, solves regions, executes three
dataflow analyses, checks accesses, and emits buffered diagnostics.

## Primary fixtures

| Fixture | Shape | Source bytes |
| --- | --- | ---: |
| Tiny | One function without an explicit borrow | 67 |
| Immediate borrows | 10,000 shared borrows consumed immediately | 854,559 |
| Overlapping borrows | 4,000 shared borrows retained together | 338,561 |
| Mutable reborrow | One 2,001-loan reborrow chain | 75,953 |
| Borrow owners | 5,000 owners with one shared reborrow each | 393,929 |
| CFG joins | 1,000 binary joins and 2,000 mutable borrows | 153,765 |
| Partial moves | 2,000 non-`Copy` fields moved individually | 242,619 |
| Closures | 1,000 closures capturing one value each | 85,558 |
| Await | 200 mutable borrows retained across await points | 13,341 |

Warnings were disabled so generated diagnostics did not dominate successful
controls.

## Primary fixture matrix

Each stable, no-analysis, and nightly Polonius-off series had three warm-ups
and 30 round-robin repetitions.

| Fixture | Stable | MAD | No analysis | Nightly off |
| --- | ---: | ---: | ---: | ---: |
| Tiny | 69.33 ms | 4.87 ms | 56.85 ms | 70.95 ms |
| Immediate borrows | 573.58 ms | 26.93 ms | 118.17 ms | 565.64 ms |
| Overlapping borrows | 395.89 ms | 15.20 ms | 81.12 ms | 399.42 ms |
| Mutable reborrow | 95.71 ms | 6.47 ms | 61.80 ms | 95.95 ms |
| Borrow owners | 389.54 ms | 12.22 ms | 110.99 ms | 393.91 ms |
| CFG joins | 198.58 ms | 10.73 ms | 69.23 ms | 198.84 ms |
| Partial moves | 261.85 ms | 12.15 ms | 74.34 ms | 264.79 ms |
| Closures | 229.05 ms | 12.34 ms | 66.20 ms | 233.17 ms |
| Await | 395.45 ms | 21.46 ms | 58.89 ms | 291.94 ms |

All stable series remained below 7.1% relative MAD. All no-analysis and
nightly-off series remained below 9.4%.

No-analysis excludes semantic analysis but is not subtracted and relabeled as
borrow checking. Stable and nightly use different compiler revisions. The
await stable/nightly difference is not attributed to one compiler change.

## Time-pass attribution

Thirty observer-affected time-pass repetitions produced:

| Fixture | `MIR_borrow_checking` |
| --- | ---: |
| Tiny | 0.81 ms |
| Immediate borrows | 261.07 ms |
| Overlapping borrows | 217.65 ms |
| Mutable reborrow | 9.63 ms |
| Borrow owners | 106.74 ms |
| CFG joins | 51.18 ms |
| Partial moves | 88.82 ms |
| Closures | 86.06 ms |
| Await | 75.14 ms |

The time-pass section surrounds broad owner scheduling and is not equivalent
to the sum of per-owner query self time. Instrumentation increased the
many-owner wall median from 393.91 to 520.68 ms.

## Self-profile attribution

Five full-event profiles produced:

| Fixture | `typeck_root` self | `mir_built` self | `mir_borrowck` self | Borrow misses |
| --- | ---: | ---: | ---: | ---: |
| Tiny | 0.15 ms | 0.29 ms | 0.27 ms | 1 |
| Immediate borrows | 214.58 ms | 101.50 ms | 209.47 ms | 1 |
| Overlapping borrows | 62.35 ms | 31.46 ms | 207.67 ms | 1 |
| Mutable reborrow | 8.34 ms | 5.11 ms | 5.82 ms | 1 |
| Borrow owners | 59.85 ms | 59.08 ms | 81.61 ms | 5,000 |
| CFG joins | 53.70 ms | 15.65 ms | 38.53 ms | 1 |
| Partial moves | 258.95 ms | 37.52 ms | 61.40 ms | 1 |
| Closures | 42.42 ms | 17.99 ms | 56.45 ms | 1 |
| Await | 28.09 ms | 8.29 ms | 23.48 ms | 2 |

Self-profile changed wall time and is diagnostic. The event does not expose
move-path count, loan count, active overlap, region SCCs, dataflow iterations,
place-conflict comparisons, or diagnostic subphases.

The immediate and overlapping fixtures made borrow checking comparable to or
larger than HIR type checking. Partial moves remained type-check dominated.

## Loan-count and lifetime scaling

Nightly Polonius-off wall series used three warm-ups and 30 repetitions.
Profiles used three repetitions.

| Loans | Immediate wall | Immediate borrowck | Overlap wall | Overlap borrowck |
| ---: | ---: | ---: | ---: | ---: |
| 100 | 81.18 ms | 1.97 ms | 77.66 ms | 2.72 ms |
| 500 | 90.90 ms | 5.71 ms | 92.07 ms | 11.19 ms |
| 1,000 | 109.39 ms | 12.53 ms | 119.76 ms | 21.75 ms |
| 2,000 | 149.15 ms | 29.78 ms | 182.79 ms | 61.51 ms |
| 4,000 | 263.25 ms | 64.89 ms | 410.11 ms | 198.28 ms |

All wall series remained below 9.0% relative MAD. Borrow-check profile MAD
was 6.15 ms or lower.

The two families contain the same declaration, borrow, and use counts. Only
ordering changes whether each loan ends immediately or all remain live until
the final use sequence. The difference therefore supports loan lifetime and
active overlap as independent dimensions without establishing a universal
complexity function.

## Owner scaling

| Owners | Wall | `mir_borrowck` self |
| ---: | ---: | ---: |
| 100 | 114.42 ms | 2.19 ms |
| 500 | 136.46 ms | 8.13 ms |
| 1,000 | 163.45 ms | 14.91 ms |
| 2,500 | 253.12 ms | 41.46 ms |
| 5,000 | 419.02 ms | 85.43 ms |

The 1,000-owner wall series reached 10.4% relative MAD; the others remained
below 8.7%. Every owner produced an independent query miss. Aggregate
self-time growth includes per-query setup and does not directly represent
parallel wall time.

## Nested body controls

Await scaling produced:

| Await points | Wall | `mir_borrowck` self |
| ---: | ---: | ---: |
| 10 | 115.02 ms | 1.64 ms |
| 25 | 119.75 ms | 2.77 ms |
| 50 | 131.25 ms | 4.73 ms |
| 100 | 160.51 ms | 11.04 ms |
| 200 | 298.23 ms | 25.15 ms |

The 50-await wall series reached 11.7% relative MAD; the others remained
below 8.5%.

In the primary await profile, `mir_borrowck` had 23.48 ms self time and
129.15 ms total time. `optimized_mir` appeared with 77.38 ms total time nested
under the coroutine-related query graph. This supports separate self, total,
nested-body, and later-MIR reporting.

The closure fixture generated 1,001 `mir_built` and `mir_promoted` misses but
one root borrow-check miss. Root aggregation must not hide nested-body count.

## Polonius controls

The pinned modes are:

- `off`: ordinary NLL and the default;
- `legacy`: fact generation plus `polonius-engine`;
- `next`: the in-tree experimental implementation.

A first matrix with 1,000-loan, 250-join, 500-owner, and 50-await fixtures was
cancelled after 155 minutes. No complete distribution was promoted.

The replacement used smaller fixtures, two warm-ups, 15 round-robin
repetitions, and explicit mode flags:

| Fixture | Off | Legacy | Next |
| --- | ---: | ---: | ---: |
| Tiny | 96.16 ms | 92.46 ms | 107.19 ms |
| Immediate loans 100 | 98.71 ms | 721.08 ms | 116.95 ms |
| Overlapping loans 100 | 107.93 ms | 782.84 ms | 98.48 ms |
| CFG joins 25 | 99.45 ms | 261.93 ms | 96.34 ms |
| Owners 50 | 122.63 ms | 128.00 ms | 122.56 ms |
| Await points 10 | 149.37 ms | 292.95 ms | 142.79 ms |

Several small series exceeded 10% relative MAD. The immediate and overlapping
legacy gaps were much larger than their noise and support only a directional
legacy-cost result. Off-versus-next differences are not promoted.

Legacy and next are not production recommendations. The pinned
Polonius-next implementation also documents unresolved semantic subtleties.

## Incremental edit matrix

The fixture contained:

- one shared `Scalar` alias;
- one borrow-using helper;
- 1,000 caller body owners;
- one local value, borrow, and helper call per caller.

Each scenario had an independent incremental directory. Primary wall time used
15 repetitions; query summaries used five profiles.

| Scenario | Wall median | Relative MAD | `typeck_root` misses | `mir_built` misses | `mir_borrowck` misses |
| --- | ---: | ---: | ---: | ---: | ---: |
| Fresh directory | 386.36 ms | 30.2% | 1,001 | 1,001 | 1,001 |
| Untouched source | 303.17 ms | 22.0% | 0 | 0 | 0 |
| Identical rewrite | 372.97 ms | 20.0% | 0 | 0 | 0 |
| One caller body | 373.62 ms | 4.6% | 1 | 1 | 1 |
| Helper body | 404.48 ms | 17.9% | 1 | 1 | 1 |
| Shared alias | 559.48 ms | 19.2% | 1,001 | 1,001 | 1,001 |

Only the one-body wall series met the 10% relative-MAD gate. The query counts
were identical across all five profiles and are the promoted incremental
evidence.

The helper edit changed one arithmetic literal without adding definitions or
changing the signature. The shared edit changed `Scalar` between `u32` and
`i32`, invalidating every caller's type and MIR dependencies.

No `mir_borrowck` event for untouched source is query-provider reuse, not
evidence of a serialized borrow-check result.

## Expected failures

Nightly Polonius-off metadata compilation used three warm-ups and 30
round-robin repetitions. All fixtures retained exit status 1 and complete
stderr.

| Failure | Median | MAD | Stderr |
| --- | ---: | ---: | ---: |
| Use after move | 95.63 ms | 10.73 ms | 1,064 bytes |
| Mutable/shared conflict | 83.17 ms | 6.16 ms | 897 bytes |
| Return reference to local | 85.19 ms | 7.53 ms | 497 bytes |

The use-after-move series reached 11.2% relative MAD. Failure timing is
diagnostic only. Errors are correctness evidence, not fast paths.

## Diagnostics assessment

Current nightly exposes:

- `MIR_borrow_checking` as a broad time-pass section;
- `thir_body`, `mir_built`, `mir_promoted`, `mir_borrowck`, and
  `optimized_mir` query events;
- per-root cache hits, misses, self time, total time, and incremental load
  evidence;
- MIR dumps, dataflow graphs, NLL facts, and Polonius facts through unstable
  diagnostic flags;
- complete move, conflict, lifetime, closure, and coroutine diagnostics.

It does not provide one stable machine-readable view combining:

- MIR statements, blocks, locals, joins, backedges, and cleanup edges;
- move paths, moves, initializations, partial moves, and drop state;
- loans, kinds, live ranges, active overlap, and two-phase activation;
- projection depth and place-conflict comparisons;
- region variables, outlives edges, SCC density, type tests, and iterations;
- nested closure/coroutine aggregation;
- borrow-check subphase time.

FERRIUM can join source, query, and controlled edit evidence externally.
Compiler counters or rustc-perf contributions require maintainer guidance and
explicit approval.

## Limitations

- Synthetic bodies do not represent complete application ownership designs.
- Source counts are not exact compiler loan, move-path, region, or MIR counts.
- MIR pretty-printing overflowed the Windows compiler stack on the largest
  generated body and was not used as promoted evidence.
- Stable complete and nightly diagnostic runs use different compiler
  revisions.
- Time passes and self-profile materially affect wall time.
- Five- and three-profile medians are diagnostic, not primary distributions.
- Incremental wall series were noisy; query counts carry the conclusion.
- Several bounded Polonius and failure series exceeded 10% relative MAD.
- The larger legacy Polonius matrix was cancelled rather than completed.
- Polonius-next remains experimental and can change rapidly.
- Windows scheduling, filesystem, antivirus, and indexing remain host-local
  influences.
- Linux and macOS have not been measured.
- No rustc-perf collector run was executed.
- Failed compilations stop before complete successful work.

## Reproduction evidence

The retained evidence includes:

- deterministic primary, scaling, incremental, Polonius, and failure
  generators;
- stable, no-analysis, and Polonius-off distributions;
- 30-sample time-pass distributions;
- five-run full-event self-profile summaries;
- 30-sample loan, owner, and await scaling distributions;
- three-mode bounded controls plus the cancelled-matrix record;
- incremental edit distributions and query summaries;
- expected failure output;
- pinned rustc and rustc-perf source evidence;
- exact source sizes, toolchains, commands, and compiler revisions.
