# Blueprint Federated Execution Planning

Date: 2026-08-10
Status: Complete
Decision: add a versioned, non-authoritative Blueprint Plan between prediction
and approved execution. The plan composes system-owned impact closures,
commands, validation, reuse candidates, resource budgets, uncertainty, and
fallback into one application-level DAG while preserving each subsystem's
identity and execution authority.

The governing rule is:

> The plan is global; the work is local.

## Decision supported

This research closes
[BLUE-Q02](questions/blueprint/BLUE-Q02-federated-execution-plan.md) and
defines planning input to PLANNING-001, RESOLUTION-001, EXECUTION-001,
VALIDATION-001, TRUST-001, VIEW-001, and CONFORMANCE-001.

It answers whether Blueprint should:

- replace Cargo's unit planning;
- generate a static Makefile;
- flatten every tool into one universal graph;
- calculate one global rebuild decision; or
- coordinate owner-specific incremental plans through one application-level
  strategy.

Blueprint chooses the last option. Implementation authority remains closed.

## Evidence closure

The performance and ecosystem programs found mature planners and caches with
different scopes:

| Owner system | Native planning or state | Blueprint consequence |
|---|---|---|
| Cargo | one resolved unit graph and build queue per command, target, profile, feature, package, and platform selection | request and observe Cargo invocations; do not recreate dependency resolution |
| rustc | crate-local query demand, invalidation, incremental generations, monomorphization, CGUs, backend work, and emission | consume supported summaries; do not schedule or mutate private queries |
| rust-analyzer | separate incremental semantic database, cancellation, flycheck, and build-data processes | coordinate foreground priority and duplicate-work visibility |
| build scripts and procedural macros | host executables with declared, observed, hidden, generated, filesystem, environment, process, and network effects | model execution cones and widen on unknown inputs |
| linkers | whole-program inputs, state, debug packaging, capability, finalization, and fallback | plan link work separately from crate compilation |
| validation systems | checks, lints, tests, doctests, examples, policy, compliance, packaging, and deployment gates | preserve activity-specific coverage and mandatory gates |
| caches and artifact stores | lookup, transport, immutable content, retention, corruption, trust, and eviction | evaluate eligibility and economics; never equate a hit with correctness |
| Typebook/RUNE and boundary standards | semantic contract, C ABI, WIT, wire, profile, compatibility, and conformance identity | plan contract work independently from package versions |
| operating environment | CPU, memory, storage, filesystem, security, indexing, VM, power, thermal, and concurrent-session constraints | make the resource envelope and interference policy first-class |

Primary local evidence:

- [Cargo build-unit identity](2026-08-07-cargo-build-unit-identity.md);
- [Cargo build-unit multiplication](2026-08-08-cargo-build-unit-multiplication.md);
- [Reuse across Cargo commands](2026-08-08-command-artifact-reuse.md);
- [Cross-workspace artifact reuse](2026-08-08-cross-workspace-artifact-reuse.md);
- [Procedural-macro cost, inputs, and reuse](2026-08-08-procedural-macro-cost-input-reuse.md);
- [Build-script input, output, and rerun precision](2026-08-09-build-script-input-output-precision.md);
- [Monomorphization and generic-instance reuse](2026-08-09-monomorphization-generic-instance-reuse.md);
- [Linking and incremental linking](2026-08-09-linking-incremental-linking.md);
- [Impact-aware validation selection](2026-08-09-impact-aware-validation-selection.md);
- [Remote artifact provenance](2026-08-09-remote-artifact-provenance.md);
- [System effects on build latency](2026-08-09-system-effects-build-latency.md);
- [Rust security and provenance](2026-08-09-rust-security-provenance.md);
- [Rust native dependency boundary](2026-08-10-rust-native-dependency-boundary.md);
- [Rust compatibility-tested stack profiles](2026-08-10-rust-compatibility-stack-profiles.md);
- [Rust contract and interface strategy](2026-08-10-rust-contract-interface-strategy.md);
- [Rust ecosystem intervention decisions](2026-08-10-rust-ecosystem-intervention-decisions.md);
- [Rust build-state references](2026-08-10-rust-build-state-references.md); and
- [Query Forest component model](../specs/FOREST_COMPONENT_MODEL.md).

The complete disposition matrix is retained in
[EXP-01 finding-closure matrix](blue-q02-federated-planning/results/EXP-01-finding-closure-matrix.md).

## Four distinct planning records

Blueprint must distinguish:

1. **Application Definition:** durable consumer intent.
2. **Prediction Record:** evidence-backed forecast of impact, cost, capability,
   and uncertainty.
3. **Blueprint Plan:** versioned, non-executable federated DAG describing the
   proposed work and fallback.
4. **Action Plan:** approved executable projection with exact commands,
   authority, permissions, isolation, stop conditions, rollback, and cleanup.

A Blueprint Plan does not grant permission to execute. An Action Plan cannot
silently add work that was absent from the selected Blueprint Plan except
through a recorded fallback or approved replan.

## Federated graph model

Blueprint does not merge every subsystem into one interchangeable graph.
Instead, it links typed plans and observations:

```text
Application Definition
  + current change
  + prior Query Forest root
  + policy and resource envelope
        |
        v
Prediction and component closures
  +-- Cargo invocation and unit closure
  +-- compiler and code-generation impact
  +-- macro/build-script execution cone
  +-- native and linker closure
  +-- contract and compatibility closure
  +-- validation coverage closure
  +-- artifact eligibility and economics
  +-- environment and resource demand
        |
        v
Blueprint Plan
  -> Resolution
    -> approved Action Plan
      -> owner tools execute
        -> observed Query Forest root
```

Each owner remains authoritative for its local graph. Blueprint records typed
cross-system dependencies without claiming that a Cargo unit, rustc query,
test activity, native component, contract operation, and cache blob share one
identity or execution rule.

BLUE-Q03 defines the multi-dimensional scope coordinates and typed mappings
used to compose those closures in
[Blueprint cross-command scope model](2026-08-10-blueprint-cross-command-scope-model.md).

## Closure composition

The plan calculates the smallest defensible work set:

```text
required work =
  union(owner-specific affected closures)
  + mandatory capability and policy gates
  + explicit finalization work
  + conservative fallback for unknowns
```

This is not one global rebuild calculation. The global plan is cheap metadata
composition; execution remains local to the affected closures.

Examples:

- a Rust body edit may alter one crate's compiler work, downstream generic
  ownership, final link inputs, and selected tests without changing contracts;
- a Typebook contract edit may require generated projections, consumers,
  compatibility checks, and service tests without rebuilding unrelated crates;
- a build-script input change may widen through generated output and native
  link consumers even when ordinary Rust source is unchanged;
- a documentation-only edit may produce an empty compilation closure while
  retaining repository policy gates; and
- an unknown filesystem or environment input widens to the full safe owner
  boundary rather than disappearing.

## Per-command Cargo plans

Cargo builds a different unit graph for `check`, `build`, `test`, `clippy`,
`doc`, and `doctest`, and for changes in selected packages, targets, profiles,
features, platforms, and configuration.

Blueprint therefore records one Cargo invocation plan per activity:

```text
Cargo Invocation Plan
  -> command and selection
  -> resolved package graph
  -> observed unit graph
  -> freshness and artifacts
  -> timing, locks, failures, and limitations
```

Blueprint may select and order Cargo commands, but Cargo owns resolution,
unit construction, freshness, scheduling, and compiler invocation. Stable
`cargo metadata` is the first supported input. Unstable unit-graph or compiler
detail remains optional, versioned evidence.

## Blueprint Plan contents

One plan records:

- plan ID, version, parent plan, triggering change, prior root, and evidence
  time;
- application, source, package, contract, platform, toolchain, and environment
  identities;
- hard correctness, capability, policy, privacy, and support constraints;
- owner-specific closures with source, confidence, and unknowns;
- command and activity DAG with owner, inputs, outputs, dependencies, and
  observation barriers;
- expected fresh, rebuilt, reused, restored, skipped, failed, and fallback
  states;
- validation coverage, mandatory gates, omitted scope, and full-reference
  comparison;
- artifact candidates, integrity and trust checks, transfer/materialization
  cost, avoided work, and fallback;
- CPU, memory, job, storage, I/O, network, security, indexing, foreground
  latency, and concurrent-session budget;
- isolation, coalescing, cancellation, priority, retry, and stop policy;
- predicted critical path, user latency, machine work, bytes, and maintenance
  cost;
- approval requirements and prohibited actions;
- replan triggers, rollback, cleanup, and complete-removal behavior; and
- expected evidence and root output.

## Resource-aware scheduling

The plan must protect the developer's machine as a correctness-adjacent
operational constraint.

It may:

- cap aggregate Cargo, rustc, linker, test, and agent concurrency;
- preserve CPU and memory reserve for the editor and operating system;
- serialize memory-heavy linking or release optimization;
- prioritize low-latency `check` feedback over speculative final work;
- coalesce equivalent pending read-only observations;
- cancel superseded work without corrupting shared state;
- isolate writable targets across worktrees and unrelated repositories;
- account for storage growth, indexing, antivirus, VM, and filesystem effects;
  and
- defer non-critical work when expected benefit is lower than machine cost.

It may not silently change repository profiles, codegen settings, validation,
security controls, host configuration, or CI policy.

## Adaptive replanning

Planning is iterative because the actual owner graph may reveal information:

```text
plan generation
  -> execute to observation barrier
  -> compare predicted and observed state
  -> continue, widen, fall back, or request approved replan
```

Replan triggers include:

- Cargo resolution or unit graph differs from expectation;
- a build script, procedural macro, generator, or native tool reveals a new
  input or side effect;
- required artifacts are absent, corrupt, incompatible, revoked, or
  uneconomic;
- validation exposes an uncovered capability;
- platform, toolchain, provider, or environment identity changes;
- memory, storage, thermal, lock, or concurrent-session pressure exceeds the
  envelope; or
- a command fails, is cancelled, or produces stale or partial state.

Observed deviations become evidence. They are never rewritten as if they were
part of the original prediction.

## Consolidated contracts

The closure pass found three recurring requirements that should be defined
once and reused everywhere.

### Canonical identity taxonomy

Keep separate:

- package and source identity;
- Cargo unit and freshness identity;
- action identity;
- artifact namespace and content digest;
- rustc query and incremental-generation identity;
- generic, CGU, backend, object, linker-state, and final-output identity;
- contract and compatibility-profile identity;
- native provider, discovered component, ABI, and runtime-loaded identity;
- validation-plan and result identity;
- environment and execution identity; and
- Query Forest root and human ref identity.

### Execution-cone and uncertainty policy

Build scripts, procedural macros, native tools, generators, environment,
filesystem, network, runtime, and deployment inputs use one policy:

- declared and observed inputs are attributed;
- unsupported observation remains explicit;
- hidden or unknown inputs widen the affected owner boundary;
- unchanged output may narrow downstream work only with evidence; and
- no unknown becomes a reusable or validated claim.

### Conservative fallback contract

Missing, stale, corrupt, incompatible, replayed, revoked, unsupported,
not-observed, unknown, uneconomic, or resource-exceeding states use a named
fallback. The default is ordinary isolated owner-tool operation with the full
required validation boundary.

## Rude Q&A

**I changed one function. Why are we rebuilding the world?**
The pipeline submitted a global command without calculating component
closures. Blueprint composes the affected Cargo, compiler, link, contract, and
validation closures and runs only the defensible union.

**Why did `cargo test` rebuild after `cargo build`?**
They have different unit graphs and activity semantics. Blueprint predicts
their real overlap rather than calling the commands equivalent.

**Why are five agents melting my laptop?**
Independent processes each assumed they owned the machine. Blueprint plans one
aggregate CPU, memory, storage, and concurrency envelope.

**The cache said hit. Why did Cargo rebuild?**
Transport lookup and Cargo freshness are different decisions. Blueprint
records both and skips restoration when it cannot save net work.

**Why did a cache hit produce the wrong program?**
A path, name, or cache key was treated as identity. Blueprint requires exact
action, source, platform, provenance, and content checks or rebuilds.

**Why did one build script rerun half the workspace?**
Its execution cone or generated outputs widened the dependency closure.
Blueprint exposes the trigger and unknowns instead of hiding them.

**Why are we running every test for a README edit?**
The repository lacks an evidence-backed validation closure. Blueprint may
select an empty build closure while retaining mandatory policy gates.

**Why did a tiny Rust edit still relink?**
The final link graph changed even if most crates were fresh. Blueprint treats
link input identity and finalization as separate work.

**Why did switching worktrees ruin reuse?**
Writable state was shared unsafely or path identity changed. Blueprint
isolates mutation and only proposes reuse through an explicit immutable
boundary.

**Will Blueprint always avoid work?**
No. Correct work remains. Blueprint removes avoidable work, explains required
work, budgets it, and falls back rather than guessing.

## Recommendations

### Adopt now

- Add Blueprint Plan as a first-class non-executable record.
- Add PLANNING-001 between prediction and resolution.
- Specify federated owner graphs and per-command Cargo invocation plans.
- Standardize closure composition, identity taxonomy, execution-cone
  uncertainty, conservative fallback, resource envelopes, and adaptive
  replanning.
- Keep “the plan is global; the work is local” as the architecture test.

### Prototype behind a compatibility boundary

- Read-only plan generation from Git change, Cargo metadata, prior root, and
  validation policy.
- Per-command Cargo-plan comparison.
- Predicted versus observed closure and resource comparison.
- Dry-run resource scheduling and concurrent-session conflict detection.
- Validation-coverage selection with mandatory full-fallback controls.
- Cache-economics simulation without artifact restoration.

### Propose upstream

- stable Cargo unit-plan and artifact-manifest interfaces;
- Cargo-owned cross-workspace cache identity, integrity, locking, and garbage
  collection;
- rustc-owned stage reuse, macro caching, generic ownership, and stable linker
  input research;
- linker-owned incremental-state and capability diagnostics; and
- tool-owned machine-readable input, side-effect, and invalidation evidence.

### Reject or defer

- replacing Cargo's resolver or unit scheduler;
- a static generated Makefile as the canonical plan;
- flattening all owner graphs into interchangeable nodes;
- one universal identity, cache key, score, or compatibility hash;
- shared writable targets across unrelated workspaces or worktrees;
- hidden profile, feature, validation, host, CI, or security mutation;
- automatic remote restoration;
- compiler-private query or cache manipulation; and
- implementation before held-out plan benefit and conformance.

## Findings

### FERRIS-716: every Cargo activity creates a distinct unit plan

**Sources:** PERF-Q02, PERF-Q03, PERF-Q21, Cargo metadata and unit-graph
evidence.

**Observed behavior:** command, package, target, feature, profile, host/target,
and activity selections alter Cargo's units and required artifacts.

**Implication:** Blueprint records per-command Cargo invocation plans and must
not model one static workspace build graph.

**Confidence:** High.

### FERRIS-717: the plan is global while executable work remains local

**Sources:** PERF-Q01 through PERF-Q36 and ECOS-Q01 through ECOS-Q12.

**Observed behavior:** affected work crosses subsystem boundaries, but each
owner has distinct identities, invalidation rules, capabilities, and tools.

**Implication:** Blueprint composes owner-specific closures into an
application-level plan without taking over their execution.

**Confidence:** High.

### FERRIS-718: subsystem graphs must remain federated

**Sources:** PERF-Q17, PERF-Q18, PERF-Q21 through PERF-Q30, ECOS-Q03,
ECOS-Q04, ECOS-Q09, ECOS-Q11, and CONTRACT-001 research.

**Observed behavior:** Cargo units, rustc queries, tests, contracts, native
tools, link inputs, content blobs, and human refs have incompatible semantics.

**Implication:** typed cross-system edges may join them, but no universal graph
node or hash may erase owner-specific meaning.

**Confidence:** High.

### FERRIS-719: planning requires a canonical identity taxonomy

**Sources:** PERF-Q02, PERF-Q05, PERF-Q21, PERF-Q30, ECOS-Q03, ECOS-Q09,
ECOS-Q11, BLUE-Q01.

**Observed behavior:** package, unit, freshness, action, artifact, content,
contract, provider, validation, root, and ref identities answer different
questions.

**Implication:** PLANNING-001 must reference IDENTITY-001 and reject universal
keys.

**Confidence:** High.

### FERRIS-720: execution cones are first-class planning inputs

**Sources:** PERF-Q22, PERF-Q23, PERF-Q30, ECOS-Q05, ECOS-Q06, ECOS-Q09.

**Observed behavior:** macros, build scripts, native tools, generated code,
filesystem, environment, process, and network effects can escape ordinary
package identity.

**Implication:** unknown execution-cone inputs widen the plan and prohibit
unsafe reuse or narrow validation.

**Confidence:** High.

### FERRIS-721: validation is a closure, not a final boolean

**Sources:** PERF-Q35 and ECOS-Q11.

**Observed behavior:** package-only test selection misses activity, feature,
target, profile, platform, policy, and capability obligations.

**Implication:** every Blueprint Plan carries selected and full-reference
validation coverage plus mandatory gates and fallback.

**Confidence:** High.

### FERRIS-722: resource protection is part of the plan

**Sources:** PERF-Q07, PERF-Q16, PERF-Q17, PERF-Q18, PERF-Q33.

**Observed behavior:** concurrent editors, agents, Cargo processes, compilers,
linkers, indexing, security tools, VM layers, and storage can trade latency for
duplicated work, memory pressure, and machine instability.

**Implication:** CPU, memory, jobs, storage, I/O, foreground priority,
isolation, cancellation, and concurrent-session demand are first-class plan
constraints.

**Confidence:** High.

### FERRIS-723: planning must adapt at observation barriers

**Sources:** PERF-Q05, PERF-Q18, PERF-Q22, PERF-Q23, PERF-Q30, PERF-Q35.

**Observed behavior:** actual resolution, hidden inputs, missing artifacts,
failures, validation, and resource pressure can invalidate a prediction during
execution.

**Implication:** plans are versioned and may continue, widen, fall back, or
request an approved replan without rewriting prediction history.

**Confidence:** High.

### FERRIS-724: reuse is an economic decision after eligibility

**Sources:** PERF-Q05, PERF-Q06, PERF-Q18, PERF-Q30, BLUE-Q01.

**Observed behavior:** compatible content can cost more to locate, verify,
transfer, extract, and materialize than rebuilding.

**Implication:** reuse candidates enter a plan only after identity, trust,
integrity, compatibility, availability, and net-benefit checks.

**Confidence:** High.

### FERRIS-725: planning includes lifecycle and ownership

**Sources:** PERF-Q36, ECOS-Q05 through ECOS-Q12, CONTRACT-001 and PLATFORM-001
research.

**Observed behavior:** crate, contract, profile, provider, native, support, and
upstream decisions require owners, expiry, renewal, substitution, removal, and
rollback.

**Implication:** a plan that optimizes one run while orphaning maintenance or
support work is incomplete.

**Confidence:** High.

### FERRIS-726: Blueprint Plan is ready for specification

**Sources:** FERRIS-716 through FERRIS-725 and the complete finding-closure
matrix.

**Observed behavior:** the missing planning record and its boundaries can be
defined without implementing a scheduler or replacing owner tools.

**Implication:** add PLANNING-001 and integrate Blueprint Plan into the
application, Forest, execution, view, and conformance sequence.

**Confidence:** High.

## Nine-role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: correctness and mandatory capability gates precede incrementality; unknown execution cones widen or fall back. |
| Compiler Performance Engineer | Accepted: per-command Cargo and compiler-stage distinctions remain visible; reuse requires measured avoided work. |
| Interop Boundary Auditor | Accepted: contracts, native providers, ABI, generated code, link state, artifacts, and runtime identity remain separate. |
| AI Assurance Skeptic | Accepted: predictions, plans, approvals, observations, deviations, and outcomes remain distinct records. |
| Ecosystem Strategist | Accepted: owner routing and upstream contribution replace tool takeover or a FERRIS distribution. |
| Rust Maintainer | Accepted: Cargo retains resolution, unit construction, freshness, scheduling, and ordinary removable workflows. |
| Native Platform Adopter | Accepted: host/target, SDK, linker, filesystem, security, VM, resource, and deployment constraints are first-class. |
| Scope Keeper | Accepted: read-only plan generation and comparison may advance; scheduler replacement, mutation, and restoration remain closed. |
| Validation Checker | Accepted: selected/full coverage, mandatory gates, unknown fallback, resource exceedance, replan, removal, and rollback require conformance fixtures. |

## Limitations

- No stable Cargo unit-plan interface exists for production dependence.
- No held-out study has yet measured plan comprehension, incremental benefit,
  or machine-pressure reduction.
- Component closure precision varies by owner and evidence availability.
- Resource scheduling across processes, containers, VMs, and remote workers
  requires platform-specific validation.
- General artifact restoration and automatic mutation remain closed.
