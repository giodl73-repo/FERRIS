---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:crate-taxonomy-discovery-alternatives
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Crate Taxonomy, Discovery, and Alternatives
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/01-CRATE-TAXONOMY-DISCOVERY-AND-ALTERNATIVES.md
canonical_path: rust-crate-ecosystem/01-CRATE-TAXONOMY-DISCOVERY-AND-ALTERNATIVES.md
backsource_ids: [proof-backfill:rust-crate-ecosystem:01-crate-taxonomy-discovery-alternatives]
concepts: [crate taxonomy, crate discovery, dependency alternatives, facade crate, sys crate]
root_concepts: [crate discovery]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Crate Taxonomy, Discovery, and Alternatives

## The Big Picture

The first evaluation error is comparing crates that occupy different layers.
A facade, a runtime, a protocol model, and a `-sys` binding may all appear in a
search for the same capability, but they create different obligations.

```
+===========================================================================+
|                         CAPABILITY TO CANDIDATE                           |
+===========================================================================+
| Need                                                                     |
|  |                                                                       |
|  +-> standard library? -> small internal implementation? -> external?     |
|                                      |                                    |
|                                      v                                    |
|  facade/API -> implementation -> runtime/framework -> native binding      |
|      |              |                |                  |                 |
|      v              v                v                  v                 |
|  low policy     algorithm/code    architectural      system toolchain     |
|  surface        dependency        commitment         and ABI exposure     |
+===========================================================================+
```

Name the layer before naming candidates. "HTTP" might mean URI parsing, message
types, a client, a server framework, TLS, an async runtime integration, or all
of them. Those are not substitutes.

## A Working Taxonomy

| Crate type | Typical responsibility | Main evaluation pressure |
|------------|------------------------|--------------------------|
| Leaf utility | One algorithm, format, or data structure | API quality, MSRV, dependency weight |
| Data/model crate | Types and serialization contracts | SemVer stability, feature design |
| Facade | Stable API over selectable implementations | Abstraction leakage, backend policy |
| Runtime/framework | Scheduling, lifecycle, global conventions | Architectural lock-in, operational model |
| Adapter/integration | Connects two ecosystems | Version matrix, feature fan-out |
| `-sys`/FFI binding | Raw native ABI and link discovery | Native provenance, ABI, build scripts |
| Safe native wrapper | Rust invariants over a `-sys` layer | Soundness boundary, upstream native lifecycle |
| Proc macro | Runs during compilation and emits tokens | Build-time trust, diagnostics, compiler compatibility |
| Build/tool crate | Code generation, packaging, test support | Host execution, reproducibility |
| Facade plus re-exports | Curated ecosystem entry point | Hidden dependency breadth, version coupling |

Taxonomy is not branding. Read `Cargo.toml`, public modules, build scripts, and
dependency edges. A crate called "core" can be a framework; a crate called
"client" can re-export an entire runtime.

## Discovery Funnel

Use broad signals only to create a candidate set, then switch to primary
evidence.

```
problem statement
      |
      v
standard library / platform API / existing workspace capability
      |
      v
crates.io search + docs.rs + repository topics + known ecosystem maps
      |
      v
3-5 candidates classified by layer
      |
      v
manifest/docs/source/release review
      |
      v
1-2 executable spikes and an explicit "build instead" alternative
```

Useful commands:

```text
cargo search <term> --limit 10
cargo info <crate>
cargo tree -p <candidate>
cargo tree -p <candidate> -e features
```

`cargo search` and `cargo info` depend on registry support and the installed
Cargo version. They are discovery tools, not approvals. docs.rs builds are also
configuration-specific: a successful docs.rs build does not prove every target
or feature set works.

## The Alternatives Must Include "No Crate"

Before comparing packages, compare solution shapes.

| Alternative | Prefer when | Reject when |
|-------------|-------------|-------------|
| Standard library | Capability is present and sufficient | Missing protocol/algorithm or excessive custom glue |
| Small internal module | Behavior is narrow, stable, and easy to test | Domain is security-sensitive or deceptively complex |
| External crate | Mature implementation materially reduces risk or effort | Governance and transitive cost exceed value |
| Platform service/API | Capability belongs operationally outside process | Latency, availability, or portability makes it unsuitable |
| Generate at build time | Source data/schema is authoritative and generation is deterministic | Generator adds opaque host execution or unstable outputs |
| Vendor/fork | Continuity or auditability requires custody | Team cannot sustain divergence |

The "not invented here" and "not invented there" biases are symmetric. A
200-line parser can hide years of edge cases; a 20-crate dependency tree can be
an absurd price for three convenience functions.

## Candidate Card

Capture the same facts for every candidate:

```text
Candidate: example-crate
Layer: facade / implementation / runtime / adapter / native / tool
Required API: ...
Source: crates.io / alternate registry / git / path
License expression: ...
Declared rust-version: ...
Default features: ...
Required features: ...
Direct dependencies: ...
Build.rs / proc macro / native code: yes/no
Supported targets claimed: ...
Release and security policy: ...
Exit option: replace / fork / internalize / remove
```

This card prevents a polished README from defining the comparison. It also makes
missing evidence visible.

## Popularity Is a Weak Signal

Downloads and stars can indicate ecosystem visibility, documentation demand, or
the chance that someone has encountered a bug. They do not establish quality.

| Signal | What it may indicate | Why it is weak |
|--------|----------------------|----------------|
| crates.io downloads | Historical or current distribution activity | CI, transitive use, caching patterns, and age distort counts |
| Repository stars | Attention or interest | Not usage, maintenance, correctness, or compatibility |
| Dependents | Integration pressure and ecosystem role | Dependents may be stale or concentrated in one framework |
| Release frequency | Activity | Stable software may release rarely; churn can also be a warning |
| Contributor count | Participation | Commit count does not reveal review authority or succession |

Use these signals to ask questions, never to close them.

## Old World -> New World Bridge

Universal component selection already separates interface, implementation, and
deployment environment. Rust adds package targets and host-executed build units
to that map.

| Familiar component category | Rust ecosystem form |
|-----------------------------|---------------------|
| Interface assembly/package | Facade or types crate |
| Provider/plugin | Backend or adapter crate |
| Application framework | Runtime/framework crate |
| Native interop package | `-sys` crate plus safe wrapper |
| Code generator/MSBuild task | Build dependency, `build.rs`, proc macro, or `xtask` |

NuGet package identity is a useful supplemental bridge, but Cargo package,
crate target, and source identity are separate concepts. One package can expose
a library, binaries, examples, tests, and build scripts.

## Common Confusion Points

- **"The most downloaded crate is the default."** It is the most downloaded
  under an opaque counting process, not necessarily the best fit.
- **"Facade crates reduce dependency cost."** They can reduce API coupling while
  increasing graph breadth through re-exports and default backends.
- **"`-sys` means unsafe to use."** It signals raw native bindings by convention.
  The real question is where safety invariants are enforced.
- **"No dependencies means low risk."** A crate may contain large unsafe,
  cryptographic, generated, or native code of its own.
- **"Small crate means easy to replace."** A small API at a central boundary can
  create more coupling than a large implementation behind an internal adapter.

## Decision Cheat Sheet

| Situation | Prefer | Required follow-up |
|-----------|--------|--------------------|
| Capability exists in `std` | Standard library | Confirm target/MSRV support |
| Narrow stable behavior under a few hundred reviewable lines | Internal module | Tests, ownership, and security review where relevant |
| Complex protocol/algorithm with mature implementations | External implementation crate | [02](02-EVALUATION-SCORECARDS-AND-EVIDENCE.md) evidence pass |
| Multiple backends are realistic | Facade plus internal adapter | Feature and dependency-direction review |
| Native library is unavoidable | Established `-sys` plus safe wrapper | [12](12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md) |
| Compile-time generation is optional convenience | Avoid or isolate it | [13](13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md) |

## Primary Sources

- Cargo package layout: https://doc.rust-lang.org/cargo/guide/project-layout.html
- Cargo targets: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
- Cargo `search`: https://doc.rust-lang.org/cargo/commands/cargo-search.html
- Cargo `info`: https://doc.rust-lang.org/cargo/commands/cargo-info.html
- docs.rs: https://docs.rs/about

## Related Guides

- Previous: [00-OVERVIEW.md](00-OVERVIEW.md)
- Next: [02-EVALUATION-SCORECARDS-AND-EVIDENCE.md](02-EVALUATION-SCORECARDS-AND-EVIDENCE.md)
- Native taxonomy: [12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md](12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md)
