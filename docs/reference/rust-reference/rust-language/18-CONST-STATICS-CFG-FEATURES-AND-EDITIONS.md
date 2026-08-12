---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:const-statics-cfg-features-and-editions
kind: guide
module: rust-language
section: languages
title: Const, Statics, cfg, Features, and Editions
status: source-custody
source_custody: partial
current_path: rust-language/18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md
canonical_path: rust-language/18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md
backsource_ids: [mdloom-backfill:rust-language:18-const-statics-cfg-features-and-editions]
concepts: [const, static, const eval, lazy initialization, cfg, features, target selection, editions, MSRV, rust-version]
root_concepts: [compile-time configuration]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Const, Statics, cfg, Features, and Editions

This guide covers the compile-time knobs: values computed at compile time
(`const`), program-lifetime storage (`static`), conditional compilation (`cfg`),
opt-in code paths (Cargo **features**), cross-compilation **targets**, the
language **editions** that let Rust evolve without breaking old code, and the
**MSRV** (Minimum Supported Rust Version) policy. Together they answer "what code
exists in this build, computed how, for which platform, on which compiler."

```
+===============================================================================+
|                   COMPILE-TIME CONFIGURATION LAYERS                           |
+===============================================================================+

  COMPILE-TIME VALUES              CONDITIONAL COMPILATION
  ------------------              -----------------------
  const NAME: T = expr;           #[cfg(test)]              only in `cargo test`
    value item; no unique identity #[cfg(target_os="linux")] per-OS code
  static S: T = expr;             #[cfg(feature="tls")]     per Cargo feature
    single address, 'static       cfg!(...) macro           compile-time bool value

  LAZY / RUNTIME INIT             FEATURES (Cargo.toml)      EDITIONS
  -------------------             ---------------------      --------
  OnceLock / LazyLock (std)       [features]                 2015 2018 2021 2024
    thread-safe one-time init       tls = ["dep:openssl"]    opt-in surface changes,
  (once_cell crate historically)  ADDITIVE only; a feature   same compiler; per-crate
                                    must never remove API     edition = "2024"
  TARGETS                                                    MSRV
  -------                                                    ----
  rustup target add ...           rust-version = "1.74"  (Cargo checks it)
  --target x86_64-...             the oldest compiler you promise to support
```

## `const` vs `static`

Both are compile-time-initialized, but they differ in identity:

| Property | `const` | `static` |
|----------|---------|----------|
| Semantics | a value evaluated at each use; storage is a codegen choice | one named memory location |
| Lifetime | no storage identity of its own | `'static`, lives whole program |
| Mutability | always immutable | immutable, or `static mut` (unsafe) |
| Address | taking a reference may create/promote storage, but no unique named address is promised | has one address (`&STATIC` is `'static`) |
| Type | must be annotated | must be annotated |

```rust
const MAX: u32 = 100;                    // value substituted semantically at each use
static GREETING: &str = "hello";         // one address, 'static
static COUNTER: AtomicU64 = AtomicU64::new(0);   // shared, thread-safe mutation via atomics
```

Use `const` for named literals and compile-time-known values; use `static` when
you need a single addressable instance (e.g. a global atomic or identity-bearing
table). The compiler may promote or deduplicate constant storage, but that is not
part of a const item's contract. Avoid `static mut` — accessing it is `unsafe` and
race-prone ([17](17-UNSAFE-RUST-FFI-AND-ABI.md)); use atomics or a `OnceLock`.

## Const Evaluation

Rust evaluates a growing subset of code at compile time. `const fn` can run in
const contexts (array lengths, other consts, const generics —
[20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)):

```rust
const fn square(n: usize) -> usize { n * n }
const SIZE: usize = square(8);           // computed at compile time
let buf = [0u8; SIZE];                   // array length from a const fn
```

The set of operations allowed in `const fn` has expanded steadily (loops,
`match`, and many const-stable inherent/free functions). As of Rust 1.95,
general const trait implementations and trait-method calls remain nightly, as do
more advanced const generics (`generic_const_exprs`). Inline `const { ... }`
blocks (force an expression into a const context) stabilized in **Rust 1.79**.

## Lazy / One-Time Initialization

A `static` initializer must be a const expression, so you cannot directly store a
`HashMap` built at runtime. The std tools (stabilized relatively recently):

- **`OnceLock<T>`** (stable since Rust 1.70) — set once, then read; thread-safe.
- **`LazyLock<T>`** (stable since Rust 1.80) — a `static` initialized by a closure
  on first access. This is the modern replacement for the `lazy_static!` macro
  and much of the `once_cell` crate.

```rust
use std::sync::LazyLock;
static CONFIG: LazyLock<Vec<String>> = LazyLock::new(|| {
    load_config_from_env()               // runs once, on first use
});
// later: CONFIG.iter() ...
```

If your MSRV predates 1.80, use the `once_cell` crate (`Lazy`/`OnceCell`), which
provided this for years and remains widely used.

## Conditional Compilation: `cfg`

`#[cfg(...)]` includes or excludes code at compile time based on predicates;
`cfg!(...)` yields a compile-time constant `bool` while both surrounding branches
still parse and type-check. Common predicates: `test`, `debug_assertions`,
`target_os`, `target_arch`, `target_pointer_width`, `feature = "..."`, and boolean
combinators `all(...)`, `any(...)`, `not(...)`.

```rust
#[cfg(target_os = "windows")]
fn platform() -> &'static str { "windows" }
#[cfg(not(target_os = "windows"))]
fn platform() -> &'static str { "unix-ish" }

#[cfg(test)]
mod tests { /* compiled only under `cargo test` */ }
```

`#[cfg_attr(cond, attr)]` conditionally applies another attribute (e.g.
`#[cfg_attr(feature = "serde", derive(Serialize))]`).

## Cargo Features

Features are named, **additive** compile-time flags declared in `Cargo.toml`. They
gate optional code and optional dependencies. The cardinal rule: **features must
be additive** — enabling a feature may add API but must never remove or change
existing behavior, because Cargo *unifies* the features requested by all consumers
of a crate in the dependency graph.

```toml
[features]
default = ["std"]
std = []
tls = ["dep:openssl"]            # enabling `tls` pulls in the optional openssl dep
serde = ["dep:serde", "chrono/serde"]
```

```rust
#[cfg(feature = "tls")]
pub fn connect_tls() { /* only exists when `tls` is on */ }
```

Because features unify across the graph, a non-additive feature (one that removes
an item or flips behavior) causes hard-to-debug breakage when two dependents ask
for different feature sets. Design them strictly additive. The **feature
resolver** version matters: resolver `"2"` (default in the 2021 edition) stopped
unifying features between build/host and normal dependencies; resolver `"3"`
(stabilized around Rust 1.84, default in the 2024 edition) is **MSRV-aware** —
it will not pick dependency versions requiring a newer compiler than your
`rust-version`.

## Targets and Cross-Compilation

A **target triple** (`x86_64-pc-windows-msvc`, `aarch64-apple-darwin`,
`wasm32-unknown-unknown`, `thumbv7em-none-eabihf`) names the platform. Add one
with `rustup target add ...` and build with `cargo build --target ...`. For
`no_std` / embedded, `#![no_std]` drops the standard library (keeping `core` and
optionally `alloc`) — the domain of `embassy` and bare-metal work.

## Editions

An **edition** (2015, 2018, 2021, 2024) is an opt-in bucket of *surface* language
changes — new keywords, changed defaults, idiom shifts — that would otherwise
break existing code. Crucially, **editions interoperate**: a 2024-edition crate
can depend on a 2015-edition crate and vice versa in one Cargo/rustc build graph;
each crate is parsed under its own edition and compiled together. This does
**not** imply a stable Rust ABI for separately built artifacts or different
compiler versions. You set the edition per crate:

```toml
[package]
edition = "2024"
```

Notable edition changes: 2018 brought the current module path system and
`async`/`await` keyword reservation; 2021 brought disjoint closure captures
([08](08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md)) and default resolver 2; 2024
(stable since **Rust 1.85**, early 2025) reserved the `gen` keyword, changed some
RPIT lifetime-capture defaults, and made `extern` blocks require `unsafe`
([17](17-UNSAFE-RUST-FFI-AND-ABI.md)). `cargo fix --edition` automates most
migrations.

## MSRV and `rust-version`

The **MSRV** is the oldest compiler you promise to compile on. Declare it so Cargo
can warn (and, with resolver 3, avoid picking too-new dependencies):

```toml
[package]
rust-version = "1.74"            # MSRV: builds fail-fast on older toolchains
```

Libraries often keep a conservative MSRV; applications track stable more freely.
Do not confuse MSRV (compiler version) with edition (language surface) — a crate
can be edition 2021 with MSRV 1.74.

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| `#define` / `#ifdef` (C) | `const` / `#[cfg(...)]` | Typed, hygienic, not textual |
| `constexpr` (C++) | `const fn` / const eval | Growing subset; some parts nightly |
| `static const` (C++) | `const` (value item) / `static` (addressed storage) | Two distinct concepts |
| `#if DEBUG` / build configs (.NET) | `#[cfg(debug_assertions)]` / features | Per-crate, additive features |
| conditional package refs / `#if` symbols | Cargo features | Additive, unified across graph |
| target frameworks / RIDs (.NET) | target triples | `rustup target add` + `--target` |
| language version (`<LangVersion>`) | edition | Interoperable across the graph |
| framework/runtime min version | MSRV (`rust-version`) | Compiler floor, checked by Cargo |
| `Lazy<T>` (.NET) | `LazyLock` / `OnceLock` | Std since 1.80/1.70; else `once_cell` |

## Common Confusion Points

- **`const` vs `static`.** A const has value semantics and no unique named
  storage identity; a static is one addressable location. Take `&STATIC` when
  you need a stable `'static` reference.
- **`static mut` is a trap.** Unsafe and racy; use atomics or `OnceLock`/`Mutex`.
- **Features must be additive.** A feature that removes/renames API breaks the
  graph when features unify. Design them purely additive.
- **Edition != compiler version.** Edition is language surface (interoperable);
  MSRV is the compiler floor. A crate has both, independently.
- **`LazyLock`/`OnceLock` are recent.** Great on modern stable; on older MSRVs use
  the `once_cell` crate.
- **Const eval has limits.** Advanced const generics/`generic_const_exprs` are
  nightly; do not assume arbitrary compile-time computation on stable.
- **Resolver version matters.** Set `resolver = "2"` or `"3"` at the workspace
  root; `"3"` is MSRV-aware.

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| A named compile-time constant | `const NAME: T = ...` |
| A single global instance / address | `static` (atomics for mutation) |
| Compute a value at compile time | `const fn` / `const { }` (1.79+) |
| Lazily initialize a global | `LazyLock` (1.80+) / `OnceLock` / `once_cell` |
| Compile code only for tests | `#[cfg(test)]` |
| Per-OS / per-arch code | `#[cfg(target_os = ...)]` |
| Optional functionality / deps | Cargo `[features]` (additive) |
| Cross-compile | `rustup target add` + `--target` |
| Bare-metal / embedded | `#![no_std]` (+ `alloc`) |
| Adopt new language surface | bump `edition`, run `cargo fix --edition` |
| Promise a compiler floor | `rust-version` (MSRV) |

## Primary Sources

- Reference — const and static items: https://doc.rust-lang.org/reference/items/constant-items.html
- Reference — Conditional compilation (`cfg`): https://doc.rust-lang.org/reference/conditional-compilation.html
- The Cargo Book — Features: https://doc.rust-lang.org/cargo/reference/features.html
- The Edition Guide: https://doc.rust-lang.org/edition-guide/
- std::sync::LazyLock / OnceLock: https://doc.rust-lang.org/std/sync/struct.LazyLock.html
- The Cargo Book — rust-version (MSRV): https://doc.rust-lang.org/cargo/reference/rust-version.html

## Related Guides

- Previous: [17-UNSAFE-RUST-FFI-AND-ABI.md](17-UNSAFE-RUST-FFI-AND-ABI.md)
- Next: [19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md](19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md)
- cfg_attr with derives: [13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md](13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md)
- const generics: [20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)
