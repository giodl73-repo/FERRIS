---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:structs-enums-patterns-and-match
kind: guide
module: rust-language
section: languages
title: Structs, Enums, Patterns, and Match
status: source-custody
source_custody: partial
current_path: rust-language/05-STRUCTS-ENUMS-PATTERNS-AND-MATCH.md
canonical_path: rust-language/05-STRUCTS-ENUMS-PATTERNS-AND-MATCH.md
backsource_ids: [mdloom-backfill:rust-language:05-structs-enums-patterns-and-match]
concepts: [structs, enums, algebraic data types, destructuring, match, guards, exhaustiveness, let-else, if let, while let]
root_concepts: [algebraic data types]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Structs, Enums, Patterns, and Match

Rust's data-modeling core is algebraic data types: **structs** are product types
("this AND that") and **enums** are sum types ("this OR that"), and `match` is
the exhaustive eliminator that forces you to handle every case. If you have used
F# discriminated unions, Scala case classes, or TypeScript discriminated unions,
this is the same idea with a borrow-checked twist. The payoff is that illegal
states become *unrepresentable* and the compiler proves you handled everything.

```
+===============================================================================+
|                      ALGEBRAIC DATA TYPES + PATTERN MATCH                     |
+===============================================================================+

  PRODUCT (struct) = AND                 SUM (enum) = OR
  ----------------------                 ---------------
  struct Point { x: f64, y: f64 }        enum Shape {
    -> has x AND y                          Circle(f64),               // tuple variant
  struct Meters(f64);   // newtype          Rect { w: f64, h: f64 },   // struct variant
  struct Unit;          // zero-size        Dot,                       // unit variant
                                         }  -> is Circle OR Rect OR Dot

  MATCH = exhaustive eliminator          THE PATTERN TOOLBOX
  -----------------------------          -------------------
  match s {                              literal   1   "s"   'c'
    Shape::Circle(r) => ...,             binding   x   ref y  mut z
    Shape::Rect { w, h } => ...,         wildcard  _   ..
    Shape::Dot => ...,                   struct    Point { x, y }
  }  // compiler ERRORS if a case        tuple     (a, b, _)
     // is missing (exhaustiveness)      or        1 | 2 | 3
                                         range     0..=9
                                         guard     n if n > 0
```

## Structs: Three Flavors

```rust
struct Point { x: f64, y: f64 }     // named-field
struct Meters(f64);                 // tuple struct (newtype when 1 field)
struct Marker;                      // unit struct (zero-sized)

let p = Point { x: 1.0, y: 2.0 };
let p2 = Point { x: 5.0, ..p };     // struct update: take rest from p
let m = Meters(3.0);
let dist = m.0;                     // tuple-struct field by index
```

The single-field tuple struct — the **newtype** — is a workhorse: `struct
UserId(u64)` gives you a distinct type that cannot be confused with a raw `u64`
or an `OrderId(u64)` without adding stored state; optimized code normally erases
the wrapper ([20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)).
The struct-update syntax `..p` fills remaining fields from another instance
(moving or copying them per field).

Methods live in `impl` blocks; there is no inheritance ([06](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)):

```rust
impl Point {
    fn origin() -> Self { Point { x: 0.0, y: 0.0 } }   // associated fn (no self)
    fn dist(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
}
```

## Enums: Sum Types With Payloads

Rust enums are true tagged unions — each variant may carry different data. This
is far richer than a C enum (which is just named integers).

```rust
enum Event {
    Click { x: i32, y: i32 },   // struct-like payload
    Key(char),                  // tuple payload
    Close,                      // no payload
}
```

`Option<T>` and `Result<T, E>` are just library enums (`Some`/`None`,
`Ok`/`Err`), which is why the same pattern-matching machinery handles them
([11](11-ERRORS-RESULT-OPTION-AND-PANIC.md)). Enums are how you make illegal
states unrepresentable: a connection that is `Disconnected | Connecting { since:
Instant } | Connected { socket: TcpStream }` cannot hold a socket while
disconnected — the type forbids it.

## Match: Exhaustive by Construction

`match` requires that the arms cover *every* possible value; omit a case and the
program does not compile. This is the feature that turns "did I handle the new
enum variant?" from a runtime surprise into a compile error — invaluable when a
shared enum grows a variant.

```rust
fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rect { w, h } => w * h,
        Shape::Dot => 0.0,
    }   // remove any arm -> compile error: non-exhaustive patterns
}
```

The pattern grammar composes: bind (`r`, `w`), destructure (`Rect { w, h }`),
alternate (`1 | 2`), range (`0..=9`), wildcard (`_`), rest (`..`), and add a
**guard** for conditions the pattern alone cannot express:

```rust
match msg {
    Event::Key(c) if c.is_ascii_digit() => handle_digit(c),
    Event::Key(c) => handle_char(c),
    Event::Click { x, y } => handle_click(x, y),
    _ => {}                                       // catch-all
}
```

Two subtleties: a guard makes the arm non-exhaustive on its own (you still need a
fallthrough), and matching on a reference (`match &shape`) automatically binds
inner fields by reference (**match ergonomics** / default binding modes), so you
rarely write `ref` by hand anymore.

## `if let`, `while let`, and `let ... else`

Full `match` is overkill when you care about one variant. The sugar:

```rust
if let Some(v) = maybe {          // run block only if it matches
    use_value(v);
} else {
    // optional else
}

while let Some(item) = stack.pop() {   // loop until the pattern stops matching
    process(item);
}

// let-else (stable since Rust 1.65): bind or diverge, keeping the binding in scope
let Some(config) = load_config() else {
    eprintln!("no config");
    return;                        // the else block MUST diverge (return/break/panic)
};
use_config(config);               // config is in scope here, no rightward drift
```

`let ... else` is the ergonomic win for the "extract or bail" pattern that
otherwise forces an `if let ... { ... } else { return }` with the happy path
nested inside. It flattens control flow. `matches!(x, Pat)` is a companion macro
returning a `bool` for quick predicate checks.

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| C `enum` (named ints) | Rust `enum` (tagged union) | Variants carry typed payloads |
| F# discriminated unions / Scala `sealed` | Rust `enum` | Same sum-type modeling |
| TypeScript discriminated union + `switch` | `enum` + `match` | `match` exhaustiveness is enforced, not opt-in |
| `switch` with `default` fallthrough (C#/Java) | `match` with `_` | No fallthrough-by-accident; arms are isolated |
| `is`/pattern matching (C# 9+) | `match` / `if let` | First-class, exhaustive |
| `class` + inheritance | `struct`/`enum` + traits | Composition over inheritance ([06](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)) |
| null checks scattered in code | `Option` + `match`/`if let` | Handling is forced by the type |

For a C# reader, the closest analogy is records + pattern matching, but Rust's
exhaustiveness is mandatory and its enums predate and exceed C#'s. For an F#/ML
reader, this is home territory — the twist is that payloads are owned/borrowed
under the ownership rules.

## Common Confusion Points

- **Non-exhaustive match won't compile.** That is the feature. Add the missing
  arm or a `_`. Prefer listing variants over `_` for enums you own, so adding a
  variant *forces* a revisit.
- **Guards break exhaustiveness.** `n if cond => ...` does not prove totality;
  you still need an unguarded fallback arm.
- **`if let` chains lose exhaustiveness.** They are convenience, not proof; use
  `match` when you must handle all cases.
- **`let-else` requires a diverging `else`.** The `else` block must `return`,
  `break`, `continue`, or `panic!` — it cannot fall through with a value.
- **Match ergonomics can hide `&`.** Matching `&value` binds fields by reference
  automatically; you usually do not need `ref`/`ref mut` anymore, but know why
  your bindings are references.
- **`#[non_exhaustive]` enums** from other crates force a `_` arm even if you
  match every current variant — a deliberate API-evolution tool ([20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)).

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| Model "has A and B" | `struct` |
| Model "is A or B or C" | `enum` |
| A distinct type over one value | newtype `struct Id(u64)` |
| Handle every case of an enum | `match` (exhaustive) |
| React to just one variant | `if let` |
| Loop while a pattern matches | `while let` |
| Extract-or-bail without nesting | `let ... else` |
| A boolean "does it match?" | `matches!(x, Pat)` |
| Condition a pattern arm | guard: `Pat if cond =>` |
| Make illegal states impossible | encode invariants in enum variants |

## Primary Sources

- The Book, Ch. 5 (Structs): https://doc.rust-lang.org/book/ch05-00-structs.html
- The Book, Ch. 6 (Enums and Pattern Matching): https://doc.rust-lang.org/book/ch06-00-enums.html
- The Book, Ch. 18 (Patterns and Matching): https://doc.rust-lang.org/book/ch18-00-patterns.html
- Reference — Patterns: https://doc.rust-lang.org/reference/patterns.html
- let-else RFC 3137: https://rust-lang.github.io/rfcs/3137-let-else.html

## Related Guides

- Previous: [04-BORROWING-REFERENCES-AND-LIFETIMES.md](04-BORROWING-REFERENCES-AND-LIFETIMES.md)
- Next: [06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)
- Option/Result as enums: [11-ERRORS-RESULT-OPTION-AND-PANIC.md](11-ERRORS-RESULT-OPTION-AND-PANIC.md)
- API evolution with `#[non_exhaustive]`: [20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)
