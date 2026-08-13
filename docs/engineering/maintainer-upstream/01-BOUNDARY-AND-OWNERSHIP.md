# Boundary and Ownership

Status: Guidance
Implementation authority: None

## Boundary statement

Ferris owns its evidence schema, local research fixtures, packet records, and
later approved adapter behavior. It does not own Cargo resolution, rustc
architecture, rustc-perf admission, crate governance, standards, platform
behavior, or consumer deployment decisions. The detailed program boundary is
defined in the
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md).

An evidence record is not authority. A successful reproduction is not
approval. A complete packet is not permission to post. Funding is not
governance. A fork is not a successor. An adapter is not a standard.

## Owner discovery

Discover owners before deciding the intervention:

1. Identify the exact artifact: repository, package, release, target, feature,
   benchmark, specification, native dependency, or consumer workflow.
2. Find the canonical upstream home and its current contribution instructions.
3. Separate registry publication authority from source, review, release,
   security, support, and standards authority.
4. For monorepos, map the crate or component to its path and path-scoped work.
5. Record repository transfer, rename, archive, fork, and successor lineage
   separately.
6. Identify the practical intake route: issue tracker, Zulip or team process,
   contribution template, benchmark procedure, RFC process, working group, or
   maintainer contact.
7. Record the observation date and facts that must be renewed before posting.

The multidimensional model is required because crates.io owners are
publication authorities, not a complete maintainer roster, and repository
activity can overstate crate-path activity. See
[Rust maintenance and stewardship](../../research/2026-08-09-rust-maintenance-stewardship.md).

## Routing record

Before drafting an upstream request, record:

| Field | Required content |
|---|---|
| Subject | Exact package, tool, benchmark, standard, or owner workflow |
| Current owner | Named project, team, repository, or standards body |
| Authority type | Source, review, release, publication, security, support, platform, or consumer |
| Canonical home | Repo-relative evidence or an authoritative link already cited by the source documents |
| Intake path | Current owner-native contribution or decision process |
| Maintainer question | One bounded question the owner can answer |
| Renewal date | When ownership and intake facts were last checked |
| Alternatives | Documentation, fixture, diagnostic, patch, support, adapter, defer, or decline |
| External approval | Approver and state; absent means no posting |

When the owner is uncertain, request routing help with a minimal neutral
summary. Do not send a large patch to force ownership discovery.

## Owner-specific adaptations

### rustc

Route compiler semantics, diagnostics, tests, and implementation to the
appropriate rustc team or component owner. A packet should identify the
compiler phase or query only when evidence supports it; otherwise present the
observed boundary and ask for classification. Compiler acceptance does not
prove behavioral correctness or soundness.

### Cargo

Cargo remains authoritative for dependency resolution, workspace membership,
features, targets, profiles, fingerprints, freshness, build units, and local
scheduling. Begin from an accepted issue or explicit owner interest. Use
Cargo's requested test, benchmark, or documentation form, and distinguish
Cargo runtime from rustc compilation. See the Cargo checklist in the
[contribution packet specification](../../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).

### rustc-perf

Ask whether the case is useful as a benchmark before investing in a complete
registration when usefulness is uncertain. Use upstream profile, scenario,
and metric vocabulary; follow the benchmark split, manifest, configuration,
license, lockfile, local timing, and collector procedures in the
[contribution packet specification](../../specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).
Only an authorized upstream owner requests an official perf run.

### Crates and ecosystem tools

Respect the crate's repository, governance, release policy, MSRV, supported
targets, feature policy, security route, and maintenance capacity. Do not
infer abandonment from release age, issue totals, activity, owner count,
downloads, or funding. Offer the smallest useful fixture, diagnostic,
documentation change, compatibility evidence, or focused patch.

### Standards and shared contracts

Use the current standards body, project, or community process. Ferris may
contribute evidence of a repeated, product-neutral need, but it must not
declare an ecosystem standard from a consumer-specific adapter or a single
case. Typebook/RUNE must remain product-neutral and independently usable.

## Stewardship support without takeover

Permitted support, after the applicable approval, may include:

- paid maintainer and review time;
- release engineering and cross-platform testing;
- audit remediation under current owner direction;
- documentation and contributor onboarding;
- succession planning and emergency-access work;
- renewable owner, TrustPub, release, yank, archive, transfer, lifecycle, and
  successor evidence;
- compatibility and migration evidence for owner-declared alternatives; and
- preparation of contact or contribution packets.

Support must not:

- rank maintainers or publish a universal maintenance score;
- infer an unmaintained state without declaration or failed contact;
- assume control because a project is quiet, concentrated, or underfunded;
- condition support on adopting Ferris governance;
- silently create a Microsoft or Ferris fork;
- replace the owner's review, release, security, or succession decisions; or
- create an indefinite obligation without renewal and exit terms.

These controls follow the stewardship findings and the intervention policy in
[ECOS-Q12](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md).

## Decision rights

| Decision | Authority |
|---|---|
| Is the issue in upstream scope? | Current upstream owner |
| Is the reproducer representative? | Packet maintainer proposes; owner confirms usefulness |
| May evidence be public? | Evidence owner and organizational approver |
| May an external post or PR be created? | Explicit organizational approval plus owner-native intake |
| Is a benchmark admitted? | Benchmark owner |
| Is a patch architecturally acceptable? | Upstream reviewer or team |
| Is a crate supported by a consumer? | Consumer repository or profile owner |
| Is stewardship support renewed? | Current maintainers and support sponsor |
| Is a successor recognized? | Maintainer/community declaration plus compatibility and migration evidence |
| May Ferris implement an adapter? | Separate specification, validation, role review, and implementation pulse |

## Boundary acceptance

The boundary is acceptable only when:

- every claim and action has a named owner;
- current owner-native workflow remains usable without Ferris;
- uncertainty is visible rather than filled with inferred authority;
- a decline or redirect does not strand the consumer;
- support has response, renewal, and retirement rules; and
- removal leaves no hidden manifest, resolver, registry, service, or
  governance dependency.
