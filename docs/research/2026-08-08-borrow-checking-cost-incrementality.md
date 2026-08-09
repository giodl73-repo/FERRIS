# Borrow-Checking Cost and Incrementality

Date: 2026-08-08
Question: PERF-Q14
Status: Complete
Decision: add promoted-MIR, move-path, loan-lifetime, place-conflict,
region-constraint, CFG, nested-body, Polonius-mode, and borrow-edit evidence to
FERRIUM; prioritize read-only per-owner topology diagnostics and orthogonal
fixtures over borrow rewrites, unsafe workarounds, or custom checkers.

## Executive conclusion

Borrow checking can dominate a semantic owner, but raw borrow count is not a
reliable predictor. In the primary profiles, 10,000 shared borrows consumed
immediately produced 209.47 ms of `mir_borrowck` self time. Only 4,000 shared
borrows retained simultaneously produced 207.67 ms. An orthogonal 4,000-loan
sweep measured 64.89 ms when each borrow ended immediately and 198.28 ms when
all remained live until the end.

The cost difference follows the compiler architecture. `mir_borrowck` clones
promoted MIR, renumbers regions, gathers move paths and loans, performs a
second MIR type check, constructs and solves region constraints, computes
three iterative dataflow analyses, checks place conflicts, and emits
diagnostics. Loan lifetime, active-set overlap, move-path topology, projection
shape, CFG joins and backedges, region graph structure, and nested body
requirements all matter.

Large source shapes did not imply borrow-check dominance. Two thousand
non-`Copy` partial moves measured 61.40 ms of borrow checking but 258.95 ms of
HIR type checking. A 2,001-loan mutable reborrow chain measured only 5.82 ms
of borrow checking. One thousand closures measured 56.45 ms, while their root
caused 1,001 MIR bodies to be built and promoted.

Async work showed why query total and query self time must remain separate.
Two hundred borrows across await points measured 23.48 ms of `mir_borrowck`
self time but 129.15 ms total time inside the query, including nested
coroutine-related work. Its stable complete median was 395.45 ms and nightly
Polonius-off median was 291.94 ms. That total cannot be labeled borrow
checking alone.

`mir_borrowck` is a per-typeck-root query but is not marked
`cache_on_disk`. No reusable serialized loan, move, region, or diagnostic
result exists. Incremental dependency tracking can still avoid provider
execution: untouched and identically rewritten source produced no
`mir_borrowck` events. One caller-body or helper-body edit produced one miss.
A shared type-alias change produced 1,001 misses across the helper and all
callers.

The default pinned compiler uses ordinary NLL with Polonius off. Legacy and
next Polonius remain experimental modes. In bounded 15-run controls, legacy
measured 721.08 ms for 100 immediate loans and 782.84 ms for 100 overlapping
loans, versus 98.71 and 107.93 ms with Polonius off. Several small mode series
were noisy, so only the large legacy gap is directional evidence. A larger
legacy matrix was cancelled after 155 minutes because it made the research
operationally unbounded. Polonius-next stayed near off on these small controls,
but this does not establish production readiness or equivalence.

FERRIUM should expose owner-level borrow-check cost together with MIR scale,
loan lifetime, move and projection topology, region constraints, CFG shape,
nested bodies, mode, and incremental dependency. It should not recommend
shortening borrows, cloning, changing ownership, splitting functions, adding
`unsafe`, or enabling experimental modes from counts alone.

Credible contribution paths are orthogonal rustc-perf fixtures for immediate
versus overlapping loans, owner width, move paths, CFG joins, closures,
coroutines, diagnostics, and localized versus shared-type edits. Finer
borrow-check subphase counters require maintainer guidance.

No upstream activity was created.

## Decision supported

This research determines:

- when borrow checking became a material part of controlled compilation;
- which loan-lifetime, owner, CFG, move, closure, and coroutine shapes changed
  cost;
- where MIR construction, HIR type checking, borrow checking, and later MIR
  work separate or remain nested;
- how borrow-check queries reuse incremental dependencies despite lacking a
  serialized result;
- how production NLL and experimental Polonius modes must be identified;
- which diagnostics and minimized fixtures FERRIUM can defend.

It does not authorize ownership rewrites, cloning, lifetime changes, function
splitting, `unsafe`, Polonius enablement, compiler forks, or upstream activity.

## Evidence reviewed

### Local evidence

- [Trait-solving cost and reuse](2026-08-08-trait-solving-cost-reuse.md)
- [Experiment](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [THIR and MIR query declarations](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L599-L712)
- [`mir_borrowck` query declaration](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L1239-L1246)
- [`mir_borrowck` provider entry](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/lib.rs#L109-L145)
- [borrow-check setup and region renumbering](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/lib.rs#L318-L397)
- [NLL, dataflow, and final MIR walk](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/lib.rs#L397-L650)
- [borrow-set representation](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/borrow_set.rs#L20-L170)
- [place-conflict checking](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/places_conflict.rs#L1-L110)
- [region inference context](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/region_infer/mod.rs#L82-L120)
- [region-constraint propagation](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/region_infer/mod.rs#L481-L780)
- [nested closure and coroutine roots](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/root_cx.rs#L150-L350)
- [Polonius configuration](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/config.rs#L3618-L3643)
- [Polonius-next implementation boundary](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/polonius/mod.rs#L1-L200)

### Performance fixtures

rustc-perf source revision:
`58b05b9a296dfb148aa54c9ec61e8890c65a4223`.

- [`ucd`](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/ucd)
- [`wg-grammar`](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/wg-grammar)
- [`await-call-tree`](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/await-call-tree)
- [`match-stress`](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/match-stress)
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)

## Borrow-checking model

```text
type-check root
  -> build THIR
  -> lower THIR to MIR
  -> elaborate drops and check constants
  -> promote MIR
  -> evaluate mir_borrowck for the root
       -> collect nested closure and coroutine bodies
       -> clone MIR and replace regions with inference variables
       -> gather move paths, moves, and initialization events
       -> gather loans and two-phase activation points
       -> type-check MIR and generate liveness/outlives/type-test constraints
       -> solve region constraints over SCCs and CFG points
       -> run borrow, maybe-uninitialized, and ever-initialized dataflow
       -> compare accesses against active loans and move state
       -> propagate closure requirements and opaque-type results
       -> buffer diagnostics or return hidden type information
  -> continue MIR transforms and optimized_mir
```

Drop elaboration and optimized MIR are adjacent but separate work. The
`mir_borrowck` query's total time can include nested dependencies; self time
is the closer event-local boundary.

## Findings

### FERRIUM-157: borrow checking has a distinct per-root query boundary

**Sources**

- [THIR and MIR query declarations](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L599-L712)
- [`mir_borrowck` query declaration](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L1239-L1246)
- [Experiment: compiler boundaries](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#compiler-boundaries)

**Observed behavior**

`thir_body`, `mir_built`, `mir_promoted`, `mir_borrowck`, and
`optimized_mir` are separate queries. `mir_borrowck` is evaluated for typeck
roots and returns hidden opaque-type information rather than a public loan or
region-analysis result.

**Implication**

FERRIUM must compare these boundaries before attributing a complete semantic
or MIR interval to borrow checking.

**Confidence:** High.

### FERRIUM-158: active loan lifetime mattered more than loan count alone

**Sources**

- [borrow-set representation](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/borrow_set.rs#L20-L170)
- [Experiment: loan-count and lifetime scaling](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#loan-count-and-lifetime-scaling)

**Observed behavior**

At 4,000 loans, immediate consumption measured 64.89 ms of borrow-check self
time while retaining every loan measured 198.28 ms. The primary 10,000
immediate and 4,000 overlapping fixtures both measured about 208-209 ms.

**Implication**

Reports need loan count, live range, active-set overlap, borrow kind, and use
topology. Counting `&` expressions is insufficient.

**Confidence:** High for the generated shared-borrow controls.

### FERRIUM-159: mutable reborrow depth was not independently expensive

**Sources**

- [Experiment: primary fixture matrix](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#primary-fixture-matrix)

**Observed behavior**

A 2,001-loan mutable reborrow chain measured 5.82 ms of `mir_borrowck` self
time and 95.71 ms stable complete time.

**Implication**

Syntactic depth and loan count do not establish active conflict or region
complexity. Reborrow chains need measured MIR and liveness context.

**Confidence:** High for this linear chain.

### FERRIUM-160: one large owner and many small owners have different costs

**Sources**

- [nested closure and coroutine roots](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/root_cx.rs#L150-L350)
- [Experiment: owner scaling](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#owner-scaling)

**Observed behavior**

Five thousand one-borrow owners produced 5,000 `mir_borrowck` misses and
81.61 ms aggregate self time. The scaling medians rose from 2.19 ms at 100
owners to 85.43 ms at 5,000.

**Implication**

Owner width adds query setup and scheduling opportunity. It is not equivalent
to placing the same number of loans in one MIR body and does not justify
automatic function splitting.

**Confidence:** High.

### FERRIUM-161: neighboring phases can dominate borrow-heavy source shapes

**Sources**

- [Experiment: self-profile attribution](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#self-profile-attribution)

**Observed behavior**

The 2,000-field partial-move fixture measured 258.95 ms of `typeck_root`,
37.52 ms of `mir_built`, and 61.40 ms of `mir_borrowck` self time. The
1,000-join fixture measured 53.70, 15.65, and 38.53 ms respectively.

**Implication**

Partial moves, drops, control flow, and aggregate volume must be attributed
across HIR type checking, MIR construction, borrow checking, drop elaboration,
and later MIR work.

**Confidence:** High for the profiled boundaries.

### FERRIUM-162: nested closures and coroutines widen the root query

**Sources**

- [nested closure and coroutine roots](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/root_cx.rs#L150-L350)
- [Experiment: nested body controls](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#nested-body-controls)

**Observed behavior**

One thousand closures caused 1,001 `mir_built` and `mir_promoted` misses but
one root `mir_borrowck` miss, with 56.45 ms self time. Two hundred awaits
produced two borrow-check misses, 23.48 ms self time, and 129.15 ms total time
inside the query.

**Implication**

Closure count, capture shape, coroutine states, awaits, opaque types, and
nested query time must accompany root cost. Query total is not pure
borrow-check self work.

**Confidence:** High for these generated roots.

### FERRIUM-163: `mir_borrowck` is not a serialized incremental result

**Sources**

- [`mir_borrowck` query declaration](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L1239-L1246)
- [Experiment: incremental edit matrix](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#incremental-edit-matrix)

**Observed behavior**

The query has no `cache_on_disk` declaration. Untouched and identically
rewritten incremental sessions nevertheless executed no borrow-check provider
because the dependency graph kept the ensured work green.

**Implication**

FERRIUM must distinguish skipping a green query from loading a serialized
borrow set or region solution. The latter does not occur.

**Confidence:** High.

### FERRIUM-164: body edits localized while a shared type change invalidated all roots

**Sources**

- [Experiment: incremental edit matrix](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#incremental-edit-matrix)

**Observed behavior**

One caller-body edit and one helper-body edit each produced one
`typeck_root`, `mir_built`, and `mir_borrowck` miss. Changing a shared
`Scalar` alias produced 1,001 misses for each query.

**Implication**

Borrow-check recomputation follows body, MIR, signature, and shared-type
dependencies rather than text distance. Reports need exact edit class.

**Confidence:** High for the query counts; wall distributions were noisy.

### FERRIUM-165: Polonius modes are experimental and operationally distinct

**Sources**

- [Polonius configuration](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/config.rs#L3618-L3643)
- [Polonius-next implementation boundary](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_borrowck/src/polonius/mod.rs#L1-L200)
- [Experiment: Polonius controls](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#polonius-controls)

**Observed behavior**

The default is off. In bounded controls, legacy measured 721.08 ms for 100
immediate loans and 782.84 ms for 100 overlapping loans, versus 98.71 and
107.93 ms off. The larger legacy matrix was cancelled after 155 minutes.
Several small series were noisy, and next remained near off without proving
equivalence.

**Implication**

Mode is part of benchmark identity. Legacy and next are diagnostic research
surfaces, not automatic product configuration or a basis for broad ranking.

**Confidence:** High for mode identity and the large legacy gap; low for small
off-versus-next differences.

### FERRIUM-166: borrow failures remain correctness and diagnostic controls

**Sources**

- [Experiment: expected failures](perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md#expected-failures)

**Observed behavior**

Use-after-move, mutable/shared conflict, and returned-local-reference fixtures
retained exit status 1 and complete diagnostics. Medians ranged from 83.17 to
95.63 ms; the use-after-move series exceeded the 10% relative-MAD gate.

**Implication**

Move, conflict, lifetime, closure, and coroutine errors cannot be treated as
performance fast paths or repaired by semantics-changing rewrites.

**Confidence:** High for failure behavior; medium for failure timing.

## Recommendations

### Adopt now

1. Add borrow-check-root, promoted-MIR, move-path, loan-lifetime,
   place-conflict, region-constraint, CFG, nested-body, mode, and edit-class
   vocabulary to the measurement contract.
2. Preserve stable complete time, no-analysis, time passes, per-query self and
   total time, incremental events, and failures as separate evidence.
3. Retain immediate-versus-overlapping loan, reborrow, owner, CFG, move,
   closure, await, and shared-type fixtures.
4. Record query misses and nested-body counts before attributing wall time.
5. Keep Polonius controls bounded and explicitly experimental.

Owners: FERRIUM Rust Safety Steward, Compiler Performance Engineer, and
Validation Checker.

Validation: repeated distributions, exact generators, query events, edit
controls, failure stderr, current Linux rustc-perf follow-up, and
cross-platform reproduction.

### Prototype behind a compatibility boundary

Prototype a read-only report that combines:

- type-check root, nested closures/coroutines, and source span;
- THIR, built MIR, promoted MIR, borrow-check, and optimized-MIR events;
- MIR statements, locals, blocks, joins, backedges, and cleanup edges;
- move paths, move/init events, partial moves, and drop-relevant places;
- loans, borrow kind, live range, active overlap, and two-phase distance;
- place projection depth and conservative conflict kinds;
- region variables, outlives edges, SCCs, type tests, and closure requirements;
- Polonius mode, query self/total time, cache events, and edit class.

The adapter must remain optional and versioned. It must not reproduce the
borrow checker, infer safety from counts, or recommend source changes without
behavioral validation.

Owners: FERRIUM with rustc remaining authoritative.

Validation: synthetic controls, held-out borrow-heavy crates,
source-attribution accuracy, false-positive review, stable/nightly agreement,
privacy review, and removable adapters.

### Reject or defer

- automatic borrow shortening or scope insertion;
- automatic cloning, ownership conversion, or interior mutability;
- automatic function splitting or closure extraction;
- lifetime, async, generator, drop, or aggregate rewrites;
- adding `unsafe` to bypass diagnostics;
- enabling legacy or next Polonius in product builds from these results;
- custom borrow checkers or persistent compiler daemons;
- direct rustc-internal product dependencies;
- upstream issue or PR creation without explicit approval.

These can alter ownership, aliasing, destruction order, concurrency,
diagnostics, public API, memory use, runtime cost, and safety.

## Candidate contribution paths

No upstream activity was created. If the user later approves outreach,
maintainer guidance should precede any branch, issue, comment, or pull request.

Candidate paths:

1. equal-count immediate versus overlapping-loan fixtures;
2. one-body versus many-owner borrow topology;
3. projection-conflict, partial-move, and CFG-join sweeps;
4. closure-capture and coroutine-await controls;
5. localized body, helper-body, shared-type, and signature edits;
6. successful and failing variants with solver-mode metadata;
7. stable counters or events for loans, move paths, region constraints, and
   dataflow iterations.

Each path needs current Linux rustc-perf reproduction, benchmark cost review,
comparison with `ucd`, `wg-grammar`, and async fixtures, diagnostic stability,
and project-specific maintainer approval.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: no ownership, lifetime, cloning, async, drop, or unsafe rewrite is described as semantics-neutral; experimental modes are not production recommendations. |
| Compiler Performance Engineer | Accepted: stable totals, no-analysis, THIR/MIR boundaries, query self/total time, topology sweeps, incrementality, failures, variance, and observer effects remain separate. |
| Interop Boundary Auditor | Accepted: source, HIR typeck, MIR construction, borrow checking, drop, coroutine, incremental, toolchain, and upstream boundaries remain explicit. |
| AI Assurance Skeptic | Accepted: borrow count does not become a complexity law; noisy and cancelled controls remain visible; missing provider execution is not called serialized-result reuse. |
| Ecosystem Strategist | Accepted: rustc and rustc-perf remain authoritative; FERRIUM supplies orthogonal evidence and defers outreach. |
| Rust Maintainer | Accepted: ordinary Cargo and Rust usage remains unchanged; diagnostics target explanation rather than compiler rituals or unsafe workarounds. |
| Native Platform Adopter | Accepted: Windows, NTFS, warm-cache scope, unstable diagnostics, Polonius modes, cancellation, and missing Linux/macOS evidence are explicit. |
| Scope Keeper | Accepted: Q14 covers borrow checking; general MIR optimization, query invalidation, source refactoring, and codegen remain later questions. |
| Validation Checker | Accepted: exact generators, toolchains, 15- and 30-sample distributions, five-run profiles, scaling sweeps, incremental edits, failures, source revisions, noise, and limitations are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

## Decision

PERF-Q14 is complete.

FERRIUM should model borrow checking through promoted MIR, move and loan
topology, active lifetime, projection conflicts, region constraints, CFG
dataflow, nested bodies, mode, and edit dependencies. The next question is
PERF-Q15: determine which MIR construction and optimization work repeats
unnecessarily.
