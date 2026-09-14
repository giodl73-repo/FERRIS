# Pulse 11: Corrected MSVC-Import Enterprise Onboarding

Status: Authorized
Implementation authority: One disposable evidence attempt only

## Objective

Complete Pulse 10 with the independently proven `VsDevCmd.bat` invocation
syntax.

## Authority

The pulse may:

- prepend the installed Visual Studio Installer directory to `PATH` only for
  the environment-import process so `vswhere.exe` resolves; and
- invoke `cmd.exe /d /c` with one command string containing the quoted
  `VsDevCmd.bat` path, x64 arguments, and `set`.

The resulting `PATH` and `LIB` values remain process-local. Windows Action
Plans retain exactly `LIB`, `PATH`, `PATHEXT`, `SYSTEMROOT`, and `WINDIR`;
Linux remains `PATH` only.

The pulse otherwise inherits every Pulse 10 input, preflight, command, gate,
stop condition, removal invariant, cleanup requirement, and exclusion.

Success requires four successful executions and verified receipts, complete
onboarding removal, four passing post-removal owner commands, exact clean
revisions, and complete disposable cleanup.
