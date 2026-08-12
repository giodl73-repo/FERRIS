---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:ecosystem-tools-rustdoc-clippy-ra-miri
kind: guide
module: rust-architecture
section: rust-architecture
title: The Tooling Layer - rustdoc, rustfmt, clippy, rust-analyzer, and Miri
status: source-custody
source_custody: partial
current_path: rust-architecture/19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md
canonical_path: rust-architecture/19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md
backsource_ids: [mdloom-backfill:rust-architecture:19-ecosystem-tools-rustdoc-clippy-ra-miri]
concepts: [rustdoc, rustfmt, clippy, rust-analyzer, miri, compiler internal apis]
root_concepts: [rust tooling]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# The Tooling Layer - rustdoc, rustfmt, clippy, rust-analyzer, and Miri

## The Big Picture

The Rust tooling layer is best organized by coupling depth to `rustc`. Some
tools reuse compiler internals directly. One important tool, rust-analyzer,
deliberately does not call `rustc` as a library; it reimplements an IDE-shaped
front end because editor workloads need latency, partial programs, and stable
embedding more than batch-compiler fidelity. This is the architectural axis that
keeps the tool layer intelligible.

```
+===========================================================================+
|                         TOOL COUPLING TO RUSTC                            |
+---------------------------------------------------------------------------+
| shallow / syntactic                                                       |
|                                                                           |
|  rustfmt  -> parser/AST-level formatting                                  |
|                                                                           |
| medium / front-end consumer                                               |
|                                                                           |
|  rustdoc  -> rustc front end for docs + doctests                          |
|                                                                           |
| deep / compiler driver                                                    |
|                                                                           |
|  clippy   -> rustc driver with extra lints over HIR/MIR/types             |
|                                                                           |
| deep / MIR interpreter                                                    |
|                                                                           |
|  Miri     -> MIR/const-eval interpreter for UB detection                  |
|                                                                           |
| different architecture                                                    |
|                                                                           |
|  rust-analyzer -> reimplements lazy IDE front end; talks LSP              |
+===========================================================================+
        |                 |                 |                    |
        v                 v                 v                    v
 toolchain-pinned   unstable internals   stable CLI surface   Cargo/rustup glue
```

Ownership remains distinct. `rustdoc` ships with the project toolchain alongside
`rustc`. `rustfmt`, clippy, rust-analyzer, and Miri are project tools, commonly
installed as rustup components or with nightly toolchains, but they are not Cargo
and not the standard library. Cargo invokes some of them through subcommands;
Cargo does not own their analysis.

---

## Tool Coupling Map

| Tool | Owns / ships | Coupling model | Stable surface | Typical invocation |
|------|--------------|----------------|----------------|--------------------|
| rustdoc | Official docs tool, shipped with toolchain | rustc front-end consumer: parse, resolve, typeck; doctest harness | Documented CLI mostly stable; JSON output unstable | `cargo doc`, `rustdoc` |
| rustfmt | Rustfmt project component | Mostly syntactic AST formatting via parser/compiler crates | CLI/config where documented | `cargo fmt`, `rustfmt` |
| clippy | Clippy project component | rustc driver with extra lints over HIR/MIR/type info | Lint names and CLI are user-facing; internals unstable | `cargo clippy` |
| rust-analyzer | RA project component/editor server | Reimplements front-end with lazy query engine; shells to Cargo/rustc as needed | LSP behavior/config where documented | editor LSP, `rust-analyzer` |
| Miri | Miri project, nightly-oriented | MIR interpreter extending const-eval machinery | CLI documented, model evolves | `cargo +nightly miri test` |

The stable line is not "tool vs compiler." It is "documented command/API vs
private rustc crates." rustdoc, clippy, and Miri build on unstable compiler
internals and are therefore toolchain-pinned. rustfmt uses parser/AST machinery
and is also version-sensitive. rust-analyzer avoids that dependency by owning a
parallel analysis implementation.

---

## rustdoc and rustfmt: Documents and Syntax

`rustdoc` is not a Markdown scraper. It runs enough of the compiler front end to
understand items, names, cfgs, types, intra-doc links, visibility, and examples.
Doctests are compiled and run as tests; a code block in an API comment can fail
your build. That makes rustdoc closer to a compiler front-end consumer than to a
static site generator.

```
source + doc comments
        |
        v
rustdoc uses rustc front end: parse -> resolve -> typeck
        |
        +--> HTML docs
        +--> doctest crates -> rustc -> test execution [20]
        +--> JSON output (unstable output format)
```

`rustfmt` sits shallower. It parses Rust into syntax/AST form and reprints it
according to style rules, usually from `rustfmt.toml`. It does not need full type
information, borrow checking, or codegen. That is why it is fast, deterministic,
and resilient across incomplete build states compared with semantic tools.
Parsing and AST context are in
[04](04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md).

| Tool | Needs type info? | Main job | Common command |
|------|------------------|----------|----------------|
| rustdoc | yes, enough for docs and doctests | Explain public API and validate examples | `cargo doc --open` |
| rustfmt | no full type check | Normalize syntax layout | `cargo fmt` |

---

## Clippy: A Linting Compiler Driver

Clippy is architecturally a compiler driver: effectively `rustc` plus hundreds
of additional lints. It runs after parsing and name resolution, with access to
HIR, type information, and sometimes MIR-level facts. It uses the same lint
infrastructure family as built-in compiler lints, so diagnostics, levels, and
allow/warn/deny attributes feel native. See
[15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md) for diagnostic machinery
and [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md) for MIR context.

```
Cargo unit graph
      |
      v
cargo clippy -> clippy driver -> rustc front/middle end
                                  |
                                  v
                        built-in lints + clippy lints
                                  |
                                  v
                              diagnostics
```

| Lint layer | Example | Decision point |
|------------|---------|----------------|
| rustc built-in | unreachable code, unused imports, future-incompat | Language/compiler correctness and compatibility |
| clippy default | suspicious patterns, needless clones, API misuse | Practical Rust idiom and bug risk |
| clippy pedantic/nursery | stricter style and evolving ideas | Team policy, not universal law |

Because clippy links unstable compiler crates, it is pinned to the same toolchain
version. Installing `clippy` for one toolchain does not make it a stable Rust
analysis library for every other compiler.

---

## rust-analyzer: IDE Engine, Not rustc-as-a-Service

rust-analyzer is the sharp contrast. Roslyn made the compiler and IDE service
the same public engine. Rust did not. `rustc` is a batch compiler with unstable
internal crates and a query system optimized for compiling complete crates, not
for answering thousands of partial-program editor questions per minute. RA
therefore reimplements the front end: lexer, parser, name resolution, macro
expansion approximations, type inference, and incremental queries.

```
editor
  |
  v
LSP protocol
  |
  v
rust-analyzer
  |  lazy salsa-style queries over an error-resilient syntax tree
  |  own name resolution and type inference for IDE latency
  |
  +--> cargo metadata / check command for build graph and diagnostics
  +--> rustc output when configured for check-on-save
```

RA is not pretending to be a second language. It is an IDE-optimized
implementation of enough Rust semantics to deliver navigation, completion,
inlay hints, refactors, and diagnostics under incomplete-code conditions. It
still shells out to Cargo for build graph information and to `rustc`/`cargo check`
for authoritative diagnostics when configured. The distinction matters: RA can be
excellent without making rustc a stable embeddable library.

| Need | Why rustc is awkward | Why RA reimplements |
|------|----------------------|---------------------|
| completion after half a token | batch compiler expects coherent inputs | error-resilient green/red syntax trees |
| goto definition in large workspaces | full compilation latency is too high | lazy per-query analysis |
| stable editor embedding | rustc internals are unstable | LSP is the stable integration surface |

---

## Miri: MIR-Level Execution and UB Detection

Miri interprets MIR, extending the same broad machinery family as const
evaluation into a program interpreter. It is not a production runtime and not a
substitute for normal testing. Its value is precise execution under a Rust memory
model experiment: catching undefined behavior in unsafe code, invalid alignment,
out-of-bounds access, use-after-free, data races in supported scenarios, and
aliasing violations under Stacked Borrows or Tree Borrows models.

```
Rust source -> rustc front end -> MIR [09]
                              |
                              v
                            Miri
                              |
        +---------------------+----------------------+
        | UB checks: provenance, aliasing, alignment |
        | interpreter result, not native execution   |
        +--------------------------------------------+
```

| Miri strength | Caveat |
|---------------|--------|
| Excellent for unsafe code tests | Usually nightly-only and toolchain-pinned |
| Finds UB native tests may miss | Not all OS/FFI behavior is modeled |
| Uses MIR before backend artifacts | Does not validate final machine-code optimizer behavior |
| Documents aliasing model pressure | Model can evolve with the language/tool |

Miri's depth is why it is valuable and why it cannot be treated as a stable
library boundary. It is coupled to rustc internals, MIR, and const-eval.

---

## Concrete Toolchain Trace

These commands show the intended surfaces. Use rustup to install components;
use Cargo subcommands where they exist.

```powershell
rustup component add rustfmt clippy rust-analyzer
cargo fmt
cargo clippy -- -W clippy::pedantic
cargo doc --open
cargo test --doc
rustup +nightly component add miri
cargo +nightly miri test
```

Two stability readings are useful. `cargo clippy` is a documented user command;
clippy's internal HIR visitors are not a contract. `cargo doc` is stable; the
shape of rustdoc's internal clean AST or unstable JSON output is not.

---

## Old World -> New World

| Old-world concept | Rust analogue | Important difference |
|-------------------|---------------|----------------------|
| DocFX / Sandcastle / XML docs | rustdoc | Doctests compile and run; docs participate in test discipline |
| `dotnet format` / EditorConfig formatting | rustfmt | Mostly syntactic; not a semantic analyzer |
| Roslyn analyzers / StyleCop / FxCop | clippy | Ships toolchain-pinned, implemented as a rustc driver rather than package-local analyzer assemblies |
| Roslyn IDE language service | rust-analyzer | Rust's IDE engine reimplements analysis instead of sharing a stable compiler API |
| ASan / Valgrind / managed runtime verifier | Miri | MIR interpreter for Rust UB, not native instrumentation or production runtime |
| MSBuild invoking analyzers | Cargo invoking tools | Cargo orchestrates; tools own their analysis and diagnostics |

The standout contrast is Roslyn. In .NET, compiler-as-platform is the design.
In Rust, `rustc` internals stay unstable, and the IDE service is architecturally
separate. That is not an accident; it is a tradeoff favoring compiler evolution
and editor latency over a stable public compiler API.

---

## Decision Cheat Sheet

| Question | Tool | Why |
|----------|------|-----|
| Need API docs or doctest validation? | rustdoc / `cargo doc`, `cargo test --doc` | Uses compiler front end and test harness |
| Need consistent formatting? | rustfmt / `cargo fmt` | Syntax-level formatting policy |
| Need idiom and bug-pattern lints? | clippy / `cargo clippy` | Deep compiler-driver lints with type information |
| Need IDE navigation/completion/refactors? | rust-analyzer | LSP server with lazy editor analysis |
| Need to test unsafe code for UB? | Miri / `cargo +nightly miri test` | MIR interpreter with UB checks |
| Need stable machine-readable compiler internals? | Usually: do not | rustc internals are private and version-sensitive |
| Need exact build graph input for tools? | `cargo metadata` | Cargo-owned stable-ish JSON command surface |

---

## Common Confusion Points

| Confusion | Correction |
|-----------|------------|
| "rustdoc just renders comments." | It uses the compiler front end and compiles/runs doctests. |
| "rustfmt is a type-aware refactoring engine." | It is mostly syntactic formatting, not semantic rewriting. |
| "clippy is an external grep-like linter." | It is a rustc driver with compiler IR and type information. |
| "rust-analyzer calls rustc as a library." | RA reimplements a lazy IDE front end and shells out for Cargo/rustc data where useful. |
| "Miri proves my program safe." | Miri executes tests under an interpreter and detects many UB classes; coverage and model limits remain. |
| "Tool APIs are stable because commands are stable." | Documented CLIs may be stable; private compiler crates are not. |
| "Components are owned by rustup." | rustup installs/selects components; it does not own their analysis semantics. |

---

## Primary Sources

| Source | Why it matters |
|--------|----------------|
| The rustdoc Book | Official documentation and doctest behavior |
| rustfmt repository and configuration docs | Formatting model and configuration surface |
| The Clippy Book and `rust-lang/rust-clippy` | Lint groups, commands, and implementation context |
| rust-analyzer manual and `dev/architecture.md` | RA's reimplemented front end and query architecture |
| `rust-lang/miri` and rustc-dev-guide const-eval/Miri chapters | MIR interpretation and UB model context |
| rustc-dev-guide on lints, drivers, HIR, MIR, and diagnostics | Why rustdoc/clippy/Miri are toolchain-sensitive |
| Siblings: [00](00-OVERVIEW.md), [02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md), [04](04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md), [05](05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md), [09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md), [15](15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md), [18](18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md), [20](20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md) | Adjacent implementation and supply-chain layers |
