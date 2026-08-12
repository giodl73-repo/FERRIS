# rust-architecture/ — Status

**22 files (STATUS.md + 21 guides) | Complete ✅ | Source-first, awaiting source-backfill validation**

This module is a peer-level architecture reference for the **Rust implementation
ecosystem** — the compiler (`rustc`), `cargo`, the standard library, `rustup`,
codegen backends, analysis, artifacts, platform layers, and the build/test/
release machinery. It deliberately separates the **language specification**
(Rust Reference, RFC/edition process) from the **reference implementation**
(rustc), **stable public contracts** from **compiler internals**, and
**rustc/Cargo ownership** from **ecosystem tools**. Where the module names an
internal (a query, a DefId, a MIR pass, `rmeta` layout, an on-disk cache), it is
flagged as version-sensitive and not a stable contract.

## Guides

| File | Topic | Status |
|------|-------|--------|
| `00-OVERVIEW.md` | End-to-end architecture of the Rust implementation ecosystem — the three ownership boundaries (language vs compiler, stable vs internal, rustc/Cargo vs ecosystem), a single `cargo build` trace through every authority, and reading paths | ✅ done |
| `01-PROJECT-GOVERNANCE-RFCS-AND-RELEASE-TRAIN.md` | Project teams and the Leadership Council, the RFC → feature-gate → stabilization pipeline, editions (opt-in, per-crate, one compiler), the three-channel six-week release train, and the stability/compatibility promise | ✅ done |
| `02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md` | rustup as toolchain multiplexer: proxy/shim dispatch, `+toolchain`, channel manifests, components, `rust-std` per target, target triples and tiers, and override precedence (`rust-toolchain.toml`) | ✅ done |
| `03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md` | One-crate invocation, the driver, `Session` vs `TyCtxt`, providers, demand-driven memoized queries, and the dependency graph that underpins incremental compilation | ✅ done |
| `04-LEXING-PARSING-AST-AND-DIAGNOSTIC-SPANS.md` | SourceMap and interned `Span`s, two-stage lexing, token trees and the proc-macro boundary, the hand-written recursive-descent/Pratt parser, the AST, and error recovery | ✅ done |
| `05-MACRO-EXPANSION-HYGIENE-AND-NAME-RESOLUTION.md` | The interleaved expansion/resolution fixed-point loop, declarative vs procedural macros, SyntaxContext-based hygiene, the resolver, namespaces, modules/imports/visibility, and the stable `proc_macro` API vs the internal bridge | ✅ done |
| `06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md` | AST → HIR lowering and desugaring, the DefId/HirId identity system, DefPathHash stability for incremental/cross-crate, bodies, THIR as the bridge to MIR, and crate metadata identity | ✅ done |
| `07-TYPE-INFERENCE-CHECKING-AND-REGION-CONSTRAINTS.md` | Per-body `typeck`, the inference context and unification, function-local (not global HM) inference, coercions, autoref/autoderef method resolution, region-constraint collection, and obligation registration | ✅ done |
| `08-TRAIT-SOLVING-COHERENCE-AND-NEXT-SOLVER.md` | Goals/obligations, canonicalization for caching, candidate selection, coherence and the orphan rule (language contract), the specialization caveat, and the in-progress next-generation solver caveat | ✅ done |
| `09-MIR-CONSTRUCTION-TRANSFORMS-AND-INTERPRETATION.md` | The MIR CFG (locals, places, rvalues, terminators), drop elaboration and unwind edges, the MIR optimization pipeline (internal, conservative), and CTFE via the MIR interpreter engine shared with Miri | ✅ done |
| `10-BORROW-CHECKING-NLL-AND-POLONIUS.md` | MIR-level `mir_borrowck`, NLL regions as CFG point-sets, region inference, loans/moves/two-phase borrows, the signature borrow diagnostics, and the experimental Polonius caveat | ✅ done |
| `11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md` | Monomorphization as Rust's generics strategy, mono-item (`Instance`) collection, the polymorphization caveat, codegen-unit partitioning for parallelism/incremental reuse, `dyn` fat pointers and vtable layout (internal), and symbol mangling | ✅ done |
| `12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md` | The codegen backend interface, the default LLVM path (MIR → LLVM IR → objects), Cranelift for fast debug builds, the experimental GCC backend (vs the separate gccrs front end), and LTO/PGO/BOLT context | ✅ done |
| `13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md` | Crate types (rlib/rmeta/dylib/staticlib/cdylib/bin/proc-macro), version-locked `rmeta` and the absence of a stable Rust binary ABI, the platform linker handoff and native objects, and DWARF/PDB debug info | ✅ done |
| `14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md` | Query-graph incrementalism vs Cargo freshness, the red-green algorithm, stable fingerprints/ICH, on-disk work products and the query cache, invalidation, and the limits of reuse | ✅ done |
| `15-DIAGNOSTICS-ERROR-CODES-AND-EXPLAINABILITY.md` | The diagnostic pipeline, structured suggestions and applicability (rustfix), error codes and `--explain`, lint levels and groups, JSON diagnostics as the tool-integration contract, and UI snapshot tests | ✅ done |
| `16-CORE-ALLOC-STD-PANIC-AND-PLATFORM-LAYERS.md` | The core ⊂ alloc ⊂ std facade and `no_std`, the lang-items/intrinsics compiler-library coupling caveat, the `#[global_allocator]` hook, unwind vs abort panic runtimes, the internal `std::sys` platform shim, and the no-runtime/no-GC model | ✅ done |
| `17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md` | Manifests and SemVer, the resolver and lockfile (coexisting incompatible versions), workspaces, additive feature unification, the unit graph and profiles, metadata pipelining, and Cargo's own fingerprint/freshness layer | ✅ done |
| `18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md` | The host/target split, `build.rs` directive protocol, the `links` key and `-sys` convention, native discovery (`cc`/`pkg-config`/`vcpkg`), the proc-macro process/server boundary, and cross-compilation | ✅ done |
| `19-RUSTDOC-RUSTFMT-CLIPPY-RUST-ANALYZER-AND-MIRI.md` | Ownership and architecture of the major tools organized by coupling depth to rustc internals — rustdoc/clippy/Miri build on compiler-private crates (toolchain-pinned); rustfmt is parser-level; rust-analyzer reimplements an IDE front end; Miri interprets MIR | ✅ done |
| `20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md` | The stage0→stage1→stage2 bootstrap, x.py/bootstrap and `config.toml`, the compiler test suites and bors merge queue, rustc-perf gating, Crater ecosystem regression runs, and release artifact distribution consumed by rustup | ✅ done |

## Coverage Notes

This module is the **implementation-ecosystem** companion to the language itself.
It is written for a reader who knows compiler theory, type theory, dataflow, SSA,
and CI/CD cold, so DFAs, unification, dominators, and staged bootstrapping are
starting points, not lessons; the guides concentrate on **how rustc, Cargo,
rustup, the standard library, and the surrounding tools are actually built and
how their responsibilities divide**. The compiler pipeline (guides 03–16) is the
deepest cluster; governance/toolchain (01–02), Cargo (17–18), the tooling layer
(19), and the supply chain (20) frame it.

Three separations are maintained throughout and are the module's editorial spine:
(1) **language vs compiler** — the Rust Reference and the RFC/edition process
define Rust; `rustc` is one conforming implementation; (2) **stable vs internal**
— HIR/THIR/MIR shapes, DefId/HirId, query keys, `rmeta` and incremental cache
formats, MIR pass ordering, vtable layout, the proc-macro bridge, and every `-Z`
flag are called out as version-sensitive implementation details, never as
guarantees; (3) **rustc/Cargo vs ecosystem** — rustdoc, rustfmt, clippy,
rust-analyzer, and Miri are distinguished by how (and how deeply) they depend on
compiler internals, with rust-analyzer's deliberate reimplementation highlighted.
Evolving subsystems (the next-generation trait solver, Polonius, the Cranelift
and GCC backends, polymorphization) are presented with explicit
status/version-sensitivity caveats rather than as finished defaults. Bridges run
to MSBuild/NuGet/Roslyn/RyuJIT/CLR/PDB and to universal CS concepts (memoized
build graphs, red-green trees, self-hosting bootstrap, monomorphization vs
type-erasure) so the material lands for any senior engineer.

Cross-references: this module points at `../rust-language/` (where it exists) for
the language definition itself (syntax, semantics, the type-system rules `rustc`
conforms to) and is careful not to re-derive it here.

## Official Source Families

Every guide closes with a **Primary Sources** section drawn from the official
families below. Because much of the material describes internals, the
authoritative source for those is the rustc-dev-guide plus the (explicitly
unstable) in-tree source; stable-contract material is sourced from the Reference,
the std docs, and the user-facing books.

- **rustc-dev-guide** (`rustc-dev-guide.rust-lang.org`) — internal architecture:
  driver/queries, HIR/THIR/MIR, trait solving, borrowck, monomorphization,
  codegen, metadata, incremental, diagnostics, bootstrapping, testing.
- **The Rust Reference** (`doc.rust-lang.org/reference`) — language semantics and
  the stable contracts rustc conforms to (coercions, coherence, destructors,
  linkage/ABI, `no_std`, constant evaluation).
- **The Cargo Book + Cargo contributor/architecture docs** (`doc.rust-lang.org/cargo`,
  `rust-lang/cargo`) — manifests, resolution, features, workspaces, build scripts,
  profiles, build cache.
- **The rustup Book** (`rust-lang.github.io/rustup`) and the **Platform Support /
  target-tier** docs — toolchains, components, targets.
- **Standard library docs** (`doc.rust-lang.org/std`, `/core`, `/alloc`) and the
  **Unstable/Embedded/Rustonomicon** books for `no_std`, allocators, panic, and
  lang-item caveats.
- **rust-lang repositories** — `rust-lang/rust`, `rust-lang/rfcs`,
  `rust-lang/cargo`, `rust-lang/rustup`, `rust-lang/rust-analyzer`,
  `rust-lang/rust-clippy`, `rust-lang/miri`, `rust-lang/rustc_codegen_cranelift`,
  `rust-lang/rustc_codegen_gcc`, `rust-lang/polonius`.
- **rustc-perf** (`rust-lang/rustc-perf`, `perf.rust-lang.org`) and **Crater**
  (`rust-lang/crater`) — performance gating and ecosystem regression testing;
  the **error-codes index** and the **Rust Forge** release-process docs.

## Certification Status

- **Structural/style:** complete. All 21 guides follow the MAXIM style contract
  (Big Picture ASCII diagram first, layered drill-down, architecture tables, a
  concrete command/trace, an old-world → new-world bridge, a Decision Cheat
  Sheet, Common Confusion Points, and Primary Sources). Cross-links between
  guides resolve. No `@editor` tags outstanding.
- **Source custody:** frontmatter is `status: source-custody`,
  `source_custody: none` — this module is **source-first and has not yet been
  run through source-backfill**. It is **ready for source-backfill validation**
  (MDLOOM source markdown/sidecars, CROP views, MDPORT packs, FLETCH registry)
  as the next step.
- **Factual certification:** **NOT** claimed as Certified Gold. Internals are
  described as of current rustc/Cargo behavior and are version-sensitive by
  design; treat implementation details as subject to change and verify against
  the Primary Sources during a numbers/names fact-check wave before any Gold
  stamping.
