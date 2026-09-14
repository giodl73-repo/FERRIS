# Pulse 09: Windows Command-Discovery Enterprise Onboarding

Status: Authorized
Implementation authority: One disposable evidence attempt only

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
