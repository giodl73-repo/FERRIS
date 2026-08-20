# rust-production-engineering/ - Status

**17 files (STATUS.md + 16 guides) | Complete | Source-first, awaiting source-backfill validation**

This canonical module covers operating Rust software after language and compiler
mechanics are understood: configuration, observability, diagnostics, lifecycle,
capacity, resilience, persistence, artifacts, hosting, delivery, recovery,
incidents, service governance, and release assurance.

```text
inputs -> execution -> evidence -> state -> delivery -> operations
  01       05-07       02-04      08       09-12       13-15
```

## Guides

| File | Topic | Status |
|---|---|---|
| `00-OVERVIEW.md` | Production-engineering landscape, layer boundaries, operating loop, and reading paths | done |
| `01-CONFIGURATION-ENVIRONMENTS-AND-SECRETS.md` | Typed configuration, precedence, validation, secret lifecycle, and workload identity | done |
| `02-STRUCTURED-LOGGING-AND-TRACING.md` | Structured events, spans, context propagation, sampling, volume, and redaction | done |
| `03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md` | Metric semantics, cardinality, histograms, health contracts, and telemetry architecture | done |
| `04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md` | Failure taxonomy, error chains, panic policy, backtraces, dumps, and symbols | done |
| `05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md` | Startup, readiness, signals/controls, bounded draining, cancellation, and termination | done |
| `06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md` | Execution models, runtime selection, admission, bounded queues, task ownership, and saturation | done |
| `07-TIMEOUTS-RETRIES-BACKPRESSURE-AND-RESILIENCE.md` | End-to-end deadlines, retry safety, shedding, breakers, bulkheads, and hedging | done |
| `08-PERSISTENCE-TRANSACTIONS-AND-DATA-ACCESS.md` | Durable invariants, transactions, pools, uncertain outcomes, outbox, and migrations | done |
| `09-PACKAGING-ARTIFACTS-RELEASES-AND-UPGRADES.md` | Immutable artifacts, build identity, linking, provenance, compatibility, and upgrades | done |
| `10-CONTAINERS-KUBERNETES-SYSTEMD-AND-WINDOWS-SERVICES.md` | Portable hosting contract and platform-specific supervision/deployment adapters | done |
| `11-CI-CD-AND-PROMOTION.md` | Integration gates, pipeline trust, caching, build-once promotion, and progressive delivery | done |
| `12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md` | Evidence ladder, deterministic faults, staging limits, load/soak, and restore drills | done |
| `13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md` | Incident control, scoped triage, Rust diagnostics, hypotheses, mitigation, and reconciliation | done |
| `14-SLOS-RUNBOOKS-OWNERSHIP-AND-COST.md` | SLIs/SLOs, error budgets, alerting, runbooks, dependency ownership, and unit cost | done |
| `15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md` | Evidence-based readiness, risk classes, progressive gates, and bounded exceptions | done |

## Decision Cheat Sheet

| Need | Start with |
|---|---|
| Validate startup inputs and credentials | 01, then 05 |
| Design telemetry without uncontrolled cost | 02 and 03 |
| Bound overload, retries, and cancellation | 06 and 07 |
| Protect durable state and migrations | 08, then 12 |
| Define artifact, host, and promotion contracts | 09 through 11 |
| Prepare incident ownership and release evidence | 13 through 15 |

## Editorial Contract

All 16 guides implement the seven MAXIM style surfaces:

1. a Big Picture ASCII landscape;
2. layered drill-down from that landscape;
3. additional ASCII structural diagrams;
4. comparison and decision tables;
5. universal old-world/new-world bridges first, with Microsoft/Azure/Windows
   bridges only as supplemental context;
6. a Decision Cheat Sheet;
7. Common Confusion Points.

Every guide also contains an executable or explicitly scoped command, Rust
example, manifest, platform unit, or deployment fragment. Library, runtime, and
platform responsibilities are separated throughout. Tokio, Kubernetes, cloud
providers, systemd, and Windows SCM are presented as choices or adapters rather
than universal requirements.

## Scope Boundaries

- `../rust-language/` owns Rust language semantics, including ownership, errors,
  async/futures, threads, and unsafe Rust.
- `../rust-architecture/` owns rustc, Cargo, toolchain, standard-library, and
  artifact implementation architecture.
- This module owns production operating contracts and the choices that connect
  a Rust program to real delivery and hosting systems.

Universal production principles are load-bearing. Azure services, Azure
Pipelines, Windows tooling, and .NET prior art appear only where they add a
useful bridge for the learner profile.

## Review Notes

An independent cross-review on 2026-08-11 applied the required roles, the
seven-point contract, and Gold-level factual skepticism. The pass corrected
telemetry arithmetic, panic/allocator and unwind boundaries, bounded shutdown
and retry examples, build-identity claims, Cargo target semantics, fault-test
wording, Linux/Windows diagnostic commands, and Kubernetes/systemd/Windows SCM
lifecycle caveats.

The module is structurally complete and source-first. The review used a Gold
level of skepticism but does not claim factual Gold certification or registry
status. No inline review markers remain.

Focused PROOF over `STATUS.md` and all 16 guides reports **17 files checked,
0 errors, 0 warnings**.

## Source Custody

Every numbered guide uses `maxim.frontmatter.v1` with:

- `module: rust-production-engineering`;
- `status: source-custody`;
- `source_custody: partial`;
- matching canonical/current paths;
- module-local IDs, concepts, root concepts, and PROOF backsource IDs.

Per task scope, source backfill was not run and no `.proof`, `.mdcrop`,
`.mdport`, `.fletch`, navigation, registry, or repository-level artifact was
generated or edited. The next step is an explicitly authorized source-backfill
and independent factual review.
