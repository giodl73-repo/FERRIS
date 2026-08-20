---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:smart-pointers-interior-mutability-and-self-reference
kind: guide
module: rust-language
section: languages
title: Smart Pointers, Interior Mutability, and Self-Reference
status: source-custody
source_custody: partial
current_path: rust-language/16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md
canonical_path: rust-language/16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md
backsource_ids: [proof-backfill:rust-language:16-smart-pointers-interior-mutability-and-self-reference]
concepts: [Box, Rc, Arc, Weak, Cell, RefCell, UnsafeCell, Cow, interior mutability, self-reference]
root_concepts: [smart pointers, interior mutability]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Smart Pointers, Interior Mutability, and Self-Reference

The borrow rules ([04](04-BORROWING-REFERENCES-AND-LIFETIMES.md)) are strict:
single owner, aliasing XOR mutation. Real programs sometimes need *shared*
ownership (graphs, trees with back-edges) or *mutation through a shared
reference* (caches, observer patterns). The smart-pointer and interior-mutability
types are the sanctioned escape valves — each moves one check from **compile time
to run time** in a controlled, still-memory-safe way. Knowing which type relaxes
which rule is the whole game.

```
+===============================================================================+
|            WHICH RULE ARE YOU RELAXING? (compile-time -> run-time)            |
+===============================================================================+

  OWNERSHIP POINTERS                       INTERIOR MUTABILITY (mutate via &T)
  ------------------                       -----------------------------------
  Box<T>   single owner, on heap           Cell<T>    move values in/out (Copy-friendly)
    (recursive types, trait objects)          get/set/replace, NO references handed out
  Rc<T>    shared owner, 1 thread           RefCell<T> hands out &/&mut, borrow rules
    ref-counted; drop when count=0             CHECKED AT RUNTIME (panics on violation)
  Arc<T>   atomic shared owner; cross-thread UnsafeCell<T> opts out of &T's normal
    only when T satisfies Send/Sync (15)       immutability rule (unsafe primitive)
  Weak<T>  non-owning ref (breaks cycles)

  COMBINE FOR REAL PATTERNS                 COW
  ------------------------                  ---
  Rc<RefCell<T>>   shared + mutable, 1 thr  Cow<'a, B>  borrow until you must mutate,
  Arc<Mutex<T>>    shared + mutable, N thr    then clone-on-write (Borrowed | Owned)
  Rc<T> + Weak<T>  parent<->child, no leak
```

## Ownership Pointers

### `Box<T>` — Single Owner on the Heap

`Box<T>` is the simplest smart pointer: it heap-allocates a `T` and owns it. Uses:
recursive types (a type cannot contain itself by value — `Box` breaks the
infinite size), trait objects (`Box<dyn Trait>`, [07](07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md)),
and moving a large value to the heap. It is `unique_ptr` with move semantics.

```rust
enum List { Cons(i32, Box<List>), Nil }   // Box gives the recursive type a finite size
let l = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
```

### `Rc<T>` and `Arc<T>` — Shared Ownership

`Rc<T>` (Reference Counted) allows *multiple owners* of the same heap value; the
value drops when the last `Rc` does. It is single-thread only (non-atomic count).
`Arc<T>` makes the reference count atomic, so ownership can cross threads when
`T` satisfies the required `Send`/`Sync` bounds
([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)). It does not make an arbitrary
`T` thread-safe or synchronize mutation. Both pointer types normally hand out
shared access; mutation requires an appropriate interior-mutability or locking
type.

```rust
use std::rc::Rc;
let a = Rc::new(vec![1, 2, 3]);
let b = Rc::clone(&a);          // cheap: bumps the refcount, no deep copy
println!("count = {}", Rc::strong_count(&a));   // 2
```

### `Weak<T>` — Non-Owning References (Break Cycles)

`Rc`/`Arc` cannot free a **reference cycle** (A owns B owns A) — the counts never
reach zero, and you leak. `Weak<T>` is a non-owning handle that does not keep the
value alive; upgrade it to an `Rc` with `.upgrade()` (returns `Option`, `None` if
already dropped). The canonical use: a child holds a `Weak` back-pointer to its
parent while the parent holds `Rc` to children.

```rust
use std::rc::{Rc, Weak};
struct Node { parent: RefCell<Weak<Node>>, children: RefCell<Vec<Rc<Node>>> }
// parent link is Weak -> no cycle -> no leak
```

## Interior Mutability: Mutate Through `&T`

The borrow checker forbids mutating through a shared `&T`. Interior-mutability
types provide a **safe** hole in that rule by moving the aliasing check to
runtime (or restricting the API).

### `Cell<T>` — Values Only, No References

`Cell` lets you `get`/`set`/`replace` the whole value through `&self`, but never
hands out a reference to the interior. Cheap and lock-free; ideal for `Copy`
types (counters, flags) accessed through shared references.

```rust
use std::cell::Cell;
struct Counter { n: Cell<u32> }
let c = Counter { n: Cell::new(0) };
c.n.set(c.n.get() + 1);          // mutate through &c, no &mut needed
```

### `RefCell<T>` — Runtime-Checked Borrows

`RefCell` hands out real `&T` (`.borrow()`) and `&mut T` (`.borrow_mut()`) but
enforces the aliasing-XOR-mutation rule **at runtime**: violate it and it
**panics** (`already borrowed: BorrowMutError`) instead of failing to compile.
You trade compile-time proof for flexibility.

```rust
use std::cell::RefCell;
let cache = RefCell::new(Vec::new());
cache.borrow_mut().push(1);      // exclusive borrow, checked at runtime
let len = cache.borrow().len();  // shared borrow — fine, previous guard dropped
```

### `UnsafeCell<T>` — The Primitive Underneath

`UnsafeCell<T>` is the primitive that opts its contents out of the usual
"shared reference implies immutable contents" rule. Its `.get()` exposes a raw
pointer; using that pointer must still preserve validity, provenance, reference
uniqueness, and any required synchronization. In particular,
`&UnsafeCell<T>` does **not** justify creating an aliased `&mut T`. Safe cell and
lock types encapsulate those obligations; use `UnsafeCell` directly only while
building such an abstraction
([17](17-UNSAFE-RUST-FFI-AND-ABI.md)).

### Composite Patterns

| Pattern | Meaning | Thread |
|---------|---------|--------|
| `Rc<RefCell<T>>` | shared + interior-mutable | single |
| `Arc<Mutex<T>>` | shared + synchronized-mutable | multi |
| `Arc<RwLock<T>>` | shared + many-readers/one-writer | multi |
| `Rc<T>` + `Weak<T>` | shared with back-edges, no cycle leak | single |

`Rc<RefCell<T>>` is the single-thread analog of `Arc<Mutex<T>>`; the mutex is
replaced by runtime borrow-checking. If you find yourself deep in `Rc<RefCell<>>`
graphs, consider an **arena** (a `Vec` + integer indices) instead — it is often
simpler and faster than pointer chasing.

## `Cow<'a, B>` — Clone on Write

`Cow` (Clone-on-Write) holds either `Borrowed(&B)` or `Owned(B::Owned)`. It lets
an API return a borrowed slice in the common case and only allocate when it must
modify — e.g. a function that usually passes text through unchanged but sometimes
escapes it:

```rust
use std::borrow::Cow;
fn sanitize(input: &str) -> Cow<str> {
    if input.contains('<') {
        Cow::Owned(input.replace('<', "&lt;"))   // allocate only when needed
    } else {
        Cow::Borrowed(input)                     // zero-copy fast path
    }
}
```

## Self-Reference Strategies

A struct cannot safely own a value *and* hold a reference into it (moving the
struct would dangle the reference — the borrow checker forbids it). Options, in
preferred order:

1. **Indices/arena** — store items in a `Vec`, refer by `usize`. Simple, fast,
   cache-friendly, no borrow gymnastics. Prefer this.
2. **`Rc`/`Arc` (+ `Weak` for cycles)** — shared ownership instead of references.
3. **`Pin` + `unsafe`** — for genuinely self-referential state machines (what the
   async transform generates, [14](14-ASYNC-FUTURES-AND-PINNING.md)); the
   `ouroboros`/`self_cell` crates encapsulate the unsafe.

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| `std::unique_ptr` (C++) | `Box<T>` | Move-only single owner |
| `std::shared_ptr` (C++) | `Arc<T>` (`Rc` = non-atomic) | Thread-safety is a type choice |
| `std::weak_ptr` | `Weak<T>` | Same cycle-breaking role |
| GC handles cycles (.NET/Java) | you break cycles with `Weak` | Refcounting can't; be deliberate |
| `mutable` member (C++) | `Cell`/`RefCell` | Interior mutability, but checked |
| `readonly` field you cheat past | `RefCell` (runtime-checked) | Panics instead of silent UB |
| copy-on-write string (C++ SSO era) | `Cow<str>` | Explicit, in the type |

For a C++ reader the mapping is nearly one-to-one (`Box`/`Arc`/`Weak` =
`unique`/`shared`/`weak_ptr`), with the addition that thread-safety is encoded
(`Rc` vs `Arc`). For a GC-language reader the new obligation is that
**reference-counting does not collect cycles** — you must reach for `Weak`.

## Common Confusion Points

- **`Rc` vs `Arc`.** Single-thread vs multi-thread. Using `Rc` across threads is
  a compile error; `Arc` pays for an atomic counter.
- **`Cell` vs `RefCell`.** `Cell` swaps whole values, no references, no panics.
  `RefCell` hands out references and **panics** on aliasing violations at runtime.
- **`RefCell` panics are your fault made visible.** `BorrowMutError` means two
  live borrows overlapped — restructure so borrows do not nest.
- **Reference cycles leak.** `Rc<RefCell<Rc>>` graphs can form cycles that never
  drop; use `Weak` for back-edges.
- **Reaching for `Rc<RefCell<T>>` reflexively.** Often an arena (`Vec` + indices)
  is simpler. Shared-mutable-graph is the last resort, not the first.
- **`Cow` is not free laziness.** It clones on the first mutation; if you always
  mutate, just own the data.
- **Self-referential structs need `Pin`/unsafe or indices** — plain references
  will not compile.

## Decision Cheat Sheet

| I need... | Use |
|-----------|-----|
| Heap allocation, single owner | `Box<T>` |
| Recursive type / trait object | `Box<T>` / `Box<dyn Trait>` |
| Shared ownership, one thread | `Rc<T>` |
| Shared ownership, many threads | `Arc<T>` when `T` is thread-safe ([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)) |
| Break a reference cycle | `Weak<T>` |
| Mutate a `Copy` value via `&self` | `Cell<T>` |
| Mutate via `&self`, need references | `RefCell<T>` (runtime-checked) |
| Shared + mutable, one thread | `Rc<RefCell<T>>` |
| Shared + mutable, many threads | `Arc<Mutex<T>>` |
| Borrow-or-own return value | `Cow<'a, B>` |
| A graph without pointer soup | arena: `Vec<T>` + indices |
| Build a new sync primitive | `UnsafeCell<T>` + `unsafe` ([17](17-UNSAFE-RUST-FFI-AND-ABI.md)) |

## Primary Sources

- The Book, Ch. 15 (Smart Pointers): https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
- std::boxed::Box: https://doc.rust-lang.org/std/boxed/struct.Box.html
- std::rc / std::sync::Arc: https://doc.rust-lang.org/std/rc/index.html
- std::cell (Cell/RefCell/UnsafeCell): https://doc.rust-lang.org/std/cell/index.html
- std::borrow::Cow: https://doc.rust-lang.org/std/borrow/enum.Cow.html

## Related Guides

- Previous: [15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)
- Next: [17-UNSAFE-RUST-FFI-AND-ABI.md](17-UNSAFE-RUST-FFI-AND-ABI.md)
- Borrow rules being relaxed: [04-BORROWING-REFERENCES-AND-LIFETIMES.md](04-BORROWING-REFERENCES-AND-LIFETIMES.md)
- Self-referential futures & Pin: [14-ASYNC-FUTURES-AND-PINNING.md](14-ASYNC-FUTURES-AND-PINNING.md)
