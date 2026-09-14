# Pulse 13: LF-Bound Cross-Platform Enterprise Onboarding

Status: Complete; invalid during Linux tool setup
Implementation authority: Exhausted

## Objective

Repeat the complete Pulse 12 protocol after proving the Linux harness is
LF-only before either platform materializes.

## Preflight

The exact harness bytes MUST contain zero carriage-return bytes and MUST parse
under native Ubuntu Bash. Failure stops before any checkout or prerequisite.

## Authority

The pulse inherits Pulse 12's exact Ferris and consumer revisions, generator,
PowerShell archive and digest, Windows MSVC environment, Linux `PATH`-only
environment, planning, Action Plans, execution, receipt verification, removal,
owner commands, stop conditions, cleanup, and exclusions.

Both platforms MUST rerun. Pulse 12 records are context only and MUST NOT count
as Pulse 13 success evidence.

Success requires four successful executions and verified receipts, complete
onboarding removal, four passing post-removal owner commands, exact clean
revisions, and complete disposable cleanup.

## Result

The byte-level LF and Bash syntax gates passed. Linux then created only its
tool root and exact Ferris clone before `cargo` resolution failed: the
non-login WSL process did not include `/root/.cargo/bin` in `PATH`.

No consumer was materialized and Windows did not start. The partial Linux root
and archive were removed. Pulse 13 is invalid onboarding evidence and
exhausted.
