---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:mir-construction-transforms
kind: guide
module: rust-architecture
section: rust-architecture
title: MIR - Construction, Transforms, and Interpretation (CTFE)
status: source-custody
source_custody: partial
current_path: rust-architecture/09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md
canonical_path: rust-architecture/09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md
backsource_ids: [proof-backfill:rust-architecture:09-mir-construction-transforms]
concepts: [mir, control flow graph, drop elaboration, mir optimization, const evaluation, ctfe]
root_concepts: [mir]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# MIR - Construction, Transforms, and Interpretation (CTFE)

## The Big Picture

MIR is rustc's mid-level IR: a deliberately small, CFG-shaped representation
between typed Rust and backend IR. It is not the Rust language and it is not a
stable compiler API. It is where rustc makes ownership, drops, control flow, and
compile-time execution explicit enough for borrow checking, optimization, CTFE,
and code generation to share one substrate.

```
+===========================================================================+
|                         MIR IN THE RUSTC PIPELINE                         |
|                                                                           |
|  HIR + typeck results                                                     |
|      [06] lowering identity, [07] types/regions/obligations               |
+---------------------------------------------------------------------------+
|             |                                                             |
|             v                                                             |
|  THIR - typed high-level expression tree                                  |
|             |                                                             |
|             v                                                             |
|  MIR build - locals, places, rvalues, blocks, terminators                 |
|             +--> borrowck input [10]      +--> CTFE interpreter           |
|             |    flow-sensitive loans     |    const/static/len/const fn  |
|             |                             |                               |
|             +--> MIR transforms/opts      +--> compile-time constants     |
|             |                                                             |
|             v                                                             |
|  mono + codegen units [11] -> backend IR / LLVM / other backends [12]     |
+===========================================================================+
```

Authority boundary: the Rust Reference owns drop semantics and constant-eval
rules; rustc owns MIR and its passes; Cargo invokes rustc but does not define
MIR; rustup selects the toolchain; std supplies library and lang-item surface;
LLVM/backends consume lowered code; ecosystem tools such as Miri build on the
same internal ideas without making them stable contracts.

---

## What MIR Is

MIR is a function body as basic blocks. Each block has zero or more statements
and exactly one terminator. That is the point: almost everything complex in Rust
syntax is lowered away before the analyses that need exact control flow run.

```
+-----------------------+
| fn body               |
|  locals: _0, _1, ...  |
+-----------+-----------+
            |
            v
+-----------------------+      +-----------------------+
| bb0                   | ---> | bb1                   |
|  statements           |      |  statements           |
|  terminator           |      |  terminator           |
+-----------+-----------+      +-----------+-----------+
            |                              |
            v                              v
+-----------------------+      +-----------------------+
| bb2                   |      | cleanup / unwind edge |
|  ...                  |      |  drop then resume     |
+-----------------------+      +-----------------------+
```

| MIR concept | Shape | Why it matters |
|-------------|-------|----------------|
| Local | `_0` return place, `_1..` args, compiler temps | Gives every value a concrete storage slot |
| Place | local plus projections: field, index, deref, downcast | The unit of move, borrow, assignment, and drop |
| Rvalue | value-producing expression: use, ref, aggregate, binary op | Expression complexity is normalized |
| Statement | assignment, storage live/dead, fake reads, etc. | Side effects short of control transfer |
| Terminator | `goto`, `switchInt`, `call`, `drop`, `assert`, `return`, unwind | All control flow is explicit |

Treat the statement and terminator set as rustc internals. The stable contract is
not "MIR has this opcode"; it is that Rust programs observe the Reference's
semantics.

---

## Construction: HIR -> THIR -> MIR

MIR is built after the front end has names, types, and most obligations. The
important transition is from a language-shaped tree to a control-flow-shaped
body. See [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md) for HIR identity
and [07](07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md) for type and
region constraints feeding this point.

```
+------------------+      +------------------+      +------------------+
| typed HIR        | ---> | THIR             | ---> | MIR build        |
| item identity    |      | typed expr tree  |      | blocks/locals    |
| typeck tables    |      | pattern structure|      | explicit control |
+------------------+      +------------------+      +------------------+
                                   |                         |
                                   v                         v
                           match decision tree        `switchInt` / edges
                           overloaded ops resolved    calls / drops / temps
```

| Source construct | MIR construction effect |
|------------------|-------------------------|
| `match` / `if let` | Pattern tests become decision trees and branches, often `switchInt` |
| temporaries | Become locals with explicit storage lifetime markers |
| calls and operators | Become normalized calls or primitive rvalues after type resolution |
| early return / `?` | Becomes explicit branching to return or cleanup paths |
| logical scope exit | Initially records drops that later passes make concrete |

This is why going HIR directly to LLVM would be the wrong seam. LLVM does not
know Rust moves, partial initialization, drop obligations, or borrow diagnostics.
MIR is the compiler-owned level where those facts still exist but syntax no
longer obscures control flow.

---

## Drop Elaboration: Rust RAII Made Concrete

Drop semantics are stable language behavior; drop elaboration is rustc's current
implementation strategy. The MIR builder introduces logical drops. The drop
elaboration pass turns them into conditional, path-sensitive drops using drop
flags and move/initialization state.

```
+---------------------+      +---------------------+      +------------------+
| logical MIR         | ---> | move/init analysis  | ---> | elaborated MIR   |
| "drop x here"       |      | is place initialized|      | if flag: drop x  |
| cleanup edges       |      | on this path?       |      | update flags     |
+----------+----------+      +----------+----------+      +---------+--------+
           |                            |                           |
           v                            v                           v
  source-level RAII             conditional moves             concrete cleanup
```

| Drop concern | Rust-specific consequence |
|--------------|---------------------------|
| Conditional initialization | A value initialized only on one branch must be dropped only on that branch |
| Moves | Moving a place suppresses the old owner's later drop |
| Partial moves | Fields can have independent initialized state in supported cases |
| Panic/unwind | Cleanup blocks run drops along unwinding edges; panic strategy lives lower, see [16](16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md) |
| Drop glue | Type-directed destructor paths later feed monomorphization and codegen, see [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md) |

This is one of Rust's signature compiler obligations. Deterministic destruction
is not bolted on at codegen; it is represented and checked in the middle end.

---

## MIR Transforms and Optimizations

After construction, rustc runs MIR passes that simplify, validate, prepare, and
sometimes optimize the body. The exact pass set and order are internal and
version-sensitive. Read dumps as a trace of one compiler build, not as a spec.

| Pass family | Typical purpose | Stability caveat |
|-------------|-----------------|------------------|
| CFG simplification | Merge blocks, remove unreachable edges, normalize terminators | Internal pass names/order |
| Inlining | Inline eligible MIR bodies before backend IR | Heuristics change by release/profile |
| Const propagation / value simplification | Fold known values, simplify branches/asserts | Conservative; LLVM still does heavy lifting |
| Dead code/storage cleanup | Reduce irrelevant statements and temps | Often diagnostic/codegen motivated |
| Match branch simplification | Collapse impossible or redundant branches after typing | Implementation detail |
| Validation and borrowck support | Preserve invariants for [10](10-BORROW-CHECKING-NLL-AND-POLONIUS.md) and diagnostics | Not a user contract |

```
MIR build
   |
   +--> analysis-prep MIR ---------------> borrowck [10]
   |
   +--> optimized MIR -------------------> mono/codegen [11][12]
           simplify cfg, inline, fold, clean
```

MIR optimization is intentionally not the whole optimization story. rustc uses
MIR to reduce semantic noise and LLVM IR volume; LLVM and other backends own most
machine-level optimization, register allocation, and instruction selection. See
[12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md).

---

## CTFE and the MIR Interpreter

Compile-time function evaluation evaluates selected Rust computations by
interpreting MIR in an abstract machine. `const`, `static` initializers, array
lengths, const generics where allowed, and `const fn` bodies all route through
this machinery when they need a compile-time value.

```
+----------------------+        +-----------------------------+
| const context        | -----> | MIR interpreter             |
| const/static/len     |        | abstract allocations        |
| const fn invocation  |        | provenance-aware pointers   |
+----------------------+        | checked operations          |
                                +-----------------------------+
                                               |
                                               v
          in-compiler CTFE result                         standalone Miri tool
          feeds type/layout/codegen                       extra UB diagnostics
```

| Term | Precise role |
|------|--------------|
| CTFE | rustc's in-compiler use of the interpreter to obtain required compile-time values |
| Miri engine | The MIR interpreter architecture rustc uses and the Miri tool extends |
| Miri tool | Ecosystem tool with additional UB checks and a user-facing command surface |
| Abstract machine | Allocation/provenance model for what a compile-time pointer/value means |
| Limits | Const evaluation is bounded and intentionally rejects operations outside const rules |

The observable result of accepted const evaluation is a stable language matter;
the interpreter data structures, provenance model details, diagnostics, and dump
formats are implementation details.

---

## Concrete Trace

`--emit=mir` is available as a rustc emit kind for inspection. The output format
is not a stable interface. Cargo does not have a first-class stable "show me MIR"
artifact; use direct `rustc` or `cargo rustc` only as an experiment. Nightly `-Z`
flags are explicitly UNSTABLE.

| Task | Command |
|------|---------|
| Emit MIR from one file | `rustc --edition 2021 --crate-type lib --emit=mir src\lib.rs` |
| Dump all MIR stages | `rustc +nightly --crate-type=lib -Z dump-mir=all src\lib.rs` - UNSTABLE |
| Pretty-print MIR | `rustc +nightly --crate-type=lib -Z unpretty=mir src\lib.rs` - UNSTABLE |
| See the build invoking rustc | `cargo build -v` |

```rust
pub fn choose(x: bool) -> i32 {
    let a = 10;
    if x { a + 1 } else { a - 1 }
}

pub const N: usize = {
    let x = 3usize;
    x * 2 + 1
};
```

A simplified MIR shape for `choose` is enough to see the point:

```text
bb0: switchInt(_1) -> [0: bb2, otherwise: bb1]

bb1: _0 = Add(const 10_i32, const 1_i32)
     goto -> bb3

bb2: _0 = Sub(const 10_i32, const 1_i32)
     goto -> bb3

bb3: return
```

The `const N` initializer is not "macro-expanded into 7". rustc evaluates the
MIR for the const context and records the resulting compile-time value.

---

## Old World -> New World Bridge

| Old world | Rustc MIR analogue | Important difference |
|-----------|--------------------|----------------------|
| Bound tree -> lowered CFG | HIR/THIR -> MIR | MIR still carries Rust ownership/drop facts |
| SSA-ish mid-level IR | MIR locals/places plus explicit CFG | MIR is not pure SSA; places model storage and moves |
| C++ RAII destructor insertion | Drop elaboration with flags and cleanup edges | Rust tracks moves/initialization statically and makes drops path-sensitive |
| C# constant folding | CTFE | CTFE is a real interpreter for allowed Rust, not only algebraic folding |
| LLVM IR | Backend input after rustc lowering | Too low-level for borrowck and Rust diagnostics |

The mental model is a compiler middle end tuned for Rust's ownership semantics:
low enough for dataflow, high enough that destruction and borrowing are still
first-class.

---

## Decision Cheat Sheet

| Question | Answer | Authority |
|----------|--------|-----------|
| Need to understand why borrowck accepts/rejects a path? | Inspect MIR conceptually, then read [10](10-BORROW-CHECKING-NLL-AND-POLONIUS.md) | rustc internals |
| Need a stable guarantee about drop order? | Read the Reference destructors/drop scopes | language |
| Need to reduce generated LLVM IR? | Look at MIR inlining/cleanup only as a clue; tune code/profile/LTO | rustc/backends |
| Need compile-time execution behavior? | Read const-eval rules; use MIR dumps only for debugging | language + rustc |
| Need to see current MIR? | `rustc --emit=mir`; nightly `-Z dump-mir` for deeper traces | rustc, unstable format |
| Need build integration? | Cargo invokes rustc; it does not define MIR | Cargo |

---

## Common Confusion Points

| Confusion | Correction |
|-----------|------------|
| "MIR is Rust's IL." | It is an internal rustc IR, not a stable runtime format like IL or JVM bytecode. |
| "MIR pass order is documented, so I can depend on it." | No. Pass names, order, and dumps are version-sensitive implementation details. |
| "Drop order is an implementation detail." | The user-observable drop semantics are stable; drop elaboration mechanics are internal. |
| "CTFE is the same thing as the Miri command." | CTFE is rustc's compile-time use; Miri is an ecosystem tool extending the interpreter with more checks. |
| "LLVM should handle this." | LLVM cannot enforce Rust move/drop/borrow semantics because those facts are erased before backend IR. |

---

## Primary Sources

| Source | Use it for |
|--------|------------|
| rustc-dev-guide: "The MIR" | MIR concepts, locals, places, rvalues, statements, terminators |
| rustc-dev-guide: "MIR construction" | HIR/THIR to MIR build and lowering details |
| rustc-dev-guide: "MIR passes and optimizations" | Current pass families and caveats |
| rustc-dev-guide: "Drop elaboration" | Drop flags, move paths, conditional drops |
| rustc-dev-guide: "Constant evaluation" and "Miri / the interpreter" | CTFE and interpreter architecture |
| RFC 1211 | Why MIR was introduced |
| The Rust Reference: destructors and constant evaluation | Stable language semantics |

*Cross-links:* [00](00-OVERVIEW.md) for the whole compiler map; [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md) and [07](07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md) for inputs; [10](10-BORROW-CHECKING-NLL-AND-POLONIUS.md), [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md), [12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md), and [16](16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md) for consumers and runtime-facing consequences.