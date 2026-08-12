---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:data-pipeline-and-etl
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Data Pipeline and ETL Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/06-DATA-PIPELINE-AND-ETL.md
canonical_path: rust-application-blueprints/06-DATA-PIPELINE-AND-ETL.md
backsource_ids: [mdloom-backfill:rust-application-blueprints:06-data-pipeline-and-etl]
concepts: [data pipeline, etl, schema contract, watermark, lineage, reconciliation, dataset publication]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Data Pipeline and ETL Blueprint

## The Big Picture

```
+============================================================================+
| source authorities: database | files | object store | API | event log      |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| EXTRACT: snapshot/version -> decode -> preserve source identity            |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| TRANSFORM: normalize -> validate -> enrich -> partition -> reconcile       |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| LOAD/PUBLISH: stage -> verify -> manifest/swap -> expose                   |
+----------------------+----------------------+------------------------------+
                       v                      v
                destination data        lineage + run ledger
```

A pipeline is a custody chain for data. The blueprint must say which source
version was read, which transformations were applied, which records were
rejected, when output became visible, and how a reader can distinguish one
published dataset from another.

## Workspace Layout

```
customer-export/
|-- Cargo.toml
|-- crates/
|   |-- export-contract/        # canonical records and schema versions
|   |-- extract-source/
|   |-- transform-customer/
|   |-- load-destination/
|   |-- quality-rules/
|   `-- pipeline-application/
|-- apps/
|   `-- customer-export-job/
|-- schemas/
|-- fixtures/
`-- tests/
    |-- golden-records/
    `-- reconciliation/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]
```

Separate canonical data meaning from encoding adapters. A CSV row, database row,
and wire object can all map to the same canonical record without making any one
encoding authoritative.

## Extraction and Source Identity

| Source | Identity required for repeatability |
|--------|-------------------------------------|
| Database | snapshot/transaction boundary or monotonic change token |
| Object/file store | object version, checksum, and path/key |
| API | page/cursor semantics plus upstream version where available |
| Event log | topic/stream and partition offsets |
| Mutable directory | enumerated manifest captured before processing |

```
discover inputs
      |
      v
freeze manifest M
      |
      v
read only members/version named by M
      |
      v
record checksum/count/range in run ledger
```

A timestamp is not automatically a watermark. A watermark is a contract about
completeness relative to an ordering domain; late data policy must say when that
claim can be revised.

## Transform, Quality, and Rejection

| Rule class | Example outcome |
|------------|-----------------|
| Structural | reject malformed encoding with source position |
| Semantic | reject or quarantine invalid domain value |
| Referential | defer, join against versioned reference, or reject |
| Deduplication | select by stable business/source key and ordering rule |
| Enrichment | record reference dataset/version |
| Privacy/security | minimize, tokenize, redact, or deny export |

```
decoded record
      |
      +--> valid ------> canonical transform ------> accepted partition
      |
      `--> invalid ----> rejection record
                         {source id, rule id, safe detail, run id}
```

The pipeline owner owns transformation semantics. Source owners own source
meaning and availability. Destination owners own publication constraints.
Security/data-governance owners define permitted movement and retention.

Treat source manifests, schemas, and data as untrusted inputs even when they
come from an internal store. Bound row/record size, nesting, decompression,
partition count, and temporary storage. Separate source and destination
credentials, minimize exported fields, protect rejection samples, and ensure
lineage does not become a second copy of regulated payloads.

## Loading and Publication

Prefer staged publication:

```
write versioned staging location
       |
       v
validate counts + invariants + checksums
       |
       v
write immutable manifest
       |
       v
atomically update reader-visible pointer/catalog
```

| Destination | Publication boundary |
|-------------|----------------------|
| Relational table | transaction, partition swap, or version column |
| Files/object store | immutable prefix plus manifest/current pointer |
| Search/index | new index then alias swap |
| API-owned system | idempotent upsert with reconciliation |
| Stream | output offsets plus schema and replay contract |

Avoid exposing a directory while files are still arriving unless readers have a
manifest that excludes incomplete content.

## Testing, Reconciliation, and Rollback

Evidence should include:

- golden input/output records for transformation intent;
- property tests for normalization and invariants where useful;
- schema compatibility tests for old and new encodings;
- duplicate, missing, reordered, and late inputs;
- count/sum/hash reconciliation against the source manifest;
- restart after a staged partition but before publication.

```text
cargo test --workspace --all-targets
cargo run -p customer-export-job -- --input-manifest fixtures/run-001.json --run-id test-001
```

| Failure/change | Recovery |
|----------------|----------|
| Transform defect before publish | discard staging and rerun |
| Defect after versioned publish | point readers to prior manifest; issue corrected version |
| In-place destructive load | restore/repair from authoritative source and audit trail |
| Schema change | dual-read/dual-write or versioned dataset during window |
| Late data | correction run with explicit supersession |

Rollback should switch visibility, not mutate history. Immutable source/output
versions make comparison and repair possible.

Removal requires more than deleting code: stop discovery and publication,
identify downstream readers of the active manifest, freeze or transfer dataset
authority, apply retention/deletion policy to staging, rejects, lineage, and
published versions, then revoke credentials after rollback is no longer
required.

## Universal Bridge First

The universal bridge is compiler construction and relational query execution:
decode into an intermediate representation, apply explicit passes, validate
invariants, and emit an artifact with provenance. ETL differs mainly because
inputs can be incomplete and authorities are distributed.

Supplementally, this maps to Azure Data Factory or SSIS stages, but orchestration
metadata is not a substitute for a repository-owned schema, run identity,
lineage record, and reconciliation rule.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| Small finite transform | scheduled/batch [05] plus canonical contract crate |
| Large restartable dataset | manifest, partitions, checkpoint ledger |
| Continuous ordered changes | event-driven [07] with projection semantics |
| Reader needs atomic dataset | immutable version plus manifest/pointer swap |
| Late data expected | event-time watermark and correction policy |
| Multiple encodings | canonical model plus source/destination adapters |
| Regulated movement | explicit field policy, lineage, rejection retention |

## Common Confusion Points

- **ETL and ELT are placement choices, not governance models.** Custody,
  semantics, and publication still need owners.
- **Row count equality is weak reconciliation.** Add keys, ranges, aggregates,
  checksums, or domain invariants as appropriate.
- **A schema registry does not define business meaning.** It records encoding
  contracts; ownership and interpretation remain social and operational.
- **Watermarks do not eliminate late data.** They define when a system acts as
  though a prefix is complete.
- **Rejecting records silently corrupts totals.** Rejections need identity,
  safe diagnostics, metrics, and disposition.
- **In-place overwrite destroys rollback evidence.** Prefer versioned
  publication where storage and readers allow it.
- **Lineage can leak data.** Record identifiers, checksums, rule versions, and
  safe diagnostics without copying protected payloads into operational stores.

## Primary Sources

- Cargo Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Rust Serde project documentation: https://serde.rs/
- Apache Arrow format: https://arrow.apache.org/docs/format/Columnar.html
- Apache Parquet format: https://parquet.apache.org/docs/file-format/
- Rust `Iterator`: https://doc.rust-lang.org/std/iter/trait.Iterator.html

## Related Guides

- Scheduled execution: [05-SCHEDULED-AND-BATCH-JOB.md](05-SCHEDULED-AND-BATCH-JOB.md)
- Event streams: [07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md](07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md)
