---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:api-design-semver-and-advanced-type-patterns
kind: guide
module: rust-language
section: languages
title: API Design, SemVer, and Advanced Type Patterns
status: source-custody
source_custody: partial
current_path: rust-language/20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md
canonical_path: rust-language/20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md
backsource_ids: [mdloom-backfill:rust-language:20-api-design-semver-and-advanced-type-patterns]
concepts: [API design, semver, sealed traits, non_exhaustive, typestate, newtype, builders, HRTB, GAT, const generics, phantom types]
root_concepts: [api design]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# API Design, SemVer, and Advanced Type Patterns

This capstone guide is about designing public APIs that evolve gracefully and
using Rust's type system to make misuse *unrepresentable*. Cargo interprets
version requirements using **SemVer conventions**; crate publishers remain
responsible for classifying changes honestly. Because the type system is so
expressive, a surprising amount of API safety comes from patterns (newtype,
typestate, sealed traits) rather than runtime checks. It closes with the
advanced type machinery (HRTBs, GATs, const generics, phantom types) that
library authors reach for.

```
+===============================================================================+
|                 DESIGN FOR EVOLUTION + ENCODE INVARIANTS IN TYPES             |
+===============================================================================+

  SEMVER (Cargo)                          EVOLUTION LEVERS
  --------------                          ----------------
  MAJOR.MINOR.PATCH                       #[non_exhaustive]  reserve the right to add
    0.x: MINOR may break (pre-1.0)          enum variants / struct fields / errors
  breaking: remove/rename pub item,       sealed trait       only YOU may implement it
    add trait method w/o default,         pub use facade     hide internal module layout
    change signature, new required bound  private item/mod   keep it outside public API

  TYPE-STATE / SAFETY PATTERNS            ADVANCED TYPE MACHINERY
  ----------------------------            -----------------------
  newtype     UserId(u64)  distinct type  HRTB     for<'a> Fn(&'a T)  (any lifetime)
  builder     fluent, validated ctor      GAT      type Item<'a>;      (stable 1.65)
  typestate   Conn<Open> vs Conn<Closed>  const generic  Matrix<const N: usize>
    illegal transitions won't COMPILE     PhantomData<T>  mark unused type params
```

## Cargo SemVer: What Breaks

Cargo uses SemVer-compatible ranges during dependency resolution; it cannot
verify that a published release actually preserves compatibility. Use these
categories as API review guidance:

| Clearly breaking (MAJOR, or MINOR pre-1.0) | Usually compatible | Possibly breaking — investigate downstream interactions |
|---------------------------------------------|--------------------|----------------------------------------------------------|
| Remove/rename a `pub` item | Deprecate with `#[deprecated]` | Add a new public item (name/glob-import collisions) |
| Add a required public-trait method | Add an optional additive feature | Add a **defaulted** trait method (method ambiguity) |
| Add a field to an exhaustively constructible public struct | Add a variant/field where `#[non_exhaustive]` reserved growth | Add a public trait impl (inference or method-resolution changes) |
| Add/tighten a public generic bound | Loosen a bound | Add an inherent method that collides with a trait method |
| Change a function signature or make a public type private | Documentation-only changes | Change inference, auto-trait, or variance behavior without changing syntax |

`cargo semver-checks` catches many public-API changes, but it is an ecosystem
linter rather than a proof: it cannot validate behavioral compatibility or
every downstream inference/name-resolution interaction.

`#[doc(hidden)]` only omits a public item from generated documentation. The item
remains name-resolvable and part of the compatibility surface. Keep implementation
details private; use `doc(hidden)` only when a deliberately public item should
be visually de-emphasized.

## `#[non_exhaustive]`: Reserve the Right to Grow

Marking an enum, struct, or variant `#[non_exhaustive]` forces downstream code to
account for future additions — enums require a `_` arm, structs cannot be
constructed with a literal or matched exhaustively by outside crates. This lets
you add variants/fields later **without** a major bump. Use it on error enums and
config structs that will grow.

```rust
#[non_exhaustive]
pub enum ApiError { NotFound, Timeout }   // downstream must add `_ =>` -> future variants are non-breaking
```

## The Newtype Pattern

Wrapping a value in a single-field tuple struct gives a **distinct type** with
no additional stored field; optimized code normally erases the wrapper. Use
`#[repr(transparent)]` when ABI equivalence with the field is part of the
contract. Three payoffs: (1) prevent mixing semantically different values of the
same underlying type; (2) implement a foreign trait on a foreign type (the
orphan-rule workaround, [06](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)); (3)
enforce a documented invariant at construction.

```rust
#[derive(Debug)]
pub struct InvalidEmail;

pub struct Email(String);                 // cannot be confused with a raw String

impl Email {
    pub fn parse(s: &str) -> Result<Self, InvalidEmail> {
        let Some((local, domain)) = s.split_once('@') else {
            return Err(InvalidEmail);
        };
        if !local.is_empty() && !domain.is_empty() && !domain.contains('@') {
            Ok(Email(s.to_owned()))
        } else {
            Err(InvalidEmail)
        }
    }
}
// `Email` guarantees this narrow invariant: exactly one '@', with nonempty sides.
// RFC syntax, normalization, and deliverability require a stronger policy.
```

## The Builder Pattern

For structs with many optional fields (Rust has no named/default arguments), a
builder gives a fluent, validated constructor and keeps the struct's own
construction private:

```rust
let client = HttpClient::builder()
    .timeout(Duration::from_secs(30))
    .retries(3)
    .build()?;                             // build() validates and can fail
```

## Typestate: Encode State in the Type

The **typestate** pattern makes illegal state transitions a *compile* error by
encoding the state in a type parameter. A `Connection<Open>` and
`Connection<Closed>` are different types, and methods exist only on the states
where they are valid:

```rust
struct Open; struct Closed;
struct Connection<S> { sock: TcpStream, _state: std::marker::PhantomData<S> }

impl Connection<Closed> {
    fn open(self) -> Connection<Open> { /* ... */ }
}
impl Connection<Open> {
    fn send(&mut self, data: &[u8]) { /* only callable when Open */ }
    fn close(self) -> Connection<Closed> { /* ... */ }
}
// conn.send(..) on a Closed connection -> does not compile
```

`PhantomData<S>` carries the state type without storing any data (below). This
turns a class of runtime "wrong state" bugs into compile errors — the same spirit
as making illegal states unrepresentable with enums ([05](05-STRUCTS-ENUMS-PATTERNS-AND-MATCH.md)),
extended to *protocols*.

## Sealed Traits

A **sealed trait** can be *used* by downstream code but not *implemented* by it —
you keep the freedom to add methods without breaking anyone. Implement it by
making the trait depend on a private supertrait:

```rust
mod private { pub trait Sealed {} }
pub trait Format: private::Sealed {        // downstream can call, cannot impl
    fn render(&self) -> String;
}
impl private::Sealed for Json {}
impl Format for Json { fn render(&self) -> String { /* ... */ } }
```

Because only your crate can name `private::Sealed`, only your crate can implement
`Format`. This is a key API-stability tool for traits you want to evolve.

## Advanced Type Machinery

### HRTBs — Higher-Ranked Trait Bounds

`for<'a>` expresses "for **any** lifetime," needed when a bound must hold for
lifetimes the caller picks later — most often with closures over references:

```rust
fn apply<F>(f: F) where F: for<'a> Fn(&'a str) -> &'a str { /* f works for any 'a */ }
```

You mostly meet HRTBs implicitly (the compiler inserts them for `Fn` traits over
references); you write `for<'a>` explicitly when the inference cannot.

### GATs — Generic Associated Types (stable since Rust 1.65)

A GAT is an associated type that is itself generic (over a lifetime or type),
enabling patterns like a lending iterator whose items borrow from the iterator:

```rust
trait LendingIterator {
    type Item<'a> where Self: 'a;          // the associated type is generic in 'a
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```

GATs stabilized in **Rust 1.65** and unblock a family of previously-impossible
abstractions (streaming/lending iterators, some async-trait patterns). They are
stable but still advanced — reach for them when a normal associated type cannot
express the borrow relationship.

### Const Generics

Types can be parameterized by **const values** (currently integers, `bool`,
`char`), so array-size and dimension become type parameters — `min_const_generics`
is stable since **Rust 1.51**:

```rust
struct Matrix<const R: usize, const C: usize> { data: [[f64; C]; R] }
fn identity<const N: usize>() -> Matrix<N, N> { /* ... */ }
```

More elaborate const-generic expressions (`generic_const_exprs`, arithmetic in
const-generic positions) remain **nightly** — do not assume `Matrix<N, {N+1}>`
compiles on stable.

### PhantomData and Phantom Types

`PhantomData<T>` is a zero-sized marker that tells the compiler a type "acts as
if" it owns/uses a `T` even though it stores none — used for unused type/lifetime
parameters (as in typestate above), to carry variance/drop-check information, and
in FFI wrappers. It has no runtime footprint; it only informs the type system.

## Old World -> New World Bridge

| Old world | Rust pattern | Difference |
|-----------|--------------|-----------|
| .NET/NuGet SemVer | Cargo SemVer | Same MAJOR.MINOR.PATCH; enforced in resolution |
| `[Obsolete]` | `#[deprecated]` | Same deprecation signal |
| non-sealed vs `sealed` class (C#) | sealed-trait pattern | Sealed = "callable, not implementable" |
| primitive obsession -> value objects | newtype | Distinct type, zero cost |
| fluent builder (C#/Java) | builder pattern | Rust lacks named args, so builders are idiomatic |
| state machine with runtime checks | typestate | Illegal transitions fail to **compile** |
| generics + variance (C#) | HRTBs / variance | Lifetime-aware; `for<'a>` for closures |
| fixed-size arrays / `stackalloc` | const generics | Size is a type parameter |
| marker/attribute-only types | `PhantomData<T>` | Zero-sized type-system marker |

## Common Confusion Points

- **What is a breaking change?** Adding a required trait method, a struct field
  (non-`non_exhaustive`), or a tighter bound all break downstream. Add defaulted
  methods and mark growable types `#[non_exhaustive]`.
- **`#[non_exhaustive]` has a cost to callers.** They must add `_` arms / cannot
  use struct literals. Use it where growth is expected, not everywhere.
- **Sealed traits need a private supertrait** (or a private method) — there is no
  `sealed` keyword.
- **Typestate `PhantomData` stores nothing.** The state lives purely in the type
  parameter; forgetting `PhantomData` for an unused parameter is a compile error.
- **GATs are stable (1.65) but advanced.** Use a plain associated type unless you
  truly need a borrow-parameterized item.
- **Const generics are limited on stable.** Basic `const N: usize` works;
  arithmetic on const generics is nightly.
- **HRTBs are usually implicit.** You write `for<'a>` only when the compiler
  cannot infer the "for any lifetime" bound.

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| Let an enum/struct grow without a major bump | `#[non_exhaustive]` |
| Prevent mixing same-underlying-type values | newtype `struct Id(u64)` |
| Validate-on-construct | newtype + fallible constructor |
| Configure a many-optional-field object | builder pattern |
| Make illegal state transitions uncompilable | typestate + `PhantomData` |
| Let others call but not implement a trait | sealed trait |
| Bound a closure over any lifetime | HRTB `for<'a>` |
| An associated type that borrows from `self` | GAT (1.65+) |
| Parameterize a type by a size/number | const generics (basic: stable) |
| Mark an unused type/lifetime parameter | `PhantomData<T>` |
| Keep an item out of the public API | leave it private / place it in a private module |
| Hide a deliberately public item from generated docs only | `#[doc(hidden)]` (still SemVer-public) |
| Check SemVer before release | `cargo semver-checks` |

## Primary Sources

- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- The Cargo Book — SemVer compatibility: https://doc.rust-lang.org/cargo/reference/semver.html
- Reference — Generics & const generics: https://doc.rust-lang.org/reference/items/generics.html
- Rust 1.65 release notes (GATs, let-else): https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html
- std::marker::PhantomData: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
- Rustonomicon — Higher-Rank Trait Bounds: https://doc.rust-lang.org/nomicon/hrtb.html

## Related Guides

- Previous: [19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md](19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md)
- Back to start: [00-OVERVIEW.md](00-OVERVIEW.md)
- Traits & coherence underpin sealing/newtype: [06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)
- API layout & re-exports: [12-MODULES-CRATES-PACKAGES-AND-VISIBILITY.md](12-MODULES-CRATES-PACKAGES-AND-VISIBILITY.md)
