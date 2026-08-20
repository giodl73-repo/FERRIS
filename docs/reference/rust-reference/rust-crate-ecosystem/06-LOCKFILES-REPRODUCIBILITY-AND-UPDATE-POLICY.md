---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:lockfiles-reproducibility-update-policy
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Lockfiles, Reproducibility, and Update Policy
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md
canonical_path: rust-crate-ecosystem/06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md
backsource_ids: [proof-backfill:rust-crate-ecosystem:06-lockfiles-reproducibility-update-policy]
concepts: [Cargo.lock, reproducible builds, cargo update, dependency policy, locked builds]
root_concepts: [Cargo.lock]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Lockfiles, Reproducibility, and Update Policy

## The Big Picture

`Cargo.toml` declares acceptable graphs. `Cargo.lock` records one graph. A
reproducible dependency process needs both, plus source availability, toolchain
control, native inputs, and an update policy.

```
+===========================================================================+
|                          REPRODUCIBLE RESOLUTION                          |
+===========================================================================+
| Inputs: Cargo.toml ranges + indexes + Cargo version                       |
| Resolution: Cargo resolver                                                |
| Selection: Cargo.lock versions + sources + checksums or git revisions     |
| Verification: --locked -> same selected graph                             |
| Change: cargo update -> reviewed new graph                                |
+===========================================================================+
```

The lockfile makes resolution repeatable. It does not by itself make compilation
bit-for-bit reproducible; compilers, build scripts, environment variables,
native linkers, system libraries, timestamps, and network access can still
matter.

## When to Commit `Cargo.lock`

| Package type | Recommended posture | Why |
|--------------|---------------------|-----|
| Application, service, CLI, firmware | Commit | The repository owns the shipped graph |
| Workspace containing deployable artifacts | Commit one root lockfile | CI and products share exact resolution |
| Reusable library only | Often commit for repository CI; downstream still resolves from manifest | Tests known graph while preserving consumer resolution |
| Published library consumer | Do not assume upstream lock controls your graph | Registry dependencies are resolved in the consuming graph |

Cargo behavior around packaging lockfiles has evolved, especially for packages
with binary targets. The durable rule is simpler: downstream resolution follows
the published manifest and consuming graph; do not use a library's repository
lockfile as its compatibility contract.

## Locked, Offline, and Frozen

| Flag | Meaning |
|------|---------|
| `--locked` | Fail if Cargo would change `Cargo.lock` |
| `--offline` | Avoid network access and use locally available registry/git data |
| `--frozen` | Equivalent to passing both `--locked` and `--offline` |

Examples:

```text
cargo fetch --locked
cargo build --locked
cargo test --workspace --locked
cargo build --frozen
```

Offline operation is constrained by what is already cached or vendored. Without
a suitable lockfile and source cache, `--offline` can fail or choose only among
locally known versions. Treat a successful connected build as insufficient
proof for an air-gapped profile.

Lockfile format is also a Cargo-version compatibility surface. A newer Cargo
may write a lockfile version that an older supported Cargo cannot read. Update
and commit the lockfile with a toolchain compatible with the declared support
floor, or explicitly test that floor after lockfile changes.

## Update Shapes

Do not combine every dependency update into one opaque change.

```
security emergency -> smallest safe graph change -> test -> release
routine renewal    -> bounded batch by layer/risk -> test -> observe
major migration    -> explicit work package -> compatibility/rollback plan
```

| Update | Command shape | Review focus |
|--------|---------------|--------------|
| All compatible packages | `cargo update` | Broad graph and behavior change |
| One package | `cargo update -p name` | Direct and transitive movement |
| Exact selected version | `cargo update -p name --precise X.Y.Z` | Emergency pin or controlled rollback |
| Manifest range change | Edit `Cargo.toml`, then update | New compatibility promise |
| Source patch | `[patch]`, then update | Fork/source custody and removal date |

After any update:

```text
git diff -- Cargo.lock Cargo.toml
cargo tree -d
cargo tree -e features
cargo test --workspace --locked
cargo build --release --locked
```

Add target, `no_std`, native, advisory, license, and packaging checks required by
the supported profile.

## Reading Lockfile Changes

A lockfile diff is not noise.

| Diff pattern | Question |
|--------------|----------|
| One direct crate, many transitive moves | Did defaults or version constraints change? |
| New duplicate major version | Is type duplication or binary cost acceptable? |
| Registry source becomes git | Who approved mutable/source-control custody? |
| Checksum changes at same name/version | Stop; registry immutability assumptions need investigation |
| Native/proc-macro crate appears | Did build-time or platform trust expand? |
| Package disappears | Was functionality internalized, feature-disabled, or accidentally removed? |

Registry checksum handling is source-specific. For crates.io and compatible
registries, Cargo uses checksums recorded through registry metadata and the
lockfile. A matching checksum is integrity evidence, not author identity or code
quality evidence.

## Reproducibility Layers

```
Layer 1: resolution      Cargo.toml + Cargo.lock
Layer 2: source          registry/git/path availability and integrity
Layer 3: Rust toolchain  rustc/Cargo version, target components
Layer 4: host tools      build.rs, proc macros, generators
Layer 5: native tools    compiler, linker, headers, system packages
Layer 6: environment     cfg/env/time/network/filesystem inputs
```

Each lower layer can invalidate a stronger claim. Say "locked dependency graph"
unless the full build has been tested for reproducibility.

## Update Policy Example

```text
Routine:
  cadence: monthly
  batch: one ecosystem layer or <= 10 lockfile packages
  gates: tests, supported targets, audit, license/source policy
  observation: one deployment ring before next batch

Emergency:
  trigger: exploitable advisory or compromised source
  authority: dependency owner + incident commander
  action: update, disable feature, patch, fork, or remove
  deadline: risk-based

Rollback:
  mechanism: revert manifest/lock together
  exception: never roll back into a known unacceptable vulnerability
```

The numerical batch limit is an organizational example, not a Cargo rule.

## Old World -> New World Bridge

| Familiar artifact | Cargo artifact |
|-------------------|----------------|
| Package reference range | `Cargo.toml` |
| Restore lock file | `Cargo.lock` |
| Reproducible restore mode | `--locked` |
| Offline feed/cache | Vendored directory or populated Cargo cache plus `--offline` |
| Central dependency servicing | Planned lockfile update batches |

For NuGet users, `Cargo.lock` resembles `packages.lock.json`, but Cargo commonly
allows multiple incompatible versions and records source/checksum details for
the entire graph.

## Common Confusion Points

- **"Committed lockfile means reproducible binary."** It means reproducible
  resolution under compatible source/index behavior.
- **"`--offline` is more reproducible than `--locked`."** It changes network
  behavior; without complete local sources it can fail or constrain resolution.
- **"Libraries should never commit lockfiles."** They can use one for repository
  CI while still testing compatibility separately.
- **"Routine `cargo update` is harmless within SemVer."** Behavior, MSRV,
  features, build scripts, and transitive graph can change compatibly.
- **"Rollback means restore the old lock."** Restore manifest, lock, source
  overrides, and relevant toolchain/config as one profile.

## Decision Cheat Sheet

| Goal | Use |
|------|-----|
| Repeat CI resolution | Commit lockfile and run `--locked` |
| Prepare disconnected build | `cargo vendor` or managed cache, then test `--frozen` |
| Update one dependency | `cargo update -p` plus graph diff |
| Hold exact emergency version | `--precise` in lockfile with expiry/removal plan |
| Prove bit reproducibility | Add toolchain, host/native, environment, and artifact comparison controls |
| Test library compatibility | Separate lower-bound/MSRV jobs from locked current CI |

## Primary Sources

- Cargo lockfiles: https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
- Cargo update: https://doc.rust-lang.org/cargo/commands/cargo-update.html
- Cargo build cache: https://doc.rust-lang.org/cargo/reference/build-cache.html
- Cargo vendor: https://doc.rust-lang.org/cargo/commands/cargo-vendor.html
- Cargo source replacement: https://doc.rust-lang.org/cargo/reference/source-replacement.html

## Related Guides

- Previous: [05-WORKSPACE-ARCHITECTURE-AND-DEPENDENCY-DIRECTION.md](05-WORKSPACE-ARCHITECTURE-AND-DEPENDENCY-DIRECTION.md)
- Next: [07-CRATES-IO-ALTERNATE-REGISTRIES-SOURCE-REPLACEMENT-AND-VENDORING.md](07-CRATES-IO-ALTERNATE-REGISTRIES-SOURCE-REPLACEMENT-AND-VENDORING.md)
- Renewal/rollback: [15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md](15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md)
