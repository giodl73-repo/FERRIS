---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:bindings-types-and-inference
kind: guide
module: rust-language
section: languages
title: Bindings, Types, and Inference
status: source-custody
source_custody: partial
current_path: rust-language/02-BINDINGS-TYPES-AND-INFERENCE.md
canonical_path: rust-language/02-BINDINGS-TYPES-AND-INFERENCE.md
backsource_ids: [mdloom-backfill:rust-language:02-bindings-types-and-inference]
concepts: [bindings, mutability, shadowing, scalar types, tuples, arrays, type inference, coercions, casts, never type, unit type]
root_concepts: [type system]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Bindings, Types, and Inference

Rust's surface type layer looks familiar — fixed-width integers, tuples, arrays
— but three things behave differently from what a C++/.NET reader expects:
bindings are **immutable by default**, `let` supports **shadowing** (rebind the
same name to a new type), and inference is **local and bidirectional** rather
than the whole-program Hindley-Milner of Haskell. Get these three straight and
the primitive layer holds no surprises.

```
+===============================================================================+
|                       THE PRIMITIVE / BINDING LAYER                           |
+===============================================================================+

  BINDINGS                         SCALARS
  --------                         -------
  let x = 5;         immutable     i8 i16 i32 i64 i128 isize   (signed)
  let mut x = 5;     mutable       u8 u16 u32 u64 u128 usize   (unsigned)
  let x: i64 = 5;    annotated     f32 f64                     (IEEE-754)
  let x = x + 1;     SHADOWING     bool  (true/false, 1 byte, NOT 0/1)
                     (new binding) char  (Unicode scalar, 4 bytes, NOT a byte)

  COMPOUND                         SPECIAL
  --------                         -------
  (i32, &str, f64)   tuple         ()   unit    "no meaningful value"
  [i32; 4]           array         !    never   "this never returns"
  &[i32]  &str       slice ref

  INFERENCE                        CONVERSIONS
  ---------                        -----------
  local + bidirectional            as        explicit numeric cast (lossy OK)
  flows forward AND backward        From/Into value conversion (lossless)
  needs a witness eventually        deref/unsize coercion (implicit, limited)
```

## Bindings: Immutable by Default, Shadowing, Mutability

`let x = 5` creates an **immutable** binding. This is the reverse of C#/C++/Java,
where `int x = 5` is mutable and you add `const`/`readonly`/`final` to lock it.
In Rust you add `mut` to *unlock*:

```rust
let x = 5;
// x = 6;            // ERROR: cannot assign twice to immutable variable `x`
let mut y = 5;
y = 6;               // OK

// Shadowing: a NEW binding that reuses the name — can change type
let spaces = "   ";              // &str
let spaces = spaces.len();       // usize — different type, same name
// This is not mutation; the first binding is shadowed, not overwritten.
```

Shadowing is not mutation. Each `let` introduces a fresh binding; the previous
one is simply no longer nameable. This lets you refine a value through a pipeline
(parse a string, then reuse the name for the parsed number) without inventing
`spaces_str`, `spaces_len` names, and without `mut`. Mutability is about
*changing a value in place*; shadowing is about *reusing a name*.

`const` and `static` are different again: `const` is a typed compile-time value
with no unique storage identity, while `static` is a single memory location with
`'static` lifetime. Both are covered in
[18](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md).

## Scalars: Fixed-Width, No Surprises

| Category | Types | Notes |
|----------|-------|-------|
| Signed int | `i8 i16 i32 i64 i128 isize` | `isize` = pointer width (index type) |
| Unsigned int | `u8 u16 u32 u64 u128 usize` | `usize` indexes collections |
| Float | `f32 f64` | IEEE-754; `f64` is the default float |
| Boolean | `bool` | 1 byte; `true`/`false`, never coerces to/from int |
| Character | `char` | 4 bytes; a Unicode scalar value, **not** a byte |

The default integer type is `i32`; the default float is `f64`. There is no
implicit integer widening — assigning an `i32` where an `i64` is expected is an
error, not a silent promotion.
Ordinary integer operators follow the build's overflow-check setting. Cargo's
default dev/test profiles check and panic; its default release profile disables
those checks, so overflowing operations wrap in two's-complement arithmetic.
`[profile.*].overflow-checks` can change either profile, and constant overflow is
diagnosed separately. For explicit, profile-independent intent use
`wrapping_add`, `checked_add` (returns `Option`), `saturating_add`, or
`overflowing_add`. This is a deliberate contrast to C's undefined signed
overflow.

`char` being 4 bytes trips up C veterans: a `char` is a Unicode scalar value
(`'A'`, `'β'`, `'😀'` are each one `char`), while a raw byte is `u8`. Text
mechanics are the subject of [10](10-STRINGS-TEXT-AND-UNICODE.md).

## Compound Types: Tuples and Arrays

```rust
let pair: (i32, &str) = (1, "one");
let (n, name) = pair;            // destructuring bind
let first = pair.0;             // positional access

let arr: [i32; 4] = [10, 20, 30, 40];   // fixed length is part of the TYPE
let zeros = [0u8; 1024];               // [expr; N] repeat init
let slice: &[i32] = &arr[1..3];        // borrow a view: [20, 30]
```

Two facts matter. First, an array's length `N` is part of its type: `[i32; 3]`
and `[i32; 4]` are different, incompatible types (a form of const generic — see
[20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)). Second, the *dynamic*
sequence you usually want is `Vec<T>` (heap, growable) or a slice `&[T]` (a
borrowed view: pointer + length).
An array stores its elements inline wherever the array itself is placed: a
local is commonly stack-allocated, a `static` has static storage, and
`Box<[T; N]>` places it on the heap. Reach for arrays when the size is fixed;
collections are [09](09-COLLECTIONS-ITERATORS-AND-RANGES.md).

## The Two Special Types: `()` and `!`

- **Unit `()`** is the zero-field tuple — the type of an expression with "no
  meaningful value." Functions with no `-> T` return `()`; a `;` turns an
  expression into a statement of type `()`. It is Rust's `void`, but it is a
  real, first-class value you can store and pass.
- **Never `!`** is the empty type: it has *no* values, and it is the type of
  expressions that never produce a value — `panic!()`, `return`, `break`,
  `loop {}`, `std::process::exit`. Because `!` coerces to *any* type, this
  type-checks:

```rust
let n: i32 = match parse(s) {
    Ok(v) => v,
    Err(_) => return,   // `return` has type ! , coerces to i32
};
```

The `!` type is fully usable in *diverging* positions on stable, but **naming
`!` explicitly as a type in arbitrary positions** (e.g. `fn f() -> !` is fine,
but `let x: ! = ...` and generic uses) is gated behind the unstable `never_type`
feature on nightly. Treat "expressions can diverge with `!`" as stable, and
"I can write `!` anywhere I write a type" as not-yet-stable.

## Inference: Local, Bidirectional, Needs a Witness

Rust inference is **local** (function-body scoped) and **bidirectional**:
information flows both from initializers to bindings *and* backward from later
uses to earlier ones.

```rust
let mut v = Vec::new();   // type unknown here...
v.push(3u8);              // ...backward inference fixes v: Vec<u8>

let parsed: i64 = "42".parse().unwrap();   // annotation on `parsed`...
let parsed = "42".parse::<i64>().unwrap(); // ...or turbofish on the call
```

Unlike Haskell's whole-program Hindley-Milner, Rust never infers *across*
function boundaries — every function signature must be fully annotated. That is a
deliberate API-stability decision: a function's type is its contract. Inside a
body, though, you rarely annotate. When the compiler cannot find a witness it
asks for one ("type annotations needed"); supply it with either a binding
annotation (`let x: T`) or the turbofish (`method::<T>()`).

## Coercions and Casts: Three Distinct Mechanisms

Do not conflate these — they have different safety guarantees.

| Mechanism | Syntax | Semantics | Can lose data? |
|-----------|--------|-----------|----------------|
| **`as` cast** | `x as u8` | Primitive numeric/pointer cast | **Yes** — truncates, wraps, saturates float->int |
| **`From`/`Into`** | `u64::from(x)`, `x.into()` | Value conversion, *infallible + lossless by convention* | No (that is the contract) |
| **`TryFrom`/`TryInto`** | `u8::try_from(x)?` | Fallible conversion | Returns `Err` instead of losing data |
| **Deref/unsize coercion** | implicit | `&String -> &str`, `&[T; N] -> &[T]`, `&Box<T> -> &T` | No |

```rust
let big: i64 = 300;
let small = big as u8;              // 44 — silently truncates. `as` is unchecked.
let ok = u8::try_from(big);         // Err — the SAFE way to narrow
let wide: i64 = i64::from(10i32);   // lossless widening via From
```

Use `From`/`Into` for lossless conversions (they are the idiomatic API — see
[11](11-ERRORS-RESULT-OPTION-AND-PANIC.md) for how `?` uses `From`), `TryFrom`
when narrowing can fail, and reserve `as` for cases where you *intend* the
truncation (bit manipulation, FFI) — clippy will nag on risky `as` casts.

## Old World -> New World Bridge

| C++ / .NET | Rust | Difference that bites |
|------------|------|-----------------------|
| `int x = 5;` (mutable default) | `let x = 5;` (immutable default) | Add `mut` to mutate |
| `const` / `readonly` / `final` | the *absence* of `mut` | Immutability is the baseline |
| implicit int widening (`int`->`long`) | no implicit numeric conversion | Use `From`/`as` explicitly |
| C `char` = 1 byte | Rust `char` = 4-byte Unicode scalar | Byte is `u8`, not `char` |
| `void` | `()` unit — a real value | You can bind and pass `()` |
| C++ `[[noreturn]]` | the `!` never type | Coerces to any type |
| `static_cast<T>` | `as` (for primitives) / `From` | `as` is unchecked; prefer `TryFrom` |
| `var`/`auto` local inference | `let` local inference | But never across fn signatures |

## Common Confusion Points

- **Shadowing vs mutation.** `let x = x + 1` is a new binding; `x = x + 1`
  requires `mut` and mutates in place. They read similarly, mean different things.
- **`usize` vs `u64`.** Indices and lengths are `usize` (pointer-width). Do not
  hardcode `u64` for a length; you will fight casts. Convert at boundaries.
- **`as` is a footgun for narrowing.** `300i32 as u8` is `44`, silently. Reach
  for `try_from` when the value might not fit.
- **Overflow is not C-style signed UB.** Cargo's default dev/test profiles panic
  and default release disables checks, but profiles can override that. State
  intent explicitly with `wrapping_*`, `checked_*`, or `saturating_*`.
- **Inference "leaks" backward.** `let v = Vec::new(); v.push(1u8);` compiles
  because inference is bidirectional; a lone `Vec::new()` with no later witness
  will demand an annotation.
- **You cannot write `!` as a type yet (stably).** Diverging expressions have
  type `!`, but explicit `let x: ! = ...` needs nightly `never_type`.

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| A variable I will reassign | `let mut x = ...` |
| Refine a value through a pipeline (maybe changing type) | shadowing: `let x = ...; let x = ...` |
| A fixed-size inline buffer | `[T; N]` |
| A growable sequence | `Vec<T>` ([09](09-COLLECTIONS-ITERATORS-AND-RANGES.md)) |
| A borrowed view over a sequence | `&[T]` |
| Widen a number safely | `i64::from(x)` / `x.into()` |
| Narrow a number safely | `u8::try_from(x)?` |
| Bit-level truncation on purpose | `x as u8` |
| Signal "cannot happen / never returns" | `!` (via `panic!`, `return`, `unreachable!()`) |

## Primary Sources

- The Book, Ch. 3 (Common Programming Concepts): https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
- Reference — Types: https://doc.rust-lang.org/reference/types.html
- Reference — Type coercions: https://doc.rust-lang.org/reference/type-coercions.html
- Reference — Never type: https://doc.rust-lang.org/reference/types/never.html
- std::convert (From/Into/TryFrom): https://doc.rust-lang.org/std/convert/index.html

## Related Guides

- Previous: [01-TOOLCHAIN-AND-WORKFLOW.md](01-TOOLCHAIN-AND-WORKFLOW.md)
- Next: [03-OWNERSHIP-MOVES-COPY-AND-DROP.md](03-OWNERSHIP-MOVES-COPY-AND-DROP.md)
- Text & `char`: [10-STRINGS-TEXT-AND-UNICODE.md](10-STRINGS-TEXT-AND-UNICODE.md)
- `const`/`static`: [18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)
