# Pulse 10: MSVC-Bound Repeatable Enterprise Onboarding

Status: Complete; invalid during preflight
Implementation authority: Exhausted

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

## Result

The dependency-complete generator and exact Ferris revision built. The
process-local `VsDevCmd.bat` import then failed because the `cmd.exe` command
line had invalid quoting. The failure occurred before either consumer was
materialized, before runtime staging, and before planning or execution.

The tool-only disposable root was removed. Pulse 10 is invalid onboarding
evidence and exhausted.
