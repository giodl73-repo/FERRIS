# Pulse 16: Adoption, Ordinary Cargo, and Removal

Status: Authorized; implementation and validation pending
Implementation authority: Bounded to this document

## Goal and authority

Adopt one explicit profile marker into an isolated ordinary Cargo consumer,
prove owner Cargo still passes, remove the marker completely, prove exact
pre-adoption tree restoration, and freeze one canonical Removal Record.

This pulse authorizes only temporary-copy mutation, one explicit marker,
locked/offline owner tests, exact snapshots, and a repository-owned removal
record. It does not authorize product adoption/removal commands, committed
consumer mutation, registry changes, or deployment.

## Acceptance

- ordinary Cargo passes before, during, and after adoption;
- adoption adds only the explicit marker;
- removal deletes the marker and restores the exact original tree;
- the Removal Record names removed and retained evidence explicitly; and
- Windows and Unix repository gates pass.

## Evidence

- [Nine-role review](../../../../docs/plans/reviews/PULSE-16-REMOVAL-ROLE-REVIEW.md)
