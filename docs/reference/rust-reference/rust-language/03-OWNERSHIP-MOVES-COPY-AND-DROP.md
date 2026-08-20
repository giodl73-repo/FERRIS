---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:ownership-moves-copy-and-drop
kind: guide
module: rust-language
section: languages
title: Ownership, Moves, Copy, and Drop
status: source-custody
source_custody: partial
current_path: rust-language/03-OWNERSHIP-MOVES-COPY-AND-DROP.md
canonical_path: rust-language/03-OWNERSHIP-MOVES-COPY-AND-DROP.md
backsource_ids: [proof-backfill:rust-language:03-ownership-moves-copy-and-drop]
concepts: [ownership, affine types, moves, Copy, Clone, Drop, RAII, destructors, partial moves]
root_concepts: [ownership]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Ownership, Moves, Copy, and Drop

Ownership is the mechanism that lets Rust free memory deterministically with no
garbage collector and no manual `free`. The model is **affine types**: every
value has exactly one owner, and a value can be *used at most once* by move. When
the owner goes out of scope, the value's destructor (`Drop`) runs. This is RAII
promoted from a C++ convention to a type-system guarantee.

```
+===============================================================================+
|                          THE OWNERSHIP MODEL                                  |
+===============================================================================+

  THREE RULES                              LIFECYCLE OF A VALUE
  -----------                              --------------------
  1. each value has ONE owner              let s = String::from("hi");  <- born
  2. owner drops -> value is freed              |  owner = s
  3. assign/pass -> ownership MOVES         let t = s;   <- MOVED: s dead, t owns
                                                 |
                                            }    <- t's scope ends -> Drop::drop(t)
                                                    heap buffer freed HERE

  MOVE vs COPY                             THE FOUR TRAITS
  ------------                             ---------------
  heap-owning types  -> MOVE              Copy   : bitwise dup, old stays valid
    String, Vec, Box, most structs       Clone  : explicit (maybe deep) dup
  plain-old-data     -> COPY              Drop   : runs a destructor at scope end
    i32, bool, char, &T, [T;N] of Copy   (Copy and Drop are mutually exclusive)
```

## Affine Semantics: Move by Default

Assignment, argument passing, and returning all **move** ownership for non-`Copy`
types. After a move the source binding is statically dead — using it is a compile
error, caught before the program runs.

```rust
let s1 = String::from("hello");
let s2 = s1;                 // MOVE: the String's (ptr,len,cap) header goes to s2
// println!("{s1}");         // ERROR: borrow of moved value: `s1`

fn consume(s: String) { /* s owned here; dropped at end unless moved out */ }
let s3 = String::from("x");
consume(s3);                 // s3 MOVED into the function
// s3 is now dead
```

A move is cheap: for a `String` it copies the three-word header (pointer,
length, capacity) and invalidates the source. The heap buffer is **not** copied
— that would be `clone()`. Crucially, Rust does *not* run a destructor on the
moved-from binding; ownership (and thus the drop responsibility) transferred.
An ordinary move therefore cannot double-drop: exactly one initialized place
retains responsibility for the destructor.

## Copy: The Opt-Out for Plain Data

Types that are cheap to duplicate bit-for-bit implement the `Copy` marker trait.
For them, `let y = x` *copies* and leaves `x` valid — the C-like behavior.

```rust
let x = 5;         // i32 is Copy
let y = x;         // COPY, not move
println!("{x} {y}");   // both valid: 5 5
```

`Copy` types: all scalars (`i32`, `f64`, `bool`, `char`), shared references `&T`
(but not `&mut T`), and tuples/arrays whose elements are all `Copy`. A user type
may implement `Copy` only when every field is `Copy`, and `Copy` is incompatible
with implementing `Drop`. The compiler cannot recognize semantic ownership
hidden in an integer or raw handle, so API authors must not mark such resource
handles `Copy` unless duplicating the handle is genuinely valid.

## Clone: Explicit Duplication

`Clone` is the *explicit*, possibly expensive duplication. `String::clone`
allocates a new heap buffer and copies the bytes; `Rc::clone` merely bumps a
reference count. The point is that the cost is visible at the call site — you
never accidentally deep-copy a megabyte.

```rust
let a = String::from("data");
let b = a.clone();   // NEW heap allocation; a and b both own independent buffers
```

Idiom: reach for `clone()` when you genuinely need two independent owners.
Reaching for it merely to silence the borrow checker is a code smell — usually
the right fix is to pass a `&T` borrow ([04](04-BORROWING-REFERENCES-AND-LIFETIMES.md))
or restructure ownership. But a deliberate, documented clone is fine; premature
ownership gymnastics to avoid one small clone is often worse.

## Drop: Deterministic Destructors (RAII)

On normal scope exit, and during panic unwinding, Rust runs drop glue for each
still-initialized value in a language-defined order. Locals drop in reverse
declaration order. This is C++-style RAII with statically tracked moves, but
destructors are not guaranteed to run after process abort/exit or when safe code
deliberately leaks through `mem::forget` or a reference cycle. Memory safety
must never depend on a destructor eventually running.

```rust
struct Guard(&'static str);
impl Drop for Guard {
    fn drop(&mut self) { println!("dropping {}", self.0); }
}
fn main() {
    let _a = Guard("a");
    let _b = Guard("b");
}   // prints: dropping b, then dropping a  (reverse order)
```

You rarely implement `Drop` yourself — `String`, `Vec`, `File`, `MutexGuard`
already free their resource. You implement it for RAII wrappers: closing a
handle, releasing a lock, flushing a buffer. Rules of the road:

- You cannot call `.drop()` manually; use `std::mem::drop(x)` to drop early.
- `Drop::drop` takes `&mut self`, so you cannot move fields out of it.
- To *prevent* a drop (e.g. you transferred ownership to C), use
  `std::mem::forget` or `ManuallyDrop` — leaking is safe, unlike a double-free.

## Partial Moves and Reassembly

You can move *individual fields* out of a struct, leaving the rest usable — a
"partial move." The whole value is then partially invalid.

```rust
struct Pair { a: String, b: String }
let p = Pair { a: "x".into(), b: "y".into() };
let a = p.a;             // moves ONLY p.a out
// let whole = p;        // ERROR: use of partially moved value: `p.b` is fine, p is not
println!("{}", p.b);     // OK: p.b still owns its String
```

The borrow checker tracks moves at field granularity. To take a value out of a
`&mut` place without a full move, use `std::mem::take` (leaves `Default::default()`
behind) or `std::mem::replace` (leaves a value you supply) or `Option::take`.

## Old World -> New World Bridge

| C++ / .NET | Rust | Difference |
|------------|------|-----------|
| RAII destructors (C++) | `Drop` | Ordered on normal exit/unwinding; moves prevent double-drop |
| `std::move` + move ctor | move is the *default*; source is statically dead | No "moved-from valid but unspecified" state — it is unusable |
| copy constructor (implicit) | `Clone` (explicit `.clone()`) | Never implicit; cost is visible |
| trivially copyable / POD | `Copy` | Same idea; opt-in derive |
| `IDisposable` + `using` (.NET) | `Drop` at scope end | No `using` block needed; deterministic |
| GC finalizers (nondeterministic) | scope-based `Drop` (no GC) | Predictable on normal exit/unwinding; aborts and leaks can skip it |
| `std::unique_ptr` ownership | plain owned `T` / `Box<T>` | Ownership is language-level, not a wrapper |
| use-after-move bug (compiles in C++) | use-after-move is a compile error | Whole bug class eliminated |

The sharpest contrast is with C++: there, a moved-from object is left in a
"valid but unspecified state" and using it compiles (and often lurks as a bug).
In Rust the moved-from binding is *dead* — the compiler rejects any use. And
unlike .NET's GC, `Drop` timing is deterministic, so RAII patterns (locks,
transactions, temp files) are reliable.

## Common Confusion Points

- **"Why was my value moved? I only passed it."** Passing a non-`Copy` value by
  value *is* a move. Pass `&x` to lend it, or `x.clone()` to duplicate, or
  redesign so the callee returns ownership back.
- **Move vs Copy is invisible at the call site.** `let y = x;` moves or copies
  depending on whether `x`'s type is `Copy`. When in doubt, check the type.
- **Clone to appease the checker.** Sometimes correct, often a smell. Prefer
  borrowing; clone when two independent owners are genuinely needed.
- **Drop order.** Locals drop in *reverse* declaration order; struct fields drop
  in declaration order. This matters for locks and dependent resources.
- **You cannot make a self-referential struct with plain ownership.** A struct
  cannot own a value and also hold a reference into it — see
  [16](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md).
- **`Copy` and `Drop` are incompatible.** The compiler checks fields and trait
  impls, not semantic ownership hidden in raw handles; resource wrappers must
  enforce that design rule themselves.

## Decision Cheat Sheet

| Situation | Do |
|-----------|-----|
| Pass a value but keep using it | Borrow it: `&x` / `&mut x` ([04](04-BORROWING-REFERENCES-AND-LIFETIMES.md)) |
| Need two independent owners | `x.clone()` (deliberately) |
| Small POD you want value semantics | derive `#[derive(Copy, Clone)]` |
| Release a resource at scope end | Let `Drop` run, or implement it for a wrapper |
| Drop something early | `std::mem::drop(x)` |
| Take a field out of `&mut self` | `std::mem::take` / `replace` / `Option::take` |
| Hand ownership to C, skip the destructor | `std::mem::forget` / `ManuallyDrop` ([17](17-UNSAFE-RUST-FFI-AND-ABI.md)) |
| Move one field, keep the rest | partial move (compiler tracks it) |

## Primary Sources

- The Book, Ch. 4 (Understanding Ownership): https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
- Reference — Destructors: https://doc.rust-lang.org/reference/destructors.html
- std::marker::Copy: https://doc.rust-lang.org/std/marker/trait.Copy.html
- std::ops::Drop: https://doc.rust-lang.org/std/ops/trait.Drop.html
- std::mem (drop/forget/take/replace): https://doc.rust-lang.org/std/mem/index.html

## Related Guides

- Previous: [02-BINDINGS-TYPES-AND-INFERENCE.md](02-BINDINGS-TYPES-AND-INFERENCE.md)
- Next: [04-BORROWING-REFERENCES-AND-LIFETIMES.md](04-BORROWING-REFERENCES-AND-LIFETIMES.md)
- Shared ownership & interior mutability: [16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)
- Thread safety of shared data: [15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)
