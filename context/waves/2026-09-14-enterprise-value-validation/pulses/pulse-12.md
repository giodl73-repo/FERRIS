# Pulse 12: Link-Complete Repeatable Enterprise Onboarding

Status: Authorized
Implementation authority: One disposable evidence attempt only

## Objective

Complete Pulse 11 with the operating-system temporary directories required by
the installed MSVC linker.

## Authority

Windows Action Plans may inherit and approvals may allow exactly:

- `LIB`;
- `PATH`;
- `PATHEXT`;
- `SYSTEMROOT`;
- `TEMP`;
- `TMP`; and
- `WINDIR`.

Before either consumer is materialized, the staged runtime MUST resolve Cargo
and `link.exe`, and a disposable Rust source MUST compile and link successfully
under exactly those variables. The source and executable MUST then be removed.
Linux remains `PATH` only.

The pulse otherwise inherits every Pulse 11 input, process-local MSVC import,
command, gate, stop condition, removal invariant, cleanup requirement, and
exclusion.

Success requires four successful executions and verified receipts, complete
onboarding removal, four passing post-removal owner commands, exact clean
revisions, and complete disposable cleanup.
