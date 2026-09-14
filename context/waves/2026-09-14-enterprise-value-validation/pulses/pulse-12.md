# Pulse 12: Link-Complete Repeatable Enterprise Onboarding

Status: Complete; incomplete cross-platform
Implementation authority: Exhausted

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

## Result

The exact Windows link preflight passed. Both Windows consumers then planned,
executed their unchanged owner commands through Ferris, and produced receipts
that passed `verify`.

The Linux harness was transferred with CRLF line endings and stopped at
`set -o pipefail` before creating its `/opt` root, cloning a consumer, or
running Ferris. The cross-platform success gate therefore did not pass.

Windows aggregate identities were retained privately, and all Windows tools,
consumers, onboarding files, runtime files, the archive, and any Linux root
were removed. Pulse 12 is incomplete cross-platform evidence and exhausted.
