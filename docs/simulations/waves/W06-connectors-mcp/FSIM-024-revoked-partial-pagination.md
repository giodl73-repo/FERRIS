# FSIM-024: Revoked Connector During Pagination

Wave: W06
Revision: 1
State: Simulated
Claim state: simulated

## Question

What does Ferris record when a connector returns one page of owner data and is
then revoked before the remaining pages are read?

## Locked fixture

- application: `forge`
- repositories and workspaces: three repositories in one authorized tenant
- source and change: read-only affected-scope query
- contracts and profiles: unchanged
- environment: paginated owner API through connector `K4`
- policy: connector revocation denies future calls immediately
- available evidence: page 1 returns repository `a`; continuation token exists;
  connector is revoked before page 2
- explicit unknowns: repositories and edges on later pages
- negative or matched control: complete pagination before revocation

Changing the fixture requires a new revision.

## Governing specifications

- CONNECTOR-001 failure behavior and owner semantics;
- TRUST-001 revocation; and
- EVIDENCE-001 result states.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Revocation record becomes effective before page 2 | TRUST-001 |
| Scope | Requested tenant scope is larger than observed page 1 | Pagination |
| Evidence | Page 1 is externally reported partial evidence | CONNECTOR-001 |
| Causality | Revocation causes collection stop, not an empty owner result | Failure semantics |
| Prediction | Unseen repository closure remains unknown | No empty-success |
| Validation | Evidence completeness is insufficient for narrowing | VALIDATION-001 |
| Planning | Widen from another owner source, request evidence, or block | Safe fallback |
| Resolution | No partial result is eligible as complete affected scope | CONNECTOR-001 |
| Trust/action | Future connector calls are denied; historical page remains attributable | Revocation |
| Public view | Shows partial, revoked, continuation outstanding, and unknown scope | VIEW-001 |

## Assertions

- [x] repository `a` evidence remains historical and attributable;
- [x] later pages are not treated as empty;
- [x] revocation stops future calls;
- [x] partial evidence cannot authorize narrowing; and
- [x] the complete control remains a separate root and result.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original failure, revocation, and partial-result rules produce one safe
trace.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
