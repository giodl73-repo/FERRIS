# Pulse 14: Renewal and Exact Rollback

Status: Authorized; implementation and validation pending
Implementation authority: Bounded to this document

## Goal and authority

Execute one real controlled renewal from pure-data `r1` to `r2` in an isolated
copy, validate the renewed owner workflow, restore the exact `r1` tree, and
validate the restored workflow and byte identity.

This pulse authorizes test-only copying and replacement in an OS temporary
directory, locked/offline owner tests, exact tree snapshots, and review. It
does not authorize mutation of committed fixtures, product lifecycle
commands, registry changes, deployment, or support.

## Acceptance

- the isolated initial tree exactly matches committed `r1`;
- renewal exactly matches committed `r2` and its owner test passes;
- rollback restores every `r1` path and byte and removes every `r2`-only path;
- the restored owner test passes; and
- Windows and Unix repository gates pass.

## Evidence

- [Nine-role review](../../../../docs/plans/reviews/PULSE-14-RENEWAL-ROLE-REVIEW.md)
