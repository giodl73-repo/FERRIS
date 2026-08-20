---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:allocation-ownership-memory-footprint-and-allocators
kind: guide
module: rust-performance
section: rust-performance
title: Allocation, Ownership, Memory Footprint, and Allocators
status: source-custody
source_custody: partial
current_path: rust-performance/04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md
canonical_path: rust-performance/04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md
backsource_ids: [proof-backfill:rust-performance:04-allocation-ownership-memory-footprint-and-allocators]
concepts: [allocation, ownership, memory footprint, allocators, collections, cloning, reference counting]
root_concepts: [memory performance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Allocation, Ownership, Memory Footprint, and Allocators

## The Big Picture

Rust makes ownership explicit, but ownership alone does not imply stack
allocation, low memory use, or zero copies. Memory behavior emerges from
representation, allocation frequency, capacity policy, lifetime, sharing, and
the process allocator.

```
+=============================================================================+
|                         MEMORY COST PIPELINE                                |
|                                                                             |
| values + ownership graph                                                    |
|        |                                                                    |
|        +-> inline/stack/static storage                                      |
|        +-> heap owners: Box, Vec, String, maps                              |
|        +-> shared owners: Rc/Arc + Weak                                     |
|        +-> copies/clones/buffers                                            |
|                         |                                                   |
|                         v                                                   |
| allocation sites -> allocator arenas -> virtual pages -> resident working   |
|                         |                                                   |
|                         v                                                   |
| volume/rate     live bytes     retained capacity     fragmentation     RSS  |
+=============================================================================+
```

## Separate the Memory Metrics

| Metric | What it tells you | What it can hide |
|--------|-------------------|------------------|
| Allocation count | Frequency of allocator calls | Size and lifetime |
| Allocated bytes | Total requested volume over time | Reuse, deallocation, residency |
| Live bytes | Reachable allocated memory at a point | Allocator metadata/fragmentation |
| Peak live bytes | Maximum application-owned heap | OS page cache and mapped files |
| RSS / working set | Resident process pages | Shared pages and reclaimability |
| PSS / private working set | Better attribution of shared pages where the OS exposes it | Application object ownership |
| Virtual size | Address space reservations/mappings | Actual physical memory |
| Retained capacity | Memory collections keep for reuse | Whether retention is useful |

A service can allocate terabytes cumulatively while keeping a small working set,
or allocate little after startup while retaining a huge arena. Collect the
metric that matches the operational problem.

## Ownership Shapes Cost

```
T                inline value; no heap implied
Box<T>           normally one heap allocation for non-zero-sized T
Vec<T>           inline pointer/length/capacity + buffer when capacity needs it
String           Vec<u8>-like owned UTF-8 buffer
Rc<T>            non-atomic shared ownership + allocation
Arc<T>           atomic shared ownership + allocation
Cow<'a, T>       borrowed until mutation/ownership is required
&[T] / &str      borrowed view; no ownership or allocation by itself
```

| Choice | Performance benefit | Performance risk |
|--------|---------------------|------------------|
| Borrowed slice/string | Avoids copy and allocation | Couples lifetime; may keep a large owner alive |
| Move ownership | Transfers without copying buffer contents | Can force architecture changes |
| `clone()` | Clear ownership boundary | May deep-copy large buffers or increment refcounts |
| `Arc::clone` | Cheap payload sharing | Atomic refcount traffic and shared lifetime |
| `Cow` | Avoids copy on read-mostly paths | Branching and surprise allocation on mutation |
| Arena/bump allocation | Cheap bulk allocation and teardown | Coarse lifetime, retained memory, destructor constraints |

The most valuable Rust optimization is often changing ownership so data moves
once through the pipeline rather than cloning across layer boundaries.

## Collection Capacity and Reuse

```rust
fn encode_rows(rows: &[Row], out: &mut Vec<u8>) {
    out.clear();                       // retains capacity
    let estimate = rows.len().checked_mul(32).expect("batch too large");
    out.reserve(estimate);             // estimate; measure accuracy
    for row in rows {
        encode_row(row, out);
    }
}
```

Reuse avoids allocation when capacity is appropriate. It can also pin the
largest-ever request's buffer indefinitely. Define a retention policy:

```rust
if out.capacity() > MAX_RETAINED {
    *out = Vec::new();
} else {
    out.clear();
}
```

| Method | Meaning |
|--------|---------|
| `with_capacity` / `reserve` | Grow ahead of time; may over-allocate |
| `reserve_exact` | Requests tighter growth; allocator can still round |
| `clear` | Drops elements, retains allocation |
| `shrink_to_fit` | Requests capacity reduction; may allocate/copy and is not guaranteed exact |
| `mem::take` | Replaces with default and transfers old allocation |

Benchmark under real size distributions. A reserve policy tuned to median input
may repeatedly grow at p99; one tuned to maximum may waste memory everywhere.

## Finding Allocation Sites

Tools are platform-specific:

| Platform/tool | Useful evidence | Caveat |
|---------------|-----------------|--------|
| DHAT via Valgrind | Allocation sites, bytes, lifetimes | Linux-oriented, high overhead |
| heaptrack | Allocation call stacks and temporary allocations | Linux; requires symbols |
| Valgrind Massif | Heap-over-time snapshots | Slow; allocator/accounting model differs |
| jemalloc/mimalloc statistics | Allocator internals and retained pages | Only when using/configuring that allocator |
| Windows WPR/WPA heap profiles | Native heap events and stacks | Windows SDK/profile setup and overhead |
| Custom counting allocator | Test-local counts/bytes | Instrumentation perturbs behavior; global scope |

```
# Linux example; use an optimized symbolized binary.
heaptrack ./target/profiling/my_app workload.json
heaptrack_gui heaptrack.my_app.*.gz

# Valgrind DHAT example.
valgrind --tool=dhat ./target/profiling/my_app workload.json
```

Valgrind generally does not run native Windows executables. Use WPR/WPA,
Visual Studio diagnostics, allocator instrumentation, or a Linux reproduction
where valid. Containers may require ptrace permissions.

## Global Allocators

Rust's stable `#[global_allocator]` hook selects a process-wide allocator for
Rust allocation APIs that use the global allocator:

```rust
use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;
```

Crates can provide jemalloc-, mimalloc-, or other allocator bindings. Swapping
allocators may improve contention, fragmentation, or tail latency for a given
workload; it can also increase RSS or binary size. Measure:

1. throughput and latency under representative concurrency;
2. peak and steady RSS;
3. fragmentation/retained pages after load subsides;
4. startup and binary impact;
5. target and license/deployment constraints.

Native libraries can allocate through their own C/C++ runtime or allocator, so
changing Rust's global allocator does not necessarily change every allocation in
the process. Memory must be released through the same allocator contract that
created it; allocator changes do not repair an FFI ownership mismatch.

The general per-collection allocator API (`Allocator`-parameterized standard
collections) remains nightly-only under `allocator_api` in Rust 1.97.1. Check
the current toolchain before adopting it because that status can change. The
stable global hook is a different surface.

## Reference Counting and Shared State

`Arc` increments/decrements are atomic. The payload may also be behind a lock,
creating two independent costs:

```
Arc<Mutex<T>>
 |    |
 |    +-> lock acquisition, queueing, cache-line transfer
 +------> atomic refcount operations and shared lifetime
```

Avoid cloning an `Arc` in a tight loop when one clone can be moved into the
worker. Do not replace `Arc` with unsafe raw pointers merely to remove measured
refcount cost without proving lifetime and thread-safety contracts.

## Old World -> New World Bridge

| Familiar model | Rust mapping |
|----------------|--------------|
| C++ RAII and `unique_ptr` | Ownership plus `Drop` and `Box<T>` |
| `shared_ptr` | `Arc<T>` across threads, `Rc<T>` within one thread |
| ArrayPool / object pools | Reused `Vec`/buffer pools, with explicit retention policy |
| CLR allocation profiler | Native allocation profiler; no default managed heap/GC events |
| `Span<T>` / `ReadOnlySpan<T>` | `&mut [T]` / `&[T]` borrowed views |
| Arena allocator | Bump/arena crates with bulk lifetime |

The universal lesson is to reason about ownership and lifetime before changing
the allocator. Reducing copies or shortening live ranges usually generalizes
better than replacing one allocator with another.

## Common Confusion Points

- **Stack vs heap is not determined by `Copy`.**
- **`clone()` may be deep, shallow, or refcount-only.** Read the type contract.
- **Allocation volume is not live memory or RSS.**
- **`clear()` does not free a collection's capacity.**
- **A faster allocator cannot repair an allocation-heavy design.**
- **A pool can turn transient peaks into permanent retention.**
- **Borrowing can retain an unexpectedly large owner.**
- **Allocator results are workload-, OS-, target-, and concurrency-specific.**

## Decision Cheat Sheet

| Observation | First action |
|-------------|--------------|
| Many short-lived allocations | Profile sites; reserve/reuse or batch ownership |
| Large copies between layers | Pass ownership or borrowed views; verify lifetime impact |
| High steady RSS after a spike | Inspect retained capacities and allocator arenas |
| `Arc` hot in CPU profile | Reduce clone frequency or sharing scope; check contention separately |
| Fragmentation under concurrency | Compare allocators under the same workload and RSS policy |
| Need temporary graph/object allocation | Evaluate an arena with a matching bulk lifetime |
| Need per-collection custom allocation | Check stable/nightly status; avoid accidental nightly dependency |
| Memory regression gate | Track allocation and peak/steady RSS, not one proxy |

## Primary Sources

- Rust Performance Book, heap allocations: https://nnethercote.github.io/perf-book/heap-allocations.html
- `std::alloc`: https://doc.rust-lang.org/std/alloc/
- `Vec`: https://doc.rust-lang.org/std/vec/struct.Vec.html
- `Arc`: https://doc.rust-lang.org/std/sync/struct.Arc.html
- Valgrind DHAT: https://valgrind.org/docs/manual/dh-manual.html

## Related Guides

- CPU attribution: [03-CPU-PROFILING-AND-FLAME-GRAPHS.md](03-CPU-PROFILING-AND-FLAME-GRAPHS.md)
- Layout and locality: [05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md](05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md)
- Data movement: [10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md](10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md)
