# Performance Program Role Checkpoint

Date: 2026-08-08
Scope: PERF-Q01 through PERF-Q20, the compiler query-plan model, and the
labeled Rust Build Forest
Status: Research checkpoint; implementation gate remains closed
Decision: continue the remaining performance research; preserve the read-only
compiler query plan and labeled Build Forest as the leading bounded product
direction; do not begin a cache, compiler fork, artifact restoration service,
automatic validation reducer, or RDR implementation.

## Executive conclusion

The `.roles` review finds that FERRIUM's Rust performance research is coherent,
evidence-led, and increasingly differentiated.

The program began with a broad question about Rust's competitive gaps. Twenty
performance questions now establish a more precise opportunity:

- not a replacement language, compiler, package manager, or linker;
- not a generic timing dashboard;
- not a shared writable target directory or proprietary compiler cache;
- not an AI claim that compilation or one test proves correctness;
- a read-only explanation and control plane that joins Cargo work, rustc
  queries, incremental reuse, artifacts, linking, validation, concurrent
  sessions, and historical build roots.

The strongest user-facing abstraction is the **compiler query plan**. It shows
planned versus observed work, dependencies, cacheability, invalidation,
expected and actual cost, serial and parallel regions, downstream cutoff,
linking, validation, and uncertainty.

The strongest architecture is the **labeled Rust Build Forest**. It adds
immutable roots, mutable human labels, lineage, provenance, session ownership,
artifact and evidence references, storage policy, and visualization above
Cargo and rustc while preserving compiler-private incremental generations as
atomic opaque units.

Every role supports continuing the research. No role approves implementation
yet. The recurring objections are:

1. current evidence is still weighted toward synthetic controls, one Windows
   machine, and pinned nightly diagnostics;
2. user-value and maintainer-effort reduction have not been tested on a held-
   out workflow;
3. remote trust, artifact transport, restoration, signing, revocation, and
   compatibility remain intentionally gated by PERF-Q30;
4. ABI, native-link, dynamic-link, target, and cross-platform cases remain
   incomplete;
5. the 36-question program and forest architecture must converge on one small
   removable prototype rather than expanding into a platform prematurely.

The correct status at roughly two-thirds of the current performance sequence
is therefore:

```text
research model: accepted
continued measurement: accepted
read-only prototype direction: conditionally accepted
implementation gate: closed
cache or compiler intervention: rejected for now
```

## Evidence reviewed

### Program doctrine and roles

- [FERRIUM engineering principles](../governance/ENGINEERING_PRINCIPLES.md)
- [FERRIUM role index](../../.roles/ROLE.md)
- all five parliament roles;
- both stakeholder roles;
- both editorial roles.

### Completed performance decisions

- PERF-Q01 through PERF-Q07:
  [first-seven checkpoint](2026-08-08-first-seven-performance-questions.md)
- PERF-Q08: [rustc startup and metadata](2026-08-08-rustc-startup-metadata.md)
- PERF-Q09: [parsing and tokenization](2026-08-08-parsing-tokenization.md)
- PERF-Q10:
  [declarative macro expansion](2026-08-08-declarative-macro-expansion.md)
- PERF-Q11:
  [name resolution and HIR lowering](2026-08-08-name-resolution-hir-lowering.md)
- PERF-Q12:
  [type inference and type checking](2026-08-08-type-inference-checking.md)
- PERF-Q13:
  [trait-solving cost and reuse](2026-08-08-trait-solving-cost-reuse.md)
- PERF-Q14:
  [borrow-checking cost and incrementality](2026-08-08-borrow-checking-cost-incrementality.md)
- PERF-Q15:
  [MIR construction and optimization](2026-08-08-mir-construction-optimization.md)
- PERF-Q16:
  [frontend parallelism](2026-08-08-frontend-parallelism.md)
- PERF-Q17:
  [query dependency precision](2026-08-08-query-dependency-precision.md)
- PERF-Q18:
  [incremental cache overhead](2026-08-08-incremental-cache-overhead.md)
- PERF-Q19:
  [early-phase incrementality](2026-08-08-early-phase-incrementality.md)
- PERF-Q20:
  [Relink-Don't-Rebuild](2026-08-08-relink-dont-rebuild.md)
- [Rust Build Forest opportunity](2026-08-08-rust-build-forest-opportunity.md)

## Cumulative learning model

### 1. Latency is a layered system, not one compiler number

PERF-Q01 through PERF-Q08 separate end-to-end wall time, Cargo planning,
queueing, locks, process startup, metadata demand, frontend work, backend work,
linking, and validation. Instrumentation itself can change the measured
workload.

**Role implication:** performance claims must name the consumer workflow and
evidence layer. No role accepts a single aggregate compile-time number.

### 2. Reuse is governed by several identities

Cargo graph units, artifact namespaces, symbol metadata, freshness
fingerprints, rustc incremental generations, cross-crate interfaces, and final
link inputs are related but different. Lower unit count, matching package
names, a cache hit, equal runtime output, or equal metadata bytes do not prove
safe reuse.

**Role implication:** every cache or cutoff decision is a correctness decision.
Identity, provenance, negative cases, recovery, and incompatibility must remain
visible.

### 3. Developer responsiveness and total machine work can conflict

Shared target directories can coalesce identical work but block incompatible
commands. Isolated targets restore concurrency by duplicating CPU, memory,
storage, and compiler work. Parallel rustc jobs can help large crates while
multiple editor, agent, Cargo, and validation sessions still oversubscribe the
machine.

**Role implication:** FERRIUM needs a session and resource plan, not a universal
`jobs` or target-directory recommendation.

### 4. Compiler bottlenecks are topology-specific

Parsing cost depends on source shape, macro expansion on matcher and output
shape, resolution on namespace propagation, type checking on owner and
constraint topology, trait solving on goal and candidate topology, borrow
checking on MIR and loan topology, and optimization on MIR and codegen shape.

**Role implication:** there is no defensible "Rust is slow because X" product
story. The tool must explain the measured graph and preserve unknown causes.

### 5. Incrementality spends work to prove that work can be skipped

Rustc's red-green algorithm, stable hashing, graph loading, query-result
deserialization, persistence, and work-product management have measurable
cost. Provider execution can still produce an equal result and contain later
invalidation. Small reusable regions may cost more to prove and restore than
to recompute.

**Role implication:** cache-hit rates and query-miss counts are insufficient.
The plan must show proof cost, reconstruction cost, avoided work, downstream
containment, and storage.

### 6. Early frontend persistence requires decomposition, not one cache

Parsing, expansion, resolution, AST indexing, and HIR lowering have different
identities and correctness frontiers. HIR-owner reconstruction and structural
parsing are credible fixture candidates. General persistent expansion,
resolution, AST serialization, or a syntax daemon are not justified.

**Role implication:** contribute minimized upstream fixtures before proposing a
compiler architecture.

### 7. Relink-Don't-Rebuild needs semantic cutoff and stable retained identities

Checksum freshness eliminates identical rewrites but not real content edits.
Private and public non-inline body edits are leading candidates for retaining
downstream compilation while still rebuilding upstream code and relinking.
Inline, generic, const, macro, layout, ABI, definition, and symbol identities
remain correctness-sensitive.

**Role implication:** FERRIUM may explain expected and observed cutoff. It must
not force reuse or define a competing interface hash.

### 8. The visual opportunity is a query plan over a build forest

The query plan explains one command or related session set. The Build Forest
retains named immutable roots, lineage, provenance, evidence, and policies
across time. The forest references complete compiler cache generations and
artifacts but does not interpret or splice rustc-private files.

**Role implication:** this is the defensible product wedge only if it remains
read-only, removable, provenance-aware, and useful from stable evidence before
optional nightly detail is added.

## Role findings

### FERRIUM-248: Rust Safety Steward conditionally accepts the model, not artifact reuse

**Sources:** FP-04, FP-09, PERF-Q05, PERF-Q17 through PERF-Q20, and the Rust
Safety Steward role.

**Review:** The program consistently distinguishes compiler acceptance from
behavioral proof, preserves macro, build-script, layout, ABI, definition,
symbol, diagnostic, and validation boundaries, and rejects forced reuse.

**Unresolved objection:** Any cache restore, RDR cutoff, validation reduction,
or direct compiler-state manipulation requires explicit soundness and
correctness evidence beyond the current read-only model.

**Disposition:** Continue research. Block state-changing reuse and validation
interventions.

### FERRIUM-249: Compiler Performance Engineer accepts the decomposition but requires broader proof

**Sources:** FP-02, FP-08, PERF-Q01 through PERF-Q20, and the Compiler
Performance Engineer role.

**Review:** The program separates workflows, cache states, observer effects,
compiler phases, resource contention, proof cost, and linking. Negative and
counterintuitive results changed recommendations rather than being hidden.

**Unresolved objection:** Many compiler-detail findings use synthetic fixtures,
one Windows host, and exploratory repetition counts. Upstream optimization
claims still require rustc-perf-compatible cases and representative
cross-platform evidence.

**Disposition:** Continue measurement and fixture work. Do not promise
performance gains for a product yet.

### FERRIUM-250: Interop Boundary Auditor preserves ABI and native-link work as an explicit gap

**Sources:** FP-04, FP-07, PERF-Q20, planned PERF-Q28 and PERF-Q29, and the
Interop Boundary Auditor role.

**Review:** The current research does not conceal ABI, layout, native object,
panic, allocation, dynamic-linking, or target-specific behavior behind a
generic cache claim.

**Unresolved objection:** Mixed Rust/C/C++, cdylib, staticlib, build-script
native outputs, linker arguments, and platform ABI cases have not yet been
measured by the current performance sequence.

**Disposition:** No objection to the read-only plan. No approval for artifact
reuse or link recommendations across language boundaries.

### FERRIUM-251: AI Assurance Skeptic accepts evidence separation and demands calibrated uncertainty

**Sources:** FP-01, FP-10, FP-11, all experiment limitations, and the AI
Assurance Skeptic role.

**Review:** Commands, revisions, failed pilots, unsupported surfaces,
reproducibility controls, and unknown causes remain visible. The query plan
distinguishes observed work from counterfactual or theoretical savings.

**Unresolved objection:** A future planner or forecast could turn incomplete
Cargo and compiler evidence into confident causal text. Predictions need
held-out accuracy, uncertainty, source attribution, and human approval.

**Disposition:** Continue. Require observed, inferred, predicted, and unknown
states in every user-facing explanation.

### FERRIUM-252: Ecosystem Strategist finds a contribution-first wedge

**Sources:** FP-05, FP-06, the official upstream work reviewed by each question,
and the Ecosystem Strategist role.

**Review:** The program repeatedly chose Cargo, rustc, rustc-perf,
rust-analyzer, Clippy, existing linkers, and upstream project goals over
replacement tooling. Active Rust efforts already cover RDR, cross-command
incremental bases, cache layout, fine-grained locking, frontend parallelism,
and faster linking.

**Unresolved objection:** FERRIUM becomes duplicative if it implements those
mechanisms. Its differentiated value must remain joined explanation, history,
lineage, policy, and evidence across tools.

**Disposition:** Accept the read-only query-plan and forest wedge. Reject a
parallel compiler, package manager, cache protocol, or linker.

### FERRIUM-253: Rust Maintainer conditionally accepts the ordinary-workflow boundary

**Sources:** FP-03, FP-05, FP-07, the Rust Maintainer role, and every question's
non-goals.

**Review:** Recommendations preserve ordinary Cargo commands, avoid source
rewrites, expose failures, and prefer removable adapters. The proposed plan can
explain why work occurred without requiring maintainers to learn rustc
internals.

**Unresolved objection:** The accumulated vocabulary and 36-question research
program can itself become an opaque system. A first prototype must answer one
maintainer question with a small surface and actionable output.

**Disposition:** Continue. Require a usability test that demonstrates reduced
investigation effort.

### FERRIUM-254: Native Platform Adopter does not yet approve operational adoption

**Sources:** FP-02, FP-07 through FP-10, PERF-Q05 through PERF-Q07, the Build
Forest decision, and the Native Platform Adopter role.

**Review:** The program recognizes Windows, CI, filesystem, antivirus, storage,
lock, isolation, cleanup, rollback, and concurrent-session concerns.

**Unresolved objection:** Linux and macOS behavior, network filesystems,
enterprise endpoint controls, large-workspace storage growth, support burden,
retention, disaster recovery, and removal have not been validated for a forest
implementation.

**Disposition:** Accept research. Block production or remote deployment.

### FERRIUM-255: Scope Keeper requires convergence before implementation

**Sources:** FP-12, the 36-question registry, the Build Forest architecture,
and the Scope Keeper role.

**Review:** Each completed question is bounded and records non-goals. The
program has resisted attractive compiler, scheduler, cache, and source-rewrite
implementations.

**Unresolved objection:** The query plan, forest, cache diagnosis, validation
planning, concurrency control, RDR, and upstream fixtures could expand into an
unbounded platform.

**Disposition:** Continue the planned research, then choose one prototype.
Freeze all other capabilities as non-goals for that prototype.

### FERRIUM-256: Validation Checker keeps the implementation gate closed

**Sources:** the measurement contract, prototype gate, experiment corpus, and
the Validation Checker role.

**Review:** The program has reproducible commands, synthetic and public
fixtures, environment identity, negative cases, failures, limitations, and
role-specific decisions.

**Unresolved objection:** No held-out evaluation yet demonstrates that one
read-only explanation improves a maintainer decision. Cross-platform and
operational cases remain incomplete, and the exact first prototype acceptance
threshold has not been frozen.

**Disposition:** Research is valid. Prototype authorization is withheld.

## Prototype-gate status

| Required gate item | Current status | Role judgment |
|---|---|---|
| One research question, named consumer, representative fixtures | Partial | Many questions and consumers exist; the first prototype question is not selected |
| Current tools and upstream initiatives | Strong | Completed repeatedly through Q20 |
| Baselines, environment, variance, and limitations | Strong for research | Needs held-out prototype baseline and broader platforms |
| Compatibility, safety, and correctness boundaries | Strong | Remote, ABI, and retained-artifact cases remain gated |
| Success measures, negative cases, stop conditions, and non-goals | Partial | Research questions have them; first prototype thresholds are not frozen |
| Adoption, removal, rollback, and maintenance implications | Partial | Architectural principles exist; no implemented workflow has been evaluated |
| Ferris evidence versus human decisions | Strong | Must be preserved in UI and forecast behavior |
| Completed `.roles` review with objections visible | Complete | This checkpoint records the objections |

The gate is closed because the partial items are product-selection questions,
not because the completed research is invalid.

## Converged direction

The roles jointly support one candidate for later selection:

> A removable, read-only compiler query-plan view backed first by stable Cargo
> evidence, with optional versioned nightly diagnostics, that can attach one
> observed command or session set to an immutable local Build Forest root.

The smallest credible first use case is:

```text
"Why did this edit rebuild these units, what was reused, what waited, what
remains unknown, and which validation or link work actually ran?"
```

That use case does not require:

- storing or restoring compiler-private incremental files;
- remote artifact transport;
- changing Cargo scheduling;
- rewriting manifests, features, profiles, targets, or source;
- reducing tests automatically;
- implementing RDR;
- predicting unobserved work before the observed explanation is validated.

## Required evidence before implementation

1. Complete enough of PERF-Q21 through PERF-Q30 to close cross-command,
   codegen, linking, and provenance blind spots.
2. Select one maintainer workflow and freeze its input, output, success,
   uncertainty, and stop contracts.
3. Evaluate the explanation on held-out edits in at least three public
   repositories, including one macro/build-script-heavy and one link-heavy
   case.
4. Demonstrate that a maintainer can reach the correct diagnosis faster than
   with raw Cargo output alone.
5. Repeat the selected workflow on Windows and at least one Unix platform.
6. Preserve failure, unknown-cause, unsupported-toolchain, and concurrent-
   session controls.
7. Keep remote transport, restoration, signing, and shared-cache behavior
   closed until PERF-Q30.

## Decision

Continue the question process.

The roles do not request a change in research direction. They request
convergence and stronger adoption evidence before implementation. The query
plan and labeled Build Forest remain strategic architecture, not permission to
build a platform.
