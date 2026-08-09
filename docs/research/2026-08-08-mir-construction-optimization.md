# MIR Construction and Optimization

Date: 2026-08-08
Question: PERF-Q15
Status: Complete
Decision: add MIR-body, pass-traversal, drop, promotion, coroutine, inlining,
validation, CTFE-path, and edit-frontier evidence to FERRIUM; prioritize
read-only pass attribution and upstream-quality isolated fixtures over source
rewrites or compiler-policy changes.

## Executive conclusion

MIR work is not one phase and repeated cost is usually not one query executing
twice. The pinned compiler memoizes query results and moves large bodies through
the pipeline with `Steal<Body>`. Repetition comes from successive whole-body
passes, promoted and coroutine-generated bodies, CTFE/runtime forks, inline
expansion, validation, and invalidated owners.

The strongest construction control was one 10,000-temporary body. Stable
metadata compilation measured 440.49 ms versus 80.92 ms for tiny, while
`mir_built` self time measured 69.89-71.43 ms. Construction scaled from
1.34 ms at 100 temporaries to 86.66 ms at 10,000 in the level-1 profile sweep.
Source bytes alone did not predict cost: a 92.8 KB, 2,000-field aggregate
measured only 129.61 ms stable and 1.52 ms of `mir_built` self time.

Optimization cost followed traversal and topology. At MIR level 2, the
10,000-temporary body spent 58.14 ms in the inliner even though it had no
ordinary call sites, 60.71 ms in destination propagation, and 10.68 ms in GVN.
Ten thousand `wrapping_add` calls spent 68.24 ms in inlining and 60.47 ms in
destination propagation. Fixed-`-Copt-level=3`, interleaved 15-run controls
moved those bodies from 439.32 and 335.94 ms at MIR level 0 to 515.95 and
470.48 ms at MIR level 2.
Most smaller fixtures remained within noise across levels.

Required semantic transforms had distinct hot shapes. One thousand partial
moves spent 10.78 ms at level 1 and 12.90 ms at level 2 in drop elaboration.
The scaling sweep rose from 0.73 ms at 100 partial moves to 12.18 ms at 1,000.
One thousand promoted arrays spent 2.25 ms in `PromoteTemps`; their stable
complete median was 230.41 ms, but neighboring construction, checking, and
encoding remained part of that total.

Coroutine state transformation was the dominant MIR pass in both async
controls. One hundred sequential await/saved-local pairs measured 15.30 ms at
level 1; 100 locals simultaneously live across 100 suspensions measured
19.87 ms. The scaling profiles rose from 0.62 ms at 10 awaits to 33.28 ms at
100 in one series. This supports suspension and saved-local topology as first-
class dimensions, not a universal complexity law.

`optimized_mir` is disk-cached, but reuse still follows dependencies. Untouched
and identically rewritten 1,000-owner fixtures executed no MIR queries. One
owner edit produced one `mir_built` and one `optimized_mir` miss. A shared
const edit produced 1,001 built-MIR misses and 1,000 optimized-MIR misses.
Ordinary incremental level-2 builds kept an inline-helper edit local because
MIR inlining is normally disabled with incremental compilation. Forcing
`Inline` expanded that edit to 1,001 built-MIR and 1,002 optimized-MIR misses.

Inlining is not simply removable overhead. Disabling it increased the
10,000-wrapping-call median from 450.06 to 722.74 ms and increased encoded MIR
from 955,336 to 1,137,839 bytes. On partial moves, disabling inlining changed
encoded size but left wall time within noise. Recommendations therefore need
caller/callee topology, downstream pass cost, output shape, profile, and
incremental mode.

CTFE remains a separate path. A 1,000-constant graph measured 1.78 ms of
`mir_for_ctfe` self time across 1,003 misses, while const allocation evaluation
measured 123.22 ms self time. That fixture cannot be labeled MIR optimization.

Validation is observer work. `-Zvalidate-mir` increased bounded medians by
about 7-24% across the four tested large fixtures. Textual `--emit=mir` was
also rejected as a primary workflow after one body produced 803 MB at level 0
and more than 1 GB at level 2. Binary `-Zalways-encode-mir` plus self-profile
provided the bounded diagnostic path.

FERRIUM should expose body scale, block/edge shape, pass schedule, required
versus optional transforms, generated-body count, optimization level,
incremental mode, edit frontier, and observer mode. It should not recommend
function splitting, inlining attributes, match rewrites, aggregate changes,
async rewrites, const removal, MIR-level changes, or validation disabling from
one count or pass event.

Credible upstream paths are isolated rustc-perf fixtures for partial-move drop
elaboration, pass traversal over large bodies, promoted-body count, saved-local
coroutine layout, cleanup/unwind fanout, and inline-edit invalidation. No
upstream activity was created.

## Decision supported

This research determines:

- which query and pass boundaries constitute MIR work;
- which body, drop, promotion, inlining, and coroutine topologies were material;
- how explicit MIR optimization levels changed bounded workflows;
- where CTFE, borrow checking, validation, serialization, and backend work
  remain separate;
- how disk-cached optimized MIR reused or invalidated across controlled edits;
- which diagnostics and isolated fixtures FERRIUM can defend.

It does not authorize source rewrites, compiler flags in production, compiler
forks, validation reduction, custom MIR passes, or upstream activity.

## Evidence reviewed

### Local evidence

- [Borrow-checking cost and incrementality](2026-08-08-borrow-checking-cost-incrementality.md)
- [Experiment](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [MIR query declarations](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L597-L716)
- [`Steal` ownership transfer](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_data_structures/src/steal.rs#L1-L66)
- [analysis-to-runtime transforms](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/lib.rs#L632-L668)
- [CTFE MIR path](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/lib.rs#L491-L531)
- [drop elaboration](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/elaborate_drops.rs#L21-L94)
- [pass manager and profile activities](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/pass_manager.rs#L13-L117)
- [inlining policy and thresholds](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/inline.rs#L262-L383)
- [dataflow const-prop limits](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/dataflow_const_prop.rs#L31-L76)
- [coroutine layout conflicts](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/coroutine/layout.rs#L229-L288)
- [coroutine state transform](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/coroutine/mod.rs#L1032-L1179)
- [`no_hash` and disk-cache behavior](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/src/doc/rustc-dev-guide/src/queries/incremental-compilation-in-detail.md#L450-L486)

### Performance fixtures

rustc-perf source revision:
`58b05b9a296dfb148aa54c9ec61e8890c65a4223`.

- [compile benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/benchmark_set/compile_benchmarks.rs#L6-L82)
- [benchmark descriptions](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)
- [scenario model](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/scenario.rs#L1-L23)
- [profile model](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/profile.rs#L9-L25)

## Findings

### FERRIUM-167: MIR is a sequence of ownership and query boundaries

**Sources:** [MIR query declarations](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L597-L716), [`Steal`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_data_structures/src/steal.rs#L1-L66), and [experiment boundaries](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#compiler-boundaries).

**Observed behavior:** `thir_body`, `mir_built`, promotion, drop-elaborated MIR,
CTFE MIR, and optimized MIR are distinct. `Steal` avoids cloning the main body;
whole-body pass traversal and generated bodies create repeated work.

**Implication:** FERRIUM must report query ownership, pass schedule, generated
bodies, self time, and total time separately.

**Confidence:** High.

### FERRIUM-168: MIR construction follows body topology, not bytes alone

**Sources:** [Experiment construction scaling](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#construction-and-primary-totals).

**Observed behavior:** `mir_built` rose from 1.34 ms at 100 temporaries to
86.66 ms at 10,000. A 92.8 KB wide aggregate measured only 1.52 ms.

**Implication:** Reports need statements, locals, expressions, blocks, and
owner shape; source size is context only.

**Confidence:** High for the generated controls.

### FERRIUM-169: pass traversal can dominate without successful rewrites

**Sources:** [pass manager](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/pass_manager.rs#L13-L117) and [experiment pass attribution](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#pass-level-attribution).

**Observed behavior:** The level-2 inliner spent 58.14 ms scanning the
10,000-temporary body, while destination propagation spent 60.71 ms.

**Implication:** Pass time is not evidence that a pass changed the body.
Input scale, eligibility, changes made, and later-body size must remain
separate.

**Confidence:** High.

### FERRIUM-170: MIR optimization levels are topology-sensitive

**Sources:** [optimization defaults](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/session.rs#L800-L805) and [interleaved level matrix](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#optimization-level-controls).

**Observed behavior:** Level 2 materially increased the two 10,000-operation
bodies and partial moves, while smaller controls generally stayed within
noise. Levels 3 and 4 added no universal penalty or benefit.

**Implication:** MIR-level comparisons require interleaving and pass evidence;
sequential series and one total cannot rank optimization policies.

**Confidence:** High for the tested encoded-MIR workflow.

### FERRIUM-171: partial moves create a distinct drop-elaboration hot spot

**Sources:** [drop elaboration](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/elaborate_drops.rs#L21-L94) and [drop scaling](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#drop-promotion-and-cleanup-topology).

**Observed behavior:** Drop elaboration rose from 0.73 ms at 100 partial moves
to 12.18 ms at 1,000; the primary level-2 median was 12.90 ms.

**Implication:** Move paths, drop-needing fields, partial initialization,
cleanup edges, and drop flags must accompany drop-cost claims.

**Confidence:** High.

### FERRIUM-172: promoted bodies multiply MIR work but do not isolate CTFE

**Sources:** [promoted MIR queries](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L655-L754) and [promotion control](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#drop-promotion-and-cleanup-topology).

**Observed behavior:** One thousand promoted arrays spent 2.25 ms in
`PromoteTemps`, alongside 15-16 ms of MIR construction and other passes.

**Implication:** Promotion count, promoted-body size, runtime MIR, and CTFE
execution must not be collapsed into one label.

**Confidence:** High.

### FERRIUM-173: coroutine state transformation has its own topology

**Sources:** [coroutine layout](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/coroutine/layout.rs#L229-L288), [state transform](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/coroutine/mod.rs#L1032-L1179), and [coroutine scaling](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#coroutine-topology).

**Observed behavior:** State-transform self time reached 15.30 ms for 100
sequential saved-local awaits and 19.87 ms for 100 locals live across 100
suspensions.

**Implication:** Await count, saved locals, simultaneous liveness, storage
conflicts, borrows, and drop shims are separate dimensions.

**Confidence:** High for the generated controls.

### FERRIUM-174: inlining trades current pass cost for later body shape

**Sources:** [inlining policy](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/inline.rs#L262-L383) and [inlining controls](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#inlining-and-validation-controls).

**Observed behavior:** Disabling inlining made 10,000 wrapping calls slower and
larger, but did not materially improve the partial-move fixture.

**Implication:** FERRIUM must not recommend `inline`, `inline(always)`, or pass
disabling from inliner time alone.

**Confidence:** High for the bounded controls.

### FERRIUM-175: optimized-MIR reuse is dependency- and policy-sensitive

**Sources:** [`cache_on_disk`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/src/doc/rustc-dev-guide/src/queries/incremental-compilation-in-detail.md#L482-L486) and [incremental matrix](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#incremental-edit-matrix).

**Observed behavior:** Unchanged sessions ran no MIR providers; a local edit
missed one body; a shared const missed 1,000 optimized bodies. Forced inlining
made one helper edit miss 1,002 optimized bodies.

**Implication:** Reports need edit class, dependency frontier, incremental
mode, inlining policy, query hits/misses, and cache-load cost.

**Confidence:** High for query counts.

### FERRIUM-176: CTFE, validation, and MIR serialization are separate observers

**Sources:** [CTFE MIR path](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_mir_transform/src/lib.rs#L491-L531) and [experiment controls](perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md#ctfe-failures-and-observer-effects).

**Observed behavior:** CTFE execution dominated its MIR preparation; validation
added measurable observer cost; textual MIR output exceeded 800 MB for one
fixture and was excluded.

**Implication:** CTFE, validation, dumps, encoding, self-profile, and complete
wall time remain separately labeled. Failed and stopped runs stay visible.

**Confidence:** High.

## Recommendations

### Adopt now

- Extend read-only build explanations with MIR body, pass, drop, promotion,
  coroutine, optimization-level, and edit-frontier vocabulary.
- Retain orthogonal synthetic fixtures and stable complete metadata totals.
- Use binary encoded-MIR and self-profile only as versioned diagnostic layers.
- Preserve failure stderr, stopped runs, query misses, and observer overhead.

### Prototype behind a compatibility boundary

- A nightly adapter joining `mir_built`, `optimized_mir`, and `mir_pass_*`
  events to source owners and controlled edits.
- Parametric rustc-perf candidates for partial moves, promoted bodies,
  saved-local coroutines, and pass traversal.
- Finer compiler counters only with maintainer guidance and explicit approval.

### Reject or defer

- Automatic source, async, const, match, aggregate, or function rewrites.
- Production MIR-level, inlining, validation, or pass-policy changes.
- Treating MIR dumps, encoded output size, or query self time as wall time.
- Compiler forks, custom MIR optimizers, or upstream filing without approval.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: required semantic transforms remain distinct from optional optimization, and validation is not disabled. |
| Compiler Performance Engineer | Accepted: stable totals, interleaved levels, pass profiles, scaling, incrementality, and observer effects are separated. |
| Interop Boundary Auditor | Accepted: no ABI or cross-language recommendation follows from synthetic MIR evidence. |
| AI Assurance Skeptic | Accepted: the stopped text-emission run, noisy sequential levels, expected failures, and diagnostic limits remain visible. |
| Ecosystem Strategist | Accepted: the recommendation begins with rustc-perf fixtures and diagnostics rather than replacement tooling. |
| Rust Maintainer | Accepted: ordinary stable compilation remains primary and no compiler ritual is proposed for adopters. |
| Native Platform Adopter | Accepted: no production flag, migration, or workflow requirement is introduced. |
| Scope Keeper | Accepted: the work answers PERF-Q15 and leaves parallelism, broader invalidation, hashing, and codegen to later questions. |
| Validation Checker | Accepted: commands, revisions, distributions, profiles, scaling, edit controls, failures, and limitations are recorded. |

## Non-goals

- Ranking all MIR passes or optimization levels.
- Measuring LLVM, monomorphization, object emission, or linking.
- Proving a compiler regression from synthetic fixtures.
- Recommending semantic rewrites or unstable production flags.
- Creating upstream issues, branches, comments, or pull requests.
