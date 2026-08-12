---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:supply-chain-controls
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: cargo-audit, cargo-deny, cargo-vet, Advisories, and Supply-Chain Controls
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/08-CARGO-AUDIT-CARGO-DENY-CARGO-VET-ADVISORIES-AND-SUPPLY-CHAIN-CONTROLS.md
canonical_path: rust-crate-ecosystem/08-CARGO-AUDIT-CARGO-DENY-CARGO-VET-ADVISORIES-AND-SUPPLY-CHAIN-CONTROLS.md
backsource_ids: [mdloom-backfill:rust-crate-ecosystem:08-supply-chain-controls]
concepts: [cargo audit, cargo deny, cargo vet, RustSec, software supply chain]
root_concepts: [rust supply chain]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# cargo-audit, cargo-deny, cargo-vet, Advisories, and Supply-Chain Controls

## The Big Picture

No single Rust tool is a supply-chain firewall. Each control answers a narrower
question, with coverage bounded by its inputs, database, configuration, and
version.

```
+===========================================================================+
|                         SUPPLY-CHAIN CONTROL STACK                        |
+===========================================================================+
| Cargo.lock -> cargo-audit -> known RustSec matches                        |
|            -> cargo-deny  -> advisory/license/ban/source policy           |
|            -> cargo-vet   -> review and audit-trust requirements          |
|            -> SBOM/provenance -> inventory and build custody              |
|                                                                           |
| Repository controls -> owners, CI, credentials, review, release           |
| Result: layered evidence, not proof that all code is benign or correct.   |
+===========================================================================+
```

Use scans to enforce explicit policy and route exceptions. Do not translate
"tool passed" into "dependency is secure."

## `cargo-audit`: Known Advisory Matching

`cargo-audit` examines the resolved dependency graph, normally from
`Cargo.lock`, against the RustSec Advisory Database.

```text
cargo install cargo-audit --version 0.22.2 --locked
cargo audit
cargo audit --json
```

The version is the reviewed example as of 2026-08-11. Pin installation in CI
through a managed tool image, installer manifest, or locked bootstrap process;
do not reinstall an unbounded latest version on every run.

| It can report | It cannot establish |
|---------------|---------------------|
| Matching RustSec vulnerabilities | Absence of unknown vulnerabilities |
| Informational categories supported by the tool/database | Exploitability in your exact product without analysis |
| Dependency path to a finding | Runtime reachability by itself |
| Patched/unaffected ranges published in advisory | Correctness of the upstream advisory |

An ignored advisory is an accepted exception, not a fixed finding. Record owner,
rationale, affected profile, compensating control, and expiry outside or next to
the machine-readable ignore.

## `cargo-deny`: Policy Across the Graph

`cargo-deny` can check advisory, license, ban/duplicate, and source policies.
Generate a configuration for the installed version:

```text
cargo install cargo-deny --version 0.20.2 --locked
cargo deny init
cargo deny check
cargo deny check advisories licenses bans sources
```

The configuration schema evolves. Start from `cargo deny init` produced by the
pinned tool version, then review it. A policy shape might express:

```toml
[advisories]
ignore = []

[bans]
multiple-versions = "warn"
wildcards = "deny"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause", "ISC"]
confidence-threshold = 0.93

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
allow-git = []
```

This fragment is valid for cargo-deny 0.20.2 but remains illustrative policy.
In that release, yanked and unmaintained findings are part of the advisories
check; exceptions belong in reviewed advisory ignore entries rather than a
`yanked` lint key. Validate keys and defaults whenever the pinned tool changes.
Source URLs, exceptions, and license allowlists are organizational decisions,
not universal best practices.

## `cargo-vet`: Review Trust and Audit Criteria

`cargo-vet` models whether dependency versions satisfy named audit criteria
through local audits, trusted imports, exemptions, and review history.

```text
cargo install cargo-vet --version 0.10.2 --locked
cargo vet init
cargo vet
```

A conceptual audit entry:

```toml
[[audits.example-crate]]
who = "Reviewer Name <reviewer@example.invalid>"
criteria = "safe-to-deploy"
version = "1.4.2"
notes = "Reviewed unsafe boundary and network parsing paths."
```

Use the exact schema generated/documented by the pinned cargo-vet version.
Criteria are claims with organizational meaning. Define what
`safe-to-deploy`, `safe-to-run`, or custom criteria require. Imported audits
transfer trust to another audit publisher; they are not free review.

By default, cargo-vet 0.10.2 asks Cargo for an all-features metadata graph.
That may over-approximate the shipped profile or fail for mutually exclusive
features. Use its `--no-all-features`, `--no-default-features`, and `--features`
options to run each supported profile deliberately. cargo-vet's own `--locked`
option controls imported-audit refresh; it is not Cargo's "do not change
Cargo.lock" flag.

```
our criteria
     |
     +-> local full audit
     +-> delta audit from reviewed version
     +-> imported audit from trusted source
     +-> temporary exemption with owner/expiry
```

## Controls Beyond Cargo Plugins

| Control | Threat/question addressed | Boundary |
|---------|-----------------------------|----------|
| Locked graph | What exact packages were selected? | Does not assess code |
| Registry checksum | Did archive bytes match metadata? | Does not prove author or safety |
| Protected branches/review | Who may change manifests/locks/policy? | Depends on identity and repository controls |
| Least-privilege publish tokens | Who may publish? | Registry and credential implementation matter |
| Provenance/attestation | How was an artifact built? | Trust depends on builder and verification |
| SBOM | What was included? | Inventory may omit runtime/system components |
| Sandboxed builds | What can host-executed code access? | Platform-specific and rarely absolute |
| Incident playbook | How will findings be triaged and removed? | Requires rehearsed ownership |

Build scripts and proc macros execute during the build. A policy that scans only
runtime dependencies misses a material host-side trust boundary.

## Advisory Triage

```
finding
  |
  +-> Is selected version affected?
  |       no -> record why and close
  |
  +-> Is vulnerable code reachable in supported profile?
  |       unknown -> investigate; do not assume no
  |
  +-> Is fixed version compatible?
  |       yes -> update and validate
  |       no  -> disable feature / patch / fork / replace / mitigate
  |
  +-> Assign deadline, owner, release, and evidence
```

Severity scores are inputs, not deadlines. Product exposure, data sensitivity,
network reachability, exploit maturity, and compensating controls shape the
risk. Conversely, "not reachable today" can become false when a feature or call
path changes.

## Toolchain Policy Example

```text
On every manifest/lock change:
  cargo test --workspace --locked
  cargo audit
  cargo deny check
  cargo vet

Nightly:
  refresh advisory data under pinned tool versions
  scan supported release branches

Exceptions:
  owner + rationale + affected version/profile + expiry + removal issue
```

Network access, database refresh, and imported audit behavior should be explicit
in CI. A cached database improves availability but can reduce freshness.

## Old World -> New World Bridge

| Familiar control | Rust ecosystem form |
|------------------|---------------------|
| CVE/SCA scan | RustSec plus `cargo-audit`/advisory checks |
| License/source policy | `cargo-deny` |
| Third-party source review | `cargo-vet` audits and criteria |
| Approved package feed | Registry/source policy plus ingest controls |
| Component exception register | Advisory/license/audit exemption with owner and expiry |

Microsoft Defender, GitHub dependency features, or enterprise SCA platforms can
supplement this stack. They do not change the portable requirement: know the
resolved graph, control sources, define review criteria, and own exceptions.

## Common Confusion Points

- **"No RustSec finding means no vulnerability."** Coverage is limited to known,
  published, matching advisories.
- **"`cargo-deny` is a security scanner."** It is a configurable graph-policy
  checker spanning several domains.
- **"Imported cargo-vet audits remove our accountability."** They make another
  party's review part of your trust model.
- **"Yanked means vulnerable."** Yanking blocks new selection but can reflect
  many reasons; existing lockfiles may still name the version.
- **"A supply-chain tool should auto-fix."** Resolution changes can alter API,
  MSRV, features, targets, and behavior. Automation still needs bounded review.

## Decision Cheat Sheet

| Question | Primary control | Required companion |
|----------|-----------------|--------------------|
| Does our lock match a known RustSec advisory? | `cargo-audit` | Product reachability/response analysis |
| Are sources/licenses/duplicates within policy? | `cargo-deny` | Reviewed, versioned configuration |
| Has code review evidence met our trust criteria? | `cargo-vet` | Defined criteria and trusted audit sources |
| Can we prove exact shipped inventory? | Lockfile/SBOM | Native/runtime artifact inventory |
| Can we survive a compromised or abandoned crate? | Source control, fork/remove playbook | [10](10-MAINTENANCE-STEWARDSHIP-BUS-FACTOR-AND-FORKS.md) and [15](15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md) |

## Primary Sources

- RustSec Advisory Database: https://rustsec.org/
- cargo-audit: https://github.com/RustSec/rustsec/tree/main/cargo-audit
- cargo-deny: https://embarkstudios.github.io/cargo-deny/
- cargo-vet: https://mozilla.github.io/cargo-vet/
- Cargo credentials: https://doc.rust-lang.org/cargo/reference/registry-authentication.html

## Related Guides

- Previous: [07-CRATES-IO-ALTERNATE-REGISTRIES-SOURCE-REPLACEMENT-AND-VENDORING.md](07-CRATES-IO-ALTERNATE-REGISTRIES-SOURCE-REPLACEMENT-AND-VENDORING.md)
- Next: [09-LICENSING-PROVENANCE-AND-ATTRIBUTION.md](09-LICENSING-PROVENANCE-AND-ATTRIBUTION.md)
- Build-time code: [13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md](13-PROC-MACROS-CODE-GENERATION-AND-TOOL-DEPENDENCIES.md)
