# Pulse 05: Dependency-Complete Repeatable Enterprise Onboarding

Status: Authorized
Implementation authority: One disposable evidence attempt only

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
