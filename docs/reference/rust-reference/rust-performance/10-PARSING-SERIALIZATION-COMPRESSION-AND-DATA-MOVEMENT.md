---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:parsing-serialization-compression-and-data-movement
kind: guide
module: rust-performance
section: rust-performance
title: Parsing, Serialization, Compression, and Data Movement
status: source-custody
source_custody: partial
current_path: rust-performance/10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md
canonical_path: rust-performance/10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md
backsource_ids: [proof-backfill:rust-performance:10-parsing-serialization-compression-and-data-movement]
concepts: [parsing, serialization, compression, data movement, zero copy, serde, buffering]
root_concepts: [data movement]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Parsing, Serialization, Compression, and Data Movement

## The Big Picture

Data pipelines pay for every boundary: read, validate, decode, allocate, copy,
transform, encode, compress, and write. "Zero copy" is meaningful only when the
ownership and lifetime graph proves which copies disappeared.

```
INPUT BYTES
    |
    +-> framing -> validation -> parse/decode -> domain transform
    |      |            |             |                 |
    |      |            |             +-> borrow/own    +-> copy/allocate
    |      |            +-> UTF-8 / schema / limits
    |      +-> incremental state over reusable read buffers
    |
    +-> result -> encode/serialize -> compress -> frame -> OUTPUT BYTES

cost = scanning + branches + allocation + copies + compression + I/O
```

## Establish the Format Contract

Performance comparisons are invalid if formats provide different semantics.

| Dimension | Questions |
|-----------|-----------|
| Schema | Self-describing or predefined? Evolution rules? Unknown fields? |
| Validation | UTF-8, checksums, ranges, canonicalization, duplicate keys? |
| Representation | Text, binary, varint, fixed width, endianness? |
| Ownership | Borrowed views permitted? Must values outlive input buffer? |
| Security | Maximum depth/length, decompression ratio, malformed input behavior? |
| Operations | Full decode, selective fields, random access, streaming? |

JSON-to-struct and fixed-schema binary-to-borrowed-view are not equivalent jobs.
Compare throughput only after preserving required behavior.

## Borrowed vs Owned Parsing

```rust
#[derive(serde::Deserialize)]
struct Record<'a> {
    id: u64,
    name: &'a str,
}
```

A borrowed field can point into the input buffer, avoiding an allocation and
copy. The record then cannot outlive that buffer. Escaping it into a long-lived
queue may force ownership later or keep a large buffer alive.

| Strategy | Benefit | Cost |
|----------|---------|------|
| Borrow from input | Fewer allocations/copies | Lifetime coupling, buffer retention |
| Own each field | Simple independent lifetime | Allocations and copies |
| Intern/deduplicate | Shares repeated values | Lookup, synchronization, retained table |
| Arena per batch | Cheap bulk lifetime | Batch-scoped ownership and retained peak |

Choose at the architecture boundary, not by adding `.to_owned()` until the
borrow checker accepts the design.

## Streaming and Incremental Parsing

```
network chunks: [abc][defgh][ij...]
logical frames:  [abcdef][ghij...]

parser state must handle:
  partial prefix | complete frame(s) | partial suffix
```

Streaming avoids buffering an entire document and can reduce latency to first
result. It adds state-machine complexity and may reduce vectorization or require
boundary copies. Use a reusable byte buffer and compact/advance it rather than
reallocating for each read.

Ecosystem types such as `bytes::Bytes` provide refcounted slices over shared
storage. They can make ownership transfer cheap, but refcounts and retained
backing buffers remain real costs.

## Serde and Representation Choices

Serde separates data-model traversal from format implementations. Derive-based
code is convenient and often fast, but performance still depends on:

- format crate and configuration;
- owned vs borrowed field types;
- map/key handling;
- validation requirements;
- input shape and error frequency;
- monomorphized code size.

```rust
let value: MyType = serde_json::from_slice(input)?;
serde_json::to_writer(&mut output, &value)?;
```

`to_writer` can avoid building an intermediate `String`/`Vec<u8>`, but the
writer's buffering and error behavior matter. Reuse output buffers when
lifetime and retention policy allow.

## Compression Is a Resource Trade

```
uncompressed bytes --compress--> fewer I/O bytes
        |                            |
        +-> more CPU/latency         +-> less network/storage time
```

| Workload | Likely choice |
|----------|---------------|
| CPU-bound, cheap network | Low compression level or none |
| Bandwidth-bound | More compression may increase end-to-end throughput |
| Interactive small messages | Framing/latency may dominate; avoid tiny independent blocks |
| Archival | Compression ratio and decode compatibility matter more |
| Highly parallel service | Codec CPU and memory per stream can become capacity limit |

Benchmark representative compressibility. Repeating zeros and random bytes are
useful extremes, not a realistic corpus. Include compression level, dictionary,
block size, thread count, checksum, and codec version.

## Copies and Data Movement

Common hidden copies:

| Pattern | Copy source |
|---------|-------------|
| `read_to_string` then parse bytes | UTF-8 validation and owned string |
| Serialize to `Vec`, then write | Intermediate output buffer |
| `format!` in hot logging path | Allocation and formatting |
| Clone request for task/queue | Deep payload copy or refcount traffic |
| Convert `String` <-> bytes repeatedly | Validation/ownership transitions |
| Transpose AoS/SoA per stage | Full dataset movement |

Sometimes a copy improves locality and breaks a long lifetime. The goal is not
"zero copies"; it is minimum total cost under the ownership and safety contract.

## Benchmarking the Pipeline

```rust
use criterion::Criterion;
use std::hint::black_box;

fn bench_decode(c: &mut Criterion, corpus: &[Vec<u8>]) {
    c.bench_function("decode-corpus", |b| {
        b.iter(|| {
            for input in black_box(corpus) {
                let value: MyType = serde_json::from_slice(input).unwrap();
                black_box(value);
            }
        })
    });
}
```

Also measure end-to-end read/decode/transform/encode/write. A codec microbenchmark
can improve while total latency regresses because buffers grow or batching
changes. Report per-size/per-shape results as well as any weighted corpus result;
otherwise one large or common record can hide a regression in another class.
Profile allocation sites and CPU counters.

## Platform and Tool Caveats

```
# Linux example: compare syscall volume around an end-to-end run.
strace -c ./target/release/pipeline corpus/

# Cross-platform stable benchmark runner is application-defined.
cargo bench --bench codec
```

Criterion is an external crate but runs on stable Rust. Compression libraries
may bind native code, use target-specific SIMD, or select implementation paths
at runtime. Record features, target CPU, and whether the build used system or
vendored native libraries. Windows and Linux builds can therefore differ
materially.

## Old World -> New World Bridge

| Prior art | Rust |
|-----------|------|
| `Span<byte>` parser | Borrowed `&[u8]` / `&str` parser |
| `System.Text.Json` source generation | Serde derive monomorphization |
| `ArrayPool<byte>` | Reused `Vec<u8>` or buffer pool with retention limits |
| Pipelines / sequence segments | Incremental parser over chunked buffers |
| gzip/Brotli stream | Rust codec crates over `Read`/`Write` or async adapters |
| Protocol buffer generated types | Rust generated schemas and borrowed/owned choices |

The universal bridge is a staged transducer. Make each materialization boundary
visible and justify it.

## Common Confusion Points

- **"Zero copy" may only move the copy to another layer.**
- **Borrowed results can retain a huge input buffer.**
- **Binary is not automatically faster than text** under different validation
  or schema contracts.
- **Compression ratio benchmarks need realistic data.**
- **A codec benchmark is not an I/O benchmark.**
- **Buffer reuse can inflate steady RSS.**
- **Malformed-input paths matter for exposed parsers.**
- **Native codec features make results target-specific.**

## Decision Cheat Sheet

| Need | Start with |
|------|------------|
| Lowest allocation parser | Borrow from a stable input buffer; measure retention |
| Independent long-lived records | Own fields or copy selected values intentionally |
| Huge/streaming input | Incremental parser with bounded reusable buffer |
| Avoid intermediate serialization buffer | Writer-based encoding |
| Network is bottleneck | Compare codec/level by end-to-end time and CPU cost |
| CPU is bottleneck | Lower compression or optimize parsing/layout |
| Need format comparison | Equalize semantics, validation, schema, and corpus |
| Suspected copy overhead | Allocation profile plus CPU profile and byte accounting |

## Primary Sources

- Serde: https://serde.rs/
- `serde::Deserialize`: https://docs.rs/serde/latest/serde/trait.Deserialize.html
- `bytes`: https://docs.rs/bytes/
- Rust Performance Book, serialization: https://nnethercote.github.io/perf-book/serialization.html
- `std::io`: https://doc.rust-lang.org/std/io/

## Related Guides

- Memory ownership: [04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md](04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md)
- I/O: [09-FILES-NETWORKING-BUFFERING-AND-IO.md](09-FILES-NETWORKING-BUFFERING-AND-IO.md)
- Benchmark statistics: [13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md](13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md)
