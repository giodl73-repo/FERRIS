---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:packaging-artifacts-releases-upgrades
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Packaging, Artifacts, Releases, and Upgrades
status: source-custody
source_custody: partial
current_path: rust-production-engineering/09-PACKAGING-ARTIFACTS-RELEASES-AND-UPGRADES.md
canonical_path: rust-production-engineering/09-PACKAGING-ARTIFACTS-RELEASES-AND-UPGRADES.md
backsource_ids: [proof-backfill:rust-production-engineering:09-packaging-artifacts-releases-upgrades]
concepts: [packaging, artifacts, releases, upgrades, provenance, reproducible builds, sbom, rollback]
root_concepts: [software delivery]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Packaging, Artifacts, Releases, and Upgrades

## The Big Picture

A release is an identified, immutable set of artifacts plus compatibility and
transition rules. `cargo build --release` creates optimized outputs; it does not
by itself create provenance, a package, an upgrade plan, or a rollback-safe
release.

```
+============================================================================+
|                          RELEASE SUPPLY CHAIN                              |
|                                                                            |
| source + lockfile + toolchain + build policy                               |
|                  |                                                         |
|                  v                                                         |
|          compile/link/package --> verify --> sign/attest --> publish       |
|                  |                                      |                  |
|                  v                                      v                  |
|     binary + symbols + licenses + SBOM             immutable identity      |
|                  |                                      |                  |
|                  +-------------- promote ---------------+                  |
|                                         |                                  |
|                               upgrade / rollback / reconcile               |
+============================================================================+
```

Promote the same bytes between environments. Rebuilding "the same version" for
production creates a new artifact with different evidence.

## Artifact Set

| Artifact | Purpose |
|---|---|
| Executable/library | runnable product |
| Debug symbols | postmortem symbolication |
| Manifest/checksums | identity and integrity |
| Dependency/license inventory | legal and vulnerability response |
| SBOM/provenance attestation | supply-chain traceability |
| Configuration schema/sample | deployment contract |
| Migrations | state transition |
| Release notes | operator/user-visible change |

Store symbols with access controls and retention at least as long as the
corresponding binaries can run or appear in dumps.

## Executable Build Identity

```toml
# Cargo.toml
[package]
name = "release-identity"
version = "0.1.0"
edition = "2021"

[profile.release]
lto = "thin"
codegen-units = 1
strip = "none"
```

```rust
fn main() {
    let revision = option_env!("BUILD_REVISION").unwrap_or("development");
    let source_date_epoch = option_env!("SOURCE_DATE_EPOCH").unwrap_or("unspecified");
    println!(
        "name={} version={} revision={} source_date_epoch={}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        revision,
        source_date_epoch
    );
}
```

Build in a clean checkout with a pinned toolchain:

```bash
BUILD_REVISION="$(git rev-parse HEAD)" \
SOURCE_DATE_EPOCH="$(git show -s --format=%ct HEAD)" \
cargo build --release --locked
./target/release/release-identity
sha256sum target/release/release-identity
```

PowerShell uses:

```powershell
$env:BUILD_REVISION = git rev-parse HEAD
$env:SOURCE_DATE_EPOCH = git show -s --format=%ct HEAD
cargo build --release --locked
.\target\release\release-identity.exe
Get-FileHash .\target\release\release-identity.exe -Algorithm SHA256
```

The epoch is a declared build input derived from the source revision rather
than the wall clock. Reusing the same value avoids making every rebuild differ
solely because it ran later. Exact reproducibility still depends on compiler,
linker, target, paths, native dependencies, archive metadata, and all other
declared or ambient inputs; embedding metadata does not prove reproducibility.

## Target and Linking Policy

| Choice | Benefit | Operational consequence |
|---|---|---|
| Target-native dynamic linking | smaller artifact; host integration | compatible shared libraries required |
| Static linking where supported | simpler deployment envelope | larger binary; libc/licensing/patch posture |
| Linux GNU target | broad glibc ecosystem | minimum glibc compatibility matters |
| Linux musl target | static-friendly deployment | DNS/performance/native dependency differences |
| Windows MSVC target | native Windows ABI/toolchain | MSVC runtime and PDB handling |

Rust has no stable Rust-to-Rust dynamic ABI. Plugin or shared-library boundaries
need a stable C ABI, a process protocol, WebAssembly contract, or tightly pinned
toolchain agreement.

## Version and Compatibility Surfaces

```
artifact version
  +--> command/config schema
  +--> network protocol/API
  +--> persisted data/schema
  +--> message/event schema
  +--> platform requirements
```

Rollback is safe only when all these surfaces remain backward compatible. A
binary rollback after an irreversible data migration is not a rollback plan.
Prefer roll-forward when data has crossed a one-way boundary, and make that
decision explicit before release.

## Upgrades

Use staged exposure and overlapping compatibility:

1. Publish immutable artifact and evidence.
2. Verify in an environment representative of production.
3. Expand schemas/protocols if needed.
4. Expose a small cohort.
5. Compare user-impact and saturation signals.
6. Increase exposure or halt.
7. Contract old compatibility only after the rollback window closes.

Self-update logic increases privilege and recovery complexity. Prefer the
platform package manager or supervisor unless offline/desktop constraints make
self-update necessary.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library/build | Cargo profiles, feature policy, build metadata, SBOM/sign tools |
| Runtime | rarely involved except runtime-linked assets and compatibility |
| Platform | archive/package/container registry, signature verification, rollout |

A container image is one packaging format. Native packages, archives, firmware
images, and Windows installers can carry the same release contract.

## Old World -> New World Bridge

The universal bridge is from **version labels** to **content-addressed evidence**.
The trustworthy deployment unit is the verified artifact digest plus provenance,
not a mutable tag or a human-readable version alone.

MSI/MSIX, NuGet packages, and signed Windows binaries are familiar packaging
models. OCI images and attestations extend the same ideas to filesystem bundles
and multi-platform manifests; Azure Container Registry is one possible
repository, not the architecture itself.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Native archive/package | host integration and small deployment surface matter |
| Container image | a filesystem/process envelope and registry workflow help |
| Static linking | target supports it and operational/legal tradeoffs are accepted |
| Separate symbols | production binary should be small/stripped but dumps need analysis |
| Embedded revision | running process must map to exact source/evidence |
| SBOM + provenance | supply-chain and incident response require dependency/build trace |
| Rollback | data/protocol compatibility remains reversible |
| Roll-forward | irreversible state transition has already occurred |

## Common Confusion Points

- **`Cargo.lock` is necessary, not sufficient, for reproducible bytes.**
- **A tag can move; a digest should not.**
- **Static binaries still depend on kernel and platform contracts.**
- **Stripping symbols without preserving them destroys future crash evidence.**
- **Semantic versioning of a crate does not define service data migration
  compatibility.**

## Primary Sources

- Cargo profiles: https://doc.rust-lang.org/cargo/reference/profiles.html
- Cargo lockfiles: https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
- Rust platform support: https://doc.rust-lang.org/rustc/platform-support.html
- SLSA specification: https://slsa.dev/spec/
- SPDX: https://spdx.dev/
- CycloneDX: https://cyclonedx.org/

## Related Guides

- Previous: [08-PERSISTENCE-TRANSACTIONS-AND-DATA-ACCESS.md](08-PERSISTENCE-TRANSACTIONS-AND-DATA-ACCESS.md)
- Next: [10-CONTAINERS-KUBERNETES-SYSTEMD-AND-WINDOWS-SERVICES.md](10-CONTAINERS-KUBERNETES-SYSTEMD-AND-WINDOWS-SERVICES.md)
- Cargo artifact internals: [../rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md](../rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md)
- Promotion: [11-CI-CD-AND-PROMOTION.md](11-CI-CD-AND-PROMOTION.md)
