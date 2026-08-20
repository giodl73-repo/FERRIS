# rust-language/ — Status

Peer-level, user-facing Rust language reference. This module *expands* the compact
syntax card at `../languages/09-RUST.md` — it does not duplicate its role. The card
stays the fast lookup; this module is the deep, layered explanation for a senior
engineer who wants current Rust specifics with universal + C++/.NET bridges.

## Files

| File | Topic | Status | Coverage notes | Source families |
|------|-------|--------|----------------|-----------------|
| 00-OVERVIEW.md | Whole-language landscape, three core ideas, reading paths, version posture | done | Orientation + how to navigate the 21 guides; stable-first posture stated | Book, Reference |
| 01-TOOLCHAIN-AND-WORKFLOW.md | rustup, cargo, rustc, rustfmt, clippy, rust-analyzer, rustdoc, inner loop | done | Full toolchain + project loop; channels/components | Rustup, Cargo, rustfmt, clippy, rustdoc |
| 02-BINDINGS-TYPES-AND-INFERENCE.md | Bindings, mutability, shadowing, scalars, tuples, arrays, inference, coercions, casts, unit/never | done | `!`-in-arbitrary-position labeled nightly | Book, Reference, std |
| 03-OWNERSHIP-MOVES-COPY-AND-DROP.md | Affine ownership, moves, Copy, Clone, Drop/RAII, partial moves | done | Foundational; destructor order + partial-move rules | Book, Reference, Nomicon |
| 04-BORROWING-REFERENCES-AND-LIFETIMES.md | Shared/exclusive borrows, NLL, elision, explicit lifetimes, reborrowing, variance | done | User-level variance; NLL mental model | Book, Reference, Nomicon |
| 05-STRUCTS-ENUMS-PATTERNS-AND-MATCH.md | ADTs, destructuring, guards, exhaustiveness, let-else, if/while let | done | let-else labeled stable 1.65 | Book, Reference |
| 06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md | Bounds, where, associated types/consts, coherence/orphan rule, blanket impls, supertraits | done | Coherence/orphan rule explained | Book, Reference |
| 07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md | Monomorphization, dyn, object safety/dyn compatibility, impl Trait, RPIT, RPITIT | done | RPITIT/AFIT stable 1.75; TAIT labeled nightly | Book, Reference, release notes |
| 08-CLOSURES-FUNCTION-TRAITS-AND-CALLABLES.md | Capture modes, Fn/FnMut/FnOnce, move, fn pointers, callbacks | done | Disjoint capture = 2021 edition | Book, Reference, std |
| 09-COLLECTIONS-ITERATORS-AND-RANGES.md | Vec, maps, sets, deques, slices, iterator adapters, three iter modes, marker traits | done | ExactSize/Fused/DoubleEnded covered | std, Book |
| 10-STRINGS-TEXT-AND-UNICODE.md | String/&str, UTF-8 boundaries, chars/graphemes caveat, formatting, OsString/Path, bytes | done | Grapheme caveat (needs external crate) called out | std, Reference |
| 11-ERRORS-RESULT-OPTION-AND-PANIC.md | Recoverable vs unrecoverable, `?`, From conversion, custom errors, anyhow/thiserror boundary | done | try_trait_v2 residual labeled nightly | Book, std |
| 12-MODULES-CRATES-PACKAGES-AND-VISIBILITY.md | mod/use/pub, crate roots, package/workspace, preludes, re-exports, API layout | done | Package vs crate vs workspace distinguished | Book, Reference, Cargo |
| 13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md | macro_rules, hygiene, fragments, attributes, derives, proc macros, build-time trust boundary | done | Proc-macro build-time trust boundary flagged | Reference, Book, TLBORM |
| 14-ASYNC-FUTURES-AND-PINNING.md | async lowering, Future poll, executors, wakers, cancellation, Pin/Unpin, Send futures | done | No runtime in std stated; Pin explained | std, async-book, Reference |
| 15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md | Thread model, channels, Mutex/RwLock, atomics/orderings, scoped threads, rayon | done | thread::scope stable 1.63; orderings summarized | std, Nomicon |
| 16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md | Box/Rc/Arc/Weak, Cell/RefCell/UnsafeCell, Cow, self-reference | done | Self-reference strategies incl. Pin pointer | std, Nomicon |
| 17-UNSAFE-RUST-FFI-AND-ABI.md | Five unsafe powers, sound abstraction, repr, extern, ownership across FFI, panic/unwind, C++ bridges | done | Unwind-across-FFI + repr rules covered; cxx bridge | Nomicon, Reference, std, cxx |
| 18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md | const eval, static, lazy init, cfg, features, targets, editions, MSRV | done | OnceLock 1.70 / LazyLock 1.80 / inline const 1.79; 2024 edition = 1.85 | Reference, Cargo, edition-guide |
| 19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md | Unit/integration/doc tests, examples, compile-fail, property testing, fuzzing, criterion, Miri | done | `#[bench]`, Miri, sanitizers labeled nightly | Book, rustdoc, rust-fuzz, criterion, Miri |
| 20-API-DESIGN-SEMVER-AND-ADVANCED-TYPE-PATTERNS.md | API evolution & semver, sealed traits, non_exhaustive, typestate, newtype, builders, HRTB, GAT, const generics, phantom types | done | GATs stable 1.65; generic_const_exprs labeled nightly | API guidelines, Cargo semver, Reference, Nomicon |

All 21 guides: **done**.

## Completed

2026-08-11 — All 21 content guides (00–20) written to the full MAXIM template:
Big-Picture ASCII diagram first, layered downward, comparison tables, at least one
practical Rust example, an old-world/new-world bridge (universal CS first, then
C++/.NET where useful), a Decision Cheat Sheet, Common Confusion Points, a
Primary Sources block of official URLs only, and cross-links to sibling guides.

Stable Rust is explained first throughout. Every nightly/unstable or
version-sensitive item is labeled inline and *not* implied to be stable — e.g.
TAIT (`type_alias_impl_trait`), `try_trait_v2` residual, `generic_const_exprs`,
the built-in `#[bench]` harness, Miri and the sanitizers (nightly), and the
reserved 2024 `gen` keyword. Version facts are pinned where they matter: 2024
edition stable in Rust 1.85; RPITIT/AFIT in 1.75; let-else and GATs in 1.65;
`thread::scope` in 1.63; `OnceLock` in 1.70; `LazyLock` in 1.80; inline `const {}`
in 1.79; `min_const_generics` in 1.51.

No `@editor` tags were added — no unresolved defects are known. Content is
source-first and internally consistent, but has **not** yet been run through the
source-backfill validation pipeline (no `.proof`/`.mdcrop`/`.mdport`/`.fletch`
artifacts were generated) and is **not** factually Gold-certified. The module is
**ready for source-backfill validation**.

## Coverage Notes

This module is the *deep* Rust reference; the compact card at
`../languages/09-RUST.md` remains the fast syntax lookup and is cross-referenced,
not duplicated. The ordering is a learning pipeline: toolchain and the type/binding
substrate (`01`,`02`) come first; the ownership trilogy — ownership/moves (`03`),
borrowing/lifetimes (`04`) — is the conceptual core every later guide reads in;
data modeling (`05`) and the trait/generics system (`06`,`07`,`08`) build the
abstraction layer; the standard library in anger (`09`,`10`,`11`) shows the model
applied; program structure (`12`,`13`) covers modules and metaprogramming;
concurrency (`14`,`15`) and the escape hatches (`16`,`17`) handle the hard cases;
and the build/release surface (`18`,`19`,`20`) closes with const/edition/target
mechanics, the test-and-measure toolkit, and API design.

Treatment is peer-level for a senior engineer with deep CS/compiler knowledge and
.NET/C++ familiarity: no dumbing down, universal bridges first with C++/.NET
bridges layered on where they clarify (RAII, move semantics, `std::variant`,
`IDisposable`, `Task`, `dynamic_cast`, NuGet SemVer, sealed classes). Every guide
uses pure-ASCII diagrams per the module contract.

Primary Sources across the module draw only from official families: The Rust
Programming Language book, the Rust Reference, the standard library docs, the
Rustonomicon, the rustc / Cargo / rustup / rustfmt / clippy / rustdoc books, the
Edition Guide, the Unstable Book, the async book, the API Guidelines, the
release-notes blog, the Rust Fuzz book, `cxx.rs`, TLBORM, the Miri repo, and the
Criterion book. No third-party tutorial or blog content is load-bearing.

Key cross-reference: `../languages/09-RUST.md` (the compact card this module
expands). This module does not modify the card, the `languages` section pages,
`TRACKER.md`, mkdocs config, or any generated directory.
