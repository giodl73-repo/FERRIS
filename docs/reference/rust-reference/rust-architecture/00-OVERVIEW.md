---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:overview
kind: guide
module: rust-architecture
section: rust-architecture
title: The Rust Implementation Ecosystem, End to End - Landscape
status: source-custody
source_custody: partial
current_path: rust-architecture/00-OVERVIEW.md
canonical_path: rust-architecture/00-OVERVIEW.md
backsource_ids: [mdloom-backfill:rust-architecture:00-overview]
concepts: [rust implementation, rustc, cargo, rustup, toolchain, standard library, backend, architecture]
root_concepts: [rust-architecture]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# The Rust Implementation Ecosystem, End to End — Landscape

## The Big Picture

Rust is not one artifact. It is a **language with a distributed public
contract**: the Rust Reference is the primary stable-language reference,
accepted RFCs and edition guides record design and evolution, and compatibility
policy governs stable releases. Rust also has a **reference compiler** (`rustc`),
a **build and package orchestrator** (`cargo`), a **toolchain multiplexer**
(`rustup`), a **standard library** shipped as prebuilt crates where target
support provides them, one or more **code-generation backends** (LLVM by
default), and a constellation of **ecosystem tools** (rustdoc, rustfmt, clippy,
rust-analyzer, Miri). These are *distinct authorities*. Conflating them is the
single most common source of confusion, so this module keeps the ownership
boundaries explicit at every layer.

This guide is the map. Everything below drills into one box.

```
+===========================================================================+
|                    THE RUST IMPLEMENTATION ECOSYSTEM                      |
|                                                                           |
|  SPEC / GOVERNANCE          [01]                                          |
|  Rust Reference · RFCs · editions · release channels · compat promises    |
|  Authority: project teams + lang/libs. NOT rustc internals.               |
+---------------------------------------------------------------------------+
             | documents stable rules/evolution; rustc implements them
             v
+---------------------------------------------------------------------------+
|  TOOLCHAIN MANAGEMENT       [02]  rustup                                  |
|  proxies (rustc/cargo shims) · channels · components · target libs        |
|  Authority: rustup project. Installs, does not compile.                   |
+---------------------------------------------------------------------------+
             | selects a toolchain; hands control to cargo/rustc
             v
+---------------------------------------------------------------------------+
|  BUILD ORCHESTRATION        [17][18]  cargo                               |
|  manifests · resolver · lockfile · units · build scripts · features       |
|  Authority: Cargo. Plans and schedules; invokes rustc per crate.          |
+---------------------------------------------------------------------------+
             | one rustc invocation per crate (compilation unit)
             v
+===========================================================================+
|  THE COMPILER: rustc          Authority: rustc (implementation detail)    |
|                                                                           |
|  driver/session/query [03]                                                |
|     |                                                                     |
|     v                                                                     |
|  lex/parse/AST [04] -> macro expand + resolve [05] -> HIR [06]            |
|     -> type/infer [07] -> trait solve [08] -> MIR build [09]              |
|     -> borrowck [10] -> MIR optimize [09] -> mono/CGUs [11]               |
|     -> backend [12]                                                       |
|     -> artifacts/metadata/link [13]                                       |
|                                                                           |
|  cross-cutting: incremental [14] · diagnostics [15]                       |
+---------------------------------------------------------------------------+
             | links against
             v
+---------------------------------------------------------------------------+
|  STANDARD LIBRARY           [16]  core / alloc / std                      |
|  Authority: libs teams. Stable API surface; internals version-sensitive.  |
+---------------------------------------------------------------------------+
             | analyzed / documented / linted / interpreted by
             v
+---------------------------------------------------------------------------+
|  ECOSYSTEM TOOLS            [19]  rustdoc · rustfmt · clippy · RA · Miri  |
|  SUPPLY CHAIN               [20]  bootstrap · CI · crater · rustc-perf    |
+===========================================================================+
```

Read top-down for *authority* (who decides), bottom-up for *dependency* (what
runs on what). The compiler box is the deepest and gets the most guides, but it
is only one authority among several.

---

## The Three Boundaries That Organize Everything

The whole module hangs on three separations. Internalize these first.

| Boundary | Left side (contract) | Right side (implementation) |
|----------|----------------------|-----------------------------|
| **Language vs compiler** | Reference, accepted RFCs, editions, compatibility policy — documented stable behavior and evolution | `rustc` phases, IRs, query keys — one way to realize it |
| **Stable vs internal** | `#[stable]` std API, CLI flags without `-Z`, `--emit` kinds | HIR/MIR shapes, `rmeta` layout, `-Z` flags, pass ordering |
| **rustc/Cargo vs ecosystem** | `rustc` (codegen), `cargo` (build graph) owned by the project | rustdoc/clippy/RA/Miri build *on* private compiler crates |

Anything on the right can and does change between releases — often within a
six-week cycle. When this module names an internal (a `TyCtxt`, a `DefId`, a MIR
pass, an `rmeta` section), it is describing *how a given rustc happens to work
today*, not a promise. The Decision Cheat Sheets flag which is which.

**Old world → new world.** If you built on MSBuild + Roslyn + NuGet + the CLR:
MSBuild ≈ Cargo (build graph, targets, incremental), Roslyn ≈ rustc (the
compiler, but AOT to native rather than to IL + JIT), NuGet ≈ Cargo's crates.io
integration, the CLR/BCL ≈ *there is none at runtime* — Rust's std is statically
linked and there is no managed runtime or GC. `dotnet` the multiplexer that
picks an SDK ≈ `rustup`. Roslyn's public compiler API surface has no stable Rust
analogue: rustc's internals are deliberately unstable, which is why rust-analyzer
reimplements analysis rather than calling rustc as a library.

---

## What Each Guide Covers

```
GOVERNANCE + TOOLCHAIN            THE COMPILER FRONT-END
  01 governance / RFCs / editions    04 lex / parse / AST / spans
  02 rustup / toolchains / targets   05 macros / hygiene / resolution
                                      06 HIR lowering / DefId identity
THE COMPILER CORE                  THE COMPILER MIDDLE + BACK
  03 driver / session / queries      07 inference / checking / regions
                                      08 trait solving / coherence
  CROSS-CUTTING                       09 MIR build / transforms / CTFE
  14 incremental / fingerprints       10 borrowck / NLL / Polonius
  15 diagnostics / error codes        11 mono / codegen units / vtables
                                      12 backends: LLVM / Cranelift / GCC
  STD + PLATFORM                       13 artifacts / metadata / link
  16 core / alloc / std / panic

CARGO + ECOSYSTEM                  SUPPLY CHAIN
  17 cargo resolution / features     20 bootstrap / CI / crater / perf / dist
  18 build scripts / proc macros
  19 rustdoc / rustfmt / clippy /
     rust-analyzer / Miri
```

---

## A Single Trace: `cargo build` on a Two-Crate Project

Follow one command through every authority. Each arrow crosses a boundary named
above.

```
$ cargo build
  |
  | (rustup proxy on PATH resolves the active toolchain)      [02]
  v
cargo reads Cargo.toml + Cargo.lock, builds the unit graph    [17]
  | resolves features, decides host vs target units           [18]
  v
for each crate, in dependency order:
  cargo spawns:  rustc --edition 2021 --crate-type lib ...    [03]
    |
    v
  rustc: parse -> expand macros -> resolve names              [04][05]
         lower to HIR -> typeck -> trait select               [06][07][08]
         build MIR -> borrow-check -> optimize for codegen    [09][10]
         monomorphize -> partition into codegen units         [11]
         hand each CGU to LLVM -> object code                 [12]
         emit rlib (objects + rmeta metadata)                 [13]
  |
  v
rustc drives the final platform link for the unit Cargo scheduled [13]
  | records fingerprints for next-time freshness              [14][17]
  v
target/debug/<bin>
```

To *see* it rather than trust it:

```
$ cargo build -v                                  # each rustc command line
$ cargo build --timings                           # unit schedule HTML [17]
$ rustc +nightly --crate-type=lib -Z unpretty=hir lib.rs  # HIR dump [06]
$ rustc --crate-type=lib --emit=metadata lib.rs   # rmeta only [13]
```

Every `-Z` in that list is a klaxon: you are looking at an **implementation
detail** that the project reserves the right to change.

---

## Reading Paths

Pick the path that matches your question. None require reading the module in
file order.

| You want to understand… | Read, in order |
|--------------------------|----------------|
| **How a build actually runs** | 02 → 17 → 18 → 03 → 13 → 14 |
| **How the compiler thinks** | 03 → 04 → 05 → 06 → 07 → 08 → 09 → 10 |
| **Why Rust catches memory bugs** | 07 → 09 → 10 (→ 08 for trait obligations) |
| **How code becomes machine code** | 09 → 11 → 12 → 13 |
| **The language process, not the code** | 01 → 02 → 20 |
| **The tools around the compiler** | 19 → 15 → 16 |
| **The std library layering** | 16 → 06 → 11 (lang items feed codegen) |
| **Shipping and quality-gating rustc itself** | 20 → 14 → 15 |

---

## Decision Cheat Sheet

| Question | Answer | Authority |
|----------|--------|-----------|
| "Is this behavior guaranteed?" | Check the owning stable surface: Reference/compatibility policy, stable library docs, or documented Cargo/rustc CLI | owning project team |
| "Which Rust version am I on?" | `rustc --version`; managed by rustup | rustup |
| "Who decides what compiles?" | rustc implements the documented language contract; project process resolves gaps | rustc / lang |
| "Who decides build order?" | Cargo's unit graph | Cargo |
| "Is HIR/MIR/`rmeta` stable?" | No — internal, version-sensitive | rustc |
| "Where does GC live?" | Nowhere; no runtime GC exists | language design |
| "Can I call rustc as a library?" | Only via unstable `rustc_private`; RA doesn't | rustc |
| "What's an edition?" | Opt-in surface change, same compiler | lang / 01 |

---

## Common Confusion Points

- **"Rust" the language vs `rustc` the compiler.** The Reference documents the
  primary stable language rules;
  accepted RFCs, edition guidance, and compatibility policy complete that
  public contract where the Reference has gaps. `rustc` is the reference
  *implementation*. Bugs in rustc are not language changes, and rustc internals
  (queries, IRs) are not the language. See [01].
- **Cargo does not compile.** Cargo plans and schedules units; `rustc` compiles
  them and drives link-producing invocations. "Cargo is slow to build X" almost
  always means rustc/LLVM/linker time in a unit Cargo scheduled. See [17].
- **rustup is not a compiler either.** It is a version multiplexer and installer;
  the `rustc`/`cargo` on your PATH are usually *proxies* that dispatch to the
  active toolchain. See [02].
- **std is not "the runtime."** It is a set of statically linked crates
  (`core`/`alloc`/`std`) with a stable API and version-sensitive internals; there
  is no CLR/JVM-style managed runtime. See [16].
- **LLVM is a backend, not "the Rust compiler."** rustc owns the front and middle
  end and defines a backend interface; LLVM (or Cranelift, or the GCC backend) is
  swappable. See [12].
- **Nightly `-Z`/internal dumps are not contracts.** `-Z unpretty=mir`, query
  names, pass ordering, and `rmeta` layout can change any release. See [03][14].

---

## Primary Sources

- **rustc-dev-guide** — `rustc-dev-guide.rust-lang.org` — the authoritative map
  of rustc's internal architecture (queries, HIR/MIR, trait solving, borrowck).
- **The Rust Reference** — `doc.rust-lang.org/reference` — the primary
  stable-language reference; consult its issue history, accepted RFCs, and
  compatibility process where the text has omissions or implementation gaps.
- **The Cargo Book + Cargo contributor docs** — `doc.rust-lang.org/cargo` and the
  `rust-lang/cargo` repository.
- **The rustup Book** — `rust-lang.github.io/rustup`.
- **Standard library docs** — `doc.rust-lang.org/std` (and `core`/`alloc`).
- **rust-lang repositories** — `rust-lang/rust`, `rust-lang/cargo`,
  `rust-lang/rustup`, `rust-lang/rfcs`; `rust-lang/rustc-perf` for performance.

*Cross-links:* every box above is a guide in this directory
([01](01-PROJECT-GOVERNANCE-RFCS-AND-RELEASE-TRAIN.md) …
[20](20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md)). For the *language*
itself (syntax, semantics, type-system rules) see `../rust-language/` where it
exists; this module is deliberately about the *implementation ecosystem*, not the
language definition.
