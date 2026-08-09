# EXP-01: MIR Topology, Pass Cost, and Reuse

Date: 2026-08-08
Question: PERF-Q15
Status: Complete

## Purpose

Measure MIR work across body construction, pass traversal, drop elaboration,
cleanup/unwind calls, promotions, inlining, coroutine transformation, CTFE,
validation, optimization levels, and incremental edit frontiers.

Stable metadata compilation is the primary complete distribution. Nightly
no-analysis, binary encoded-MIR, time-pass, self-profile, pass-policy,
incremental, CTFE, validation, and failure runs are diagnostic boundaries.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- local NTFS workspace;
- stable rustc `1.95.0 (59807616e 2026-04-14)`, LLVM `22.1.2`;
- nightly rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`, LLVM `23.1.0`;
- host `x86_64-pc-windows-msvc`.

Stable and no-analysis series used three warm-ups and 30 round-robin
repetitions. Optimization-level and control comparisons were interleaved.
Profiles used five repetitions unless stated otherwise.

## Compiler boundaries

The pinned compiler defines:

- `thir_body`: stealable THIR, `no_hash`;
- `mir_built`: stealable built MIR;
- `mir_promoted`: stealable main and promoted MIR, `no_hash`;
- `mir_drops_elaborated_and_const_checked`: runtime-ready MIR, `no_hash`;
- `mir_for_ctfe`: disk-cached CTFE MIR without normal optimization;
- `promoted_mir`: disk-cached promoted CTFE MIR;
- `optimized_mir`: disk-cached codegen-ready MIR.

`Steal<Body>` transfers ownership between phases rather than cloning the main
body. Query memoization prevents duplicate provider execution in one session.
Repeated work comes from later whole-body passes, generated bodies, CTFE and
runtime paths, inlining, validation, and invalidation.

## Primary fixtures

| Fixture | Shape | Source bytes |
|---|---|---:|
| Tiny | One constant function | 67 |
| Temporaries | 10,000 locals and binary assignments | 576,765 |
| Wrapping calls | 10,000 small method calls | 408,988 |
| Match | 2,000 integer arms | 41,904 |
| Aggregate | 2,000-field construction | 92,821 |
| Partial moves | 1,000 non-`Copy` fields moved individually | 118,619 |
| Cleanup calls | 1,000 potentially unwinding calls with a live drop guard | 30,342 |
| Inline calls | 1,000 calls to one always-inline callee | 25,226 |
| Promotions | 1,000 promoted array temporaries | 90,460 |
| Coroutine awaits | 100 sequential saved-local/await pairs | 8,211 |
| Coroutine live | 100 locals live across 100 awaits | 8,202 |

## Construction and primary totals

| Fixture | Stable metadata | MAD | No analysis | `mir_built` level 1 |
|---|---:|---:|---:|---:|
| Tiny | 80.92 ms | 5.36 ms | 72.66 ms | 0.37 ms |
| Temporaries | 440.49 ms | 36.71 ms | 124.46 ms | 69.89 ms |
| Wrapping calls | 327.31 ms | 14.10 ms | 114.48 ms | 38.83 ms |
| Match | 115.44 ms | 8.79 ms | 74.45 ms | 3.80 ms |
| Aggregate | 129.61 ms | 8.92 ms | 77.68 ms | 1.48 ms |
| Partial moves | 204.77 ms | 13.16 ms | 81.92 ms | 14.37 ms |
| Cleanup calls | 108.58 ms | 8.44 ms | 80.94 ms | 3.83 ms |
| Inline calls | 103.01 ms | 8.62 ms | 76.18 ms | 3.63 ms |
| Promotions | 230.41 ms | 18.65 ms | 81.33 ms | 15.09 ms |
| Coroutine awaits | 203.54 ms | 12.01 ms | 73.70 ms | 5.16 ms |
| Coroutine live | 215.41 ms | 11.70 ms | 74.05 ms | 5.08 ms |

All stable primary series remained below 8.4% relative MAD. No-analysis is not
subtracted and relabeled as MIR time. Stable and nightly use different
compiler revisions.

Construction scaling from three-profile medians:

| Temporaries | `mir_built` level 1 |
|---:|---:|
| 100 | 1.34 ms |
| 1,000 | 12.27 ms |
| 5,000 | 32.16 ms |
| 10,000 | 86.66 ms |

The curve is fixture-specific. It includes THIR-to-MIR lowering for one large
body and does not establish a universal per-statement coefficient.

## Pass-level attribution

Five level-2 profiles produced:

| Fixture | Inline | Destination propagation | GVN | Elaborate drops | Promote temps | State transform |
|---|---:|---:|---:|---:|---:|---:|
| Temporaries | 58.14 ms | 60.71 ms | 10.68 ms | 6.03 ms | 1.76 ms | 0.00 ms |
| Wrapping calls | 68.24 ms | 60.47 ms | 9.95 ms | 4.69 ms | 0.97 ms | 0.00 ms |
| Partial moves | 20.27 ms | 1.85 ms | 6.36 ms | 12.90 ms | 0.31 ms | 0.00 ms |
| Cleanup calls | 1.85 ms | 3.54 ms | 0.59 ms | 0.64 ms | 0.12 ms | 0.00 ms |
| Inline calls | 4.24 ms | 0.90 ms | 0.76 ms | 0.38 ms | 0.10 ms | 0.00 ms |
| Promotions | 0.01 ms | 0.06 ms | 3.06 ms | 1.81 ms | 2.25 ms | 0.04 ms |
| Coroutine awaits | 0.01 ms | 0.91 ms | 0.82 ms | 1.62 ms | 0.11 ms | 16.09 ms |
| Coroutine live | 0.01 ms | 1.20 ms | 0.96 ms | 1.54 ms | 0.16 ms | 21.59 ms |

Pass activities are separate from `optimized_mir` query self time. For
example, the temporary fixture's `optimized_mir` self time was only 4.70 ms
while pass activities accounted for much more work. Query self time therefore
does not represent nested pass execution.

The inliner scans candidate bodies even when it performs no useful expansion.
The temporary fixture has binary assignments rather than ordinary calls, yet
its inliner activity was material.

## Optimization-level controls

Fifteen repetitions interleaved MIR levels 0 through 4 using:

```text
-Copt-level=3 -Zalways-encode-mir --emit=metadata -Zmir-opt-level=<0..4>
```

| Fixture | Level 0 | Level 1 | Level 2 | Level 3 | Level 4 |
|---|---:|---:|---:|---:|---:|
| Temporaries | 439.32 ms | 454.49 ms | 515.95 ms | 536.90 ms | 550.05 ms |
| Wrapping calls | 335.94 ms | 334.99 ms | 470.48 ms | 493.03 ms | 521.89 ms |
| Partial moves | 200.04 ms | 212.10 ms | 229.65 ms | 247.10 ms | 240.74 ms |
| Promotions | 228.62 ms | 240.97 ms | 231.10 ms | 223.09 ms | 248.57 ms |
| Coroutine live | 187.19 ms | 195.32 ms | 195.04 ms | 192.92 ms | 192.33 ms |

The complete matrix retained all fixtures and encoded output sizes. Tiny,
match, aggregate, cleanup, inline, promotion, and coroutine differences were
generally small relative to noise. Sequential non-interleaved level series
were retained but not promoted because machine drift made later levels appear
uniformly faster.

The fixed codegen optimization setting isolates MIR-level policy from
`-Copt-level` selection, while the encoded workflow avoids LLVM and linking.
It is diagnostic, nightly-only, and not equivalent to a user release build.
A separate paired debug/optimized matrix was retained as workflow context but
was not used for the MIR-level claim.

## Drop, promotion, and cleanup topology

| Size | Partial-move drop elaboration | Promotion pass |
|---:|---:|---:|
| 100 | 0.73 ms | 0.25 ms |
| 250 | 2.56 ms | 0.59 ms |
| 500 | 5.25 ms | 1.11 ms |
| 1,000 | 12.18 ms | 2.19 ms |

The two axes are separate fixtures. Partial moves create move paths,
initialization state, conditional drops, and drop flags. Promotions create
additional bodies that later require their own handling.

The cleanup fixture's stable median was only 108.58 ms and drop-elaboration
self time was 0.64 ms at level 2. One live guard with many calls did not
produce a dominant cleanup result; a richer fanout fixture remains an upstream
research candidate.

## Coroutine topology

Three-profile scaling medians at level 1:

| Size | Sequential awaits | Simultaneously live locals |
|---:|---:|---:|
| 10 | 0.62 ms | 1.08 ms |
| 25 | 1.68 ms | 2.42 ms |
| 50 | 5.97 ms | 4.67 ms |
| 100 | 33.28 ms | 20.95 ms |

The five-profile primary medians were lower—15.30 and 19.87 ms—showing
observer noise in the largest coroutine controls. The increasing curves and
material 100-point costs are promoted; the exact ratio and a universal
quadratic claim are not.

Coroutine transformation includes liveness across suspension, saved-local
layout, storage conflicts, state dispatch, drop elaboration, and shim
generation. Await count alone does not identify those dimensions.

## Inlining and validation controls

Fifteen interleaved repetitions compared default level 2, validation, and
`-Inline`:

| Fixture | Default | Validate | No inline | Default bytes | No-inline bytes |
|---|---:|---:|---:|---:|---:|
| Temporaries | 509.39 ms | 630.94 ms | 493.24 ms | 1,168,597 | 1,168,597 |
| Wrapping calls | 450.06 ms | 554.36 ms | 722.74 ms | 955,336 | 1,137,839 |
| Partial moves | 234.51 ms | 276.45 ms | 230.08 ms | 795,832 | 642,742 |
| Coroutine live | 187.01 ms | 200.06 ms | 184.45 ms | 226,982 | 226,982 |

Validation changed no encoded output. It is diagnostic correctness checking
and not part of the primary compile distribution.

The no-inline result demonstrates a trade rather than a simple speedup.
Inlining made wrapping calls cheaper and smaller after later simplification.
Partial moves paid inliner time but showed no material complete-time penalty.

## Incremental edit matrix

The fixture contained one shared const, one always-inline helper, and 1,000
owners. Each scenario used an independent incremental directory.

| Configuration and edit | `mir_built` misses | `optimized_mir` hits | `optimized_mir` misses |
|---|---:|---:|---:|
| Level 1 untouched | 0 | 0 | 0 |
| Level 1 identical rewrite | 0 | 0 | 0 |
| Level 1 one owner | 1 | 1 | 1 |
| Level 1 shared const | 1,001 | 0 | 1,000 |
| Level 1 helper body | 1 | 1 | 1 |
| Level 2 untouched | 0 | 0 | 0 |
| Level 2 identical rewrite | 0 | 0 | 0 |
| Level 2 one owner | 1 | 1 | 1 |
| Level 2 shared const | 1,001 | 0 | 1,000 |
| Level 2 helper body | 1 | 1 | 1 |
| Forced inline one owner | 1 | 2 | 2 |
| Forced inline shared const | 1,001 | 1,999 | 1,001 |
| Forced inline helper body | 1,001 | 2,001 | 1,002 |

Untouched and identical rewrites produced no MIR provider events. This is
incremental dependency reuse, not proof that every intermediate MIR value was
deserialized.

Query-result-cache and dependency-graph loading each cost roughly 15-40 ms in
these one-shot profiles. The values were noisy and are retained as context,
not an optimization claim.

Ordinary level-2 incremental builds did not fan out the helper body because
MIR inlining is normally disabled in incremental mode. The forced control
uses unstable pass policy and exists only to expose the dependency boundary.

## CTFE, failures, and observer effects

The CTFE fixture defined 1,000 const items, each executing a 20-iteration const
function. Its stable complete median was 207.71 ms with 5.46 ms MAD.

| Event | Misses | Hits | Self time | Total time |
|---|---:|---:|---:|---:|
| `mir_for_ctfe` | 1,003 | 41,999 | 1.78 ms | 44.15 ms |
| `optimized_mir` | 4 | 2 | 0.12 ms | 0.51 ms |
| `eval_to_const_value_raw` | 2,000 | 3 | 2.20 ms | 363.43 ms |
| `eval_to_allocation_raw` | 1,000 | 1,000 | 123.22 ms | 166.62 ms |

Nested totals overlap and must not be added. The result demonstrates that CTFE
execution can dominate CTFE MIR preparation.

Three expected failures retained exit status 1 and complete stderr:

| Failure | Stderr |
|---|---:|
| Type mismatch | 538 bytes |
| Const division by zero | 385 bytes |
| Invalid transmute size | 590 bytes |

The initial optimized-MIR forcing method used `--emit=mir`. The
10,000-temporary level-0 output was 802,835,949 bytes with a 9,180.00 ms
three-sample median. The stopped level-2 run had already produced
1,018,137,722 bytes. These are excluded observer failures, not compiler
optimization results.

`-Ztime-passes` exposed `MIR_borrow_checking`, `codegen_crate`, and total but no
standalone MIR-optimization interval in this workflow. Five self-profiles are
the promoted pass attribution.

## Diagnostics assessment

Current nightly exposes query cache hits/misses, self and total time, named
`mir_pass_*` activities, incremental load activities, MIR dumps, validation,
pass overrides, optimization levels, and binary MIR encoding.

It does not provide one stable machine-readable view combining:

- owners, statements, locals, blocks, edges, cleanups, and generated bodies;
- pass eligibility, traversal count, changes made, and before/after body size;
- move paths, drop flags, promoted bodies, and coroutine saved-local conflicts;
- inlining candidates, accepted sites, expansion size, and later-pass work;
- edit dependency frontiers and serialized-result provenance;
- CTFE preparation versus interpreter work;
- observer cost and complete stable workflow latency.

FERRIUM can join controlled source, query, pass, and edit evidence externally.
Compiler counters and rustc-perf additions require maintainer guidance.

## Limitations

- Synthetic bodies do not represent complete applications.
- Stable totals and nightly diagnostics use different revisions.
- Binary MIR encoding is nightly and adds serialization work.
- Profiles and validation affect scheduling and wall time.
- Pass activity self time does not report changes made or output-size delta.
- Three-profile scaling is diagnostic and can be noisy.
- The coroutine curves support topology sensitivity, not a universal
  complexity class.
- Metadata/encoded-MIR workflows exclude LLVM, object emission, and linking.
- The cleanup fixture did not create a dominant fanout result.
- No async-drop-specific fixture was promoted.

## Reproduction

Session artifacts retained the fixture generators, source, JSON distributions,
profile summaries, failure stderr, and excluded-run note. Representative
commands were:

```text
node generate-fixtures.js
node measure-fixtures.js stable-metadata
node measure-fixtures.js no-analysis
node measure-fixtures.js level-round-robin
FERRIUM_PROFILE_REPETITIONS=5 node profile-fixtures.js
node measure-scaling.js
node measure-incremental.js
node measure-controls.js
node measure-ctfe.js
node measure-failures.js
```

Regenerable compiler outputs, profile directories, and incremental caches are
not repository artifacts.
