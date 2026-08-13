# Adoption, Rollback, and Removal

Status: Guidance
Implementation authority: None

## Lifecycle principle

Upstream support and consumer adaptations must be incremental, renewable,
reversible, and removable. Ordinary Cargo, editor, repository, CI, release,
and owner-native workflows must continue to work after Ferris-specific
artifacts are removed. This follows [CONTEXT.md](../../../CONTEXT.md), the
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md),
and the intervention rules in
[ECOS-Q12](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md).

## Adoption record

Before adoption, record:

- named consumer and current upstream owner;
- exact problem and why documentation or direct upstream use is insufficient;
- selected intervention: fixture, diagnostic, documentation, patch,
  stewardship support, adapter, or exceptional fork;
- supported source revision, release, toolchain, target, feature, platform,
  provider, native, and runtime scope;
- positive, negative, failure, unsupported, and version-skew evidence;
- install or integration steps and all changed owner-visible files;
- ordinary workflow baseline;
- operational, training, audit, security, CI, and maintenance burden;
- fallback, rollback, substitution, removal, expiry, and renewal owner; and
- approval and implementation authority.

This guide provides no implementation authority.

## Incremental adoption

Prefer adoption that:

1. starts with one repository, owner workflow, or packet;
2. adds no hidden manifest, resolver, registry, lockfile policy, service, or
   required Ferris runtime;
3. leaves direct owner-tool commands documented and functional;
4. allows side-by-side comparison with the existing workflow;
5. exposes unsupported and stale states before broader use;
6. measures setup, investigation, review, CI, and renewal cost;
7. defines a stop condition before expansion.

For native or interop work, preserve existing C++, build, debugging,
deployment, ABI, compliance, and support constraints. Migration must be
incremental and reversible, as required by the
[Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md)
and [Native Platform Adopter](../../../.roles/stakeholders/native-platform-adopter.md).

## Response and renewal

Each adopted artifact must define:

- response owner and expected response window;
- which failures require immediate disablement or rollback;
- supported upstream releases and renewal cadence;
- ownership, contribution-path, license, platform, and evidence renewal;
- version-skew behavior;
- regression and noise investigation responsibility;
- succession when the packet or adapter maintainer leaves;
- budget and support end date where funding is involved.

Renewal is a new evidence decision, not an automatic extension. Compare the
new state with the previous immutable snapshot. Preserve changed owners,
publication policy, source lineage, platform support, behavior, and
maintenance burden as typed diffs.

## Rollback

A rollback plan must identify:

- trigger: correctness regression, unsafe behavior, owner rejection,
  unsupported release, excessive noise, unavailable dependency, expired
  evidence, cost threshold, or governance breach;
- last known ordinary workflow;
- exact files, configuration, CI, service, data, and permissions to restore;
- artifact and cache handling;
- evidence and audit records retained after rollback;
- commands and expected results;
- owner responsible for the decision and execution;
- validation that rollback does not change Cargo or consumer correctness.

Rollback must be tested on the declared platforms and include failed rollback
behavior. A rollback that depends on the failing adapter or service is not a
valid rollback.

## Complete removal

Removing a packet, adapter, or stewardship artifact is capability cleanup.
If Ferris integration is partially or completely removed, PRODUCT-001 also
requires a canonical, versioned
[Removal Record](../../specs/FERRIS_PRODUCT_CONTRACT.md#removal-record).
The record captures authority, Ferris inventory, action freeze and drain,
evidence export, cleanup, owner-native verification, retained evidence,
residual state, and recovery ownership. The checklist below supplies evidence
to that record rather than replacing it.

Removal is complete only when:

- Ferris-specific hooks, configuration, credentials, permissions, CI steps,
  branches, services, caches, and generated artifacts are removed or
  intentionally retained under a named policy;
- direct Cargo and owner-native commands work;
- manifests, lockfiles, source selection, features, targets, release process,
  and deployment semantics match the intended owner state;
- no hidden compatibility layer or shadow governance remains;
- retained evidence has a retention and deletion owner;
- documentation points to the ordinary workflow;
- consumer correctness and required validation remain unchanged.

## Adapter decision rules

Use a consumer adapter only when all conditions hold:

1. A named consumer has a measured gap.
2. Existing owner contracts cannot close it directly.
3. The adapter direction and owning repository are explicit.
4. Allocation, copying, field loss, ordering, backpressure, cancellation,
   panic, runtime, ABI, and threading consequences are recorded as applicable.
5. Feature, target, provider, native, compiler, and version assumptions are
   explicit.
6. Positive and negative semantic tests exist.
7. Expiry, renewal, substitution, rollback, and removal are funded and owned.
8. The adapter is not presented as a product-neutral standard.
9. A separately approved implementation pulse exists.

Prefer a local, thin, removable adapter. Reject a generic Ferris adapter crate
without a named consumer.

## Fork decision rules

A fork is exceptional. Do not fork merely because:

- a release is old;
- activity is concentrated or quiet;
- an issue or pull request has not received a prompt response;
- the project lacks funding, CODEOWNERS, or a visible team;
- a downstream feature is desired;
- a patch has not been accepted.

Consider a fork only when:

1. the capability is material to a named consumer;
2. current owner contact and contribution routes were attempted respectfully
   and recorded, or an explicit lifecycle declaration exists;
3. documentation, direct contribution, stewardship support, substitution, and
   a removable adapter cannot meet the need;
4. license, trademark, namespace, publication, security, and source-lineage
   obligations are understood;
5. compatibility and migration evidence is complete;
6. governance, maintainers, release engineering, security response, platform
   support, funding, and succession are committed;
7. consumers can migrate back or to another owner;
8. the fork is not called a successor without explicit declaration and
   adoption evidence;
9. retirement and archival conditions are defined;
10. organizational approval and separate implementation authority exist.

The preferred outcome remains current-owner stewardship or a community-owned
successor, not a permanent Ferris or Microsoft compatibility dialect. This
matches the [leadership opportunity map](../../leadership/MICROSOFT_RUST_UPSTREAM_OPPORTUNITY_MAP.md).

## Retirement

Retire an artifact when:

- the behavior no longer reproduces;
- upstream incorporates or supersedes it;
- the owner declines it and no active consumer need remains;
- evidence, license, support, or maintenance expires without renewal;
- burden exceeds the agreed threshold;
- a safer or simpler ordinary workflow replaces it;
- the consumer removes the capability.

Retirement records the reason, final supported version, replacement if any,
consumer migration, retained evidence, deletion schedule, and confirmation of
complete removal.

## Lifecycle acceptance

Adoption is acceptable only when one owner can demonstrate installation,
ordinary use, renewal, rollback, and complete removal with measured commands
and without changing Cargo authority or trapping the repository.
