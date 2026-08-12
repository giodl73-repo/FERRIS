---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:cargo-features-unification-optional-dependencies
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Cargo Features, Unification, and Optional Dependencies
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/04-CARGO-FEATURES-UNIFICATION-AND-OPTIONAL-DEPENDENCIES.md
canonical_path: rust-crate-ecosystem/04-CARGO-FEATURES-UNIFICATION-AND-OPTIONAL-DEPENDENCIES.md
backsource_ids: [mdloom-backfill:rust-crate-ecosystem:04-cargo-features-unification-optional-dependencies]
concepts: [cargo features, feature unification, optional dependencies, default features, resolver]
root_concepts: [cargo features]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Cargo Features, Unification, and Optional Dependencies

## The Big Picture

Cargo features are additive capability requests. They are not private build
configurations, runtime switches, or a reliable way to choose exactly one of
several mutually exclusive modes.

```
+===========================================================================+
|                         FEATURE FLOW                                      |
+===========================================================================+
| app requests "json" ----+                                                 |
| adapter requests "std" -+--> union for package/version/unit boundary      |
| tests request "trace" --+             |                                   |
|                                       v                                   |
|                     cfg(feature = "...")                                  |
|                     optional dependency edges                             |
|                     dependency feature requests                           |
+===========================================================================+
| Resolver version changes some unification boundaries, not additivity.     |
+===========================================================================+
```

The policy question is not "which features does our manifest request?" It is
"which features does the resolved graph activate for each relevant unit?"

## Declaring Features

```toml
[dependencies]
serde = { version = "1", optional = true, features = ["derive"] }
tracing = { version = "0.1", optional = true }

[features]
default = ["std"]
std = []
json = ["dep:serde"]
telemetry = ["dep:tracing"]
full = ["json", "telemetry"]
```

`dep:serde` makes the optional dependency activation explicit and suppresses
the implicit public feature named `serde`. That lets the crate expose a
capability name such as `json` rather than leaking an implementation choice.

In source:

```rust
#[cfg(feature = "json")]
pub mod json;

#[cfg(not(feature = "std"))]
compile_error!("this target profile requires the `std` feature");
```

Use `compile_error!` sparingly. It can make invalid combinations clear, but a
large mutually-exclusive matrix is usually evidence that separate crates,
backends, or runtime configuration would model the system better.

## Feature Unification

Suppose two dependents request different features:

```text
api  -> codec features ["json"]
cli  -> codec features ["telemetry"]
                 |
                 v
codec is built with ["json", "telemetry"] in the shared unit boundary
```

This is why features should be additive. Enabling one feature must not disable
behavior required by another.

| Rule | Consequence |
|------|-------------|
| Requests are unioned | One edge can activate capability for all users in that unit |
| Default features are ordinary named features | Any edge that leaves defaults on can re-enable them |
| Features are package-ID/unit scoped | Different versions or sources are distinct; resolver context can create separate host/target units |
| Target/host/dev boundaries depend on resolver | Inspect against the workspace resolver and command |
| Public feature names are part of compatibility | Removing or repurposing one can break downstream users |

Resolver v2 and v3 avoid several legacy unifications: target-specific features
for targets not being built, build/proc-macro host dependencies versus normal
target dependencies, and some dev-dependency contexts. Resolver v3 retains those
feature rules and adds MSRV-aware version selection. Describe behavior by
resolver version, not as timeless "Cargo behavior."

## Default Features

Defaults optimize first-use ergonomics, but they are sticky across the graph.

```toml
[dependencies]
codec = { version = "2", default-features = false, features = ["alloc"] }
```

This edge does not guarantee defaults are off globally. Another dependency can
request `codec` without `default-features = false`, and the union enables them.

| Default-feature strategy | Prefer when | Cost |
|--------------------------|-------------|------|
| Small portable default | Most users need it and compatibility is broad | Lowest surprise |
| `std` default, `no_std` opt-out | Desktop/server is primary but embedded is supported | Must continuously test both |
| No defaults | Every capability has meaningful cost or policy impact | More setup and fragmented examples |
| "full" opt-in feature | Convenience for applications/tests | Must not become required by library internals |

Avoid adding a feature to `default` casually. Downstream users who did not name
it can receive new dependencies, native requirements, compile time, behavior,
or MSRV pressure.

## Optional Dependencies and Names

Optional dependencies are conditional graph edges:

```toml
[dependencies]
rustls = { version = "0.23", optional = true, default-features = false }
native-tls = { version = "0.2", optional = true }

[features]
tls-rustls = ["dep:rustls"]
tls-native = ["dep:native-tls"]
```

If exactly one TLS backend must be selected, validate the combinations:

```rust
#[cfg(all(feature = "tls-rustls", feature = "tls-native"))]
compile_error!("select at most one TLS backend");
```

This works, but it fights additivity when two dependents choose different
backends. A stronger architecture exposes backend traits or separate adapter
crates so the application owns the final selection.

## Inspecting Activated Features

```text
cargo tree -e features
cargo tree -e features -i codec
cargo tree -d
cargo metadata --format-version 1
```

Read `cargo tree -e features` as an edge explanation. Exact formatting and edge
display options vary by Cargo version; preserve the command and toolchain in
evidence.

Test a risk-based matrix:

```text
cargo test --no-default-features
cargo test --no-default-features --features std
cargo test --no-default-features --features json
cargo test --all-features
```

`--all-features` is not sufficient when features conflict, targets differ, or
minimal configurations matter.

## Old World -> New World Bridge

Universal build systems distinguish compile-time capabilities from runtime
configuration. Cargo features belong entirely to compile-time graph formation.

| Familiar mechanism | Cargo feature comparison |
|--------------------|--------------------------|
| C/C++ preprocessor symbol | Similar `cfg` effect, but activated through package graph |
| MSBuild property | More global and target-oriented; Cargo features are additive package capabilities |
| NuGet optional package | Optional dependency activated by a feature |
| Runtime app setting | Not equivalent; Cargo feature is compiled in |

For .NET readers, features are not assembly binding redirects or configuration
sections. They are closer to transitive build properties whose requests are
unioned.

## Common Confusion Points

- **"`default-features = false` disables defaults."** It disables them on one
  edge. Another edge can enable them.
- **"Features are mutually exclusive profiles."** Cargo unifies them; model
  alternatives with separate crates or runtime selection where possible.
- **"Unused optional dependency has no effect."** Its feature names and version
  constraints remain part of the manifest contract even when inactive.
- **"Resolver 2 stopped feature unification."** It narrowed selected boundaries;
  additive union remains fundamental.
- **"`--all-features` is the strongest test."** It can be invalid or hide the
  minimal configuration that users actually need.

## Decision Cheat Sheet

| Need | Prefer | Avoid |
|------|--------|-------|
| Add independent capability | Additive named feature | Feature that disables another |
| Hide implementation crate name | Capability feature using `dep:` | Accidental implicit dependency feature |
| Support `no_std` | Small `std`/`alloc` layering with tested defaults | Scattered negative `cfg` logic |
| Select one backend | Application-owned adapter or separate crates | Two transitively unified exclusive features |
| Understand graph activation | `cargo tree -e features` | Reading only the direct manifest |
| Ship a feature contract | Document and test named combinations | Promising every powerset combination |

## Primary Sources

- Cargo features: https://doc.rust-lang.org/cargo/reference/features.html
- Cargo resolver: https://doc.rust-lang.org/cargo/reference/resolver.html
- Cargo tree: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
- Conditional compilation: https://doc.rust-lang.org/reference/conditional-compilation.html

## Related Guides

- Previous: [03-SEMVER-MSRV-DEPENDENCY-CONSTRAINTS-AND-COMPATIBILITY.md](03-SEMVER-MSRV-DEPENDENCY-CONSTRAINTS-AND-COMPATIBILITY.md)
- Next: [05-WORKSPACE-ARCHITECTURE-AND-DEPENDENCY-DIRECTION.md](05-WORKSPACE-ARCHITECTURE-AND-DEPENDENCY-DIRECTION.md)
- Target profiles: [11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md](11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md)
