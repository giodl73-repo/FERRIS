# Private Enterprise Onboarding Pulse 04

Date: 2026-09-13
Status: Complete; incomplete at WSL Cargo-wrapper gate
Decision: Retain passing Windows baselines and preserve both consumers unchanged

## Scope

Pulse 04 authorized one fresh attempt with platform tool locations preflighted
before owner command invocation. It retained unchanged owner scripts and the
same removable native WSL PowerShell pathway.

Exact private identities, paths, revisions, timings, and raw output remain in
custody.

## Findings

### FERRIS-ONBOARD-013: Process-local tool binding fixed the Windows gate

The Windows preflight resolved Cargo and rustc 1.95.0 plus PowerShell 7.6.6
without a persistent user or machine environment change. Both unchanged
Windows owner commands then passed formatting, denied-warning Clippy,
locked/offline tests, metadata cardinality, and deterministic topology checks.

This is owner-baseline evidence, not Ferris onboarding evidence.

### FERRIS-ONBOARD-014: WSL inherited an unavailable Cargo wrapper

The WSL preflight resolved Cargo and rustc 1.95.0, Git 2.53.0, and native
PowerShell 7.6.6. Both owner commands nevertheless stopped at Clippy because
Cargo discovered an ancestor user configuration declaring `kache` as
`rustc-wrapper`, while that executable was unavailable in WSL.

The configuration was external to both consumers. The result is an
invocation-environment unavailability, not an owner correctness failure and not
a Ferris product failure.

### FERRIS-ONBOARD-015: Owner configuration authority remained intact

Pulse 04 did not disable or override the configured Cargo wrapper and did not
retry either WSL command. It stopped before tags, consumer mutation, onboarding
files, Action Plans, Ferris execution, receipts, or removal tests.

The temporary WSL PowerShell symlink, installation tree, and archive were
removed and verified absent. Both consumers remain clean at their custody
revisions.

### FERRIS-ONBOARD-016: Cross-platform onboarding remains not observed

The Windows baseline is now observed and passing. The WSL baseline remains
unavailable before compilation, so Ferris onboarding and complete removal
remain untested.

## Claim boundary

Pulse 04 provides Windows owner-baseline, WSL configuration-gate, and cleanup
evidence only. It provides no hosted-CI, official Ubuntu support, clean-runner,
support, production, affected-only, performance, savings,
workflow-replacement, onboarding, or removal evidence.

Authority and exact stop conditions are recorded in
[`Pulse 04`](../../context/waves/2026-09-11-enterprise-onboarding-proof/pulses/pulse-04.md).
The closeout review is
[`FERRIS Enterprise Onboarding Pulse 04 Role Review`](../plans/reviews/FERRIS_ENTERPRISE_ONBOARDING_PULSE_04_ROLE_REVIEW.md).
