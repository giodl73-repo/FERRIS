# Pulse 07: Platform-Environment Repeatable Enterprise Onboarding

Status: Authorized
Implementation authority: One disposable evidence attempt only

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
