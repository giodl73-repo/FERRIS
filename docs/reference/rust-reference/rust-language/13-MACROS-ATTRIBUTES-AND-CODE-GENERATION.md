---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:macros-attributes-and-code-generation
kind: guide
module: rust-language
section: languages
title: Macros, Attributes, and Code Generation
status: source-custody
source_custody: partial
current_path: rust-language/13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md
canonical_path: rust-language/13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md
backsource_ids: [mdloom-backfill:rust-language:13-macros-attributes-and-code-generation]
concepts: [macro_rules, declarative macros, hygiene, fragment specifiers, attributes, derive, procedural macros, build scripts, code generation]
root_concepts: [macros]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Macros, Attributes, and Code Generation

Rust macros transform **tokens and syntactic fragments**, not raw text like C's
preprocessor and not a type-checked semantic AST. Expansion happens before the
generated Rust is resolved and type-checked. `macro_rules!` provides mixed-site
hygiene that prevents many accidental captures; procedural macros emit tokens
unhygienically and must construct paths and names deliberately. The two families
are **declarative** macros (`macro_rules!`, token-pattern expansion) and
**procedural** macros (Rust functions transforming a `TokenStream`, including
custom `#[derive]` macros used by `serde`, `clap`, and friends).

```
+===============================================================================+
|                       MACRO / CODEGEN LANDSCAPE                               |
+===============================================================================+

  DECLARATIVE (macro_rules!)              PROCEDURAL (host proc-macro crate)
  --------------------------              -----------------------------------
  pattern -> template, mixed hygiene      fn(TokenStream) -> TokenStream
  vec![], println!, matches!              three kinds:
  match on FRAGMENTS:                        #[derive(Serialize)]  derive macro
    $x:expr $t:ty $i:ident $p:pat            #[route(GET, "/")]    attribute macro
    $s:stmt $b:block $l:literal              sql!( ... )           function-like proc macro
  repetition: $( ... )*  $( ... ),+       lives in a `proc-macro = true` crate
                                          uses syn + quote to parse/generate

  ATTRIBUTES (metadata on items)          TRUST BOUNDARY
  ------------------------------          --------------
  #[derive(Debug, Clone)]  auto-impl      proc macros + build.rs run ARBITRARY code
  #[cfg(test)]             conditional     on YOUR machine at BUILD time.
  #[inline]  #[repr(C)]   codegen hints    Auditing dependencies matters:
  #[allow(dead_code)]     lint control     a malicious macro can do anything cargo can.
```

## Declarative Macros: `macro_rules!`

A declarative macro matches input token patterns and expands to a template. It is
"macro by example": you write the shape of the call and the shape of the output.

```rust
macro_rules! my_vec {
    () => { Vec::new() };
    ($($x:expr),+ $(,)?) => {{        // one-or-more exprs, optional trailing comma
        let mut v = Vec::new();
        $( v.push($x); )+             // repeat the push for each matched $x
        v
    }};
}
let v = my_vec![1, 2, 3];            // expands to the push sequence
```

**Fragment specifiers** constrain what a metavariable matches — this is why
macros are more robust than text substitution:

| Specifier | Matches | Example |
|-----------|---------|---------|
| `expr` | an expression | `1 + 2`, `f(x)` |
| `ty` | a type | `Vec<u8>` |
| `ident` | an identifier | `foo` |
| `pat` | a pattern | `Some(x)` |
| `stmt` / `block` | a statement / `{...}` | |
| `literal` | a literal | `42`, `"s"` |
| `tt` | a single token tree (most flexible) | anything balanced |

**Mixed-site hygiene:** loop/block labels and local variables introduced by a
`macro_rules!` expansion are resolved at the macro definition site, while other
symbols are generally resolved at the invocation site. Thus a local `let mut v`
inside the macro does not capture the caller's `v`. This removes major classes
of C preprocessor capture bugs, but it is not full semantic hygiene and does not
apply to procedural macro output.

## Attributes

Attributes are `#[...]` annotations attached to items. Some are pure metadata,
some drive codegen, some are hooks for proc macros. The essentials:

| Attribute | Effect |
|-----------|--------|
| `#[derive(Trait, ...)]` | auto-generate impls (compiler built-ins or custom derive macros) |
| `#[cfg(...)]` / `#[cfg_attr(...)]` | conditional compilation ([18](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)) |
| `#[test]` / `#[bench]` | mark test/bench functions ([19](19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md)) |
| `#[repr(C)]` / `#[repr(transparent)]` | control memory layout ([17](17-UNSAFE-RUST-FFI-AND-ABI.md)) |
| `#[inline]` / `#[inline(always)]` | inlining hints |
| `#[allow/warn/deny(lint)]` | lint control |
| `#[non_exhaustive]` | API evolution ([20](20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md)) |
| `#![...]` (inner) | applies to the enclosing crate/module (`#![no_std]`) |

## Procedural Macros

Proc macros are Rust functions, compiled into a special `proc-macro` crate, that
receive a `TokenStream` and return a `TokenStream`. They run *in the compiler* at
build time. Three kinds:

- **Custom derive macros** (`#[derive(Serialize)]`) — generate trait impls from
  a struct/enum definition. `serde` and `thiserror` provide proc-macro derives;
  standard derives such as `Clone` and `Debug` are compiler-provided built-ins
  using the same surface syntax.
- **Attribute macros** (`#[tokio::main]`, `#[route(GET, "/")]`) — rewrite the
  annotated item arbitrarily.
- **Function-like macros** (`sqlx::query!(...)`) — like `macro_rules!` but with
  full Rust code driving the expansion (e.g. checking SQL against a live DB
  schema at compile time).

Authoring uses the ecosystem trio: `proc-macro2`, `syn` (parse tokens into an
AST), and `quote` (generate tokens from a template).

```rust
// Consuming a derive macro (the common case) — serde generates the impl:
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Config { name: String, retries: u32 }
```

## The Build-Time Trust Boundary

This is the security point every senior engineer must internalize: **proc macros
and `build.rs` build scripts execute arbitrary code on the build machine, with
the privileges of the build.** A dependency's proc macro can read your files,
hit the network, or tamper with output — the same power `cargo` itself has. Rust
gives *runtime* memory safety; it does **not** sandbox *build-time* code.
Consequences:

- Audit dependencies (especially proc-macro and build-script crates); pin
  versions via `Cargo.lock`; use `cargo vet`/`cargo audit`/`cargo deny` in CI.
- `cargo expand` shows what a macro expands to — invaluable for debugging *and*
  for auditing what generated code actually does.
- Reproducible, sandboxed CI builds limit blast radius.

A `build.rs` at package root runs before compilation (for codegen, linking to
native libs, probing the environment) — same trust caveat applies.

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| C preprocessor `#define` | `macro_rules!` | Syntactic fragments + mixed-site hygiene avoid many capture bugs |
| C++ templates (codegen) | generics ([06](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)) + macros | Generics for types, macros for syntax |
| C# source generators (Roslyn) | proc macros | Same "generate code at compile time" role |
| .NET attributes + reflection | attributes + derive macros | Codegen at compile time, not runtime reflection |
| Java annotation processors | derive/attribute macros | Direct analog |
| `T4` templates / codegen tools | `build.rs` + `quote` | Integrated into the build |
| runtime reflection/serialization | `#[derive(Serialize)]` (compile-time) | No reflection cost; generated statically |

The bridge that lands for a .NET reader: **derive macros are Roslyn source
generators**, and `#[derive(Serialize)]` replaces runtime reflection-based
serialization with statically generated code — faster and checked at compile
time. The bridge that lands for a C reader: `macro_rules!` is `#define` without
the footguns, because hygiene and fragment types prevent textual accidents.

## Common Confusion Points

- **Macros are not functions.** `println!` (with `!`) is a macro; it can take a
  variable number of typed arguments and generate code, which a function cannot.
- **Declarative vs procedural.** `macro_rules!` for simple pattern expansion in
  the same crate; proc macros (separate `proc-macro` crate) for derives,
  attributes, and anything needing to parse Rust syntax.
- **Hygiene differs by family.** `macro_rules!` locals and labels get mixed-site
  hygiene; procedural macros are unhygienic. To make a declarative macro refer
  to a caller-chosen name, pass it as an `ident` fragment.
- **`derive` needs the trait's derive macro in scope.** `#[derive(Serialize)]`
  requires the `serde` derive feature; a missing feature yields "cannot find
  derive macro."
- **Build-time code is unsandboxed.** Treat proc-macro and build-script
  dependencies as trusted-code decisions, not just library choices.
- **Debug expansions with `cargo expand`.** When a macro misbehaves, expand it;
  do not guess.

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| Variadic / syntax-shaped helper in one crate | `macro_rules!` |
| Auto-implement a trait from a type def | derive macro (`#[derive(...)]`) |
| Rewrite/annotate an item | attribute proc macro |
| DSL / compile-time-checked call syntax | function-like proc macro |
| Conditionally compile code | `#[cfg(...)]` ([18](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)) |
| Control memory layout for FFI | `#[repr(C)]` ([17](17-UNSAFE-RUST-FFI-AND-ABI.md)) |
| Generate code / link native libs pre-build | `build.rs` |
| See what a macro produces | `cargo expand` |
| Vet build-time trust | `cargo audit` / `cargo deny` / `cargo vet` |

## Primary Sources

- The Book, Ch. 19.5 (Macros): https://doc.rust-lang.org/book/ch19-06-macros.html
- Reference — Macros: https://doc.rust-lang.org/reference/macros.html
- Reference — Procedural macros: https://doc.rust-lang.org/reference/procedural-macros.html
- Reference — Attributes: https://doc.rust-lang.org/reference/attributes.html
- The Little Book of Rust Macros: https://veykril.github.io/tlborm/
- The Cargo Book — Build scripts: https://doc.rust-lang.org/cargo/reference/build-scripts.html

## Related Guides

- Previous: [12-MODULES-CRATES-PACKAGES-AND-VISIBILITY.md](12-MODULES-CRATES-PACKAGES-AND-VISIBILITY.md)
- Next: [14-ASYNC-FUTURES-AND-PINNING.md](14-ASYNC-FUTURES-AND-PINNING.md)
- cfg & features: [18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)
- repr & FFI layout: [17-UNSAFE-RUST-FFI-AND-ABI.md](17-UNSAFE-RUST-FFI-AND-ABI.md)
