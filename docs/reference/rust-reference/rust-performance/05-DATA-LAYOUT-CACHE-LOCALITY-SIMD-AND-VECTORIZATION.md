---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:data-layout-cache-locality-simd-and-vectorization
kind: guide
module: rust-performance
section: rust-performance
title: Data Layout, Cache Locality, SIMD, and Vectorization
status: source-custody
source_custody: partial
current_path: rust-performance/05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md
canonical_path: rust-performance/05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md
backsource_ids: [proof-backfill:rust-performance:05-data-layout-cache-locality-simd-and-vectorization]
concepts: [data layout, cache locality, simd, vectorization, alignment, array of structs, structure of arrays]
root_concepts: [data layout]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Data Layout, Cache Locality, SIMD, and Vectorization

## The Big Picture

The CPU consumes cache lines and vectors, not source-level structs. Rust gives
precise tools for representation, but default Rust layout is optimized for the
language implementation's needs and is not a stable external ABI.

```
+=============================================================================+
|                         DATA TO EXECUTION                                   |
|                                                                             |
| domain model -> Rust representation -> bytes/padding/alignment              |
|                      |                                                      |
|                      v                                                      |
| access order -> cache lines -> prefetch/TLB -> scalar or vector operations  |
|                      |                              |                       |
|                      +-> working set                 +-> auto-vectorization |
|                     +-> false sharing              +-> std::arch intrinsics |
|                                                     +-> portable SIMD(*)    |
|                                                                             |
| (*) std::simd is nightly-only under portable_simd in Rust 1.97.1            |
+=============================================================================+
```

## Layout Contracts

| Representation | Contract | Use |
|----------------|----------|-----|
| Default `repr(Rust)` | Field/layout details may change; compiler controls layout | Internal Rust data |
| `#[repr(C)]` | C-compatible field ordering/layout rules for the target ABI | FFI and externally interpreted layouts |
| `#[repr(transparent)]` | Wrapper layout follows its single non-zero-sized field | FFI-safe newtypes where other rules also hold |
| Integer `repr` on enum | Controls discriminant representation subject to enum rules | FFI/protocol cases with care |
| Packed representation | Removes/reduces padding for a representation contract | Rare ABI/storage interop with alignment hazards; prefer explicit parsing for wire formats |

```rust
use std::mem::{align_of, size_of};

#[repr(C)]
struct Header {
    tag: u8,
    len: u32,
    flags: u16,
}

fn main() {
    println!("size={} align={}", size_of::<Header>(), align_of::<Header>());
}
```

Run on every supported target; ABI alignment can differ. Do not take references
to misaligned packed fields: that can be undefined behavior. Copy bytes into an
aligned value or use the appropriate unaligned operations in carefully reviewed
unsafe code.

## Padding and Field Order

```
naive conceptual order:   [u8][---pad---][u64........][u16][-pad-]
grouped conceptual order: [u64........][u16][u8][pad]
```

Reordering fields can reduce size, increasing cache density. Under default Rust
layout, source order is not an ABI promise and the compiler controls actual
layout; with `repr(C)`, source order is significant. Verify with `size_of`, not
eyeballing. Smaller is not automatically faster: alignment, access frequency,
and vector loads matter.

## Array of Structs vs Structure of Arrays

Suppose a simulation updates position from velocity but rarely reads labels.

```
AoS:
[x y vx vy label][x y vx vy label][x y vx vy label]...
 cache lines carry cold label bytes through the hot loop

SoA:
x:     [x x x x x x ...]
y:     [y y y y y y ...]
vx:    [v v v v v v ...]
vy:    [v v v v v v ...]
label: [cold references ...]
```

| Layout | Best when | Cost |
|--------|-----------|------|
| Array of structs | Operations consume most fields per entity | Simple ownership, good entity locality |
| Structure of arrays | Hot loops consume a few fields across many entities | More coordination; insertion/removal complexity |
| Hybrid/chunked | Mixed operations and bounded working sets | More design complexity |

Make the workload choose. A serializer that emits complete records may prefer
AoS; a numeric kernel often prefers SoA.

## Locality Before SIMD

The optimization ladder is:

```
remove work -> improve algorithm -> make access contiguous -> reduce working set
            -> remove unpredictable branches -> inspect auto-vectorization
            -> use explicit SIMD only if evidence remains
```

Pointer-heavy graphs and hash maps trade locality for other properties. Before
micro-tuning instructions, consider compact indices, slabs, sorted vectors,
small-vector representations, or batching.

## Auto-Vectorization

Simple loops over contiguous slices are optimizer-friendly:

```rust
pub fn saxpy(a: f32, x: &[f32], y: &mut [f32]) {
    assert_eq!(x.len(), y.len());
    for (xi, yi) in x.iter().zip(y.iter_mut()) {
        *yi = a * *xi + *yi;
    }
}
```

Whether this vectorizes depends on compiler version, target features, aliasing
knowledge, trip count, floating-point semantics, and surrounding code. Inspect
optimized assembly and benchmark; do not claim that iterator syntax or indexing
alone determines vectorization.

```
# External maintained tool; command name remains `cargo asm`.
cargo install cargo-show-asm --locked
cargo asm --lib my_crate::saxpy

# Stable rustc emission for one selected library target.
cargo rustc --release --lib -- --emit=asm
```

`cargo-show-asm` is external; pin its version in repeatable automation. Select
`--bin <name>` instead of `--lib` for a binary target. On Windows/MSVC, assembly
and object conventions differ; use `dumpbin`, LLVM tools, or Visual Studio
disassembly as appropriate.

## Explicit SIMD

There are three broad routes:

| Route | Stability/portability | Use |
|-------|-----------------------|-----|
| Auto-vectorization | Available on stable; emitted vector code is not guaranteed | Default first choice |
| `std::arch` target intrinsics | Stable APIs for supported architectures, often `unsafe` | Hot kernels with explicit target dispatch |
| Portable SIMD (`std::simd`) | Nightly-only under `portable_simd` in Rust 1.97.1; recheck later toolchains | Cross-architecture vector code when toolchain policy permits |

Runtime dispatch on x86 can use stable feature-detection macros:

```rust
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn sum_scalar(data: &[f32]) -> f32 {
    data.iter().copied().sum()
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn sum_avx2(data: &[f32]) -> f32 {
    // This complete dispatch example lets the optimizer use AVX2. A measured
    // explicit-intrinsics kernel can replace the body without changing the gate.
    data.iter().copied().sum()
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn choose_kernel(data: &[f32]) -> f32 {
    if std::is_x86_feature_detected!("avx2") {
        // SAFETY: call an AVX2-targeted implementation only after detection.
        unsafe { sum_avx2(data) }
    } else {
        sum_scalar(data)
    }
}
```

The AVX2 function must carry the matching target-feature contract and its unsafe
preconditions must be documented. The sample demonstrates dispatch, not a
universal speedup or a promise that this reduction vectorizes. Provide a scalar
fallback and test both paths. On ARM, use the corresponding architecture
facilities and deployment baseline.

## Cache and Counter Evidence

```
# Linux example; event names/availability vary by CPU.
perf stat -e cycles,instructions,cache-references,cache-misses,branches,branch-misses \
  -- ./target/release/kernel_bench
```

Virtual machines may not expose reliable counters. Use elapsed time and a
physical or dedicated host for claims that depend on cache events. For NUMA
systems, record CPU and memory placement; a locality win on one socket can
disappear when memory is remote. Normalize event counts per instruction, access,
or unit of work and check perf's multiplexing percentage.

## Old World -> New World Bridge

| Prior art | Rust expression |
|-----------|-----------------|
| C/C++ POD layout | `repr(C)` when an ABI contract is required |
| Struct packing pragmas | `repr(packed)` with stricter unaligned-access hazards |
| `Span<T>` numeric loops | slices and iterators over contiguous storage |
| `System.Numerics.Vector<T>` / intrinsics | auto-vectorization, `std::arch`, possibly portable SIMD |
| ECS SoA/chunk layouts | Same data-oriented design; Rust ownership helps encode exclusive access |
| Cache-line padding types | Explicit alignment/padding wrappers after measuring false sharing |

Rust's advantage is not a magic layout. It is the ability to pair data-oriented
representation with checked borrowing and controlled unsafe kernels.

## Common Confusion Points

- **Default Rust layout is not a stable FFI or persistence format.**
- **Packed data is not automatically cache-efficient.** Misalignment can cost
  more and introduces unsafe access hazards.
- **Smaller structs can still be slower** if fields needed together become
  scattered.
- **Iterator syntax does not prevent vectorization.**
- **`target-cpu=native` is not portable.**
- **Explicit SIMD can lose on small inputs** because dispatch and tail handling
  dominate.
- **Portable SIMD stability must be checked against the current toolchain.**

## Decision Cheat Sheet

| Problem | First choice |
|---------|--------------|
| Oversized hot records | Inspect `size_of`/alignment and hot-vs-cold field split |
| Cache misses in a field-wise loop | Evaluate SoA or chunked layout |
| Branch-heavy kernel | Separate classes/batches; then inspect vectorization |
| Need a portable default | Scalar/auto-vectorized implementation |
| Need ISA-specific speed | Runtime-dispatched `std::arch` kernel plus scalar fallback |
| FFI layout | `repr(C)` and target-specific layout tests |
| Persistent/wire format | Explicit serialization, not transmuted Rust layout |
| Need to prove improvement | Counters plus end-to-end benchmark on supported CPUs |

## Primary Sources

- Rust Reference, type layout: https://doc.rust-lang.org/reference/type-layout.html
- `std::arch`: https://doc.rust-lang.org/std/arch/
- `std::simd` nightly status and API: https://doc.rust-lang.org/std/simd/
- Rust Performance Book, data structures: https://nnethercote.github.io/perf-book/data-structures.html
- Rust Reference, `target_feature`: https://doc.rust-lang.org/reference/attributes/codegen.html#the-target_feature-attribute

## Related Guides

- Memory ownership: [04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md](04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md)
- Code generation: [06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md](06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md)
- Parsing/data movement: [10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md](10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md)
