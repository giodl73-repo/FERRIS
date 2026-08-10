# FSIM-012: Material Projection Inconsistency

Wave: W03
Revision: 1
State: Retraced
Claim state: simulated

## Question

Can Ferris plan from an Ownership Map and an Identity Ledger that disagree
about the same package owner in one immutable root?

## Locked fixture

- application: `forge`
- repositories and workspaces: workspaces `api` and `client`
- source and change: package `api-core` changes
- contracts and profiles: unchanged
- environment: unchanged
- policy: exact reviewed owner mappings may select the smallest safe boundary
- available evidence: root `R40` records `api-team` as owner of `api-core`;
  the Ownership Map reports `api-team` but the Identity Ledger projection
  reports `client-team` without a conflict diagnostic
- explicit unknowns: whether the omission is engine corruption or a
  non-deterministic projection defect
- negative or matched control: a separately derived repository-level
  projection that explicitly marks the relationship unknown

Changing the fixture requires a new revision.

## Governing specifications

- FOREST-003 consistency rules and failure and fallback;
- PLANNING-001 projection consistency prerequisite; and
- VIEW-001 exit classes.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | `api-core` Change Record is valid | FOREST-002 |
| Scope | Exact owner boundary is disputed | Canonical owner record remains source |
| Evidence | Map and ledger disagree over `R40` | FOREST-003 inconsistency |
| Causality | Projection conflict cannot rewrite canonical ownership | Source links required |
| Prediction | No narrowing is eligible from the inconsistent ledger | FSIM-SCR-007 |
| Validation | Mandatory dependent coverage is incomplete | Unknown cannot aggregate to success |
| Planning | A plan consuming the conflicting pair is blocked | PLANNING-001 |
| Resolution | No work-reducing candidate may use the inconsistent result | FSIM-SCR-007 |
| Trust/action | Approval and action are ineligible | Material inconsistency |
| Public view | Projection contradiction is `internal`; truthful owner conflict would be `incomplete` or `blocked` | VIEW-001 |

## Assertions

- [x] the ledger contradiction does not rewrite canonical ownership;
- [x] the inconsistent projection cannot feed planning or action;
- [x] the negative control may widen from explicitly unknown evidence;
- [x] coarser fallback must be independently derived; and
- [x] engine invariant failure remains distinct from conflicting owner facts.

## Simulation issues

- `FSIM-SI-008`.

## Specification changes

- `FSIM-SCR-007`.

## Retrace

The locked fixture now blocks the inconsistent projection. The negative
control can conservatively widen only because it independently and explicitly
represents the unknown relationship.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
