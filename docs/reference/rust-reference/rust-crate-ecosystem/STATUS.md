# rust-crate-ecosystem/ - Status

**17 files (STATUS.md + 16 numbered guides) | Complete | Source-custody partial**

This canonical MAXIM module covers selection, integration, governance, and
retirement of Rust crate dependencies. It complements `../rust-language/`
(language semantics and user-facing Rust) and `../rust-architecture/`
(rustc/Cargo implementation architecture) without duplicating either module.

## Guides

| File | Topic | Status |
|------|-------|--------|
| `00-OVERVIEW.md` | Ecosystem landscape, authority boundaries, evidence strength, and reading paths | done |
| `01-CRATE-TAXONOMY-DISCOVERY-AND-ALTERNATIVES.md` | Crate layers, discovery funnel, alternatives including no crate, and weak popularity signals | done |
| `02-EVALUATION-SCORECARDS-AND-EVIDENCE.md` | Hard gates, weighted scorecards, evidence ledger, graph inspection, and executable spikes | done |
| `03-SEMVER-MSRV-DEPENDENCY-CONSTRAINTS-AND-COMPATIBILITY.md` | Cargo requirements, SemVer limits, `rust-version`, resolver versions, and compatibility matrices | done |
| `04-CARGO-FEATURES-UNIFICATION-AND-OPTIONAL-DEPENDENCIES.md` | Additive features, default-feature traps, resolver boundaries, and optional dependencies | done |
| `05-WORKSPACE-ARCHITECTURE-AND-DEPENDENCY-DIRECTION.md` | Workspace mechanics, crate boundaries, inward dependency direction, and central policy | done |
| `06-LOCKFILES-REPRODUCIBILITY-AND-UPDATE-POLICY.md` | Lockfile custody, locked/offline builds, bounded updates, rollback inputs, and reproducibility layers | done |
| `07-CRATES-IO-ALTERNATE-REGISTRIES-SOURCE-REPLACEMENT-AND-VENDORING.md` | Public/private sources, mirrors, source replacement, git/path dependencies, and vendoring | done |
| `08-CARGO-AUDIT-CARGO-DENY-CARGO-VET-ADVISORIES-AND-SUPPLY-CHAIN-CONTROLS.md` | RustSec matching, policy checks, review trust, exceptions, and layered controls | done |
| `09-LICENSING-PROVENANCE-AND-ATTRIBUTION.md` | SPDX metadata, source evidence, graph-level license review, and notice output | done |
| `10-MAINTENANCE-STEWARDSHIP-BUS-FACTOR-AND-FORKS.md` | Maintenance signals, authority concentration, upstream engagement, and fork ownership | done |
| `11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md` | Target matrices, `core`/`alloc`/`std`, target dependencies, and executable support claims | done |
| `12-NATIVE-DEPENDENCIES-BUILD-SCRIPTS-AND-SYSTEM-PACKAGES.md` | Host/target split, build scripts, `-sys` crates, native supply models, and cross compilation | done |
| `13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md` | Host-executed macros/generators, generated custody, tool pinning, and `xtask` | done |
| `14-INTERNAL-CRATES-API-BOUNDARIES-PUBLISHING-AND-DEPRECATION.md` | Internal visibility, public API surface, packaging, publishing order, and staged deprecation | done |
| `15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md` | Supported-profile records, renewal, bounded updates, rollback, and removal evidence | done |

## Editorial and Technical Posture

All sixteen guides use `maxim.frontmatter.v1`, module
`rust-crate-ecosystem`, `status: source-custody`, and
`source_custody: partial`, with canonical/current paths and PROOF backsource
IDs matching each numbered file.

Every guide implements the seven MAXIM style surfaces:

1. opening Big Picture ASCII diagram;
2. layered drill-down from the diagram;
3. structural ASCII boxes/flows;
4. comparison and decision tables;
5. universal old-world/new-world bridge first, with .NET/Microsoft examples
   only as supplemental context;
6. Decision Cheat Sheet;
7. Common Confusion Points.

Popularity/download/star counts are consistently treated as weak discovery
signals. Cargo resolver, feature, build-script, and configuration behavior is
bounded by version where material. Registry checksum, advisory, audit, license,
and security-tool claims are deliberately narrow: the guides state what a
control can establish and what it cannot.

Concrete examples cover `Cargo.toml`, `.cargo/config.toml`, build scripts,
workspace policy, lock/update commands, target matrices, policy tools, package
publication, supported-profile records, and rollback/removal evidence.

## Review Maturity

The module received an independent cross-review on 2026-08-11 through all four
repository roles:

- Reader Path Editor: overview routes and adjacent-guide links support task-based
  entry and movement.
- Reference Integrity Auditor: authorities, exceptions, version-sensitive
  behavior, and bounded claims remain visible.
- Executable Evidence Auditor: examples state toolchain/target assumptions or
  pin externally versioned policy tools where shown, use current command/config
  schema, and state graph/profile scope.
- Learner Advocate: openings establish why the topic matters; examples and
  decision tables stay concrete without re-teaching general architecture.

The pass corrected Cargo resolver/MSRV qualifications, feature package-ID
boundaries, lockfile/Cargo compatibility, registry/source replacement and
vendoring semantics, current cargo-deny/audit/vet examples, license metadata,
`no_std`/`alloc` target caveats, build metadata flow, publishability rules, and
renewal/removal evidence.

Full-module validation result:

```text
OK — 17 files checked, 0 errors, 0 warnings
```

No unresolved inline editorial findings remain. This status does not claim
Certified Gold; independent review and a clean proof gate establish a stronger
candidate, not final Gold certification.

## Source Custody

The numbered guides are canonical source. Per task scope, no generated
`.proof`, `.mdcrop`, `.mdport`, `.fletch`, MkDocs, tracker, or other external
files were edited, and source backfill was not run. The module remains
`source_custody: partial` until a later explicitly scoped backfill wave.
