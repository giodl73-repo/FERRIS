---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:registries-source-replacement-vendoring
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: crates.io, Alternate Registries, Source Replacement, and Vendoring
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/07-CRATES-IO-ALTERNATE-REGISTRIES-SOURCE-REPLACEMENT-AND-VENDORING.md
canonical_path: rust-crate-ecosystem/07-CRATES-IO-ALTERNATE-REGISTRIES-SOURCE-REPLACEMENT-AND-VENDORING.md
backsource_ids: [proof-backfill:rust-crate-ecosystem:07-registries-source-replacement-vendoring]
concepts: [crates.io, alternate registry, source replacement, cargo vendor, registry policy]
root_concepts: [cargo registries]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# crates.io, Alternate Registries, Source Replacement, and Vendoring

## The Big Picture

Cargo package identity includes source identity, while configuration can
redirect retrieval for an existing source. An alternate registry publishes a
distinct package universe. Source replacement mirrors an existing source.
Vendoring materializes selected sources into a directory. `[patch]` changes
which package revision participates in resolution.

```
+===========================================================================+
|                         CARGO SOURCE CHOICES                              |
+===========================================================================+
|                                                                           |
| Cargo.toml dependency                                                     |
|      |                                                                    |
|      +-> crates.io registry --------+                                     |
|      +-> alternate registry --------+--> resolver --> Cargo.lock           |
|      +-> git repository ------------+                                     |
|      +-> path dependency -----------+                                     |
|                                                                           |
| Operational overlays:                                                     |
|   source replacement = mirror an existing source                          |
|   cargo vendor       = directory source for locked packages               |
|   [patch]            = replace package candidate during resolution         |
+===========================================================================+
```

Do not describe these as interchangeable "private feeds." Their identity,
publication, authentication, and resolution semantics differ.

## crates.io and Registry Boundaries

crates.io is Rust's default public registry service. Cargo consumes registry
index metadata and crate archives. Registry metadata and checksums support
resolution and integrity checking, but crates.io publication is not a product
security, maintenance, or license approval.

| Registry provides | Registry does not universally provide |
|-------------------|----------------------------------------|
| Package names/versions and dependency metadata | Product-specific suitability |
| Archive distribution | Complete provenance of every generated/source input |
| Checksums through registry protocol | Proof of absence of malicious behavior |
| Owners/yanks and publication workflow | Guaranteed maintainer continuity |

Bound claims to the registry and Cargo versions in use. Registry protocol
support, authentication, sparse indexes, and credential-provider behavior have
evolved.

## Alternate Registries

Configure a named registry:

```toml
# .cargo/config.toml
[registries.corp]
index = "sparse+https://packages.example.invalid/cargo/index/"
```

Use it in a manifest:

```toml
[dependencies]
internal-protocol = { version = "2.3", registry = "corp" }
```

Authentication belongs in Cargo's credential mechanisms or the registry's
supported provider flow, not committed plaintext configuration.

An alternate registry creates a source identity distinct from crates.io. The
same package name/version from two registries is not automatically the same
package. A package published to crates.io cannot depend on a package from an
alternate registry. Other registries define their own publication policy, so
validate the target registry and every dependency that remains in the published
manifest.

## Source Replacement

Source replacement redirects one source to another expected to contain the same
package set, such as an internal mirror or vendor directory.

```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "company-mirror"

[source.company-mirror]
registry = "sparse+https://packages.example.invalid/crates-io/index/"
```

Use source replacement for transport/custody of the same packages. Use
`[patch]` when intentionally substituting a different revision or package
candidate.

Cargo expects a replacement source to stand in for the original source, not to
introduce additional package identities. Mirrored registry packages must match
the original checksums. If the content intentionally differs, it is a patch or
fork and should be represented as such.

```
source replacement:
  crates.io serde 1.0.X -> mirrored crates.io serde 1.0.X

[patch]:
  crates.io serde candidate -> reviewed fork/revision for this graph
```

Cargo applies constraints to replacement sources to preserve source identity
expectations. Test the exact Cargo version and mirror implementation rather than
assuming every registry server implements every protocol detail.

## Vendoring

Create a vendor directory from the resolved graph:

```text
cargo vendor --locked vendor
```

Cargo prints configuration similar to:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

Then prove disconnected use:

```text
cargo build --frozen
cargo test --workspace --frozen
```

| Vendoring buys | Vendoring does not buy |
|----------------|------------------------|
| Local availability of selected source | Automatic legal approval |
| Inspectable source snapshot | Automatic update/advisory handling |
| Reduced dependency on registry uptime | Bit-reproducible native/build outputs |
| Easier air-gapped transfer | Upstream maintainer continuity |

Choose whether `vendor/` is committed, generated into a release bundle, or held
in an artifact store. Each choice changes repository size, review mechanics, and
custody. Record the generation command and lockfile.

## Git and Path Dependencies

```toml
[dependencies]
parser = { git = "https://github.com/example/parser", rev = "0123456789abcdef0123456789abcdef01234567" }
domain = { path = "../domain" }
```

Prefer an immutable git `rev` over a floating branch for reproducible product
graphs. The lockfile records a concrete revision, but the remote still must
remain available unless mirrored or vendored.

For a package intended for registry publication, pair an internal path with a
version where appropriate:

```toml
domain = { path = "../domain", version = "0.4.0" }
```

The path supports workspace development; the version supports registry
resolution during publication.

## Enterprise Distribution Pattern

Universal policy first:

```
approved upstream sources
          |
          v
ingest/mirror with identity and audit trail
          |
          v
policy scan + supported-profile approval
          |
          v
build from lockfile through controlled source
```

An Azure Artifacts, Artifactory, or other enterprise registry can implement
parts of this pattern. The product choice is supplemental; the portable design
is source identity, immutable records, least-privilege publication, documented
ingest, and tested recovery.

## Old World -> New World Bridge

| Familiar package-distribution concept | Cargo source mechanism |
|---------------------------------------|------------------------|
| Public package feed | crates.io |
| Private package feed | Alternate registry |
| Transparent repository mirror | Source replacement |
| Checked-in third-party source drop | `cargo vendor` directory source |
| Temporary central override | `[patch]` scoped to the consuming graph |
| Project reference | Path dependency |

The universal distinction is between changing **where identical package
content is obtained** and changing **which package content is selected**.
Source replacement serves the first purpose; `[patch]` serves the second.

## Common Confusion Points

- **"Mirror and alternate registry are the same."** A mirror replaces transport
  for an existing source; an alternate registry is a distinct source.
- **"Vendored means reviewed."** It means copied into custody. Review is another
  process.
- **"Checksums prove authorship."** They prove byte agreement with registry
  metadata under that source's protocol.
- **"`[patch]` changes the published package."** It changes resolution for the
  graph where the patch applies.
- **"Private registry prevents dependency confusion."** It can reduce exposure,
  but naming, source rules, credentials, and publish permissions must also be
  designed correctly.

## Decision Cheat Sheet

| Need | Use | Required control |
|------|-----|------------------|
| Consume public Rust packages normally | crates.io | Lock, policy scan, renewal |
| Publish organization-only packages | Alternate registry | Auth, naming, retention, recovery |
| Mirror public packages internally | Source replacement | Identity-preserving ingest and sync policy |
| Build disconnected | `cargo vendor` plus `--frozen` | Bundle provenance and update process |
| Test unreleased upstream fix | Git `rev` or `[patch]` | Immutable revision and expiry |
| Develop related workspace packages | Path dependency | Version companion if publishing |

## Primary Sources

- Cargo registries: https://doc.rust-lang.org/cargo/reference/registries.html
- Registry authentication: https://doc.rust-lang.org/cargo/reference/registry-authentication.html
- Source replacement: https://doc.rust-lang.org/cargo/reference/source-replacement.html
- Overriding dependencies: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html
- Cargo vendor: https://doc.rust-lang.org/cargo/commands/cargo-vendor.html

## Related Guides

- Previous: [06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md](06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md)
- Next: [08-CARGO-AUDIT-CARGO-DENY-CARGO-VET-ADVISORIES-AND-SUPPLY-CHAIN-CONTROLS.md](08-CARGO-AUDIT-CARGO-DENY-CARGO-VET-ADVISORIES-AND-SUPPLY-CHAIN-CONTROLS.md)
- Publishing: [14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md](14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md)
