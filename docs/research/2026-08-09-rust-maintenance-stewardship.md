# Rust Maintenance and Stewardship

Date: 2026-08-09
Status: Complete
Question: ECOS-Q05
Decision: represent stewardship as renewable evidence across registry
authority, publication provenance, source governance, path-scoped work,
responsiveness, succession, lifecycle declarations, and replacement lineage.
Do not reduce these dimensions to one maintenance score or infer abandonment
from release age.

## Decision supported

ECOS-Q05 defines the maintenance and stewardship evidence required by the
Crate Ecosystem Ledger.

It does not:

- certify any selected crate as maintained;
- treat organization ownership, team ownership, funding, popularity, commit
  count, or release cadence as a guarantee;
- label a responsive maintainer's project unmaintained;
- identify a fork as a successor without an explicit relationship and
  compatibility evidence; or
- make FERRIUM an arbiter of open-source maintainer performance.

## Stewardship evidence model

A stewardship record must keep these dimensions separate:

| Dimension | Required evidence |
|---|---|
| Registry authority | Current crates.io user and team owners, observation time, owner-set diff, and authority to publish, yank, or manage owners |
| Publication provenance | Exact version, checksum, human publisher or trusted provider/repository/run/commit, publish time, yank actions, and current trustpub-only policy |
| Source authority | Canonical repository identity, owner kind, transfer or rename lineage, default branch, archive/disable/fork state, and tag-to-source relationship |
| Work | Crate-path commits, contributors, merged changes, reviewers, CODEOWNERS, release work, and observation window |
| Responsiveness | Issue, pull-request, release, and security-report response observations with samples and latency rather than open-count totals |
| Concentration | Independent current owners, publishers, path contributors, approvers, release actors, and emergency access paths |
| Succession | Documented maintainer additions, team or organization custody, release automation, handoff, seeking-maintainer statements, and contingency ownership |
| Lifecycle | Explicit maintenance declaration, stable/quiet intent, limited-maintenance scope, deprecation, archive, RustSec unmaintained advisory, and contact evidence |
| Replacement | Fork/network relation, successor declaration, alternative recommendation, namespace/version identity, migration, compatibility, and adoption evidence |
| Support context | Funding, employer or foundation support, governance policy, and service commitments without treating them as guarantees |

These are observations and claims with dates, sources, and confidence. They are
not additive points.

## Measured queue

The nineteen ECOS-Q02 releases were re-observed through crates.io and their
eighteen canonical GitHub repositories. Exact commands and tables are in
[EXP-01](ecos-q05-maintenance-stewardship/results/EXP-01-stewardship-census.md).

Registry observations:

- four crates had one current individual owner and no team owner:
  `proc-macro2`, `quote`, `syn`, and `memchr`;
- eleven had at least one current GitHub team owner;
- thirteen had one publication authority identity across their latest ten
  non-yanked releases, while six had multiple identities;
- four were currently configured as trustpub-only;
- three selected exact releases were published through GitHub trusted
  publishing, with repository, workflow run, and commit recorded by crates.io;
- four crates had recent historical human publishers who were not in the
  current owner set; and
- the selected releases belonged to packages with zero through seventeen
  currently yanked historical versions.

Repository observations:

- fourteen canonical repositories were organization-owned and four were
  user-owned;
- none was archived, disabled, or a fork at observation time;
- fourteen exposed a GitHub security-policy URL;
- seven exposed a funding link;
- only `tokio-rs/tracing` contained a CODEOWNERS file in GitHub's recognized
  default-branch locations; and
- recent commit and pull-request participation varied from highly concentrated
  to broadly distributed.

No selected package had a RustSec `informational = "unmaintained"` advisory in
advisory-db commit `565436d86a136c840d01ad4a7851fc7391295404`.

These results are queue evidence, not health verdicts.

## Findings

### FERRIUM-561: stewardship is multidimensional and time-dependent

**Sources:** registry, publication, GitHub, and RustSec observations in
EXP-01.

**Observed behavior:** Owner counts, publication actors, repository ownership,
commit participation, review participation, security-policy presence, release
age, and lifecycle declarations described different properties. No single
field established maintenance.

**Implication:** The Crate Ecosystem Ledger must preserve dimensions and
snapshots rather than calculate one universal maintenance score.

**Confidence:** High.

### FERRIUM-562: crates.io owners are publishing authorities, not a maintainer roster

**Sources:** crates.io owners API and Cargo `owner` documentation.

**Observed behavior:** Current owners may publish or yank. Individual owners
can manage the owner set; team owners cannot. The API does not say who reviews
code, answers issues, sets policy, or performs day-to-day maintenance.

**Implication:** Registry authority must not be relabeled as governance,
contribution, review, or support authority.

**Confidence:** High.

### FERRIUM-563: a team owner is useful continuity evidence but has opaque membership

**Sources:** Cargo owner documentation, Rust Project crate-ownership policy,
and the eleven measured team-owned crates.

**Observed behavior:** A team owner moves publish access behind a GitHub team,
but the crates.io owner response does not enumerate team membership,
decision-making, quorum, emergency access, or succession.

**Implication:** Record team identity and governing organization, but keep
membership, access policy, and continuity unknown unless separately observed.

**Confidence:** High.

### FERRIUM-564: owner transfer requires temporal evidence

**Sources:** current owner responses and recent-version publication histories.

**Observed behavior:** `cfg-if`, `getrandom`, `tracing-core`, and
`tower-service` had recent historical publishers absent from the current owner
set. A current owner list alone cannot establish when or why authority changed.

**Implication:** OSPREY must renew and diff owner sets. A transfer event should
identify additions, removals, observation interval, statements, and related
repository movement without inventing missing chronology.

**Confidence:** High.

### FERRIUM-565: publication authority can be a person or an authenticated workflow

**Sources:** crates.io version API and trusted-publishing documentation.

**Observed behavior:** Most selected releases named a human publisher.
`getrandom 0.4.3`, `rand_core 0.10.1`, and `cc 1.4.2` instead recorded GitHub
repository, workflow run, and commit provenance.

**Implication:** Publisher identity must be a tagged union rather than a user
field. Trusted publication narrows credential and origin questions but does
not prove review, source correctness, or stewardship.

**Confidence:** High.

### FERRIUM-566: current publication policy and historical release provenance differ

**Sources:** crates.io crate and exact-version responses.

**Observed behavior:** Four packages were trustpub-only at observation time,
but only three selected exact releases used trusted publishing. `libc
0.2.189` named a human publisher even though the package is now trustpub-only.

**Implication:** Store policy observations separately from per-release
provenance. Do not retroactively apply current controls to older releases.

**Confidence:** High.

### FERRIUM-567: release age is not abandonment

**Sources:** `tower-service 0.3.3` registry metadata and the
`tower-rs/tower/tower-service` path history.

**Observed behavior:** The release was approximately 725 days old, but the
package path changed in January 2026 and the repository had active 2026 work
and reviewed pull requests. A small stable contract can need fewer releases.

**Implication:** Release age may trigger review but cannot assign lifecycle
state. Stability intent, path activity, responsiveness, unresolved defects,
and maintainer statements must be considered.

**Confidence:** High.

### FERRIUM-568: repository activity can overstate crate activity

**Sources:** monorepo repository histories and path-filtered commit queries.

**Observed behavior:** `serde_core`, `tracing-core`, `tower-service`, and
`futures-core` share repositories with other packages. Repository-wide commit
counts include unrelated paths.

**Implication:** Stewardship evidence must resolve package-to-path identity and
prefer path-scoped work observations where possible.

**Confidence:** High.

### FERRIUM-569: contribution and review concentration are dependency signals, not authority

**Sources:** one-year default-branch histories and twenty recent merged pull
requests per repository.

**Observed behavior:** The latest hundred human-attributed commits were 99%
one author for proc-macro2, 88% for quote, and 100% for syn. Other repositories
showed broader commit and approval participation. Bots, direct pushes,
co-authorship, squash merges, private discussion, delegated review, and
non-GitHub processes can alter the interpretation.

**Implication:** Concentration should produce a review question about
continuity and succession, not an automatic bus-factor number or rejection.

**Confidence:** High for the sample; Low for a governance conclusion.

### FERRIUM-570: CODEOWNERS and approval records are partial governance evidence

**Sources:** GitHub CODEOWNERS documentation and measured repository queries.

**Observed behavior:** A CODEOWNERS file can request or require review from
write-authorized users or teams. Only one measured repository exposed such a
file, while many showed peer approvals without one. No visible CODEOWNERS file
does not prove absent ownership policy.

**Implication:** Record declared code ownership, protection/ruleset evidence,
and observed approvals independently. Preserve unavailable or private policy
as unknown.

**Confidence:** High.

### FERRIUM-571: a security policy is a contact route, not response evidence

**Sources:** GitHub security-policy documentation and repository queries.

**Observed behavior:** Fourteen of eighteen repositories exposed a security
policy URL. The field does not prove that reports receive timely responses or
that fixes and disclosures follow a defined service level.

**Implication:** ECOS-Q06 must join contact instructions with advisory,
disclosure, patch, release, and response-time evidence.

**Confidence:** High.

### FERRIUM-572: archive state is explicit; silence is not

**Sources:** GitHub archive documentation and measured repository state.

**Observed behavior:** GitHub archives are read-only and explicitly indicate
that a repository is no longer actively maintained. None of the measured
repositories was archived. Lack of recent work without an archive or
maintainer statement remains ambiguous.

**Implication:** Treat archive as a strong lifecycle event, while quiet
repositories remain `unknown`, `stable/quiet`, or `needs contact` according to
additional evidence.

**Confidence:** High.

### FERRIUM-573: an unmaintained claim requires declaration or failed contact

**Sources:** RustSec `HOWTO_UNMAINTAINED.md`.

**Observed behavior:** RustSec distinguishes explicit declarations from
implicit unmaintained status. For implicit status it requires prolonged stale
activity plus failed contact, and defers to a responsive author who states the
crate is maintained.

**Implication:** FERRIUM should ingest RustSec advisories and cited contact
evidence. It must not generate an unmaintained verdict from metrics alone.

**Confidence:** High.

### FERRIUM-574: forks and successors are different relationships

**Sources:** GitHub fork and repository-transfer documentation.

**Observed behavior:** A fork records network lineage and a transfer preserves
repository history while redirecting old locations. Neither relationship by
itself establishes that a new crate namespace or fork is the community-
recognized compatible successor.

**Implication:** Replacement records need source lineage, namespace identity,
maintainer declaration, compatibility, migration, publication authority, and
adoption evidence.

**Confidence:** High.

### FERRIUM-575: yanks are release-level disposition, not stewardship grades

**Sources:** Cargo yanking documentation and measured package histories.

**Observed behavior:** Yanking prevents new resolution while preserving
existing locked builds and is reversible. Selected packages had different
numbers of historical yanks, including zero and seventeen.

**Implication:** Record each yank and reason where available. Counts alone
cannot compare maintainer quality because responsive correction can increase
the count.

**Confidence:** High.

### FERRIUM-576: funding is context rather than continuity proof

**Sources:** GitHub funding-link observations.

**Observed behavior:** Seven repositories exposed funding links. The data did
not identify allocation, duration, maintainer compensation, service
commitments, or succession.

**Implication:** Funding may explain capacity and support intervention
decisions, but it must not become a maintained/unmaintained proxy.

**Confidence:** High.

### FERRIUM-577: renewable diffs are the central stewardship primitive

**Sources:** owner, policy, publication, repository, work, and lifecycle
observations.

**Observed behavior:** Several decisive facts are changes: owner additions or
removals, repository transfer, trustpub-only adoption, new publisher identity,
archive, security-policy addition, seeking-maintainer statement, advisory, or
successor declaration.

**Implication:** OSPREY must retain immutable stewardship snapshots and emit
typed changes with source and uncertainty. Point-in-time dashboards cannot
recover historical transitions after the fact.

**Confidence:** High.

## Decision

### Adopt now

- Adopt the multidimensional stewardship record and immutable renewal diff.
- Keep registry owner, publisher, repository owner, contributor, reviewer,
  code owner, security contact, sponsor, and successor as distinct roles.
- Preserve `unknown` rather than filling absent evidence with a health label.
- Treat explicit declarations, archives, transfers, owner changes, trusted
  publishing changes, yanks, RustSec advisories, and successor statements as
  typed events.
- Scope activity and response evidence to the crate path where possible.
- Use release age, concentration, unanswered issues, and funding only as
  review triggers with documented windows and limitations.
- Require contact evidence before an inferred-unmaintained claim.

Owner: FERRIUM.

Expected validation: ECOS-Q06 security-response and provenance evidence,
ECOS-Q07 platform stewardship, ECOS-Q10 selection comparisons, ECOS-Q11
profile renewals, and ECOS-Q12 intervention decisions.

Non-goals: ranking maintainers, creating a universal bus-factor score,
declaring projects abandoned, replacing crates.io or RustSec governance, or
automatically promoting forks.

### Prototype behind a compatibility boundary

- periodic crates.io owner and trustpub policy snapshots;
- per-release human/workflow provenance and yank-event ingestion;
- canonical repository transfer, rename, archive, and fork lineage;
- package-to-monorepo-path mapping;
- bounded activity, review, and responsiveness windows;
- lifecycle-statement and seeking-maintainer capture;
- RustSec unmaintained advisory ingestion; and
- successor graphs with compatibility and migration evidence.

### Reject or defer

- one maintenance-health score;
- fixed days-since-release abandonment thresholds;
- downloads, stars, open-issue totals, funding, or organization ownership as
  maintenance proof;
- owner count or contributor count as a literal bus factor;
- repository-wide activity as proof for every package in a monorepo;
- absence of GitHub approval records as proof that review did not occur;
- automatic unmaintained claims without declaration or failed contact; and
- automatic successor designation from fork or name similarity.

## Role review

### Rust Safety Steward

Accepts explicit release authority, security contact, lifecycle, and successor
lineage. Requires ownership changes, compromised publication paths, unsafe
maintenance boundaries, and emergency response to remain visible rather than
hidden by a score.

### Compiler Performance Engineer

Accepts bounded snapshots and path-scoped queries. Requires renewal cost,
rate limits, monorepo mapping, and graph-scale ingestion to be measured before
continuous portfolio scanning.

### Interop Boundary Auditor

Accepts transfer and successor lineage as separate from compatibility.
Requires forks and replacements to retain crate identity, version, API,
feature, runtime, data, and migration evidence.

### AI Assurance Skeptic

Accepts explicit unknowns and RustSec's contact gate. Rejects AI-generated
abandonment, bus-factor, responsiveness, or successor claims inferred from
activity metrics alone.

### Ecosystem Strategist

Accepts stewardship evidence as a selection and intervention input. Requires
upstream contact and succession support before proposing FERRIUM-owned forks
or replacements.

### Rust Maintainer

Accepts stable/quiet as distinct from unmaintained and avoids punishing low
release frequency. Requires measurements to be bounded, respectful,
correctable, and linked to primary maintainer statements.

### Native Platform Adopter

Accepts renewal and succession evidence. Requires support claims to include
platform-specific maintainers, release coverage, native-provider ownership,
and response paths in later questions.

### Scope Keeper

Accepts Q05 as the stewardship evidence model. Security assurance, target
support, fragmentation cost, native dependency policy, stack selection, and
intervention remain closed.

### Validation Checker

Accepts exact registry authorities, release histories, eighteen repository
observations, path checks, RustSec snapshot identity, commands, and
limitations. Requires future renewals to preserve timestamps and diffs.

## Limitations

- Measurements are point-in-time public observations.
- GitHub team membership and private repository permissions were not observed.
- Owner-change chronology before this snapshot was not available.
- Commit attribution is affected by bots, rebases, squash merges, author
  metadata, and co-authorship.
- Pull-request review data excludes private, synchronous, issue, chat, mailing
  list, and direct-push review.
- Visible branch-protection data was not treated as complete because rulesets,
  permissions, and private policy can change visibility.
- Repository-wide one-year history was capped to the latest hundred commit
  nodes for concentration calculations.
- Responsiveness latency was not measured.
- Funding amount, duration, allocation, and service obligations were unknown.
- No maintainer was contacted; no implicit-unmaintained determination was
  attempted.
- RustSec absence is not proof of maintenance.
