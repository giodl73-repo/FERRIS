# Name Resolution and HIR Lowering

Date: 2026-08-08
Question: PERF-Q11
Status: Complete
Decision: add namespace topology, resolution subphase, HIR owner, stable-hash,
and edit-frontier evidence to FERRIUM; prioritize read-only diagnostics and
orthogonal fixtures over source rewriting, resolver caches, or compiler forks.

## Executive conclusion

Rust name resolution is not one path lookup phase, and HIR lowering is not one
crate-wide conversion.

The current frontend combines:

1. reduced module-graph construction interleaved with expansion;
2. batched fixed-point import resolution;
3. crate-wide effective visibility computation;
4. a full late-resolution AST walk for expressions, types, patterns,
   lifetimes, labels, and locals;
5. AST indexing;
6. per-definition HIR lowering;
7. stable owner hashing that protects later incremental queries.

The controlled evidence found a namespace topology capable of dominating the
frontend. A 28.6 KB crate that propagated 1,000 public names through 100 glob
re-export layers took 3,055 ms to emit stable metadata. Ordinary fixtures
between 209 and 499 KB took 210–529 ms.

The glob fixture did not parse slowly. Its root-parse median was 59.57 ms,
similar to the other fixtures. Diagnostic time passes attributed a 1,665 ms
`resolve_crate` median to roughly 344 ms of import finalization and 1,131 ms
of effective visibility computation. Late expression/type resolution took
less than 1 ms.

The scaling matrix showed that propagated binding count and dependency depth
both matter. With 1,000 base bindings, increasing the chain from 10 to 100
layers raised stable metadata time from 143.71 to 3,435.13 ms. Ten layers
times 1,000 bindings and 100 layers times 100 bindings both propagated 10,000
bindings, but measured 143.71 and 215.49 ms.

Late resolution had a different shape. Ten thousand qualified item paths
produced a 21.48 ms late-resolution median, compared with 7–10 ms for large
flat-item, body-binding, import, and module controls. Cost followed resolved
path and scope topology rather than source bytes alone.

HIR lowering was owner-sensitive. A single function containing 60,018 HIR
records had only four `lower_to_hir` misses and 9.35 ms of lowering self time.
Ten thousand flat structs had 20,008 HIR records, 20,003 lowering misses, and
32.47 ms of lowering self time. Local-node volume and owner-query volume are
separate dimensions.

Once rustc ran, incremental compilation did not skip the early frontend.
`resolver_for_lowering_raw`, `index_ast`, and `lower_to_hir` are
`eval_always`. Fresh, untouched, rewritten, body, import, visibility, module,
and macro scenarios all reran roughly 5.9–6.1 ms of resolution and reported
7,017 `lower_to_hir` misses.

Incrementality still protected later work. `hir_owner` is a feedable,
stable-hashed boundary. Aggregate `hir_owner` profile time fell from 26.60 ms
with a fresh directory to 1.23 ms with untouched source even though lowering
reran. The compiler reconstructs and compares owner results, then preserves
downstream reuse when hashes remain equal.

Eight frontend jobs did not accelerate the flat-item, body, named-import, or
glob controls. Current import resolution already uses parallel work inside a
fixed-point batch, but dependent batches, effective visibility computation,
and late crate walking remain constraining shapes. The 2026 project goal still
lists parallel name resolution and macro expansion as longer-term work.

FERRIUM should prototype a read-only namespace and HIR topology report that
joins import form, re-export depth, propagated names, visibility fanout,
resolved path shape, AST/HIR records, owner count, resolution subphases, and
edit class. Credible upstream paths are a dedicated rustc-perf glob/visibility
fixture and structured fixed-point or visibility diagnostics.

Automatic glob replacement, module movement, visibility changes, import
rewrites, persistent resolver caches, and parallel resolver changes remain
deferred. Those interventions can change ambiguity, privacy, macro behavior,
diagnostics, public API, and compiler correctness.

No upstream activity was created.

## Decision supported

This research determines:

- which namespace and owner shapes create disproportionate frontend work;
- which work belongs to imports, visibility, late resolution, or HIR lowering;
- what incremental compilation reruns versus what stable owner hashes protect;
- whether frontend jobs accelerate representative resolution and lowering
  controls;
- which external diagnostics and upstream fixtures are defensible.

It does not authorize import rewrites, visibility changes, module
restructuring, compiler changes, resolver caches, HIR persistence, parallel
resolution, or upstream filing.

## Evidence reviewed

### Local evidence

- [Parsing and tokenization](2026-08-08-parsing-tokenization.md)
- [Declarative macro expansion](2026-08-08-declarative-macro-expansion.md)
- [Experiment](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Compiler sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [`configure_and_expand`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs#L128-L164)
- [`Resolver::resolve_crate`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/lib.rs#L2054-L2071)
- [late-resolution model](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/late.rs#L1-L7)
- [late-resolution crate walks](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/late.rs#L5697-L5711)
- [import fixed-point algorithm](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/imports.rs#L764-L810)
- [import finalization](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/imports.rs#L923-L949)
- [parallel import-resolution lock note](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/build_reduced_graph.rs#L120-L127)
- [resolution and HIR query definitions](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L182-L247)
- [`lower_to_hir`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_ast_lowering/src/lib.rs#L654-L704)
- [HIR owner hashing](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/hir/mod.rs#L173-L199)
- [feedable HIR providers](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/hir/mod.rs#L452-L459)
- [rustc-dev-guide name resolution](https://github.com/rust-lang/rustc-dev-guide/blob/0e48eac6e3fb4b92ad46495325d6237a7b3ed989/src/name-resolution.md)
- [rustc-dev-guide HIR](https://github.com/rust-lang/rustc-dev-guide/blob/0e48eac6e3fb4b92ad46495325d6237a7b3ed989/src/hir.md)

### Performance direction

- [2026 parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)
- [parallel compiler tracking issue](https://github.com/rust-lang/rust/issues/113349)
- [parallel frontend test-suite issue](https://github.com/rust-lang/rust/issues/118698)
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)
- [`unused-warnings` fixture](https://github.com/rust-lang/rustc-perf/tree/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/unused-warnings)

## Frontend model

```text
parsed crate and expansion fragments
  -> build reduced module graph
  -> plant imports and macro scopes
  -> run batched import fixed-point work as names become available
  -> complete macro expansion
  -> finalize imports and ambiguities
  -> compute crate-wide effective visibilities and re-export effects
  -> walk the fully expanded AST for late names, paths, lifetimes, labels,
     patterns, and local bindings
  -> package crate-wide resolver outputs
  -> index AST definitions
  -> lower each local definition through lower_to_hir
  -> feed stable-hashed hir_owner and hir_attr_map results
  -> collect module and crate HIR item sets
  -> continue type checking, borrow checking, lints, metadata, and codegen
```

Early resolution and macro expansion are coupled. HIR owner granularity begins
after the crate-wide resolver has produced the data lowering consumes.

## Findings

### FERRIUM-127: resolution combines early fixed-point work and a late
crate walk

**Sources**

- [`configure_and_expand`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs#L128-L164)
- [`Resolver::resolve_crate`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/lib.rs#L2054-L2071)
- [late-resolution crate walks](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/late.rs#L5697-L5711)

**Observed behavior**

Macro expansion constructs the reduced graph and import work incrementally.
After expansion, `resolve_crate` finalizes imports, computes effective
visibilities, finalizes macro resolutions, and walks the complete AST for late
names.

**Implication**

One aggregate “resolution” number hides algorithms with different inputs,
complexity, parallelism, and safe interventions. FERRIUM must preserve early
graph, import, visibility, late-path, and lowering boundaries.

**Confidence:** High.

### FERRIUM-128: namespace topology can dominate much larger source

**Sources**

- [Experiment: stable complete compilation](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#stable-complete-compilation)
- [Experiment: root and no-analysis boundaries](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#root-and-no-analysis-boundaries)

**Observed behavior**

The 28,591-byte glob chain measured 3,055.14 ms stable. The 499,197-byte
qualified-path fixture measured 529.31 ms, and the 208–307 KB item, body,
import, and module controls measured 210–261 ms.

Root parsing was similar across fixtures. The glob fixture diverged after
parsing and before semantic analysis.

**Implication**

Source bytes, line count, and item count are weak resolution estimates.
Import kind, re-export topology, propagated bindings, and visibility fanout
belong in the cost model.

**Confidence:** High for the synthetic Windows fixture.

### FERRIUM-129: effective visibility dominated the deep re-export chain

**Sources**

- [`Resolver::resolve_crate`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/lib.rs#L2054-L2071)
- [Experiment: resolution subphases](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#resolution-subphases)
- [Experiment: self-profile](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#self-profile-and-hir-topology)

**Observed behavior**

The 100-layer glob fixture had a 1,665.31 ms diagnostic `resolve_crate`
median: 343.71 ms in import finalization, 1,130.96 ms in effective visibility,
and 0.74 ms in late resolution.

Five-run self-profile medians independently attributed 324.03 ms to import
finalization and 1,194.94 ms to effective visibility.

**Implication**

This fixture is primarily a public-namespace and visibility propagation
problem, not a local path-resolution or HIR-lowering problem. A report that
only counts imports or paths would misdiagnose it.

**Confidence:** High for attribution; diagnostic timings are
observer-affected.

### FERRIUM-130: propagated binding count and dependency depth both matter

**Sources**

- [import fixed-point algorithm](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/imports.rs#L764-L810)
- [Experiment: glob propagation scaling](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#glob-propagation-scaling)

**Observed behavior**

At 1,000 base bindings, stable time rose from 143.71 ms at 10 layers to
3,435.13 ms at 100 layers. At 100 layers, increasing base bindings from 100
to 1,000 raised stable time from 215.49 to 3,435.13 ms.

Two fixtures that each propagated 10,000 bindings measured differently:
143.71 ms for 10 layers times 1,000 bindings and 215.49 ms for 100 layers
times 100 bindings.

**Implication**

FERRIUM should report both propagated-binding work and import dependency
depth. The measured upper range was superlinear, but this experiment does not
claim a universal asymptotic bound.

**Confidence:** High for the measured matrix.

### FERRIUM-131: late resolution follows path and scope shape

**Sources**

- [late-resolution model](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/late.rs#L1-L7)
- [late-resolution visitor](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/late.rs#L868-L874)
- [Experiment: resolution subphases](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#resolution-subphases)

**Observed behavior**

Ten thousand constants containing 20,000 qualified paths measured 21.48 ms of
late resolution. Large flat items, local bindings, named imports, and modules
measured roughly 6–9 ms.

The body fixture placed 10,000 local bindings and paths in one function, while
the qualified fixture distributed paths across 10,000 item owners. Both path
count and scope/owner topology changed work.

**Implication**

Late-resolution diagnostics need path kinds, scope/rib depth, local binding
count, and owner distribution. “Names resolved” alone is not causal enough.

**Confidence:** High for the controlled fixtures.

### FERRIUM-132: HIR local-node volume and owner-query volume are different

**Sources**

- [`lower_to_hir`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_ast_lowering/src/lib.rs#L654-L704)
- [resolution and HIR query definitions](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L198-L247)
- [Experiment: self-profile and HIR topology](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#self-profile-and-hir-topology)

**Observed behavior**

The one-function body fixture had 60,018 HIR records, four
`lower_to_hir` misses, and 9.35 ms of lowering self time. The flat-item
fixture had 20,008 HIR records, 20,003 misses, and 32.47 ms of lowering self
time.

**Implication**

HIR diagnostics must record owner count, owner kind, local nodes per owner,
and lowering time. Total HIR nodes alone can rank fixtures incorrectly.

**Confidence:** High for the diagnostic profiles.

### FERRIUM-133: resolution and owner lowering rerun; stable owner results
protect later queries

**Sources**

- [resolution and HIR query definitions](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/queries.rs#L182-L247)
- [HIR owner hashing](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/hir/mod.rs#L173-L199)
- [feedable HIR providers](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/hir/mod.rs#L452-L459)
- [Experiment: incremental edit matrix](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#incremental-edit-matrix)

**Observed behavior**

`resolver_for_lowering_raw`, `index_ast`, and `lower_to_hir` are
`eval_always`. Every incremental scenario reran approximately 5.9–6.1 ms of
resolution and 13.2–16.1 ms of lowering, with 7,017 lowering misses.

`hir_owner` is feedable and stable-hashed. Its aggregate profile time fell
from 26.60 ms fresh to 1.23 ms untouched.

**Implication**

The correct model is reconstruction plus stable comparison, not persistent
AST/HIR reuse. Body-oriented downstream queries can remain green even though
the resolver and lowering frontend ran again.

**Confidence:** High.

### FERRIUM-134: edit classes diverged after a similar frontend boundary

**Sources**

- [Experiment: incremental edit matrix](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#incremental-edit-matrix)

**Observed behavior**

Fresh, untouched, and identical-rewrite wall medians were approximately
615–622 ms. Body and macro edits measured 669–680 ms. Import, module, and
visibility edits measured 704–733 ms.

Resolution and lowering medians remained approximately flat across all edit
classes. The wall differences therefore arose outside those aggregate
frontend regions or from work nested elsewhere in the pipeline.

**Implication**

FERRIUM should join edit class to owner hashes, query invalidation, metadata,
and downstream work before attributing an import or visibility edit's complete
cost to resolution. That broader attribution belongs with PERF-Q17 and
PERF-Q20.

**Confidence:** High for the fixture; causal downstream split remains
incomplete.

### FERRIUM-135: current frontend jobs did not accelerate the controls

**Sources**

- [parallel import work and fixed-point batches](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/imports.rs#L764-L810)
- [parallel import-resolution lock note](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/build_reduced_graph.rs#L120-L127)
- [2026 parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)
- [Experiment: frontend jobs](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#frontend-jobs)

**Observed behavior**

Eight jobs did not improve flat-item, body-binding, named-import, or glob
controls. Several glob/job diagnostic series were noisy and support no exact
slowdown claim.

The import algorithm can process entries in one batch in parallel, but
dependent batches repeat to a fixed point. Effective visibility and late crate
walking remained large serial-looking regions in the measured profiles.

**Implication**

More jobs are not an external remedy for expensive namespace topology.
Parallel resolution remains compiler-owned work requiring correctness,
determinism, diagnostics, and contention validation.

**Confidence:** High for “no observed speedup”; medium for generalization.

### FERRIUM-136: existing diagnostics expose phases but not namespace
causality

**Sources**

- [Experiment: diagnostics assessment](perf-q11-resolution-hir/results/EXP-01-namespace-topology-and-owner-invalidation.md#diagnostics-assessment)
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)
- [2026 parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)

**Observed behavior**

Time passes distinguish import finalization, effective visibility, late
resolution, and aggregate resolution. Self-profile and input stats add HIR
queries and record counts.

They do not identify fixed-point batches, per-import propagated bindings,
re-export depth, per-module visibility work, path/rib hot spots, or source
owners associated with lowering cost.

rustc-perf includes `unused-warnings`, many-declaration crates, macro cases,
and broad real programs. Its inventory does not identify a dedicated deep
glob re-export, effective-visibility, or import-only edit fixture.

**Implication**

FERRIUM's defensible contribution is a source-attributed namespace and owner
census plus orthogonal fixtures. Structured compiler diagnostics are a
possible upstream path after maintainer guidance.

**Confidence:** High for the reviewed inventory.

## Recommendations

### Adopt now

1. Add resolution and HIR vocabulary to the measurement contract:
   reduced graph, import fixed point, import dependency depth, propagated
   bindings, ambiguity, effective visibility, late path resolution, AST owner,
   HIR owner, local node, stable owner hash, and edit class.
2. Retain the glob depth/binding matrix as a synthetic regression fixture.
3. Require stable complete wall time before optimization claims.
4. Treat no-analysis, time passes, self-profile, input stats, and incremental
   query events as separate observer-affected evidence.
5. Preserve ambiguity, privacy, and unresolved-name failures.

Owners: FERRIUM Compiler Performance Engineer, Rust Safety Steward, and
Validation Checker.

Validation: stable distributions, exact fixture manifests, nightly diagnostic
revision, failure stderr, and cross-platform follow-up.

### Prototype behind a compatibility boundary

Prototype a read-only report that combines:

- module and import graph;
- named, glob, re-export, macro-generated, and prelude imports;
- import dependency depth and strongly connected components;
- estimated propagated binding work;
- effective visibility and public re-export fanout;
- qualified, unqualified, local, lifetime, label, and pattern paths;
- AST/HIR record counts;
- owner count and local-node distribution;
- resolution and lowering profile events;
- edit class and downstream invalidation evidence.

The prototype must consume ordinary source, Cargo metadata, stable compiler
output where possible, and optional nightly adapters. It must not rewrite
imports or claim compiler-equivalent resolution.

Owners: FERRIUM with rustc and Cargo remaining authoritative.

Validation: synthetic topology controls, held-out real crates, false-positive
review, stable/nightly agreement, privacy review, and removable adapters.

### Reject or defer

- automatic glob-to-named-import rewriting;
- import reordering as a performance change;
- visibility narrowing or widening;
- automatic module splitting or movement;
- checking in generated HIR or resolved source;
- persistent cross-session resolver or HIR caches;
- compiler daemons;
- parallel resolver implementation;
- replacing rustc's resolver or HIR;
- upstream issue or PR creation without explicit approval.

These can change ambiguity, privacy, macro hygiene, edition behavior, lints,
diagnostics, public API, incremental identity, and compiler correctness.

## Candidate contribution paths

No upstream activity was created. If the user later approves outreach,
maintainer guidance should precede any branch, issue, comment, or pull request.

Candidate paths:

1. an orthogonal rustc-perf fixture varying glob depth and propagated
   bindings;
2. an incremental import/body/visibility edit matrix;
3. machine-readable import fixed-point batch and propagated-binding
   statistics;
4. per-module effective visibility timing or counters;
5. source-attributed late-resolution and HIR-owner profile events;
6. parallel-frontend fixtures that separate independent imports from
   dependency chains.

Each path needs current Linux rustc-perf reproduction, benchmark cost review,
diagnostic stability discussion, and project-specific maintainer approval.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: import, visibility, module, and macro rewrites are not presented as semantics-preserving optimizations. |
| Compiler Performance Engineer | Accepted: stable totals, root/no-analysis boundaries, resolution subphases, HIR owners, scaling, incrementality, jobs, failures, variance, and observer effects remain separate. |
| Interop Boundary Auditor | Accepted: source, expansion, resolver, AST, HIR, query, metadata, filesystem, and compiler-version boundaries remain explicit. |
| AI Assurance Skeptic | Accepted: noisy tiny, time-pass, glob-job, and diagnostic series are not promoted into exact speedup or slowdown claims. |
| Ecosystem Strategist | Accepted: rustc, rustc-perf, the compiler project goal, and Cargo remain authoritative; FERRIUM supplies decomposition and fixtures. |
| Rust Maintainer | Accepted: ordinary name resolution and module APIs remain unchanged; diagnostics do not prescribe obscure rewrites. |
| Native Platform Adopter | Accepted: Windows, NTFS, warm-cache scope, nightly diagnostics, and missing Linux/macOS evidence are explicit. |
| Scope Keeper | Accepted: Q11 covers name resolution and HIR lowering; type inference, broad query invalidation, and crate modularization remain later questions. |
| Validation Checker | Accepted: exact generators, toolchains, source sizes, 15-, 20-, and 30-sample distributions, five-run profiles, expected failures, source revisions, and limitations are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

## Decision

PERF-Q11 is complete.

FERRIUM should preserve namespace cost as import dependency, propagated
binding, effective visibility, late path, HIR owner, and stable-hash topology
rather than one resolver time. The next question is PERF-Q12: determine which
type-inference and type-checking patterns create disproportionate work after
resolution and lowering.
