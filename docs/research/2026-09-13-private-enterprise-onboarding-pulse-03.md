# Private Enterprise Onboarding Pulse 03

Date: 2026-09-13
Status: Complete; incomplete at Windows Cargo PATH gate
Decision: Preserve both private baselines unchanged after complete prerequisite
rollback

## Scope

Pulse 03 authorized one corrective successor to provision a removable native
Linux PowerShell prerequisite, resume the unchanged two-platform owner
baselines, and proceed to removable Ferris onboarding only if those baselines
passed.

Exact private identities, paths, revisions, timings, and raw output remain in
custody.

## Findings

### FERRIS-ONBOARD-009: The verified archive provided native Linux PowerShell

The official PowerShell 7.6.6 Linux x64 archive matched published SHA-256 digest
`ddbc4a2d113bbd46d283cfedcbcd117a70caefd7673f41f2b4e0000badf103bc`.
Its ELF x86-64 executable launched natively and reported PowerShell 7.6.6,
platform `Unix`, Ubuntu 26.04 LTS, and architecture `X64`.

This demonstrates a viable local prerequisite pathway. It does not establish
official Microsoft support for Ubuntu 26.04.

### FERRIS-ONBOARD-010: The Windows invocation environment lacked Cargo

Both unchanged Windows owner commands exited at their first `Get-Command cargo`
lookup because the inherited `PATH` did not contain the installed Cargo
directory. Formatting, Clippy, tests, metadata, and topology verification did
not run.

The result is an invocation-environment unavailability. It is not an owner
correctness failure and not a Ferris product failure.

### FERRIS-ONBOARD-011: The changed-input stop and rollback held

Pulse 03 did not modify `PATH` and retry. It stopped before WSL owner baselines,
tags, consumer mutation, onboarding files, Action Plans, Ferris execution,
receipts, or removal tests.

The temporary PowerShell symlink, exact installation tree, and downloaded
archive were removed and verified absent. Both private consumer worktrees
remain clean at their Pulse 01 revisions.

### FERRIS-ONBOARD-012: Onboarding remains not observed

Pulse 03 proved that the WSL PowerShell prerequisite can be installed,
identified, and removed within a bounded protocol. It did not observe Ferris
onboarding or removal value because the independent Windows baseline gate did
not start Cargo.

## Claim boundary

Pulse 03 provides prerequisite and stop-boundary evidence only. It provides no
hosted-CI, official Ubuntu support, clean-runner, support, production,
affected-only, performance, savings, workflow-replacement, onboarding, or
removal evidence.

Authority and exact stop conditions are recorded in
[`Pulse 03`](../../context/waves/2026-09-11-enterprise-onboarding-proof/pulses/pulse-03.md).
The closeout review is
[`FERRIS Enterprise Onboarding Pulse 03 Role Review`](../plans/reviews/FERRIS_ENTERPRISE_ONBOARDING_PULSE_03_ROLE_REVIEW.md).
