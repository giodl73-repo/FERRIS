---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:borrowing-references-and-lifetimes
kind: guide
module: rust-language
section: languages
title: Borrowing, References, and Lifetimes
status: source-custody
source_custody: partial
current_path: rust-language/04-BORROWING-REFERENCES-AND-LIFETIMES.md
canonical_path: rust-language/04-BORROWING-REFERENCES-AND-LIFETIMES.md
backsource_ids: [proof-backfill:rust-language:04-borrowing-references-and-lifetimes]
concepts: [borrowing, shared references, exclusive references, non-lexical lifetimes, lifetime elision, explicit lifetimes, reborrowing, variance]
root_concepts: [borrowing]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Borrowing, References, and Lifetimes

Ownership ([03](03-OWNERSHIP-MOVES-COPY-AND-DROP.md)) says a value has one owner.
Borrowing is how you *access* a value without taking ownership — and the borrow
checker is the proof engine that ensures no reference outlives the data it points
to and that mutation never races with aliasing. The single rule to memorize:
**at any moment, a value may have many shared borrows `&T` OR exactly one
exclusive borrow `&mut T`, never both.** Aliasing XOR mutation. Everything about
lifetimes is bookkeeping to enforce that rule across time.

```
+===============================================================================+
|                      BORROWING: ALIASING XOR MUTATION                         |
+===============================================================================+

   SHARED &T  (read)                    EXCLUSIVE &mut T  (read+write)
   +-----------------------------+      +-----------------------------+
   |  &v  &v  &v ... &v          |      |          &mut v             |
   |  any number of readers      |      |  exactly ONE, no other      |
   |  no writer may coexist      |      |  borrow may coexist         |
   +-----------------------------+      +-----------------------------+

   THE INVARIANT                        NLL: a borrow lives from FIRST use
   -------------                        to LAST use, not to end of scope
   data race needs: 2 accesses,         +---------------------------------+
   >=1 write, no sync                   | let mut v = ...;                |
   the &T-xor-&mut-T rule forbids       | let r = &v;   <- borrow starts  |
   exactly that pattern -> no data      | use(r);       <- LAST use = end |
   races EVEN across threads (15)       | v.push(1);    <- OK now         |
                                        +---------------------------------+
```

## Shared vs Exclusive Borrows

```rust
let s = String::from("hello");
let r1 = &s;            // shared borrow
let r2 = &s;            // another shared borrow — fine, both read
println!("{r1} {r2}");  // OK: many readers

let mut m = String::from("hi");
let w = &mut m;         // exclusive borrow
w.push_str(" there");   // mutate through it
// let r = &m;          // ERROR here: cannot borrow `m` as shared while `w` (mut) is live
println!("{w}");
```

`&mut` is misread as "mutable"; the load-bearing property is **exclusive**. A
`&mut T` is the *only* live path to that value for its duration, which is exactly
why the compiler can allow mutation without fear of a concurrent reader observing
a torn value. This is also the connective tissue to concurrency: since data races
require simultaneous aliasing + mutation, and the borrow rule forbids that, the
same check that stops single-threaded bugs stops data races ([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)).

## The NLL Mental Model

Modern Rust uses **Non-Lexical Lifetimes (NLL)**: a borrow lasts from its first
use to its *last* use, not mechanically to the end of the enclosing block. This
makes the checker match intuition.

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];      // shared borrow begins
println!("{first}");    // ... last use of `first` HERE
v.push(4);              // OK: the shared borrow already ended
```

Under the old lexical rule this failed because `first`'s borrow "lived" to the
closing brace. With NLL the borrow is dead after its last read, so the mutable
`push` is legal. Think of a borrow's lifetime as the *span of actual use*, and
most "why is this still borrowed?" confusion evaporates. When it does not, the
culprit is usually a borrow stored in a longer-lived place (a struct field, a
returned reference).

## Lifetimes: Naming How Long a Borrow Is Valid

A **lifetime** is a compile-time region of code during which a reference is
valid. Lifetimes have no runtime representation — they are purely a static proof
that a reference never outlives its referent. Most of the time you never write
one, thanks to *elision*. You write them explicitly when a function returns a
reference and the compiler cannot tell which input it borrows from.

```rust
// Explicit: the output borrows from EITHER input, so both share lifetime 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` does not *change* how long anything lives; it *relates* the inputs and
output. The signature reads: "the returned reference is valid no longer than the
shorter of the two inputs." The compiler then rejects any call where you would
use the result after an input was dropped.

### Elision Rules (why you rarely write lifetimes)

The compiler applies three rules to fill in lifetimes for `&` parameters:

1. Each elided input reference gets its own distinct lifetime.
2. If there is exactly **one** input lifetime, it is assigned to all outputs.
3. If there is a `&self`/`&mut self`, **its** lifetime is assigned to all outputs.

```rust
fn first_word(s: &str) -> &str { /* ... */ }         // rule 2: output borrows s
impl Doc { fn title(&self) -> &str { &self.title } } // rule 3: output borrows self
```

You only annotate when elision is ambiguous — typically multiple reference
inputs with a reference output (like `longest`). Structs that *hold* references
must annotate the field's lifetime, tying the struct's validity to the borrowed
data:

```rust
struct Excerpt<'a> { part: &'a str }   // an Excerpt cannot outlive the str it borrows
```

### `'static`

`'static` means "valid for the entire program." String literals are
`&'static str` (baked into the binary). A `'static` bound on a generic (`T:
'static`) means the type contains no non-`'static` borrows — often required when
handing data to detached work such as `thread::spawn`, or storing it type-erased.
It does **not** mean "leaks forever"; an owned `String` satisfies `'static`
because it borrows nothing. `std::thread::scope` is the deliberate exception for
fork-join work: the scope joins every child before returning, so scoped threads
may borrow non-`'static` locals.

## Reborrowing

You can create a shorter-lived borrow *through* an existing `&mut` — a
**reborrow**. This is how passing a `&mut T` to a function does not consume your
mutable reference:

```rust
fn bump(n: &mut i32) { *n += 1; }
let mut x = 0;
let r = &mut x;
bump(r);        // implicit reborrow: `&mut *r` is passed, r is usable afterward
bump(r);        // still valid — r was reborrowed, not moved
```

`&mut T` is not `Copy`, so naively it would move on each pass; reborrowing
(inserting `&mut *r`) is what the compiler does implicitly so `&mut` ergonomics
feel like references, not one-shot tokens.

## Variance at the User Level

You will occasionally hit **variance** — how subtyping of lifetimes flows through
generic types. The practical, non-theoretical summary:

| Type | Variance in its lifetime | Plain-English rule |
|------|--------------------------|--------------------|
| `&'a T` | covariant in `'a` and `T` | a longer-lived ref can be used where a shorter one is expected |
| `&'a mut T` | covariant in `'a`, **invariant** in `T` | you cannot substitute the pointee type |
| `Cell<T>`, `RefCell<T>` | invariant in `T` | interior mutability blocks variance |
| `fn(T) -> U` | contravariant in `T`, covariant in `U` | function subtyping |

Ninety percent of the time variance is invisible and correct. It surfaces as
mysterious "lifetime may not live long enough" errors around `&mut`, `Cell`, or
`PhantomData` ([20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)). The fix
is usually to loosen or tie lifetimes, not to understand the full theory — but
knowing "`&mut T` is invariant in `T`" explains why you cannot shorten the
pointee's lifetime behind a mutable reference.

## Old World -> New World Bridge

| C++ / .NET | Rust | Difference |
|------------|------|-----------|
| `const T&` (read) vs `T&` (write) | `&T` vs `&mut T` | Rust enforces exclusivity globally, not by convention |
| dangling reference / use-after-free | safe references cannot outlive the referent | Raw pointers may dangle, but dereferencing or reborrowing them requires `unsafe` |
| iterator invalidation (mutate while iterating) | compile error | The `&v` borrow blocks the `&mut v` push |
| `std::span` / `ArraySegment<T>` (borrowed view) | `&[T]` slice | Lifetime-checked, cannot outlive backing store |
| pointer lifetime "documented in comments" | `'a` in the type | Machine-checked, not prose |
| `ref`/`out` parameters (.NET) | `&mut` parameters | One exclusive borrow at a time |

The mental upgrade: in C++ the compiler trusts you that a `const T&` is still
valid. In Rust the compiler *proves* it. The same code pattern that a C++
reviewer eyeballs ("is this reference still alive?") is a mechanical check here.

## Common Confusion Points

- **`&mut` means exclusive, not just writable.** Even a `&mut` you never write
  through blocks other borrows, because exclusivity — not mutation — is the
  invariant.
- **"Lifetimes make my value live longer."** No. Lifetimes only *describe and
  check* existing lifetimes; annotating `'a` never extends anything.
- **Fighting a returned reference.** If you cannot express the borrow, the honest
  fix is often to return an owned `String`/`Vec` instead of `&str`/`&[T]`, or to
  restructure with `Rc`/`Arc` ([16](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)).
- **NLL surprises.** A borrow stored in a struct field or returned outlives the
  function body; that is where "still borrowed" errors usually come from.
- **`'static` panic.** A `T: 'static` bound (common with threads/`Box<dyn ...>`)
  does not require leaking — owned data satisfies it.
- **Self-referential structs are disallowed** by these rules; that need signals
  `Pin`, `Rc`, indices, or an arena ([14](14-ASYNC-FUTURES-AND-PINNING.md), [16](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)).

## Decision Cheat Sheet

| Situation | Do |
|-----------|-----|
| Read a value without owning it | `&T` |
| Mutate a value in place, caller keeps it | `&mut T` (exclusive) |
| Return a reference tied to an input | add `'a` linking input and output |
| Store a borrow in a struct | give the struct a lifetime param `struct S<'a>` |
| A borrow "won't die" though it should | check for a stored/returned reference; consider owning the data |
| Need shared mutable access | interior mutability, not `&mut` aliasing ([16](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)) |
| Hand data to a detached `thread::spawn` | ensure `'static` (own it or `Arc` it) ([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)) |
| Borrow locals in fork-join threads | `std::thread::scope` ([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)) |

## Primary Sources

- The Book, Ch. 4.2 (References and Borrowing): https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
- The Book, Ch. 10.3 (Validating References with Lifetimes): https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
- Reference — Lifetime elision: https://doc.rust-lang.org/reference/lifetime-elision.html
- Rustonomicon — Subtyping and Variance: https://doc.rust-lang.org/nomicon/subtyping.html

## Related Guides

- Previous: [03-OWNERSHIP-MOVES-COPY-AND-DROP.md](03-OWNERSHIP-MOVES-COPY-AND-DROP.md)
- Next: [05-STRUCTS-ENUMS-PATTERNS-AND-MATCH.md](05-STRUCTS-ENUMS-PATTERNS-AND-MATCH.md)
- Interior mutability & self-reference: [16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)
- Concurrency link: [15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)
