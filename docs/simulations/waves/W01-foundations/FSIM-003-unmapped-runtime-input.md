# FSIM-003: Unmapped Shared Runtime Input

Wave: W01
Revision: 1
State: Retraced
Claim state: simulated

## Question

What happens when a changed application input is outside Cargo ownership and
has no reviewed mapping to packages or tests?

## Locked fixture

- Application `ledger-cli` has the two workspaces from FSIM-001.
- Root file `shared/currency-rules.json` changes.
- Both applications may read the file at runtime.
- Cargo metadata contains no dependency on the file.
- The Application Definition declares it as application runtime data but does
  not map it to a component or validation activity.
- A full application validation procedure exists.
- No action or execution is requested.

Negative control: a mapped runtime file with an approved mapping to one
component and one integration-test family.

## Governing specifications

- APPLICATION-001 runtime-data declarations;
- FOREST-002 Change Record;
- SCOPE-001 unknown mapping and widening;
- VALIDATION-001 non-Cargo input fallback;
- PLANNING-001 conservative closures; and
- VIEW-001 explanation.

## Initial hand trace

The specs required the “smallest safe owner boundary” but allowed multiple
interpretations: repository containing the file, both workspaces, or the whole
declared application.

Initial issue: FSIM-SI-002.

## Retraced expected behavior

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Change Record identifies application runtime data outside Cargo ownership | FOREST-002 |
| Scope | Exact file scope is known; component/test mappings are unknown | SCOPE-001 |
| Widening | Package and workspace candidates cannot establish complete runtime coverage, so selection widens to the declared application full-reference boundary | SCOPE-001 |
| Evidence | Missing mapping is retained as not observed, not as an empty dependency set | EVIDENCE-001 |
| Validation | Full application runtime and mandatory repository validation are selected | VALIDATION-001 |
| Planning | Separate owner commands remain, joined by one application-level fallback reason | PLANNING-001 |
| Public view | Explanation names the changed file, missing mapping, attempted boundaries, and application fallback | VIEW-001 |

The mapped-file control may select its declared component and integration-test
family while retaining mandatory gates and selected/full-reference comparison.

## Assertions

- [x] A non-Cargo file cannot produce empty package impact and success.
- [x] Widening attempts are ordered and explained.
- [x] The first safe boundary, not merely the nearest filesystem parent, wins.
- [x] The mapped negative control permits reviewed narrowing.
- [x] No execution occurs.

## Specification changes

- FSIM-SCR-001; and
- FSIM-SCR-002.

## Claim boundary

Full-reference selection is a simulated policy result, not evidence that any
runtime path actually reads the file.
