---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:dispatch-trait-objects-and-impl-trait
kind: guide
module: rust-language
section: languages
title: Dispatch, Trait Objects, and impl Trait
status: source-custody
source_custody: partial
current_path: rust-language/07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md
canonical_path: rust-language/07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md
backsource_ids: [mdloom-backfill:rust-language:07-dispatch-trait-objects-and-impl-trait]
concepts: [monomorphization, static dispatch, dynamic dispatch, trait objects, dyn, object safety, dyn compatibility, impl Trait, RPIT, RPITIT, TAIT]
root_concepts: [dispatch]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Dispatch, Trait Objects, and impl Trait

Once you have traits ([06](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)), the next
decision is *how the call is dispatched*: at compile time (static,
monomorphized, with no dynamic-dispatch cost) or at run time (dynamic, via a
vtable behind `dyn`). Rust makes
this an explicit, visible choice at the use site. Java instance methods are
virtual by default; C# class methods are non-virtual unless marked
`virtual`/`abstract`/`override`, while interface references dispatch
dynamically. Rust instead uses generics for static dispatch and an explicit
`dyn` trait object for dynamic dispatch. This guide also covers `impl Trait`,
the syntax that hides concrete types in argument and return position.

```
+===============================================================================+
|              STATIC vs DYNAMIC DISPATCH (TYPICAL RUSTC MODEL)                 |
+===============================================================================+

  STATIC (monomorphization)              DYNAMIC (trait object)
  -------------------------              ----------------------
  fn draw<T: Shape>(s: &T)               fn draw(s: &dyn Shape)
    compiler stamps a copy per T           ONE function; call goes through vtable
    draw::<Circle>, draw::<Square>...

  &T  = thin pointer                     &dyn Shape = pointer + vtable metadata
                                           +----------+----------+
                                           | data ptr |  vtable  |
                                           +----------+----------+
                                                          |
                                              +-----------v-----------+
                                              | vtable: area(), draw()|
                                              | drop, size, align     |
                                              +-----------------------+

  cost: bigger binary, inlinable,        cost: pointer indirection; usually
        zero call overhead                     less inlining, smaller binary,
  use:  hot paths, generic libraries           heterogeneous collections
```

The pointer-plus-vtable picture is the current conceptual/rustc model, not a
stable binary-layout contract. Exact vtable contents and Rust ABI details may
change, and an optimizer may devirtualize a call when the concrete type becomes
provable.

## Static Dispatch: Monomorphization

A generic function is a *template*: the compiler generates a specialized copy for
each concrete type argument, just like C++ templates. Calls resolve at compile
time and inline freely — there is no vtable, no indirection.

```rust
fn print_all<T: std::fmt::Display>(items: &[T]) {
    for x in items { println!("{x}"); }
}
print_all(&[1, 2, 3]);        // compiler emits print_all::<i32>
print_all(&["a", "b"]);       // and print_all::<&str>
```

Cost: **binary bloat** (one copy per type) and longer compile times. Benefit:
maximal speed — the optimizer sees through the abstraction. This is the default;
`impl Trait` in argument position is just sugar for it (`fn f(x: impl Display)`
== `fn f<T: Display>(x: T)`).

## Dynamic Dispatch: Trait Objects (`dyn`)

A **trait object** `dyn Trait` erases the concrete type and dispatches through
vtable metadata at runtime. Because the value's size is not known at compile
time, trait objects live behind a pointer: `&dyn Trait`, `Box<dyn Trait>`,
`Rc<dyn Trait>`. Current rustc implementations represent these as a data pointer
plus vtable metadata; code must not treat the vtable layout as a stable ABI.

```rust
trait Draw { fn draw(&self); }
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle), Box::new(Square),   // heterogeneous — different types, one Vec
];
for s in &shapes { s.draw(); }            // each call dispatches via its vtable
```

Use `dyn` when you need a **heterogeneous collection** (a `Vec` of mixed types), a
runtime-selected implementation inside one Rust build, or to **cap
monomorphization bloat** in a large generic API. It is the same broad mechanism
as a C++ `virtual` call or a C# interface reference. It is **not** a stable
dynamic-library/plugin ABI across compiler builds: use an explicit C ABI or a
deliberately versioned stable-ABI framework for that boundary.

## Object Safety / "dyn Compatibility"

Not every trait can become a `dyn` object. A trait is **dyn compatible** (the
term the reference adopted; historically "object safe") only if the compiler can
build a vtable for it. The main disqualifiers:

- methods that are **generic** over type parameters (a vtable slot cannot
  represent infinitely many monomorphizations);
- methods returning `Self` by value (the concrete size is erased);
- associated constants; certain uses of `Self` in signatures.

```rust
trait Good { fn area(&self) -> f64; }              // dyn compatible
trait Bad  { fn make() -> Self; fn cmp<T>(&self, t: T); }  // NOT dyn compatible
// Box<dyn Bad>  // ERROR: the trait cannot be made into an object
```

Design fix: put the non-dyn-safe methods in a separate trait, or take
`&mut dyn`/`&dyn` params instead of generic ones, or return `Box<dyn ...>`
instead of `Self`. Methods can be individually excluded from the vtable with a
`where Self: Sized` bound so the *rest* of the trait stays dyn compatible.

## `impl Trait` in Return Position (RPIT)

`impl Trait` in **return** position means "I return *some* concrete type that
implements this trait, but I'm not naming it." It is static dispatch with a hidden
type — essential for returning closures and iterators whose types are unnameable:

```rust
fn adder(n: i32) -> impl Fn(i32) -> i32 {   // the closure's type has no name
    move |x| x + n
}
fn evens() -> impl Iterator<Item = u32> {   // hides a giant Filter<Map<...>> type
    (0..).filter(|n| n % 2 == 0)
}
```

RPIT returns a *single* concrete type chosen by the function body — you cannot
return `impl Trait` and have two different concrete types on different branches
(use `Box<dyn Trait>` for that). It is zero-cost: no boxing, no vtable.

## RPITIT and async fn in traits (recent, now stable)

Returning `impl Trait` from a **trait method** — **RPITIT** (Return-Position
`impl Trait` In Traits) — and the closely related `async fn` in traits (AFIT)
were stabilized in **Rust 1.75 (late 2023)**. Before that they required the
`async-trait` crate's boxing workaround.

```rust
trait Fetch {
    fn ids(&self) -> impl Iterator<Item = u64>;   // RPITIT (stable since 1.75)
    async fn get(&self, id: u64) -> String;       // AFIT (stable since 1.75)
}
```

Caveat worth knowing: `async fn` in a trait does not yet let you *name* or
*bound* the returned future's `Send`-ness at the trait level in the simplest
form; for public traits that must be usable across threads you often still reach
for the `trait-variant` crate or return an explicit `impl Future + Send`. AFIT is
production-usable but has these ergonomic edges — see
[14](14-ASYNC-FUTURES-AND-PINNING.md).

## TAIT — Type Alias impl Trait (nightly / not fully stable)

**TAIT** lets you name an `impl Trait` type via a type alias so multiple
functions can share one hidden type:

```rust
// NIGHTLY ONLY as of this writing — requires #![feature(type_alias_impl_trait)]
// type Ints = impl Iterator<Item = i32>;
// fn a() -> Ints { 0..10 }
```

As of current stable Rust, **general TAIT is unstable** (`type_alias_impl_trait`)
and its associated-type cousin (`impl_trait_in_assoc_type`) is also nightly. Do
not assume TAIT is available; if you need a shared hidden type on stable, box it
(`Box<dyn Trait>`) or expose the concrete type. Check the Unstable Book /
release notes before relying on it.

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| C++ templates | generics / monomorphization | Same codegen model; bounds are checked at definition |
| C++ `virtual` methods | `dyn Trait` (vtable) | Opt-in per use, not per-class default |
| Java/C# interface reference | `&dyn Trait` / `Box<dyn Trait>` | Explicit `dyn`; fat pointer, not header vtable |
| C# `where T:` generics (JIT-specialized for value types) | monomorphization | Rust always monomorphizes; no boxing surprise |
| Returning an interface | `-> impl Trait` (static) or `-> Box<dyn Trait>` (dynamic) | Choose cost explicitly |
| `Func<>`/lambda return | `-> impl Fn(...)` | Names the unnameable closure type |

The big difference from Java/C#: there, virtual dispatch is the invisible default
and you pay for it everywhere. In Rust, static dispatch is default and `dyn` is a
visible opt-in. You never accidentally pay for dynamic dispatch.

## Common Confusion Points

- **`impl Trait` in arg vs return position.** Argument position = generic sugar
  (caller picks the type). Return position = the *function* picks one hidden
  concrete type (caller cannot).
- **`dyn` needs a pointer.** `dyn Trait` is unsized; you must use `&dyn`,
  `Box<dyn>`, `Rc<dyn>`, etc. `let x: dyn Trait` alone does not compile.
- **RPIT returns exactly one type.** Two different concrete types across `if`
  branches won't compile under `impl Trait`; use `Box<dyn Trait>`.
- **Object safety / dyn compatibility.** Generic methods and `-> Self` break
  `dyn`. Segregate them or bound with `where Self: Sized`.
- **AFIT is stable but has Send-bound edges.** For cross-thread public async
  traits, you may still need `impl Future + Send` or a helper crate.
- **TAIT is not stable.** Do not write `type X = impl Trait;` on stable; it will
  not compile without a nightly feature gate.

## Decision Cheat Sheet

| Situation | Choose |
|-----------|--------|
| Hot path, one or few types, want inlining | generics (static dispatch) |
| Heterogeneous collection of trait impls | `Vec<Box<dyn Trait>>` |
| Runtime-selected implementations compiled in one Rust build | `dyn Trait` |
| Dynamic-library/plugin ABI across compiler builds | explicit C ABI or a stable-ABI framework; not `dyn Trait` |
| Return a closure or iterator | `-> impl Trait` (RPIT) |
| Different concrete return types per branch | `-> Box<dyn Trait>` |
| Trait method returning an iterator/future | RPITIT / `async fn` (stable 1.75+) |
| Share one hidden type across functions | `Box<dyn>` on stable; TAIT only on nightly |
| Keep a trait usable as `dyn` | avoid generic methods / `-> Self`; use `where Self: Sized` |

## Primary Sources

- The Book, Ch. 18.2 (Trait Objects): https://doc.rust-lang.org/book/ch18-02-trait-objects.html
- Reference — Trait objects: https://doc.rust-lang.org/reference/types/trait-object.html
- Reference — impl Trait: https://doc.rust-lang.org/reference/types/impl-trait.html
- Rust 1.75 release notes (RPITIT/AFIT): https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html
- The Unstable Book (type_alias_impl_trait): https://doc.rust-lang.org/unstable-book/language-features/type-alias-impl-trait.html

## Related Guides

- Previous: [06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)
- Next: [08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md](08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md)
- Async trait methods in depth: [14-ASYNC-FUTURES-AND-PINNING.md](14-ASYNC-FUTURES-AND-PINNING.md)
- Smart pointers behind `dyn`: [16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)
