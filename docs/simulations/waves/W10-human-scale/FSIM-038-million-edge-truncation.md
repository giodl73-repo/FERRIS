# FSIM-038: Million-Edge Projection Truncation

Wave: W10
Revision: 1
State: Retraced
Claim state: simulated

## Question

How does Ferris summarize a graph whose authorized result exceeds one million
edges when an omitted page contains a mandatory unknown native dependency?

## Locked fixture

- application: `forge`
- repositories and workspaces: federated portfolio with 1.4 million projected
  edges
- source and change: shared native SDK change
- contracts and profiles: mandatory native validation
- environment: projection budget returns 100,000 edges per page
- policy: mandatory unknowns must remain visible in every summary
- available evidence: first page has ordinary dependencies; page 9 contains an
  unknown native edge affecting release
- explicit unknowns: exact total beyond authorized filters
- negative or matched control: complete small projection

Changing the fixture requires a new revision.

## Governing specifications

- FOREST-003 projection result and failure and fallback;
- VIEW-001 bounded output; and
- VALIDATION-001 unknown fallback.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Native SDK Change Record | FOREST-002 |
| Scope | Full authorized application projection | SCOPE-001 |
| Evidence | Immutable result has stable ordering and continuation | FSIM-SCR-021 |
| Causality | Pagination does not remove omitted native consequence | Source aggregate |
| Prediction | No narrower claim uses unseen pages | Safe fallback |
| Validation | Mandatory unknown native dimension appears in first summary | Severity preservation |
| Planning | Plan widens or blocks despite detail on page 9 | VALIDATION-001 |
| Resolution | Truncated result cannot be success-shaped | FOREST-003 |
| Trust/action | No release action proceeds on hidden unknown | Safety boundary |
| Public view | Shows counts, highest omitted severity, unknown, and continuation | VIEW-001 |

## Assertions

- [x] continuation binds the same immutable result;
- [x] page order is stable;
- [x] mandatory unknown appears in the first human summary;
- [x] exact unknown total remains unknown rather than fabricated; and
- [x] the small control requires no truncation.

## Simulation issues

- `FSIM-SI-022`.

## Specification changes

- `FSIM-SCR-021`.

## Retrace

The fixture now preserves the release-blocking unknown before the operator
retrieves page 9.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
