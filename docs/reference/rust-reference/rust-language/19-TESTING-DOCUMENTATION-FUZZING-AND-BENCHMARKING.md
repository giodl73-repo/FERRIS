---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:testing-documentation-fuzzing-and-benchmarking
kind: guide
module: rust-language
section: languages
title: Testing, Documentation, Fuzzing, and Benchmarking
status: source-custody
source_custody: partial
current_path: rust-language/19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md
canonical_path: rust-language/19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md
backsource_ids: [mdloom-backfill:rust-language:19-testing-documentation-fuzzing-and-benchmarking]
concepts: [unit tests, integration tests, doc tests, examples, compile-fail tests, property testing, fuzzing, criterion, Miri, sanitizers]
root_concepts: [testing]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Testing, Documentation, Fuzzing, and Benchmarking

Rust bakes testing and documentation into the toolchain: `#[test]` needs no
framework, documentation examples are compiled and run as tests, and the same
`cargo test` command drives unit, integration, and doc tests. Beyond the built-in
harness sits a rich verification ladder — property testing, fuzzing, the Miri
UB-detector, sanitizers, and `criterion` for statistically honest benchmarks.
This layer is how you convert "it compiles" into "it is correct and fast."

```
+===============================================================================+
|                        THE VERIFICATION LADDER                                |
+===============================================================================+

  LEVEL             TOOL                     CATCHES
  -----             ----                     -------
  unit tests        #[test] in src           logic per function/module
  integration       tests/*.rs (own crate)   the PUBLIC API end-to-end
  doc tests         ``` in /// comments      docs that lie / drift from code
  examples          examples/*.rs            runnable usage, compiled in CI
  compile-fail      trybuild crate           "this SHOULD not compile" (macros, bounds)
  property tests    proptest / quickcheck    invariants over RANDOM inputs (shrinks)
  fuzzing           cargo-fuzz (libFuzzer)   panics/UB on adversarial byte inputs
  UB detection      Miri (nightly)           use-after-free, UB in UNSAFE code
  sanitizers        ASan/TSan/MSan (nightly) memory/thread/leak errors at runtime
  benchmarking      criterion                statistically rigorous timing

  cargo test  --> runs unit + integration + doc tests in one command
```

## Unit Tests: `#[test]`, No Framework

Unit tests live alongside the code, conventionally in a `#[cfg(test)] mod tests`
so they compile only under `cargo test` ([18](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)).
They can call private items because they are inside the same module tree.

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds() { assert_eq!(add(2, 3), 5); }
    #[test]
    #[should_panic(expected = "overflow")]
    fn panics_on_bad_input() { do_thing_that_panics(); }
    #[test]
    fn fallible() -> Result<(), String> {          // tests can return Result
        if add(2, 2) == 4 { Ok(()) } else { Err("math broke".into()) }
    }
}
```

Assertions: `assert!`, `assert_eq!`, `assert_ne!`. Attributes: `#[ignore]` (skip
by default), `#[should_panic(expected = "...")]`. Tests run **in parallel** by
default; `cargo test -- --test-threads=1` serializes, and `-- --nocapture` shows
`println!` output.

## Integration Tests: The Public API

Files under `tests/` each compile as a **separate crate** that links your library
as an external dependency — so they exercise only the **public** API, exactly as a
real consumer would. This is a valuable constraint: it verifies your `pub` surface
is actually usable.

```rust
// tests/api.rs
use mycrate::add;                     // only pub items are visible
#[test]
fn public_add_works() { assert_eq!(add(1, 1), 2); }
```

## Doc Tests: Documentation That Cannot Rot

Code blocks in `///` doc comments are **compiled and executed** by `cargo test`.
This is Rust's signature feature: your examples cannot drift out of sync with the
API, because a broken example fails CI.

```rust
/// Adds two numbers.
///
/// ```
/// use mycrate::add;
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

Doc-test knobs: hide setup lines with a leading `#`, use ` ```no_run ` (compile
but don't execute — for network/IO examples), ` ```ignore ` (don't compile), and
` ```compile_fail ` (assert it fails to compile). `cargo doc --open` renders the
HTML; `#![deny(missing_docs)]` forces every public item to be documented.

## Examples and Compile-Fail Tests

`examples/*.rs` are runnable programs (`cargo run --example name`) that double as
compiled-in-CI usage documentation. For libraries whose *correctness includes
what must NOT compile* — proc macros, trait bounds, sealed traits
([20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)) — the **`trybuild`**
crate runs "compile-fail" tests that assert specific code is rejected with a
specific error, guarding your API's negative space.

## Property Testing

Instead of hand-picked cases, property tests assert an **invariant** over many
random inputs and, on failure, **shrink** to a minimal counterexample. The two
mainstays are `proptest` and `quickcheck`.

```rust
proptest::proptest! {
    #[test]
    fn roundtrip(s in ".*") {                       // any string
        let encoded = encode(&s);
        prop_assert_eq!(decode(&encoded), s);       // decode . encode == id
    }
}
```

Property tests find edge cases (empty inputs, huge values, boundary bytes) you
would never enumerate manually — the natural fit for parsers, serializers, and
data-structure invariants.

## Fuzzing

Fuzzing feeds **adversarial, coverage-guided random bytes** to a target to
provoke panics, crashes, or UB. `cargo-fuzz` (libFuzzer) is the standard front
end (nightly toolchain); `afl.rs` wraps AFL++. Fuzz targets are the front line for
anything parsing untrusted input.

```rust
// fuzz/fuzz_targets/parse.rs  (run via `cargo +nightly fuzz run parse`)
libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = mycrate::parse(data);        // must never panic / UB on any input
});
```

## Miri and Sanitizers (Exercising `unsafe`)

Safe Rust relies on sound `unsafe` foundations
([17](17-UNSAFE-RUST-FFI-AND-ABI.md)); those obligations still require review
and testing. Miri and sanitizers provide high-value **dynamic evidence** on the
paths, targets, and configurations they execute. They find defects, but they do
not prove an unsafe abstraction sound:

- **Miri** — an interpreter for Rust's mid-level IR that detects undefined
  behavior: out-of-bounds accesses, use-after-free, invalid alignment, data
  races, and violations of the aliasing model (Stacked/Tree Borrows). Run with
  `cargo +nightly miri test`. It is slow, and unsupported FFI/syscalls may need
  isolation or stubbing.
- **Sanitizers** — AddressSanitizer, ThreadSanitizer, LeakSanitizer,
  MemorySanitizer, available via `-Z sanitizer=...` on nightly for native-level
  memory/thread checking, especially across FFI. Availability and coverage vary
  by target.

Both require **nightly** in this workflow; wire supported jobs into CI for any
crate with meaningful `unsafe`, alongside invariant review and targeted tests.

## Benchmarking with criterion

Rust's built-in `#[bench]` harness is **unstable** (nightly only). The stable,
recommended tool is the **`criterion`** crate: it runs statistically rigorous
measurements (warm-up, outlier detection, confidence intervals) and detects
regressions between runs.

Install Criterion and disable Cargo's built-in benchmark harness for this target:

```bash
cargo add --dev criterion
```

```toml
# Cargo.toml
[[bench]]
name = "bench"
harness = false
```

```rust
// benches/bench.rs
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn add(a: i32, b: i32) -> i32 { a + b }

fn bench_add(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter(|| add(black_box(2), black_box(3)))
    });
}

criterion_group!(benches, bench_add);
criterion_main!(benches);
```

`std::hint::black_box` prevents the optimizer from folding away the work you are
trying to measure — essential for honest microbenchmarks.

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| xUnit / NUnit / MSTest | built-in `#[test]` | No framework/package needed |
| `[TestMethod]` / `[Fact]` | `#[test]` | Same idea, zero setup |
| `Assert.Equal` | `assert_eq!` | Macro; prints both values on failure |
| separate integration-test project | `tests/*.rs` (own crate) | Sees only the public API |
| XML doc comments (not executed) | doc tests (**executed**) | Examples can't rot — checked in CI |
| BenchmarkDotNet | `criterion` | Same statistical rigor |
| runnable samples / demos | `examples/*.rs` | Compiled in CI |
| FsCheck / property testing | `proptest` / `quickcheck` | With input shrinking |
| Valgrind / ASan on native code | Miri + sanitizers | Miri checks Rust's own UB model |

For a .NET reader the headline is that documentation examples are *executed
tests* and testing needs no external package. For a C/C++ reader, Miri is the
Valgrind analog but purpose-built for Rust's aliasing model, catching UB that
even ASan misses.

## Common Confusion Points

- **Doc examples are real tests.** A wrong example in `///` fails `cargo test`.
  Use `no_run`/`ignore`/`compile_fail` deliberately for non-executable snippets.
- **Integration tests see only `pub`.** Each `tests/*.rs` is its own crate; it
  cannot reach private items — that is the point.
- **Tests run in parallel.** Shared global/filesystem state causes flakiness;
  isolate state or serialize with `--test-threads=1`.
- **`#[bench]` is nightly.** Use `criterion` on stable; do not rely on the
  built-in bench harness for CI.
- **`black_box` or the optimizer lies.** Microbenchmarks without `black_box` may
  measure nothing because the compiler removed the work.
- **Miri/sanitizers need nightly** and are slow — run them in a dedicated CI job,
  not the fast path.
- **Property/fuzz are complementary.** Property tests check invariants; fuzzing
  hunts for crashes on adversarial input. Use both for parsers.

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| Test a function's logic (incl. private) | `#[test]` in `#[cfg(test)] mod tests` |
| Test the public API end-to-end | `tests/*.rs` integration test |
| Keep examples honest | doc tests in `///` |
| Ship runnable usage | `examples/*.rs` |
| Assert code must NOT compile | `trybuild` compile-fail tests |
| Check invariants over random input | `proptest` / `quickcheck` |
| Hunt crashes on untrusted bytes | `cargo-fuzz` |
| Detect UB in `unsafe` code | `cargo +nightly miri test` |
| Catch native memory/thread bugs | sanitizers (`-Z sanitizer=...`, nightly) |
| Benchmark reliably | `criterion` (+ `black_box`) |
| Force docs on public items | `#![deny(missing_docs)]` |

## Primary Sources

- The Book, Ch. 11 (Writing Automated Tests): https://doc.rust-lang.org/book/ch11-00-testing.html
- The Book, Ch. 14.2 (Documentation comments as tests): https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html
- rustdoc Book — Documentation tests: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html
- The Rust Fuzz Book (cargo-fuzz): https://rust-fuzz.github.io/book/
- Miri (rust-lang/miri): https://github.com/rust-lang/miri
- criterion.rs: https://bheisler.github.io/criterion.rs/book/

## Related Guides

- Previous: [18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)
- Next: [20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)
- Verifying unsafe code: [17-UNSAFE-RUST-FFI-AND-ABI.md](17-UNSAFE-RUST-FFI-AND-ABI.md)
- Test-only compilation: [18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)
