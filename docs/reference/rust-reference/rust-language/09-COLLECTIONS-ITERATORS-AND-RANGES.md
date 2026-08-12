---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:collections-iterators-and-ranges
kind: guide
module: rust-language
section: languages
title: Collections, Iterators, and Ranges
status: source-custody
source_custody: partial
current_path: rust-language/09-COLLECTIONS-ITERATORS-AND-RANGES.md
canonical_path: rust-language/09-COLLECTIONS-ITERATORS-AND-RANGES.md
backsource_ids: [mdloom-backfill:rust-language:09-collections-iterators-and-ranges]
concepts: [Vec, HashMap, BTreeMap, HashSet, VecDeque, slices, iterators, iterator adapters, ranges, ExactSizeIterator, FusedIterator, DoubleEndedIterator]
root_concepts: [collections, iterators]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Collections, Iterators, and Ranges

The standard collections are unremarkable; the **iterator protocol** is the star.
Rust's `Iterator` is a lazy, statically typed pull-based abstraction: adapters like
`map`/`filter` build a nested struct that does nothing until a *consumer*
(`collect`, `sum`, `for`) drives it. Monomorphization gives the optimizer enough
structure to fuse many chains into allocation-free loops, but identical codegen
is an optimization outcome, not a language guarantee. The one place ownership
shows up: choosing among
`iter()` (borrow), `iter_mut()` (mutable borrow), and `into_iter()` (consume).

```
+===============================================================================+
|                        COLLECTIONS + THE ITERATOR PIPELINE                    |
+===============================================================================+

  OWNED COLLECTIONS           BORROWED VIEW        THE THREE ITER MODES
  -----------------           -------------        --------------------
  Vec<T>      growable array  &[T]  slice          v.iter()      -> &T
  VecDeque<T> ring buffer     &str  string slice   v.iter_mut()  -> &mut T
  HashMap<K,V> hash table                          v.into_iter() -> T  (consumes v)
  BTreeMap<K,V> sorted map
  HashSet<T>  hash set        RANGES               PIPELINE (lazy!)
  BTreeSet<T> sorted set      0..10   [0,10)       source ---adapters---> consumer
  BinaryHeap<T> max-heap      0..=10  [0,10]       iter() .map() .filter()  .collect()
  LinkedList<T> (rarely)      'a'..='z'            \____lazy, nothing runs____/  \runs/

  ITERATOR MARKER TRAITS
  ----------------------
  Iterator          : fn next(&mut self) -> Option<Item>   (the core)
  DoubleEndedIterator: also next_back()  -> .rev(), consume from both ends
  ExactSizeIterator : .len() is known exactly  (Vec, Range<usize>)
  FusedIterator     : once None, always None   (safe to call next() forever)
```

## The Core Collections

| Type | Backing | Use when | Ordering |
|------|---------|----------|----------|
| `Vec<T>` | contiguous heap array | default list; push/pop at end; index | insertion |
| `VecDeque<T>` | ring buffer | push/pop both ends (queue/deque) | insertion |
| `HashMap<K,V>` | hash table (implementation unspecified) | key -> value, expected O(1) | **unspecified** |
| `BTreeMap<K,V>` | B-tree | sorted keys, range queries | sorted by key |
| `HashSet<T>` / `BTreeSet<T>` | map with `()` values | membership | unspecified / sorted |
| `BinaryHeap<T>` | binary max-heap | priority queue, pop max | heap order |
| `LinkedList<T>` | doubly linked | almost never (poor locality) | insertion |

`Vec<T>` is the default; reach past it deliberately. `HashMap` iteration order is
**not** specified. The default `RandomState` seeds each map to resist
hash-collision attacks; the hasher and table algorithm are implementation
details and may change. Use `BTreeMap` when you need deterministic/sorted
iteration. The `entry` API is the idiomatic get-or-insert:

```rust
use std::collections::HashMap;
let mut counts: HashMap<&str, u32> = HashMap::new();
for word in "a b a c a".split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;   // one lookup, insert-if-absent
}
// counts = {"a": 3, "b": 1, "c": 1}
```

## Slices: Borrowed Views

A slice `&[T]` (or `&str` for text, [10](10-STRINGS-TEXT-AND-UNICODE.md)) is a fat
pointer — data pointer + length — that borrows a contiguous run from an array,
`Vec`, or another slice. Take slice parameters, not `&Vec<T>`, so your function
accepts arrays and vecs alike:

```rust
fn sum(xs: &[i32]) -> i32 { xs.iter().sum() }   // accepts &Vec, &[T;N], &[T]
sum(&vec![1, 2, 3]);
sum(&[4, 5, 6]);
```

## Iterators: Lazy, Composable, Zero-Cost

Everything hangs off one method: `fn next(&mut self) -> Option<Self::Item>`.
Adapters are lazy wrappers; consumers pull. Because each adapter is a distinct
monomorphized type, optimized builds can usually fuse a
`map().filter().sum()` chain into an allocation-free loop. Benchmark or inspect
codegen when the exact hot-path result matters.

```rust
let total: i32 = (1..=100)
    .filter(|n| n % 3 == 0)    // lazy
    .map(|n| n * n)            // lazy
    .sum();                    // CONSUMER: drives the pipeline, produces a value
```

| Adapter (lazy) | Does | Consumer (drives) | Does |
|----------------|------|-------------------|------|
| `map(f)` | transform each | `collect()` | build a collection |
| `filter(p)` | keep matching | `sum()`/`product()` | fold to a number |
| `enumerate()` | pair with index | `count()` | count items |
| `zip(other)` | pair two iters | `fold(init, f)` | general reduction |
| `take(n)`/`skip(n)` | slice the stream | `find(p)`/`any`/`all` | search/predicate |
| `flat_map(f)` | map+flatten | `for_each(f)` | side effects |
| `rev()` | reverse (needs DoubleEnded) | `min`/`max` | extremes |
| `chain(other)` | concatenate | `reduce(f)` | fold with no init |
| `peekable()` | look ahead | `collect::<Result<_,_>>()` | short-circuit on Err |

`collect` is turbofish-driven and can build any `FromIterator` type, including
collecting `Iterator<Item = Result<T, E>>` into `Result<Vec<T>, E>` (stops at the
first error) — a pattern worth memorizing:

```rust
let nums: Result<Vec<i32>, _> = ["1", "2", "x"].iter().map(|s| s.parse::<i32>()).collect();
// Err(ParseIntError) — short-circuits on "x"
```

## The Three Iteration Modes (Ownership)

```rust
let v = vec![1, 2, 3];
for x in v.iter()      { /* x: &i32   — v still owned afterward */ }
for x in v.iter_mut()  { /* x: &mut i32 — needs `let mut v` */ }
for x in v.into_iter() { /* x: i32    — v is CONSUMED, moved into the loop */ }
for x in &v            { /* sugar for v.iter()  */ }
for x in v             { /* sugar for v.into_iter() — CONSUMES v */ }
```

This is the single most common ownership surprise: `for x in v` *moves* the
collection, unlike C#'s `foreach` which borrows. Write `for x in &v` to iterate
by shared reference. `for x in &mut v` iterates mutably.

## Iterator Marker Traits

Beyond `Iterator`, three refinement traits let algorithms specialize:

- **`DoubleEndedIterator`** — has `next_back()`; enables `.rev()` and consuming
  from both ends (`Vec`, `Range`, slices).
- **`ExactSizeIterator`** — `.len()` is known precisely (not just a `size_hint`),
  letting `collect` pre-allocate exactly (`Range<usize>`, `Vec` iter).
- **`FusedIterator`** — guarantees `next()` keeps returning `None` after the
  first `None`; adapters can rely on this. `.fuse()` enforces it on any iterator.

You mostly consume these traits implicitly; you implement them when writing your
own iterator and want it to compose optimally.

## Ranges

`start..end` (half-open), `start..=end` (inclusive), `start..` (unbounded, for
slicing), `..end`, `..`. A `Range` is itself an iterator (`0..10` yields
`0..=9`). Ranges double as slice indices (`&v[2..5]`) and as `match` patterns
(`0..=9 => ...`). Note `0..10` and `0..=10` are different types (`Range` vs
`RangeInclusive`); most APIs take `impl RangeBounds`.

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| C# LINQ (`Select`/`Where`/`Aggregate`) | iterator adapters (`map`/`filter`/`fold`) | Lazy, statically typed adapters; no mandatory `IEnumerable` boxing |
| Java Streams | iterators | Similar laziness; monomorphized, not boxed |
| C++ ranges (C++20) | iterator adapters | Same lazy-pipeline philosophy |
| `List<T>` / `ArrayList` | `Vec<T>` | Growable heap array |
| `Dictionary<K,V>` | `HashMap` / `BTreeMap` | Choose hashed vs sorted explicitly |
| `SortedDictionary` | `BTreeMap` | Sorted iteration guaranteed |
| `foreach` (borrows) | `for x in &v` (borrow) / `for x in v` (moves!) | Default `for x in v` consumes |
| `yield return` generators | iterators (or nightly `gen` blocks) | You implement `Iterator` or chain adapters |

The LINQ bridge is exact in spirit, but Rust's adapter pipeline has concrete
static types rather than mandatory interface dispatch; optimized builds often
fuse it aggressively. The trap for a C# reader is `for x in v` moving the
collection; `foreach` never does that.

## Common Confusion Points

- **`for x in v` consumes `v`.** Use `&v` / `&mut v` to borrow. This is the top
  beginner surprise coming from C#/Java.
- **Iterators are lazy.** `v.iter().map(f);` does nothing without a consumer.
  Clippy warns on "iterator not used." Add `.collect()`, `.for_each()`, `.sum()`, etc.
- **`HashMap` order is unspecified and randomized.** Do not depend on it; use
  `BTreeMap` for order.
- **`collect` needs a target type.** `collect::<Vec<_>>()` or a typed binding;
  the turbofish tells it what to build.
- **`iter()` vs `into_iter()` on `&collection`.** `(&v).into_iter()` yields `&T`,
  not `T`; the consuming form needs an owned `v`.
- **`Range` types differ.** `0..10` (`Range`) vs `0..=10` (`RangeInclusive`) are
  distinct; write functions against `RangeBounds` if you must accept both.

## Decision Cheat Sheet

| I want... | Use |
|-----------|-----|
| A default growable list | `Vec<T>` |
| Queue / push-pop both ends | `VecDeque<T>` |
| Key -> value, fastest lookup | `HashMap<K,V>` |
| Key -> value, sorted / range queries | `BTreeMap<K,V>` |
| Priority queue | `BinaryHeap<T>` |
| Function param over any sequence | `&[T]` (not `&Vec<T>`) |
| Transform/filter a sequence | iterator adapters + a consumer |
| Iterate without consuming | `for x in &v` / `v.iter()` |
| Mutate in place while iterating | `v.iter_mut()` |
| Consume and take ownership of items | `v.into_iter()` |
| Collect fallible items | `collect::<Result<Vec<_>, _>>()` |

## Primary Sources

- The Book, Ch. 8 (Common Collections): https://doc.rust-lang.org/book/ch08-00-common-collections.html
- The Book, Ch. 13.2 (Iterators): https://doc.rust-lang.org/book/ch13-02-iterators.html
- std::collections overview: https://doc.rust-lang.org/std/collections/index.html
- std::iter::Iterator: https://doc.rust-lang.org/std/iter/trait.Iterator.html
- std::ops::Range / RangeBounds: https://doc.rust-lang.org/std/ops/struct.Range.html

## Related Guides

- Previous: [08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md](08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md)
- Next: [10-STRINGS-TEXT-AND-UNICODE.md](10-STRINGS-TEXT-AND-UNICODE.md)
- Closures feed adapters: [08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md](08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md)
- Fallible collect + `?`: [11-ERRORS-RESULT-OPTION-AND-PANIC.md](11-ERRORS-RESULT-OPTION-AND-PANIC.md)
