---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:monomorphization-codegen-units-vtables
kind: guide
module: rust-architecture
section: rust-architecture
title: Monomorphization, Codegen Units, and Vtables
status: source-custody
source_custody: partial
current_path: rust-architecture/11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md
canonical_path: rust-architecture/11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md
backsource_ids: [proof-backfill:rust-architecture:11-monomorphization-codegen-units-vtables]
concepts: [monomorphization, instance collection, codegen units, vtables, dynamic dispatch, polymorphization]
root_concepts: [monomorphization]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Monomorphization, Codegen Units, and Vtables

## The Big Picture

Rust has two main dispatch stories. Generic code is usually monomorphized:
rustc stamps out concrete instances and backend code sees ordinary functions.
Trait objects use dynamic dispatch: a fat pointer carries data plus a vtable
pointer. Codegen units then partition the resulting mono items so backends can
compile in parallel and incremental compilation can reuse work.

```
+===========================================================================+
|                    FROM MIR TO BACKEND WORK PRODUCTS                      |
|                                                                           |
|  optimized MIR [09]                                                       |
|       |                                                                   |
|       v                                                                   |
|  mono item collector                                                      |
|  roots: main/exported/reachable/#[no_mangle]                              |
|       |                                                                   |
|       v                                                                   |
|  Instances: DefId + concrete substitutions                                |
|  plus statics, vtables, drop glue                                         |
|       |                                                                   |
|       v                                                                   |
|  codegen-unit partitioner                                                 |
|       partitions: CGU 0   CGU 1   CGU 2   ...   CGU N                     |
|                         v                                                 |
|              LLVM / Cranelift / GCC backend [12]                          |
+===========================================================================+
```

Authority boundary: the Rust Reference owns generic and trait-object semantics,
including dyn-compatibility rules; rustc owns monomorphization, collector
algorithms, codegen-unit partitioning, vtable emission, and symbol spelling;
Cargo selects profiles and passes stable flags; rustup selects the toolchain;
std supplies traits and drop glue targets; LLVM/backends compile CGUs;
ecosystem tools such as `rustfilt`, profilers, and analyzers inspect artifacts
without defining language behavior.

---

## Monomorphization: Static Dispatch by Stamping

Monomorphization is the default zero-cost generics strategy: for each concrete
instantiation rustc needs, it creates backend code specialized to those concrete
types. The payoff is direct calls and optimization through concrete layouts. The
cost is code size and compile time.

```
source generic item
   fn max_of<T: Ord>(a: T, b: T) -> T
        |
        +-----------------------+------------------------+
        |                       |                        |
        v                       v                        v
   max_of::<i32>          max_of::<String>         max_of::<MyKey>
   concrete layout        concrete layout          concrete layout
   direct comparisons     direct comparisons       direct comparisons
```

| Strategy | Runtime shape | Trade-off |
|----------|---------------|-----------|
| Rust monomorphization | Specialized native code per used type | Fast/static, but bloat and compile cost |
| C++ templates | Similar per-type instantiation | Strong old-world analogue |
| .NET generics | JIT specializes value types, shares many reference-type cases | Less AOT bloat, runtime/JIT involved |
| Java erasure | Mostly one erased body plus casts/bridges | Smaller code, less specialization |

```rust
pub fn max_of<T: Ord + Clone>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

pub fn use_it() {
    let _ = max_of::<i32>(1, 2);
    let _ = max_of::<String>("a".into(), "b".into());
}
```

Conceptually, the backend does not optimize one generic `max_of<T>`. It sees the
instances that survive collection and partitioning.

---

## Mono Item Collection

After MIR exists, rustc collects the concrete work items that require code or
metadata. A central unit is an `Instance`: roughly a `DefId` plus concrete
substitutions, after trait selection and normalization have made the callable
thing specific enough to generate.

```
+-------------------------+
| roots                   |
| main, exports, reachable|
| #[no_mangle], lang glue |
+------------+------------+
             |
             v
+-------------------------+        calls/uses/needs        +----------------+
| collect mono items      | -----------------------------> | more instances |
| functions, statics      |                                | statics, glue  |
| vtables, drop glue      | <----------------------------- | vtables        |
+------------+------------+            until fixed point   +----------------+
             |
             v
      partition into CGUs
```

| Mono item | Why it is collected |
|-----------|---------------------|
| Function `Instance` | A reachable call needs machine code |
| Static | Addressable data must be emitted or referenced |
| Drop glue | Destructors and recursive field drops need callable code |
| Vtable | A `dyn Trait` value needs method/drop/layout entries |
| Shim | ABI adaptation, closures, fn pointers, and trait dispatch support |

Collection is reachability-driven and crate-type-sensitive. The exact roots,
collection modes, and transitive traversal details are rustc internals. They are
also where surprises about "why did this generic get compiled?" usually start.

---

## Polymorphization: Removed Optimization

Rustc formerly implemented **polymorphization**, an optimization that attempted
to share instances when a generic parameter could not affect generated code.
That implementation was removed in 2024 because its maintenance cost and
interactions with MIR optimization outweighed its demonstrated benefit.

| Question | Current answer |
|----------|----------------|
| Does rustc currently share generic instances through polymorphization? | No |
| Should architecture or size estimates assume it? | No; reason from ordinary monomorphization |
| Can identical native code still be folded? | A backend or linker may perform identical-code folding, but that is separate, target-dependent machinery |

```
generic substitutions
        |
        v
distinct rustc Instances -> mono item collection -> codegen units
                                                   |
                                                   v
                              backend/linker may deduplicate identical code
                              (optimization detail, not a Rust guarantee)
```

The safe current statement is simpler: Rust's compiler model is
monomorphization. Any later machine-code deduplication is a backend/linker
optimization and must not be treated as semantic instance sharing.

---

## Codegen Units

Codegen units are rustc's partitioning of mono items into backend work products.
They exist for parallelism and for incremental reuse. The flag is stable; the
partitioning heuristics are not.

```
mono items
   |
   v
+------------------ codegen-unit partitioner -------------------+
| module affinity, inlining pressure, size, incremental reuse   |
| heuristics are internal and change over time                  |
+-------------+------------------+------------------+-----------+
              |                  |                  |
              v                  v                  v
          CGU A              CGU B              CGU C
              |                  |                  |
              v                  v                  v
          backend job        backend job        backend job
```

| Setting | Effect | When to use |
|---------|--------|-------------|
| `-C codegen-units=N` | Stable rustc flag controlling partition count | Direct rustc or `cargo rustc` experiments |
| Cargo profile `codegen-units` | Profile-level setting in `Cargo.toml` | Normal project tuning |
| Higher N | More parallel backend work, often faster compile | Debug/dev, large crates |
| `codegen-units=1` | More whole-crate optimization opportunity, slower builds | Release/perf-sensitive binaries |
| LTO | Lets backend optimize across CGU/crate boundaries | See [12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md) and [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md) |
| Incremental | CGUs become reusable work products | See [14](14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md) |

Cargo defaults differ by profile and have changed over time; debug profiles tend
to favor incremental parallelism, release profiles tend to use fewer CGUs for
better optimization. Check the current Cargo/rustc docs before turning this into
policy.

---

## Dynamic Dispatch and Vtables

`dyn Trait` is the other path: type erasure at the call site with dynamic
method dispatch. A trait object pointer is a fat pointer: data pointer plus
metadata pointer. For `dyn Trait`, that metadata is a vtable pointer. The fact of
dynamic dispatch is stable; the byte layout of the vtable is not.

```
&dyn Draw
data pointer   -----> concrete value: Button
vtable pointer -----> table below
                     |
                     v
             +-----------------------------------+
             | drop-in-place fn                  |
             | size / align                      |
             | Draw::draw method fn              |
             | more dyn-compatible method fns    |
             +-----------------------------------+
```

| Choice | Dispatch | Strength | Cost |
|--------|----------|----------|------|
| `fn f<T: Trait>(x: T)` | Static, monomorphized | Inlining and concrete layout | Code bloat per type |
| `fn f(x: &dyn Trait)` | Dynamic via vtable | Smaller code, heterogeneous values | Indirect call, object-safety limits |
| `Box<dyn Trait>` | Dynamic plus heap ownership | Type-erased ownership | Allocation and vtable call |

Dyn-compatibility, historically "object safety," is a language rule owned by the
Reference. Vtable construction and layout are rustc implementation details.

---

## Symbols, Mangling, and Concrete Trace

Monomorphized instances need distinct linker symbols. rustc mangles names so
instances, crates, hashes, and namespaces do not collide. The legacy and `v0`
name mangling schemes are compiler/codegen concerns; demangling is the user
workflow. The spelling of an internal symbol is not a semantic contract.

| Task | Command |
|------|---------|
| Build optimized artifact | `cargo build --release` |
| Force one CGU for a release experiment | `cargo rustc --release -- -C codegen-units=1` |
| Request v0 mangling | `cargo rustc --release -- -C symbol-mangling-version=v0` |
| Emit object files | `cargo rustc --release -- --emit=obj` |
| Inspect symbols | `llvm-nm -C target\release\deps\*.obj` or `dumpbin /symbols ...` on MSVC |
| Demangle outside the toolchain | `rustfilt` from the ecosystem |

```rust
trait Draw { fn draw(&self); }
struct Button;
impl Draw for Button { fn draw(&self) {} }

fn static_call<T: Draw>(x: &T) { x.draw(); }
fn dynamic_call(x: &dyn Draw) { x.draw(); }

pub fn demo() {
    let b = Button;
    static_call(&b);       // monomorphized static dispatch
    dynamic_call(&b);      // fat pointer + vtable dispatch
}
```

A symbol trace will show concrete instances and glue, but do not over-read exact
names. They are a diagnostic window into rustc and the linker, not a stable API.

---

## Old World -> New World Bridge

| Old world | Rust analogue | Difference that matters |
|-----------|---------------|-------------------------|
| C++ templates | Monomorphized Rust generics | Closest match: per-type code, bloat, compile-time cost |
| .NET generics | Rust generics | Rust is AOT and usually specializes all concrete instances; no CLR/JIT sharing model |
| Java erasure | `dyn Trait` in spirit, not mechanics | Rust dynamic dispatch is explicit and uses trait objects, not erased generic bodies |
| C++ vtable in object header | Rust `dyn Trait` fat pointer metadata | The pointer carries the vtable; the object need not contain a vptr header |
| COM vtable/interface pointer | Rust trait object call table | Similar indirect-call intuition; Rust adds ownership/drop/layout constraints |
| RyuJIT backend work | rustc CGUs into LLVM/backend jobs | Partitioning happens before backend optimization and interacts with incremental/LTO |

The central trade-off is familiar: static specialization buys speed and type
layout knowledge; dynamic dispatch buys code sharing and late binding.

---

## Stability Boundary

| Stable contract | Internal / version-sensitive |
|-----------------|------------------------------|
| Generic semantics and trait bounds | Mono item collector algorithms |
| Trait object semantics and dyn-compatibility rules | Vtable byte layout and entry order |
| `-C codegen-units` stable flag | CGU partitioning heuristics |
| `-C symbol-mangling-version` stable flag | Exact symbol hashes/spelling as program interface |
| Observable drop behavior | Drop glue generation details |
| Backend selection flags where stabilized | LLVM/Cranelift/GCC internal optimization choices |

Do not build FFI or serialization protocols on rustc's current vtable layout.
The stable fact is that trait objects dispatch dynamically according to the
language rules.

---

## Decision Cheat Sheet

| Question | Answer | Authority |
|----------|--------|-----------|
| Need maximum runtime speed for generic code? | Prefer static dispatch; profile code size | language + rustc |
| Need heterogeneous values behind one interface? | Use `dyn Trait` if the trait is dyn-compatible | language Reference |
| Binary too large from generics? | Inspect monomorphization pressure; consider `dyn`, factoring, LTO, profile settings | rustc/Cargo/backends |
| Release perf more important than build time? | Try `codegen-units=1` and LTO, then measure | Cargo/rustc/backends |
| Need faster dev builds? | Higher CGUs and incremental are usually appropriate | Cargo/rustc |
| Need symbol inspection? | Use `cargo rustc -- --emit=obj`, `llvm-nm`, `dumpbin`, `rustfilt` | rustc/ecosystem tools |
| Need a stable vtable ABI? | Rust does not provide one for trait objects | language/rustc boundary |

---

## Common Confusion Points

| Confusion | Correction |
|-----------|------------|
| "Rust generics are like Java generics." | Rust usually monomorphizes; Java mostly erases. |
| "`dyn Trait` is slower Rust generics." | It is a different representation and dispatch model, chosen explicitly. |
| "A vtable layout description is an ABI guarantee." | No. Layout is rustc-internal and can change. |
| "More codegen units always make builds better." | More CGUs can reduce optimization quality; measure per profile. |
| "Polymorphization removes generic bloat." | Rustc removed that optimization; inspect actual mono items and backend/linker output instead. |
| "Cargo decides monomorphization." | Cargo schedules crates and passes flags; rustc collects and emits mono items. |

---

## Primary Sources

| Source | Use it for |
|--------|------------|
| rustc-dev-guide: "Monomorphization" and "Collector" | Mono items, instances, reachability |
| rustc-dev-guide: "Codegen units" and partitioning notes | CGU work products and heuristics |
| rustc-dev-guide: "Symbol mangling" | Legacy and v0 mangling implementation |
| rustc-dev-guide: dyn/vtable codegen material | Trait object lowering and glue |
| The Rust Reference: generics | Stable generic semantics |
| The Rust Reference: trait objects and dyn compatibility | Stable trait object rules |
| rustc book: codegen options | `-C codegen-units`, `-C symbol-mangling-version`, backend flags |
| Cargo Book: profiles | Profile-level `codegen-units`, LTO, incremental settings |

*Cross-links:* [00](00-OVERVIEW.md) for the end-to-end compiler map; [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md) for optimized MIR input; [08](08-TRAIT-SOLVING-COHERENCE-AND-NEXT-SOLVER.md) for trait selection; [12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md), [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md), and [14](14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md) for backend, artifact, and cache consequences.