---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:licensing-provenance-attribution
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Licensing, Provenance, and Attribution
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/09-LICENSING-PROVENANCE-AND-ATTRIBUTION.md
canonical_path: rust-crate-ecosystem/09-LICENSING-PROVENANCE-AND-ATTRIBUTION.md
backsource_ids: [proof-backfill:rust-crate-ecosystem:09-licensing-provenance-attribution]
concepts: [rust licensing, SPDX, provenance, attribution, cargo metadata]
root_concepts: [dependency licensing]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Licensing, Provenance, and Attribution

## The Big Picture

License metadata, source provenance, and attribution are related but distinct.
The manifest states a license claim. The distributed source and notices provide
evidence. Product obligations depend on how the software is used and shipped.

```
+===========================================================================+
|                        LICENSE AND PROVENANCE CHAIN                       |
+===========================================================================+
| manifest claim -> registry/archive/source -> resolved graph review        |
| resolved graph -> source/license files -> policy/legal result             |
| resolved graph -> artifact inventory -> notices/attribution bundle        |
|                                                                           |
| Metadata is an assertion; preserve evidence and resolve conflicts.        |
+===========================================================================+
```

This guide is engineering governance, not legal advice. Organizations should
route ambiguous or high-impact terms through qualified counsel.

## Manifest License Fields

Preferred SPDX expression:

```toml
[package]
license = "MIT OR Apache-2.0"
```

Use `OR` when recipients may choose either license. Use `AND` when all stated
licenses apply. The old slash form is ambiguous and should not be used as an
SPDX expression.

For a non-standard license text:

```toml
[package]
license-file = "LICENSE.txt"
```

Cargo's `license` and `license-file` fields are mutually exclusive: use the SPDX
expression when one applies, otherwise point to the distributed license text.
Cargo metadata is useful but not dispositive. Inspect the packaged files:

```text
cargo package --list
cargo metadata --format-version 1
```

## Evidence Hierarchy

| Evidence | Use | Caveat |
|----------|-----|--------|
| `Cargo.toml` SPDX expression | Automated policy and discovery | Publisher assertion |
| License files in crate archive | Distributed terms | May conflict with metadata or omit generated inputs |
| Repository license/history | Provenance context | Repository head may differ from published crate |
| Source headers/notices | File-level obligations | Inconsistent practices require interpretation |
| Upstream clarification | Resolve ambiguity | Preserve durable written record |
| Legal review | Interpret obligations for product use | Scope-specific, not a universal engineering rule |

If metadata and distributed files disagree, stop treating the license as clean.
Record the conflict and seek clarification.

## Graph-Level Review

The direct crate's license is only one node.

```
product
  +-> direct Rust crate
  |     +-> transitive Rust crates
  |     +-> generated source/templates
  |     +-> vendored C/C++/data
  |     +-> build tools
  +-> system/shared native libraries
```

License tools generally see Cargo packages better than arbitrary generated data,
firmware blobs, fonts, schemas, or system packages. Add those inventories
explicitly.

Example policy workflow:

```text
cargo deny check licenses
cargo metadata --format-version 1
cargo package --list
```

`cargo-deny` license detection uses manifest and source evidence under its
configured confidence and exceptions. Its result is policy evidence, not legal
interpretation. Generate configuration with the pinned tool version and review
each exception.

## Provenance Questions

| Question | Evidence |
|----------|----------|
| Which registry/git/path supplied the package? | `Cargo.lock`, `cargo metadata` |
| Which exact source archive/revision was built? | Lockfile checksum or git revision plus retained source |
| Does repository tag match published archive? | Reproducible comparison or upstream provenance |
| Who may publish new versions? | Registry owners and governance documentation |
| Does generated code include third-party material? | Generator inputs/templates and output headers |
| Does a vendored native library have separate terms? | Vendor directory and upstream notices |

A registry checksum is strong evidence of archive identity relative to registry
metadata. It is not a complete provenance chain from author workstation to
publication.

## Attribution Output

Define the product's notice format before release.

```text
Component: example-crate 1.4.2
Source: crates.io
License: MIT OR Apache-2.0
Copyright/notice: [preserved upstream text]
Source URL: https://...
Modifications: none / described
Included in: server binary / source distribution / build tooling only
```

Distinguish shipped runtime code from build-only tools, but do not assume
build-only automatically means no obligation. License terms and distribution
model control.

| Distribution form | Review emphasis |
|-------------------|-----------------|
| Hosted service | Source/network-copyleft triggers and internal policy |
| Binary/firmware | Notices, relinking/source obligations, native components |
| SDK/library | Downstream license clarity and re-exported API |
| Source distribution/vendor bundle | Complete license files and modifications |
| Container image | OS packages plus Rust graph and copied assets |

## Internal Crates

Internal code still needs explicit ownership and license posture if it may be
published, shared across legal entities, or open-sourced later.

```toml
[workspace.package]
license = "MIT OR Apache-2.0"
repository = "https://github.com/example/project"
```

Member inheritance reduces drift, but generated, copied, or imported files can
carry different terms. Keep third-party code out of generic internal headers and
track its provenance at entry.

## Old World -> New World Bridge

| Familiar software-composition task | Cargo ecosystem task |
|------------------------------------|----------------------|
| Third-party notices from binaries/packages | Generate notices from lockfile plus non-Cargo inventory |
| Source drop manifest | Retain registry archives/git revisions/vendor bundle |
| NuGet license metadata | Cargo SPDX `license` field |
| Container license scan | Combine OS/native inventory with Cargo graph |
| Vendor legal approval | Supported-profile license/provenance gate |

Enterprise compliance platforms can automate evidence collection. Keep the
portable source of truth grounded in package identity, source, version, terms,
and actual distribution.

## Common Confusion Points

- **"`MIT OR Apache-2.0` means both apply."** `OR` offers a choice; `AND` combines
  obligations.
- **"SPDX metadata is legal proof."** It is structured publisher metadata.
- **"Only direct dependencies need notices."** Transitive and non-Cargo inputs
  may carry obligations.
- **"Build dependencies are never distributed."** Generated output, templates,
  embedded assets, and distribution workflow can complicate that claim.
- **"Open source policy equals license policy."** Provenance, security,
  maintenance, and source approval remain separate.

## Decision Cheat Sheet

| Situation | Action |
|-----------|--------|
| Standard SPDX expression matches distributed files and allowlist | Record evidence and include required notices |
| Metadata/file conflict | Hold and seek upstream/legal clarification |
| Non-standard license | Review `license-file` and distribution obligations |
| Vendored/native/generated input | Add separate provenance and notice entry |
| Internal crate may be published | Set explicit workspace license/repository metadata |
| Tool reports low-confidence license | Inspect source files; do not silently allow |

## Primary Sources

- Cargo manifest license fields: https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields
- SPDX license expressions: https://spdx.github.io/spdx-spec/v2.3/SPDX-license-expressions/
- Cargo package: https://doc.rust-lang.org/cargo/commands/cargo-package.html
- cargo-deny licenses: https://embarkstudios.github.io/cargo-deny/checks/licenses/index.html
- crates.io package policies: https://crates.io/policies

## Related Guides

- Previous: [08-CARGO-AUDIT-CARGO-DENY-CARGO-VET-ADVISORIES-AND-SUPPLY-CHAIN-CONTROLS.md](08-CARGO-AUDIT-CARGO-DENY-CARGO-VET-ADVISORIES-AND-SUPPLY-CHAIN-CONTROLS.md)
- Next: [10-MAINTENANCE-STEWARDSHIP-BUS-FACTOR-AND-FORKS.md](10-MAINTENANCE-STEWARDSHIP-BUS-FACTOR-AND-FORKS.md)
- Source custody: [07-CRATES-IO-ALTERNATE-REGISTRIES-SOURCE-REPLACEMENT-AND-VENDORING.md](07-CRATES-IO-ALTERNATE-REGISTRIES-SOURCE-REPLACEMENT-AND-VENDORING.md)
