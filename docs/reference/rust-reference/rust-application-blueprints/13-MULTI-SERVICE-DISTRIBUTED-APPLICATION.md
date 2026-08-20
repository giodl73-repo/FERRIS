---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:multi-service-distributed-application
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Multi-Service Distributed Application Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/13-MULTI-SERVICE-DISTRIBUTED-APPLICATION.md
canonical_path: rust-application-blueprints/13-MULTI-SERVICE-DISTRIBUTED-APPLICATION.md
backsource_ids: [proof-backfill:rust-application-blueprints:13-multi-service-distributed-application]
concepts: [distributed application, service boundary, data ownership, consistency, resilience, contract testing, independent deployment]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Multi-Service Distributed Application Blueprint

## The Big Picture

```
+============================================================================+
| clients -> edge/gateway -> independently deployed service authorities      |
+----------------------+----------------------+------------------------------+
                       v                      v
                 service A                service B
                 owns data A              owns data B
                       |                      |
                       +--> API/events <------+
                               |
                               v
                    workflow/projection services
                               |
                               v
              shared platform: routing | identity | telemetry | delivery
```

A distributed application earns its complexity only when boundaries need
independent authority: data ownership, deployment, scaling, failure isolation,
security, or team responsibility. Splitting one application into networked
processes without those needs replaces local compile-time coupling with partial
failure and version skew.

## Repository Layout

```
commerce/
|-- Cargo.toml
|-- services/
|   |-- catalog/
|   |   |-- Cargo.toml
|   |   `-- src/
|   |-- orders/
|   `-- payments/
|-- crates/
|   |-- protocol-primitives/    # small, stable wire-level values only
|   |-- telemetry-contract/
|   `-- test-support/
|-- contracts/
|   |-- http/
|   `-- events/
|-- deploy/
`-- tests/
    `-- system-scenarios/
```

```toml
[workspace]
resolver = "3"
members = ["services/*", "crates/*", "tests/*"]
```

A single workspace can host multiple deployables, but every service needs its
own artifact, configuration, owner, data authority, SLO/operational objective,
and rollback path. Avoid sharing internal domain crates across services if doing
so creates lockstep release authority.

## Service and Data Boundaries

| Boundary test | Strong signal |
|---------------|---------------|
| Data | one service is the sole mutation authority |
| Lifecycle | service can deploy/roll back without coordinated code release |
| Failure | outage is contained and callers have a defined degraded behavior |
| Scale | workload shape differs materially |
| Security | separate identity/capability boundary is required |
| Team | owner can operate and evolve the contract |

```
service A needs fact owned by B
      |
      +--> synchronous query: fresh, coupled availability/latency
      |
      `--> subscribed projection: stale by policy, locally available
```

Do not share a database schema as a shortcut. If two services can mutate the
same records, the architectural owner is unresolved. Read replicas or analytical
copies are acceptable only with explicit lag and mutation restrictions.

## Failure, Consistency, and Coordination

| Mechanism | Use | Constraint |
|-----------|-----|------------|
| Deadline | bound end-to-end waiting | propagate remaining budget |
| Retry | transient idempotent attempt | bounded, jittered, non-multiplying |
| Circuit/load shedding | protect capacity | define degraded result |
| Outbox/event | asynchronous fact propagation | duplicates and lag |
| Saga/process manager | long-running coordination | compensation is domain action |
| Cache/projection | local availability/latency | staleness and invalidation policy |

```
client deadline
   |
service A budget
   |-- local work
   `-- call B with smaller remaining budget
          `-- no retry beyond caller's total budget
```

Platform teams own routing, workload identity, deployment substrate, and shared
telemetry infrastructure. Service teams own semantic contracts, capacity
assumptions, dependency policy, runbooks, and data recovery. A platform default
must not silently become business semantics.

Transport identity or mTLS authenticates a workload; it does not authorize a
domain action. Each service must validate caller capability, tenant/data scope,
message/request size, and propagated identity at its own authority boundary.
Protect internal endpoints from becoming an SSRF or confused-deputy path, and
partition credentials so one service compromise does not grant mutation rights
to another service's store.

## Testing and Rollback

```
service-local tests
   -> serialized provider/consumer contract tests
   -> real dependency integration tests
   -> multi-service scenario tests
   -> failure/latency/rollback exercises
```

```text
cargo test --workspace --all-targets
# build each service artifact independently
# run a repository-owned environment with at least N and N-1 contract fixtures
# inject one dependency timeout and verify bounded degraded behavior
```

Rollback must assume mixed versions:

| Surface | Rule |
|---------|------|
| API | additive/tolerant window or versioned endpoint |
| Event | consumers first, producers second; retain old readers |
| Database | expand/contract within owning service |
| Workflow | persisted state interpretable by old/new coordinator |
| Deployment | one service can revert without reverting unrelated services |

If every release and rollback requires all services together, the system is a
distributed monolith; consider merging until an independent boundary exists.

Extracting or retiring a service requires a data-authority transfer plan:
backfill/version state, shadow or compare reads, change the single writer,
reconcile, cut traffic, preserve a bounded rollback bridge, then revoke the old
service's routes and credentials. Indefinite dual-write makes authority
ambiguous and is a migration state, not the target architecture.

## Universal Bridge First

The universal bridge is distributed authority: network boundaries turn every
call into a fallible, delayed, duplicated, and version-skewed interaction.
Service-oriented architecture, actors, and internet protocols all require
explicit contracts under partial failure.

Supplementally, ASP.NET services, Azure hosting, and service meshes provide
familiar implementation options. They do not choose domain boundaries or make a
shared database safe.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| One team/data owner/deployment | modular monolith or HTTP service [03] |
| Independent data and release authority | separate service |
| Fresh cross-service decision | synchronous API with deadline and failure policy |
| Locally available derived fact | event-fed projection [07] |
| Long cross-service workflow | explicit process manager |
| Shared code convenience only | avoid service split; use crate/module |
| Coordinated releases everywhere | merge boundary or fix contract independence |

## Common Confusion Points

- **Network boundaries are not modularity for free.** They add failure,
  security, telemetry, and compatibility obligations.
- **A gateway does not own downstream domains.** It routes/composes according to
  an explicit edge contract.
- **Retries can amplify outages.** Budget them end to end and combine with load
  shedding.
- **Eventual consistency is incomplete language.** State expected lag,
  monotonicity, repair, and user-visible behavior.
- **Shared libraries can create lockstep services.** Share protocol primitives,
  not another service's private model.
- **Observability correlation is not a distributed transaction.** It explains
  work; it does not make effects atomic.
- **mTLS is not business authorization.** A valid workload identity still needs
  operation and data-scope policy at the owning service.

## Primary Sources

- HTTP Semantics (IETF): https://www.rfc-editor.org/rfc/rfc9110
- CloudEvents specification: https://cloudevents.io/
- AsyncAPI specification: https://www.asyncapi.com/docs/reference/specification/latest
- Rust async book: https://rust-lang.github.io/async-book/
- Cargo Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html

## Related Guides

- HTTP service: [03-HTTP-AND-API-SERVICE.md](03-HTTP-AND-API-SERVICE.md)
- Event-driven integration: [07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md](07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md)
