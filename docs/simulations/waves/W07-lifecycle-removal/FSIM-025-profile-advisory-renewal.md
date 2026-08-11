# FSIM-025: Profile Renewal After Advisory

Wave: W07
Revision: 1
State: Simulated
Claim state: simulated

## Question

What happens when a critical advisory affects an active dependency before the
selected platform profile reaches its scheduled expiry?

## Locked fixture

- application: `forge`
- repositories and workspaces: one service workspace
- source and change: no application source change
- contracts and profiles: profile `P12` is active for another 30 days
- environment: exact supported toolchain and provider
- policy: critical advisories trigger immediate renewal
- available evidence: advisory maps exactly to an active lockfile dependency
- explicit unknowns: whether a compatible patched release exists
- negative or matched control: advisory for a package absent from the lock and
  active closures

Changing the fixture requires a new revision.

## Governing specifications

- PLATFORM-001 assurance, renewal, and substitution;
- APPLICATION-001 lifecycle; and
- TRUST-001 consumer trust.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Advisory evidence changes assurance state | PLATFORM-001 renewal trigger |
| Scope | Exact affected lock and active closure | Profile identity |
| Evidence | Advisory source, time, mapping, and limitations retained | Assurance |
| Causality | Advisory match causes renewal, not proof of exploitability | Claim separation |
| Prediction | Patch availability remains unknown | No inferred substitution |
| Validation | Existing support evidence becomes stale or conditional | Profile lifecycle |
| Planning | Renew, substitute, mitigate, or block | PLATFORM-001 |
| Resolution | Owner selects an eligible response with rollback | RESOLUTION-001 |
| Trust/action | Prior profile cannot authorize future use if policy revokes eligibility | TRUST-001 |
| Public view | Shows early renewal trigger and exact affected closure | VIEW-001 |

## Assertions

- [x] scheduled expiry does not delay an emergency trigger;
- [x] advisory match is not relabeled exploitation evidence;
- [x] patch availability remains unknown until observed;
- [x] substitution requires migration and rollback evidence; and
- [x] the absent-package control does not affect profile eligibility.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original renewal and trust rules provide one unambiguous lifecycle trace.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
