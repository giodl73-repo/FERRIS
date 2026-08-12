---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:blueprint-contract-anatomy
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Blueprint Contract Anatomy and Cross-Cutting Concerns
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md
canonical_path: rust-application-blueprints/01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md
backsource_ids: [mdloom-backfill:rust-application-blueprints:01-blueprint-contract-anatomy]
concepts: [application contract, ports and adapters, operational semantics, configuration, observability, shutdown, rollback]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Blueprint Contract Anatomy and Cross-Cutting Concerns

## The Big Picture

```
+============================================================================+
| INITIATOR CONTRACT                                                         |
| args / request / delivery / schedule / callback / interrupt                |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| ENTRYPOINT SHELL                                                           |
| parse | authenticate | admit | correlate | bound concurrency | stop/drain  |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| APPLICATION CONTRACT                                                       |
| use cases | domain invariants | ports | typed errors | completion meaning  |
+---------------------+-----------------------------+------------------------+
                      v                             v
+-----------------------------------+  +-------------------------------------+
| EFFECT ADAPTERS                   |  | CROSS-CUTTING POLICY                |
| store | network | clock | broker  |  | config | telemetry | security       |
| filesystem | OS | hardware        |  | idempotency | schema | recovery     |
+---------------------+-------------+  +------------------+------------------+
                      +-------------------------------+
                                                      v
                                      observable result + durable effects
```

The contract is the smallest set of decisions that must remain stable while
implementations change. It includes more than Rust traits: process semantics,
data ownership, compatibility windows, operational evidence, and rollback
limits are equally real interfaces.

## The Five Contract Layers

| Layer | Owns | Must not silently own |
|-------|------|-----------------------|
| Initiator | invocation identity and input envelope | domain mutation policy |
| Entrypoint | admission, decoding, lifecycle, process result | durable truth |
| Application | use-case ordering and domain invariants | framework types |
| Port | capability needed from an external authority | vendor configuration |
| Adapter | protocol, client, serialization, retry mechanics | business meaning |

An application port should be phrased in domain capability:

```rust
pub trait ReservationStore {
    fn load(&self, id: ReservationId) -> Result<Option<Reservation>, StoreError>;
    fn commit(
        &self,
        expected: Revision,
        value: &Reservation,
    ) -> Result<Revision, StoreError>;
}
```

The port exposes optimistic concurrency because the use case must reason about
conflict. It does not expose a database connection pool because pool ownership
belongs to an adapter and process lifecycle.

## Concrete Workspace Boundary

```
reservation-system/
|-- Cargo.toml
|-- crates/
|   |-- reservation-domain/
|   |   `-- src/lib.rs
|   |-- reservation-application/
|   |   `-- src/{lib.rs,ports.rs,use_cases.rs}
|   |-- adapter-postgres/
|   |-- adapter-system-clock/
|   `-- telemetry-contract/
|-- apps/
|   `-- reservation-entry/
|       `-- src/main.rs
`-- tests/
    `-- contract-scenarios/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]

[workspace.dependencies]
reservation-domain = { path = "crates/reservation-domain" }
reservation-application = { path = "crates/reservation-application" }
```

Keep dependency arrows inward:

```
entrypoint ---> application ---> domain
     |
     +--------> adapter --------> external system
                     |
                     `----------> application port
```

The domain must not depend on the adapter. Whether the adapter implements a
trait defined by the application or uses explicit generic parameters is a local
choice; the ownership direction is not.

## Cross-Cutting Concerns as Explicit Policy

| Concern | Contract question | Failure if implicit |
|---------|-------------------|---------------------|
| Config | precedence, validation, reload, secret redaction? | different instances run different policy |
| Errors | which failures are reject, retry, conflict, or defect? | entrypoints invent inconsistent behavior |
| Time | wall clock or monotonic duration; who supplies it? | tests flake and expiry rules drift |
| Identity | invocation, causation, correlation, idempotency? | retries duplicate effects |
| Telemetry | stable event names and required fields? | dashboards couple to prose logs |
| Shutdown | stop admission, cancel, drain, checkpoint, timeout? | deploy loses or duplicates work |
| Security | who authenticates and who authorizes? | adapters become policy authorities |
| Evolution | supported readers/writers and migration order? | rollback crosses an incompatible boundary |

Security is not one adapter row. Record trust boundaries, input and resource
limits, credential and capability owners, tenant/data separation,
dependency/supply-chain policy, and the audit evidence required for privileged
recovery. Authentication evidence can be adapter-produced; authorization
remains a semantic decision by the authority that owns the protected action.

```
load raw config
      |
      v
parse + validate ----invalid----> fail before admission
      |
      v
construct typed Settings
      |
      +----> entrypoint lifecycle
      +----> adapter constructors
      `----> redacted diagnostic summary
```

Do not pass an untyped environment map through the domain. Validate once at the
composition root and pass only settings each component owns.

## Completion, Retry, and Idempotency

Completion semantics are blueprint-specific but must be singular:

| Blueprint | Completion fact |
|-----------|-----------------|
| CLI | process exits after requested effect is durable or explicitly queued |
| HTTP | response contract is committed; later work is separately identified |
| Worker | delivery is acknowledged only after the chosen completion point |
| Batch/ETL | run ledger and published output agree |
| Library | returned value/error satisfies documented postconditions |
| Device | state transition is applied or safely rejected |

```
attempt(identity)
    |
    +--> no prior result --> execute --> record result --> return
    |
    `--> prior result ----> return same semantic result
```

Idempotency is not "the operation probably tolerates duplicates." It is a
contract over identity, effect, stored result, and retention window.

## Ownership and Authority Ledger

Write this table before choosing crates:

| Asset/decision | Single authority | Readers | Recovery authority |
|----------------|------------------|---------|--------------------|
| Domain invariant | domain/application owner | all entrypoints | application owner |
| API/message schema | producer or protocol owner | consumers | compatibility owner |
| Database record | named service/application | reports, projections | same owner or documented operator |
| Retry policy | layer owning semantic repeatability and remaining deadline | operators | entrypoint/application owner |
| Credentials | deployment platform | adapter | platform operator |
| Runbook | deployable owner | operators | incident commander |

If recovery authority differs from mutation authority, define the handoff and
audit trail. An operator bypass is still an interface.

## Testing and Rollback Contract

```
domain examples           -> invariant proof
application scenarios     -> port-level behavior
adapter contract suite    -> protocol fidelity
entrypoint process test   -> exit/lifecycle semantics
recovery exercise         -> rollback or forward repair
```

Recommended baseline:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

Add explicit feature-matrix jobs. `--all-features` is valid only if features are
designed to compose; mutually exclusive backends should be separate crates or
separately tested profiles.

Rollback must name the irreversible boundary:

| Change | Safe rollback condition |
|--------|-------------------------|
| Code only | previous artifact accepts current config and data |
| Config | previous and current parsers accept the reverted form |
| Database | old code works during expand/contract window |
| Message | old consumer accepts messages emitted during rollout |
| External effect | compensating or reconciliation path exists |

Retirement needs equivalent evidence: stop new admission, drain or reconcile
in-flight work, revoke credentials and callbacks, archive or export state under
its retention policy, remove routes/schedules/subscriptions, and prove that
supported callers no longer exercise the old contract.

## Universal Bridge First

This anatomy is the operational form of information hiding: stable semantic
interfaces surround volatile mechanisms. Hexagonal architecture, clean
architecture, functional core/imperative shell, and actor boundaries differ in
notation but share that pressure.

As supplemental .NET context, the composition root resembles a Generic Host
startup path and ports resemble application-facing interfaces. The important
difference is not language syntax: it is refusing to let DI registration,
configuration providers, or logging frameworks become the semantic contract.

## Decision Cheat Sheet

| Decision | Prefer | When |
|----------|--------|------|
| Separate crate | independent policy, compatibility, or test boundary | change must be reviewable without adapter details |
| Internal module | implementation decomposition | no independent ownership or versioning exists |
| Trait port | multiple implementations or test substitute at a real authority boundary | capability matters more than concrete client |
| Concrete dependency | one stable implementation, no substitution need | abstraction would only rename the client |
| Typed config | always at composition boundary | invalid combinations must fail before work |
| Idempotency record | retries can repeat durable effects | identity and retention can be defined |
| Forward repair | effects or schemas are irreversible | binary rollback cannot restore truth |

## Common Confusion Points

- **Traits are not automatically architecture.** A trait around every function
  adds indirection without creating an authority boundary.
- **"Cross-cutting" does not mean global singleton.** Context should be explicit
  and scoped; global mutable state obscures ownership and tests.
- **Retries belong to the layer that can prove repeatability.** Lower adapters
  may retry only when explicitly delegated; layered automatic retries multiply
  attempts and exceed deadlines.
- **Logs are not a telemetry contract.** Stable event identity and fields are;
  rendered prose is an operator view.
- **A successful enqueue is not completed business work.** Name the handoff
  explicitly in the caller's contract.
- **Rollback readiness is not migration reversibility.** Expand/contract and
  forward repair are often safer than destructive down migrations.

## Primary Sources

- Cargo Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Cargo Features: https://doc.rust-lang.org/cargo/reference/features.html
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- Rust error handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html
- Rust testing: https://doc.rust-lang.org/book/ch11-00-testing.html
- Rust standard library synchronization: https://doc.rust-lang.org/std/sync/

## Related Guides

- Landscape: [00-OVERVIEW.md](00-OVERVIEW.md)
- Selection and evolution: [15-BLUEPRINT-SELECTION-AND-EVOLUTION.md](15-BLUEPRINT-SELECTION-AND-EVOLUTION.md)
