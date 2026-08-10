# Blueprint Planning Engine Program

Status: Architecture planning; Phase 0 complete; implementation not authorized
Public product: Ferris
Primary command: `ferris`
Cargo entrypoint: `cargo ferris`
Internal subsystem: Blueprint Model and Blueprint Plan
Technical capability: Cargo Application Model
Historical code name: OSPREY
Name availability observation: `ferris` was occupied by an unrelated library
and `cargo-ferris` returned 404 on crates.io on 2026-08-10; these observations
are not reservations
Predecessor gate: Crates Series complete; separate implementation gates remain

## Mission

Blueprint is Ferris's proposed Cargo-native, cross-workspace planning engine.
It joins Cargo graph truth,
consumer-owned application definitions, RUNE contracts, supported profiles,
rustc, rust-analyzer, procedural macros, build scripts, backends, linkers,
validation, environment, and upstream evidence into one reviewable
application-to-confidence model.

Blueprint is not authorized implementation. This plan organizes the architecture,
research dependencies, stage gates, and smallest credible future proof.

OSPREY was the architecture code name used during the performance and Crates
Series research. Historical findings and reviews retain that term. Ferris is
the selected public product name; Blueprint names the internal model and plan.

## Product and command surfaces

Ferris exposes one semantic engine through two public entrypoints:

```console
ferris
cargo ferris
```

- `ferris` is the complete enterprise surface for applications,
  repositories, workspaces, contracts, profiles, policy, CI, deployment,
  roots, and refs.
- `cargo ferris` is the Cargo-native current-workspace surface implemented
  through Cargo's official external-subcommand convention.

The entrypoints share command IDs, configuration, schemas, plans, policy,
outputs, exit codes, evidence, and conformance. They differ only in discovery
defaults and available scope.

The public commands `ferris blueprint` and `cargo blueprint` are retired
before implementation. Qualified internal package names MAY retain Blueprint
where they implement that subsystem.

See the [Ferris program](FERRIS_PROGRAM.md) and
[Blueprint competitive positioning and CLI strategy](../research/2026-08-10-blueprint-competitive-positioning.md).

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
Blueprint supplies that model without claiming ownership of the underlying
tools.

## Application records

Blueprint separates:

- the **application definition**, which declares components, services,
  requirements, RUNE contracts, providers, platforms, validation, and
  lifecycle intent;
- the normalized **Blueprint model**, which joins that intent to Cargo graph
  truth and exact evidence; and
- the **FERRIS Application Contract**, which records resolved compatibility,
  validation, support, expiry, renewal, substitution, removal, and rollback.

Cargo remains authoritative for packages, targets, features, sources,
resolution, and lock state. Blueprint does not replace Cargo metadata,
`Cargo.toml`, `Cargo.lock`, or the resolver.

## Federated Blueprint Plan

The **Blueprint Plan** is the missing dynamic record between prediction and an
approved Action Plan. It is a versioned, non-executable DAG that composes:

- multi-dimensional scope slices and typed cross-command mappings;
- one owner-specific affected closure per participating system;
- one Cargo invocation plan per command and activity;
- contract, native, link, validation, and lifecycle work;
- artifact eligibility, integrity, trust, and net-benefit checks;
- CPU, memory, job, storage, I/O, foreground-latency, and concurrency budgets;
- uncertainty, observation barriers, fallback, and replan triggers; and
- expected evidence and root output.

The plan is global; the work is local. Cargo owns dependency resolution, unit
construction, freshness, scheduling, and compiler invocation. Rustc, linkers,
test systems, Typebook/RUNE, native tools, and cache providers retain their own
identities and execution rules. Blueprint links their plans and evidence
without flattening them into one universal graph.

A Blueprint Plan does not authorize execution. Resolution selects a plan; the
approved Action Plan supplies exact commands, permissions, isolation, stop
conditions, validation, rollback, and cleanup. See
[Blueprint federated execution planning](../research/2026-08-10-blueprint-federated-execution-planning.md).

## Blueprint workflows

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
- immutable roots, typed refs, ref history, lineage, and evidence references;
  and
- selected-plan versus full-reference confidence.

### Predict

Forecast a proposed edit's:

- owner-specific affected closures and Cargo invocation plans;
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

- compare candidate Blueprint Plans and select, widen, defer, or reject one;
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

- execute the approved projection of one versioned Blueprint Plan;
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
[Query Forest component model](../specs/FOREST_COMPONENT_MODEL.md).
The complete normative sequence is tracked in the
[FERRIS specification registry](../specs/README.md).

The Forest is the canonical evidence model and immutable-root history. It is
not one monolithic runtime component.

### 1. Scope plane

Scope is not one hierarchy. It is the bounded domain over which one ownership,
selection, compilation, execution, validation, capability, lifecycle, or
evidence statement applies.

Blueprint retains a useful organizational containment projection:

```text
portfolio
  -> repository
    -> workspace
      -> package
        -> target
```

It separately records:

- application, component, service, contract, operation, and provider scope;
- source file, generated file, module, item, body, and input scope;
- command activity, package selection, target, feature, profile, platform, and
  Cargo unit scope;
- compiler owner, query, mono-item, CGU, backend, artifact, link, and final
  output scope;
- validation activity, test binary, runtime test case, capability, and
  repository-gate scope;
- native discovery, generated binding, ABI, linked, loaded, runtime, package,
  and deployment scope; and
- observation, prediction, plan, action, outcome, root, ref, expiry, and
  lifecycle scope.

Mappings are typed, directional, cardinality-aware, conditional, sourced, and
confidence-bearing. Every command separates package and target selection from
activity, compilation, runtime execution, validation coverage, and omitted or
unknown scope. AI starts from stable owner-native anchors and may propose finer
mappings, but deterministic policy or human approval must authorize narrowing.
See
[Blueprint cross-command scope model](../research/2026-08-10-blueprint-cross-command-scope-model.md).

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
- immutable roots, parent lineage, typed refs, update generations, and
  retention.

No hash is treated as a universal compatibility key.

BLUE-Q01 defines the public reference vocabulary:

- branches are moving development refs;
- tags are write-once published refs;
- channels are policy-controlled promotion refs;
- aliases are local conveniences;
- pins and leases retain roots;
- tombstones deny future resolution while preserving history; and
- labels are searchable metadata only.

Every moving ref update uses an expected prior value and appends durable
history. Ref resolution never substitutes for compatibility, integrity, trust,
validation, availability, or restore-economics checks. Git remains
authoritative for source branches; Blueprint records their association without
creating duplicate source-control administration. See
[Rust build-state references](../research/2026-08-10-rust-build-state-references.md).

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

### Phase 0: Crates Series - Complete

Complete ECOS-Q01 through ECOS-Q12 in the
[Crates Series research program](ECOSYSTEM_LIBRARY_RESEARCH_PROGRAM.md).

This precedes Blueprint implementation because Query Forest scope must cover more
than compilation. Foundational crates introduce:

- ecosystem capability ownership;
- feature and version fragmentation;
- runtime and type-system interchange;
- async-runtime assumptions;
- unsafe, native, build-script, and macro boundaries;
- platform, MSRV, `no_std`, WASM, and embedded claims;
- advisories, licensing, stewardship, and release identity; and
- complete dependency and maintenance closures.

**Gate:** satisfied on 2026-08-10. All twelve ECOS questions are complete,
reviewed by all nine FERRIS roles, and end with an ecosystem capability,
dependency-governance, owner-routing, intervention, renewal, removal, and
rollback model that Blueprint can represent. This opens Phase 1 specification
work, not implementation.

### Phase 1: Query Forest architecture

Produce specifications only:

- node, edge, scope, identity, state, and evidence vocabulary;
- multi-dimensional scope coordinates, typed mappings, AI narrowing controls,
  and complexity budgets;
- Cargo/rustc/linker/validation/ecosystem adapter boundaries;
- immutable root, typed ref, update-generation, and lineage model;
- federated planning, closure composition, resource envelope, and adaptive
  replan model;
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
- per-command Cargo, component-closure, resource-budget, and fallback examples;
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

1. completed Crates Series and final ecosystem role review (satisfied
   2026-08-10);
2. FOREST-001 component model and the remaining applicable specifications
   in the [specification registry](../specs/README.md);
3. Blueprint Application Model, Query Forest ontology, and schema;
4. cross-command scope-coordinate and mapping specification;
5. identity, lineage, and typed-reference specification;
6. adapter and upstream ownership matrix;
7. prediction, confidence, and unknown-state contract;
8. federated Blueprint Plan, closure, resource, fallback, and replan contract;
9. resolve/execute approval and rollback state machine;
10. validation and capability preservation contract;
11. privacy, security, provenance, and retention design;
12. enterprise principal, authorization, policy, approval, tenancy, secret,
    audit, budget, data-residency, and revocation design;
13. connector manifest, support maturity, owner semantics, MCP, and removal
    contract;
14. FERRIS evidence packet integration;
15. held-out workflow and success thresholds;
16. cross-platform execution plan;
17. adoption, removal, maintenance, and support plan; and
18. an explicitly approved implementation pulse.

## Initial success measures

- Every shown edge has a source and confidence.
- Planned and observed work remain distinguishable.
- Predictions are evaluated against held-out edits.
- Every plan preserves owner-specific graphs and identities.
- Every scope claim names subject, activity, configuration, platform,
  lifecycle, evidence state, owner, and fallback as applicable.
- AI-proposed scope narrowing is reproducible, evidence-gated, and compared
  with the full-reference scope.
- Selected work equals affected closures plus mandatory gates and explicit
  conservative fallback.
- Aggregate CPU, memory, storage, I/O, and concurrency remain inside the
  declared resource envelope or trigger a recorded replan.
- Unknown inputs widen plans rather than disappearing.
- Validation coverage and omitted scope remain explicit.
- A maintainer reaches the correct diagnosis faster than with raw tools.
- Windows and Unix evidence agree on mechanism or explain divergence.
- Blueprint can be removed without changing repository correctness.
- No child repository depends on Blueprint to perform ordinary Cargo work.
- Every executed action has approval, rollback, and yielded evidence.
- Every mutable ref update prevents lost updates and preserves history.
- Every missing, incompatible, stale, corrupt, revoked, unknown, or
  uneconomic root candidate falls back to ordinary Cargo operation.

## Non-goals

- A Rust replacement language.
- A Cargo, rustc, rust-analyzer, backend, or linker fork.
- A universal compiler cache or shared writable target directory.
- A universal planner that replaces Cargo, rustc, linkers, validation systems,
  or native tools.
- BUILD-file migration, hermetic-build claims, or Bazel/Buck2-equivalent remote
  execution in the initial product.
- A second independent implementation for the Cargo and enterprise commands.
- A static generated Makefile as the canonical Blueprint Plan.
- Direct manipulation of rustc-private incremental state.
- A curated crate distribution or global lockfile.
- Universal crate rankings, scores, or certification.
- Automatic source, manifest, profile, feature, CI, host, or validation changes.
- Autonomous upstream issues or pull requests.
- A product implementation during architecture, planning-reference, or
  held-out workflow phases.
