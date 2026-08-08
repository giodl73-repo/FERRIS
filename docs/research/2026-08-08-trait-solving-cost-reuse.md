# Trait-Solving Cost and Reuse

Date: 2026-08-08
Question: PERF-Q13
Status: Complete
Decision: add solver mode, goal identity, candidate width, call count,
supertrait depth, projection depth, query visibility, and impl-set invalidation
evidence to FERRIUM; prioritize read-only topology diagnostics and orthogonal
fixtures over trait rewrites, custom solvers, or unsupported compiler claims.

## Executive conclusion

Trait-solving cost was governed more by goal topology than by a single count.
Ten thousand repeated concrete marker goals remained comparatively cheap:
stable metadata compilation measured 199.39 ms, and the old solver needed one
`evaluate_obligation` miss. Five thousand unique marker goals measured
288.95 ms and produced 5,000 misses. Repeated and unique associated-type
projections showed the same reuse distinction.

The largest tested hot spot was same-named method-candidate width. One thousand
traits each defined `value`, one type implemented one candidate, and one body
made 1,000 calls. Stable metadata compilation measured 2,664.59 ms versus
84.68 ms for the nightly no-analysis boundary. With the old solver enabled
everywhere, the final profiles recorded 3,526.83 ms of `typeck_root` self time
and 1,003,006 `evaluate_obligation` accesses. One million were cache hits.
The event's own self time was only 11.36 ms, so standalone query time did not
represent the complete search cost folded into method checking.

Independent sweeps showed both dimensions mattered. At 1,000 calls, widening
the candidate set from 10 to 1,000 traits raised old-solver wall time from
96.85 to 2,185.89 ms. At 1,000 candidate traits, increasing calls from 1 to
1,000 raised it from 137.16 to 2,195.45 ms. Source bytes, obligation count,
candidate count, and call count are therefore incomplete estimates in
isolation.

The globally enabled next solver was close to the old solver on ordinary
controls but slower on the candidate fixture. At 1,000 candidates and 1,000
calls, the scaling medians were 2,185.89 ms old and 2,744.73 ms global-next.
This is a fixture-specific result, not a general solver ranking. Current
nightly defaults to the next solver for coherence only; the experiment used
explicit `no` and `globally` modes to create interpretable endpoints.

Incremental behavior exposed a more important architectural distinction.
Untouched and identically rewritten source executed no `typeck_root` provider
under either solver. One caller-body edit and one impl-method-body edit each
produced one miss. Adding or removing an otherwise unrelated implementation
for the same trait produced one old-solver `evaluate_obligation` miss while
all 1,000 caller bodies remained reusable. Under the global next solver, the
same edit produced 1,000 `typeck_root` misses. A shared bound change produced
1,001 misses under both modes.

The old solver exposes a canonical `evaluate_obligation` query. The next
solver instead uses an in-process canonical global cache, a cycle-aware
provisional cache, fixpoint evaluation, and fulfillment fast paths. Ordinary
goal work is consequently less visible as a standalone query and can remain
inside `typeck_root`. FERRIUM should report solver mode, candidate topology,
query visibility, and edit dependency together rather than compare one event
name across solver architectures.

Credible contribution paths are orthogonal rustc-perf fixtures for repeated
versus unique goals, method-candidate width, call count, supertrait and
projection depth, and impl-set edit reuse. Any upstream issue, comment,
branch, or pull request still requires explicit user approval and maintainer
guidance.

No upstream activity was created.

## Decision supported

This research determines:

- which tested trait topologies produced disproportionate semantic work;
- where repeated canonical goals reused solver results;
- how method-candidate width and call count interacted;
- how old and global-next solver evidence must be interpreted differently;
- which body, impl-body, impl-set, and shared-bound edits remained local or
  invalidated broadly;
- which diagnostics and minimized fixtures FERRIUM can defend.

It does not authorize changing trait bounds, renaming methods, splitting
traits, replacing associated types, implementing a solver, enabling unstable
flags in product builds, or creating upstream activity.

## Evidence reviewed

### Local evidence

- [Type inference and type checking](2026-08-08-type-inference-checking.md)
- [Experiment](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [`NextSolverConfig`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/config.rs#L1014-L1021)
- [`parse_next_solver_config`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/options.rs#L1768-L1780)
- [`evaluate_obligation` query](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L2546-L2552)
- [old/new obligation dispatch](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs#L85-L117)
- [new-solver search graph](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_next_trait_solver/src/solve/search_graph.rs#L26-L48)
- [canonical global cache](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_type_ir/src/search_graph/global_cache.rs#L29-L45)
- [new-solver fulfillment fast paths](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_trait_selection/src/solve/fulfill.rs#L155-L181)
- [stalled-goal evaluation](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_trait_selection/src/solve/fulfill.rs#L197-L287)

### Performance fixtures

rustc-perf source revision:
`58b05b9a296dfb148aa54c9ec61e8890c65a4223`.

- [`projection-caching`](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/projection-caching)
- [`many-assoc-items`](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/many-assoc-items)
- [`deeply-nested-multi`](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/deeply-nested-multi)
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)

## Trait-solving model

```text
type-check owner
  -> register a predicate or method-lookup obligation
  -> canonicalize the goal and parameter environment
  -> enumerate relevant trait implementations and method candidates
  -> normalize associated types and nested projections
  -> evaluate nested goals, supertraits, cycles, and ambiguity
  -> reuse a solver-local or canonical cached result where eligible
  -> apply constraints and repeat fulfillment until stable
  -> report success, ambiguity, no solution, or overflow
  -> write the resolved result into TypeckResults
```

The old solver's canonical obligation query and the new solver's in-process
search-graph cache are not equivalent telemetry surfaces. Both remain
interleaved with owner type checking.

## Findings

### FERRIUM-147: solver mode is part of the benchmark identity

**Sources**

- [`NextSolverConfig`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/config.rs#L1014-L1021)
- [`parse_next_solver_config`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_session/src/options.rs#L1768-L1780)
- [Experiment: solver modes](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#solver-modes)

**Observed behavior**

Nightly defaults to next-solver coherence with old-solver body checking.
`-Znext-solver=no` disables it, while `globally` enables it throughout. A bare
`-Znext-solver` also means `globally`.

**Implication**

Reports must record the effective solver mode. “Nightly” and “next solver” are
not sufficient benchmark identities.

**Confidence:** High.

### FERRIUM-148: repeated canonical goals reused strongly

**Sources**

- [Experiment: primary fixture matrix](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#primary-fixture-matrix)
- [Experiment: self-profile attribution](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#self-profile-attribution)

**Observed behavior**

Ten thousand repeated marker goals produced one old-solver obligation miss;
10,000 repeated projections produced two. Five thousand unique marker goals
produced 5,000 misses, and 2,000 unique projection sites produced 4,000.

**Implication**

Raw obligation count is not a cost estimate. Canonical identity and cache
reuse must accompany the count.

**Confidence:** High for the controlled concrete goals.

### FERRIUM-149: tested supertrait and projection depth grew moderately

**Sources**

- [Experiment: structural scaling](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#structural-scaling)

**Observed behavior**

Stable supertrait-depth medians rose from 73.72 ms at depth 10 to 191.95 ms at
depth 400. Projection depth rose from 75.34 ms at depth 8 to 124.28 ms at
depth 256. Neither approached the method-candidate fixture's multi-second
cost.

**Implication**

Depth remains a required dimension, but the tested bounded chains do not
justify treating depth as the universal trait-solving hazard.

**Confidence:** High for these acyclic chains; low beyond them.

### FERRIUM-150: method-candidate width was the dominant tested hot spot

**Sources**

- [Experiment: method topology](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#method-candidate-width-and-call-count)

**Observed behavior**

With 1,000 calls, widening same-named trait candidates from 10 to 1,000 raised
old-solver wall time from 96.85 to 2,185.89 ms. With 1,000 candidates,
increasing calls from 1 to 1,000 raised it from 137.16 to 2,195.45 ms.

**Implication**

Reports need candidate width, applicable-candidate count, method name, call
count, receiver type, and import scope. Source size alone hides the topology.

**Confidence:** High for the generated same-name fixture.

### FERRIUM-151: standalone query time under-reported method search work

**Sources**

- [`evaluate_obligation` query](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L2546-L2552)
- [Experiment: self-profile attribution](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#self-profile-attribution)

**Observed behavior**

The old-solver candidate profile recorded 1,003,006 obligation accesses but
only 11.36 ms of `evaluate_obligation` self time; `typeck_root` self time was
3,526.83 ms. The global-next profile exposed 999,000 proof-tree query accesses
with 1.72 ms self time while `typeck_root` measured 4,541.28 ms.

**Implication**

One solver query event is not complete solver cost. Method probing,
candidate assembly, normalization, fulfillment, cache access, and integration
with inference can remain inside the owner event.

**Confidence:** High for the profiled event set.

### FERRIUM-152: global-next performance was pattern-dependent

**Sources**

- [Experiment: solver comparison](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#solver-comparison)

**Observed behavior**

Old and global-next wall medians differed by less than 13% on the seven
ordinary primary controls. At 1,000 candidates and calls, global-next measured
2,744.73 ms versus 2,185.89 ms old in the scaling series.

**Implication**

FERRIUM must publish per-pattern comparisons and compiler revisions, not a
universal old-versus-new ranking.

**Confidence:** High for these fixtures and this nightly.

### FERRIUM-153: unchanged and body-local edits reused owner results

**Sources**

- [Experiment: incremental edit matrix](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#incremental-edit-matrix)

**Observed behavior**

Untouched and identical rewrites produced no `typeck_root` execution under
either solver. A one-caller edit and an impl-method-body edit each produced one
miss. A shared bound change produced 1,001 misses.

**Implication**

Trait use does not erase body-result reuse. Reports must distinguish caller
body, impl body, impl header, trait predicate, and shared-bound edits.

**Confidence:** High.

### FERRIUM-154: impl-set edits crossed different incremental boundaries

**Sources**

- [old/new obligation dispatch](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs#L85-L117)
- [Experiment: incremental edit matrix](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#incremental-edit-matrix)

**Observed behavior**

Adding or removing an unrelated `Marker for Other` implementation produced
one old-solver obligation-query miss and zero caller `typeck_root` misses. The
global next solver produced 1,000 caller misses for the same 1,000 repeated
`Leaf: Marker` goals.

**Implication**

Query placement affects edit reuse independently of clean-build speed. This
fixture is a candidate regression and architecture probe, not proof that the
global next solver always invalidates more broadly.

**Confidence:** High for the controlled edit; medium for causal
generalization.

### FERRIUM-155: new-solver reuse is less visible in ordinary query summaries

**Sources**

- [new-solver search graph](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_next_trait_solver/src/solve/search_graph.rs#L26-L48)
- [canonical global cache](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_type_ir/src/search_graph/global_cache.rs#L29-L45)
- [new-solver fulfillment fast paths](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_trait_selection/src/solve/fulfill.rs#L155-L181)

**Observed behavior**

The next solver uses a canonical in-process global cache, provisional
cycle-aware reuse, stalled-goal state, and registration fast paths. Its
ordinary goals did not appear through the old `evaluate_obligation` query.

**Implication**

Diagnostics need solver-aware goal, candidate, cache, cycle, and invalidation
statistics. Comparing event names directly would misrepresent reuse.

**Confidence:** High for the pinned architecture.

### FERRIUM-156: trait failures remained correctness and diagnostic controls

**Sources**

- [Experiment: expected failures](perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md#expected-failures)

**Observed behavior**

Unsatisfied bound, ambiguous method, and recursive overflow fixtures retained
exit status 1 and complete stderr under both solvers. Median failure time
ranged from 62.46 to 66.49 ms.

**Implication**

Ambiguity, no-solution, normalization, and overflow results cannot be treated
as performance fast paths or repaired by changing semantics silently.

**Confidence:** High.

## Recommendations

### Adopt now

1. Add solver-mode, canonical-goal, candidate-width, call-count, structural
   depth, query-visibility, and impl-set-edit vocabulary to the measurement
   contract.
2. Preserve stable complete time, no-analysis, time passes, solver-aware
   profiles, incremental edits, and failures as separate evidence.
3. Retain repeated-versus-unique goal, candidate-width, call-count,
   supertrait-depth, projection-depth, and impl-set fixtures.
4. Record method import scope, receiver type, implemented candidate, and
   applicability rather than infer cost from trait count alone.
5. Treat solver-mode comparisons as fixture- and revision-specific.

Owners: FERRIUM Compiler Performance Engineer, Rust Safety Steward, and
Validation Checker.

Validation: repeated distributions, exact generators, solver configuration,
query events, cache hits and misses, incremental edits, failure stderr,
Linux rustc-perf follow-up, and cross-platform reproduction.

### Prototype behind a compatibility boundary

Prototype a read-only report that combines:

- type-check owner and source span;
- effective old, coherence-only, or global-next solver mode;
- goal kind, canonical identity, repetition, and parameter environment;
- trait and impl count plus relevant candidate width;
- method name, receiver type, call count, and applicable candidates;
- supertrait, projection, normalization, and recursive depth;
- solver-aware cache, cycle, stalled-goal, query, and `typeck_root` evidence;
- body, impl-body, impl-header, impl-set, and shared-bound edit classes.

The adapter must remain optional and versioned. It must not reproduce trait
selection, prescribe API rewrites, or equate missing query events with missing
solver work.

Owners: FERRIUM with rustc remaining authoritative.

Validation: synthetic controls, held-out trait-heavy crates, attribution
accuracy, solver-version compatibility, false-positive review, privacy review,
and removable adapters.

### Reject or defer

- automatic trait-bound simplification;
- automatic trait splitting, merging, or method renaming;
- associated-type or generic API rewrites;
- changing imports to manipulate method lookup;
- enabling unstable solver modes in product builds;
- custom trait solvers or persistent compiler daemons;
- direct rustc-internal product dependencies;
- universal old-versus-new performance claims;
- upstream issue or PR creation without explicit approval.

These changes can alter coherence, method resolution, ambiguity, diagnostics,
public APIs, inference, code generation, semver behavior, and downstream
invalidation.

## Candidate contribution paths

No upstream activity was created. If the user later approves outreach,
maintainer guidance should precede any branch, issue, comment, or pull request.

Candidate paths:

1. repeated versus unique concrete marker and projection goals;
2. orthogonal same-name method-candidate width and call-count sweeps;
3. supertrait and associated-type projection depth sweeps;
4. old/coherence-only/global-next benchmark configuration pairs;
5. localized caller, impl-body, impl-set, and shared-bound incremental edits;
6. solver-aware candidate, cache, and invalidation diagnostics.

Each path needs current Linux rustc-perf reproduction, benchmark runtime and
noise review, comparison with existing solver fixtures, diagnostic stability,
and project-specific maintainer approval.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: no trait, impl, projection, import, or API rewrite is described as semantics-neutral; compiler acceptance is not a correctness proof. |
| Compiler Performance Engineer | Accepted: stable totals, boundaries, solver modes, query visibility, topology sweeps, incrementality, failures, variance, and observer effects remain separate. |
| Interop Boundary Auditor | Accepted: source, typeck, solver, coherence, incremental, MIR/borrow, toolchain, and upstream boundaries remain explicit. |
| AI Assurance Skeptic | Accepted: the candidate fixture does not become a universal complexity law or solver ranking; negative and broader-invalidation results remain visible. |
| Ecosystem Strategist | Accepted: rustc and rustc-perf remain authoritative; FERRIUM supplies orthogonal evidence and defers outreach. |
| Rust Maintainer | Accepted: ordinary Cargo and Rust APIs remain unchanged; fixtures and diagnostics target explanation rather than compiler rituals. |
| Native Platform Adopter | Accepted: Windows, NTFS, warm-cache scope, unstable diagnostics, solver configuration, and missing Linux/macOS evidence are explicit. |
| Scope Keeper | Accepted: Q13 covers trait topology and solver reuse; borrow checking, MIR, general query invalidation, and source refactoring remain later questions. |
| Validation Checker | Accepted: exact generators, toolchains, 15- and 30-sample distributions, five-run profiles, scaling sweeps, incremental edits, failures, source revisions, and limitations are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

## Decision

PERF-Q13 is complete.

FERRIUM should model trait solving through solver mode, canonical goal reuse,
candidate width, call count, structural depth, query visibility, and edit
dependencies. The next question is PERF-Q14: determine when borrow checking
materially dominates builds and which loan, region, move, drop, or closure
topologies create the work.
