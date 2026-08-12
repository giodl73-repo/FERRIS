---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:rustc-driver-query-system
kind: guide
module: rust-architecture
section: rust-architecture
title: The rustc Driver, Session, and Demand-Driven Query System
status: source-custody
source_custody: partial
current_path: rust-architecture/03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md
canonical_path: rust-architecture/03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md
backsource_ids: [mdloom-backfill:rust-architecture:03-rustc-driver-query-system]
concepts: [rustc driver, compiler session, tyctxt, query system, providers, dependency graph]
root_concepts: [rustc driver]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# The rustc Driver, Session, and Demand-Driven Query System

## The Big Picture

One `rustc` invocation compiles **one crate**: one library, binary, test crate,
proc-macro crate, or build-script crate. Cargo plans a package graph, then
spawns one compiler process per crate-shaped unit; `rustc` owns the compiler run
inside that process. The important architectural correction is that rustc is not
just a fixed sequence of passes. The driver forces high-level work, and most of
that work is serviced by a demand-driven, memoized query system.

Everything named here below the CLI contract is rustc implementation detail.
`rustc_driver`, `rustc_interface`, `Session`, `TyCtxt`, query keys, provider
layout, and dependency-graph internals move between releases and are exposed
only through unstable `rustc_private` or `-Z` surfaces.

```
+===========================================================================+
|                         ONE RUSTC INVOCATION                              |
|                                                                           |
|  CLI: rustc [flags] crate_root.rs                                         |
|       --crate-type  --edition  --emit  -C  -L  --extern                   |
|       -Z ... = nightly-only, unstable, internal                           |
+---------------------------------------------------------------------------+
             |
             v
+---------------------------------------------------------------------------+
|  rustc_driver / rustc_interface                                           |
|  parse args -> create Session -> register lints -> run callbacks          |
|  Authority: rustc internals; not a stable embedding API                   |
+---------------------------------------------------------------------------+
             |
             v
+---------------------------------------------------------------------------+
|  Session                                                                  |
|  options · source map · diagnostics emitter · target info · lint config   |
+---------------------------------------------------------------------------+
             |
             v
+---------------------------------------------------------------------------+
|  GlobalCtxt + TyCtxt<'tcx>                                                |
|  arenas · interners · query tables · dependency graph · type database     |
+---------------------------------------------------------------------------+
             |
             v
+---------------------------------------------------------------------------+
|  DRIVER FORCES TOP-LEVEL WORK                                             |
|  analysis queries -> MIR/borrowck/typeck as demanded -> codegen queries   |
|                                                                           |
|  Some front-end work is still imperative: lex/parse/expand/resolve        |
|  feed HIR and the query world. See [04] and [05].                         |
+---------------------------------------------------------------------------+
             |
             v
+---------------------------------------------------------------------------+
|  OUTPUTS                                                                  |
|  metadata (.rmeta) · rlib/staticlib/cdylib · object files · final link    |
+===========================================================================+
```

Read the diagram as ownership, not as a promise of stable phase order. The
language contract lives in the Rust Reference and governance process [01]. The
compiler architecture described here is the reference compiler's current
implementation strategy.

---

## Invocation Boundary: Cargo Schedules, rustc Compiles

Cargo is the build orchestrator; rustc is the compiler. A Cargo build can
execute many rustc processes because dependencies, build scripts, proc-macros,
tests, examples, and binaries are separate crate units. See [18] for the host
versus target complications around build scripts, proc macros, and native tools.

| Boundary | What happens | Stable surface |
|----------|--------------|----------------|
| `cargo build` | Reads manifests, resolves features, schedules units | Cargo CLI and manifest rules |
| `rustc ... crate_root.rs` | Compiles one crate per process | Documented rustc CLI flags |
| `--crate-type` | Chooses lib/bin/rlib/dylib/staticlib/cdylib/proc-macro shape | Stable flag |
| `--edition` | Selects edition-gated surface syntax and name rules | Stable flag |
| `--emit` | Selects outputs such as metadata, link, obj, asm, llvm-ir | Stable flag, output details vary |
| `-C ...` | Codegen options: opt level, debuginfo, target CPU, LTO | Stable flag family |
| `-L`, `--extern` | Supplies search paths and upstream crate metadata | Stable flag family |
| `-Z ...` | Compiler debugging and internal controls | Nightly-only, unstable |

Concrete shape:

```text
rustc --crate-type=lib --emit=metadata,link -C opt-level=2 --edition 2021 src/lib.rs
cargo build -v        # see the exact rustc command lines Cargo spawned
```

`cargo build -v` is often the fastest way to reacquire the real command-line
model. Cargo did not "call into" the compiler as a library; it spawned rustc with
crate metadata, extern paths, cfgs, feature cfgs, and output locations.

---

## Driver and Session

The driver is the `csc.exe main` equivalent: parse arguments, establish services,
then run compilation through callbacks. The crates normally named in this area,
`rustc_driver` and `rustc_interface`, are `rustc_private`. Tools such as rustdoc,
clippy, and Miri build in lockstep with a specific compiler or nightly because
that API is deliberately not stable. See [19] for the ecosystem-tool boundary.

```
+---------------------------------------------------------------+
| rustc_driver                                                  |
|                                                               |
|  argv                                                         |
|   |                                                           |
|   v                                                           |
|  Options + callbacks                                          |
|   |                                                           |
|   v                                                           |
|  Session                                                      |
|   |-- SourceMap: files, byte positions, macro provenance      |
|   |-- Diagnostics: emitter, codes, suggestions                |
|   |-- Target: pointer width, ABI, cfg, relocation model       |
|   |-- Lints/options: edition, crate type, codegen knobs       |
|   v                                                           |
|  enter compiler context -> force analysis/codegen             |
+---------------------------------------------------------------+
```

`Session` is not "the compiler database." It is per-run configuration plus shared
services. The queryable semantic universe appears once rustc creates the global
compiler context and hands out `TyCtxt<'tcx>`.

---

## GlobalCtxt, TyCtxt<'tcx>, and Interned Identity

`TyCtxt<'tcx>` is the typing context threaded through almost every serious
compiler operation as `tcx`. It is arena-allocated inside the global context and
acts as the central interned, queryable database for the crate and its
upstreams. The lifetime `'tcx` is the compiler's arena lifetime: values interned
there are cheap to reference and compare for the duration of the compilation.

| Object | Owns | Mental model | Stability |
|--------|------|--------------|-----------|
| `Session` | Options, source map, diagnostics, target services | Compilation options plus service locator | Internal |
| `GlobalCtxt` | Arenas, query caches, interners, dep graph | The backing store for compiler-wide state | Internal |
| `TyCtxt<'tcx>` | Typed handle into interned/queryable compiler data | Roslyn `Compilation` plus semantic database, but private | Internal |
| Interners | Types, constants, generic args/substs, symbols | Canonical identity and structural sharing | Internal |

The shape of `TyCtxt`, the exact interning strategy, and the names of its query
methods are not public contracts. When a guide later names HIR owners [06], type
queries [07], MIR queries [09], or codegen-unit queries [11], read those names as
current rustc architecture, not as an embedding API.

---

## Queries: Demand-Driven Compilation

At the top level, rustc behaves less like "run pass A, then pass B" and more like
a memoized function graph. A query has the conceptual shape
`fn(TyCtxt<'tcx>, Key) -> Value`. Providers implement the query; the query engine
memoizes results, tracks dependencies, detects cycles, and reports recursive
requirements as compiler errors where appropriate.

```
+------------------+        requests         +-------------------+
| codegen driver   | ----------------------> | optimized_mir(f)  |
+------------------+                         +-------------------+
                                                    |
                                                    v
                                             +-------------------+
                                             | mir_built(f)      |
                                             +-------------------+
                                                    |
                           +--> typeck(owner)
                           |        |
                           |        v
                           |    trait/region work
                           |
                           +--> type_of(def_id)
```

Illustrative internal query names include `type_of(def_id)`, `typeck(owner)`,
`mir_built(def_id)`, `optimized_mir(def_id)`, and queries that form or retrieve
codegen units. They are useful vocabulary for reading rustc-dev-guide and traces,
but the keys, values, and names are implementation details.

This demand model is why a change in one item need not force every downstream
computation to be redone. It is also why "the type checker pass" is an
oversimplification: type-related facts are pulled by later work as needed and
cached once computed.

---

## Dependency Graph and Incremental Compilation

Every executed query records the queries and inputs it read. Those edges form the
DepGraph, the substrate used by incremental compilation [14]. On a later build,
rustc can compare fingerprints and mark nodes green when their inputs are still
valid, red when they must be recomputed.

```
+-------------------+     reads      +-------------------+
| optimized_mir(F)  | -------------> | mir_built(F)      |
+-------------------+                +-------------------+
          |                                    |
          | reads                              | reads
          v                                    v
+-------------------+                +-------------------+
| typeck(F owner)   | -------------> | HIR owner body    |
+-------------------+                +-------------------+
          |
          v
+-----------------------------------------------------------+
| DepGraph node fingerprints: reuse if green, recompute red |
| Internal representation; incremental behavior is rustc    |
| implementation, not a language contract.                  |
+-----------------------------------------------------------+
```

Classic compiler diagrams are push pipelines: parse pushes AST to bind, bind
pushes to typecheck, typecheck pushes to IR. rustc exposes much of its work as a
pull graph, but that does not make an ordinary build a partial-crate compiler.
The driver forces crate-level analysis and codegen roots; parsing, expansion,
resolution, and HIR formation establish crate-wide prerequisites. Per-owner and
per-definition queries make later semantic work, dependency recording, and
incremental validation granular within those forced roots.

---

## Concrete Trace and Introspection

Use stable commands to observe the crate boundary and nightly commands to inspect
internals. The nightly dumps are debugging aids. Their formats, names, and even
availability are not guaranteed.

```text
# Stable: compile one crate root directly.
rustc --crate-type=lib --emit=metadata,link -C opt-level=2 --edition 2021 src/lib.rs

# Stable: let Cargo show the rustc process per crate unit.
cargo build -v

# Nightly/unstable: internal views. Requires a nightly toolchain.
rustc +nightly -Z unpretty=hir-tree src/lib.rs
rustc +nightly -Z time-passes src/lib.rs
rustc +nightly -Z self-profile src/lib.rs
rustc +nightly -Z help
```

The `-Z unpretty=hir-tree` path is commonly useful when crossing from parsing
and macro expansion [04][05] into HIR identity [06]. `-Z self-profile` is useful
for performance work, but it is still an internal compiler profiling interface,
not a stable telemetry contract.

---

## Old world -> New world

| Old world / universal model | Rust implementation analogue | Where the analogy breaks |
|-----------------------------|------------------------------|---------------------------|
| `csc.exe` main | `rustc_driver` | Rustc's driver crates are private, not a stable compiler SDK |
| Roslyn `ParseOptions` / `CompilationOptions` | `Session` options plus target configuration | `Session` also owns diagnostics/source services |
| Roslyn `Compilation` / `SemanticModel` | `TyCtxt<'tcx>` as central queryable DB | `TyCtxt` is unstable `rustc_private`, not a public API |
| Roslyn red-green reuse | DepGraph red-green incremental marking | Rustc red/green applies to query results, not public syntax nodes |
| Excel recalculation / build graph | Demand-driven memoized queries | Query keys and providers are compiler internals |
| MSBuild project graph | Cargo unit graph [18] | Cargo schedules crates; rustc compiles one crate at a time |

The best mental model is a demand-driven build graph inside a single compiler
process. The driver chooses the root goals; the query engine performs the
minimum memoized recalculation to satisfy them.

---

## Decision Cheat Sheet

| Question | Use / inspect | When | Who owns it |
|----------|---------------|------|-------------|
| What command compiled this crate? | `cargo build -v` | Reconstruct flags, externs, cfgs | Cargo invokes; rustc consumes |
| Compile one crate manually | `rustc [flags] crate_root.rs` | Reduce Cargo effects | rustc CLI |
| Emit metadata or link output | `--emit=metadata,link` | Artifact experiments [13] | rustc CLI |
| Tune codegen | `-C opt-level=...`, `-C debuginfo=...` | Backend/perf settings [11][13] | rustc / backend |
| Inspect HIR/query-adjacent internals | `rustc +nightly -Z unpretty=...` | Debug compiler behavior | rustc internals, unstable |
| Embed rustc as a tool | Avoid unless pinned to nightly | Clippy/rustdoc/Miri-style tools [19] | rustc_private, unstable |
| Reason about reuse | DepGraph / incremental docs [14] | Build performance diagnosis | rustc internals |
| Decide language meaning | Rust Reference / RFCs [01] | Stability and compatibility questions | lang team, not query names |

---

## Common Confusion Points

- **One Cargo build is not one rustc invocation.** Cargo usually spawns many
  rustc processes: dependencies, proc macros, build scripts, tests, examples,
  and final binaries are separate crate units.
- **`Session` and `TyCtxt` are not the same thing.** `Session` is options and
  services; `TyCtxt<'tcx>` is the interned/queryable compiler database.
- **The query system is not a stable plugin API.** Query names such as `type_of`
  and `optimized_mir` are internal rustc vocabulary.
- **Rustc is not purely demand-driven from byte zero.** Lexing, parsing, macro
  expansion, and resolution have imperative machinery before feeding HIR into
  the query world. See [04] and [05].
- **`-Z` means unstable.** `-Z unpretty`, `-Z time-passes`, and `-Z self-profile`
  are compiler debugging aids, not guaranteed output formats.
- **Roslyn is a helpful analogy, not a contract.** Rust has no stable Roslyn-like
  compiler API. Ecosystem tools either pin to rustc internals or reimplement
  front-end analysis.

---

## Primary Sources

- **rustc-dev-guide: Overview** — the official map of the compiler's major
  crates, phases, and internal boundaries.
- **rustc-dev-guide: Queries: demand-driven compilation** — query providers,
  memoization, cycles, and dependency tracking.
- **rustc-dev-guide: The `ty` module: representing types** — `TyCtxt`, interned
  type representation, arenas, and compiler type identity.
- **rustc-dev-guide: Rustc driver and interface** — `rustc_driver`,
  `rustc_interface`, callbacks, and embedding caveats.
- **The rustc book: command-line arguments** — stable rustc CLI shape, including
  `--crate-type`, `--edition`, `--emit`, `-C`, `-L`, and `--extern`.
- **The Unstable Book / `rustc -Z help`** — nightly-only flags such as
  `-Z unpretty`, `-Z time-passes`, and `-Z self-profile`; debugging aids only.
- **`rust-lang/rust` source tree** — authoritative for the implementation at a
  given revision, but not a stable public contract.

*Cross-links:* start with the landscape [00](00-OVERVIEW.md), then read the
front-end [04](04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md) and macro/name
boundary [05](05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md). HIR identity
begins in [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md), type work in
[07](07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md), MIR in
[09](09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md), incremental reuse in
[14](14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md), and tools in
[19](19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md). For the language
surface itself, see `../rust-language/` where it exists.