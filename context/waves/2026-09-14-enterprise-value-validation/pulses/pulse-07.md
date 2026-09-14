# Pulse 07: Platform-Environment Repeatable Enterprise Onboarding

Status: Complete; invalid before owner launch
Implementation authority: Exhausted

## Objective

Complete Pulse 06 with the non-secret Windows operating-system environment
required by the owner PowerShell runtime declared explicitly.

## Authority

Windows Action Plans may inherit and approvals may allow exactly:

- `PATH`;
- `SystemRoot`; and
- `WINDIR`.

Linux Action Plans continue to inherit and allow only `PATH`.

Before planning, each staged PowerShell runtime MUST launch under exactly its
platform declaration. Failure stops the pulse before Action Plan execution.

The pulse otherwise inherits Pulse 06's exact revisions, dependency-complete
generator, root request, planning, consumer-root execution, verification,
owner commands, stop conditions, removal invariant, cleanup, and exclusions.

## Success

Success requires four successful `go` invocations, four valid receipts,
complete onboarding removal, four passing post-removal owner commands, exact
clean consumer revisions, and complete disposable cleanup.

No product change, persisted consumer change, performance or savings claim, CI
authority, production claim, or support claim is authorized.

## Result

The dependency-complete tools built, both Windows consumers and runtimes were
materialized, and each runtime launched under exactly `PATH`, `SystemRoot`, and
`WINDIR`. Both planning commands and Action Plan generation succeeded.

Ferris rejected the first execution before owner launch with
`FERRIS-EXECUTION-ENVIRONMENT-UNSUPPORTED`. The execution contract requires
environment names to be uppercase, unique, and sorted; `SystemRoot` violated
that canonical representation. No owner lane, receipt, second consumer
execution, Linux materialization, or consumer mutation followed. All
disposable state was removed.

Pulse 07 is invalid onboarding evidence and exhausted.
