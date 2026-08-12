---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:overview
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Rust Application Blueprints - Landscape and Reading Paths
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/00-OVERVIEW.md
canonical_path: rust-application-blueprints/00-OVERVIEW.md
backsource_ids: [mdloom-backfill:rust-application-blueprints:00-overview]
concepts: [rust applications, application blueprints, architecture selection, cargo workspace, operational boundaries, reading paths]
root_concepts: [rust-application-blueprints]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Rust Application Blueprints - Landscape and Reading Paths

## The Big Picture

```
+============================================================================+
|                    RUST APPLICATION BLUEPRINTS                             |
|----------------------------------------------------------------------------|
| CALLER-FACING: CLI [02]; HTTP/API [03]; reusable library/SDK [08]          |
| DURABLE WORK: worker [04]; scheduled [05]; ETL [06]; messaging [07]        |
| MULTI-AUTHORITY: distributed application [13]                              |
|----------------------------------------------------------------------------|
| SHARED CONTRACT [01]                                                       |
| use cases | domain policy | ports | config | errors | telemetry | shutdown |
|----------------------------------------------------------------------------|
| SPECIAL EXECUTION BOUNDARIES                                               |
| plugin host [09] | Wasm/component [10] | embedded [11] | Windows [12]      |
|----------------------------------------------------------------------------|
| REPOSITORY + LIFECYCLE                                                     |
| monorepo/multi-workspace [14] | selection, evolution, exit criteria [15]   |
+============================================================================+
```

A blueprint is a **decision frame**, not a generated product and not a crate
list. It identifies the initiating authority, the durable contracts, the
operational unit, the ownership boundary, and the evidence required to change or
roll back the application safely. Frameworks can fill adapter slots, but they do
not own the architecture.

These blueprints are neutral. No consuming product, including Ferris, is an
assumed dependency, required adopter, roadmap commitment, or compatibility
authority. A consumer must choose and validate its own blueprint.

---

## The Axes That Select a Blueprint

Start with the authority that initiates work, then constrain it by state,
latency, failure, and deployment boundaries.

| Axis | Questions that change the design |
|------|----------------------------------|
| Initiation | Human command, request, queue delivery, clock, file arrival, interrupt, or host callback? |
| Response contract | Exit status, HTTP response, acknowledgement, artifact, event, library value, or device actuation? |
| State authority | Process memory, database, broker, filesystem, host, device, or another service? |
| Failure scope | One invocation, one message, one partition, one service, one device, or the whole release? |
| Deployment unit | Binary, library version, container, Wasm component, plugin, firmware image, or repository cohort? |
| Compatibility | Source, semantic version, wire schema, ABI, host import, database, or hardware contract? |

```
What starts the work?
  human/shell    caller code    request    time/message    host/device
       |              |            |            |              |
      CLI          library        HTTP       batch/worker    plugin/device
                                    |
                       independent authority needed?
                              /             \
                            yes              no
                             |                |
                        distributed      one deployable
```

The tree narrows the field; it does not settle the decision. Guide
[15](15-BLUEPRINT-SELECTION-AND-EVOLUTION.md) supplies weighted selection and
exit criteria.

## Reading Paths

| Reader intent | Suggested path |
|---------------|----------------|
| Establish a neutral application core | 01 -> one entry blueprint -> 15 |
| Ship a request/response service | 01 -> 03 -> 13 -> 15 |
| Process durable asynchronous work | 01 -> 04 -> 07 -> 05 |
| Build analytical movement and transformation | 01 -> 06 -> 05 -> 14 |
| Publish code for other teams | 01 -> 08; add 09 or 10 only for an extension boundary |
| Target a special host | 01 -> choose 10, 11, or 12 -> 15 |
| Decide repository shape and release ownership | 01 -> 14 -> 15 |

Read [01](01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md)
before copying any tree. It explains which layers own policy and which remain
replaceable adapters.

## A Baseline Workspace Shape

Start with one package when it has one owner, one policy surface, and one
deployment unit. When domain policy, application orchestration, adapters, and
entrypoints need independently testable boundaries, this workspace is a useful
next shape:

```
acme/
|-- Cargo.toml                 # workspace membership and shared policy
|-- Cargo.lock                 # committed for applications
|-- crates/
|   |-- domain/                # rules and types; no transport authority
|   |-- application/           # use cases and ports
|   `-- adapters/              # database, network, clock, filesystem
|-- apps/
|   `-- entrypoint/            # CLI, server, worker, job, or device shell
|-- tests/
|   `-- scenarios/             # cross-crate behavioral evidence
`-- ops/
    |-- deploy/                # manifests or installer inputs
    `-- runbooks/              # observe, recover, roll back
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]

[workspace.package]
edition = "2024"

[workspace.lints.rust]
unsafe_code = "forbid"
```

Workspace package and lint values are opt-in inheritance. Participating member
manifests use fields such as `edition.workspace = true` and:

```toml
[lints]
workspace = true
```

The examples pair edition 2024 with dependency resolver 3. A repository pinned
to an older toolchain must select a resolver that toolchain supports and test its
feature/MSRV behavior. The exact edition and minimum supported Rust version are
repository policy, not blueprint law. A constrained target or FFI adapter may
need `unsafe`; in that case let only the owning crate decline the inherited
`forbid` policy and apply a documented local audit policy.

## Cross-Cutting Concerns Are Contracts

Cross-cutting concerns belong in the blueprint because they affect every failure
and rollback path.

| Concern | Neutral contract | Adapter-owned detail |
|---------|------------------|----------------------|
| Configuration | Typed settings, precedence, validation, reload policy | environment, file, registry, secret store |
| Time | `Clock` or explicit timestamps where determinism matters | system clock, simulated clock |
| Identity | Request/message/run correlation and idempotency key | header, broker metadata, scheduler run id |
| Observability | Event names, fields, severity, metric semantics | tracing/export backend |
| Shutdown | Stop admission, drain bounded work, persist checkpoint | signal, SCM control, host callback |
| Security | Capability and authorization decision points | TLS, token validation, OS/device credential |
| Data evolution | Versioned schema and compatibility window | migration engine, registry, file format |

Do not create a universal `common` crate that owns all of these. Prefer small
ports in the application layer and implementations in adapter crates. Shared
policy is valuable; shared accidental coupling is not.

## Operational Boundaries and Authority

```
 product / domain owner
          | owns invariants and acceptance semantics
          v
 application owner
          | owns use cases, error taxonomy, compatibility
          v
 entrypoint owner
          | owns admission, concurrency, shutdown, process result
          v
 platform operator
          | owns deployment, credentials, routing, retention
          v
 external authority
          `-- broker / database / OS / host / hardware
```

| Boundary | Must be explicit before implementation |
|----------|----------------------------------------|
| Admission | What may start new work, and when is it refused? |
| Completion | What observable fact means the work is complete? |
| Retry | Who retries, with what identity, budget, and backoff? |
| Durability | Which state survives process or device loss? |
| Cancellation | Which work can stop, and what partial effects remain? |
| Authority | Which component may mutate each record, schema, or external resource? |

A boundary also needs an exit: who can disable admission, export or transfer
owned state, revoke credentials/capabilities, remove deployed artifacts, and
prove no supported caller still depends on it. Retirement is an architecture
operation, not repository cleanup.

Two components that can both "fix" the same durable record do not share
responsibility; they share an unresolved authority conflict.

## Testing and Rollback Are Blueprint Inputs

The minimum evidence stack is layered:

```
pure policy tests
      |
port contract tests
      |
adapter integration tests
      |
entrypoint process tests
      |
deployment smoke + recovery exercise
```

| Change | Evidence | Rollback implication |
|--------|----------|----------------------|
| Pure policy | deterministic unit/property tests | code rollback is usually sufficient |
| Adapter | contract plus real dependency test | dependency configuration may also revert |
| Wire/schema | old/new compatibility matrix | expand before contract; keep old readers |
| Durable side effect | replay/reconciliation test | code rollback may not reverse data |
| Deployment topology | smoke, drain, failover exercise | preserve previous artifact and routing path |

`cargo test --workspace --all-targets` is a useful baseline, not proof of
operability. Each guide names the external authority that needs integration or
hardware evidence.

## Universal Bridge First

The durable bridge is **ports-and-adapters plus explicit operational semantics**:
functional core/imperative shell, hexagonal architecture, and actor/message
boundaries all separate decision logic from effect ownership. Rust strengthens
that separation with crate visibility, types, ownership, and `Send`/`Sync`, but
the architectural idea is language-independent.

As supplemental context, a .NET solution with domain/application projects,
hosted executables, DI registrations, and adapter projects maps naturally to a
Cargo workspace. Cargo features are not MSBuild configurations, however:
features are additive within a resolution graph and should not encode mutually
exclusive deployment environments.

## Decision Cheat Sheet

| Need | Start with | Add when required |
|------|------------|-------------------|
| Human-invoked automation | CLI [02] | library [08] for reusable core |
| Synchronous network contract | HTTP/API [03] | distributed [13] only for independent ownership/deployment |
| Durable background delivery | worker [04] | event-driven [07] for producer/consumer schema ownership |
| Clock-driven finite run | scheduled/batch [05] | ETL [06] for dataset publication semantics |
| Reusable consumer-facing code | library/SDK [08] | plugin [09] for third-party behavior |
| Portable sandboxed component | Wasm [10] | plugin [09] when host lifecycle dominates |
| Hardware-constrained control | embedded [11] | edge messaging through [07] where connectivity permits |
| Windows lifecycle or native UI | Windows [12] | keep policy in neutral crates |
| Many packages or release trains | monorepo [14] | selection/evolution [15] for split/merge gates |

## Common Confusion Points

- **A blueprint is not a framework recommendation.** Runtime and web libraries
  occupy adapter or entrypoint slots; they do not redefine domain authority.
- **One workspace does not mean one deployable.** A workspace is a Cargo
  coordination boundary. Deployment and rollback units must still be named.
- **Async is not an architecture.** It is an execution technique. Admission,
  completion, retry, and durability decide whether a service, worker, or event
  blueprint applies.
- **"Microservice ready" is not an exit criterion.** Independent data authority,
  deployment need, failure isolation, and team ownership are.
- **A rollback is not always a binary downgrade.** Messages, migrations, files,
  device state, and external effects may require forward repair or
  reconciliation.
- **Shared types do not create shared authority.** They can improve
  compatibility while still allowing exactly one owner for each mutation.

## Primary Sources

- Cargo Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Cargo Features: https://doc.rust-lang.org/cargo/reference/features.html
- Cargo dependency resolver: https://doc.rust-lang.org/cargo/reference/resolver.html
- Cargo Profiles: https://doc.rust-lang.org/cargo/reference/profiles.html
- The Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- The Rust Reference: https://doc.rust-lang.org/reference/
- Rust standard library documentation: https://doc.rust-lang.org/std/

## Related Guides

- Contract anatomy: [01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md](01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md)
- Selection and evolution: [15-BLUEPRINT-SELECTION-AND-EVOLUTION.md](15-BLUEPRINT-SELECTION-AND-EVOLUTION.md)
- Module status: [STATUS.md](STATUS.md)
