---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:reusable-library-and-sdk
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Reusable Library and SDK Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/08-REUSABLE-LIBRARY-AND-SDK.md
canonical_path: rust-application-blueprints/08-REUSABLE-LIBRARY-AND-SDK.md
backsource_ids: [proof-backfill:rust-application-blueprints:08-reusable-library-and-sdk]
concepts: [rust library, sdk, public api, semantic versioning, cargo features, msrv, runtime neutrality]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Reusable Library and SDK Blueprint

## The Big Picture

```
+============================================================================+
| consumer crate                                                             |
+------------------------------+---------------------------------------------+
                               v
+----------------------------------------------------------------------------+
| public facade: stable names | constructors | traits | errors | docs        |
+------------------------------+---------------------------------------------+
                               v
+----------------------------------------------------------------------------+
| semantic core -> optional capabilities -> protocol/OS adapters             |
+----------------------+----------------------+------------------------------+
                       v                      v
                 consumer runtime       external authority
```

A library executes inside someone else's process, dependency graph, runtime,
allocator, logging policy, and release schedule. Its blueprint therefore
optimizes for a small public surface, explicit ownership transfer, composable
features, bounded dependencies, and predictable compatibility.

## Package and Workspace Layout

Start with one package unless boundaries are independently useful:

```
acme-sdk/
|-- Cargo.toml
|-- src/
|   |-- lib.rs                 # curated facade and re-exports
|   |-- client.rs
|   |-- error.rs
|   `-- model/
|-- examples/
|-- tests/
`-- README.md
```

Split only when dependency or platform boundaries justify it:

```
acme-sdk/
|-- Cargo.toml
|-- crates/
|   |-- acme-types/            # protocol-neutral public values
|   |-- acme-core/             # shared behavior
|   |-- acme-sdk/              # default facade
|   |-- acme-sdk-async/        # if truly independent
|   `-- acme-sdk-test-support/ # non-production helpers
`-- examples/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*"]

[workspace.package]
edition = "2024"
rust-version = "1.85"
```

Rust 1.85 is the minimum compiler for edition 2024 and makes this example a
valid manifest, not a placeholder. A real library should choose the oldest
compiler it actually tests and can support. Member packages opt into workspace
package values explicitly, for example with `edition.workspace = true` and
`rust-version.workspace = true`.

## Public API and Authority

| Surface | Compatibility pressure |
|---------|------------------------|
| Public type/field | construction and pattern matching by consumers |
| Trait | implementors outside the crate; adding required items can break |
| Error enum | exhaustive matches can freeze variants |
| Generic bound | affects inference, monomorphization, and accepted types |
| Feature | participates additively in dependency resolution |
| Re-export | makes another crate's name/version part of the facade |
| Serialization | becomes a durable/wire contract when persisted or transmitted |

Use private fields plus constructors when invariants or future evolution matter.
Use `#[non_exhaustive]` only when the resulting construction/matching trade-off is
intentional and documented.

The library owner owns public API and MSRV policy. The protocol owner owns wire
meaning. The consumer owns task scheduling, logging subscribers, process exit,
and global configuration unless the API explicitly delegates those capabilities.

## Features, Async, and Dependencies

```
default facade
   |-- core capability (always)
   |-- feature "serde"  -> serialization integration
   |-- feature "tls-x"  -> one compatible adapter family
   `-- no hidden mutually exclusive environment modes
```

| Rule | Reason |
|------|--------|
| Features are additive | Cargo may unify them across the graph |
| Defaults should be useful but modest | consumers can opt out of weight |
| Optional dependencies remain implementation details where possible | avoids exposing graph choices |
| Never start a global runtime implicitly | consumer owns process execution |
| Accept caller-provided clients/executors where boundary matters | improves policy control and tests |
| Expose timeout/cancellation control or document caller ownership | futures can be dropped at any await point |

An async SDK may choose a runtime ecosystem, but that is a compatibility and
dependency decision, not a universal requirement. State the support boundary and
avoid claiming runtime neutrality unless tests prove it.

Library security includes the code executed at build time. Audit proc macros,
build scripts, native toolchains, default features, and transitive sources;
isolate `unsafe` behind documented invariants and test supported targets.
Libraries should not install global allocators, panic hooks, signal handlers,
logging subscribers, or trust roots unless that process-wide authority is an
explicit opt-in contract.

## Evidence, Release, and Rollback

```
unit tests
  -> doctests/examples
  -> public API integration tests
  -> minimal-feature and all-supported-feature matrix
  -> MSRV + current stable toolchain checks
  -> downstream fixture build
```

```text
cargo test --workspace --all-targets
cargo test -p acme-sdk --no-default-features
cargo test -p acme-sdk --all-features
cargo doc --workspace --no-deps
```

Only run `--all-features` as a supported case if all features are intended to
coexist.

Published versions are immutable artifacts in normal package workflows. A bad
release is recovered by publishing a corrected version, documenting impact, and
using registry withdrawal/yank mechanisms only according to registry policy;
those mechanisms do not remove already downloaded code.

Removal of an API or feature requires the declared deprecation and SemVer/MSRV
policy, downstream fixture/ecosystem evidence, migration instructions, and a
replacement or explicit end-of-support statement. The library cannot observe
every consumer, so elapsed time alone is not evidence that removal is safe.

| Change | Release implication |
|--------|---------------------|
| Internal fix | patch if documented behavior remains |
| Add compatible API | minor under common SemVer practice |
| Break public contract/MSRV policy | major or documented pre-1.0 policy |
| Wire behavior change | coordinate with service/schema compatibility, not SemVer alone |

## Universal Bridge First

The universal bridge is library ecology: unlike an application, a library is a
guest in a graph assembled by another authority. Minimize global decisions and
make capabilities explicit.

Supplementally, NuGet packages and .NET class libraries have similar public API
pressure. Cargo feature unification is the important difference: features are
not target-framework configurations or mutually exclusive build flavors.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| Reuse only inside one application | internal crate with repository policy |
| Broad external reuse | narrow facade, SemVer/MSRV policy, examples and contract tests |
| Common values across clients | small types crate if it reduces real dependency pressure |
| Multiple transports | port plus optional adapter crates |
| Consumer extension | traits only at genuine capability seams |
| Process isolation or untrusted code | plugin [09] or Wasm [10], not ordinary callbacks |
| CLI plus reusable engine | library here, thin CLI [02] |

## Common Confusion Points

- **Many crates do not guarantee modularity.** They can increase release and
  dependency complexity without adding ownership boundaries.
- **`pub` is a commitment.** Documentation omissions do not reliably make a
  reachable API private.
- **Features are not runtime switches.** Their additive compile-time behavior
  can activate combinations you did not select directly.
- **SemVer does not cover every contract.** MSRV, wire formats, performance, and
  operational behavior need explicit policy.
- **An error type is API design.** Preserve machine-actionable categories and
  sources without exposing unstable implementation details.
- **Yanking is not rollback.** Existing lockfiles and caches may continue using
  the version.
- **A library is supply-chain code.** Build scripts, proc macros, native
  dependencies, and default features execute under consumer trust.

## Primary Sources

- Cargo SemVer compatibility: https://doc.rust-lang.org/cargo/reference/semver.html
- Cargo Features: https://doc.rust-lang.org/cargo/reference/features.html
- Cargo manifest `rust-version`: https://doc.rust-lang.org/cargo/reference/rust-version.html
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- rustdoc book: https://doc.rust-lang.org/rustdoc/

## Related Guides

- CLI consumer: [02-CLI-AND-DEVELOPER-TOOL.md](02-CLI-AND-DEVELOPER-TOOL.md)
- Plugin boundary: [09-PLUGIN-AND-EXTENSION-HOST.md](09-PLUGIN-AND-EXTENSION-HOST.md)
