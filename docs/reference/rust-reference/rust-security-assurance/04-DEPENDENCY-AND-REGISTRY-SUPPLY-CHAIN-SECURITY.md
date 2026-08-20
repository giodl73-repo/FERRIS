---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:dependency-and-registry-supply-chain-security
kind: guide
module: rust-security-assurance
section: security-engineering
title: Dependency and Registry Supply-Chain Security
status: source-custody
source_custody: partial
current_path: rust-security-assurance/04-DEPENDENCY-AND-REGISTRY-SUPPLY-CHAIN-SECURITY.md
canonical_path: rust-security-assurance/04-DEPENDENCY-AND-REGISTRY-SUPPLY-CHAIN-SECURITY.md
backsource_ids: [proof-backfill:rust-security-assurance:04-dependency-and-registry-supply-chain-security]
concepts: [cargo dependencies, crates.io, registries, lockfiles, advisories, supply chain]
root_concepts: [software supply chain]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Dependency and Registry Supply-Chain Security

Cargo makes the dependency graph visible and repeatable, but visibility is not
trust. A resolved crate can contain malicious logic, vulnerable unsafe code,
build-time execution, or an unexpectedly activated feature. Supply-chain
assurance controls **source identity, resolution, review, execution, and
response** as separate stages.

## The Big Picture

```
+============================================================================+
|                         CARGO SUPPLY CHAIN                                 |
+============================================================================+
| source intent                                                              |
| Cargo.toml constraints -> registry/git/path identity -> resolver           |
|         |                   |                         |                    |
|         v                   v                         v                    |
| policy review          publisher/source trust       Cargo.lock selection   |
+----------------------------------------------------------------------------+
| fetched source -> build.rs/proc macro -> rustc/linker -> artifact          |
| checksums          host code execution      compiler/native trust          |
+----------------------------------------------------------------------------+
| artifact -> SBOM/provenance/signature -> deployment -> advisory response   |
+============================================================================+
```

Each arrow needs evidence. `Cargo.lock` answers "which package source and
version did Cargo select?" It does not answer "should we trust this code?"

## Source Types Have Different Identities

| Dependency source | Identity recorded by Cargo | Principal risk |
|-------------------|----------------------------|----------------|
| Registry package | registry, name, version, checksum | publisher/account compromise; malicious or vulnerable release |
| Git dependency | repository URL and selected commit in lockfile | mutable branch/tag intent; repository compromise; submodules/build behavior |
| Path dependency | local path and current files | workspace/change-control integrity; no registry checksum |
| Patched/replaced source | effective source after `[patch]`/source config | reviewers inspect manifest intent but miss actual selected source |

For applications and binaries, commit `Cargo.lock` and build with `--locked`.
Library packages also benefit from testing their lockfile, but consumers resolve
their own compatible graph; do not imply the library's lockfile constrains
downstream users.

## Inspect Resolution, Features, and Targets

From the workspace root, using the approved Cargo toolchain:

```text
cargo metadata --locked --format-version 1
cargo tree --locked --workspace --all-features
cargo tree --locked --workspace -e features
cargo tree --locked --workspace --duplicates
```

These commands are read-only with respect to the lockfile; `--locked` fails if
resolution would change it. `--all-features` is a review lens, not necessarily
the release configuration. Preserve evidence for each supported
target/profile/feature set because optional dependencies can carry security
behavior.

```
manifest request
      |
      v
version resolution + source selection
      |
      v
feature unification per Cargo rules
      |
      v
target-specific unit graph
      |
      v
runtime code + host-side build code
```

## Evaluate More Than Popularity

| Review dimension | Questions |
|------------------|-----------|
| Necessity | Can the feature be implemented with std or an existing dependency? |
| Privilege | Does it parse hostile input, handle secrets, run at build time, or contain unsafe/FFI? |
| Maintenance | Are releases, ownership, security policy, and issue response coherent? |
| Source | Is the selected registry/repository expected and protected by project policy? |
| Scope | Which targets/features activate it? Is default-feature behavior intended? |
| Change size | Can updates be reviewed as bounded deltas rather than graph-wide churn? |
| Exit | Is replacement possible if the crate becomes abandoned or compromised? |

Download count, star count, and brand recognition are weak signals. They may
help triage; they do not establish integrity.

## Advisory and Policy Tools

`cargo-audit` checks the resolved graph against the RustSec advisory database.
`cargo-deny` can enforce advisory, license, duplicate-version, and source
policies. Both are third-party Cargo subcommands: pin their versions and validate
their configuration in CI rather than assuming stable output across upgrades.

With approved, exactly versioned tool binaries already installed:

```text
cargo audit
cargo deny check advisories bans licenses sources
```

Build/install those tools in a controlled tool job using an exact approved
version and preserved binary digest; `cargo install --locked` alone still
selects the newest matching package unless `--version` is also supplied. An
empty result means "no matching record under this database/configuration at
this time," not "no vulnerability exists." See
[13](13-ADVISORIES-VULNERABILITY-RESPONSE-PATCHING-AND-DISCLOSURE.md).

## Vendoring and Network Isolation

`cargo vendor` can create a local source tree and configuration for controlled
or offline builds:

```text
cargo vendor --locked vendor
# Merge the emitted source-replacement snippet into .cargo/config.toml.
cargo build --locked --offline
```

`cargo vendor` prints the source-replacement configuration; merely creating the
`vendor` directory does not make Cargo consume it. The build succeeds only when
every required source/index artifact is available through the configured
local/cache sources. Vendoring improves availability and reviewability; it also
creates a patching obligation. Preserve upstream identity and checksums so the
vendor tree does not become an untraceable fork.

## Update Discipline

```
proposed update
    |
    +--> lockfile/source diff
    +--> feature and target diff
    +--> build-time/unsafe/native delta
    +--> advisory and policy checks
    +--> tests/fuzz/evidence affected by change
    +--> rollback and response owner
```

Prefer small, frequent, reviewable updates over rare dependency avalanches.
Security fixes may require urgent exceptions to normal batching, but still
record source identity, applicability, and validation.

## Old World -> New World Bridge

| Established ecosystem | Cargo equivalent | Security difference |
|-----------------------|------------------|---------------------|
| NuGet/npm lockfile | `Cargo.lock` | Same reproducibility role; feature/target graphs still need inspection |
| Private package feed | alternate Cargo registry/source | Source policy and credential isolation still required |
| Package signature/checksum | registry checksum plus external provenance/signing | Integrity signal is not publisher trust |
| Dependabot/Renovate update | automated Cargo lockfile PR | Review selected source, features, build code, and evidence |
| Software composition analysis | RustSec/cargo-deny plus complete SBOM | Advisory matching is only one assurance input |

GitHub dependency review, Dependabot, and Advanced Security can supplement this
workflow for teams using GitHub. Azure Artifacts or another private registry can
enforce distribution policy. Neither vendor ecosystem replaces source review,
build isolation, or incident readiness.

## Common Confusion Points

- **"`Cargo.lock` pins everything."** It pins resolved package/source identity
  for that graph; target tools, native libraries, compiler, and path contents
  need separate control.
- **"Registry checksum means safe."** It detects content mismatch relative to
  registry metadata, not malicious intent.
- **"No RustSec advisory means no issue."** Databases are incomplete and
  applicability is contextual.
- **"A private registry is inherently trusted."** It changes governance and
  access paths; compromised publishers or administrators remain threats.
- **"Vendoring freezes risk."** It freezes source until you update it, including
  vulnerabilities.
- **"Default features are harmless."** They can activate native code, network
  stacks, parsers, or build dependencies.

## Decision Cheat Sheet

| Situation | Do |
|-----------|----|
| Application/binary release | Commit lockfile and build with `--locked` |
| High-privilege new crate | Perform maintainer/source/unsafe/build-time review before adoption |
| Git dependency | Pin through lockfile to a reviewed commit; avoid mutable-reference assumptions |
| Offline/restricted build | Vendor sources, preserve identity, and test `--offline` |
| Advisory scanner is green | Treat as one dated signal, not a security verdict |
| Large dependency update | Split where possible; inspect graph/features/targets and regenerate evidence |
| Private registry | Apply identity, access, immutability, backup, and response controls |

## Primary Sources

- Cargo Book, Specifying Dependencies:
  https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
- Cargo Book, Source Replacement:
  https://doc.rust-lang.org/cargo/reference/source-replacement.html
- Cargo Lockfiles: https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
- crates.io policies: https://crates.io/policies
- RustSec Advisory Database: https://rustsec.org/
- cargo-audit: https://github.com/rustsec/rustsec/tree/main/cargo-audit
- cargo-deny: https://embarkstudios.github.io/cargo-deny/

## Related Guides

- Previous: [03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md](03-RUST-MEMORY-MODEL-UNDEFINED-BEHAVIOR-AND-VALIDITY.md)
- Next: [05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md](05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md)
- Provenance and SBOMs: [12-ARTIFACT-PROVENANCE-SBOMS-SIGNING-AND-REPRODUCIBLE-EVIDENCE.md](12-ARTIFACT-PROVENANCE-SBOMS-SIGNING-AND-REPRODUCIBLE-EVIDENCE.md)
