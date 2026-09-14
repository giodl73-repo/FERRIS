# Native-Linux Enterprise Baseline

Date: 2026-09-14
Status: Complete
Decision: Accept the two private consumers as cross-platform owner baselines

## Scope

Two custody-bound private consumers, publicly identified only as `EO-01` and
`EO-02`, were evaluated at their exact existing revisions. Windows used the
clean custody checkouts. Linux used temporary local-only Git clones in native
WSL storage outside the Windows user-profile Cargo configuration hierarchy.

No Ferris onboarding file, Action Plan, execution, or receipt entered either
consumer.

## Findings

### FERRIS-VALUE-001: Native clones preserved source identity

Each Linux clone matched its source `HEAD` and tracked-tree identity, used only
a local custody path as its origin, and began clean. No remote fetch occurred.

### FERRIS-VALUE-002: All four owner baselines passed

Both unchanged owner commands passed on Windows and native Linux with Rust
1.95. The checks covered formatting, denied-warning Clippy, locked/offline
tests, metadata cardinality, and deterministic topology integrity.

This establishes owner correctness on the two local platforms. It does not
establish hosted-CI equivalence or Ferris behavior.

### FERRIS-VALUE-003: Invalid timing evidence was discarded

The WSL elapsed-time wrapper produced invalid arithmetic values after the
commands passed. Those values were excluded. The command results remain
behavioral evidence, but this pulse produces no latency or performance
measurement.

### FERRIS-VALUE-004: Cleanup and source immutability held

Both native clones and all build output were deleted. The temporary native
PowerShell installation, symlink, and archive were removed. Both custody source
checkouts remained clean at their exact revisions.

## Decision

The first enterprise-value gate passes. The consumers are suitable for a
separately authorized pinned Ferris onboarding and complete-removal proof.

This result provides no onboarding, affected-only, CI replacement, production,
support, performance, savings, or commercial evidence.

Authority is recorded in
[`Pulse 01`](../../context/waves/2026-09-14-enterprise-value-validation/pulses/pulse-01.md).
The closeout review is
[`FERRIS Native-Linux Enterprise Baseline Role Review`](../plans/reviews/FERRIS_NATIVE_LINUX_ENTERPRISE_BASELINE_ROLE_REVIEW.md).
