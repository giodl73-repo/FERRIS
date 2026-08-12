---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-architecture:project-governance-release-train
kind: guide
module: rust-architecture
section: rust-architecture
title: Project Governance, RFCs, Editions, and the Release Train
status: source-custody
source_custody: partial
current_path: rust-architecture/01-PROJECT-GOVERNANCE-RFCS-AND-RELEASE-TRAIN.md
canonical_path: rust-architecture/01-PROJECT-GOVERNANCE-RFCS-AND-RELEASE-TRAIN.md
backsource_ids: [mdloom-backfill:rust-architecture:01-project-governance-release-train]
concepts: [rust governance, rfc process, editions, release channels, stability, compatibility]
root_concepts: [rust governance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Project Governance, RFCs, Editions, and the Release Train

## The Big Picture

Rust evolves by separating **authority**, **implementation**, and **distribution**. The language is governed by the Rust Project through teams and RFCs; `rustc` implements that language; `rustup` and the release train distribute toolchains on a predictable cadence.

The useful mental model is not "the compiler team decides Rust." The compiler team owns the reference compiler; language design, library API, Cargo behavior, releases, and infrastructure each have their own authorities.

```
+===========================================================================+
|                 RUST GOVERNANCE AND RELEASE PIPELINE                      |
|                                                                           |
|  RUST PROJECT GOVERNANCE                                                  |
|  Leadership Council · project teams · lang/libs/compiler/cargo/infra      |
|  Foundation: trademark/legal/funding, NOT language design                 |
+---------------------------------------------------------------------------+
             | charters teams; teams own decisions in their domains
             v
+---------------------------------------------------------------------------+
|  DESIGN PROCESS                                                           |
|  Rust Reference · RFCs · editions · tracking issues · stabilization FCP   |
|  Authority: lang/types/libs-api/cargo/etc. depending on subject           |
+---------------------------------------------------------------------------+
             | accepted design becomes an unstable implementation task
             v
+---------------------------------------------------------------------------+
|  IMPLEMENTATION                                                           |
|  rustc feature gates · std stability attributes · Cargo behavior          |
|  Authority: compiler/libs/Cargo teams. Internals are NOT stable.          |
+---------------------------------------------------------------------------+
             | rides the release channels when stabilized
             v
+---------------------------------------------------------------------------+
|  RELEASE TRAIN                                                            |
|  nightly -> beta -> stable, six-week cadence, point releases as needed    |
|  Authority: release + infra teams, with owning teams approving content    |
+---------------------------------------------------------------------------+
             | users opt in by toolchain channel and crate edition
             v
+---------------------------------------------------------------------------+
|  USER SURFACE                                                             |
|  stable Rust 1.x · Cargo.toml edition · crates.io semver · lints          |
+===========================================================================+
```

Read [02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md) for how a machine selects one of these channels, [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md) for crate semver, and [20](20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md) for Crater, CI, and distribution.

---

## The Authority Stack

Rust governance is deliberately federated. The **Rust Project** is the technical project: teams make technical decisions in scoped areas. The **Leadership Council**, created by RFC 3392, replaced the old core team as the top-level governance body, but it is not a language-design committee that hand-edits syntax.

| Authority | Owns | Does not own |
|-----------|------|--------------|
| **Leadership Council** | Project-wide governance, accountability, team structure | Day-to-day language or compiler design |
| **lang team** | Language design, editions, Reference-facing semantics | rustc internal architecture |
| **types team** | Type-system design/implementation direction, trait solving collaboration | Cargo/package policy |
| **libs-api team** | Stable standard-library API surface | Private std implementation details alone |
| **compiler team** | `rustc`, compiler internals, implementation strategy | Independent authority to change the language contract |
| **Cargo team** | Cargo UX, resolver, manifests, build orchestration | `rustc` type checking or LLVM codegen |
| **infra/release teams** | CI, artifacts, channels, release mechanics | Feature approval outside owning teams |
| **devtools teams/projects** | rustdoc, rustfmt, clippy, rust-analyzer, Miri | Language design unless routed through RFC/team process |
| **Rust Foundation** | Legal, trademark, funding, sponsorship support | Designing Rust the language |

```
+-------------------+      +-------------------+      +-------------------+
| Governance        | ---> | Domain teams      | ---> | Implementations   |
| council/process   |      | lang/libs/cargo   |      | rustc/cargo/std   |
+-------------------+      +-------------------+      +-------------------+
        |                          |                          |
        v                          v                          v
 project legitimacy         technical authority          versioned artifacts
```

The Reference and edition/RFC process are the public language authority. `rustc` is the reference compiler, but a `TyCtxt`, query key, MIR pass, or `-Z` diagnostic dump is an implementation detail, not a spec. The standard library is similar: `std::vec::Vec`'s stable API is a contract; its internal layout strategy is not a general promise beyond documented guarantees.

---

## RFCs, Feature Gates, and Stabilization

The Rust RFC process is a funnel, not a queue of guaranteed work. A substantial design starts as a pull request against `rust-lang/rfcs`, receives public discussion, and enters a **final comment period** when the owning team believes the disposition is clear. Team sign-off and "disposition merge" merge the RFC text. That still means: accepted direction, not shipped feature.

```
+-------------+    +-------------+    +-------------+    +-------------+
| RFC PR      | -> | Discussion  | -> | FCP + team  | -> | Merged RFC  |
| proposal    |    | tradeoffs   |    | sign-off    |    | design text |
+-------------+    +-------------+    +-------------+    +-------------+
                                                               |
                                                               v
+-------------+    +-------------+    +-------------+    +-------------+
| Tracking    | -> | Nightly     | -> | Stabilize   | -> | Stable Rust |
| issue       |    | feature gate|    | report/FCP  |    | release     |
+-------------+    +-------------+    +-------------+    +-------------+
```

Small changes can skip a full RFC when the owning team treats them as implementation, clarification, or narrow policy work. Conversely, a merged RFC can sit unimplemented for years.

Nightly feature gates are explicit unstable surface:

| Mechanism | Example | Contract |
|-----------|---------|----------|
| Crate feature gate | `#![feature(let_else)]` | Nightly only; can change before stabilization |
| Compiler `-Z` flag | `rustc -Z unstable-options` | Unstable implementation/debug surface |
| Standard-library stability attrs | `#[stable]`, `#[unstable(feature = "...")]`, `#[rustc_const_stable]` | Internal machinery; stable/unstable distinction is public |
| Stable CLI/language surface | `rustc --edition 2021`, stable std APIs | Public compatibility promise |

The stability attributes live in the Rust source tree and are compiler-recognized internal machinery for the libs process. Users should rely on the documented stable/unstable boundary, not on copying those attributes into ordinary crates.

---

## Release Train and Channels

Rust uses a train model: trains leave on schedule; features board when ready. Nightly is cut every night from active development. Periodically, a beta branch is cut from nightly. After roughly six weeks on beta, that branch becomes stable. Critical regressions or security fixes can produce point releases.

| Channel | Cadence | Purpose | Risk profile |
|---------|---------|---------|--------------|
| **nightly** | Every night | Experimentation, gated features, `-Z` flags | Unstable; can regress or change |
| **beta** | Six-week stabilization window | Release candidate for next stable | Mostly stable; fixes only |
| **stable** | Six-week release cadence | Production Rust 1.x | Stability guarantee applies |
| **point release** | As needed | Critical fix on stable | Narrow, conservative |

```
nightly:  N N N N N N N N N N N N N N
                 |
                 | branch
                 v
beta:            B B B B B B
                             |
                             | promote
                             v
stable:                      S --------> S+1 --------> S+2
```

Rust version numbers are 1.x; Rust 1.0 shipped in May 2015. A stabilized feature typically spends time on nightly, then one beta cycle, then stable. See [02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md) for selecting these channels with `rustup` and [20](20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md) for how artifacts are built and distributed.

---

## Editions: Opt-In Source Evolution

Editions are Rust's mechanism for source-level evolution without ecosystem fracture. The editions are **2015**, **2018**, **2021**, and **2024**. A crate opts in through `Cargo.toml`; crates of different editions interoperate in the same dependency graph because the same `rustc` supports all editions.

| Property | Edition behavior |
|----------|------------------|
| Scope | Per crate, declared in `Cargo.toml` |
| Changes | Source-level parsing/name-resolution/idiom/default changes |
| Non-goal | Not a new compiler line, not Rust 2.0, not a semver break |
| Migration | `cargo fix --edition` applies machine-checkable rewrites |
| Interop | 2015/2018/2021/2024 crates link together normally |

```
+----------------+      +----------------+      +----------------+
| crate A        | ---> | crate B        | ---> | crate C        |
| edition 2018   |      | edition 2021   |      | edition 2024   |
+----------------+      +----------------+      +----------------+
          \                 same rustc, same linker, same Cargo graph
           +---------------------------------------------------------->
```

An edition can reserve a keyword or alter a default only because existing crates do not silently move editions. That is the central compatibility trick: the language surface can modernize while old code keeps compiling under its declared edition. For language semantics beyond architecture, see `../rust-language/` where it exists.

---

## Compatibility, Crater, and Ecosystem Semver

Rust's stability guarantee is strong but not metaphysical. Stable code should keep compiling across stable releases, subject to carefully managed caveats: soundness fixes, inference changes, new warnings, future-incompatibility lints, and cases where the old behavior was never a promised contract.

| Layer | Compatibility mechanism | Owner |
|-------|-------------------------|-------|
| Language | Reference, RFC 1122-style forward-compatibility, editions | lang/types |
| Standard library | stable/unstable API attributes and docs | libs-api/libs-impl |
| Compiler | stable CLI and behavior; internals may change | compiler |
| Ecosystem | crate semver, Cargo resolver, lockfiles | crate authors/Cargo |
| Release quality | Crater runs, CI, beta soak, point releases | release/infra + owning teams |

Crater builds large portions of the ecosystem to measure real breakage before stabilization or risky compiler changes. It is evidence, not a veto machine, but it gives the project empirical leverage. Crate semver is separate: Cargo and crates.io conventions decide dependency compatibility, not the language spec. That boundary is [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md).

---

## A Single Trace: A Feature Becoming Stable

The exact feature changes, but the shape is stable. `let_else` is a useful concrete example: RFC 3137, tracking issue 87335, nightly feature gate, then stabilization in Rust 1.65.

1. Design text landed in `rust-lang/rfcs#3137`.
2. Implementation and stabilization work was tracked in
   `rust-lang/rust#87335`.
3. Before Rust 1.65, the source required a nightly compiler and an explicit
   feature gate:

```rust
#![feature(let_else)]

fn parse_port(s: &str) -> u16 {
    let Ok(port) = s.parse::<u16>() else {
        return 8080;
    };
    port
}
```

For a dependency-free crate, a dated pre-stabilization nightly makes that
historical state reproducible:

```text
rustup toolchain install nightly-2022-08-11
cargo +nightly-2022-08-11 build
```

4. Rust 1.65 stabilized `let-else`; remove `#![feature(let_else)]`, then the
   released stable toolchain accepts the same body:

```text
rustup toolchain install 1.65.0
cargo +1.65.0 build
```

A crate's edition is orthogonal to that channel choice:

```toml
[package]
name = "service-core"
version = "0.1.0"
edition = "2021"
```

Nightly enables experiments; editions select source dialect; stable releases carry the compatibility promise. Do not collapse those three axes.

---

## Old World -> New World

The closest analogues are all imperfect, but useful.

| Old world / adjacent model | Rust analogue | Important difference |
|----------------------------|---------------|----------------------|
| C# `LangVersion` | Cargo `edition = "2021"` | Editions are per-crate and supported by one rustc line |
| .NET SDK selected by `global.json` | rustup toolchain override or `rust-toolchain.toml` | rustup selects compiler/Cargo/std bundle; Cargo is still the build tool |
| TFMs like `net8.0` | Rust target triples and std availability | Editions are not target frameworks; targets are platform/codegen choices |
| ECMAScript TC39 stages | RFC -> unstable -> stabilization | Rust stabilization is tied to toolchain channels and feature gates |
| Chrome/Chromium channels | nightly/beta/stable release train | Rust has stronger stable source compatibility expectations |

```
C# language version     ->  Rust edition
.NET SDK band           ->  Rust toolchain channel/version
NuGet semver            ->  crates.io semver
Roslyn implementation   ->  rustc implementation
BCL API stability       ->  std stable API surface
```

The bridge to keep: Rust avoided "Rust 2.0" as a breaking language fork. It chose opt-in editions plus a fast release train.

---

## Decision Cheat Sheet

| Question | What | When | Who owns it |
|----------|------|------|-------------|
| Should this change get an RFC? | RFC PR in `rust-lang/rfcs` | Broad language/library/Cargo policy change | Owning team, often lang/libs/Cargo |
| Can I use this feature in production? | Stable channel, no feature gate | Production builds and libraries | release + owning teams |
| Can I try an experiment? | Nightly with `#![feature]` or `-Z` | Prototyping, compiler work, early adoption | compiler/lang/libs; unstable |
| How do I modernize syntax? | `cargo fix --edition`, then `edition = "2024"` | Opting a crate into a new edition | lang + Cargo tooling |
| Did a compiler change break crates? | Crater/beta/future-incompat lints | Pre-release risk assessment | release/infra/compiler |
| Is a dependency compatible? | Cargo semver and resolver | Ecosystem dependency selection | crate authors + Cargo |
| Who decides std API stability? | stabilization report/FCP, stability attrs | New or changed public std API | libs-api |

---

## Common Confusion Points

- **A merged RFC is not a shipped feature.** It is accepted design; implementation, tracking, and stabilization remain.
- **Nightly is not "Rust plus extras."** It is an unstable channel where feature gates and `-Z` flags may change or vanish.
- **Editions are not compiler versions.** The same current `rustc` compiles crates from every edition.
- **The Rust Foundation is not the language design authority.** It supports legal/trademark/funding work; project teams own technical decisions.
- **Standard-library stability attributes are not user extension points.** The public contract is whether an API is stable, not the private attribute protocol.
- **Semver for crates is not Rust's language stability guarantee.** Cargo dependency compatibility is an ecosystem layer. See [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md).

| Confusion | Correct boundary |
|-----------|------------------|
| "rustc internals define Rust" | The Reference/RFCs define the language; rustc implements it |
| "edition 2024 means Rust 2024 compiler" | Edition is a per-crate source mode |
| "beta is for experiments" | Nightly is for experiments; beta is release soak |

---

## Primary Sources

| Source | Use it for |
|--------|------------|
| `github.com/rust-lang/rfcs` | RFC text, history, and accepted design records |
| **The Rust RFC Book** | Process, lifecycle, and stabilization conventions |
| **RFC 3392** | Leadership Council governance model replacing the old core team |
| **The Rust Edition Guide** — `doc.rust-lang.org/edition-guide` | Edition semantics and migration guidance |
| **Rust Forge release process docs** | Release train mechanics, beta/stable process, point releases |
| **Rust Blog release notes** | Stable release contents and version history |
| **Standard library docs / stability policy** | Stable vs unstable library API surface |

*Cross-links:* [02](02-RUSTUP-TOOLCHAINS-COMPONENTS-AND-TARGETS.md) for channels on disk, [17](17-CARGO-RESOLUTION-WORKSPACES-AND-FINGERPRINTING.md) for crate semver, [20](20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md) for Crater, CI, and distribution.