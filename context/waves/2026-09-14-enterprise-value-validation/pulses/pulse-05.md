# Pulse 05: Dependency-Complete Repeatable Enterprise Onboarding

Status: Complete; invalid before execution
Implementation authority: Exhausted

## Objective

Complete the Pulse 04 protocol after correcting the disposable Action Plan
generator's missing direct serialization dependency.

## Authority

The pulse may add `serde = "1"` only to the disposable generator manifest and
generate its disposable lockfile offline. The generator MUST build against
exact Ferris revision `b347dc34d62810f043122d0d279def316b890cf0` before
either consumer checkout is materialized.

After that gate passes, the pulse inherits Pulse 04's exact consumer revisions,
root `ferris-onboarding-request.json`, native runtime staging, planning, Action
Plan construction, `go`, receipt verification, owner commands, stop conditions,
removal invariant, platform scope, cleanup, and exclusions.

## Success

Success requires four verified execution receipts, complete removal of
`.ferris/` and the root request, four passing post-removal owner commands, exact
clean consumer revisions, and complete removal of all disposable material.

Any failed gate stops the pulse without same-pulse repair or retry. This pulse
grants no product change, persisted consumer change, performance or savings
claim, CI authority, production claim, or support claim.

## Result

The corrected generator and exact Ferris revision built successfully. Both
Windows consumers were materialized, native PowerShell runtimes launched, and
`plan` and root-request `federated-plan` succeeded. Both Action Plan generators
emitted their required files.

The first `go` invocation ran from the public Ferris worktree instead of the
`EO-01` consumer root. Ferris therefore correctly reported
`FERRIS-EXECUTION-FILE-UNAVAILABLE`; the generated file existed only in the
consumer. No owner lane, receipt, second `go`, Linux materialization, or
consumer mutation followed. All disposable state was removed.

Pulse 05 is invalid onboarding evidence and exhausted. A successor must bind
each `go` invocation's working directory to its disposable consumer root.
