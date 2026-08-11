# FSIM-043: Query Forest Component Failure Isolation

Wave: W11
Revision: 1
State: Simulated
Claim state: simulated

## Question

Does failure of the normalizer leave retained canonical roots, projections,
owner tools, and ordinary Cargo operation available without inventing new
evidence?

## Locked fixture

- application: `forge`
- repositories and workspaces: one Cargo workspace
- source and change: new adapter payload uses a malformed unsupported schema
- contracts and profiles: unchanged
- environment: Cargo and retained root `R70` remain available
- policy: invalid evidence cannot enter a new root
- available evidence: raw adapter payload, normalization failure, retained
  root `R70`, and owner-native commands
- explicit unknowns: intended meaning of malformed fields
- negative or matched control: valid supported adapter payload

Changing the fixture requires a new revision.

## Governing specifications

- FOREST-001 component boundaries and engines;
- EVIDENCE-001 normalization and conflict;
- FOREST-002 canonical serialization; and
- FOREST-003 projection behavior.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Raw adapter response is observed as malformed | EVIDENCE-001 |
| Scope | Adapter owner and affected evidence dimensions | Component boundary |
| Evidence | Normalizer rejects payload; `R70` remains immutable | FOREST-001 |
| Causality | Normalizer failure does not rewrite Cargo or retained roots | Separation |
| Prediction | New owner state remains unknown | No synthesis |
| Validation | No new canonical root or projection is validated | FOREST-002 |
| Planning | May use `R70` with stale limits, owner tools, or block | Safe fallback |
| Resolution | Repair adapter/normalizer or request owner evidence | Ownership routing |
| Trust/action | No execution authority follows | Component separation |
| Public view | Names failed component, retained capability, and unavailable new evidence | VIEW-001 |

## Assertions

- [x] malformed payload enters no canonical root;
- [x] retained `R70` remains queryable;
- [x] projections cannot present rejected evidence as current;
- [x] Cargo and owner-native commands remain available; and
- [x] the valid control may assemble a new root.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

FOREST-001 directly requires bounded, replaceable components and preservation
of owner workflows, producing one safe trace.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
