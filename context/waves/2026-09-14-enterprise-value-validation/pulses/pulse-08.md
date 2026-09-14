# Pulse 08: Canonical-Environment Repeatable Enterprise Onboarding

Status: Complete; failed at owner execution
Implementation authority: Exhausted

## Objective

Complete Pulse 07 with environment names in the canonical representation
required by the existing execution contract.

## Authority

Windows Action Plans may inherit and approvals may allow exactly this sorted
list:

- `PATH`;
- `SYSTEMROOT`; and
- `WINDIR`.

The values are the same process values preflighted in Pulse 07. Linux remains
`PATH` only.

The pulse otherwise inherits Pulse 07's exact revisions, preflight under the
declared environment, generator, root request, planning, consumer-root
execution, verification, owner commands, stop conditions, removal invariant,
cleanup, and exclusions.

Success requires four successful executions and verified receipts, complete
onboarding removal, four passing post-removal owner commands, exact clean
revisions, and complete disposable cleanup.

## Result

The canonical environment passed contract validation, and Ferris launched the
first Windows owner lane. PowerShell initialized successfully, proving
`SYSTEMROOT` and `WINDIR` were sufficient for that runtime boundary.

The owner script then failed at `Get-Command cargo`: Cargo was absent from the
child-visible `PATH` even though `PATH` was declared and present in the
invoking PowerShell process. Ferris retained a failed receipt with complete
cleanup. No second execution or Linux materialization followed, and all
disposable state was removed.

Pulse 08 is failed onboarding evidence and exhausted. The next investigation
must bind and prove the exact uppercase environment presented to the Ferris
`go` process rather than infer it from PowerShell's case-insensitive `Path`
view.
