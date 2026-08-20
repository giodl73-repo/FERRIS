---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:persistence-transactions-data-access
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Persistence, Transactions, and Data Access
status: source-custody
source_custody: partial
current_path: rust-production-engineering/08-PERSISTENCE-TRANSACTIONS-AND-DATA-ACCESS.md
canonical_path: rust-production-engineering/08-PERSISTENCE-TRANSACTIONS-AND-DATA-ACCESS.md
backsource_ids: [proof-backfill:rust-production-engineering:08-persistence-transactions-data-access]
concepts: [persistence, transactions, data access, connection pools, migrations, outbox, idempotency, consistency]
root_concepts: [persistence]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Persistence, Transactions, and Data Access

## The Big Picture

Persistence is where process-local correctness meets durable shared state.
Rust's ownership model helps control connection and transaction lifetimes, but
the database, object store, log, or queue defines the real durability,
isolation, and concurrency contract.

```
+============================================================================+
|                         DATA ACCESS PATH                                   |
|                                                                            |
| request --> validate --> acquire bounded client/connection                 |
|                              |                                             |
|                              v                                             |
|                   begin transaction / operation                            |
|                              |                                             |
|                     read --> decide --> write                              |
|                              |                                             |
|                 commit + outbox/dedup record                               |
|                      | success        | uncertain                          |
|                      v                v                                    |
|                 return outcome    reconcile by stable key                  |
|                                                                            |
| schema/migrations evolve compatibility around every deployed version       |
+============================================================================+
```

Start from invariants: uniqueness, referential integrity, state transitions,
ordering, and duplicate handling. Put invariants in the narrowest durable
authority that can enforce them atomically.

## Data-System Choices

| System | Strong fit | Operational cost |
|---|---|---|
| Relational database | multi-record invariants, transactions, rich queries | schema and connection management |
| Key-value/document store | key-oriented access and flexible records | application-managed joins/invariants |
| Object store | large immutable blobs | weak small-update semantics |
| Durable log/queue | ordered events and asynchronous work | replay, duplication, consumer state |
| Embedded database | single-process/local state | file lifecycle, locking, replication limits |

No abstraction erases the underlying consistency model. A generic repository
interface can hide the exact behavior needed to make a correct retry or
migration decision.

## Transactions and Isolation

| Concern | Question |
|---|---|
| Atomicity | Which writes must commit together? |
| Isolation | Which concurrent anomalies are acceptable? |
| Durability | What acknowledgement means data survives which failure? |
| Retry | Can the transaction body run again without duplicate side effects? |
| Uncertain commit | How is outcome discovered after connection loss? |

Keep transactions short in elapsed time and scope. Never hold one open while
waiting for arbitrary user input or a slow remote service. If a remote message
must follow a database write, use an outbox or another atomic handoff rather
than pretending two independent systems share one transaction.

## Executable SQLite Transaction

```toml
# Cargo.toml
[package]
name = "transaction-example"
version = "0.1.0"
edition = "2021"

[dependencies]
rusqlite = { version = "0.32", features = ["bundled"] }
```

```rust
use rusqlite::{params, Connection};

fn main() -> rusqlite::Result<()> {
    let mut db = Connection::open_in_memory()?;
    db.execute_batch(
        "PRAGMA foreign_keys = ON;
         CREATE TABLE account(
             id INTEGER PRIMARY KEY,
             balance INTEGER NOT NULL CHECK(balance >= 0)
         );
         INSERT INTO account(id, balance) VALUES (1, 100), (2, 0);",
    )?;

    let tx = db.transaction()?;
    let debited = tx.execute(
        "UPDATE account SET balance = balance - ?1
         WHERE id = ?2 AND balance >= ?1",
        params![40, 1],
    )?;
    if debited != 1 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }
    tx.execute(
        "UPDATE account SET balance = balance + ?1 WHERE id = ?2",
        params![40, 2],
    )?;
    tx.commit()?;

    let total: i64 = db.query_row("SELECT SUM(balance) FROM account", [], |row| row.get(0))?;
    assert_eq!(total, 100);
    println!("total={total}");
    Ok(())
}
```

Run `cargo generate-lockfile`, then `cargo run --locked`. Scope: an embedded
SQLite database and a single process. The `CHECK` constraint and conditional
debit place critical invariants in storage. A network database needs an explicit
isolation level, pool, timeout, and uncertain-commit policy.

## Pools Are Admission Controllers

Connection pools cap concurrent database work; they are not merely a
performance cache.

```
request concurrency: 200
database pool:         20
pool wait deadline:   100 ms

result: at most 20 active DB sessions; excess work waits briefly or fails
```

Size from database capacity and query behavior, not application replica count
alone. Fifty replicas each opening one hundred connections can overwhelm a
database that comfortably serves two hundred total. Record acquisition wait,
in-use count, query duration, cancellation, and timeout.

## Migrations and Compatibility

Use expand/migrate/contract sequencing:

1. Expand schema so old and new binaries both work.
2. Deploy code that writes/reads the new shape safely.
3. Backfill with bounded, observable work.
4. Verify.
5. Contract only after no deployed version needs the old shape.

Migration tools (`sqlx migrate`, Diesel migrations, refinery, Flyway, Liquibase)
are mechanisms. The compatibility window and rollback policy are architecture.
Avoid running heavyweight migrations independently from every application
replica at startup.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | SQLx, Diesel, rusqlite, native service SDK, migration tool |
| Runtime | async vs blocking driver, cancellation and pool integration |
| Platform | database service, backups, replication, encryption, failover |

SQLx commonly targets async runtimes; Diesel is primarily synchronous with
async integration options; rusqlite is local and blocking. Choose from required
semantics and operational constraints, not from an ORM label.

## Old World -> New World Bridge

The universal bridge is from **object lifetime** to **transaction lifetime**.
RAII can ensure a Rust transaction object rolls back when dropped, but it cannot
decide the business invariant or resolve an unknown remote commit.

ADO.NET connection pooling and `using` blocks are direct prior art. Azure SQL,
PostgreSQL, SQLite, and cloud-native stores differ in isolation and failure
behavior; keep those differences visible above the client crate.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Database constraint | invariant can be enforced atomically in storage |
| Optimistic concurrency | conflicts are rare and version checks are natural |
| Pessimistic lock | conflict is expected and lock scope is short/bounded |
| Transaction | multiple state changes form one durable invariant |
| Outbox | durable state and asynchronous publication must not diverge |
| Idempotency record | clients may repeat an operation after uncertain outcome |
| Bounded pool | shared data system needs explicit concurrency admission |
| Expand/contract migration | old and new versions overlap during rollout |

## Common Confusion Points

- **A dropped transaction may roll back locally, but a lost network response can
  leave commit outcome unknown.**
- **Pool timeout and query timeout are different budgets.**
- **ORM type safety does not prove isolation-level correctness.**
- **Retries can repeat transaction bodies and external side effects.**
- **Backup existence is not recovery evidence.** Restore time and consistency
  must be tested.

## Primary Sources

- SQLite transactions: https://www.sqlite.org/lang_transaction.html
- PostgreSQL transaction isolation: https://www.postgresql.org/docs/current/transaction-iso.html
- SQLx: https://docs.rs/sqlx/
- Diesel: https://diesel.rs/guides/
- rusqlite: https://docs.rs/rusqlite/

## Related Guides

- Previous: [07-TIMEOUTS-RETRIES-BACKPRESSURE-AND-RESILIENCE.md](07-TIMEOUTS-RETRIES-BACKPRESSURE-AND-RESILIENCE.md)
- Next: [09-PACKAGING-ARTIFACTS-RELEASES-AND-UPGRADES.md](09-PACKAGING-ARTIFACTS-RELEASES-AND-UPGRADES.md)
- Recovery testing: [12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md](12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md)
- Release gates: [15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md](15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md)
