---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:async-worker-and-queue-consumer
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Async Worker and Queue Consumer Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/04-ASYNC-WORKER-AND-QUEUE-CONSUMER.md
canonical_path: rust-application-blueprints/04-ASYNC-WORKER-AND-QUEUE-CONSUMER.md
backsource_ids: [proof-backfill:rust-application-blueprints:04-async-worker-and-queue-consumer]
concepts: [async worker, queue consumer, acknowledgement, idempotency, retry, dead letter, graceful drain]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Async Worker and Queue Consumer Blueprint

## The Big Picture

```
+============================================================================+
| broker authority: delivery identity | lease | order | redelivery           |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| consumer shell                                                             |
| receive -> decode -> classify -> admit -> establish attempt context        |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| application handler -> idempotency/inbox -> effects -> outcome             |
+----------------------+----------------------+------------------------------+
                       v                      v
                 durable store          external service
                       |                      |
                       +-----------+----------+
                                   v
                 ack | retry/release | quarantine/dead-letter
```

A worker is defined by **delivery completion semantics**, not by the presence of
an async runtime. The central decision is when the consumer may acknowledge
work relative to durable effects, and how redelivery is made safe.

## Workspace Layout

```
image-worker/
|-- Cargo.toml
|-- crates/
|   |-- image-domain/
|   |-- image-application/
|   |-- queue-contract/
|   |-- broker-adapter/
|   |-- image-store/
|   `-- worker-runtime/
|-- apps/
|   `-- image-worker/
`-- tests/
    |-- delivery-contract/
    `-- broker-integration/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]
```

Keep broker delivery objects in the consumer adapter. Convert them into an
application command carrying explicit message id, causation id, schema version,
attempt/deadline information, and payload.

## Delivery Semantics

| Strategy | Completion point | Cost |
|----------|------------------|------|
| Ack before work | broker forgets before effects | loss on crash; rarely acceptable |
| Ack after work | effects complete before ack | redelivery can duplicate effects |
| Transactional consume/effect | broker and state share transaction authority | strong but infrastructure-specific |
| Inbox/idempotency | record message identity with effects | storage and retention policy required |

```
delivery D
   |
   v
begin state transaction
   |
   +--> D already complete --> commit/no-op --> ack
   |
   `--> apply effects + record D complete
             |
             v
          commit --> ack
```

"Exactly once" should be bounded to a named effect under named infrastructure.
Most portable designs are at-least-once delivery plus idempotent or deduplicated
effects.

## Concurrency, Ordering, and Failure

| Concern | Explicit decision |
|---------|-------------------|
| Concurrency | global and per-key limits |
| Ordering | none, per partition/key, or total within a bounded stream |
| Lease | renewal ownership and maximum processing time |
| Retry | retryable taxonomy, delay, attempt/time budget |
| Poison input | quarantine destination and inspection/redrive authority |
| Backpressure | pause receives before memory or dependency saturation |
| Shutdown | stop receive, renew active leases if needed, drain, release safely |

```
receive loop ----> bounded channel ----> N handlers
     |                                      |
     +-- pause when full                    +-- report outcome
     |                                      |
shutdown: stop receive -> drain until deadline -> release unfinished
```

Avoid one spawned task per unbounded delivery. A semaphore, bounded channel, or
broker prefetch limit must connect admission to actual downstream capacity.

Bound envelope, decoded payload, decompression ratio, and per-tenant concurrency
before expensive allocation or work. Broker credentials should grant only the
required receive/acknowledge/dead-letter operations. Quarantine can contain
sensitive payloads; define encryption, access, retention, redaction, and
authorized redrive rather than treating it as an unrestricted debug queue.

The message contract owner defines payload compatibility. The worker owner
defines processing policy. The broker/platform owner defines retention and
delivery mechanics. The domain data owner defines mutation authority.

Lease renewal is not completion. Partition revocation, session loss, or an
expired lock can make the final acknowledgement fail after an effect committed;
the same idempotency and reconciliation contract must cover that path.

## Testing and Rollback

Test failures at every acknowledgement boundary:

| Test | Proves |
|------|--------|
| Same message twice | duplicate handling is semantically stable |
| Crash after effect/before ack | redelivery does not corrupt state |
| Dependency timeout | retry classification and deadline behavior |
| Poison payload | quarantine path preserves evidence |
| Shutdown under load | no new admission; unfinished work is safe |
| Old/new payloads | compatibility during producer/consumer skew |

```text
cargo test --workspace --all-targets
# repository-owned integration environment:
# publish fixture -> run worker -> assert effect -> force redelivery -> assert no duplicate
```

Rollback rules:

- keep the previous consumer able to read messages already retained;
- deploy tolerant readers before producers emit a new required shape;
- retain idempotency records for at least the redelivery/replay horizon;
- never redrive quarantine blindly after a code rollback;
- treat partially applied external effects as reconciliation work, not merely
  queue cleanup.

To retire a consumer, stop or transfer subscription ownership, stop new receives,
drain/release active deliveries, reconcile lag and quarantine, retain deduplication
state through the replay horizon, and revoke broker credentials only after no
rollback path needs them.

## Universal Bridge First

The universal bridge is a distributed state machine: delivery, effect, and
acknowledgement are separate transitions with crash gaps between them. Database
transactions collapse some gaps; idempotency records make others repeatable.

Supplementally, this resembles a .NET background service consuming Service Bus
or another broker. The Rust-specific choices concern executor, task ownership,
and type boundaries; the correctness problem remains acknowledgement versus
durability.

## Decision Cheat Sheet

| Situation | Choose |
|-----------|--------|
| Fast idempotent operation | ack after effect, bounded retries |
| Durable local mutation | inbox/idempotency in same store transaction |
| Non-idempotent remote effect | remote idempotency key or explicit reconciliation |
| Long work beyond lease | lease renewal, chunk/checkpoint, or scheduled job [05] |
| Ordered key stream | partition by key and serialize per key |
| High fan-out domain facts | event-driven blueprint [07] |
| Request needs later work | HTTP [03] returns durable job identity |

## Common Confusion Points

- **Async is not parallelism or durability.** It changes waiting mechanics, not
  delivery guarantees.
- **Dead-letter is not error handling.** It is retained evidence requiring an
  owner, diagnosis, and controlled redrive.
- **A retry count alone is not a budget.** Include elapsed time, deadline, and
  nested dependency attempts.
- **Ordering is never free.** It trades throughput and availability for a
  stronger stream contract.
- **Acknowledgement and commit are different authorities.** A crash can occur
  between them unless infrastructure explicitly unifies them.
- **A renewed lease can still be lost.** Revocation or session failure can race
  completion, so handlers cannot use lease ownership as a uniqueness proof.
- **Cancellation can create partial effects.** Handlers need defined safe
  points, not arbitrary task abortion.

## Primary Sources

- Rust async book: https://rust-lang.github.io/async-book/
- Rust `Future`: https://doc.rust-lang.org/std/future/trait.Future.html
- Rust synchronization primitives: https://doc.rust-lang.org/std/sync/
- CloudEvents specification: https://cloudevents.io/
- Cargo Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html

## Related Guides

- Event contracts: [07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md](07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md)
- Scheduled work: [05-SCHEDULED-AND-BATCH-JOB.md](05-SCHEDULED-AND-BATCH-JOB.md)
