---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:lexing-parsing-ast-spans
kind: guide
module: rust-architecture
section: rust-architecture
title: Lexing, Parsing, the AST, and Diagnostic Spans
status: source-custody
source_custody: partial
current_path: rust-architecture/04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md
canonical_path: rust-architecture/04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md
backsource_ids: [mdloom-backfill:rust-architecture:04-lexing-parsing-ast-spans]
concepts: [lexer, parser, abstract syntax tree, source map, spans, error recovery, token stream]
root_concepts: [rust parsing]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Lexing, Parsing, the AST, and Diagnostic Spans

## The Big Picture

Rustc's front end starts with bytes and ends with an AST that is still close to
surface syntax: pre-name-resolution, pre-type-checking, and mostly
pre-desugaring. The work is organized around `SourceMap` and `Span`. Every token,
AST node, parser recovery suggestion, and macro expansion edge carries source
provenance forward so diagnostics can point at the right bytes later [15].

The stable contracts are the Rust Reference grammar/tokens and the public
`proc_macro::TokenStream` API. The `rustc_lexer`, `rustc_parse`, `rustc_ast`,
span encoding, symbol interner, parser recovery, and `-Z unpretty` formats are
rustc implementation details.

```
+===========================================================================+
|                         RUSTC FRONT-END INPUT                             |
|                                                                           |
|  files / stdin / virtual macro output                                     |
|       |                                                                   |
|       v                                                                   |
|  SourceMap: source files -> byte positions -> Span lo/hi + SyntaxContext  |
+---------------------------------------------------------------------------+
             |
             v
+---------------------------------------------------------------------------+
|  rustc_lexer                                                              |
|  raw tokens: identifiers, literals, punctuation, comments/whitespace      |
|  low-level shared crate; not a stability contract                         |
+---------------------------------------------------------------------------+
             |
             v
+---------------------------------------------------------------------------+
|  rustc_parse token stream                                                 |
|  spans attached · symbols interned · delimiters grouped as token trees    |
|  macro boundary currency: TokenTree / TokenStream                         |
+---------------------------------------------------------------------------+
             |
             v
+---------------------------------------------------------------------------+
|  hand-written parser                                                      |
|  recursive descent + expression precedence machinery + error recovery     |
+---------------------------------------------------------------------------+
             |
             v
+---------------------------------------------------------------------------+
|  rustc_ast                                                                |
|  faithful syntactic tree, before resolution and HIR lowering              |
|  feeds macro expansion/name resolution [05], then HIR [06]                |
+===========================================================================+
```

The diagram is intentionally front-loaded with spans. Rust diagnostics are not a
late formatting layer. They are made possible because source identity is threaded
through the compiler from the first byte.

---

## SourceMap and Span: Bytes Plus Provenance

`SourceMap` is rustc's global map from source files to byte positions. A `Span`
is a compact reference to a range in that map: conceptually low byte, high byte,
and an expansion/hygiene context. The exact representation is internal and has
changed before. The durable idea is that rustc tracks both "where in the file"
and "through which macro expansion path".

```
+--------------------------+         +------------------------------+
| SourceFile               |         | Span                         |
| name: src/main.rs        |         | lo: byte offset              |
| text: fn main() { ... }  | <------ | hi: byte offset              |
| line table               |         | ctxt: SyntaxContext          |
+--------------------------+         +------------------------------+
                                                |
                                                v
                                      +------------------------------+
                                      | Diagnostic label / suggestion|
                                      | Macro backtrace / call site  |
                                      +------------------------------+
```

| Span carries | Why it matters | Authority |
|--------------|----------------|-----------|
| Byte range | Exact label placement, edits, suggestions | rustc implementation |
| Source file mapping | Line/column display and multi-file diagnostics | rustc implementation |
| `SyntaxContext` | Macro hygiene and expansion provenance [05] | rustc implementation |
| Diagnostic labels | Primary/secondary spans, machine-applicable fixes [15] | rustc implementation |

Do not build tooling against rustc's internal span layout. Use stable compiler
output, `proc_macro` spans where applicable, or ecosystem tool APIs that accept
their own compatibility burden.

---

## Two-Stage Lexing and Symbol Interning

Rustc separates raw lexical recognition from the richer parser token stream.
`rustc_lexer` identifies low-level token kinds without owning the full compiler
context. `rustc_parse` then builds tokens with spans, delimiter structure, and
interned symbols used by the rest of rustc.

```
+---------------------+       +-----------------------+       +----------------------+
| bytes               | ----> | rustc_lexer           | ----> | rustc_parse tokens   |
| UTF-8 source text   |       | raw token kinds       |       | spans + interned ids |
+---------------------+       +-----------------------+       +----------------------+
                                                                   |
                                                                   v
                                                         +----------------------+
                                                         | Symbol interner      |
                                                         | identifier strings   |
                                                         | compact comparison   |
                                                         +----------------------+
```

`rustc_lexer` is intentionally small enough to be useful to tools such as
rust-analyzer and rustfmt [19]. That reuse should not be confused with a stable
Rust API promise. It is source in the rust-lang repository, versioned with the
compiler, and subject to the same internal-change policy as other rustc crates.

Editions enter here as well as later. Reserved keywords, raw identifiers such as
`r#type`, `dyn`, and edition-specific reservations like 2021 reserved prefixes
are recognized by the same compiler under edition-gated rules [01].

---

## Token Trees and the Macro Boundary

Rust macros do not consume an AST in the usual compiler-plugin sense. They
consume and produce token streams grouped into token trees: balanced delimiters,
punctuation, identifiers, and literals. That is the shared currency between the
parser, macro expansion [05], Cargo proc-macro builds [18], and the stable
`proc_macro` crate.

| Representation | Used by | Stability |
|----------------|---------|-----------|
| Internal rustc tokens | Parser, resolver, expander | Internal |
| Internal token trees | Macro expansion machinery | Internal |
| `proc_macro::TokenStream` | Stable procedural macro API | Stable public API |
| `proc_macro2::TokenStream` | Ecosystem compatibility layer | Ecosystem crate, not rustc |

```
macro invocation source
        |
        v
+-------------------+      grouped as       +----------------------+
| tokens with spans | --------------------> | token trees          |
+-------------------+                       | ( ... ), { ... }, [] |
        |                                   +----------------------+
        |                                           |
        v                                           v
 parser sees ordinary syntax                 proc macro sees TokenStream
 when not in macro position                  not rustc_ast nodes
```

This token-tree boundary is why macro expansion must be interleaved with parsing
and resolution rather than bolted on after a complete AST is already known.

---

## Parser, AST, and Recovery

The rustc parser is hand-written recursive descent, with precedence handling for
expressions rather than a generated LR/LALR parser. That is a deliberate
production-compiler trade: macro integration, edition-specific grammar, and
high-quality recovery are easier to tune in code than in a generated grammar.

| Area | rustc choice | Consequence |
|------|--------------|-------------|
| Parser architecture | Hand-written recursive descent | Direct control over diagnostics and recovery |
| Expressions | Precedence/Pratt-style machinery | Parses Rust operator structure without grammar explosion |
| Output | `rustc_ast` | Faithful surface tree, before resolution/desugaring |
| Recovery | Insert error nodes, skip to synchronizing tokens, suggest fixes | More errors per run; heuristics are internal |
| Next form | HIR [06] | Resolved, lowered, more compiler-friendly identity |

The AST is intentionally not the semantic tree. Paths have not been resolved,
traits have not been selected, types have not been inferred, and syntactic sugar
has not yet been lowered into the middle-level representation. If you want the
compiler's stable language surface, read the Reference. If you want rustc's AST
node names, read the source for a particular compiler revision.

---

## Concrete Trace: AST Dumps, Expansion Dumps, and Spans

Nightly `-Z unpretty` is a useful microscope, not a contract. It prints compiler
internal structures in whatever format is convenient for that rustc revision.

```rust
// src/main.rs
macro_rules! wrap {
    ($e:expr) => {{ Some($e) }};
}

fn main() {
    let x = wrap!(1 + 2);
    let y: i32 = "not an int";
}
```

```text
# Nightly/unstable: AST before macro expansion is fully lowered.
rustc +nightly -Z unpretty=ast-tree src/main.rs

# Nightly/unstable: expanded source-like view after macro expansion.
rustc +nightly -Z unpretty=expanded src/main.rs

# Stable: normal diagnostics carry spans.
rustc src/main.rs
error[E0308]: mismatched types
 --> src/main.rs:7:18
  |
7 |     let y: i32 = "not an int";
  |            ---   ^^^^^^^^^^^^ expected `i32`, found `&str`
  |            |
  |            expected due to this
```

Conceptually, the caret label above is backed by a span over the literal's byte
range, plus the expected-type span over `i32`. If the token came from a macro,
that span would also carry expansion context so rustc can choose whether to show
the call site, the macro definition, or a macro backtrace.

---

## Old world -> New world

| Old world / familiar model | Rust analogue | Important difference |
|----------------------------|---------------|----------------------|
| Roslyn `SourceText` | rustc `SourceMap` file contents | Rustc's map is private compiler state |
| Roslyn `TextSpan` / `Location` | rustc `Span` | Rust spans also carry macro hygiene context |
| Roslyn syntax trees | `rustc_ast` | Rust AST is internal; no stable compiler API |
| Roslyn hand-written parser | rustc hand-written recursive-descent parser | Same production bias: diagnostics over generated grammar purity |
| C# preprocessor tokens | Rust tokens | Rust token trees feed real hygienic macros, not just conditional text |
| Lisp reader forms | Rust token trees | Similar grouped-token currency, but with Rust spans and hygiene |

The Roslyn parallel is unusually strong for parsing: both systems favor
hand-written parsers because human diagnostics matter more than textbook parser
minimalism. The divergence is API policy. Roslyn exposes syntax trees as a
supported platform; rustc's AST is an internal detail.

---

## Decision Cheat Sheet

| Question | Look at | When | Who owns it |
|----------|---------|------|-------------|
| What syntax is legal Rust? | Rust Reference grammar and tokens | Language-contract questions | lang team / Reference |
| Where did this diagnostic point? | Span labels in compiler output | User-facing error analysis [15] | rustc diagnostics |
| What did rustc parse? | `rustc +nightly -Z unpretty=ast-tree` | Debugging a specific compiler | rustc internals, unstable |
| What did macros expand to? | `rustc +nightly -Z unpretty=expanded` or `cargo expand` | Debugging generated code [05] | `-Z` rustc / ecosystem tool |
| What API do proc macros receive? | `proc_macro::TokenStream` | Writing stable proc macros [18] | standard library / proc_macro |
| What token API can tools reuse? | `rustc_lexer` with version pinning | rustfmt/RA-style tooling [19] | rustc source, unstable |
| Why does parsing differ by edition? | Edition rules [01] | Keywords/path syntax surprises | lang / rustc parser |
| Where does semantic identity begin? | HIR lowering [06] | After AST/resolution boundary | rustc internals |

---

## Common Confusion Points

- **The AST is not HIR.** AST mirrors source syntax; HIR is lowered,
  owner-oriented, and closer to semantic compiler work [06].
- **A `Span` is not just line and column.** It is a compact byte-range reference
  plus expansion/hygiene context; line/column is derived for display.
- **`rustc_lexer` reuse is not API stability.** Tools can share the crate, but
  they inherit compiler-version coupling.
- **Token streams are not strings.** Macro input preserves token boundaries,
  delimiter groups, and spans; this is why Rust macros can be hygienic [05].
- **Parser recovery is heuristic.** Suggestions and inserted error nodes are
  engineered behavior, not part of the language contract.
- **`-Z unpretty` output is not a file format.** It is a nightly debugging aid;
  do not write durable pipelines that depend on its exact shape.

---

## Primary Sources

- **rustc-dev-guide: Lexing and Parsing** — official overview of raw lexing,
  token construction, parser architecture, and AST production.
- **rustc-dev-guide: The parser** — details on the hand-written parser,
  expression parsing, recovery, and parser diagnostics.
- **rustc-dev-guide: Macro expansion** — token trees, expansion inputs, and the
  parser/expander boundary.
- **rustc-dev-guide: Emitting Diagnostics** — spans, labels, suggestions, and
  structured diagnostic construction.
- **The Rust Reference: tokens and grammar** — stable language surface for
  lexical forms, keywords, literals, paths, expressions, and items.
- **`rust-lang/rust` source: `rustc_lexer`, `rustc_parse`, `rustc_ast`** —
  authoritative for a specific compiler revision, but unstable internal source.

*Cross-links:* this guide sits after the driver/query overview [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md)
and before macro expansion/name resolution [05](05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md).
HIR lowering begins in [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md), diagnostics
are covered in [15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md), build-script
and proc-macro mechanics in [18](18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md),
and ecosystem tools in [19](19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md).
For Rust syntax as a language topic, see `../rust-language/` where it exists.