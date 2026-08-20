---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:type-inference-checking-regions
kind: guide
module: rust-architecture
section: rust-architecture
title: Type Inference, Checking, and Region Constraints
status: source-custody
source_custody: partial
current_path: rust-architecture/07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md
canonical_path: rust-architecture/07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md
backsource_ids: [proof-backfill:rust-architecture:07-type-inference-checking-regions]
concepts: [type inference, type checking, unification, coercion, method resolution, region constraints]
root_concepts: [type inference]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Type Inference, Checking, and Region Constraints

## The Big Picture

Rust type checking is local, demand-driven, and obligation-producing. rustc does
not run global Hindley-Milner over the crate. It type-checks one HIR owner body at
a time through a query such as `typeck(def_id)`, creates an inference context,
propagates expected types bidirectionally inside the body, resolves method calls
and coercions, registers trait obligations, and writes resolved facts into
`TypeckResults`. The Reference owns the language rules; `InferCtxt`, tables,
query names, and result layouts are rustc internals.

```
+===========================================================================+
|                         TYPECK FOR ONE HIR OWNER                          |
|                                                                           |
|  HIR owner/body [06]                                                      |
|      |                                                                    |
|      v                                                                    |
|  typeck(def_id) query [03]                                                |
|      |                                                                    |
|      +--> InferCtxt: ?T, ?I, ?F, region vars, const vars                  |
|      |                                                                    |
|      +--> expected-type propagation + unification                         |
|      |                                                                    |
|      +--> coercion sites: deref, unsize, fn item -> fn ptr, closure cases |
|      |                                                                    |
|      +--> method lookup: autoderef/autoref + inherent/trait candidates    |
|      |                                                                    |
|      +--> obligations sent to trait solver [08]                           |
|      |                                                                    |
|      +--> region constraints recorded for later MIR borrowck/NLL [10]     |
|      v                                                                    |
|  writeback: TypeckResults -> THIR -> MIR build [09]                       |
+===========================================================================+
```

The mental model is C# `var` and target-typed inference taken seriously inside a
function body, not ML-style principal types exported across module boundaries.
Function signatures, item types, trait interfaces, and public contracts remain
explicit.

---

## Function-Local Inference, Not Global HM

Rust deliberately limits inference across item boundaries. Inside a function or
const body, rustc can create inference variables and solve them from use sites.
Across function boundaries, signatures are the contract. This keeps APIs readable,
metadata compact, separate compilation tractable, and diagnostics bounded.

```
+------------------+        explicit         +-----------------------+
| item boundary    | ----------------------> | fn f(x: T) -> U       |
| crate metadata   |                         | trait method sigs     |
+------------------+                         +-----------------------+
          |
          | inside one body only
          v
+------------------+        inferred         +-----------------------+
| let v=Vec::new() |  ---------------------> | v: Vec<u8> after push |
| closure params   |                         | numeric fallback      |
+------------------+                         +-----------------------+
```

| Inference target | Scope | Notes |
|---|---|---|
| type variables `?T` | body-local | resolved by constraints and expected types |
| integer variables `?I` | body-local | fallback commonly chooses `i32` |
| float variables `?F` | body-local | fallback commonly chooses `f64` |
| region variables | body/typeck plus later MIR | constraints collected; NLL solves on MIR |
| const variables | body/type-level expressions | unified where const generics require it |

There is no let-generalization that exports inferred polymorphic types across a
function boundary. Genericity is declared in the item signature.

---

## InferCtxt, Unification, and Fallback

`InferCtxt` is the rustc context that owns inference variables, unification
state, snapshots, obligations, and error reporting hooks for a type-checking
operation. The implementation uses unification tables and related machinery for
types, integer/float variables, regions, and consts. It performs occurs checks
where recursive inference would otherwise create impossible types.

```
+-----------------------------------------------------------------------+
| INFERENCE VARIABLE LIFE CYCLE                                         |
|                                                                       |
| create ?T from missing annotation                                     |
|       |                                                               |
|       v                                                               |
| add facts: expected type, operator, receiver, return                  |
|       |                                                               |
|       v                                                               |
| unify facts; register trait/region obligations                        |
|       |                                                               |
|       v                                                               |
| resolve or fallback                                                   |
|       |                                                               |
|       +--> success: write concrete type into TypeckResults            |
|       +--> failure: E0282/E0283-style diagnostic [15]                 |
+-----------------------------------------------------------------------+
```

Fallback is intentionally narrow. Numeric literals may fall back when constraints
are insufficient. The never type `!` can coerce into other types at control-flow
boundaries, and unit `()` appears where block expressions carry no meaningful
value. Exact fallback and diagnostic behavior is version-sensitive; the language
rules and compatibility promises are the contract, not the internal tables.

```rust
fn demo() {
    let mut v = Vec::new();  // initially Vec<?T>
    v.push(1u8);             // back-propagates ?T = u8
    let _: Vec<u8> = v;

    let n = 0;               // integer fallback can make this i32
    let f = 0.0;             // float fallback can make this f64
}
```

When inference cannot resolve a variable, rustc reports "type annotations
needed" rather than guessing globally.

---

## Coercions Are Directed, Not Subtyping in Disguise

Rust has subtyping mainly around lifetimes and higher-ranked forms, but most
things engineers casually call "implicit conversion" are **coercions**. They are
directed, occur only at coercion sites, and are specified as language behavior.
The rustc coercion implementation is internal.

```
+===========================================================================+
|                            COERCION SITES                                 |
|                                                                           |
|  let x: Target = expr;                                                    |
|  function_argument(expr)                                                  |
|  return expr;                                                             |
|  array / tuple / block expected type propagation                          |
|                                                                           |
|  Examples:                                                                |
|  &Vec<T> -------- deref coercion --------> &[T]                           |
|  &[T; N] ------ unsizing -------------> &[T]                              |
|  &Concrete ---- unsizing -------------> &dyn Trait                        |
|  fn item ------ coercion -------------> fn pointer                        |
|  noncapturing closure ---------------> fn pointer                         |
+===========================================================================+
```

```rust
fn takes_slice(xs: &[u8]) -> usize { xs.len() }

fn coercions() {
    let v = vec![1u8, 2, 3];
    let n = takes_slice(&v);        // &Vec<u8> -> &[u8] by deref coercion

    let f: fn(i32) -> i32 = |x| x + 1; // noncapturing closure -> fn pointer
    let _ = (n, f(10));
}
```

Unsizing, including array-to-slice and concrete-to-`dyn Trait`, is tied to
language traits such as `Unsize`/`CoerceUnsized`, but user-facing stability is the
Reference behavior and stable library surface, not compiler internals.

---

## Method Resolution, Autoderef, and Obligations

Method-call syntax is a compact front-end over a substantial search. rustc builds
receiver candidates by repeatedly dereferencing the receiver type, then considers
autoref forms (`T`, `&T`, `&mut T`) at each step. It searches inherent methods
and trait methods, uses visibility and where-clauses, and pushes trait predicates
to the solver.

```
+-----------------------------------------------------------------------+
| METHOD CALL: receiver.method(arg)                                     |
|                                                                       |
| receiver type: Rc<Box<String>>                                        |
|       |                                                               |
|       v                                                               |
| autoderef chain: Rc<Box<String>> -> Box<String> -> String -> str      |
|       |                                                               |
|       v                                                               |
| candidate receiver forms: T, &T, &mut T at each step                  |
|       |                                                               |
|       v                                                               |
| inherent methods first, then trait methods in scope / prelude         |
|       |                                                               |
|       v                                                               |
| selected method + obligations -> trait solver [08]                    |
+-----------------------------------------------------------------------+
```

```rust
use std::rc::Rc;

fn method_lookup(s: Rc<Box<String>>) -> bool {
    s.starts_with("rust")
    // Rc<Box<String>> autoderef -> Box<String> -> String -> str,
    // then uses str::starts_with through an autoref receiver.
}
```

This is where Rust feels unlike both C# and C++. It resembles extension-method
receiver lookup in that receiver type and traits in scope matter, and resembles
overload resolution in that candidate selection can create obligations. But
coherence rules ([08](08-TRAIT-SOLVING-COHERENCE-AND-NEXT-SOLVER.md)) keep the
space more disciplined than open-ended ad hoc adaptation.

---

## Regions, Variance, and the NLL Boundary

Lifetimes are not runtime values. During type checking, rustc records region
relationships and outlives constraints that arise from references, variance,
where-clauses, reborrows, and subtyping. Historically much more region inference
lived in typeck. Modern rustc does the heavy NLL region inference in MIR borrow
checking; typeck produces the type facts and constraints that later phases need.

```
+----------------------+      produces       +-------------------------+
| HIR type checking    | ------------------> | TypeckResults           |
| - reference types    |                     | - resolved types        |
| - variance facts     |                     | - adjustments           |
| - outlives predicates|                     | - region constraints    |
+----------------------+                     +-------------------------+
             |                                             |
             v                                             v
       THIR / MIR build [09]                    MIR borrowck + NLL [10]
```

Variance decides how lifetime subtyping flows through type constructors. `&'a T`
is covariant in `'a` and `T`; `&'a mut T` is covariant in `'a` but invariant in
`T`; interior mutability forces additional invariance. The Reference owns the
subtyping and variance rules. rustc's region variable representation and NLL data
structures are implementation details.

---

## Writeback and Downstream Consumers

After inference variables are resolved, rustc writes the results back into tables
associated with HIR nodes. These `TypeckResults` include expression types,
method-call resolutions, adjustments/coercions, upvar captures, and obligations
or facts needed downstream. THIR construction reads these typed facts; MIR build
then lowers typed control flow into the middle IR.

```
+------------------+      resolve       +-----------------------+
| InferCtxt        | -----------------> | TypeckResults         |
| ?T, obligations  |                    | expr_ty(HirId)        |
| adjustments      |                    | adjustments(HirId)    |
+------------------+                    +-----------------------+
                                                |
                                                v
                                      +--------------------+
                                      | THIR -> MIR [09]   |
                                      +--------------------+
```

```powershell
# Stable: force ambiguity to surface in a small example.
cargo check

# Stable: disambiguate an inference variable explicitly.
# Example in source: Vec::<u8>::new() or "42".parse::<u32>()

# UNSTABLE/version-sensitive: show inferred types in one nightly compiler's HIR.
# Verified with rustc 1.99.0-nightly; the dump mode and format are not contracts.
rustup run nightly rustc --crate-type=lib -Z unpretty=hir,typed src\lib.rs
```

Diagnostics are covered in [15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md):
when rustc asks for a type annotation, it is usually protecting a local inference
boundary rather than failing to run a more global algorithm.

---

## Old World -> New World Bridge

For a .NET/Roslyn mind, Rust inference is much closer to C# `var`, target-typed
`new`, lambda parameter inference, and overload-resolution constraints than to
classic ML inference. Rust's twist is that method resolution, deref, coercion,
associated types, and trait obligations are integrated into the same local
checking pass. Coercions are analogous to implicit conversions only in user
experience; semantically they are a smaller, specified set of directed
adjustments. Autoderef feels like a cross between extension-method receiver
search and C++ overload resolution, except coherence and trait visibility make it
less ambient.

---

## Decision Cheat Sheet

| Question | Answer | Authority |
|---|---|---|
| "Does Rust infer function signatures?" | No; item signatures are explicit | language/rustc |
| "Where is inference scoped?" | Primarily one body at a time | rustc internal implementation of language behavior |
| "Are coercions the same as subtyping?" | No; coercions are directed at coercion sites | Reference |
| "Who resolves trait methods?" | typeck plus trait solver obligations | rustc internals; semantics in Reference |
| "Where are NLL lifetimes solved?" | MIR borrow checking, fed by typeck facts | rustc internal; see [10](10-BORROW-CHECKING-NLL-AND-POLONIUS.md) |
| "Can inference behavior change?" | Yes, when non-breaking; internals can change freely | language compatibility + rustc |
| "What consumes TypeckResults?" | THIR/MIR construction and diagnostics | rustc internal |

---

## Common Confusion Points

- **Rust inference is not global HM.** It is local, expected-type-driven, and
  bounded by explicit item signatures.
- **The trait solver is not optional decoration.** Method calls, operators,
  associated types, and many desugarings create obligations. See
  [08](08-TRAIT-SOLVING-COHERENCE-AND-NEXT-SOLVER.md).
- **Coercions are not arbitrary implicit conversions.** They happen at specific
  sites and include a known set of adjustments such as deref and unsizing.
- **Lifetimes are not carried as runtime tags.** Typeck collects region
  constraints; MIR borrowck/NLL performs the main flow-sensitive reasoning.
- **`InferCtxt` is not a public compiler API.** Its fields, unification tables,
  and query interactions are rustc implementation details.
- **Type annotations needed does not mean rustc is weak.** It usually means the
  local constraints intentionally do not determine a unique type.

---

## Primary Sources

- **rustc-dev-guide** — "Type checking", "Type inference", "Method lookup",
  "Variance", and the region-inference overview.
- **The Rust Reference** — type coercions, method-call expressions, subtyping,
  variance, trait bounds, and lifetime/outlives rules.
- **rust-lang/rust repository** — current rustc implementation of typeck,
  inference, adjustments, method lookup, and writeback.

*Cross-links:* start with [00](00-OVERVIEW.md). This guide consumes HIR from
[06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md), sends obligations to
[08](08-TRAIT-SOLVING-COHERENCE-AND-NEXT-SOLVER.md), feeds THIR/MIR in
[09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md), and hands region
work to borrow checking in [10](10-BORROW-CHECKING-NLL-AND-POLONIUS.md). It also
depends on query execution in [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md)
and diagnostics in [15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md). For
language-side type-system rules, see `../rust-language/` where it exists.
