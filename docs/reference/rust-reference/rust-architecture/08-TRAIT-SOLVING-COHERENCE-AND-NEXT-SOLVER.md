---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:trait-solving-coherence
kind: guide
module: rust-architecture
section: rust-architecture
title: Trait Solving, Coherence, and the Next-Generation Solver
status: source-custody
source_custody: partial
current_path: rust-architecture/08-TRAIT-SOLVING-COHERENCE-AND-NEXT-SOLVER.md
canonical_path: rust-architecture/08-TRAIT-SOLVING-COHERENCE-AND-NEXT-SOLVER.md
backsource_ids: [proof-backfill:rust-architecture:08-trait-solving-coherence]
concepts: [trait solving, obligations, canonicalization, selection, coherence, specialization]
root_concepts: [trait solving]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Trait Solving, Coherence, and the Next-Generation Solver

## The Big Picture

Trait solving is rustc's constraint engine for questions like "does `T: Trait`
hold?", "what is `<T as Iterator>::Item`?", and "is this type well-formed under
these where-clauses?" Type checking emits obligations; the solver proves or
rejects them against impls, trait definitions, where-bounds, auto traits, and
built-in rules. The Rust Reference owns trait semantics, coherence, and orphan
rules. rustc's solver architecture, canonicalization, fulfillment contexts,
selection code, and next-solver rollout are internal and version-sensitive.

```
+===========================================================================+
|                           TRAIT SOLVING PIPELINE                          |
|                                                                           |
|  typeck body [07]                                                         |
|      | emits obligations                                                  |
|      v                                                                    |
|  goals / predicates                                                       |
|      | TraitRef, projection, well-formedness, outlives                    |
|      v                                                                    |
|  canonicalize for caching [03]                                            |
|      | replace inference vars with canonical placeholders                 |
|      v                                                                    |
|  solve                                                                    |
|      | old solver: assemble candidates -> winnow -> confirm               |
|      | next solver: logic-programming-style engine (-Znext-solver)        |
|      v                                                                    |
|  result                                                                   |
|      | impl selected, associated type normalized, ambiguity, or error     |
+===========================================================================+
```

If you think in Horn clauses, this is familiar territory. Rust adds coherence,
associated types, lifetimes, auto traits, and a strict crate ownership model so
separate compilation across the ecosystem remains decidable.

---

## Goals and Obligations

An **obligation** is a proof request registered during type checking or another
compiler phase. A **goal** is the solver-facing form of that request. The names
shift across rustc versions, but the categories are stable enough to orient the
architecture.

```
+-----------------------------------------------------------------------+
| COMMON SOLVER QUESTIONS                                               |
|                                                                       |
| Trait goal:        Vec<u8>: Clone                                     |
| Projection goal:   <I as Iterator>::Item == u8                        |
| WF goal:           type Foo<T> is well-formed under its bounds        |
| Outlives goal:     T: 'a or 'a: 'b                                    |
| Auto-trait goal:   T: Send / Sync                                     |
| Const/type goal:   const generic predicates where applicable          |
+-----------------------------------------------------------------------+
```

| Predicate kind | Example | Why rustc asks |
|---|---|---|
| trait ref | `T: Display` | method calls, bounds, derives, operators |
| projection | `<T as Trait>::Assoc` | normalize associated types |
| well-formedness | `&'a T`, `Struct<T>` | ensure type and predicates make sense |
| outlives | `T: 'a` | lifetime and variance constraints |
| auto trait | `T: Send` | thread-safety and marker behavior |

Typeck ([07](07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md)) produces the
bulk of these, but borrow checking, MIR construction, and codegen can also rely
on solved trait facts.

---

## Canonicalization and Query-Friendly Solving

Raw inference variables are local to one `InferCtxt`, so caching a solver result
against them directly would be useless. rustc canonicalizes goals by replacing
local inference variables with canonical placeholders, solves the canonical
problem, and re-instantiates the result in the caller's context.

```
+===========================================================================+
|                         CANONICALIZATION SKETCH                           |
|                                                                           |
|  caller context:                                                          |
|     ?T: Iterator, prove < ?T as Iterator >::Item: Clone                   |
|                                                                           |
|        canonicalize                                                       |
|             |                                                             |
|             v                                                             |
|  canonical goal:                                                          |
|     forall<C0> { <C0 as Iterator>::Item: Clone }                          |
|                                                                           |
|        solve / cache in query system [03]                                 |
|             |                                                             |
|             v                                                             |
|  instantiate result back into caller                                      |
+===========================================================================+
```

Canonicalization is an internal mechanism, not a language feature. Its point is
engineering: make logically equivalent questions look identical enough for the
query system to cache and for recursive solving to stay tractable.

---

## Legacy Selection: Candidates, Winnowing, Confirmation

The legacy solver is often described as selection plus fulfillment. It assembles
candidate ways an obligation might hold, eliminates impossible or less-specific
choices, then confirms the selected candidate and emits any nested obligations.
The exact candidate set and implementation names change, but the shape is useful.

```
+-----------------------------------------------------------------------+
| OLD SOLVER SHAPE                                                      |
|                                                                       |
| obligation: T: Trait                                                  |
|       |                                                               |
|       v                                                               |
| assemble candidates                                                   |
|   - impl candidates: impl Trait for Type                              |
|   - where-bound candidates: where T: Trait                            |
|   - builtin candidates: tuples, fn pointers, closures, unsizing       |
|   - auto-trait candidates: Send/Sync-style structural reasoning       |
|       |                                                               |
|       v                                                               |
| winnow: remove impossible/ambiguous candidates                        |
|       |                                                               |
|       v                                                               |
| confirm: select impl, instantiate vars, emit nested obligations       |
+-----------------------------------------------------------------------+
```

Recursive obligations can overflow, so rustc has recursion and overflow handling.
When diagnostics report an overflow evaluating a requirement, the compiler is not
"running forever"; it hit a deliberate bound in this proof search.

---

## Coherence and the Orphan Rule

Coherence is the language-level guarantee that, for a given trait/type pair,
there is at most one applicable impl. That guarantee lets method dispatch,
associated type normalization, and generic reasoning avoid "which impl did you
mean?" ambiguity across crates.

```
+===========================================================================+
|                         WHY THE ORPHAN RULE EXISTS                        |
|                                                                           |
|  crate std owns:       trait Display, type Vec<T>                         |
|                                                                           |
|  your crate owns:      type MyVec<T>                                      |
|                                                                           |
|  allowed:              impl Display for MyVec<T>       (you own type)     |
|  allowed:              impl MyTrait for Vec<T>         (you own trait)    |
|  rejected:             impl Display for Vec<T>         (own neither)      |
|                                                                           |
|  Result: no third crate can introduce a competing global impl             |
+===========================================================================+
```

```rust
use std::fmt::{self, Display};

// Rejected in your crate: both Display and Vec are foreign.
impl<T: Display> Display for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vec")
    }
}

// Typical error: E0117, only traits defined in the current crate can be
// implemented for arbitrary types. Use a newtype wrapper instead.
```

Coherence and orphan rules are **not** mere rustc implementation details; they
are part of Rust's language contract, documented in the Reference and RFCs. The
particular overlap-checking algorithms and solver entry points are internal.
Negative impls, especially for auto traits, participate in this space by saying
that a type explicitly does **not** implement a trait such as `Send`.
User-written negative impls remain nightly-only behind
`#![feature(negative_impls)]`; stable code can be affected by negative impls
provided by the toolchain but cannot generally declare its own.

---

## Associated Types, Projection, and Normalization

Associated types make trait solving more than Boolean membership. A goal may need
to normalize a projection like `<I as Iterator>::Item` before ordinary type
checking can continue.

```
+-----------------------------------------------------------------------+
| PROJECTION EXAMPLE                                                    |
|                                                                       |
| trait Iterator { type Item; }                                         |
| impl Iterator for Bytes { type Item = u8; }                           |
|                                                                       |
| Goal: <Bytes as Iterator>::Item == ?X                                 |
|       |                                                               |
|       v                                                               |
| select impl Iterator for Bytes                                        |
|       |                                                               |
|       v                                                               |
| normalize ?X = u8                                                     |
+-----------------------------------------------------------------------+
```

```rust
fn first<I>(mut iter: I) -> Option<I::Item>
where
    I: Iterator,
{
    iter.next()
}

fn bytes() {
    let xs = vec![1u8, 2, 3];
    let x: Option<u8> = first(xs.into_iter());
    let _ = x;
}
```

The solver proves `IntoIter<u8>: Iterator` and normalizes the associated `Item`
to `u8`, while typeck records the resulting method and expression types.

---

## Specialization, Auto Traits, and the Next Solver

Specialization deserves a bright red caveat: `specialization` and
`min_specialization` are nightly-only, unstable features. General specialization
has known soundness hazards; do not present it as stable Rust design guidance.
On stable Rust, assume non-overlapping impls except for the stable language
features explicitly documented by the Reference.

The next-generation trait solver is also evolving. rustc has an in-progress
`-Znext-solver` implementation intended to replace legacy selection/fulfillment,
improve caching, align more closely with the logic-programming model, and handle
classes of goals the old solver struggles with. Its use has been rolled out
incrementally. On rustc 1.99.0-nightly, `-Znext-solver=coherence` is the default;
`-Znext-solver=globally` (or bare `-Znext-solver`) opts the whole compilation
into it, and `-Znext-solver=no` selects the legacy path. Those values and rollout
points are internal and version-sensitive, not a public contract.

```powershell
# UNSTABLE: pass the rustc flag through Cargo to all compiled target crates.
$env:RUSTFLAGS = "-Znext-solver=globally"
cargo +nightly check
Remove-Item Env:RUSTFLAGS

# Stable baseline for trait/coherence diagnostics.
cargo check
rustc --explain E0117
```

Chalk is relevant historical context: it formulated Rust trait solving in a more
explicit logic-programming style and informed the next solver. Treat Chalk as a
design influence and experiment, not "the shipping trait solver" inside rustc.

---

## Old World -> New World Bridge

Trait solving is closest to Haskell type-class resolution plus associated type
normalization, with Rust's crate coherence constraints bolted on as a first-class
language rule. It also resembles overload resolution, but the candidate universe
comes from impls and where-clauses rather than arbitrary overload sets. Compared
with C# interface dispatch, Rust is stricter: another crate cannot retroactively
adapt `Vec<T>` to `Display`, because that would make global reasoning depend on
which crates happen to be linked. The orphan rule is the impl-world analogue of
preventing duplicate global registrations, with an ODR flavor but at the trait
implementation level.

---

## Decision Cheat Sheet

| Question | Answer | Authority |
|---|---|---|
| "Who asks trait questions?" | Mostly typeck, with later compiler phases consuming facts | rustc |
| "What is stable: solver algorithm or trait semantics?" | Trait semantics, coherence, orphan rules | Reference/RFCs |
| "Can two crates add competing impls?" | No; orphan/coherence rules prevent it | language |
| "Is canonicalization a language feature?" | No; it is a cache/query mechanism | rustc internal |
| "Can I use specialization on stable?" | No; specialization/min_specialization are unstable nightly features | rustc/lang feature gate |
| "Is `-Znext-solver` stable?" | No; version-sensitive compiler implementation flag | rustc internal |
| "What should library authors do?" | Design around explicit traits, newtypes for foreign pairs, and stable bounds | language + ecosystem practice |

---

## Common Confusion Points

- **Traits are not just interfaces.** They carry associated types, generic
  predicates, blanket impls, auto traits, and coherence constraints.
- **The orphan rule is a feature, not bureaucracy.** It preserves global
  uniqueness of impl selection across independently compiled crates.
- **Projection is solving too.** `<T as Trait>::Assoc` often requires selecting
  an impl before a type can be known.
- **Blanket impls are powerful but consume coherence space.** `impl<T> Trait for
  T where ...` may prevent more specific impls unless specialization is involved,
  and specialization is not stable.
- **Chalk is not rustc's public API.** It shaped the model; rustc's active solver
  implementation is internal and evolving.
- **Next solver status is version-sensitive.** Do not assume a given nightly flag
  behavior or rollout point across releases.

---

## Primary Sources

- **rustc-dev-guide** — trait solving chapters covering goals/obligations,
  canonicalization, selection, coherence, and the next-generation trait solver.
- **The Rust Reference** — traits, trait bounds, implementations, coherence,
  orphan rules, associated items, and auto traits.
- **rust-lang RFCs** — especially coherence/orphan-rule RFCs and trait-system
  design RFCs.
- **rust-lang/chalk repository** — historical and experimental logic-programming
  context for Rust trait solving.
- **rust-lang/rust repository** — current rustc implementation and feature gates
  for solver behavior, specialization, and diagnostics.

*Cross-links:* start with [00](00-OVERVIEW.md). This guide receives obligations
from [07](07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md), depends on HIR
identity from [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md), participates in
query caching from [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md), and feeds
MIR/codegen decisions in [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md)
and [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md). Diagnostics surface
through [15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md). For language-side
trait rules, see `../rust-language/` where it exists.
