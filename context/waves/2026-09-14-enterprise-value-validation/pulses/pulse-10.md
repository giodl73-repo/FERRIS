# Pulse 10: MSVC-Bound Repeatable Enterprise Onboarding

Status: Authorized
Implementation authority: One disposable evidence attempt only

## Objective

Complete Pulse 09 with the already-installed Microsoft C++ linker environment
available explicitly to fresh Windows owner tests.

## Authority

The pulse may call the installed Visual Studio 2022 Build Tools
`VsDevCmd.bat` for x64 and import its output only into the invoking PowerShell
process. It may not persist environment changes or install prerequisites.

Windows Action Plans may inherit and approvals may allow exactly:

- `LIB`;
- `PATH`;
- `PATHEXT`;
- `SYSTEMROOT`; and
- `WINDIR`.

Before planning, staged PowerShell MUST resolve the exact Cargo and x64
`link.exe` executables under that environment, and `LIB` MUST be present.
Linux remains `PATH` only.

The pulse otherwise inherits Pulse 09's exact revisions, generator, root
request, planning, consumer-root execution, receipt verification, owner
commands, stop conditions, removal invariant, cleanup, and exclusions.

Success requires four successful executions and verified receipts, complete
onboarding removal, four passing post-removal owner commands, exact clean
revisions, and complete disposable cleanup.
