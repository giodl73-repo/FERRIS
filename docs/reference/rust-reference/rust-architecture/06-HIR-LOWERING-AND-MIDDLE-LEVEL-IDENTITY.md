---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:hir-lowering-identity
kind: guide
module: rust-architecture
section: rust-architecture
title: HIR Lowering and Middle-Level Identity (DefId, HirId)
status: source-custody
source_custody: partial
current_path: rust-architecture/06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md
canonical_path: rust-architecture/06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md
backsource_ids: [mdloom-backfill:rust-architecture:06-hir-lowering-identity]
concepts: [hir, lowering, desugaring, defid, hirid, crate metadata]
root_concepts: [hir lowering]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# HIR Lowering and Middle-Level Identity (DefId, HirId)

## The Big Picture

HIR is rustc's first major **semantic working form** after parsing, macro
expansion, and name resolution. The Rust Reference owns the language behavior;
rustc owns this representation. The AST still resembles written syntax. HIR
(High-level IR) is more regular, partially desugared, owner-indexed, and wired
into the query system so type checking can run body-by-body. The important move
is not just syntax lowering; it is carrying definition identity established by
the definitions/resolution machinery into an owner-indexed tree and assigning
identity to nodes inside each definition body.

```
+===========================================================================+
|                       AST TO HIR: SHAPE + IDENTITY                        |
|                                                                           |
|  parse AST [04]                                                           |
|     |                                                                     |
|     v                                                                     |
|  macro expansion + hygiene + name resolution [05]                         |
|     |  all item/module paths resolved enough to name definitions          |
|     v                                                                     |
|  HIR lowering                                                             |
|     |                                                                     |
|     +--> desugar surface forms: for, if let, ?, ranges, closures, async   |
|     |                                                                     |
|     +--> consume definition identity: DefId / LocalDefId / DefPathHash    |
|     |                                                                     |
|     +--> assign intra-owner node identity: HirId                          |
|     |                                                                     |
|     v                                                                     |
|  per-owner HIR bodies                                                     |
|     |                                                                     |
|     v                                                                     |
|  typeck(def_id) [07] -> THIR -> MIR build [09]                            |
+===========================================================================+
```

Read HIR as the compiler's bound/lowered semantic tree, not as a stable public
AST. `DefId`, `HirId`, `LocalDefId`, HIR node layouts, query names, and dump
formats are rustc implementation details. The stable contract is the language
semantics: what `for`, `?`, `async`, method calls, coercions, and pattern forms
mean according to the Reference.

---

## Where HIR Sits in the Front End

HIR exists after the compiler has stopped treating names as mere tokens. Macro
expansion has produced the final expanded crate, hygiene has separated same-text
names with different scopes, and name resolution has attached paths to the
definitions they denote. See [05](05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md).
Only then can lowering consume the durable definition map and build owner-local
HIR identity around it.

```
+-------------------+     +----------------------+     +--------------------+
| AST               | --> | expanded + resolved  | --> | HIR                |
| close to syntax   |     | names/hygiene fixed  |     | regularized tree   |
+-------------------+     +----------------------+     +--------------------+
        |                            |                          |
        | syntax authority:          | name authority:           | rustc internal
        | Rust grammar               | rustc resolver             | semantic IR
```

| Layer | What it preserves | What it enables |
|---|---|---|
| AST | source syntax and spans | parsing diagnostics and macro input/output |
| expanded AST | post-macro program | final item/module universe |
| HIR | normalized semantic structure | per-body typeck, trait obligations, later THIR/MIR |

Cargo does not participate here; it has already invoked rustc for one crate. The
standard library is just upstream crates plus lang items from this point of view.
LLVM and other backends are not in the picture yet.

---

## Lowering as Controlled Desugaring

Lowering removes many surface conveniences before type checking, but not by
inventing new language semantics. It realizes the Reference's semantics in a form
rustc can process uniformly. Some desugarings are largely HIR-shaped; some, such
as `async` state machines, are completed across later THIR/MIR phases.

```
+-----------------------------------------------------------------------+
| SURFACE FORM               HIR / LATER COMPILER SHAPE                 |
|                                                                       |
| for x in y { ... }       -> loop + match + IntoIterator protocol      |
| if let P = e { ... }     -> match e { P => ..., _ => ... } shape      |
| while let P = e { ... }  -> loop around a match                       |
| expr?                    -> Try/FromResidual-shaped early-return path |
| a..b                     -> range constructor expression              |
| receiver.method(arg)     -> method-call node resolved against UFCS    |
| closure                  -> closure expression with captured body     |
| async/await              -> coroutine/future machinery, finished later|
+-----------------------------------------------------------------------+
```

A source-equivalent sketch for `?` looks like this. Exact HIR is version-sensitive;
this is the language-level idea.

```rust
fn read_count() -> Result<u32, std::num::ParseIntError> {
    let s = std::fs::read_to_string("count.txt")?;
    Ok(s.trim().parse::<u32>()?)
}

// Source-equivalent model, not a stable rustc dump:
fn read_count_desugared() -> Result<u32, std::num::ParseIntError> {
    let s = match std::fs::read_to_string("count.txt") {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    };
    let n = match s.trim().parse::<u32>() {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    };
    Ok(n)
}
```

For a `for` loop, the implementation shape is similarly protocol-oriented:
`IntoIterator::into_iter(iterable)` followed by a loop that matches on
`Iterator::next`. That makes type checking feed obligations into trait solving
([08](08-TRAIT-SOLVING-COHERENCE-AND-NEXT-SOLVER.md)) rather than special-case
all iterator syntax.

---

## Identity: DefId, LocalDefId, HirId

The compiler needs two kinds of identity because definitions and in-body nodes
live at different scales.

```
+============================================================================+
|                           RUSTC IDENTITY MODEL                             |
|                                                                            |
|  Cross-crate definition identity                                           |
|  DefId       = (CrateNum, DefIndex)                                        |
|  LocalDefId  = DefId known to be in the current crate                      |
|  DefPath     = logical path through the crate's definition tree            |
|  DefPathHash = stable-across-builds hash of that path                      |
|             | names items, impls, functions, types, consts                 |
|             v                                                              |
|  Intra-owner HIR node identity                                             |
|  HirId = (owner LocalDefId, ItemLocalId)                                   |
|             | names expressions, patterns, locals inside one HIR owner     |
+============================================================================+
```

| Identifier | Scope | Used for |
|---|---|---|
| `DefId` | any crate loaded in this compilation | referring to items across crate boundaries |
| `LocalDefId` | current crate only | indexing local definitions and query owners |
| `HirId` | a node inside one HIR owner | mapping expressions/patterns/locals to typeck results |
| `DefPathHash` | stable logical identity across builds | incremental reuse and metadata identity |

`DefId` is not literally stable across compiler sessions: `CrateNum` is assigned
when rustc loads crates, and crate numbers can be remapped. The engineered stable
piece is `DefPathHash`, derived from the definition path so incremental
compilation and downstream metadata can match "the same" item across builds. See
[14](14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md). This remains an
internal rustc contract, not a public API guarantee.

---

## Owners, Bodies, and On-Demand Type Checking

HIR is partitioned by **owners**: items and item-like definitions that own a HIR
subtree. Function, const, and static bodies are represented as bodies attached to
owners. That partition is a perfect fit for the query model described in
[03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md): rustc can ask for
`typeck(def_id)` of one body instead of eagerly checking the whole crate.

```
+--------------------------- crate HIR ----------------------------+
|                                                                  |
|  owner: fn parse()   ---> body: expressions, patterns, locals    |
|  owner: impl Foo     ---> items: methods, associated consts      |
|  owner: const N      ---> body: const expression                 |
|  owner: struct Bar   ---> fields, generics, where-clauses        |
|                                                                  |
+------------------------------------------------------------------+
        | query keys use LocalDefId / DefId
        v
+---------------------+     +---------------------+     +---------+
| typeck(parse) [07]  | --> | THIR construction   | --> | MIR [09]|
+---------------------+     +---------------------+     +---------+
```

This is the first point where the compiler's semantic identity and demand-driven
execution visibly reinforce each other. HIR owners give the query system stable
units. `HirId`s give later tables a way to say "the type of this expression" or
"the adjustment on this receiver" without pretending that every expression is a
cross-crate definition.

---

## Crate Metadata and Cross-Crate Identity

When rustc compiles a library crate, it serializes item information into crate
metadata (`rmeta`, often inside an `rlib`). Downstream crates load that metadata
rather than reparsing the upstream source. The metadata records enough definition
identity, types, predicates, visibility, and other item facts for downstream
queries to refer to upstream definitions.

```
+------------------+       emits        +---------------------------+
| upstream crate A | -----------------> | rlib / rmeta metadata [13]|
| DefIds + paths   |                    | DefPathHashes + item info |
+------------------+                    +---------------------------+
                                                   |
                                                   | loaded by rustc for crate B
                                                   v
                                      +-----------------------------+
                                      | downstream crate B          |
                                      | remaps CrateNum for A       |
                                      | refers to A::Item by DefId  |
                                      +-----------------------------+
```

The metadata format is unstable and version-locked to rustc. It is not a CLR
assembly metadata standard, even though the analogy is useful. See
[13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md) for artifacts, metadata,
linking, and debug info.

---

## Looking at HIR Dumps

Nightly rustc can print HIR. Treat the output as a microscope slide, not an API.
Use it to understand a specific compiler build.

```powershell
# UNSTABLE: requires nightly; -Z flags are rustc implementation debugging hooks.
rustup run nightly rustc -Z unpretty=hir src\lib.rs
rustup run nightly rustc -Z unpretty=hir-tree src\lib.rs

# Cargo can show the rustc command first, then you can reproduce it on nightly.
cargo build -v
```

The useful workflow is comparative: write a tiny source form, dump HIR, change
only the construct under study, dump again. Do not write tooling that depends on
node names or tree shapes from these dumps.

---

## Old World -> New World Bridge

If Roslyn is the old reference point, HIR is closest to an internal **bound and
lowered semantic tree** between syntax and IL emission. It is not Roslyn's public
API surface. `DefId` is analogous to a metadata token or symbol handle that can
refer across assemblies; `HirId` is closer to an internal bound-node key inside a
method body. `DefPathHash` plays the role of a stable content-addressed symbol
identity used to make incremental reuse and metadata loading robust when raw
session-local IDs move.

The important contrast: .NET assembly metadata is a stable interop surface. Rust
crate metadata is compiler-private. The language contract comes from the
Reference; rustc metadata and HIR are optimized implementation machinery.

---

## Decision Cheat Sheet

| Question | Answer | Authority |
|---|---|---|
| "Where does type checking primarily read from?" | HIR owners and bodies | rustc internal |
| "Can I depend on HIR dump shape?" | No; `-Z unpretty` is unstable | rustc |
| "What identifies an upstream item?" | `DefId` in-session, backed by metadata and DefPath identity | rustc internal |
| "What identifies an expression in a function body?" | `HirId` = owner plus local node id | rustc internal |
| "What is stable about `?` or `for`?" | The Reference semantics, not the exact desugaring nodes | language |
| "Where does HIR go next?" | Typeck results, THIR, then MIR | rustc internal |
| "Where do metadata details live?" | `rmeta`/`rlib` artifacts | rustc; see [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md) |

---

## Common Confusion Points

- **HIR is not the Rust language.** It is rustc's internal representation of a
  particular compiler version. The Reference owns semantics.
- **`DefId` is not globally stable by itself.** It contains a session-local crate
  number. DefPath-derived identity is the stable-across-builds ingredient rustc
  engineers for incremental and metadata.
- **`HirId` is not for cross-crate symbols.** It names nodes inside one HIR
  owner, such as expressions and patterns in a body.
- **Desugaring is not license to reason from arbitrary pseudo-code.** The
  compiler may implement a construct through HIR, THIR, MIR, lang items, and
  traits. The stable question is what program behavior the Reference specifies.
- **THIR is the bridge, not a replacement for HIR.** Type checking consumes HIR;
  after typeck, THIR carries typed structure toward MIR construction. See
  [07](07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md) and
  [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md).

---

## Primary Sources

- **rustc-dev-guide** — "The HIR", "Lowering AST to HIR", "Identifiers in the
  compiler", and "THIR".
- **The Rust Reference** — semantics for expressions, loops, pattern matching,
  the `?` operator, closures, async blocks, and method-call expressions.
- **rust-lang/rust repository** — rustc source for current HIR, lowering,
  metadata, and identity implementation.

*Cross-links:* start with the module map in
[00](00-OVERVIEW.md). HIR follows parsing/spans
([04](04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md)) and macro/name resolution
([05](05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md)); it feeds type
checking ([07](07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md)), trait
solving ([08](08-TRAIT-SOLVING-COHERENCE-AND-NEXT-SOLVER.md)), MIR
([09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md)), metadata
([13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md)), and incremental
([14](14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md)). For language-side
semantics, see `../rust-language/` where it exists.
