# Ferris Gate B Completion Nine-Role Review

Date: 2026-08-10
Specifications: CAUSALITY-001, PREDICTION-001, VALIDATION-001, PLANNING-001
Disposition: Accepted as Draft after required revisions
Implementation authority: None

## Review question

Do the remaining Gate B specifications preserve the boundary between observed
causality, held-out prediction, capability-preserving validation, and
non-executable federated planning while providing conservative failure and
fallback behavior?

## Executive disposition

All nine roles accept the four specifications as Draft.

The review accepts:

- evidence-classified causal explanations;
- immutable predictions with evidence cutoffs and held-out evaluation;
- validation coverage independent of package selection;
- mandatory gates, full-reference comparison, and conservative widening;
- owner-specific closures and per-command Cargo plans;
- resource envelopes, observation barriers, and versioned replanning; and
- a strict boundary between Blueprint Plans and approved Action Plans.

Proposed status and implementation remain withheld pending frozen schemas,
held-out workflows, seeded failure controls, resource measurements, and
cross-platform plan fixtures.

## Role dispositions

### Rust Safety Steward

Accept after requiring causality, predictions, validation, and plans to retain
unsafe, ownership, lifetime, concurrency, aliasing, panic, unwind, macro,
build-script, native, and toolchain boundaries. Passing compilation cannot
substitute for dedicated safety or behavioral evidence.

### Compiler Performance Engineer

Accept if predicted and measured latency distinguish cold, warm, check, build,
test, codegen, link, validation, and machine-work dimensions. Resource
envelopes, variance, confounders, and over-selection cost must remain visible.

### Interop Boundary Auditor

Accept after keeping Rust, Typebook, ABI, WIT, wire, native, linker, runtime,
and deployment closures distinct. Validation must cover migration, negative,
failure, unsupported, version-skew, and rollback behavior across boundaries.

### AI Assurance Skeptic

Accept because model predictions retain evidence cutoffs, prompts, identity,
error, alternatives, and abstention. AI cannot upgrade causal claims, remove
mandatory validation, approve policy, rewrite plans, or execute actions.

### Ecosystem Strategist

Accept because Cargo and other mature owner planners retain local authority.
Ferris composes their closures and routes interface gaps upstream rather than
creating a universal build graph or compiler fork.

### Rust Maintainer

Accept if explanations and plans use owner-native commands and answer what
changed, what will run, why, what remains unknown, which checks are omitted,
and how ordinary Cargo remains available after removal.

### Native Platform Adopter

Accept for Draft. Proposed status requires Windows and Unix plans covering
native tools, SDKs, linkers, runtime loading, packaging, deployment, failure,
resource pressure, rollback, and complete removal.

### Scope Keeper

Accept after preserving separate causal, prediction, validation, planning,
resolution, execution, trust, and conformance responsibilities. Gate B ends at
non-executable planning truth.

### Validation Checker

Accept as Draft. Proposed status requires reproducible commands, frozen
fixtures, held-out edits, seeded false-omission controls, full-reference
comparisons, calibration, malformed and stale evidence, resource breaches,
replanning, and removal.

## Required revisions completed

The specifications now require:

- explicit causal claim classes and confounders;
- stage-specific build causality;
- immutable predictions and evidence cutoffs;
- held-out, calibration, and full-reference separation;
- false-omission and abstention reporting;
- validation dimensions independent of package selection;
- mandatory gates and expiring exceptions;
- selected-versus-full coverage;
- owner-specific closures and distinct Cargo activity plans;
- resource envelopes and artifact economics;
- observation barriers and immutable replans; and
- no execution authority in Gate B.

## Remaining gates

Before Proposed status:

1. freeze causal, prediction, validation, and plan schemas;
2. select three public repositories and synthetic controls;
3. freeze Windows and Unix held-out edits before prediction;
4. execute seeded validation failures and full-reference comparisons;
5. measure prediction error, calibration, investigation time, and resource
   envelopes;
6. exercise graph drift, unknown inputs, artifact rejection, pressure,
   cancellation, failure, and replanning;
7. demonstrate complete removal and ordinary Cargo fallback; and
8. repeat all nine role reviews over measured results.

## Decision

Advance CAUSALITY-001, PREDICTION-001, VALIDATION-001, and PLANNING-001 to
Draft.

Do not authorize implementation.
