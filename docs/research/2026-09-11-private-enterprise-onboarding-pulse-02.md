# Private Enterprise Onboarding Pulse 02

Date: 2026-09-11
Status: Complete; incomplete at WSL PowerShell capability gate
Decision: Preserve both private baselines unchanged and make no onboarding or
removal claim

## Scope

Pulse 02 authorized one bounded removable onboarding attempt for the two
custody-bound private consumers, publicly identified only as `EO-01` and
`EO-02`. It selected public Ferris revision
`b347dc34d62810f043122d0d279def316b890cf0` and replaced unavailable hosted
Ubuntu with local WSL 2 Ubuntu.

Exact private identities, paths, revisions, and raw preflight output remain in
custody.

## Findings

### FERRIS-ONBOARD-005: The selected WSL Rust toolchain was available

The local WSL 2 Ubuntu preflight observed Linux `x86_64`, Cargo 1.95.0, rustc
1.95.0, and Git 2.53.0. This established only tool presence. No owner command
or Ferris consumer operation ran.

### FERRIS-ONBOARD-006: The unchanged owner commands were unavailable

Both consumer correctness commands are repository-owned PowerShell scripts.
The WSL environment had no Linux `pwsh` in `PATH`, standard installation
locations, or the package database.

Running a Windows PowerShell executable through WSL would not prove Linux
execution. Translating the scripts or installing PowerShell would exceed the
authorized budget. The required Linux baseline was therefore `unavailable`,
not failed and not passed.

### FERRIS-ONBOARD-007: The pre-mutation stop boundary held

The pulse stopped before:

- either unchanged Windows or WSL owner baseline command;
- private repository mutation or baseline tagging;
- Ferris installation or a Ferris-owned onboarding file;
- planning input, Action Plan, entrypoint declaration, or approval;
- consumer `plan`, `go`, `verify`, or receipt creation; and
- removal or post-removal validation.

Both private repositories remain at their clean Pulse 01 revisions. There is no
partial onboarding to remove.

### FERRIS-ONBOARD-008: The product hypothesis remains not observed

This attempt does not show that Ferris onboarding succeeds or fails. It shows
that the selected local Linux environment could not run the unchanged
repository-owned correctness command under the approved no-install,
no-translation boundary.

## Claim boundary

Pulse 02 provides a bounded capability-gate result only. It provides no
hosted-CI, clean-runner, support, production, affected-only, performance,
savings, workflow-replacement, onboarding, or removal evidence.

Authority and exact stop conditions are recorded in
[`Pulse 02`](../../context/waves/2026-09-11-enterprise-onboarding-proof/pulses/pulse-02.md).
The closeout review is
[`FERRIS Enterprise Onboarding Pulse 02 Role Review`](../plans/reviews/FERRIS_ENTERPRISE_ONBOARDING_PULSE_02_ROLE_REVIEW.md).
