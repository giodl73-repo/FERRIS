---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:supported-profiles-renewal-removal
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Supported Profiles, Renewal, Rollback, and Removal
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md
canonical_path: rust-crate-ecosystem/15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md
backsource_ids: [mdloom-backfill:rust-crate-ecosystem:15-supported-profiles-renewal-removal]
concepts: [dependency support profile, renewal, rollback, dependency removal, crate lifecycle]
root_concepts: [dependency lifecycle]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Supported Profiles, Renewal, Rollback, and Removal

## The Big Picture

Adoption is the start of a lifecycle. A supported profile names the exact
conditions under which the organization will build, test, ship, update, and
respond to a dependency.

```
+===========================================================================+
|                        DEPENDENCY SUPPORT LIFECYCLE                       |
+===========================================================================+
| evaluate -> approve profile -> integrate -> observe -> renew              |
| profile: versions + features + source + MSRV + targets + owners           |
| renewal: continue / constrain / replace / remove                          |
| incident: rollback / patch / fork / replace / remove                      |
+===========================================================================+
```

This "profile" is a governance profile, not Cargo's `[profile.release]` build
optimization table.

## Supported Profile Record

```text
Dependency: example-crate
Purpose/boundary: HTTP header parsing behind adapter-http
Manifest requirement: ^1.4
Locked version: 1.4.7
Source: crates.io
Resolver: 3
Features: default-features=false; features=["std"]
MSRV: Rust 1.82
Targets: x86_64 Linux GNU; x86_64 Windows MSVC
Build-time code: no build.rs; no proc macro; no native code
License/provenance: MIT OR Apache-2.0; evidence path ...
Security policy: no matching RustSec advisory as of date; deny/vet criteria ...
Internal owner: Platform Foundations
Upstream posture: active; fork feasibility medium
Renewal date: 2027-02-11
Rollback/removal plan: lock revert or internal parser adapter replacement
Evidence: commands, CI run, scorecard, decision link
```

The record should be reviewable without reconstructing intent from Git history.
Store machine-enforceable fields in manifests/config and narrative decisions in
a durable policy record.

## Profile Layers

| Layer | Must name |
|-------|-----------|
| Identity | Package name, source, version requirement, exact locked version |
| Capabilities | Default and explicit features |
| Compatibility | Resolver, Rust/Cargo floor, editions where relevant, targets |
| Execution | Build scripts, proc macros, generators, native/system inputs |
| Policy | License, source, advisory, audit criteria, exceptions |
| Ownership | Internal owner, upstream relationship, escalation |
| Lifecycle | Renewal cadence, rollback, replacement/removal trigger |

A direct dependency can have more than one supported profile, for example a
server profile with `std`/TLS and an embedded profile with `alloc`. Approve each
combination explicitly.

## Renewal

Renewal asks whether the dependency remains the right choice, not only whether a
new version exists.

```
renewal input
  +-> current lock/manifest/profile diff
  +-> upstream releases/governance/security
  +-> target/MSRV/feature matrix
  +-> graph/license/source/audit results
  +-> product usage and exit cost
        |
        v
continue / update / constrain / replace / remove
```

| Cadence driver | Example posture |
|----------------|-----------------|
| Security-critical parser/crypto/network edge | Continuous advisories plus quarterly full renewal |
| Stable leaf utility | Semiannual or annual renewal |
| Fast-moving framework/runtime | Monthly/quarterly update planning |
| Internal fork | Frequent upstream-diff and custody review |
| Deprecated/temporary patch | Short expiry with explicit removal issue |

These cadences are examples. Tie them to product risk and release behavior.

## Update and Observation

Bound updates so failures are attributable:

```text
1. open renewal work item with profile and target version
2. update manifest/lock in a bounded batch
3. inspect duplicate/features/source/build-time changes
4. run profile matrix and policy tools
5. deploy to an observation ring where applicable
6. close with new evidence and next renewal date
```

Do not silently change the support profile because a transitive update happened
to pass tests. If MSRV, source, native model, or feature set changes, review the
profile.

## Rollback

Rollback is a prepared transition to a previously acceptable state.

| Rollback layer | Artifact |
|----------------|----------|
| Manifest | Version requirements/features/patches |
| Resolution | `Cargo.lock` |
| Source | Registry mirror/vendor/git revision |
| Toolchain | Rust/Cargo and target/native tool versions |
| Product | Data/schema/protocol compatibility |
| Deployment | Previous releasable artifact and configuration |

```text
git revert <dependency-update-change>
cargo test --workspace --locked
cargo build --release --locked
```

The command shape is illustrative; verify the actual reverted change and
profile. Never roll back into a known unacceptable vulnerability merely because
it is operationally familiar. Prefer forward fix, feature disablement, patch,
or removal when the prior state is unsafe.

## Removal

Dependency removal is an architecture exercise:

```
inventory use sites
      |
      v
introduce/confirm internal boundary
      |
      +-> replace implementation
      +-> internalize small behavior
      +-> delete unused capability
      |
      v
remove features/manifest edge
      |
      v
update lock/policy/notices/SBOM/docs
      |
      v
prove no package/source/build artifact remains
```

Evidence:

```text
cargo tree -i example-crate
cargo tree -i example-crate --target all -e all --all-features
cargo tree -d
cargo metadata --format-version 1
cargo test --workspace --locked
```

The `--target all --all-features` command is a broad inventory aid, not a valid
build profile when features conflict. `cargo tree -i` may show no path only for
the selected target/feature/edge context. Repeat under each supported profile,
use an unambiguous package ID when multiple versions exist, and check vendor
directories, generated code, containers, system packages, notices, and release
artifacts.

## Removal Triggers

| Trigger | Response |
|---------|----------|
| Unacceptable vulnerability | Update, constrain, patch, fork, or remove under incident timing |
| License/source policy change | Hold new releases and establish compliant path |
| Upstream abandonment | Assess stability, fork readiness, alternatives |
| MSRV/target regression | Constrain version or replace |
| Excess graph/build cost | Feature reduction, adapter boundary, replacement |
| Duplicated capability | Consolidate after migration analysis |
| Product feature retirement | Delete capability and dependency |

## Policy-as-Code Example

```toml
# Illustrative repository-owned support metadata.
# Cargo ignores this file; an internal checker may validate it.

[[dependency]]
name = "example-crate"
source = "crates-io"
requirement = "^1.4"
features = ["std"]
default_features = false
msrv = "1.82"
targets = ["x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc"]
owner = "platform-foundations"
renew_by = "2027-02-11"
```

If an organization creates such a checker, version its schema and keep Cargo
manifests/lockfiles authoritative for actual resolution.

## Old World -> New World Bridge

| Familiar lifecycle practice | Cargo ecosystem form |
|-----------------------------|----------------------|
| Supported configuration matrix | Version/feature/MSRV/target/source profile |
| Vendor renewal | Upstream and dependency profile renewal |
| Patch Tuesday/service train | Bounded Cargo update cadence |
| Known-good rollback package | Manifest, lock, source, toolchain, artifact set |
| Product/component retirement | Remove graph edge, policy, notices, artifacts |

Enterprise portfolio tooling can aggregate profiles across repositories. The
repository still needs executable, local evidence because central status can
lag the graph that actually builds.

## Common Confusion Points

- **"Approved crate means all versions/features are approved."** Approval must
  be profile-specific.
- **"Renewal is `cargo update`."** Update is one possible renewal outcome.
- **"Reverting `Cargo.lock` is a complete rollback."** Manifest, source,
  toolchain, native inputs, data, and deployment may also have changed.
- **"No direct dependency means removed."** A transitive, build, target, or
  generated dependency may remain.
- **"Support profile equals Cargo build profile."** They are unrelated terms.

## Decision Cheat Sheet

| Lifecycle question | Action |
|--------------------|--------|
| What exactly do we support? | Create one record per version/feature/target/source profile |
| When do we reconsider? | Set risk-based renewal date and continuous event triggers |
| How do we update safely? | Bounded batch, graph diff, full profile gates, observation |
| How do we recover? | Rehearse manifest/lock/source/toolchain/artifact rollback |
| When upstream fails? | Constrain, patch, fork, replace, or remove with named owner |
| How do we prove removal? | Reverse-tree and metadata checks across every supported profile |

## Primary Sources

- Cargo manifests: https://doc.rust-lang.org/cargo/reference/manifest.html
- Cargo lockfiles: https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
- Cargo tree: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
- Cargo update: https://doc.rust-lang.org/cargo/commands/cargo-update.html
- Cargo metadata: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html

## Related Guides

- Previous: [14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md](14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md)
- Start over: [00-OVERVIEW.md](00-OVERVIEW.md)
- Evaluation record: [02-EVALUATION-SCORECARDS-AND-EVIDENCE.md](02-EVALUATION-SCORECARDS-AND-EVIDENCE.md)
- Lock/update mechanics: [06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md](06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md)
