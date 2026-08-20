---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:event-driven-and-messaging-application
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Event-Driven and Messaging Application Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md
canonical_path: rust-application-blueprints/07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md
backsource_ids: [proof-backfill:rust-application-blueprints:07-event-driven-and-messaging-application]
concepts: [event-driven architecture, command, event, outbox, inbox, schema evolution, projection, saga]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Event-Driven and Messaging Application Blueprint

## The Big Picture

```
+============================================================================+
| command/request -> authoritative application -> state change               |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| transaction: domain state + outbox event intent                            |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| publisher -> broker/log -> consumer inbox -> handler                       |
+----------------------+----------------------+------------------------------+
                       v                      v
                  projection             downstream command/effect
                       |                      |
                       +-----------+----------+
                                   v
                   observable state + replay/recovery evidence
```

An event is a statement by an authority that something happened. A command asks
an authority to do something. Treating both as anonymous messages erases
ownership, failure semantics, and compatibility.

## Workspace Layout

```
orders/
|-- Cargo.toml
|-- crates/
|   |-- order-domain/
|   |-- order-application/
|   |-- order-events/           # owned public event representations
|   |-- outbox-store/
|   |-- broker-publisher/
|   |-- projection-orders/
|   `-- consumer-inbox/
|-- apps/
|   |-- order-command-service/
|   |-- order-event-publisher/
|   `-- order-projection-worker/
`-- tests/
    `-- messaging-contract/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]
```

Shared event crates can improve compilation-time agreement inside one release
cohort, but wire compatibility must also be tested with serialized fixtures.
Consumers outside the workspace do not receive a Rust type update atomically.

## Message Semantics and Authority

| Message kind | Naming | Authority |
|--------------|--------|-----------|
| Command | imperative: `ReserveInventory` | receiving capability owner |
| Event | past tense: `InventoryReserved` | state owner that observed commit |
| Query | request for current view | view owner |
| Integration event | stable external fact | publishing bounded-context owner |

Each envelope should define:

- unique message id;
- event/command type and schema version;
- occurred/recorded time semantics;
- producer identity;
- correlation and causation;
- partition/order key where promised;
- payload and content encoding.

Do not put retry count into the semantic payload unless it is business data.
Delivery attempts are broker/consumer metadata.

Authenticated producer identity is evidence, not semantic authority by itself.
Authorize which producer may assert each event type and partition/key scope.
Minimize secrets and regulated data in retained logs: encryption and ACLs do not
erase replay, replication, export, and deletion obligations. Bound payload,
nested structure, decompression, and consumer resource use before handling.

## Atomic Publication and Consumption

The outbox closes the state-change/publication crash gap:

```
begin transaction
   |-- update authoritative state
   `-- insert event intent in outbox
commit
   |
publisher reads outbox -> publishes -> marks publication progress
```

The inbox closes the duplicate-consumption gap for local durable effects:

```
receive event E
   |
begin transaction
   |-- if E complete: no-op
   |-- else apply projection/effect
   `-- record E complete
commit -> acknowledge
```

Outbox publication itself can duplicate after a crash; downstream consumers must
still tolerate redelivery.

## Ordering, Projections, and Coordination

| Need | Mechanism |
|------|-----------|
| Per-entity order | stable partition key plus producer sequence/revision |
| Rebuildable view | projection from retained events or source snapshots |
| Cross-service process | explicit process manager/saga state |
| Request/reply | correlation, timeout, duplicate and late-reply policy |
| Schema change | tolerant readers, additive evolution, versioned event when meaning changes |

```
event stream ---> projection A (search)
             +--> projection B (reporting)
             `--> process manager ---> command to another authority
```

A process manager owns coordination state, not the participant services' data.
Compensation is a new business action; it is not a distributed transaction
rollback.

## Testing and Rollback

Evidence matrix:

| Test | Required observation |
|------|----------------------|
| Serialized fixture | old/new readers interpret supported versions |
| Duplicate event | projection/effect remains correct |
| Reordered events | rejected, buffered, or reconciled per contract |
| Publisher crash | outbox eventually publishes without lost intent |
| Consumer crash | inbox/effect remains safe |
| Replay | rebuilt state matches reconciliation target |

```text
cargo test --workspace --all-targets
# integration: commit state/outbox, stop publisher at boundary, restart,
# publish twice, and assert consumer's durable result once
```

Rollback order matters:

1. deploy tolerant consumers;
2. deploy producers that emit the new compatible shape;
3. retain old interpretation through the rollback horizon;
4. remove old fields/types only after retained messages and replays no longer
   require them.

If event meaning changes, publish a new event type rather than reinterpreting
history. Replays must remain historically honest.

Retire an event only after producers stop emitting it, retained messages and
replay sources have crossed the compatibility horizon, supported consumers no
longer depend on it, projections are reconciled, subscriptions/routes are
removed, and schema/credential ownership has an explicit successor or closure.

## Universal Bridge First

The bridge is distributed logs plus state machines: facts form an append-only
coordination surface, while each consumer derives local state under explicit
ordering and duplication assumptions. Database logs, actor mailboxes, and
event-sourced systems are related but not identical.

Supplementally, .NET event buses and mediator libraries occupy adapter or
in-process dispatch roles. A mediator call inside one process is not a durable
event architecture without an external log, ownership, and replay contract.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| One durable background task | worker [04], not a domain event network |
| Publish state change reliably | transactional outbox |
| Idempotent local projection | inbox record with effect transaction |
| Many independent reactions | integration event with owned schema |
| Cross-service workflow | process manager with explicit state/timeouts |
| Complete audit/rebuild requirement | retained event log plus snapshots and replay tests |
| Simple synchronous dependency | HTTP/API [03] may be clearer |

## Common Confusion Points

- **Events are not remote method calls.** Publishers do not own consumer
  completion or response time.
- **A broker does not create exactly-once business effects.** Transaction and
  idempotency boundaries determine that claim.
- **Event sourcing is not required for event-driven integration.** An outbox can
  publish facts from ordinary authoritative state.
- **Shared schema code is not a deployment guarantee.** Serialized compatibility
  must survive version skew.
- **Compensation is not erasure.** It records a new fact that counteracts an
  earlier outcome where business rules permit.
- **Replay is production behavior.** It needs capacity limits, side-effect
  controls, version handling, and reconciliation.
- **An append-only log conflicts with casual deletion promises.** Data
  classification and retention/removal design must precede publication.

## Primary Sources

- CloudEvents specification: https://cloudevents.io/
- AsyncAPI specification: https://www.asyncapi.com/docs/reference/specification/latest
- Rust async book: https://rust-lang.github.io/async-book/
- Cargo Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/

## Related Guides

- Queue consumers: [04-ASYNC-WORKER-AND-QUEUE-CONSUMER.md](04-ASYNC-WORKER-AND-QUEUE-CONSUMER.md)
- Distributed applications: [13-MULTI-SERVICE-DISTRIBUTED-APPLICATION.md](13-MULTI-SERVICE-DISTRIBUTED-APPLICATION.md)
