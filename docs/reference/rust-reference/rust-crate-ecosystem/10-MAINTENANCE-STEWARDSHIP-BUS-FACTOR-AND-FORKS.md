---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-crate-ecosystem:maintenance-stewardship-bus-factor-forks
kind: guide
module: rust-crate-ecosystem
section: rust-crate-ecosystem
title: Maintenance, Stewardship, Bus Factor, and Forks
status: source-custody
source_custody: partial
current_path: rust-crate-ecosystem/10-MAINTENANCE-STEWARDSHIP-BUS-FACTOR-AND-FORKS.md
canonical_path: rust-crate-ecosystem/10-MAINTENANCE-STEWARDSHIP-BUS-FACTOR-AND-FORKS.md
backsource_ids: [mdloom-backfill:rust-crate-ecosystem:10-maintenance-stewardship-bus-factor-forks]
concepts: [crate maintenance, stewardship, bus factor, dependency fork, maintainer continuity]
root_concepts: [dependency stewardship]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Maintenance, Stewardship, Bus Factor, and Forks

## The Big Picture

Maintenance is the ability to make trustworthy decisions over time, not the
number of commits in the last month. Evaluate authority, response capacity,
release discipline, succession, and your own exit options.

```
+===========================================================================+
|                         STEWARDSHIP SYSTEM                                |
+===========================================================================+
|                                                                           |
|  ownership -> issue/security intake -> review -> release -> succession    |
|      |                |                |         |          |             |
|      v                v                v         v          v             |
| registry owners   response policy   CI/evidence  SemVer   transfer/fork    |
|                                                                           |
| Product side: named dependency owner -> renewal -> incident -> exit       |
+===========================================================================+
```

"Bus factor" is shorthand for concentration risk, but a guessed number is less
useful than a map of which people or organizations can publish, merge, disclose,
and transfer control.

## Maintenance Signals

| Signal | Healthy interpretation | Misleading shortcut |
|--------|------------------------|---------------------|
| Release cadence | Matches change/security needs | More releases always better |
| Issue response | Clear triage and bounded expectations | Every issue must be closed quickly |
| Review pattern | Multiple trusted reviewers or explicit single-owner model | Contributor count equals governance depth |
| Registry owners | Current, least-privilege, succession-aware | Many owners automatically safer |
| CI matrix | Covers stated MSRV/targets/features | Badge means all profiles work |
| Changelog/migrations | Explains compatibility decisions | Version number alone proves SemVer |
| Security process | Private intake and response ownership | Public issues are the only evidence |

Stable software can be quiet. The critical question is whether someone credible
can respond when the ecosystem, compiler, target, or security context changes.

## Stewardship Review

```
Who can merge?
Who can publish?
Who receives security reports?
Who reviews unsafe/native/build-time changes?
Who can add or remove owners?
What happens during absence or transfer?
```

Capture facts and unknowns:

```text
Project: example-crate
Maintaining entity: individual / team / foundation / company / unclear
Repository merge authority: ...
Registry publish owners: ...
Security contact/process: ...
Last material compatibility response: ...
Succession/transfer policy: ...
Our internal owner: ...
```

Registry owner lists and repository permissions are different authorities. A
person able to merge may not be able to publish, and vice versa.

## Concentration Risk

| Concentration | Risk | Mitigation |
|---------------|------|------------|
| One publisher, one reviewer | Account loss or absence blocks response | Internal fork readiness, relationship, alternative |
| One sponsoring company | Strategy/funding change | Neutral governance or replaceability |
| Many contributors, one final approver | Review bottleneck hidden by activity | Understand actual decision rights |
| Automated release credentials | Credential/process compromise | Protected environments, short-lived credentials, review |
| Complex unsafe/native core known by one person | Knowledge concentration | Independent review and documented invariants |

Do not penalize a well-run single-maintainer crate automatically. Price the
continuity risk against code size, stability, replaceability, and your ability
to assume custody.

## Response Options Before Forking

```
problem
  |
  +-> configuration/feature workaround
  +-> compatible upgrade or alternate backend
  +-> upstream issue/PR
  +-> temporary [patch] to immutable revision
  +-> maintained community successor
  +-> internal fork
  +-> replacement or internal implementation
```

Forking is a transfer of maintenance obligations, not merely a Git operation.

| Fork cost | Questions |
|-----------|-----------|
| Source custody | Where is the fork hosted and mirrored? |
| Package identity | Internal registry name, `[patch]`, or unpublished source? |
| Security | Who receives advisories and reviews diffs? |
| Release | Who versions, signs, publishes, and supports artifacts? |
| Merge strategy | Track upstream, selective cherry-pick, or permanent divergence? |
| License/attribution | Are modifications and notices preserved? |
| Exit | Conditions to rejoin upstream, replace, or retire? |

## Fork Policy Example

```text
Trigger:
  critical defect with no acceptable upstream response window
  OR strategic feature required for supported profile

Approval:
  dependency owner + architecture owner + security/legal as applicable

Controls:
  immutable upstream base recorded
  fork diff reviewed
  private/public source decision documented
  package source override explicit
  release and advisory ownership assigned

Expiry:
  review every 90 days
  upstream/replacement path reconsidered
```

The 90-day period is an example. Match cadence to risk and release frequency.

## Upstream Engagement

Good dependency stewardship contributes minimal reproductions, tests, review,
funding, or maintenance help without assuming entitlement to volunteer labor.

| Need | Constructive action |
|------|---------------------|
| Bug fix | Small reproduction and focused patch |
| MSRV clarity | CI proposal and documented policy tradeoff |
| Security report | Use private process; coordinate disclosure |
| Succession | Offer sustained co-maintenance, not a one-off demand |
| Product-specific feature | Adapter/fork if upstream scope reasonably excludes it |

An upstream "no" can be healthy governance. It may tell you the crate's scope
does not match your product.

## Old World -> New World Bridge

| Traditional supplier concern | Open-source crate concern |
|------------------------------|---------------------------|
| Vendor viability | Maintainer/entity continuity |
| Support contract | Published scope and response process |
| Escrow/source access | Public source plus tested internal custody |
| Product end-of-life | Deprecation/archive/ownership transfer |
| Custom vendor branch | Internal fork with merge and release policy |

Enterprise sponsorship can reduce risk when it funds real maintainers and
succession. A company logo alone is not a support contract.

## Common Confusion Points

- **"No recent commits means abandoned."** Stable code can need little change.
- **"Many contributors means high bus factor."** Decision and publish authority
  may still be concentrated.
- **"We can always fork."** Only if the team can review, release, secure, and
  eventually exit the fork.
- **"Upstream must support our use case."** Scope alignment is part of adoption.
- **"A fork removes supply-chain risk."** It moves more of that risk into your
  organization.

## Decision Cheat Sheet

| Situation | Preferred response |
|-----------|--------------------|
| Quiet but stable crate, small surface, easy replacement | Accept with periodic renewal |
| Active project with clear governance | Engage upstream and retain exit plan |
| Single maintainer, critical deep dependency | Add fork/replacement readiness and stronger review |
| Urgent fix likely to land upstream | Temporary immutable patch |
| Permanent strategic divergence | Named internal fork product with full ownership |
| No credible continuity and expensive exit | Reject or replace before deeper adoption |

## Primary Sources

- Cargo package ownership: https://doc.rust-lang.org/cargo/commands/cargo-owner.html
- Cargo publishing: https://doc.rust-lang.org/cargo/reference/publishing.html
- crates.io policies: https://crates.io/policies
- Rust security policy: https://www.rust-lang.org/policies/security
- OpenSSF project security guidance: https://best.openssf.org/

## Related Guides

- Previous: [09-LICENSING-PROVENANCE-AND-ATTRIBUTION.md](09-LICENSING-PROVENANCE-AND-ATTRIBUTION.md)
- Next: [11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md](11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md)
- Renewal and exit: [15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md](15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md)
