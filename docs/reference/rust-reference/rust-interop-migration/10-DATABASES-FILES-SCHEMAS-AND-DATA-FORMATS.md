---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:databases-files-schemas-data-formats
kind: guide
module: rust-interop-migration
section: computing-software
title: Databases, Files, Schemas, and Data Formats
status: source-custody
source_custody: partial
current_path: rust-interop-migration/10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md
canonical_path: rust-interop-migration/10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md
backsource_ids: [proof-backfill:rust-interop-migration:10-databases-files-schemas-data-formats]
concepts: [database migration, schema evolution, file format, serialization, protobuf, Arrow, Parquet, expand contract, data compatibility]
root_concepts: [data boundary]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Databases, Files, Schemas, and Data Formats

Data is often the most durable interop boundary and the hardest rollback
constraint. A Rust replacement can be reverted only while old and new binaries
can both read the state being produced. Design the compatibility window before
the new writer ships.

## The Big Picture

```
+============================================================================+
|                        DATA BOUNDARY MIGRATION                             |
+============================================================================+
|  PRODUCERS                                                                 |
|  old binary ------------------+                                            |
|                               +--> VERSIONED CONTRACT --> storage/stream   |
|  Rust binary -----------------+       schema + semantics                   |
|                                            |                               |
|                                            v                               |
|  CONSUMERS                                                                 |
|  old binary <----------------- compatible representation                   |
|  Rust binary <---------------- compatible representation                   |
|                                                                            |
|  EXPAND                 MIGRATE/BACKFILL               CONTRACT            |
|  additive schema  --->  dual-read/write/verify  --->  remove old form      |
|                                                                            |
|  rollback is valid only while old readers understand new writes            |
+============================================================================+
```

## Choose the Data Contract

| Boundary | Strength | Migration risk |
|----------|----------|----------------|
| Relational schema | Shared query semantics, constraints, transactions | Tight coupling through writes and migrations |
| Event/protobuf schema | Decoupled producers/consumers, replay | Ordering, idempotency, compatibility discipline |
| JSON/YAML | Human/debug friendly, broad ecosystem | Weak numeric/unknown-field conventions |
| CSV | Simple interchange | Ambiguous types, quoting, locale, schema drift |
| Arrow | In-memory columnar interchange | Buffer lifetime, alignment, version/library compatibility |
| Parquet | Analytical columnar file | Schema evolution, logical types, row-group/codec support |
| Custom binary | Exact performance/size control | Highest tooling and evolution burden |

Do not persist `bincode` or another Rust-implementation-shaped format as a
long-lived cross-language contract without a separately specified schema and
compatibility policy. Rust enum/layout evolution and serializer configuration
can otherwise become accidental storage ABI.

## Expand, Migrate, Contract

```
  phase 1 EXPAND
    add nullable/new column, new event field, or new file version
    old readers still work

  phase 2 MIGRATE
    new writer emits compatible form
    dual-read and compare
    backfill with checkpoints and restartability

  phase 3 CONTRACT
    prove old readers/writers are gone
    remove old field/path in a later release
```

Never combine a destructive schema change and the only compatible application
deployment in one irreversible step. Use compatibility across at least one
rollback window and rehearse downgrade behavior.

## A Versioned Serde Envelope

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Envelope {
    version: u32,
    request_id: String,
    #[serde(default)]
    trace_id: Option<String>,
    payload: PayloadV1,
}

#[derive(Debug, Serialize, Deserialize)]
struct PayloadV1 {
    records: Vec<RecordV1>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RecordV1 {
    id: u64,
    value: String,
}
```

The version field does not create compatibility by itself. Define unknown-field,
missing-field, numeric range, ordering, duplicate-key, Unicode, and canonical
encoding behavior. For signed or hashed data, canonicalization is part of the
security contract.

## Database Ownership

Two implementations writing the same tables need an explicit authority model:

| Model | Use |
|-------|-----|
| Single writer, multiple readers | Safest migration; route writes to one authority |
| Dual write by application | High divergence risk; needs idempotency and reconciliation |
| Transactional outbox/change feed | Publish changes without a distributed transaction |
| Shadow writer | Write to isolated target and compare; not production authority |
| Backfill job | Restartable, monotonic checkpoints, rate-limited, audited |

Do not put a Rust migration behind a database trigger without including trigger
deployment, ordering, failure, and rollback in the service contract. Hidden
writers are still writers.

## Files and Atomicity

For whole-file replacement, write a new file, flush as required by the durability
contract, and atomically rename/replace where the filesystem supports the needed
semantics. Cross-filesystem moves are not atomic. Windows sharing flags, Unix
rename behavior, network filesystems, antivirus/indexers, and readers holding
open handles can change observed behavior.

Version file headers with magic, format version, byte order, and integrity
metadata. Reject unsupported versions before interpreting payload offsets.

## Boundary Hazard Register

| Hazard | Data boundary rule |
|--------|--------------------|
| ABI | Persist a specified schema/format, never Rust memory layout, `repr(Rust)`, or trait-object representation. |
| Allocator | Mapped/shared buffers retain an owner; Arrow/FFI buffers need release callbacks and alignment contracts. |
| Panic/unwind | Parse failures are data errors; contain Rust panic and never corrupt/partially publish authoritative state. |
| Lifetime | Readers must not outlive mapped files, transaction snapshots, borrowed DB rows, or shared buffers. |
| Threading | Define connection/session ownership, transaction isolation, writer concurrency, and stream ordering. |
| Target | State endianness, integer widths, path/encoding/filesystem behavior, DB/server versions, and codec support. |
| Packaging | Ship migrations, schema descriptors, codecs, seed/config data, rollback scripts, and compatibility tooling with the binary. |

## Old World -> New World Bridge

| Established practice | Rust migration use |
|----------------------|--------------------|
| Database expand/contract | Preserve mixed-version operation through rollback window |
| ETL reconciliation | Shadow Rust output and compare normalized records |
| IDL/schema registry | Canonical contract independent of generated Rust/host types |
| ADO.NET transaction ownership | Rust connection/transaction scope must preserve the same isolation/commit semantics |
| Memory-mapped file | Borrowed buffer whose lifetime is owned by mapping object |
| Data warehouse backfill | Restartable checkpoints, rate limits, and evidence of completeness |

## Common Confusion Points

- **"Serde derives define the schema."** They define one implementation's
  mapping. The external contract needs semantic and compatibility rules.
- **"Additive fields are always compatible."** Readers may reject unknown
  fields, signatures may cover canonical bytes, and business logic may treat
  absence differently.
- **"Dual write is safer."** It creates two commits without atomicity unless a
  specific mechanism closes the gap.
- **"Parquet/Arrow means zero-copy everywhere."** Buffers, alignment, codecs,
  endian handling, and lifetime can force conversion.
- **"Rollback the binary and the data rolls back."** State is persistent; new
  writes can make an old binary unusable.
- **"Database types map one-to-one."** Decimal precision, time zones, null,
  collation, UUID, and large integer semantics require explicit mapping.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Mixed old/new binaries | Additive schema plus expand/migrate/contract |
| High-confidence replacement | Shadow reads/writes and semantic diff |
| Decoupled migration | Versioned event/protocol schema |
| Analytical interchange | Arrow in-memory, Parquet at rest, with explicit version/ownership |
| Simple public interchange | JSON with a written schema and compatibility rules |
| Whole-file state | Versioned header plus atomic replace protocol |
| Rollback after new writes | Prove old reader compatibility or supply reverse migration |

## Primary Sources

- Protocol Buffers updating guidance: https://protobuf.dev/programming-guides/proto3/#updating
- Apache Arrow format: https://arrow.apache.org/docs/format/Columnar.html
- Apache Parquet format: https://parquet.apache.org/docs/file-format/
- Serde attributes: https://serde.rs/attributes.html
- PostgreSQL transactional DDL reference: https://www.postgresql.org/docs/current/ddl.html

## Related Guides

- Previous: [09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md](09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md)
- Next: [11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md](11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md)
- Rollback and exit: [15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md](15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md)
