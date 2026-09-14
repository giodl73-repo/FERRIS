# Pulse 09: Windows Command-Discovery Enterprise Onboarding

Status: Complete; failed at owner execution
Implementation authority: Exhausted

## Finding

An isolated cleared-environment probe reproduced Pulse 08's failure with
`PATH`, `SYSTEMROOT`, and `WINDIR`. Adding the existing non-secret `PATHEXT`
value made staged PowerShell resolve
`C:\Users\giodl\.cargo\bin\cargo.exe` by basename.

## Authority

Windows Action Plans may inherit and approvals may allow exactly this sorted
list:

- `PATH`;
- `PATHEXT`;
- `SYSTEMROOT`; and
- `WINDIR`.

Before planning, staged PowerShell MUST launch and resolve the exact Cargo
executable under only those variables. Linux remains `PATH` only.

The pulse otherwise inherits Pulse 08's exact revisions, generator, root
request, planning, consumer-root execution, receipt verification, owner
commands, stop conditions, removal invariant, cleanup, and exclusions.

Success requires four successful executions and verified receipts, complete
onboarding removal, four passing post-removal owner commands, exact clean
revisions, and complete disposable cleanup.

## Result

The exact-environment preflight resolved Cargo, planning succeeded, and the
first owner lane progressed through formatting and Clippy. Fresh `cargo test`
then failed because `link.exe` was not visible in the cleared environment.
Ferris retained a failed receipt with complete cleanup.

Visual Studio 2022 Build Tools and the x64 linker are installed, but their
process-local developer environment was not imported or declared. No second
execution or Linux materialization followed, and all disposable state was
removed.

Pulse 09 is failed onboarding evidence and exhausted.
