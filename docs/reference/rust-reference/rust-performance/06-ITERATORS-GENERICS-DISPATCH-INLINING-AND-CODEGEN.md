---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:iterators-generics-dispatch-inlining-and-codegen
kind: guide
module: rust-performance
section: rust-performance
title: Iterators, Generics, Dispatch, Inlining, and Codegen
status: source-custody
source_custody: partial
current_path: rust-performance/06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md
canonical_path: rust-performance/06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md
backsource_ids: [proof-backfill:rust-performance:06-iterators-generics-dispatch-inlining-and-codegen]
concepts: [iterators, generics, dispatch, inlining, monomorphization, code generation, code size]
root_concepts: [rust code generation]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Iterators, Generics, Dispatch, Inlining, and Codegen

## The Big Picture

Rust abstractions become fast when optimization can see through them. That is a
code-generation outcome, not a semantic guarantee attached to words such as
"iterator" or "zero cost."

```
+=============================================================================+
|                         ABSTRACTION TO MACHINE CODE                         |
|                                                                             |
| source                                                                      |
|  iterators + closures + generics + traits                                   |
|                |                                                            |
|                v                                                            |
| MIR optimization -> monomorphized instances -> codegen units -> backend     |
|                |                  |                 |                       |
|                |                  |                 +-> inlining/LTO        |
|                |                  +-> code size / compile time              |
|                +-> bounds-check and adapter simplification                  |
|                                                                             |
| alternate: dyn Trait / fn pointer -> indirect call -> less specialization   |
+=============================================================================+
```

## Iterators: Fusion Is an Optimization Result

```rust
pub fn sum_positive(xs: &[i32]) -> i64 {
    xs.iter()
        .copied()
        .filter(|x| *x > 0)
        .map(i64::from)
        .sum()
}
```

In optimized builds, the adapter chain often fuses into one loop. "Often" is
the correct word. Complex closures, opaque calls, aliasing, panic paths, and
missed inlining can change the result. Compare representative source variants
only after checking optimized code and end-to-end measurements.

| Form | Typical strength | Typical risk |
|------|------------------|--------------|
| Iterator chain | Expresses dataflow; enables fusion and specialization | Harder attribution; accidental allocation via `collect` |
| Indexed loop | Explicit indices and control | Bounds checks may remain; more error-prone |
| Slice `for` loop | Clear contiguous traversal | Less compositional |
| `collect::<Vec<_>>()` between stages | Materializes reuse or ownership boundary | Allocation, copy, larger working set |

Avoid "rewrite iterators as loops" as a policy. Rewrite only when measurement and
code inspection show a material issue.

## Bounds Checks

The compiler removes checks when it proves indices are in range. Slice iteration
usually communicates this well:

```rust
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
```

Using `get_unchecked` to remove a suspected check introduces unsafe proof
obligations. First inspect assembly or measure a variant. If an unsafe kernel is
justified, keep it narrow, assert preconditions at the boundary, and test with
Miri/sanitizers where supported.

## Static and Dynamic Dispatch

```
generic T: Trait             &dyn Trait / Box<dyn Trait>
       |                              |
       v                              v
monomorphize per type          fat pointer + vtable call
       |                              |
direct call/inlining           indirect call, type erased
```

| Choice | Runtime | Build/artifact | Best fit |
|--------|---------|----------------|----------|
| Generic/static dispatch | Direct/specializable | More instances, compile time, code size | Hot reusable algorithms, small type set |
| `dyn Trait` | Indirect call; less optimizer visibility | Smaller shared implementation possible | Plugins, heterogeneous values, cold paths |
| `fn` pointer | Indirect callable | Compact and ABI-like | Strategy selected at runtime |
| Enum dispatch | Branch then direct variant code | Closed set, potentially larger enum | Small known strategy set |

Dynamic dispatch cost may be negligible compared with I/O or a large operation.
It can also inhibit inlining in a tiny hot loop. Measure in context.

## Monomorphization and Code Size

Generics can instantiate similar code for many types:

```
parse<T1>  parse<T2>  parse<T3> ... -> distinct mono items
              |
              +-> compile time
              +-> instruction-cache pressure
              +-> binary size
```

Common containment patterns:

1. Keep a small generic adapter and move heavy logic into a non-generic
   function over a slice or erased internal representation.
2. Use dynamic dispatch at cold/outer boundaries.
3. Avoid encoding large state spaces in generic types unless they buy real
   correctness or specialization.
4. Inspect instance size rather than guessing.

```
# External Cargo subcommands.
cargo install cargo-llvm-lines --locked
cargo llvm-lines --release

cargo install cargo-bloat --locked
cargo bloat --release --crates
```

Both tools are external and platform/toolchain sensitive. Treat reports as
artifact observations, not stable compiler APIs.

## Inlining

`#[inline]` and `#[inline(always)]` are hints to the compiler. They are not
semantic guarantees. Inlining can:

- remove call overhead;
- expose constants and eliminate branches/bounds checks;
- enable vectorization;
- increase code size and compile time;
- worsen instruction-cache behavior.

Cross-crate generic functions carry code for monomorphization. Non-generic
cross-crate inlining depends on metadata and optimization/LTO decisions. Do not
sprinkle `#[inline(always)]` across a crate; use it only with measured codegen
evidence and re-check every supported target.

## Inspecting Code Generation

```
# Stable emission for one selected library target.
cargo rustc --release --lib -- --emit=asm
cargo rustc --release --lib -- --emit=llvm-ir

# External maintained convenience view (`cargo-show-asm` package).
cargo install cargo-show-asm --locked
cargo asm --lib my_crate::hot_function
cargo llvm-lines --release
```

Use `--bin <name>` instead of `--lib` for a binary target. Pin external tools in
automation; generated assembly and LLVM IR remain observations of the selected
compiler, target, features, and profile.
Compiler Explorer is useful for isolated functions but can omit Cargo features,
dependencies, link-time optimization, and whole-program context. A short
assembly listing can prove that a bound check or indirect call exists; it cannot
prove end-to-end impact.

Nightly `-Z` dumps such as MIR or mono-item diagnostics are explicitly
version-sensitive. Pin the nightly toolchain if used and keep release builds on
stable unless there is an independently justified nightly policy.

## `black_box` and Benchmark Integrity

```rust
use std::hint::black_box;

fn bench_body(input: &[u64]) -> u64 {
    black_box(input).iter().copied().sum()
}
```

`black_box` is a best-effort optimization barrier for benchmarking, not a
security boundary or a promise to preserve every operation. Verify the benchmark
still measures the intended work.

## Old World -> New World Bridge

| Prior art | Rust |
|-----------|------|
| C++ templates | Rust generic monomorphization |
| Interface virtual call | `dyn Trait` vtable dispatch |
| Delegate/function pointer | `fn` pointer or closure traits |
| LINQ pipeline | Iterator adapters, generally eager only when collected |
| JIT inlining diagnostics | AOT assembly/MIR inspection and LTO effects |
| Generic sharing in managed runtimes | Rust normally emits concrete native instances |

The universal decision remains specialization versus erasure. Rust makes both
explicit and lets ownership/traits carry the contracts.

## Common Confusion Points

- **"Zero-cost abstraction" means no inherent overhead is required, not that
  every use optimizes perfectly.**
- **Iterators are not always faster, and loops are not always faster.**
- **`#[inline(always)]` can regress performance.**
- **Dynamic dispatch is not automatically slow.** Granularity matters.
- **Assembly for one target is not universal.**
- **Monomorphization affects runtime, binary size, and compile time.**
- **Unsafe indexing should follow proof, not suspicion.**

## Decision Cheat Sheet

| Evidence | Action |
|----------|--------|
| Iterator chain disappears into one loop | Keep the clearer abstraction |
| Intermediate `collect` dominates allocations | Stream, reuse, or justify materialization |
| Bound check is hot and remains | Restructure iteration/proof; unsafe only as a narrow last step |
| Many large generic instances | Split generic adapter from non-generic core |
| Indirect call dominates tiny hot operation | Compare generic/enum dispatch under real code size |
| Binary/instruction cache grows | Inspect mono items and [11](11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md) |
| Need confidence | Benchmark plus code inspection; neither alone |

## Primary Sources

- Rust Performance Book, bounds checks: https://nnethercote.github.io/perf-book/bounds-checks.html
- Rust Performance Book, inlining: https://nnethercote.github.io/perf-book/inlining.html
- Rust Reference, traits: https://doc.rust-lang.org/reference/items/traits.html
- rustc-dev-guide, monomorphization: https://rustc-dev-guide.rust-lang.org/backend/monomorph.html
- `std::hint::black_box`: https://doc.rust-lang.org/std/hint/fn.black_box.html

## Related Guides

- Layout/vectorization: [05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md](05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md)
- Linking and size: [11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md](11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md)
- Compile-time cost: [12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md](12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md)
