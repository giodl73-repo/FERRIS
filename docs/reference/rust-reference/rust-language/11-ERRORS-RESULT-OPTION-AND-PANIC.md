---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:errors-result-option-and-panic
kind: guide
module: rust-language
section: languages
title: Errors - Result, Option, and Panic
status: source-custody
source_custody: partial
current_path: rust-language/11-ERRORS-RESULT-OPTION-AND-PANIC.md
canonical_path: rust-language/11-ERRORS-RESULT-OPTION-AND-PANIC.md
backsource_ids: [proof-backfill:rust-language:11-errors-result-option-and-panic]
concepts: [Result, Option, panic, error propagation, question-mark operator, From conversion, custom errors, anyhow, thiserror]
root_concepts: [error handling]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Errors — Result, Option, and Panic

Rust splits failure into two categories with different mechanisms:
**recoverable** errors are ordinary values (`Result<T, E>`, `Option<T>`)
propagated with `?`, while **unrecoverable** bugs use `panic!` (which unwinds or
aborts). There are no exceptions. The type signature tells you exactly what can
fail and how — a function that returns `Result<T, E>` can fail; one that returns
`T` cannot (barring a panic, which is a bug, not a control-flow tool).

```
+===============================================================================+
|                        THE FAILURE MODEL                                      |
+===============================================================================+

  RECOVERABLE (values)                    UNRECOVERABLE (panic)
  --------------------                    ---------------------
  Option<T>  = Some(T) | None             panic!("msg")   unwrap()   expect()
    "absence, no reason needed"           assert!  slice[out_of_bounds]  checked overflow
  Result<T,E> = Ok(T) | Err(E)                 |
    "failure, with a reason E"                 v  (default) UNWIND the stack, run Drop
                                             or ABORT (panic=abort in profile)
  PROPAGATE with ?                              |
  ----------------                         a panic is a BUG signal, not error handling
  let x = may_fail()?;   // Ok -> unwrap   catch: std::panic::catch_unwind (unwind mode;
                         // Err -> return          test/containment boundary only)

  ? CONVERTS the error via From
  -----------------------------
  fn f() -> Result<T, MyErr> {
     let a = io_op()?;   // io::Error --From--> MyErr automatically
  }
```

## Option: Absence Without a Reason

`Option<T>` replaces null. `None` means "no value," carrying no explanation. The
type forces you to handle absence before using the inner value — there is no null
dereference in safe Rust.

```rust
let map = std::collections::HashMap::from([("a", 1)]);
match map.get("a") {
    Some(v) => println!("{v}"),
    None => println!("missing"),
}
// combinators avoid the match:
let doubled = map.get("a").map(|v| v * 2).unwrap_or(0);   // 2
let v = map.get("b").copied().unwrap_or_default();        // 0
```

Key combinators: `map`, `and_then` (flatMap), `filter`, `unwrap_or`,
`unwrap_or_else`, `ok_or(err)` (convert to `Result`), `?` (propagate `None`).

## Result: Failure With a Reason

`Result<T, E>` carries the error value `E`. Pattern-match it, transform it, or
propagate it. The idiomatic path is propagation via `?`.

```rust
use std::fs;
fn read_config(path: &str) -> Result<String, std::io::Error> {
    let raw = fs::read_to_string(path)?;   // ? : Ok(v) -> v ; Err(e) -> return Err(e.into())
    Ok(raw.trim().to_owned())
}
```

## The `?` Operator and `From` Conversion

`?` is the whole ergonomic story. On `Ok(v)` it unwraps to `v`; on `Err(e)` it
`return Err(e.into())` — converting the error via the `From` trait
([06](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)). That auto-conversion is why a
function returning your `AppError` can `?` an `io::Error`, a `ParseIntError`, and
a `serde` error in the same body, as long as each has a `From` impl into
`AppError`.

```rust
// This:
let n: i32 = s.parse()?;
// desugars to:
let n: i32 = match s.parse() {
    Ok(v)  => v,
    Err(e) => return Err(e.into()),   // From<ParseIntError> for the fn's error type
};
```

`?` also works on `Option` (propagates `None`) and on any type implementing the
`Try` trait. **Caveat:** implementing `Try` for *your own* types (and the
`Residual`/`FromResidual` machinery underneath `?`) is **unstable/nightly**
(`try_trait_v2`). On stable you get `?` on `Option`, `Result`, and `ControlFlow`
— which covers essentially all real code. Do not build public APIs assuming you
can make custom `?`-able types on stable.

## Custom Error Types

For libraries, define a real error enum with variants per failure mode, implement
`std::error::Error` and `Display`, and provide `From` impls so `?` converts into
it. Doing this by hand is boilerplate; the ecosystem splits the work:

| Crate | For | What it does |
|-------|-----|--------------|
| **thiserror** | **libraries** | derive macro that generates `Error`/`Display`/`From` for your enum |
| **anyhow** | **applications / binaries** | one dynamic `anyhow::Error` (boxed `dyn Error`) + `.context()` |

```rust
// LIBRARY error with thiserror (derive generates Display + Error + From):
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("io failure: {0}")]
    Io(#[from] std::io::Error),        // From<io::Error> generated -> ? just works
    #[error("bad value: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

// APPLICATION code with anyhow (don't care about the exact type, want context):
use anyhow::{Context, Result};
fn run() -> Result<()> {
    let raw = std::fs::read_to_string("cfg").context("reading cfg")?;
    let n: i32 = raw.trim().parse().context("parsing cfg as int")?;
    Ok(())
}
```

**The boundary rule:** libraries expose *typed* errors (callers may want to match
on variants) via `thiserror`; applications collapse everything into
`anyhow::Error` and attach human-readable `.context()`. Do not export `anyhow` in
a library's public API — you rob callers of the ability to distinguish failures.
On the standard-library-only path, `Box<dyn std::error::Error + Send + Sync>` is
the no-dependency stand-in for `anyhow`.

## Panic: For Bugs, Not Flow

`panic!` signals a broken invariant: an index out of bounds, a failed `assert!`,
an `unwrap()` on `None`. On targets that support unwinding, Cargo profiles use
unwinding by default, so initialized locals are dropped and
`std::panic::catch_unwind` can contain the panic. A `panic = "abort"` profile,
or a target that supports only aborting panics, terminates instead and cannot be
caught.

- `unwrap()` / `expect("reason")` — panic on `None`/`Err`. Fine in tests,
  prototypes, and genuinely-impossible cases (document *why* with `expect`).
- `?` — the production default for propagation.
- `match` / `if let` — when this call site must handle both arms differently.

Panics are **not** exceptions to `try/catch` around. `catch_unwind` exists for
containment and test frameworks, not as a control-flow idiom. At FFI boundaries,
`extern "C"` is a non-unwind ABI and aborts if a Rust panic escapes;
`extern "C-unwind"` is the explicit ABI for supported cross-language unwinding.
Most C-facing APIs still catch and translate panics before the boundary; see
[17](17-UNSAFE-RUST-FFI-AND-ABI.md).

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| Exceptions (`throw`/`catch`) | `Result<T, E>` + `?` | Errors are values in the signature; propagation is explicit |
| Checked exceptions (Java) | `Result` in the return type | Same "declared failure," without the ceremony |
| `Nullable<T>` / null | `Option<T>` | No null deref; handling forced |
| `TryParse` out-param (.NET) | `Result` / `Option` return | Failure is the return value |
| `try { } catch (A \| B)` | `match err { A => .., B => .. }` | Pattern-match error variants |
| exception `.InnerException` chain | `source()` / anyhow `.context()` | Explicit cause chain |
| `Debug.Assert` | `assert!` / `debug_assert!` | Panics; for invariants, not user errors |
| unhandled exception crash | unhandled `Result` = compile warning; `unwrap()` panic | Compiler nudges you to handle |

The core shift for an exceptions veteran: propagation is *visible*. In C#/Java a
deep call can throw and you would never know from the signature; in Rust the
`Result` in the return type and the `?` at each hop make every fallible boundary
explicit. Propagation cost is a plain return, not stack unwinding.

## Common Confusion Points

- **`unwrap()` everywhere.** Fine while prototyping; a smell in production. Prefer
  `?`. Reserve `expect("invariant: ...")` for truly-impossible cases and document
  the reasoning.
- **`?` needs a `From` impl (or matching error type).** "the trait `From<X>` is
  not implemented" means your function's error type cannot absorb `X`; add a
  `From` (or use `thiserror`'s `#[from]`, or `anyhow`).
- **anyhow in a library API.** Do not. Return typed errors; let the app collapse
  them.
- **Panics are not error handling.** They mean "this should never happen." Do not
  `catch_unwind` as a `try/catch`.
- **`?` on `Option` vs `Result` don't mix implicitly.** Convert with `ok_or`/`ok`
  to move between them.
- **Overflow with checks enabled is a panic, not a `Result`.** Use `checked_*`
  for an `Option`-style API independent of profile settings
  ([02](02-BINDINGS-TYPES-AND-INFERENCE.md)).

## Decision Cheat Sheet

| Situation | Use |
|-----------|-----|
| Value might be absent (no reason) | `Option<T>` |
| Operation might fail (with reason) | `Result<T, E>` |
| Propagate failure to caller | `?` |
| Handle Ok/Err differently here | `match` / `if let` |
| Provide a default on absence/failure | `unwrap_or` / `unwrap_or_else` / `unwrap_or_default` |
| Convert `Option` <-> `Result` | `ok_or` / `.ok()` |
| Define a library error type | `thiserror` derive |
| Handle errors in an app/binary | `anyhow::Result` + `.context()` |
| No-dependency dynamic error | `Box<dyn Error + Send + Sync>` |
| Signal a broken invariant / bug | `panic!` / `assert!` / `expect` |
| Catch a panic at an FFI/test edge | `std::panic::catch_unwind` (not as flow control) |

## Primary Sources

- The Book, Ch. 9 (Error Handling): https://doc.rust-lang.org/book/ch09-00-error-handling.html
- std::result::Result: https://doc.rust-lang.org/std/result/enum.Result.html
- std::option::Option: https://doc.rust-lang.org/std/option/enum.Option.html
- std::error::Error: https://doc.rust-lang.org/std/error/trait.Error.html
- The `?`/Try trait (unstable): https://doc.rust-lang.org/std/ops/trait.Try.html

## Related Guides

- Previous: [10-STRINGS-TEXT-AND-UNICODE.md](10-STRINGS-TEXT-AND-UNICODE.md)
- Next: [12-MODULES-CRATES-PACKAGES-AND-VISIBILITY.md](12-MODULES-CRATES-PACKAGES-AND-VISIBILITY.md)
- `From` conversions: [06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)
- Panic across FFI: [17-UNSAFE-RUST-FFI-AND-ABI.md](17-UNSAFE-RUST-FFI-AND-ABI.md)
