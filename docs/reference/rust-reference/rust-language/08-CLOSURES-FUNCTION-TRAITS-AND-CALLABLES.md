---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:closures-function-traits-and-callables
kind: guide
module: rust-language
section: languages
title: Closures, Function Traits, and Callables
status: source-custody
source_custody: partial
current_path: rust-language/08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md
canonical_path: rust-language/08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md
backsource_ids: [mdloom-backfill:rust-language:08-closures-function-traits-and-callables]
concepts: [closures, capture modes, Fn, FnMut, FnOnce, move closures, function pointers, callbacks, higher-order functions]
root_concepts: [closures]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Closures, Function Traits, and Callables

A closure in Rust is an anonymous struct that captures its environment. Every
closure implements `FnOnce`; closures callable through mutable or shared access
also implement `FnMut` or `Fn`. Which traits apply follows from how the body uses
its captures. Understanding closures is really understanding capture-by-borrow
vs capture-by-move, because that is where the ownership rules
([03](03-OWNERSHIP-MOVES-COPY-AND-DROP.md), [04](04-BORROWING-REFERENCES-AND-LIFETIMES.md))
meet callbacks. Get the three traits straight and higher-order Rust — iterators,
async, thread spawns, callbacks — stops being mysterious.

```
+===============================================================================+
|                    CLOSURES = CAPTURED ENVIRONMENT + CALL TRAIT               |
+===============================================================================+

  CAPTURE MODES (compiler picks the least it needs)
  -------------------------------------------------
  reads a var          -> captures &T        (shared borrow)
  mutates a var        -> captures &mut T     (exclusive borrow)
  moves/consumes a var -> captures T          (by value)
  `move` keyword       -> force capture-by-value for ALL captures

  THE THREE CALL TRAITS (a hierarchy)
  -----------------------------------
        FnOnce   call by value; MAY move from captures         (super-trait)
          ^
          |  every Fn is FnMut is FnOnce
        FnMut    mutates captures; callable many times, needs &mut self
          ^
          |
        Fn       callable many times via &self; no move-out required

  +-------------+---------------+------------------+-------------------------+
  | trait       | self receiver | can call...      | closure body...         |
  +-------------+---------------+------------------+-------------------------+
  | Fn          | &self         | repeatedly       | does not move from env  |
  | FnMut       | &mut self     | repeatedly       | mutates captures        |
  | FnOnce      | self          | once via bound   | may move from captures  |
  +-------------+---------------+------------------+-------------------------+
```

## Capture Modes: The Compiler Takes the Least It Needs

A closure captures each variable by the *weakest* access its body requires:
shared borrow if it only reads, exclusive borrow if it mutates, by value if it
consumes. Since the 2021 edition, capture is **disjoint** — a closure captures
`data.field`, not all of `data`, if that is all it touches.

```rust
let name = String::from("Ada");
let greet = || println!("Hi {name}");   // captures &name (read only) -> Fn
greet(); greet();                       // callable many times; name still usable after

let mut count = 0;
let mut inc = || count += 1;            // captures &mut count -> FnMut
inc(); inc();                            // needs `mut inc`; count is 2

let owned = String::from("gone");
let consume = move || drop(owned);      // `move` forces by-value capture -> FnOnce
consume();                               // callable once; owned is moved in
```

The `move` keyword forces *every* capture to be by value. You need it whenever
the closure must **outlive** the current scope: spawning a thread
([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)), returning a closure, or storing
one in a struct. Without `move`, the closure borrows locals that will be dropped,
and the borrow checker rejects it.

## The Three Traits and Why They Form a Hierarchy

- **`Fn`** — callable through `&self`; the call does not require moving out of
  or exclusively borrowing the closure environment. It may still perform side
  effects or mutate through `Cell`, atomics, or other interior mutability.
  Concurrent sharing additionally requires `Sync`, and transfer requires
  `Send`. Every `Fn` is also `FnMut` and `FnOnce`.
- **`FnMut`** — the body *mutates* captures; it holds `&mut self`, callable many
  times but not aliased. Every `FnMut` is also `FnOnce`.
- **`FnOnce`** — the baseline trait; `call_once` takes the closure by value.
  Every closure implements it. A closure that moves out of a capture implements
  only `FnOnce` and can be called at most once.

The hierarchy matters when you write functions that *accept* closures. Accept the
**weakest** trait that does the job, so callers have the most freedom:

```rust
fn call_twice<F: Fn()>(f: F) { f(); f(); }          // requires repeated calls via &self
fn map_each<F: FnMut(&str)>(items: &[&str], mut f: F) {
    for x in items { f(x); }                        // FnMut: may mutate state across calls
}
fn defer<F: FnOnce()>(f: F) { f(); }                // one-shot: most permissive to accept
```

Returning a closure uses `impl Fn` (static, [07](07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md))
or `Box<dyn Fn>` (dynamic):

```rust
fn multiplier(n: i32) -> impl Fn(i32) -> i32 { move |x| x * n }
fn callbacks() -> Vec<Box<dyn Fn()>> { vec![Box::new(|| println!("a"))] }
```

## Function Pointers vs Closures

A plain `fn` (function pointer, type `fn(i32) -> i32`) captures *nothing* and is a
single word, not a struct. Every `fn` coerces to `Fn`/`FnMut`/`FnOnce`, so APIs
that take `impl Fn(...)` accept both closures and bare functions. Use `fn`
pointers for FFI callbacks ([17](17-UNSAFE-RUST-FFI-AND-ABI.md)) where a stable,
non-capturing C ABI is required.

```rust
fn double(x: i32) -> i32 { x * 2 }
let ptr: fn(i32) -> i32 = double;            // function pointer, no captures
let list = [1, 2, 3];
let out: Vec<i32> = list.iter().map(|&x| x * 2).collect();  // closure
let out2: Vec<i32> = list.iter().copied().map(double).collect(); // fn works too
```

## Callbacks and Storage

Storing callbacks in structs is where capture and lifetimes collide. Options:

| Storage | Type | When |
|---------|------|------|
| Owned, static-dispatched | generic field `struct S<F: Fn()>{ cb: F }` | one callback type per instance, max speed |
| Owned, dynamic | `Box<dyn Fn()>` / `Box<dyn FnMut()>` | many callback types, heterogeneous |
| Borrowed | `&dyn Fn()` with a lifetime | callback outlived by the caller |
| Thread/async | `Box<dyn Fn() + Send + 'static>` | crossing threads ([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)) |

For a callback that runs on another thread or is stored long-term, you almost
always need `move` plus `+ Send + 'static` bounds.

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| C# `Func<>` / `Action<>` / lambda | closure + `Fn`/`FnMut`/`FnOnce` | Capture mode is part of the type, checked |
| Java functional interfaces / lambdas | closures | Same idea; ownership-aware |
| C++ lambda `[=]` (by copy) vs `[&]` (by ref) | `move ||` vs default `||` | Compiler infers the minimal capture; `move` = `[=]` |
| C++ `std::function` (type-erased) | `Box<dyn Fn>` | Explicit boxing/vtable |
| C function pointer callback | `fn` pointer / `extern "C" fn` | Non-capturing; FFI-safe |
| JS closures (capture by reference) | Rust closures (capture by borrow/move) | Borrow checker enforces validity |
| LINQ `.Select(x => ...)` | `.map(|x| ...)` | Iterator adapters take closures ([09](09-COLLECTIONS-ITERATORS-AND-RANGES.md)) |

The mental bridge for a C++ reader: Rust closures are like lambdas whose capture
list the compiler writes for you, defaulting to by-reference and upgrading to
by-value only when needed or when you say `move`. The `Fn`/`FnMut`/`FnOnce` split
encodes what a C++ `const`/mutable lambda distinction hints at, but with type
enforcement.

## Common Confusion Points

- **"Which `Fn*` do I accept?"** Accept the weakest that works (`FnOnce` if you
  call once, `FnMut` if calls need exclusive closure state, `Fn` if repeated
  calls work through shared access). Producing side: your closure *is* whatever
  its body demands.
- **Missing `move`.** Returning or spawning a closure that borrows a local fails
  the borrow check; add `move` to capture by value.
- **`move` does not mean "call once."** `move` is about capture *mode*; a
  `move` closure that only reads its captures is still `Fn`.
- **`FnMut` closures need `mut` bindings.** `let mut f = || count += 1;` — the
  binding must be `mut` to call it.
- **Each closure has a unique, unnameable type.** Two closures with identical
  bodies are different types; store them behind `impl Fn` or `Box<dyn Fn>`.
- **Function pointers vs closures.** Only non-capturing closures coerce to `fn`
  pointers; capturing ones do not.

## Decision Cheat Sheet

| Situation | Use |
|-----------|-----|
| Callback callable repeatedly through shared access | `Fn` / `impl Fn(...)` |
| Callback that mutates state across calls | `FnMut` |
| One-shot callback that consumes data | `FnOnce` |
| Closure must outlive current scope | `move ||` |
| Return a closure (zero cost) | `-> impl Fn(...)` |
| Store heterogeneous callbacks | `Box<dyn Fn()>` |
| Callback on another thread | `move` + `Send + 'static` bounds |
| FFI / C callback | `extern "C" fn` pointer ([17](17-UNSAFE-RUST-FFI-AND-ABI.md)) |

## Primary Sources

- The Book, Ch. 13.1 (Closures): https://doc.rust-lang.org/book/ch13-01-closures.html
- Reference — Closure expressions: https://doc.rust-lang.org/reference/expressions/closure-expr.html
- std::ops::Fn / FnMut / FnOnce: https://doc.rust-lang.org/std/ops/trait.Fn.html
- Edition guide — disjoint closure captures (2021): https://doc.rust-lang.org/edition-guide/rust-2021/disjoint-capture-in-closures.html

## Related Guides

- Previous: [07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md](07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md)
- Next: [09-COLLECTIONS-ITERATORS-AND-RANGES.md](09-COLLECTIONS-ITERATORS-AND-RANGES.md)
- Ownership behind capture: [03-OWNERSHIP-MOVES-COPY-AND-DROP.md](03-OWNERSHIP-MOVES-COPY-AND-DROP.md)
- Closures across threads: [15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)
