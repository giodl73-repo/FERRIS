# Type Inference and Type Checking

Date: 2026-08-08
Question: PERF-Q12
Status: Complete
Decision: add body-owner, inference, expected-type, coercion, pattern,
writeback, trait-obligation, and type-result invalidation evidence to
FERRIUM; prioritize read-only per-owner diagnostics and orthogonal fixtures
over annotation rewrites, API changes, or compiler forks.

## Executive conclusion

Rust type checking has two useful boundaries:

1. item-signature and well-formedness work in `rustc_hir_analysis`;
2. body-owner inference in the disk-cached `typeck_root(LocalDefId)` query.

Within one body, however, inference is not split into reusable subqueries.
Expression checking, expected-type propagation, coercion, deferred casts,
fallback, closure analysis, obligation selection, and writeback run inside one
`typeck_root` evaluation. Trait solving is interleaved with that pipeline, so
the event is a body type-checking boundary rather than a pure unification
timer.

Controlled trait-light fixtures exposed several distinct costs. A single body
with 10,000 inferred binding equalities measured 26.32 ms of `typeck_root`
self time. Adding 9,999 generic identity calls raised that to 104.84 ms while
the separately reported `evaluate_obligation` self time remained 0.06 ms.
Ten thousand function-item coercions measured 163.98 ms, tuple patterns
176.95 ms, and expected-type-guided `Option` constructors 117.90 ms.

Explicit annotations were not a shortcut. The 10,000-binding annotated body
measured 25.19 ms of type checking versus 26.32 ms inferred, while its larger
source and HIR coincided with a slower stable total: 209.90 versus 172.49 ms.

Owner topology changed both overhead and available parallelism. Ten thousand
small inferred owners measured 79.64 ms of aggregate `typeck_root` self time
and 14.88 ms of borrow checking, compared with 26.32 and 0.13 ms for one large
body. Eight frontend jobs did not improve the one-body generic control, but
reduced the robust many-owner generic median from 527.71 to 352.85 ms.

Incremental type checking was materially different from the earlier frontend.
`typeck_root` is `cache_on_disk`. Untouched and identically rewritten source
executed no `typeck_root` events in the retained profiles. A one-body edit and
a helper-body edit each produced one miss while callers remained cached. A
shared type-alias target change produced 2,001 misses across all body owners.

The complete wall difference still cannot be called “type inference.”
Matching, liveness, item checking, MIR construction, borrow checking, lints,
metadata, cache loading, and observer overhead remain separate. FERRIUM should
therefore expose per-owner type-checking cost, query reuse, source shape, and
shared type dependencies without prescribing annotations or API rewrites.

Credible contribution paths are orthogonal rustc-perf fixtures for trait-light
generic equality, coercion, expected types, patterns, owner width, and
localized versus shared-type edits. Finer stable subphase events inside
`typeck_root` require maintainer guidance.

No upstream activity was created.

## Decision supported

This research determines:

- which trait-light language shapes produce disproportionate body type-check
  work;
- where item checking, body checking, trait solving, liveness, and borrow
  checking separate or remain entangled;
- how owner count changes fixed query overhead and parallelism;
- which edits reuse disk-cached type results and which shared type changes
  invalidate broadly;
- which diagnostics and fixtures FERRIUM can defend.

It does not authorize adding type annotations, simplifying APIs, splitting
functions, changing generic signatures, changing aliases, replacing inference,
or creating upstream activity.

## Evidence reviewed

### Local evidence

- [Name resolution and HIR lowering](2026-08-08-name-resolution-hir-lowering.md)
- [Experiment](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [`check_crate`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_analysis/src/lib.rs#L103-L151)
- [`typeck_root` query definition](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L1214-L1233)
- [`check_well_formed` query definition](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L1895-L1901)
- [`typeck_with_inspect`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_typeck/src/lib.rs#L66-L245)
- [`TypeckRootCtxt`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_typeck/src/typeck_root_ctxt.rs#L27-L95)
- [`TypeckResults`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/ty/typeck_results.rs#L32-L130)
- [type inference fallback](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_typeck/src/fallback.rs)
- [type-result writeback](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_typeck/src/writeback.rs)
- [coercion implementation](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_typeck/src/coercion.rs)

### Performance fixtures

- [`unify-linearly`](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/unify-linearly/src/main.rs)
- [`coercions`](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/coercions/src/main.rs)
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)

## Type-checking model

```text
collected item signatures and predicates
  -> check crate-wide type well-formedness and coherence
  -> enumerate HIR body owners
  -> evaluate typeck_root independently for each root owner
       -> gather locals
       -> check expressions and patterns against expectations
       -> create and unify inference variables
       -> register and select trait obligations
       -> accumulate branch and return coercions
       -> check deferred repeats, casts, closures, and sized obligations
       -> apply integer, float, and diverging fallback
       -> report ambiguity
       -> write resolved node types, arguments, adjustments, captures,
          coercions, and hidden types into TypeckResults
  -> stable-hash and optionally load/store TypeckResults on disk
  -> build THIR and MIR
  -> run borrow checking and later analysis
```

Closures and inline bodies that share one type-check root also share one
inference environment. The owner query is the reusable boundary; expression
subphases are not independently cached.

## Findings

### FERRIUM-137: item checking and body inference have distinct query
boundaries

**Sources**

- [`check_crate`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_analysis/src/lib.rs#L103-L151)
- [`typeck_root` query definition](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L1214-L1233)
- [Experiment: compiler boundaries](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#compiler-boundaries)

**Observed behavior**

`check_type_wf` and `check_well_formed` cover crate and item signature
well-formedness. `check_crate` then drives `typeck_root` across HIR body
owners. `typeck_root` is keyed by a local definition and cached on disk.

**Implication**

FERRIUM must distinguish item signatures, body roots, nested closures, and
later MIR or borrow work. One crate-wide “type checking” value loses the
incremental and parallel boundary.

**Confidence:** High.

### FERRIUM-138: explicit annotations did not reduce the measured body cost

**Sources**

- [Experiment: primary fixture matrix](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#primary-fixture-matrix)

**Observed behavior**

Ten thousand annotated bindings measured 25.19 ms of `typeck_root` self time.
The inferred chain measured 26.32 ms. Stable totals were 209.90 and 172.49 ms,
and HIR records were 90,018 and 60,018.

**Implication**

Adding annotations is not a general compile-time recommendation. Annotation
syntax and HIR can offset any reduction in inference work, and annotations can
change diagnostics and maintenance burden.

**Confidence:** High for the controlled equality chain.

### FERRIUM-139: generic equality propagation created a localized body hot
spot

**Sources**

- [`typeck_with_inspect`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_typeck/src/lib.rs#L97-L245)
- [Experiment: self-profile attribution](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#self-profile-attribution)

**Observed behavior**

Replacing plain binding equalities with 9,999 calls to an unconstrained
generic identity function raised one-body `typeck_root` self time from 26.32
to 104.84 ms. The separately visible `evaluate_obligation` event remained
0.06 ms.

**Implication**

Generic call count, inferred argument count, and equality propagation belong
in the body model even when explicit trait work is small. The aggregate event
still includes all inference and obligation interaction in that body.

**Confidence:** High for this trait-light generic control.

### FERRIUM-140: coercions, patterns, and expected types are independent
cost dimensions

**Sources**

- [coercion implementation](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_typeck/src/coercion.rs)
- [Experiment: self-profile attribution](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#self-profile-attribution)

**Observed behavior**

Ten thousand function-item coercions in one body measured 163.98 ms of
`typeck_root` self time. Ten thousand tuple patterns measured 176.95 ms.
Ten thousand expected-type-guided `Option<u32>` constructors measured
117.90 ms.

The fixtures retained used locals so dead-local liveness did not dominate.

**Implication**

A useful report needs coercion sites, branch LUBs, pattern complexity,
expected-type propagation, generic arguments, and body-local node volume.
Expression count alone is insufficient.

**Confidence:** High for the controlled fixtures.

### FERRIUM-141: owner width adds overhead but exposes body-level parallelism

**Sources**

- [`check_crate`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_analysis/src/lib.rs#L122-L151)
- [Experiment: owner topology and frontend jobs](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#owner-topology-and-frontend-jobs)

**Observed behavior**

One inferred body had one `typeck_root` miss, 26.32 ms of type-check self
time, and 0.13 ms of borrow checking. Ten thousand small owners had 10,000
misses, 79.64 ms of type-check self time, and 14.88 ms of borrow checking.

Eight jobs did not accelerate the one-body generic control. The many-owner
generic median fell from 527.71 to 352.85 ms, a 33.1% reduction; both series
were below 10% relative MAD.

**Implication**

Function splitting trades owner-query, MIR, borrow, metadata, and maintenance
overhead against parallel scheduling. FERRIUM may explain this topology but
must not recommend automatic function splitting.

**Confidence:** High for the many-owner generic job result; medium for broader
generalization.

### FERRIUM-142: body type results are reused across compiler invocations

**Sources**

- [`typeck_root` query definition](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L1225-L1228)
- [`TypeckResults`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/ty/typeck_results.rs#L32-L130)
- [Experiment: incremental edit matrix](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#incremental-edit-matrix)

**Observed behavior**

`typeck_root` is marked `cache_on_disk`, and `TypeckResults` derives
`StableHash`. Untouched and identically rewritten source produced no
`typeck_root` event in five reused-directory profiles.

**Implication**

Unlike parsing, resolution, and HIR lowering, unchanged body type results can
be loaded without rerunning the query provider. FERRIUM should report this as
body-result reuse, not whole-frontend reuse.

**Confidence:** High.

### FERRIUM-143: body edits localized while a shared type change invalidated
all owners

**Sources**

- [Experiment: incremental edit matrix](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#incremental-edit-matrix)

**Observed behavior**

The 2,001-owner fixture produced:

- one `typeck_root` miss after changing one body literal;
- one miss after changing the generic helper body without changing its
  signature;
- 2,001 misses after changing the shared `Scalar` alias target.

The helper's 2,000 callers remained cached because the callable signature did
not change.

**Implication**

Edit breadth follows query dependencies and shared type identity, not text
distance or call count alone. Reports need changed body, signature, alias,
generic predicate, and downstream-owner dimensions.

**Confidence:** High for the fixture.

### FERRIUM-144: trait solving is interleaved with body inference

**Sources**

- [`TypeckRootCtxt`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_typeck/src/typeck_root_ctxt.rs#L27-L95)
- [`typeck_with_inspect`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_typeck/src/lib.rs#L97-L245)
- [`unify-linearly`](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/unify-linearly/src/main.rs)

**Observed behavior**

One `TypeckRootCtxt` owns both the inference context and fulfillment engine.
Obligations are registered and selected during expression checking, fallback,
coercion, method lookup, closure analysis, and finalization.

Even rustc-perf's unification regression fixture uses traits, associated
types, and method calls.

**Implication**

`typeck_root` is not a pure inference timer. Q12 uses trait-light controls and
records obligation events; Q13 must investigate trait graphs and solver work
directly.

**Confidence:** High.

### FERRIUM-145: the semantic wall gap contains substantial non-typeck work

**Sources**

- [Experiment: self-profile attribution](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#self-profile-attribution)
- [Experiment: expected failures](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#expected-failures)

**Observed behavior**

Stable minus no-analysis time was much larger than the profiled
`typeck_root` event for every nontrivial fixture. Matching, liveness,
well-formedness, MIR, borrow checking, lints, metadata, and observer overhead
also contributed.

Unconstrained `Option`, unconstrained closure, and incompatible coercion
controls retained exit status 1 and complete diagnostics.

**Implication**

FERRIUM must not label the complete post-resolution wall interval “inference.”
Failed inference is correctness evidence, not a fast path.

**Confidence:** High.

### FERRIUM-146: current evidence exposes owner cost but not intra-body cause

**Sources**

- [Experiment: diagnostics assessment](perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md#diagnostics-assessment)
- [`coercions`](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/coercions/src/main.rs)
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)

**Observed behavior**

Self-profile identifies expensive `typeck_root` owners and cache misses.
Input stats provide AST/HIR volume. Neither surface separates per-owner time
into unification, expected types, coercion, fallback, writeback, pattern
checking, closure analysis, and nested obligation work.

rustc-perf contains broad coercion and unification regressions, but not the
full orthogonal owner, edit, and trait-light matrix measured here.

**Implication**

FERRIUM's defensible contribution is a source-attributed owner report and
orthogonal fixtures. Stable intra-body events are a possible upstream path
after maintainer guidance.

**Confidence:** High for the reviewed surfaces.

## Recommendations

### Adopt now

1. Add item-WF, type-check-root, inference-variable, expectation, coercion,
   fallback, writeback, result-hash, owner-width, and edit-dependency
   vocabulary to the measurement contract.
2. Preserve stable complete time, no-analysis, time passes, self-profile,
   input stats, job count, incremental edits, and failures as separate
   evidence.
3. Retain trait-light generic, coercion, expected-type, pattern, and
   owner-topology controls.
4. Record explicit trait, method, operator, closure, and projection confounds.
5. Preserve diagnostic output for ambiguous or incompatible inference.

Owners: FERRIUM Compiler Performance Engineer, Rust Safety Steward, and
Validation Checker.

Validation: repeated distributions, exact source manifests, query events,
cache misses, job controls, failure stderr, and cross-platform follow-up.

### Prototype behind a compatibility boundary

Prototype a read-only report that combines:

- body owner and enclosing item;
- source span and local HIR records;
- explicit versus inferred types;
- generic call and inferred-argument count;
- expected-type and fallback sites;
- branch/return coercions and casts;
- pattern and closure shape;
- trait methods, operators, projections, and obligation events;
- `typeck_root` self time, cache hits, misses, and load time;
- owner width, frontend jobs, edit class, and downstream invalidation.

The prototype must consume ordinary source and optional nightly profiles. It
must not claim to reproduce rustc inference or recommend source changes from
counts alone.

Owners: FERRIUM with rustc remaining authoritative.

Validation: synthetic controls, held-out real crates, source-attribution
accuracy, false-positive review, stable/nightly agreement, privacy review, and
removable adapters.

### Reject or defer

- automatic type annotation insertion;
- automatic function splitting or merging;
- generic, alias, coercion, cast, closure, or pattern rewrites;
- public API or trait-bound simplification;
- custom inference or trait-solving engines;
- persistent compiler daemons;
- direct rustc-internal product dependencies;
- upstream issue or PR creation without explicit approval.

These can change inference, fallback, coercion, borrow behavior, diagnostics,
public API, code generation, and maintenance cost.

## Candidate contribution paths

No upstream activity was created. If the user later approves outreach,
maintainer guidance should precede any branch, issue, comment, or pull request.

Candidate paths:

1. a rustc-perf matrix for plain equality versus trait-light generic calls;
2. function-item coercion, tuple-pattern, and expected-type fixtures;
3. one-body versus many-owner frontend-job controls;
4. localized body, helper-body, signature, alias, and predicate edits;
5. source-attributed `typeck_root` owner reporting;
6. stable intra-body events for coercion, fallback, writeback, and pattern
   checking.

Each path needs current Linux rustc-perf reproduction, benchmark cost review,
diagnostic stability discussion, trait-solver separation, and
project-specific maintainer approval.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: annotations, aliases, coercions, generic APIs, and function boundaries are not rewritten or described as semantics-neutral. |
| Compiler Performance Engineer | Accepted: stable totals, no-analysis, type-check passes, owner profiles, matching, liveness, borrow checking, jobs, incrementality, failures, and observer effects remain separate. |
| Interop Boundary Auditor | Accepted: source, HIR, item-WF, body typeck, trait obligations, MIR, borrow checking, incremental cache, and compiler-version boundaries remain explicit. |
| AI Assurance Skeptic | Accepted: counts do not become causal claims, the noisy single-body job series supports no speedup claim, and trait solving is not silently attributed to inference. |
| Ecosystem Strategist | Accepted: rustc and rustc-perf remain authoritative; FERRIUM supplies decomposition, attribution, and orthogonal fixtures. |
| Rust Maintainer | Accepted: ordinary Rust APIs and Cargo workflows remain unchanged; diagnostics do not prescribe annotation or abstraction churn. |
| Native Platform Adopter | Accepted: Windows, NTFS, warm-cache scope, nightly diagnostics, and missing Linux/macOS evidence are explicit. |
| Scope Keeper | Accepted: Q12 covers body inference and type checking; trait graphs, borrow checking, MIR, and broad invalidation remain later questions. |
| Validation Checker | Accepted: exact generators, toolchains, source sizes, 15- and 30-sample distributions, five-run profiles, expected failures, source revisions, and limitations are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

## Decision

PERF-Q12 is complete.

FERRIUM should model type checking as disk-cached body-owner work with
intra-body inference, coercion, pattern, expected-type, fallback, obligation,
and writeback dimensions. The next question is PERF-Q13: determine which
trait graphs, projections, normalization cases, and solver cycles create
disproportionate work.
