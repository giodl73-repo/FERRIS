---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:macro-expansion-name-resolution
kind: guide
module: rust-architecture
section: rust-architecture
title: Macro Expansion, Hygiene, and Name Resolution
status: source-custody
source_custody: partial
current_path: rust-architecture/05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md
canonical_path: rust-architecture/05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md
backsource_ids: [mdloom-backfill:rust-architecture:05-macro-expansion-name-resolution]
concepts: [macro expansion, hygiene, name resolution, modules, imports, proc macros]
root_concepts: [macro expansion]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Macro Expansion, Hygiene, and Name Resolution

## The Big Picture

Rust macro expansion is not a preprocessor pass followed by ordinary name
binding. Expansion and name resolution are intertwined. Rustc must resolve a
macro path before it can expand the invocation, but expansion can introduce new
items, modules, imports, and invocations that must themselves be resolved. The
compiler iterates this loop until no expandable invocations remain, then lowers
the resolved AST toward HIR [06].

The stable contracts are the language rules in the Rust Reference, stable
`macro_rules!`, stable derive/attribute/function-like procedural macro forms, and
the public `proc_macro` crate. The resolver, hygiene representation,
`SyntaxContext`, expansion IDs, expansion ordering internals, and proc-macro
bridge/server are rustc implementation details.

```
+===========================================================================+
|                      EXPANSION + RESOLUTION LOOP                          |
|                                                                           |
|  parsed AST with token-tree macro invocations [04]                        |
|       |                                                                   |
|       v                                                                   |
|  collect invocations: macro_rules!, derive, attr, function-like proc      |
|       |                                                                   |
|       v                                                                   |
|  resolve macro path in module/import/macro namespaces                     |
|       |                                                                   |
|       v                                                                   |
|  expand invocation                                                        |
|       |-- declarative matcher/transcriber                                 |
|       |-- proc macro dylib running on HOST via stable proc_macro API      |
|       v                                                                   |
|  integrate produced tokens/items into AST                                 |
|       |                                                                   |
|       +---- new names/imports/macros? repeat until fixed point --------+  |
|                                                                        |  |
|  resolved AST: paths bound, modules/imports known, hygiene applied <---+  |
|       |                                                                   |
|       v                                                                   |
|  HIR lowering and middle-level identity [06]                              |
+===========================================================================+
```

This is the architectural point to keep: macros are part of name construction,
and name resolution is part of macro expansion. Treating either as a cleanly
separate phase will misread real rustc behavior.

---

## Macro Kinds and Execution Boundary

Rust has declarative macros expanded by rustc and procedural macros compiled as
separate host artifacts. Cargo is involved because a proc-macro crate is built
for the **host** compiler process, even when the final target is a different
architecture [18].

| Macro kind | Input / output | Runs where | Stability |
|------------|----------------|------------|-----------|
| `macro_rules!` | Token trees matched by patterns and transcribed to tokens | Inside rustc expander | Stable language feature |
| `macro` 2.0 | Newer declarative macro form | Inside rustc expander | Unstable |
| `#[derive(...)]` proc macro | Item tokens -> generated item tokens | Host dylib loaded by compiler bridge | Stable form + stable `proc_macro` API |
| Attribute proc macro | Annotated item tokens -> replacement tokens | Host dylib | Stable form + stable `proc_macro` API |
| Function-like proc macro | `name!(tokens)` -> tokens | Host dylib | Stable form + stable `proc_macro` API |
| Proc-macro bridge/server | Connects rustc internals to proc-macro crate | rustc implementation | Internal, version-sensitive |

```
+-------------------+        Cargo builds        +---------------------------+
| proc-macro crate  | -------------------------> | host dynamic library      |
+-------------------+                            +---------------------------+
                                                             |
                                                             v
+-------------------+       stable API          +---------------------------+
| rustc expander    | <------------------------ | proc_macro::TokenStream   |
+-------------------+                           +---------------------------+
        |
        v
 generated tokens integrated into the crate being compiled
```

The public API boundary is `proc_macro`. The internal representation on the rustc
side is not stable. Ecosystem crates such as `syn`, `quote`, and `proc_macro2`
are common engineering conveniences, but they are not the language contract.

---

## Hygiene: Spans Carry Expansion History

Hygiene prevents identifiers introduced by a macro from accidentally capturing or
colliding with identifiers at the call site. Rust implements this by carrying a
`SyntaxContext` as part of each token's span [04]. That context records expansion
history through internal expansion IDs. The representation is private and changes
with rustc; the semantic intent is the stable part.

```
caller source token                 macro-generated token
+----------------------+            +----------------------+
| ident: tmp           |            | ident: tmp           |
| span: caller ctxt    |            | span: expansion ctxt |
+----------------------+            +----------------------+
          |                                      |
          +------------------+-------------------+
                             v
                   resolver compares names plus context,
                   not spelling alone
```

A minimal local-capture example:

```rust
macro_rules! make_tmp {
    () => {{
        let tmp = 1;
        tmp
    }};
}

fn main() {
    let tmp = 40;
    let x = make_tmp!() + tmp; // 41; macro-local `tmp` is distinct
}
```

`macro_rules!` has mixed-site hygiene. Local variables and labels introduced in a
macro are protected in the way C preprocessor macros never were. Other names,
especially item paths, often resolve at the invocation site unless the macro uses
mechanisms such as `$crate` or explicit paths. That mixed behavior is language
semantics; the `SyntaxContext` machinery that realizes it is internal.

---

## Name Resolution: Modules, Imports, and Namespaces

The resolver (`rustc_resolve`) builds the module tree, resolves imports, binds
paths, resolves macro names, handles shadowing and ambiguity, and records what
each path denotes. The produced bindings feed HIR lowering [06] and later query
work [03]. The resolver's data structures and ordering details are not stable
APIs.

| Resolver concern | What rustc must decide | Notes |
|------------------|------------------------|-------|
| Module tree | Which `mod` items and files define modules | Edition/path rules affect interpretation [01] |
| Imports | `use`, glob imports, re-exports, `pub use` | Ambiguity and visibility checked here |
| Paths | `crate::`, `self::`, `super::`, extern prelude names | 2018+ uniform path rules matter |
| Macros | Which macro definition an invocation names | Interleaved with expansion |
| Namespaces | Type, value, and macro namespaces | Same spelling can occupy distinct namespaces |
| Visibility | `pub`, `pub(crate)`, restricted visibility | Determines reachable API surface |

```
+-------------------+        +-------------------+        +-------------------+
|module declarations| -----> | import graph      | -----> | path bindings     |
|expansion adds defs| <----- | use / glob / pub  |        | DefId/Res records |
+-------------------+        +-------------------+        +-------------------+
```

Rust's separate namespaces are practical, not exotic. A type name, a value name,
and a macro name can share spelling when they live in different namespaces;
tuple/unit struct constructors also occupy the value namespace. The resolver is
where these distinctions become concrete compiler identity.

---

## Import and Visibility Model

Rust 2018 and later made path interpretation more uniform, but the core model is
still a lexical module tree with explicit visibility. Name resolution is not just
"search everything on the crate graph." It is a scoped binding problem with
edition-dependent starting points.

| Syntax | Meaning | Common use |
|--------|---------|------------|
| `crate::x` | Absolute path from current crate root | Stable internal absolute paths |
| `self::x` | Current module | Disambiguating local module members |
| `super::x` | Parent module | Sibling module access |
| `use a::b` | Import a binding into scope | Local naming convenience |
| `pub use a::b` | Re-export a binding | Public API shaping |
| `pub(crate)` | Visible throughout current crate | Internal crate-wide API |
| `pub(in path)` | Visible within a restricted module subtree | Deliberate encapsulation |

```rust
mod inner {
    pub struct Widget;
}

pub use crate::inner::Widget as PublicWidget;

fn build() -> PublicWidget {
    PublicWidget
}
```

The `pub use` above creates a public API path independent of the physical module
where `Widget` is defined. Resolution records that the path denotes the same
item; artifact metadata and downstream crates later consume that identity [13].

---

## Concrete Trace: Expansion Views and Re-exports

Two inspection routes are common: `cargo expand`, an ecosystem tool, and
`rustc -Z unpretty=expanded`, a nightly rustc debugging aid. Neither expanded
source form is a stable serialization of rustc internals.

```text
# Ecosystem tool, not built into rustc. Useful for proc-macro-heavy crates.
cargo install cargo-expand
cargo expand

# Nightly/unstable rustc view. Requires a nightly toolchain.
rustc +nightly -Z unpretty=expanded src/main.rs
rustc +nightly -Z help

# Cargo visibility into host/target units, including proc macros.
cargo build -v
```

A re-export and macro together exercise the loop:

```rust
pub mod api {
    pub struct Widget;
}

pub use crate::api::Widget;

macro_rules! make_widget {
    () => { Widget };
}

fn f() -> Widget {
    make_widget!()
}
```

The macro name must be resolved before expansion. The `Widget` produced by the
macro must then be resolved in the invocation context. If the macro expansion had
introduced a new item or import, rustc would integrate it and continue resolving
until the expansion/resolution work reaches a fixed point.

---

## Old world -> New world

| Old world / universal model | Rust analogue | Why it matters |
|-----------------------------|---------------|----------------|
| C/C++ preprocessor macros | Rust declarative macros | Rust tokens carry hygiene; accidental capture is controlled |
| Roslyn Source Generators | Rust procedural macros | Compile-time code component emits code into compilation |
| Compiler plugin that sees syntax | Proc macro over `TokenStream` | Stable API is tokens, not rustc AST/HIR |
| Lexical scope resolver | `rustc_resolve` | Modules/imports/re-exports are explicit binding graph work |
| Type/value namespace split | Rust type/value/macro namespaces | Same spelling can denote different categories |
| Build host vs target tools | Proc macros/build scripts [18] | Macro code runs on the host, not the final target |

The strongest bridge is Roslyn Source Generators for procedural macros: a
compile-time component produces code consumed by the compilation. The strongest
contrast is C macros: Rust keeps token structure and hygiene instead of doing
text substitution.

---

## Decision Cheat Sheet

| Question | Use / inspect | When | Who owns it |
|----------|---------------|------|-------------|
| Need simple syntactic generation | `macro_rules!` | Pattern-to-token expansion without code execution | Rust language / rustc expander |
| Need derive/attribute/function-like code generation | Procedural macro crate | Generated impls, DSLs, annotations | `proc_macro` stable API + Cargo host build |
| Need to see expanded code | `cargo expand` | Day-to-day debugging | Ecosystem tool |
| Need rustc's own expanded view | `rustc +nightly -Z unpretty=expanded` | Compiler investigation | rustc internal, unstable |
| Need avoid item path capture | `$crate` or explicit paths | Exported declarative macros | Rust macro semantics |
| Need shape public API | `pub use`, visibility modifiers | Re-export and facade design | Rust Reference / resolver |
| Need know where a path points | Name resolution docs / compiler diagnostics | Ambiguity, shadowing, import failures | rustc resolver |
| Need stable compiler integration | `proc_macro::TokenStream` only | Proc macro authoring | standard library proc_macro |

---

## Common Confusion Points

- **Macro expansion and name resolution are interleaved.** Rustc cannot expand a
  macro until it resolves which macro is meant, and expansion can create more
  names to resolve.
- **Proc macros run on the host.** Cross-compilation still builds and executes
  proc-macro code for the compiler host, not the target device [18].
- **`proc_macro::TokenStream` is stable; rustc's token structures are not.** Do
  not confuse the public API with the compiler's internal representation.
- **Hygiene is not just alpha-renaming.** Spans carry syntax context and expansion
  history; the implementation is version-sensitive.
- **`macro_rules!` hygiene is mixed-site.** Locals and labels are protected, but
  item paths may resolve at the call site unless written carefully.
- **Namespaces are separate.** Types, values, and macros have distinct binding
  spaces; some declarations create names in more than one namespace.
- **Expanded source is not the compiler's final IR.** After expansion and
  resolution, rustc lowers to HIR [06], then proceeds through type, MIR, borrow,
  and codegen stages.

---

## Primary Sources

- **rustc-dev-guide: Macro expansion** — official rustc architecture for the
  expansion queue, fixed-point behavior, and integration with parsing.
- **rustc-dev-guide: Name resolution** — resolver responsibilities, module and
  import resolution, and path binding internals.
- **rustc-dev-guide: Hygiene** — syntax contexts, expansion data, and the
  implementation of hygienic macro behavior.
- **The Rust Reference: macros** — stable language rules for macro invocation,
  `macro_rules!`, procedural macro forms, and macro scoping.
- **The Rust Reference: paths and name resolution** — stable rules for paths,
  namespaces, imports, preludes, and editions.
- **The Rust Reference: visibility and privacy** — `pub`, restricted visibility,
  module privacy, and re-export behavior.
- **Standard library `proc_macro` docs** — stable public API exposed to
  procedural macro crates.
- **`rust-lang/rust` source: `rustc_expand`, `rustc_resolve`** — authoritative
  for a given compiler revision, but unstable implementation source.

*Cross-links:* token streams and spans come from [04](04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md),
then resolved AST lowers into HIR in [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md).
The driver/query context is [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md),
Cargo's proc-macro and host-tool boundary is [18](18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md),
and ecosystem tooling is [19](19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md).
For language-level macro syntax, see `../rust-language/` where it exists.