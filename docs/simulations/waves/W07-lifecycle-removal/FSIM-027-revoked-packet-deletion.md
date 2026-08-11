# FSIM-027: Revoked Packet with Partial Deletion

Wave: W07
Revision: 1
State: Retraced
Claim state: simulated

## Question

How is a previously accepted external evidence packet represented after
revocation when one retained replica cannot be deleted?

## Locked fixture

- application: `forge`
- repositories and workspaces: one producer repository
- source and change: packet `E44` was submitted and accepted by owner `O1`
- contracts and profiles: later evidence invalidates packet eligibility
- environment: internal archive, external owner, and backup replica
- policy: revoke future use and delete private packet copies where permitted
- available evidence: internal copy deleted; external owner retains historical
  reference; backup deletion fails
- explicit unknowns: time to repair backup deletion
- negative or matched control: deletion succeeds on every governed replica

Changing the fixture requires a new revision.

## Governing specifications

- FERRIS-001 lifecycle;
- TRUST-001 revocation and deletion; and
- GOVERNANCE-001 audit.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Revocation and deletion request occur after acceptance | Lifecycle events |
| Scope | Internal, external-reference, and backup data classes | Exact deletion scope |
| Evidence | Acceptance reference, revocation, and per-replica deletion results coexist | FSIM-SCR-015 |
| Causality | Revocation changes future eligibility, not historical acceptance | TRUST-001 |
| Prediction | Backup repair time remains unknown | No invented completion |
| Validation | Packet is ineligible for future claims | Revoked eligibility |
| Planning | Recovery targets failed backup deletion only | Bounded follow-up |
| Resolution | Publication remains accepted historically; retention is deletion partial | Lifecycle facets |
| Trust/action | Future packet use denied | Revocation |
| Public view | Shows accepted, revoked, deletion partial, and recovery owner together | FERRIS-001 |

## Assertions

- [x] prior acceptance is not rewritten;
- [x] acceptance does not imply current eligibility;
- [x] failed backup deletion remains visible;
- [x] external historical reference is distinct from retained private bytes; and
- [x] the complete-deletion control has a distinct retention state.

## Simulation issues

- `FSIM-SI-016`.

## Specification changes

- `FSIM-SCR-015`.

## Retrace

The packet now carries independent publication, eligibility, and retention
states without losing historical or current truth.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
