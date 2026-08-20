---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:evaluation-scorecards-evidence
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Crate Evaluation Scorecards and Evidence
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/02-EVALUATION-SCORECARDS-AND-EVIDENCE.md
canonical_path: rust-crate-ecosystem/02-EVALUATION-SCORECARDS-AND-EVIDENCE.md
backsource_ids: [proof-backfill:rust-crate-ecosystem:02-evaluation-scorecards-evidence]
concepts: [crate evaluation, scorecard, dependency evidence, adoption decision, executable spike]
root_concepts: [dependency evaluation]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Crate Evaluation Scorecards and Evidence

## The Big Picture

A scorecard is a compression mechanism, not a substitute for judgment. It should
make evidence and uncertainty comparable across candidates while preserving
hard gates that no weighted average may erase.

```
+===========================================================================+
|                            EVALUATION EVIDENCE                            |
+===========================================================================+
| Hard gates -> weighted fit -> operating proof                             |
| license/source   API/architecture   compile/test spike                    |
| required targets compatibility      target/feature matrix                 |
| security boundary dependency cost   owner and renewal plan                |
|                                                                           |
|                              v                                            |
|                   adopt / trial / hold / reject                           |
+===========================================================================+
```

The decision record should distinguish observed facts, upstream claims,
inferences, and unknowns. "README says supported" is not the same evidence as a
CI run under the product's toolchain.

## Hard Gates Before Scoring

Run gates first. A weighted score must not allow excellent documentation to
offset an unacceptable license, unsupported target, or forbidden source.

| Gate | Pass evidence | Typical failure |
|------|---------------|-----------------|
| Functional boundary | API spike meets the actual use case | Candidate solves adjacent, not required, problem |
| Source policy | Approved registry/git/path provenance | Unknown or disallowed source |
| License | Reviewed SPDX expression and notices | Incompatible or unclear terms |
| Target/MSRV | Matrix succeeds under supported profile | README-only claim or undeclared MSRV |
| Execution boundary | Build scripts, proc macros, native code accepted | Host execution exceeds policy |
| Security | Risk owner accepts current findings and exposure | Unresolved advisory or unbounded unsafe boundary |

Record a hold when evidence is missing. Do not turn "unknown" into a middle
score.

## A Practical Weighted Scorecard

Use weights that match the product. The example below is a starting point, not a
universal standard.

| Dimension | Weight | 1 means | 3 means | 5 means |
|-----------|-------:|---------|---------|---------|
| Functional/API fit | 20 | Major adapter or missing behavior | Meets core need with manageable glue | Natural boundary, low coupling |
| Compatibility | 15 | Target/MSRV conflict | Works in current matrix | Explicit policy plus verified matrix |
| Dependency/build cost | 10 | Large opaque graph or heavy host/native execution | Acceptable graph | Small, inspectable, feature-controlled |
| Safety/security posture | 15 | Unbounded unsafe or unresolved findings | Known boundaries and mitigations | Small documented boundary, active process |
| Maintenance/stewardship | 15 | No credible owner or continuity | Responsive enough for use | Clear policy, succession, sustainable cadence |
| API stability/upgrade cost | 10 | High churn or weak SemVer discipline | Normal migration cost | Stable surface and useful release notes |
| License/provenance | 10 | Ambiguous or incompatible | Acceptable with conditions | Clear SPDX/provenance/notices |
| Documentation/operability | 5 | Cannot reproduce basic use | Usable docs and diagnostics | Excellent examples and failure guidance |

Calculation:

```text
weighted score = sum(score[1..5] * weight) / sum(weight)
```

Keep the raw notes. A final `4.2` without evidence links is false precision.

## Evidence Ledger

For each material claim, retain its type and date.

```text
Claim: supports wasm32-unknown-unknown without std
Type: observed
Evidence: cargo check --target wasm32-unknown-unknown --no-default-features
Toolchain: rustc 1.xx.y, cargo 1.xx.y
Features: alloc
Result: pass
Observed: 2026-08-11
Limits: examples and optional TLS backend not tested
```

| Evidence class | Example | Confidence boundary |
|----------------|---------|---------------------|
| Observed | Command output from the candidate spike | Applies to named versions/configuration |
| Primary claim | Upstream manifest, policy, docs, release notes | May be incomplete or stale |
| Source inspection | Reviewed unsafe/build/API paths | Applies to inspected revision and scope |
| External data | Advisory, license scan, dependent report | Depends on database/tool coverage |
| Inference | "Likely sustainable because..." | Must name assumptions |
| Unknown | No published security policy | Do not convert to a negative fact without more evidence |

## The Executable Spike

The spike should exercise the adoption boundary, not a toy example.

```
candidate dependency
      |
      +-> required API path
      +-> error behavior
      +-> concurrency/lifecycle boundary
      +-> minimum supported toolchain
      +-> target and feature combinations
      +-> release build and package/license checks
      +-> remove/replace experiment
```

Example commands:

```text
cargo check --locked --all-targets
cargo test --locked --workspace
cargo tree -e features
cargo tree -d
cargo check --target x86_64-pc-windows-msvc
cargo check --target x86_64-unknown-linux-gnu
```

`--all-features` proves one maximal additive configuration, not every valid
combination. Feature powersets grow exponentially and mutually exclusive
features are already a design smell. Select risk-based combinations explicitly;
third-party matrix helpers can automate them but must be version-pinned and
treated as tool dependencies.

## Inspect the Graph, Not Only the Direct Crate

```text
cargo tree -p candidate
cargo tree -p candidate -e features
cargo tree -i transitive-crate
cargo tree -d
cargo metadata --format-version 1
```

The graph review asks:

| Question | Why it matters |
|----------|----------------|
| Which default features are active? | Defaults can add runtimes, TLS, native code, or macros |
| Are duplicate major versions present? | Binary size, type incompatibility, and patch burden |
| Which units run on the host? | Build-time trust and reproducibility |
| Which dependencies are target-specific? | Cross-platform support and dormant risk |
| Which package owns unsafe/native boundaries? | Review and incident routing |

## Popularity and Reputation

Use popularity as a weak Bayesian prior at most. High use may increase the chance
of bug reports, ecosystem knowledge, and compatibility pressure. It may also
increase attack value and migration cost. Low use may mean niche excellence,
newness, or abandonment. Counts do not resolve those interpretations.

| Weak signal | Better follow-up |
|-------------|------------------|
| High downloads | Inspect dependents, release policy, and current graph |
| Many stars | Inspect issue/PR handling and maintainer authority |
| Frequent releases | Read changelog and SemVer behavior |
| Few maintainers | Examine succession, automation, and fork feasibility |
| Corporate use claim | Find public evidence or treat as unverified |

## Old World -> New World Bridge

This is a build-versus-buy review applied to source packages. The difference is
that Cargo resolution can change the exact component without a source edit when
the manifest permits it.

| Traditional review artifact | Cargo ecosystem artifact |
|-----------------------------|--------------------------|
| Vendor questionnaire | Upstream policy/repository review |
| Binary qualification matrix | Toolchain/target/feature spike |
| Bill of materials | `Cargo.lock` plus `cargo metadata` |
| Architecture review | API boundary and dependency-direction review |
| Renewal date | Supported-profile review in [15](15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md) |

Enterprise scorecard portals can store the outcome, but the portable evidence
should remain commands, versions, paths, and decision notes.

## Common Confusion Points

- **"A high score overrides a hard gate."** It must not.
- **"No open issue means no defect."** It may mean no report, no triage, or a
  private disclosure process.
- **"The latest release is the evaluated release."** Name the exact selected
  version and lockfile.
- **"All features is comprehensive."** It can test an unrealistic configuration
  while missing minimal and target-specific paths.
- **"A quiet repository is abandoned."** Mature crates can be stable. Evaluate
  response capacity and ownership, not commit theater.

## Decision Cheat Sheet

| Decision state | Use when | Required next action |
|----------------|----------|----------------------|
| Adopt | Gates pass, evidence is current, score meets threshold, owner exists | Record supported profile |
| Trial | Technical fit is promising but operational evidence is incomplete | Time-box spike with explicit questions |
| Hold | A material unknown blocks a gate | Assign evidence owner and expiry |
| Reject | A gate fails or cost clearly exceeds value | Record reason and reconsideration trigger |
| Adopt with condition | Risk is bounded by feature/source/target constraint | Encode condition in manifest, CI, and policy |

## Primary Sources

- Cargo metadata: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
- Cargo tree: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
- Cargo manifest reference: https://doc.rust-lang.org/cargo/reference/manifest.html
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- RustSec: https://rustsec.org/

## Related Guides

- Previous: [01-CRATE-TAXONOMY-DISCOVERY-AND-ALTERNATIVES.md](01-CRATE-TAXONOMY-DISCOVERY-AND-ALTERNATIVES.md)
- Next: [03-SEMVER-MSRV-DEPENDENCY-CONSTRAINTS-AND-COMPATIBILITY.md](03-SEMVER-MSRV-DEPENDENCY-CONSTRAINTS-AND-COMPATIBILITY.md)
- Ongoing ownership: [15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md](15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md)
