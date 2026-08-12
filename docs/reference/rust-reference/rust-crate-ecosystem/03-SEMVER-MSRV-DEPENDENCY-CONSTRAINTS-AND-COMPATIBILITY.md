---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:semver-msrv-constraints-compatibility
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: SemVer, MSRV, Dependency Constraints, and Compatibility
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/03-SEMVER-MSRV-DEPENDENCY-CONSTRAINTS-AND-COMPATIBILITY.md
canonical_path: rust-crate-ecosystem/03-SEMVER-MSRV-DEPENDENCY-CONSTRAINTS-AND-COMPATIBILITY.md
backsource_ids: [mdloom-backfill:rust-crate-ecosystem:03-semver-msrv-constraints-compatibility]
concepts: [cargo semver, MSRV, rust-version, dependency constraints, compatibility]
root_concepts: [cargo compatibility]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# SemVer, MSRV, Dependency Constraints, and Compatibility

## The Big Picture

Compatibility is the intersection of four ranges: API versions, compiler
versions, enabled features, and target environment. Cargo can solve declared
version requirements. It cannot infer undeclared compatibility policy.

```
+===========================================================================+
|                       SUPPORTABLE VERSION SPACE                           |
+===========================================================================+
|                                                                           |
|  manifest requirement  intersect  available releases                     |
|          |                         |                                      |
|          v                         v                                      |
|     Cargo resolver ----------> concrete version in Cargo.lock             |
|          |                                                                |
|          v                                                                |
|  rust-version/MSRV + features + target + native/tool constraints          |
|          |                                                                |
|          v                                                                |
|                  actually supported profile                               |
+===========================================================================+
```

Treat SemVer as an API-change protocol, MSRV as a compiler-compatibility
contract, and the lockfile as one selected graph. None substitutes for the
others.

## Cargo Version Requirements

Cargo's default plain requirement uses caret semantics.

| Requirement | Representative accepted range |
|-------------|-------------------------------|
| `"1.2.3"` or `"^1.2.3"` | `>=1.2.3, <2.0.0` |
| `"0.2.3"` | `>=0.2.3, <0.3.0` |
| `"0.0.3"` | `>=0.0.3, <0.0.4` |
| `"~1.2.3"` | `>=1.2.3, <1.3.0` |
| `">=1.2, <1.5"` | Explicit bounded interval |
| `"=1.2.3"` | Exactly 1.2.3 |
| `"*"` | Any version; usually too broad for policy |

Cargo's compatibility treatment before `1.0` is intentionally narrower than
"anything below 1.0 can break." Releases sharing the left-most nonzero component
are considered compatible by caret requirements.

```toml
[dependencies]
serde = "1.0"
tracing = "~0.1.40"
protocol-types = { version = ">=0.4.2, <0.6", default-features = false }
```

Use the broadest range the library genuinely supports, not the broadest range
that happens to compile today. Exact pins in reusable libraries often prevent
downstream resolution and security updates. Exact selection belongs primarily
in an application lockfile.

## SemVer Is a Promise, Not Proof

Cargo trusts package versions; it does not compare public APIs. SemVer-breaking
changes can be accidental, and behavior can change without type-level breakage.

| Change | Usually SemVer impact | Caveat |
|--------|-----------------------|--------|
| Remove public item | Major | Unless item was explicitly outside support |
| Add required trait item without a default | Major | Existing implementations stop compiling |
| Add defaulted trait item | Often compatible, but not risk-free | Name collisions and downstream method resolution can break |
| Add enum variant | Can break exhaustive downstream matches | `#[non_exhaustive]` changes expectations |
| Tighten generic bound | Often breaking | May reject existing callers |
| Raise MSRV | Policy-dependent | Must be documented; Cargo package version rules do not enforce a universal MSRV policy |
| Change defaults/features | Can be behaviorally or build breaking | Even if API types compile |
| Fix unsoundness | May require breaking surface | Security and SemVer can conflict |

API diff tools can add evidence, but they do not fully prove semantic
compatibility. Pin the tool version and review its documented coverage.

## MSRV and `rust-version`

MSRV is the minimum supported Rust version. Cargo exposes it as package metadata:

```toml
[package]
name = "route-core"
version = "0.4.0"
edition = "2021"
rust-version = "1.82"
```

`rust-version` lets Cargo reject a package when the active compiler is too old
and informs newer resolver behavior. It does not prove every feature or target
works at that compiler. CI must exercise the declared minimum.

```
stable/current CI: full tests, clippy, docs
MSRV CI:           cargo check/test for promised feature set
target CI:         supported cross-target checks
```

Resolver behavior matters:

| Resolver | Key compatibility behavior |
|----------|----------------------------|
| `"1"` | Legacy feature unification and selection behavior |
| `"2"` | Separates several host/dev/target feature contexts; edition 2021 commonly implies it |
| `"3"` | Adds MSRV-aware selection behavior; edition 2024 implies it for non-virtual packages |

Resolver v3 became available in Cargo 1.84. Edition 2024, stabilized with Rust
1.85, infers resolver v3 for non-virtual packages; a virtual workspace has no
package edition, so set its resolver explicitly. Resolver choice is made by the
workspace/root and dependency manifests cannot override it.

Resolver v3 defaults incompatible-Rust-version handling to `fallback`: it
prefers dependency releases whose declared `rust-version` is compatible with
the applicable package/workspace Rust-version inputs. This is a preference, not
a proof or absolute filter; Cargo may still select an incompatible release when
no compatible candidate satisfies the other constraints. Missing or inaccurate
metadata also limits protection. Always describe behavior against the Cargo
version actually used.

```toml
[workspace]
members = ["crates/*"]
resolver = "3"
```

## Constraint Placement

```
library Cargo.toml: compatible range
          |
          v
application Cargo.lock: exact selected graph
          |
          v
CI policy: allowed sources, versions, advisories, licenses, MSRV
```

| Mechanism | Use for | Avoid using it as |
|-----------|---------|-------------------|
| Version requirement | Express API compatibility | Security freeze |
| `Cargo.lock` | Reproduce one graph | Public API compatibility statement |
| `[patch]` | Temporarily replace a package source/version in a graph | Permanent invisible fork policy |
| `cargo update -p` | Deliberately move selected package | Manifest compatibility declaration |
| Policy deny/allow list | Enforce organizational constraints | Resolver replacement |

For a targeted update:

```text
cargo update -p example-crate
cargo update -p example-crate --precise 1.7.4
cargo test --locked --workspace
```

`--precise` changes the lockfile selection; it does not override the manifest's
version/source constraints. If multiple versions of the package exist, Cargo
may require an unambiguous package specification naming the currently selected
package, for example
`cargo update -p example-crate@1.7.3 --precise 1.7.4`.

## Compatibility Matrix

Make the support claim executable.

| Axis | Minimum lane | Current lane | Exceptional lane |
|------|--------------|--------------|------------------|
| Rust | Declared MSRV | Current stable | Beta/nightly observation only |
| Features | Minimum supported | Default | Selected optional/maximal |
| Target | Lowest-priority supported | Primary deployment | Cross/embedded/WASM |
| Dependency graph | Lowest allowed where practical | Locked production graph | Candidate update |

Cargo's unstable minimal-version flags are not a stable general-purpose proof
and can expose ecosystem metadata patterns that were never designed for global
minimum resolution. Label such experiments nightly and interpret failures
carefully. A more useful library test is often to pin selected direct
dependencies at documented lower bounds in a controlled job.

## Old World -> New World Bridge

SemVer plus `Cargo.lock` separates the role often collapsed into one package
version in older systems:

| Familiar concept | Cargo equivalent |
|------------------|------------------|
| Assembly/package reference range | `Cargo.toml` version requirement |
| Restored exact dependency set | `Cargo.lock` |
| Target framework/compiler floor | `rust-version` plus target matrix |
| Binding redirect/central override | Resolver selection, `[patch]`, targeted update |
| NuGet central package version | `[workspace.dependencies]` plus shared lockfile |

The important difference is that Cargo may compile multiple incompatible major
versions simultaneously. There is no universal single-version binding redirect.

## Common Confusion Points

- **"`1.2` means exactly 1.2."** It means a caret-compatible range.
- **"SemVer guarantees behavior."** It is an upstream versioning promise, not a
  formal equivalence proof.
- **"`rust-version` makes MSRV true."** It declares and helps enforce a floor;
  CI proves the actual profile.
- **"Resolver 3 solves all MSRV failures."** It can prefer compatible metadata;
  it cannot repair missing declarations, build scripts, or target constraints.
- **"Exact pins are safest everywhere."** In libraries they can make the graph
  less resolvable and updates harder. Use locks and policy at the product edge.

## Decision Cheat Sheet

| Need | Use | Verify |
|------|-----|--------|
| Reusable library compatibility | Caret or explicit compatible range | Lower bound and current graph |
| Application reproducibility | Committed `Cargo.lock` | `--locked` CI |
| Compiler floor | `rust-version` | MSRV lane with supported features/targets |
| Emergency transitive override | Bounded `[patch]` or targeted lock update | Removal date and upstream issue |
| 2024 workspace MSRV-aware selection | `resolver = "3"` | Cargo 1.84+ and accurate dependency metadata |
| Breaking API review | Release notes plus API diff and tests | Semantic migration paths |

## Primary Sources

- Cargo dependency specification: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
- Cargo resolver: https://doc.rust-lang.org/cargo/reference/resolver.html
- Cargo `rust-version`: https://doc.rust-lang.org/cargo/reference/rust-version.html
- Cargo SemVer compatibility: https://doc.rust-lang.org/cargo/reference/semver.html
- Cargo update: https://doc.rust-lang.org/cargo/commands/cargo-update.html

## Related Guides

- Previous: [02-EVALUATION-SCORECARDS-AND-EVIDENCE.md](02-EVALUATION-SCORECARDS-AND-EVIDENCE.md)
- Next: [04-CARGO-FEATURES-UNIFICATION-AND-OPTIONAL-DEPENDENCIES.md](04-CARGO-FEATURES-UNIFICATION-AND-OPTIONAL-DEPENDENCIES.md)
- Lockfile policy: [06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md](06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md)
