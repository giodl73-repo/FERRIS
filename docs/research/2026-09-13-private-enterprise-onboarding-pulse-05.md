# Private Enterprise Onboarding Pulse 05

Date: 2026-09-13
Status: Complete; incomplete at Cargo ancestor-config gate
Decision: Empty Cargo-home isolation is insufficient for these mounted paths

## Scope

Pulse 05 authorized one fresh attempt with an empty temporary WSL `CARGO_HOME`,
the unchanged owner commands, and complete removal of temporary environment
state.

Exact private identities, paths, revisions, timings, and raw output remain in
custody.

## Findings

### FERRIS-ONBOARD-017: The temporary Cargo home preserved user state

The isolated Cargo home began empty. The existing user Cargo configuration was
not copied, edited, or overridden and retained the same SHA-256 identity before
and after the attempt. The native WSL PowerShell and Rust 1.95 preflight passed.

### FERRIS-ONBOARD-018: Windows owner baselines passed again

Both unchanged Windows owner commands passed formatting, denied-warning Clippy,
locked/offline tests, metadata cardinality, and deterministic topology checks.
This confirms current Windows owner correctness but is not Ferris onboarding
evidence.

### FERRIS-ONBOARD-019: Cargo also discovered invocation-path ancestors

Both WSL commands still stopped at Clippy because Cargo found the unavailable
`kache` wrapper configuration above the invocation directory. Setting
`CARGO_HOME` alone does not suppress `.cargo` configuration discovered while
walking ancestors of the current directory.

Pulse 05 did not change the invocation directory or override configuration
after this result.

### FERRIS-ONBOARD-020: Cleanup and consumer immutability held

The temporary Cargo home, PowerShell symlink, installation tree, and archive
were removed and verified absent. Both consumers remain clean. No baseline tag,
onboarding file, Action Plan, Ferris execution, receipt, or removal test was
created.

## Claim boundary

Pulse 05 provides Windows owner-baseline, Cargo configuration-discovery, and
cleanup evidence only. It provides no hosted-CI, official Ubuntu support,
clean-runner, support, production, affected-only, performance, savings,
workflow-replacement, onboarding, or removal evidence.

Authority and exact stop conditions are recorded in
[`Pulse 05`](../../context/waves/2026-09-11-enterprise-onboarding-proof/pulses/pulse-05.md).
The closeout review is
[`FERRIS Enterprise Onboarding Pulse 05 Role Review`](../plans/reviews/FERRIS_ENTERPRISE_ONBOARDING_PULSE_05_ROLE_REVIEW.md).
