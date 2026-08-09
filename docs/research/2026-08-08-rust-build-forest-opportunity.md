# Rust Build Forest Opportunity

Date: 2026-08-08
Status: Strategic architecture recorded; bounded read-only prototype authorized
Decision: adopt the labeled build-forest model as a flagship FERRIUM Hammer
opportunity. PERF-Q30 now authorizes a bounded read-only reference, policy, and
visualization layer plus disposable exact-identity transport experiments; it
does not authorize a production artifact service or automatic restoration.
PERF-Q31 adds read-only function-cache opportunity, identity, cost, integrity,
and capability dispositions while keeping machine-code restoration and daemon
ownership outside the forest.
PERF-Q32 adds read-only dependency-surface, hint-eligibility, frontend-work,
codegen-ownership, duplication, and final-retention dispositions while keeping
profile changes, source slicing, stub rlibs, and compiler ownership outside the
forest.
PERF-Q33 adds host, execution-substrate, path-placement, cache-layer, CPU,
memory, security, indexing, power, thermal, background-pressure, and
attribution dispositions while keeping host mutation outside the forest.

## Executive conclusion

Rust has a precise incremental dependency graph, but it does not expose an
operator-visible compilation forest.

Rustc's cache is optimized to prove and reuse work within one crate compilation.
Cargo adds package graph planning and freshness. Neither layer provides named
historical roots, mutable labels pointing to immutable build states, lineage,
workspace-wide snapshot comparison, or a supported interface for composing
cache generations across concurrent sessions.

That missing layer is strategically important for AI-native Rust development.
Developers increasingly run editors, agents, checks, tests, builds, and
experiments concurrently. A labeled build forest could make their identities,
relationships, reuse, invalidation, storage, and machine pressure visible
without weakening Cargo or rustc correctness.

The target is not a replacement compiler cache. It is an external control plane
that preserves rustc's cache as an atomic compiler-private unit while adding
safe references, history, provenance, policy, and visualization above Cargo and
rustc.

## Decision supported

This note records:

- why rustc's internal graph is not a persistent build-forest interface;
- which capabilities are missing at the Cargo and rustc boundary;
- which prior compilation-forest ideas are proven enough to reuse;
- the safe architecture boundary for a FERRIUM prototype;
- how the opportunity feeds PERF-Q30 rather than bypassing its trust work; and
- how PERF-Q31 function-level evidence is represented without turning the
  forest into a machine-code cache.

It does not authorize remote transport, direct rustc cache manipulation, a
compiler fork, a shared writable target directory, or a production artifact
service.

## Evidence reviewed

### FERRIUM evidence

- [Cargo build-unit identity](2026-08-07-cargo-build-unit-identity.md)
- [Cross-workspace artifact reuse](2026-08-08-cross-workspace-artifact-reuse.md)
- [CI cache topology](2026-08-08-ci-cache-topology.md)
- [Editor and Cargo contention](2026-08-08-editor-cargo-contention.md)
- [Query dependency precision](2026-08-08-query-dependency-precision.md)
- [Incremental cache overhead](2026-08-08-incremental-cache-overhead.md)
- [Function-level machine-code caching](2026-08-09-function-level-machine-code-caching.md)
- [Crate slicing and partial dependency compilation](2026-08-09-crate-slicing-partial-compilation.md)
- [System effects on Rust build latency](2026-08-09-system-effects-build-latency.md)
- [Incremental reuse boundaries](2026-08-07-rust-incremental-reuse-boundaries.md)

### Prior compilation-forest corpus

The comparison reviewed the established CRAFTWORKS compilation-forest model:

- `CRAFTWORKS/compiler-sdk/src/astro/compiler/build-cache.ts`
- `CRAFTWORKS/compiler-sdk/src/astro/compiler/cache-snapshot.ts`
- `CRAFTWORKS/compiler-sdk/src/astro/compiler/compilation-forest.ts`
- `CRAFTWORKS/craftworks-sdk/src/pipeline/build-forest.ts`
- `CRAFTWORKS/compiler-sdk/tests/astro/compiler/cache-snapshot.integration.test.ts`
- `CRAFTWORKS/design/astro/compiler/compilation-host.md`

It also reviewed the in-progress Rust CRAFT migration:

- `CRAFT/crates/craft-ir/src/assembly/mod.rs`
- `CRAFT/crates/craft-host/src/host.rs`
- `CRAFT/crates/craft-host/tests/compile_forest_e2e.rs`
- `CRAFT/crates/craft-rs/tests/vtrace_pilot_forest_fidelity.rs`

The CRAFTWORKS implementation demonstrates named snapshots, content-derived
build keys, workspace DAGs, tiered references, profile and variant names,
atomic save and restore, integrity checks, snapshot diff, and pruning. It is
not a complete Git-like object database: snapshot names are free-form strings,
snapshots copy records, ancestry is not first-class, and cross-process or remote
coordination remains incomplete. The Rust CRAFT migration has typed forest IR
and topology verification, but its complete execution and cache behavior remain
under development.

## Capability comparison

| Capability | rustc and Cargo | Labeled build forest |
|---|---|---|
| Fine-grained dependencies | Internal rustc query graph | References internal evidence without replacing it |
| Workspace topology | Cargo package and unit graphs | Durable roots spanning selected units and targets |
| Cache history | Current and prior rustc generations | Arbitrary pinned historical roots |
| Human labels | No supported cache-label model | Mutable labels pointing to immutable roots |
| Lineage | No operator-facing ancestry | Parent and merge relationships between roots |
| Snapshot operations | No supported list, diff, restore, or pin API | List, diff, restore, pin, expire, and prune |
| Reuse tiers | Cargo artifacts and compiler-private query/work products | Explicit artifact, incremental-generation, evidence, and validation refs |
| Concurrent sessions | Filesystem locks and isolated target choices | Session ownership, pressure, conflict, and reuse visibility |
| Cross-machine reuse | External tools at artifact granularity | Provenance-gated local or remote content store |
| Visualization | Timings and optional compiler diagnostics | Graph, labels, reuse, invalidation, cost, and storage |

## Findings

### FERRIUM-217: rustc has an incremental graph, not an operator build forest

**Sources:** PERF-Q17 and PERF-Q18.

**Observed behavior:** Rustc persists dependency, query-result, and backend
work-product state for precise reuse, but its files and node identities are
compiler-private. The measured cache retained a bounded pair of session
generations rather than arbitrary named history.

**Implication:** The absence of labels and lineage is a layer gap, not evidence
that rustc's query engine is defective.

**Confidence:** High.

### FERRIUM-218: Cargo freshness does not supply historical reference semantics

**Sources:** PERF-Q02, PERF-Q05, PERF-Q06, and PERF-Q18.

**Observed behavior:** Cargo can avoid rustc entirely when a unit is fresh and
can organize package and target work, but ordinary target state does not provide
named roots, immutable history, snapshot diff, or safe cross-workspace
provenance.

**Implication:** A build forest belongs above Cargo's supported evidence and
below user-facing orchestration rather than inside Cargo freshness checks.

**Confidence:** High.

### FERRIUM-219: named snapshots and tiered references are proven control-plane primitives

**Sources:** the reviewed CRAFTWORKS build-cache, cache-snapshot,
compilation-forest, orchestration, and integration-test corpus.

**Observed behavior:** The prior system successfully represented workspace
DAGs, content-derived cache keys, compile/boost/evaluation tiers, named
snapshots, integrity checks, restore, diff, list, and prune operations.

**Implication:** FERRIUM should reuse the architectural lessons while replacing
free-form copied snapshots with a stricter identity and reference contract.

**Confidence:** High for the local implementation; medium for transfer to Rust
build artifacts.

### FERRIUM-220: rustc incremental generations must remain atomic and opaque

**Sources:** PERF-Q18 recovery controls and rustc persistence source review.

**Observed behavior:** Removing one internal cache component while retaining a
dependency graph that marked its results reusable violated a compiler invariant.
Removing the complete isolated cache restored safe recomputation.

**Implication:** Forest nodes may reference or package a complete generation
under exact compatibility identity. They must not merge, prune, deduplicate, or
restore individual rustc internal files.

**Confidence:** High.

### FERRIUM-221: parallel AI sessions make the forest a control-plane problem

**Sources:** PERF-Q07, PERF-Q16, PERF-Q17, and PERF-Q18.

**Observed behavior:** Concurrent editor, Cargo, compiler, and agent sessions
can trade lock waits for duplicated CPU, memory, storage, and compiler work.
Current outputs do not present those sessions as related branches with visible
reuse and invalidation.

**Implication:** The flagship value is not merely a larger cache. It is a visual
and policy-aware control plane for build identity, lineage, contention, reuse,
failure, storage, and validation evidence.

**Confidence:** High for the explanation gap; medium for product viability.

## Target architecture

The conceptual model is:

```text
label -> immutable forest root
          |
          +-- workspace and revision identity
          +-- Cargo unit and dependency edges
          +-- toolchain, target, feature, profile, and environment identity
          +-- artifact content references
          +-- optional atomic rustc incremental-generation references
          +-- dependency surface, demand, and codegen-ownership summaries
          +-- host, VM, placement, resource, and interference summaries
          +-- command, timing, validation, and failure evidence
          +-- parent root or roots
```

Labels such as `main`, `before-refactor`, `release-candidate`, or
`agent-session-27` are mutable references. Roots and their referenced nodes are
immutable. Retention operates from pinned labels and roots rather than by
editing compiler-private cache contents.

The first implementation boundary should be a read-only manifest and graph:

1. Observe Cargo units, commands, freshness, artifacts, and isolated rustc cache
   generations.
2. Compute a complete external identity without claiming rustc internal-format
   stability.
3. Record immutable roots and human labels.
4. Visualize sessions, branches, invalidation, reuse, storage, and contention.
5. Recommend supported rebuild or whole-cache recovery actions.

PERF-Q30 defines remote publication, restoration, signing, producer trust,
revocation, retention, and compatibility in
[remote artifact provenance and Rust Build Forest roots](2026-08-09-remote-artifact-provenance.md).
The forest separates action identity from content identity and labels, stages
successful finalized roots atomically, verifies before installation, and
materializes immutable compiler state into isolated mutable consumer
directories. Cross-platform portability and production automation remain
closed.

PERF-Q31 defines function-cache opportunity and integrity in
[function-level machine-code caching](2026-08-09-function-level-machine-code-caching.md).
The forest may record the invalidated CGU, stable and changed function
stencils, admission decision, hit or miss, restoration cost, and validation
disposition. It does not compute an independent machine-code key, retain
restorable function blobs, launch a daemon, or override rustc and Cranelift
identity.

PERF-Q32 defines selective partial-dependency evidence in
[crate slicing and partial dependency compilation](2026-08-09-crate-slicing-partial-compilation.md).
The forest may record declared surfaces, consumer demand, whole-crate frontend
work, current hint eligibility, dependency-owned and consumer-owned codegen,
repeated downstream emission, final retention, and measured outcome. It does
not edit Cargo profiles, transform source, construct stub rlibs, infer semantic
reachability, or skip compiler correctness work.

PERF-Q33 defines environment comparison and attribution in
[system effects on Rust build latency](2026-08-09-system-effects-build-latency.md).
The forest may record host and guest identity, source and target placement,
cache-layer assumptions, CPU and memory policy, concurrent sessions, security
and indexing state, power and thermal evidence, background pressure, and
attribution confidence. It does not change those settings or treat environment
placement as a portable artifact identity.

## Recommendations

### Adopt now

- Treat the Rust Build Forest as a flagship Hammer architecture target.
- Use immutable nodes and roots with mutable human labels.
- Preserve parent lineage, failure state, provenance, and validation evidence.
- Distinguish Cargo freshness, crate artifacts, rustc query reuse, backend work
  products, and final linking.
- Preserve complete rustc incremental generations as opaque atomic units.
- Record function-cache opportunity and outcome summaries without treating
  function blobs as forest artifacts.
- Record partial-dependency eligibility, codegen ownership, duplication, and
  outcome summaries without treating the forest as a compiler scheduler.
- Record environment identity and pressure so roots are not compared as if
  unlike hosts, mounts, VM limits, or security states were equivalent.

### Prototype behind a compatibility boundary

- A read-only local forest manifest built from supported Cargo evidence.
- Named, signed, versioned, and expiring labels with pinned immutable roots.
- A visual graph of concurrent sessions, unit identities, cache generations,
  reuse, invalidation, storage, and machine pressure.
- Exact-identity transport experiments for whole-generation references in
  disposable fixtures.
- Read-only function-stencil, admission, hit, miss, integrity, and capability
  summaries from upstream-owned Cranelift experiments.
- Read-only baseline-versus-hinted dependency-surface comparisons from
  disposable nightly Cargo experiments.
- Read-only environment comparison guards and placement/session visualizations.
- Policy-only rollback, expiry, revocation, retention, and quarantine controls.

### Reject or defer

- Directly reading semantic values from rustc cache files as a stable API.
- Combining internal files from different rustc generations.
- Sharing writable incremental or target directories across unrelated builds.
- Automatic restoration, production remote transport, or cross-machine reuse.
- Build-script, proc-macro, native-tool, path-package, or unknown
  execution-cone artifacts.
- Function-level machine-code restoration, a forest-owned daemon, or an
  independent source-, MIR-, object-, or symbol-derived cache key.
- Automatic `hint-mostly-unused` adoption, source-level slicing, stub rlibs,
  deferred frontend work, or compiler-private reachability.
- Security exclusions, service or power changes, forced memory pressure,
  repository migration, universal job tuning, or other forest-owned host
  mutation.
- Replacing Cargo, rustc, or established artifact-cache tools.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted: the forest cannot weaken compiler correctness and treats internal generations as opaque atomic state. |
| Compiler Performance Engineer | Accepted: reuse claims remain separated by Cargo, frontend, backend, link, storage, and contention layers. |
| Interop Boundary Auditor | Accepted: native dependencies and platform identity remain explicit PERF-Q30 inputs. |
| AI Assurance Skeptic | Accepted: labels identify evidence roots but do not certify correctness. |
| Ecosystem Strategist | Accepted: the opportunity complements Cargo and rustc rather than replacing them. |
| Rust Maintainer | Accepted: stable Cargo evidence comes first and optional compiler integration remains removable. |
| Native Platform Adopter | Accepted: provenance, rollback, retention, corruption, and operational pressure are first-class. |
| Scope Keeper | Accepted: the immediate boundary is read-only recording and visualization. |
| Validation Checker | Accepted: implementation remains gated on exact identity, fixtures, negative cases, and measured benefit. |

## Non-goals

- Defining a stable rustc incremental-cache format.
- Claiming arbitrary cache generations are portable.
- Authorizing a production remote cache service or automatic restore.
- Treating a label or cache hit as correctness evidence.
- Building product code during the current research pulse.
