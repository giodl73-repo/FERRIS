---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:files-networking-buffering-and-io
kind: guide
module: rust-performance
section: rust-performance
title: Files, Networking, Buffering, and I/O
status: source-custody
source_custody: partial
current_path: rust-performance/09-FILES-NETWORKING-BUFFERING-AND-IO.md
canonical_path: rust-performance/09-FILES-NETWORKING-BUFFERING-AND-IO.md
backsource_ids: [mdloom-backfill:rust-performance:09-files-networking-buffering-and-io]
concepts: [file io, networking, buffering, syscalls, vectored io, asynchronous io, memory mapping]
root_concepts: [io performance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Files, Networking, Buffering, and I/O

## The Big Picture

I/O performance is a pipeline of application work, copies, syscalls, kernel
queues, devices or networks, and downstream behavior. A fast parser cannot hide
an undersized socket buffer or a synchronous flush per record.

```
+=============================================================================+
|                            I/O DATA PATH                                    |
|                                                                             |
| application objects                                                         |
|      | encode/copy                                                          |
|      v                                                                      |
| user buffer -> syscall/API -> kernel buffer/cache -> device/network         |
|      ^                |                |                    |               |
|      |                |                |                    v               |
| decode/copy <----------+ completion/wake/interrupt <----- peer/storage      |
|                                                                             |
| cost: calls + bytes copied + queueing + latency + durability + backpressure |
+=============================================================================+
```

## Small Calls vs Buffered Calls

`Read` and `Write` expose byte-oriented operations. Layer buffering when the
underlying source has meaningful per-call overhead:

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_lines(path: &str) -> std::io::Result<usize> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut line = Vec::new();
    let mut count = 0;
    while reader.read_until(b'\n', &mut line)? != 0 {
        count += 1;
        line.clear();
    }
    Ok(count)
}
```

| Pattern | Benefit | Risk |
|---------|---------|------|
| `BufReader`/`BufWriter` | Amortizes syscalls, supports incremental processing | Extra copy and buffer memory |
| `read_to_end` / `read_to_string` | Simple, often efficient for bounded input | Unbounded memory and delayed processing |
| Reused caller buffer | Controls allocation and lifetime | More plumbing |
| Memory mapping | Demand paging, random access, fewer explicit reads | Page faults, lifetime/truncation hazards, platform differences |
| Vectored I/O | One call for several buffers | OS may partially support; still handle partial writes |

Buffer size is workload-specific. Larger buffers reduce call frequency until
cache pressure, latency, or memory per connection dominates.

## Correct Read/Write Loops

I/O APIs can complete partially. Use helpers such as `read_exact` and `write_all`
when their semantics match:

```rust
use std::io::Write;

fn send_frame<W: Write>(mut w: W, header: &[u8], body: &[u8]) -> std::io::Result<()> {
    w.write_all(header)?;
    w.write_all(body)?;
    Ok(())
}
```

For high call rates, consider `write_vectored` with `IoSlice`, but check
the concrete writer's documented implementation and always handle partial
progress. `Write::write_vectored` is stable; in Rust 1.97.1,
`Write::is_write_vectored` is still nightly-only under `can_vector`, so stable
code cannot use that probe as a universal gate. Two buffers do not guarantee one
physical packet or disk operation.

## Filesystem Cache, Directness, and Durability

```
write() success
   |
   +-> bytes accepted by OS cache
   |
   +-> NOT necessarily durable on storage
          |
          +-> flush user buffer
          +-> sync file data/metadata as required
          +-> account for filesystem/device guarantees
```

`BufWriter::flush` pushes user-space buffered data to the underlying writer; it
does not necessarily make data durable. File sync APIs add durability cost and
must follow a carefully specified crash-consistency protocol. Benchmarking
durable writes with sync disabled answers a different question.

Read benchmarks are similarly distorted by page cache. Report cold-cache and
warm-cache cases separately where both matter. Cache eviction commands are
privileged and system-wide; use isolated disposable hosts and approved
procedures rather than disrupting a shared machine.

Direct/unbuffered I/O is a platform-specific contract involving alignment,
request-size, filesystem, and device constraints. It can make storage behavior
more explicit for specialized systems, but it is not a universal way to bypass
the page cache or improve latency.

## Networking and Backpressure

| Layer | Performance question |
|-------|----------------------|
| Application framing | Are messages copied, scanned, or reassembled repeatedly? |
| Socket calls | Are writes tiny? Are reads sized to actual traffic? |
| TCP | Is latency from RTT, Nagle/delayed ACK interaction, congestion, retransmit? |
| TLS | Handshake reuse, record sizing, encryption CPU, certificate path |
| Runtime | Are tasks woken efficiently and bounded? |
| Downstream | Is the peer applying backpressure or timing out? |

Do not set `TCP_NODELAY` reflexively. It reduces intentional coalescing and can
help request/response latency for small writes, but may increase packets and CPU.
Measure protocol-level latency and packet behavior.

## Async I/O and Platform Engines

Rust runtimes adapt platform facilities:

```
Linux: epoll; some ecosystems expose io_uring paths
Windows: IOCP
macOS/BSD: kqueue
```

Exact runtime implementation and feature support vary. Async avoids dedicating
a blocked thread per operation; it does not remove syscalls, copies, kernel
queues, or device latency. File I/O may be implemented through blocking pools on
some runtimes/platforms because readiness models fit sockets better than regular
files.

## Measurement Commands

```
# Linux: syscall summary for a bounded run.
strace -c ./target/release/my_app workload.json

# Linux: system calls and block/network events need appropriate perf/eBPF tools.
perf stat -- ./target/release/my_app workload.json

# Cross-platform network test tool, external to Rust:
iperf3 -c server.example -P 4
```

`strace` perturbs syscall-heavy programs and is Linux-specific. Windows Process
Monitor, WPR/WPA, `netsh trace`, and PerfView/ETW provide complementary file,
TCP, and scheduling evidence. Azure networking results depend on VM SKU,
accelerated networking, placement, throttles, and region topology; record them.
`iperf3` measures a network path under its own protocol and settings, not the
Rust application's framing, TLS, backpressure, or request latency.

## Memory Mapping

Memory mapping can be effective for large random-access files and shared
read-only data, but the safe abstraction must account for external mutation and
file truncation. Ecosystem crates such as `memmap2` wrap platform APIs; mapping
is not ordinary Rust-owned memory. Page faults become part of latency, and RSS
can rise as pages become resident. Truncating a mapped file can fault the
process on some platforms; isolate file lifecycle and validate platform
behavior.

Use mapping when access patterns and file lifecycle justify it, not as a
universal replacement for buffered reads.

## Old World -> New World Bridge

| Prior art | Rust |
|-----------|------|
| `Stream`, `BufferedStream`, `BinaryReader` | `Read`/`Write`, `BufReader`/`BufWriter` |
| Scatter/gather I/O | `IoSlice` / vectored I/O |
| IOCP/epoll | Async runtime reactor/driver |
| `MemoryMappedFile` | `memmap2` or direct platform APIs |
| ETW disk/TCP traces | Same OS-level tracing for Rust native processes |
| Azure throughput/SKU tuning | Same limits; Rust changes application overhead, not platform quotas |

The universal model is amortize crossings, bound queues, avoid unnecessary data
movement, and preserve the semantic distinction between buffered and durable.

## Common Confusion Points

- **One `write` is not one packet or one durable disk operation.**
- **`flush` is not necessarily `fsync`.**
- **Warm page-cache throughput is not cold storage throughput.**
- **Larger buffers can hurt latency and memory.**
- **Async file I/O may use blocking threads.**
- **Vectored I/O can still complete partially.**
- **Memory mapping moves cost to page faults; it does not make I/O free.**
- **Cloud throughput may be capped below device or network capability.**

## Decision Cheat Sheet

| Workload | Start with |
|----------|------------|
| Many small file reads | `BufReader`, reused buffer, syscall/profile evidence |
| Bounded file consumed once | `read_to_end` if memory budget is explicit |
| Large random-access immutable file | Evaluate memory mapping and page-fault behavior |
| Many small writes | `BufWriter`, batching, or vectored writes |
| Durable journal | Explicit flush + file/directory sync protocol; benchmark durability |
| High-concurrency sockets | Async runtime with bounded per-peer work |
| Low-latency small TCP messages | Measure `TCP_NODELAY`, batching, and RTT together |
| Suspected I/O bottleneck | Trace syscalls/waits before optimizing parser code |

## Primary Sources

- `std::io`: https://doc.rust-lang.org/std/io/
- `BufReader`: https://doc.rust-lang.org/std/io/struct.BufReader.html
- `BufWriter`: https://doc.rust-lang.org/std/io/struct.BufWriter.html
- Tokio I/O: https://docs.rs/tokio/latest/tokio/io/
- Linux `strace`: https://strace.io/

## Related Guides

- Async scheduling: [07-ASYNC-RUNTIME-SCHEDULING-TASKS-AND-LATENCY.md](07-ASYNC-RUNTIME-SCHEDULING-TASKS-AND-LATENCY.md)
- Parsing/copies: [10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md](10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md)
- Production capacity: [14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md](14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md)
