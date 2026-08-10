# EXP-01: Blueprint Finding-Closure Matrix

Date: 2026-08-10
Question: BLUE-Q02
Result: the completed FERRIS corpus requires a federated Blueprint Plan, not a
Cargo replacement or one universal build graph.

| Finding area | Failure or constraint | Blueprint Plan requirement | Owner | Disposition |
|---|---|---|---|---|
| Cargo unit identity and multiplication | package count does not predict unit variants; each activity creates a different graph | retain command, activity, package, target, feature, profile, host/target, and unit identity | Blueprint observes; Cargo owns | specify now |
| Cross-command reuse | `check`, `build`, `clippy`, `test`, `doc`, and doctest share some semantic work but not equivalent artifacts | model activities and stages separately; predict observed overlap only | Blueprint; rustc upstream for deeper reuse | specify; upstream |
| Cross-workspace artifacts | shared writable targets can poison path packages; compiler outputs alone are incomplete | isolate writable state; require Cargo-owned immutable artifact boundaries | Blueprint and Cargo | specify; upstream |
| rustc incrementality | private query and generation formats have atomicity and recovery invariants | reference complete opaque generations only under exact identity; never compose private files | rustc | reject private manipulation |
| Procedural macros | native execution and hidden inputs can make reuse stale | retain execution cone, declared/observed inputs, output, unknowns, and downstream demand | Blueprint observes; rustc owns caching | specify; upstream |
| Build scripts | compile, run, replayed directives, generated output, native metadata, and side effects differ | represent separate stages and widen on hidden inputs | Blueprint observes; Cargo owns | specify now |
| Generics and codegen | collection, ownership, duplication, emission, and final retention differ | retain family, owner, substitutions, CGU/backend, and link-retention evidence | rustc and linker | prototype-gated |
| Linking | fresh crates may still require whole-program linking; debug and release finalization differ | separate link plan, input identity, state, fallback, debug package, and final output | Blueprint observes; linker/rustc own | specify; upstream |
| Validation | changed-package tests miss activity and capability obligations | selected/full coverage ledger, mandatory gates, uncertainty, and full fallback | Blueprint and repository | specify now |
| Cache topology | archive hit, Cargo freshness, integrity, trust, and benefit are separate | eligibility before transport; economics after verification; rebuild fallback | Blueprint and cache provider | specify; restoration deferred |
| Build-state refs | human refs navigate roots but cannot establish compatibility | typed refs, compare-and-set, history, retention, revocation | Blueprint | specify now |
| System effects | jobs, memory, filesystem, VM, indexing, security, power, and concurrent sessions dominate outcomes | resource envelope, foreground priority, isolation, cancellation, and pressure observation | Blueprint and platform | specify now |
| Native boundary | requested, discovered, built, linked, loaded, and executed identities differ | typed provider/tool/ABI/generated-code/artifact chain with host/target split | Blueprint, Typebook/RUNE, upstream | specify now |
| Contracts | package version is not semantic interface identity; Rust ABI is unstable | separate Rust API, Typebook, C ABI, WIT, and wire boundaries | Typebook/RUNE and Blueprint | specify now |
| Compatibility profiles | support evidence expires and differs by exact closure and platform | retain owner, expiry, renewal, substitution, removal, and rollback | Blueprint and profile issuer | prototype-gated |
| Ecosystem governance | FERRIS must not become registry, distribution, or universal authority | route action to upstream owner and retain non-goals | Blueprint | specify now |

## Consolidated plan requirements

1. Per-command Cargo invocation plans.
2. Owner-specific impact closures.
3. Canonical identity taxonomy.
4. Execution-cone and uncertainty policy.
5. Validation coverage and mandatory fallback.
6. Link and native finalization boundaries.
7. Reuse eligibility, integrity, trust, and economics.
8. Resource envelope and concurrent-session coordination.
9. Adaptive observation barriers and versioned replanning.
10. Lifecycle, support, removal, rollback, and upstream ownership.

## Non-flattening controls

- Cargo unit graphs remain Cargo graphs.
- rustc query graphs remain compiler-private evidence.
- validation activities remain capability-bearing obligations.
- contracts remain independently versioned semantic boundaries.
- native requested, discovered, linked, and loaded identities remain distinct.
- action identity, content digest, freshness, root, and ref remain distinct.
- lockfile universe and active target closure remain distinct.
- selected validation and full repository confidence remain distinct.

## Prototype boundary

The first planning prototype may generate and explain a read-only plan from:

- one Git change;
- stable Cargo metadata;
- one prior Query Forest root;
- one repository validation policy; and
- one measured machine resource envelope.

It must compare prediction with ordinary Cargo observations and demonstrate
complete removal. It may not rewrite manifests, restore artifacts, alter host
settings, or suppress mandatory validation.
