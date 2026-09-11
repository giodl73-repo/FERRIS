# Private Enterprise Onboarding Baseline Attempt

Date: 2026-09-11
Status: Complete; incomplete at hosted capability gate
Decision: Retain two private Windows-proven owner baselines, but do not freeze
onboarding cutoffs or begin Ferris onboarding

## Scope

Pulse 01 created exactly two private enterprise-shape repositories. Public
records use only `EO-01` and `EO-02`; exact host, repository identities,
revisions, paths, and raw output remain in private custody.

Neither repository contains a Ferris contract, adapter, output, Action Plan, or
dependency. Their purpose is to establish owner truth before a possible later
onboarding experiment.

## Findings

### FERRIS-ONBOARD-001: Owner-native Windows baselines passed

`EO-01` passed formatting, denied-warning Clippy, locked/offline tests for its
three-package Cargo workspace, Cargo metadata cardinality, and byte-stable
topology regeneration.

`EO-02` passed the same owner checks independently for three separate Cargo
workspaces. Its application relationship graph remained owner data and did not
combine Cargo resolution.

### FERRIS-ONBOARD-002: Removal was defined before adoption

Each repository documents the files and owner commands that a later onboarding
must leave unchanged. A later product adapter must be completely removable
while the owner validation remains green.

This is a documented baseline invariant, not yet a completed removal test,
because onboarding was not authorized.

### FERRIS-ONBOARD-003: Hosted Ubuntu was unavailable

The enterprise host recognized both checked-in workflows but reported
repository Actions disabled. Repository-level enable attempts did not change
that enforced state. No hosted job launched, so Ubuntu owner behavior is not
observed.

This state is `unavailable`, not `failed` and not `passed`.

### FERRIS-ONBOARD-004: The stop boundary held

No immutable owner-baseline tag was created. The pulse did not introduce
alternate CI, privileged organization changes, reusable credentials, a third
repository, or Ferris onboarding.

Pulse 02 remains proposed and unauthorized. A future decision must first choose
an owner-approved two-platform execution environment or explicitly revise the
two-platform requirement in a separate governance action.

## Claim boundary

The two repositories are useful private owner-native development baselines.
They are not qualified onboarding consumers and provide no production,
support, performance, affected-only, or savings evidence.

Authority and detailed results are recorded in
[`Pulse 01`](../../context/waves/2026-09-11-enterprise-onboarding-proof/pulses/pulse-01.md).
The closeout review is
[`FERRIS Enterprise Onboarding Baselines Role Review`](../plans/reviews/FERRIS_ENTERPRISE_ONBOARDING_BASELINES_ROLE_REVIEW.md).
