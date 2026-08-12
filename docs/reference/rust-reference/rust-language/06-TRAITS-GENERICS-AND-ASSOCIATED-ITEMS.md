---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:traits-generics-and-associated-items
kind: guide
module: rust-language
section: languages
title: Traits, Generics, and Associated Items
status: source-custody
source_custody: partial
current_path: rust-language/06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md
canonical_path: rust-language/06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md
backsource_ids: [mdloom-backfill:rust-language:06-traits-generics-and-associated-items]
concepts: [traits, generics, bounds, where clauses, associated types, associated constants, coherence, orphan rule, blanket impls, supertraits]
root_concepts: [traits]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Traits, Generics, and Associated Items

Traits are Rust's primary mechanism for attaching shared behavior to types and
constraining generic code. They cover roles served elsewhere by interfaces,
type classes, operator protocols, and parts of abstract base classes. Enum
dispatch, function pointers/closures, and unconstrained parametric generics are
separate forms of polymorphism. A trait contains methods, associated types, and
constants that a type can *implement*; there is no inheritance of state. For a
C# reader, traits resemble interfaces plus associated types, blanket impls,
default methods, and explicit coherence rules.

```
+===============================================================================+
|                        TRAITS: THE ABSTRACTION LAYER                          |
+===============================================================================+

  DEFINE                                 IMPLEMENT (decoupled from definition)
  ------                                 ------------------------------------
  trait Area {                           impl Area for Circle {
    const NAME: &str;   // assoc const     const NAME: &str = "circle";
    type Unit;          // assoc type       type Unit = f64;
    fn area(&self) -> Self::Unit;           fn area(&self) -> f64 { ... }
    fn describe(&self) -> String {         }
      format!("{}", Self::NAME)  // default
    }
  }

  CONSTRAIN GENERICS                     THE RULES THAT KEEP IT SOUND
  ------------------                     ---------------------------
  fn f<T: Area>(x: T) ...                COHERENCE: at most ONE impl of a trait
  fn f<T>(x: T) where T: Area + Clone     for a type (no overlapping impls)
  fn f(x: impl Area) ...                 ORPHAN RULE: impl Trait for Type only
                                          if you own Trait OR Type (else it is
  BLANKET IMPL                            someone else's to define)
  impl<T: Display> Label for T { ... }   SUPERTRAIT: trait Ord: PartialEq { }
    -> every Display type gets Label       (Ord requires PartialEq first)
```

## Defining and Implementing

```rust
trait Summary {
    fn summarize(&self) -> String;                 // required method
    fn preview(&self) -> String {                  // default method
        let summary = self.summarize();
        let mut chars = summary.chars();
        let prefix: String = chars.by_ref().take(10).collect();
        if chars.next().is_some() { format!("{prefix}...") } else { prefix }
    }
}

struct Article { title: String, body: String }
impl Summary for Article {
    fn summarize(&self) -> String { self.title.clone() }
    // preview() inherited from the default
}
```

Definition and implementation are decoupled: you can `impl` a trait you defined
for a type someone else defined, or a trait someone else defined for your type.
That supports both extension directions of the **expression problem** whenever
you own either the trait or the type: add new behavior to a foreign type through
your trait, or implement a foreign trait for your type. Coherence deliberately
forbids the foreign-trait/foreign-type quadrant, discussed below. Default methods
let a trait ship reusable logic built on its own required methods (like
`Iterator`'s many adapters over one `next`).

## Generics and Bounds

Generic code is written against trait bounds; the compiler monomorphizes one
specialized copy per concrete type ([07](07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md)).

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &x in &list[1..] { if x > max { max = x; } }
    max
}
```

Three equivalent ways to state a bound, differing in ergonomics:

```rust
fn f<T: Summary + Clone>(x: T) {}                 // inline bounds
fn f<T>(x: T) where T: Summary + Clone {}         // where-clause (cleaner for many/complex bounds)
fn f(x: impl Summary + Clone) {}                  // argument-position impl Trait (APIT)
```

`where` clauses are not just style — they can express bounds that inline syntax
cannot, such as `where Vec<T>: Clone` or `where for<'a> &'a T: Iterator`. Reach
for `where` once bounds get long or involve associated types.

## Associated Types vs Generic Parameters

This is the distinction that separates people who "know traits" from people who
have used them. An **associated type** is an output type the *implementer* fixes;
a **generic parameter** is an input the *caller* chooses.

```rust
trait Iterator {                 // associated type: ONE Item per implementing type
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

trait From<T> {                  // generic param: MANY froms per type
    fn from(value: T) -> Self;   // impl From<u8> and From<u16> for MyNum both allowed
}
```

Rule of thumb: use an **associated type** when there is exactly one sensible
choice per implementing type (an iterator yields one item type; a `Deref` targets
one type). Use a **generic parameter** when a type should implement the trait
multiple ways (`From<u8>` *and* `From<u16>`). Associated types also keep call
sites clean — `fn sum(it: impl Iterator<Item = i32>)` reads better than threading
an extra type parameter everywhere. Associated **constants** (`const NAME: &str`)
and associated **functions** (no `self`, like `T::default()`) round out the set.

## Coherence and the Orphan Rule

Rust guarantees there is **at most one** implementation of a given trait for a
given type, program-wide. This is **coherence**, and it is why method resolution
is unambiguous and why adding a dependency never silently changes which impl you
get. The enforcement mechanism is the **orphan rule**: you may write `impl Trait
for Type` only if you own the trait *or* the type (or a local type appears in the
right position).

```rust
// You own neither Display (std) nor Vec (std):
// impl Display for Vec<i32> { ... }   // ERROR: orphan rule
// Fix: wrap it in a newtype you DO own.
struct MyVec(Vec<i32>);
impl std::fmt::Display for MyVec { /* ... allowed: MyVec is local */ }
```

The **newtype pattern** is the standard workaround: wrap the foreign type in a
local one and implement the foreign trait on the wrapper
([20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)). This is the cost of
coherence — you occasionally wrap — but the benefit is that trait resolution is
global, deterministic, and immune to "which library's impl won?" chaos.

## Blanket Impls and Supertraits

A **blanket impl** implements a trait for *all* types satisfying some bound —
this is how `impl<T: Display> ToString for T` gives every `Display` type a
`to_string()` for free:

```rust
trait Label { fn label(&self) -> String; }
impl<T: std::fmt::Display> Label for T {     // every Display type is now Label
    fn label(&self) -> String { format!("[{self}]") }
}
```

A **supertrait** requires another trait as a prerequisite — `trait Ord: PartialEq
+ Eq + PartialOrd` means "you can only be `Ord` if you are also those." It lets a
trait's methods rely on the supertrait's methods:

```rust
use std::fmt::Display;
trait Printable: Display {                    // Printable requires Display
    fn print(&self) { println!("{self}"); }   // can use Display's {} because of the bound
}
```

Supertraits look like inheritance but are *requirement* relationships, not state
inheritance — there is no base-class data.

## Old World -> New World Bridge

| Old world | Rust trait feature | Difference |
|-----------|--------------------|-----------|
| Interface (C#/Java) | trait (required methods) | Plus default methods, assoc types/consts |
| Abstract base class | trait + default methods | No shared *state*, only behavior |
| Haskell type class | trait | Nearly identical; associated types = type families |
| C# extension methods | trait + impl on foreign type | Coherent and dispatchable, not just sugar |
| C++ concepts (bounds) | trait bounds / `where` | Bounds are nominal, checked at definition |
| Operator overloading | `impl Add`, `impl Index`, ... | Operators are traits |
| C# generic constraints (`where T : IFoo`) | `where T: Foo` | Same spirit; monomorphized like C++ templates |
| Java/C# type erasure | monomorphization | No hidden boxing; `dyn` is opt-in ([07](07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md)) |

The key upgrade over C# interfaces: you can implement a trait you wrote for
`i32`, `String`, or `Vec<T>` and every generic function bounded by that trait now
accepts them. Behavior attaches to types after the fact, coherently.

## Common Confusion Points

- **Associated type vs type parameter.** One-per-type -> associated type;
  many-per-type -> parameter. Choosing wrong makes call sites verbose or impls
  impossible.
- **Orphan rule blocks a foreign-trait-on-foreign-type impl.** Wrap in a newtype
  you own. This is expected, not a language wart.
- **Blanket impls can conflict.** Two overlapping blanket impls violate
  coherence; the compiler rejects them. Specialization (choosing the "more
  specific" impl) is a *nightly-only* feature — do not rely on it.
- **Supertrait is not inheritance.** It requires another trait; it does not
  inherit fields or provide an object hierarchy.
- **Default methods can be overridden** per impl; required methods must be
  provided.
- **`where` unlocks bounds inline syntax can't** express (e.g. bounds on
  associated types or higher-ranked lifetimes — see HRTBs in [20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)).

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| Define shared behavior | a `trait` |
| Provide reusable logic on required methods | default methods |
| One output type per implementer | associated type (`type Item;`) |
| Many impls with different inputs | generic trait param (`From<T>`) |
| Constrain a generic | bound: `<T: Trait>` or `where T: Trait` |
| Give every `X` a method for free | blanket impl `impl<T: X> Y for T` |
| Require a prerequisite trait | supertrait: `trait A: B` |
| Implement a foreign trait on a foreign type | newtype wrapper (orphan rule) |
| Avoid monomorphization bloat / need heterogeneity | `dyn Trait` ([07](07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md)) |

## Primary Sources

- The Book, Ch. 10 (Generic Types, Traits, and Lifetimes): https://doc.rust-lang.org/book/ch10-00-generics.html
- Reference — Traits: https://doc.rust-lang.org/reference/items/traits.html
- Reference — Implementations & coherence: https://doc.rust-lang.org/reference/items/implementations.html
- Rust by Example — Traits: https://doc.rust-lang.org/rust-by-example/trait.html
- std::convert::From (associated vs generic example): https://doc.rust-lang.org/std/convert/trait.From.html

## Related Guides

- Previous: [05-STRUCTS-ENUMS-PATTERNS-AND-MATCH.md](05-STRUCTS-ENUMS-PATTERNS-AND-MATCH.md)
- Next: [07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md](07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md)
- Closures as trait impls: [08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md](08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md)
- Advanced trait patterns (sealed, GATs, HRTBs): [20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)
