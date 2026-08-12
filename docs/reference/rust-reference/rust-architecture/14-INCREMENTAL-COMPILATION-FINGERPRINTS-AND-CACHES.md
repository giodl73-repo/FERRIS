---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:incremental-compilation-caches
kind: guide
module: rust-architecture
section: rust-architecture
title: Incremental Compilation - Fingerprints, the Dep Graph, and Caches
status: source-custody
source_custody: partial
current_path: rust-architecture/14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md
canonical_path: rust-architecture/14-INCREMENTAL-COMPILATION-FINGERPRINTS-AND-CACHES.md
backsource_ids: [mdloom-backfill:rust-architecture:14-incremental-compilation-caches]
concepts: [incremental compilation, dependency graph, red green, fingerprints, query cache, work products]
root_concepts: [incremental compilation]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Incremental Compilation - Fingerprints, the Dep Graph, and Caches

## The Big Picture

Rust has two incremental systems that people routinely conflate. Cargo decides which crates, targets, build scripts, and feature combinations are fresh enough to skip. rustc, once invoked for one crate, decides which internal compiler queries and codegen work products can be reused. The second system is built on the query engine [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md): queries record what they read, so rustc can persist a fine-grained dependency graph and validate it on the next run.

```
+===========================================================================+
|                         TWO LAYERS OF INCREMENTALISM                      |
|                                                                           |
|  Cargo layer                                                        [17]  |
|  Cargo.toml, Cargo.lock, features, profiles, build scripts, env, files    |
|        |                                                                  |
|        | decides: which units/crates need rustc at all                    |
|        v                                                                  |
|  rustc invocation for one crate                                           |
|        |                                                                  |
|        v                                                                  |
|  Query system                                                      [03]   |
|    parse? typeck? trait solve? MIR? mono? codegen unit?                   |
|        |                                                                  |
|        | records query reads                                              |
|        v                                                                  |
|  DepGraph + fingerprints + red/green marking                              |
|        |                                                                  |
|        v                                                                  |
|  On-disk incremental cache                                                |
|    serialized query results + work products                               |
|    target/debug/incremental/<crate-hash>/                                 |
|        |                                                                  |
|        v                                                                  |
|  unchanged queries reused; unchanged CGUs skip backend/LLVM [11][12]      |
+===========================================================================+
```

Cargo's fingerprinting and rustc's incremental compilation are complementary, not substitutes. A crate can be stale at the Cargo layer but still cheap inside rustc because much of its query graph is green.

---

## Cargo Freshness vs rustc Incremental Reuse

| Layer | Unit of decision | State it tracks | Typical question |
|-------|------------------|-----------------|------------------|
| Cargo | Package target / crate unit / feature-profile combination | Manifest, lockfile, source mtimes/hashes, build script outputs, env, rustc flags | "Do I invoke rustc for this unit?" |
| rustc | Query node and codegen work product inside one crate | DepGraph edges, fingerprints, serialized query results, CGU objects | "Inside this crate, what can I avoid recomputing?" |
| sccache-like tools | Whole compiler invocation | Command line plus input content hash | "Have I seen exactly this compile before?" |

```
No source change:
    Cargo says fresh -> rustc not invoked

Small source change in crate A:
    Cargo invokes rustc for A
    rustc says many nodes green -> reuse query results and some CGUs

Different machine with compiler cache hit:
    sccache may skip whole invocation -> third layer, outside rustc
```

This layering explains apparently contradictory observations: deleting `target/debug/incremental` can make a changed crate slower even when Cargo's target directory is otherwise warm; changing a dependency version can force Cargo to rebuild dependents even though rustc remains incremental within each one.

---

## The Red-Green Algorithm

```
Previous build saved:
    query node -> fingerprint(result)
    query node -> edges to input/query nodes

Next build:
    source/flags/dependency metadata get new input fingerprints
           |
           v
    visit dependent query node
           |
           +-- all inputs green and fingerprint matches? --> mark GREEN, reuse
           |
           +-- some input red/unknown? -------------------> re-run query
                                                          |
                                                          v
                                      result fingerprint same as before?
                                          | yes -> mark GREEN for dependents
                                          | no  -> mark RED
```

Red-green is more precise than timestamp invalidation because it separates "something upstream was touched" from "the value this query observes changed." If a private comment changes, parsing may rerun but type-checking of public signatures can remain effectively green. If a function body changes without changing an inlineable/public result seen by another query, downstream nodes may avoid recomputation.

| Color | Meaning | Consequence |
|-------|---------|-------------|
| Green | rustc can prove the value is unchanged | Reuse cached result or work product |
| Red | Some observed input changed in a value-changing way | Recompute and propagate invalidation |
| Unknown | Not yet validated this session | Walk dependencies until green/red is known |

The point is not magic; it is a content-addressed dependency graph at compiler-query granularity.

---

## Fingerprints, Stable Hashing, and Identity

| Mechanism | Role | Caveat |
|-----------|------|--------|
| Fingerprint | Usually a 128-bit hash of an input or query result | Internal representation, not a public hash API |
| ICH | Incremental compilation hash used to compare values across sessions | Debuggable with nightly verification flags only |
| Stable hasher | Makes hashes independent of incidental in-memory addresses | rustc-internal implementation |
| DefPathHash | Stable identity for definitions across sessions [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md) | Stable enough for rustc's cache, not a language contract |
| Input fingerprints | Source text, command-line relevant options, dependency metadata, target config | Changes can invalidate widely |

```
Bad cache key:
    pointer address of HIR node this run

Better cache key:
    DefPathHash(crate::module::Type::method) + stable-hashed query value

Result:
    same logical definition across sessions can map to the same dep node
```

Stable hashing is what lets rustc compare this run with a previous run. It is also why identity work in HIR lowering matters: if every edit renumbered every definition, the dep graph would be too noisy to help. See [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md) for the DefId/DefPathHash layer that makes middle-end identity tractable.

---

## Work Products and the On-Disk Cache

```
target/debug/incremental/
  crate_name-hash/
    session-hash/
      dep-graph data
      serialized query results
      work products:
        codegen-unit object files
        bitcode/auxiliary backend artifacts as needed
```

| Cached thing | Why it matters | Link |
|--------------|----------------|------|
| Query results | Avoid rerunning typeck/MIR/trait-related computations when green | [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md) |
| DepGraph edges | Know what must be validated before reuse | This guide |
| CGU work products | Skip backend/LLVM for unchanged codegen units | [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md), [12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md) |
| Metadata dependencies | Detect cross-crate interface changes | [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md) |

Codegen-unit sizing is the familiar granularity trade-off. Smaller or more numerous CGUs can improve parallelism and incremental reuse because fewer objects become dirty. Larger or fewer CGUs can improve optimization visibility, especially with LTO, but give the cache coarser chunks to reuse.

The cache path is intentionally mundane: under Cargo dev builds it is normally `target/debug/incremental/`. With direct rustc you can choose it with `-C incremental=<dir>`. Do not build tooling that depends on the subdirectory names or file formats.

---

## Limits of Reuse

| Limit | Practical effect |
|-------|------------------|
| Compiler version changes | Cache is version-tagged and discarded on mismatch |
| Relevant flags/profile/target changes | New session configuration invalidates affected nodes |
| Dependency metadata changes | Public API changes can invalidate downstream crates broadly |
| Not every query is cacheable | Some computations are always rerun or only tracked for dependencies |
| Macro expansion/build script/proc macro effects | Generated tokens or env-derived output can dirty large regions [18](18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md) |
| Release profile defaults | Cargo enables incremental for dev/debug by default, disables it for release by default |

```
Edit local function body:
    often narrow red region -> good reuse

Edit public trait signature:
    dependency metadata changes -> broad downstream invalidation

Switch rustc version or target flags:
    old cache no longer trusted -> rebuild
```

The release default is a policy choice, not a correctness limitation. Release builds often optimize harder, use fewer CGUs, or enable LTO/PGO; the compile-time reuse trade-off is less compelling than preserving predictable optimization behavior.

---

## Commands and Traces

| Goal | Command |
|------|---------|
| Force Cargo dev incremental on | `CARGO_INCREMENTAL=1 cargo build` |
| Force it off for comparison | `CARGO_INCREMENTAL=0 cargo build` |
| Inspect Cargo-level scheduling | `cargo build --timings` |
| Use rustc incremental directly | `rustc -C incremental=./inc src/lib.rs` |
| Nightly verify ICH behavior | `-Z incremental-verify-ich` (UNSTABLE) |
| Nightly self-profile rustc | `-Z self-profile` (UNSTABLE) |

```
# Cargo layer: incremental is normally on for dev, off for release.
CARGO_INCREMENTAL=1 cargo build
Get-ChildItem target\debug\incremental

# Direct rustc layer: choose the incremental cache directory explicitly.
rustc --crate-type=lib -C incremental=.\inc src\lib.rs

# Compare with Cargo's unit-level view.
cargo build --timings

# UNSTABLE/nightly: validate incremental hashes and profile rustc itself.
RUSTFLAGS="-Z incremental-verify-ich -Z self-profile" cargo +nightly build
```

The `-Z` flags are introspection and compiler-development tools. They are intentionally not stable interfaces for production build systems.

---

## Old world -> New World Bridge

| Old world | Rust mapping | Difference that matters |
|-----------|--------------|-------------------------|
| Roslyn red-green trees | rustc red-green dep graph | Same lineage of identity-plus-reuse thinking, but rustc applies it to query results, not just syntax trees |
| MSBuild timestamp incremental | Cargo freshness | Cargo is coarser than rustc, but still build-graph-level |
| Bazel/make task graph | rustc DepGraph inside one compiler process | Task graph is at query granularity, not project target granularity |
| ccache/sccache | Whole invocation cache layer | Separate third layer; rustc incremental is inside an invocation |
| PDB/source-server rebuild intuition | Fingerprints as content identity | rustc trusts hashes and dependency edges, not only file mtimes |

This is the compiler equivalent of moving from "did the file timestamp change?" to "did the semantic value this downstream computation read change?" The intellectual move is familiar from Roslyn, Bazel, and content-addressed caches, but rustc's implementation is private and tuned to its query model.

---

## Stability and Ownership Boundaries

| Thing | Treat as |
|-------|----------|
| Correct rebuild behavior | Observable stable expectation |
| `-C incremental=<dir>` | Stable rustc knob |
| `CARGO_INCREMENTAL` | Stable Cargo environment control |
| Cargo profile `incremental` setting | Stable Cargo configuration surface |
| Dep-node kinds, edge layout, fingerprint scheme | rustc internal |
| On-disk incremental cache format | rustc internal, version-sensitive |
| Work-product filenames/layout | rustc internal |
| `-Z incremental-verify-ich`, `-Z self-profile` | Nightly unstable introspection |

The only contract you should build on is correctness: rustc and Cargo must rebuild when necessary. Cache structure, dep-node names, fingerprints, and work-product layout can change every release.

---

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| Speed normal debug edit/build loops | Leave Cargo dev incremental enabled |
| Compare clean vs incremental behavior | `cargo clean`, then `CARGO_INCREMENTAL=0/1 cargo build` |
| See crate-level scheduling bottlenecks | `cargo build --timings` and read [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md) |
| Investigate rustc internal reuse | Nightly `-Z self-profile` / incremental verification, not production tooling |
| Improve incremental codegen reuse | Understand CGU partitioning in [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md) |
| Maximize release runtime performance | Consider fewer CGUs, LTO, PGO; do not assume incremental helps |
| Share build results across machines | Use sccache-like invocation caching, separate from rustc incremental |
| Delete suspicious cache state | Remove `target\debug\incremental` or run `cargo clean` |

---

## Common Confusion Points

| Confusion | Correction |
|-----------|------------|
| "Cargo incremental and rustc incremental are the same thing." | Cargo decides whether to invoke rustc; rustc reuses internal query/codegen work once invoked. |
| "Green means no upstream file changed." | Green means the query value is proven unchanged, possibly after validating changed inputs. |
| "Fingerprints are stable public IDs." | They are rustc-internal cache keys, not an API. |
| "The incremental cache is portable." | It is version-, flag-, and target-sensitive; treat it as disposable. |
| "Release builds cannot use incremental." | They can be configured to, but Cargo defaults it off for release. |
| "More codegen units are always better." | More CGUs can improve reuse/parallelism but may reduce optimization visibility. |
| "sccache replaces rustc incremental." | It operates at whole-invocation granularity; rustc incremental operates inside a crate compilation. |

---

## Primary Sources

| Source | Use it for |
|--------|------------|
| rustc-dev-guide: Incremental compilation | High-level model and implementation orientation |
| rustc-dev-guide: Incremental compilation in detail | Red-green marking, dep graph, fingerprints, work products |
| rustc-dev-guide: Queries | Why demand-driven queries make the DepGraph possible |
| rustc-dev-guide: Salsa-style overview | Conceptual lineage for query-based incremental computation |
| rustc-dev-guide: Debugging and testing incremental | `-Z` verification and compiler-development diagnostics |
| The rustc book: `-C incremental` | Stable direct rustc knob |
| The Cargo Book: build cache, profiles, freshness | Cargo-level fingerprints and profile defaults |

*Cross-links:* read the query-system guide [03](03-RUSTC-DRIVER-SESSION-AND-QUERY-SYSTEM.md) first, then identity in [06](06-HIR-LOWERING-AND-MIDDLE-LEVEL-IDENTITY.md), CGUs in [11](11-MONOMORPHIZATION-CODEGEN-UNITS-AND-VTABLES.md), backend costs in [12](12-BACKENDS-LLVM-CRANELIFT-GCC-AND-MACHINE-CODE.md), metadata/link artifacts in [13](13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md), and Cargo freshness in [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md).