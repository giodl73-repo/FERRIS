# OSPREY Query Forest Component Model

Specification: OSPREY-SPEC-001
Status: Draft after nine-role review
Implementation: Not authorized
Depends on: Crates Series completion before implementation
Program identity: OSPREY is a code name, not a committed product

## Normative scope

This is the first OSPREY specification. It defines the component boundaries and
responsibilities that later schema, adapter, policy, execution, evidence, and
view specifications MUST preserve.

The current status is Planning. The requirements constrain subsequent design;
they do not authorize implementation.

Normative language follows the
[FERRIUM specification registry](README.md).

### Foundational requirements

1. The Query Forest MUST remain a canonical typed evidence model and immutable
   root history, not one monolithic service.
2. Evidence adapters MUST observe one owner boundary and MUST NOT silently
   decide policy.
3. Maps MUST be projections over canonical evidence.
4. Ledgers MUST record identity, lifecycle, capability, trust, and accounting
   without independently mutating source systems.
5. Predictions MUST remain distinguishable from observations.
6. Resolution MUST remain distinguishable from execution.
7. Execution MUST require an approved Action Plan with validation, rollback,
   cleanup, and audit.
8. Unknown, unsupported, stale, and not-observed states MUST remain distinct
   from success.
9. Engines MUST be replaceable and MUST NOT redefine canonical evidence.
10. Views MUST answer bounded user questions and MUST cite their evidence.
11. FERRIS packets MUST preserve identities, sources, limitations, approvals,
    actions, validation, outcomes, and omitted scope.
12. OSPREY implementation MUST NOT begin before the Crates Series and the
    separately approved implementation gate complete.
13. Ordinary Cargo and editor workflows MUST remain functional without OSPREY,
    and repository integration MUST be removable without correctness changes.
14. The canonical model MUST remain product-neutral. Consumer-specific
    workflows MUST remain in adapters, policy, or views.
15. Measurable claims MUST inherit the
    [build latency measurement contract](BUILD_LATENCY_MEASUREMENT_CONTRACT.md),
    including workflow, commands, environment, cache state, repetitions,
    variance, failures, and limitations.
16. Safety, soundness, security, performance, and interop claims MUST use
    dedicated evidence and MUST NOT be inferred from compilation or one passing
    validation result.
17. Model-generated actions MUST record model identity, instruction or prompt
    reference, proposed action, approval, commands, results, rejection, and
    rollback.
18. No implementation MUST be required to implement every catalog component.
    The first proof MUST use only the components required by its accepted
    maintainer workflow.
19. Later specifications MAY split components further but MUST NOT collapse
    observation, projection, identity, prediction, resolution, execution,
    validation, outcome, or evidence duties into an indistinguishable service.
20. SPEC-014 MUST define executable positive, negative, failure, unsupported,
    version-skew, serialization, projection-consistency, approval, rollback,
    and packet-completeness conformance tests before Proposed status.

## Core clarification

The Query Forest is not one service, database, graph, UI, planner, or cache.

It is a **canonical evidence model** with several bounded components and
projections:

```text
evidence adapters
  -> canonical nodes, edges, identities, states, and observations
    -> maps and ledgers
      -> predictions and resolutions
        -> approved actions
          -> outcomes and FERRIS evidence
```

Components may share a serialized record or implementation later, but their
contracts remain separate.

## Layer 1: Evidence adapters

Adapters observe one owner system and emit normalized evidence. They do not
decide policy.

### Cargo adapter

Emits:

- workspace, package, target, profile, feature, and platform scope;
- resolved dependencies and selected packages;
- planned and completed compiler units;
- freshness, message, timing, lock, and target-directory evidence; and
- build-script and procedural-macro package roles.

### rustc adapter

Emits:

- invocation identity and compiler configuration;
- compiler phases and queries when available;
- query demand, invalidation, reuse, and timing summaries;
- metadata, monomorphization, CGU, backend, emission, and incremental evidence;
  and
- diagnostics, failure, and unsupported-toolchain states.

Stable evidence and versioned nightly evidence remain separate.

### rust-analyzer and editor adapter

Emits:

- editor session and semantic-database identity;
- flycheck and build-data commands;
- process, cancellation, lock, and target topology; and
- foreground latency and duplicate-work evidence.

### Macro and build-script adapter

Emits:

- native executable identity;
- declared and observed inputs;
- environment and filesystem reads when supported;
- generated outputs and ownership;
- Cargo instructions and native metadata; and
- hidden-input or stale-output uncertainty.

### Backend and linker adapter

Emits:

- backend, target, optimization, object, archive, symbol, and relocation data;
- link plan, input identity, linker engine, debug package, incremental state,
  fallback, and final output; and
- runtime, debugger, ABI, panic, unwind, and native-library capabilities.

### Validation adapter

Emits:

- checks, lints, tests, examples, doctests, release checks, formatting, policy,
  packaging, compliance, and deployment gates;
- package, target, feature, profile, platform, and execution scope;
- selected versus full-reference evidence; and
- pass, fail, skipped, unknown, and mandatory states.

### Ecosystem adapter

Defined by the Crates Series. It must emit:

- crate version, revision, license, owner, release, and maintenance identity;
- dependency, feature, build-script, macro, native, and unsafe closures;
- capability, interchange, async-runtime, MSRV, platform, `no_std`, WASM, and
  embedded evidence;
- advisory, audit, provenance, stewardship, and abandonment evidence; and
- compatibility-profile and renewal state.

### Environment adapter

Emits:

- host, guest, OS, architecture, toolchain, source, target, Cargo home, and
  temporary-storage placement;
- CPU, jobserver, memory, swap, session, and background-pressure state;
- filesystem, VM, container, security, indexing, power, and thermal evidence;
  and
- attribution confidence.

### Upstream adapter

Emits:

- upstream repository, issue, goal, benchmark, PR, owner, and status;
- accepted contribution vocabulary and required evidence;
- packet, review, maintenance, supersession, and external disposition; and
- links to authoritative artifacts.

## Layer 2: Canonical graph

The canonical graph contains typed nodes and typed edges. It is the shared
language from which the maps and ledgers are projected.

### Node types

#### Organizational scope

- Portfolio
- Repository
- Workspace
- Package
- Target
- Feature set
- Profile
- Platform

#### Source and input

- Revision
- Change set
- Source file
- Generated file
- Runtime data
- Environment input
- Native dependency
- Registry dependency
- Path dependency
- Build-script input
- Macro input

#### Execution

- Session
- Command
- Cargo unit
- rustc invocation
- Compiler phase
- Compiler query
- Macro invocation
- Build-script run
- Validation activity
- Repository gate
- Action

#### Compilation and artifacts

- Metadata artifact
- Monomorphized item
- Codegen unit
- Backend work product
- Object file
- Archive or library
- Link plan
- Linker state
- Debug package
- Executable or final artifact
- Incremental generation
- Cache entry

#### Evidence and governance

- Observation
- Prediction
- Resolution
- Approval
- Outcome
- Limitation
- Unknown
- Capability
- Safety claim
- Interop boundary
- ABI contract
- Model identity
- Agent action
- Policy
- Owner
- Upstream issue or goal
- Contribution packet
- FERRIS evidence packet
- Forest root
- Human label

### Edge types

#### Structure

- `CONTAINS`
- `MEMBER_OF`
- `SELECTS`
- `CONFIGURES`
- `TARGETS`

#### Dependency and demand

- `DEPENDS_ON`
- `DEMANDS`
- `READS`
- `IMPORTS`
- `EXPANDS`
- `GENERATES`
- `CONSUMES`
- `LINKS`
- `CROSSES_BOUNDARY`

#### Causality

- `CHANGED_BY`
- `INVALIDATES`
- `REBUILDS`
- `REUSES`
- `RESTORES`
- `SKIPS`
- `FALLS_BACK_TO`
- `FAILS_BECAUSE`
- `BLOCKS`
- `WAITS_ON`

#### Production and validation

- `PRODUCES`
- `DERIVED_FROM`
- `PACKAGES`
- `VALIDATES`
- `COVERS`
- `REQUIRES_GATE`
- `PRESERVES_CAPABILITY`
- `LOSES_CAPABILITY`

#### History and governance

- `PARENT_OF`
- `LABELED_BY`
- `OWNED_BY`
- `APPROVED_BY`
- `PROPOSED_BY`
- `VERIFIED_BY`
- `ROLLED_BACK_BY`
- `PREDICTS`
- `OBSERVED_AS`
- `RESOLVED_AS`
- `YIELDS`
- `SUPERSEDES`
- `CONTRIBUTES_TO`

Edges carry source, confidence, observation time, tool version, and limitations.

### State vocabulary

Execution and evidence states are explicit:

- planned;
- selected;
- queued;
- running;
- fresh;
- dirty;
- invalidated;
- reused;
- restored;
- rebuilt;
- relinked;
- skipped;
- passed;
- failed;
- cancelled;
- blocked;
- unsupported;
- stale;
- unknown; and
- not observed.

`Unknown`, `unsupported`, and `not observed` are not aliases for success.

## Layer 3: Maps

Maps answer structural questions. They are projections, not independent
sources of truth.

### Scope Map

Answers:

> What contains what, and which owner controls each boundary?

Shows repositories, workspaces, packages, targets, crates, compiler work,
artifacts, validation, ecosystem dependencies, environments, and owners.

### Dependency Map

Answers:

> What requires or consumes what?

Shows Cargo dependencies, compiler query demand, macro/build-script inputs,
generic demand, link inputs, runtime data, validation prerequisites, and native
or ecosystem dependencies.

### Change Map

Answers:

> What changed, where is it owned, and what declared mappings apply?

Shows source, configuration, lockfile, generated, runtime-data, environment,
policy, native, dependency, and toolchain changes.

### Invalidation Map

Answers:

> Which changes made which prior work unusable, and why?

Shows direct and transitive invalidation, hidden-input uncertainty, semantic
cutoff, retained-artifact compatibility, and fallback.

### Demand Map

Answers:

> Which downstream consumer actually requested each item or capability?

Shows query demand, generic instantiation, dependency codegen, feature
activation, target selection, link retention, and validation coverage demand.

### Critical-Path Map

Answers:

> What determined user-visible completion time?

Shows ready time, queue delay, execution, waits, locks, serial regions,
parallel width, resource contention, linking, validation, and final completion.

### Environment Map

Answers:

> Under which machine, storage, security, and concurrency conditions was the
> evidence produced?

Shows equivalence, differences, pressure, unsupported attribution, and
comparison confidence.

### Capability Map

Answers:

> Which user, runtime, debugging, platform, safety, and operational capabilities
> does this plan preserve or alter?

This prevents a faster command from being treated as equivalent when it drops
release optimization, targets, symbols, tests, lints, ABI support, or policy.

### Ownership Map

Answers:

> Who owns the behavior, evidence, decision, maintenance, and rollback?

Separates repository, Cargo, compiler, backend, linker, crate maintainer,
platform, security, CI, FERRIUM, and upstream ownership.

### Interop Boundary Map

Answers:

> Which semantics and guarantees cross a language, process, ABI, allocator,
> runtime, native-library, generated-binding, or platform boundary?

Records calling convention, layout, ownership, lifetime, aliasing, exception,
panic, unwind, threading, allocation, deallocation, error, synchronization,
versioning, generated binding, capability, negative-test, rollback, and owner
evidence. A C-shaped interface does not imply preservation of richer Rust or
C++ semantics.

## Layer 4: Ledgers

Ledgers answer identity, lifecycle, and accounting questions over time.

### Identity Ledger

Records every distinct identity domain:

- source and revision;
- Cargo unit;
- compiler semantic and incremental;
- interface and retained definition;
- generic instance;
- CGU and backend work product;
- object, archive, link state, and final output;
- environment;
- validation plan;
- crate ecosystem and release;
- evidence packet; and
- forest root.

### Reuse Ledger

Records:

- eligible reuse;
- proof performed;
- hit, miss, skip, or restoration;
- population, lookup, restore, and reconstruction cost;
- avoided work;
- integrity and provenance;
- downstream containment; and
- rejection or fallback reason.

### Artifact Ledger

Records artifact producer, inputs, toolchain, target, profile, features,
backend, capabilities, bytes, location, retention, trust, integrity, consumers,
and cleanup ownership.

### Input and Side-Effect Ledger

Records declared, observed, hidden, generated, native, network, environment,
filesystem, and persistent-output inputs and side effects.

### Cost Ledger

Records wall, stable work, CPU, memory, I/O, storage, transport, artifact,
runtime, binary-size, validation, and human-maintenance cost.

### Validation Coverage Ledger

Records selected packages and every retained or omitted activity, feature,
target, profile, platform, doctest, execution, policy, compliance, and release
gate. It distinguishes selected-plan evidence from full-reference evidence.

### Capability Ledger

Records required, observed, preserved, reduced, unsupported, and unverified
capabilities for each plan and outcome.

### Provenance and Trust Ledger

Records producer, revision, commands, environment, signatures or hashes,
publication, installation, revocation, retention, access, and consumer trust.

### Assurance Ledger

Records:

- evidence source and claim class;
- model identity and agent action;
- instruction or prompt reference;
- observed, inferred, predicted, and unknown assertions;
- human approval or rejection;
- commands, validation, failures, and limitations;
- safety, soundness, security, performance, and interop evidence; and
- rollback or escalation.

Compilation and one passing test cannot satisfy a dedicated assurance claim.

### Crate Ecosystem Ledger

Defined by the Crates Series. Records crate role, capability, version, feature
closure, interchange contracts, stewardship, advisories, licenses, unsafe and
native boundaries, target support, compatibility profiles, renewal, and
deprecation.

### Lineage Ledger

Records immutable roots, parent relationships, mutable labels, sessions,
branches, comparisons, supersession, pinning, expiry, and deletion reachability.

### Adoption and Operations Ledger

Records:

- supported and unsupported platforms, tools, ABIs, and deployment models;
- installation, configuration, upgrade, removal, and rollback;
- CI, storage, network, credential, and endpoint-control requirements;
- training, documentation, support, ownership, and on-call burden;
- compliance, audit, retention, recovery, and disaster scenarios;
- operational failures and diagnostics; and
- measured adoption and maintenance cost.

## Layer 5: Plans and records

These components support the OSPREY verbs. They are not graph maps.

### Observation Record

One normalized account of what a command or session actually did, including
unknowns and failures.

### Change Record

The proposed or observed change set, ownership, mappings, risk classes, and
unsupported inputs.

### Prediction Record

Forecasts packages, targets, compiler work, artifacts, links, validation,
capabilities, cost, uncertainty, and fallback before execution.

### Resolution Record

States the selected decision:

- explain;
- diagnose;
- compare;
- validate;
- execute;
- contribute upstream;
- defer;
- reject; or
- request human or owner input.

It includes alternatives and reasons.

### Action Plan

Defines exact approved commands, isolation, permissions, mutation boundaries,
network and credential use, validation, rollback, cleanup, and stop conditions.

### Approval Record

Names the approver, scope, expiration, conditions, and prohibited actions.

### Model Action Record

Names the model and agent, instruction or prompt reference, evidence available
at decision time, proposed action, uncertainty, approval, rejection, commands,
results, failure, and rollback. It does not convert model reasoning into an
observation.

### Execution Record

Records the commands actually run, deviations from plan, outputs, failures,
rollback, cleanup, and environment.

### Outcome Record

Compares prediction, resolution, execution, validation, capability, cost, and
user impact. It records whether the action should be retained, reverted,
changed, or escalated.

### Adoption Record

Records the consumer, supported workflow, installation, training, operational
owner, support burden, platform coverage, removal, rollback, audit,
maintenance, and measured user impact.

### Upstream Contribution Packet

Adapts one case to an upstream owner, benchmark, issue, or PR using the
[Rust performance contribution packet](RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).

### FERRIS Evidence Packet

The final portable review artifact joining:

- identities and provenance;
- model and agent actions where applicable;
- change and observation;
- shown Query Forest slices;
- prediction and error;
- resolution and approval;
- execution and rollback;
- validation and capabilities;
- outcome and limitations; and
- upstream or deferred disposition.

## Layer 6: Engines

Engines operate on the canonical model but remain replaceable.

### Normalizer

Validates adapter evidence and converts it to canonical nodes, edges, states,
identities, and observations.

### Forest assembler

Builds one immutable observed root from normalized evidence. It does not infer
causes that lack evidence.

### Projection engine

Produces the maps and ledgers for one question without copying or redefining
the underlying evidence.

### Causality engine

Classifies observed direct, transitive, inferred, and unknown causes with
source and confidence.

### Prediction engine

Produces held-out forecasts and calibrated uncertainty. It cannot silently
convert predictions into observations.

### Resolution engine

Applies repository policy, capability requirements, upstream ownership,
economics, and human preferences to generate alternatives and a recommended
decision.

### Execution orchestrator

Runs only approved Action Plans. It enforces isolation, permissions, stop
conditions, validation, rollback, cleanup, and evidence capture.

### Evidence packager

Yields FERRIS and upstream contribution packets without posting externally
unless a separate approved action permits it.

## Layer 7: Views

Views answer one user question. They are not separate sources of truth.

- Rebuild explanation view
- Critical path and contention view
- Invalidation and reuse view
- Artifact and cache compatibility view
- Generic ownership and duplication view
- Backend, CGU, LLVM, emission, and linking view
- Environment comparison view
- Crate ecosystem and dependency-governance view
- Validation coverage view
- Prediction-versus-observation view
- Capability consequence view
- Action, approval, rollback, and outcome view
- Historical root and lineage view
- Upstream contribution readiness view

The first bounded proof should implement only the views necessary for one
maintainer workflow.

## Minimal first proof

The smallest future OSPREY proof needs only:

1. Cargo adapter;
2. Environment adapter;
3. Validation adapter;
4. canonical scope, dependency, command, artifact, observation, and unknown
   nodes;
5. `CONTAINS`, `DEPENDS_ON`, `PRODUCES`, `REBUILDS`, `REUSES`, `WAITS_ON`,
   `VALIDATES`, and `OBSERVED_AS` edges;
6. Scope, Dependency, Critical-Path, and Validation maps;
7. Identity, Cost, and Validation Coverage ledgers;
8. Observation and Outcome records; and
9. one rebuild-explanation view plus a FERRIS packet.

It does not initially require:

- compiler-private query ingestion;
- prediction;
- action execution;
- remote storage;
- cache restoration;
- full ecosystem ingestion;
- automatic validation selection;
- source or workflow mutation; or
- external posting.

## Design rules

1. Maps are projections; evidence is not copied into contradictory models.
2. Ledgers record identity and lifecycle; they do not recommend actions.
3. Predictions never become observations without execution evidence.
4. Unknown and unsupported states remain first-class.
5. Capabilities and validation are not inferred from compilation success.
6. Every edge has a source, confidence, time, and owner.
7. No universal cache or compatibility identity is introduced.
8. Adapters preserve upstream ownership and version boundaries.
9. Execution requires an approved plan and rollback.
10. Yielded packets are review artifacts, not correctness certificates.
11. Ordinary Cargo and editor operation must not depend on the Forest.
12. Maintainer-facing views must use actionable language and link to evidence
    rather than requiring knowledge of internal graph terminology.
13. Every adapter and projection requires positive, negative, failure,
    unsupported, and version-skew conformance cases before implementation
    claims.
