# OSPREY Program

Status: Architecture planning only
Code name: OSPREY
Expansion: Observe, Show, Predict, Resolve, Execute, Yield
Predecessor gate: complete the Crates Series before implementation

## Mission

OSPREY is FERRIUM's proposed Rust Query Forest and Build Intelligence program.
It joins Cargo, rustc, rust-analyzer, procedural macros, build scripts,
backends, linkers, validation, environment, and upstream evidence into one
reviewable edit-to-confidence model.

OSPREY is not authorized implementation. This plan organizes the architecture,
research dependencies, stage gates, and smallest credible future proof.

## Why a program is needed

The 36-question Rust performance sequence found mature components with
different scopes:

- Cargo plans packages, targets, profiles, features, and freshness.
- rustc evaluates crate-local semantic and code-generation queries.
- rust-analyzer maintains a separate incremental editor database.
- procedural macros and build scripts execute native code with external inputs.
- backends produce machine code under target and capability constraints.
- linkers assemble whole-program native outputs and debug packages.
- repositories define tests, release checks, policy, compliance, and rollout.
- operating environments impose storage, resource, security, and concurrency
  effects.

Each subsystem is rational within its own boundary. Their identities,
telemetry, lifecycle, and decisions do not form one operator-visible model.
OSPREY supplies that model without claiming ownership of the underlying tools.

## Program verbs

### Observe

Collect supported evidence without changing the repository:

- Cargo metadata, messages, timings, unit identities, freshness, and lock state;
- rustc phase, query, invalidation, incremental, mono-item, CGU, and emission
  summaries behind versioned adapters;
- rust-analyzer, editor, Cargo process, lock, cancellation, and target topology;
- macro, build-script, generated-output, native-input, and environment evidence;
- object, library, linker, debug-package, and final-output identity;
- validation commands, package scope, activity dimensions, gates, and outcomes;
- host, VM, filesystem, resource, security, indexing, and contention context;
  and
- provenance, revisions, toolchains, commands, failures, and limitations.

### Show

Render a detailed Query Forest:

- logical scope and containment;
- dependency, demand, invalidation, reuse, and ownership edges;
- planned versus observed work;
- fresh, rebuilt, reused, restored, skipped, failed, and unknown states;
- critical path, queueing, locks, concurrency, CPU, memory, storage, and I/O;
- compiler, artifact, link, validation, and capability boundaries;
- immutable roots, mutable labels, lineage, and evidence references; and
- selected-plan versus full-reference confidence.

### Predict

Forecast a proposed edit's:

- package and target cone;
- compiler and code-generation work;
- generic and downstream ownership;
- artifact invalidation and expected reuse;
- link and debug-package work;
- validation package and activity scope;
- environment sensitivity; and
- uncertainty and fallback.

Predictions remain distinct from observations and are scored against held-out
edits.

### Resolve

Produce a human-reviewable decision:

- explain an unexpected rebuild or wait;
- choose a diagnostic or benchmark;
- recommend a supported configuration comparison;
- select conservative validation;
- escalate an unknown input or capability;
- prepare an upstream contribution packet;
- defer an unsafe or uneconomic intervention; or
- identify the responsible upstream owner.

Resolve does not silently mutate source, manifests, workflows, hosts, or CI.

### Execute

Run only an explicitly approved and reversible plan:

- diagnostics and evidence collection;
- builds, checks, tests, benchmarks, profiles, and traces;
- disposable counterfactuals in isolated worktrees and target directories;
- approved validation plans with mandatory gates and fallback;
- approved configuration experiments;
- approved upstream packet preparation; and
- rollback or cleanup defined by the action contract.

Automatic repository rewriting, cache restoration, host tuning, validation
deletion, or external posting is outside the initial execution boundary.

### Yield

Produce durable outcomes:

- a FERRIS evidence packet;
- an observed Query Forest root;
- prediction-versus-observation results;
- a validation coverage ledger;
- an approved action and rollback record;
- an upstream contribution packet;
- a documented external or deferred disposition; and
- measured user and maintainer impact.

Yield also means yielding control to the maintainer when evidence is
insufficient.

## Architecture planes

The precise components, node types, edge types, maps, ledgers, records,
engines, views, and minimal first-proof subset are defined in the
[OSPREY Query Forest component model](../specs/OSPREY_QUERY_FOREST_COMPONENT_MODEL.md).

The Forest is the canonical evidence model and immutable-root history. It is
not one monolithic runtime component.

### 1. Scope plane

Defines the hierarchy:

```text
portfolio
  -> repository
    -> workspace
      -> package
        -> target
          -> crate invocation
            -> compiler query / phase
              -> mono item
                -> codegen unit
                  -> object / library
                    -> link plan / output
      -> validation plan / gate
```

Editor sessions, generated inputs, native dependencies, environments, and
upstream evidence attach through typed cross-scope edges rather than being
forced into the package tree.

### 2. Identity and lineage plane

Separates:

- source and revision identity;
- Cargo unit and artifact identity;
- compiler semantic and incremental identity;
- cross-crate interface and retained-definition compatibility;
- generic, backend, CGU, object, linker-state, and final-output identity;
- environment and execution identity;
- validation-plan and result identity;
- evidence-packet identity; and
- immutable roots, parent lineage, mutable labels, and retention.

No hash is treated as a universal compatibility key.

### 3. Causality plane

Represents:

- dependency and consumer demand;
- direct and transitive invalidation;
- freshness and reuse proof;
- hidden or undeclared inputs;
- critical-path and queue dependencies;
- lock and resource contention;
- fallback and unsupported states; and
- observed, inferred, predicted, and unknown explanations.

### 4. Cost plane

Records wall time, stable work metrics, CPU, memory, I/O, storage, transport,
artifact bytes, runtime, binary size, validation latency, and maintenance cost.
It distinguishes user latency from total machine work.

### 5. Capability plane

Preserves behavior that performance choices can alter:

- safety and semantic analysis;
- features, targets, profiles, lints, tests, doctests, and release checks;
- ABI, panic, unwind, allocator, native-library, and platform support;
- debug, symbol, crash, profiling, sanitizer, and coverage capabilities;
- runtime performance and final optimization;
- security, compliance, signing, deployment, and rollback; and
- repository-specific policy and operational gates.

### 6. Policy and action plane

Defines:

- advisory versus executable recommendations;
- approval authority;
- confidence and fallback;
- allowed commands and isolation;
- mutation, network, credential, and external-posting boundaries;
- rollback, cleanup, and retention;
- mandatory validation; and
- audit evidence.

### 7. Presentation plane

Provides:

- forest and critical-path views;
- before/after and predicted/observed comparisons;
- causal narratives grounded in source evidence;
- omitted scope and uncertainty;
- recommended next action;
- capability and validation consequences; and
- FERRIS packet export.

## Program sequence

### Phase 0: Crates Series

Complete ECOS-Q01 through ECOS-Q12 in the
[Crates Series research program](ECOSYSTEM_LIBRARY_RESEARCH_PROGRAM.md).

This precedes OSPREY implementation because Query Forest scope must cover more
than compilation. Foundational crates introduce:

- ecosystem capability ownership;
- feature and version fragmentation;
- runtime and type-system interchange;
- async-runtime assumptions;
- unsafe, native, build-script, and macro boundaries;
- platform, MSRV, `no_std`, WASM, and embedded claims;
- advisories, licensing, stewardship, and release identity; and
- complete dependency and maintenance closures.

**Gate:** all twelve ECOS questions are complete, reviewed by all nine FERRIUM
roles, and end with an ecosystem capability and dependency-governance model
that OSPREY can represent.

### Phase 1: Query Forest architecture

Produce specifications only:

- node, edge, scope, identity, state, and evidence vocabulary;
- Cargo/rustc/linker/validation/ecosystem adapter boundaries;
- immutable root and lineage model;
- prediction and confidence contract;
- action and approval contract;
- privacy, security, retention, and portability model; and
- FERRIS packet relationship.

**Gate:** architecture review finds no universal identity, hidden mutation, or
unowned lifecycle.

### Phase 2: Planning reference model

Use static examples and retained research evidence to produce:

- canonical forest examples;
- query and edge schemas;
- causal explanation examples;
- prediction and validation-plan examples;
- resolve/execute/yield state machines;
- compatibility and fallback matrices; and
- test and acceptance specifications.

No production package or repository integration is created.

**Gate:** examples cover Cargo, compiler, crate ecosystem, native, link,
validation, environment, and unknown-input cases.

### Phase 3: Held-out workflow design

Select one maintainer question:

> Why did this edit rebuild these units, what was reused, what waited, what
> remains unknown, and which validation and link work ran?

Freeze:

- three public repositories;
- Windows and Unix execution;
- held-out edits;
- raw-tool baseline;
- investigation-time measure;
- correctness and unknown controls;
- adoption, removal, and rollback;
- privacy and retention; and
- stop conditions.

**Gate:** all nine roles approve one bounded proof.

### Phase 4: Bounded prototype

Only a later approved pulse may create code. The first prototype remains:

- local and read-only;
- removable;
- backed first by stable Cargo evidence;
- optional and versioned for nightly compiler detail;
- unable to rewrite repositories or CI;
- unable to restore compiler-private state;
- unable to post externally;
- explicit about unknowns; and
- evaluated against the held-out workflow.

**Gate:** demonstrated maintainer benefit without hidden correctness,
operational, or maintenance cost.

### Phase 5: Controlled action

Resolve, Execute, and Yield capabilities advance independently. Each action
requires its own compatibility, approval, rollback, validation, and ownership
contract. Observation does not imply mutation authority.

## Required plans before code

The following documents or decisions must exist before implementation:

1. completed Crates Series and final ecosystem role review;
2. OSPREY Query Forest ontology and schema;
3. identity and lineage specification;
4. adapter and upstream ownership matrix;
5. prediction, confidence, and unknown-state contract;
6. resolve/execute approval and rollback state machine;
7. validation and capability preservation contract;
8. privacy, security, provenance, and retention design;
9. FERRIS evidence packet integration;
10. held-out workflow and success thresholds;
11. cross-platform execution plan;
12. adoption, removal, maintenance, and support plan; and
13. an explicitly approved implementation pulse.

## Initial success measures

- Every shown edge has a source and confidence.
- Planned and observed work remain distinguishable.
- Predictions are evaluated against held-out edits.
- Unknown inputs widen plans rather than disappearing.
- Validation coverage and omitted scope remain explicit.
- A maintainer reaches the correct diagnosis faster than with raw tools.
- Windows and Unix evidence agree on mechanism or explain divergence.
- OSPREY can be removed without changing repository correctness.
- No child repository depends on OSPREY to perform ordinary Cargo work.
- Every executed action has approval, rollback, and yielded evidence.

## Non-goals

- A Rust replacement language.
- A Cargo, rustc, rust-analyzer, backend, or linker fork.
- A universal compiler cache or shared writable target directory.
- Direct manipulation of rustc-private incremental state.
- A curated crate distribution before the Crates Series decision.
- Universal crate rankings or certification without renewal and ownership.
- Automatic source, manifest, profile, feature, CI, host, or validation changes.
- Autonomous upstream issues or pull requests.
- A product implementation during the Crates Series or architecture phases.
