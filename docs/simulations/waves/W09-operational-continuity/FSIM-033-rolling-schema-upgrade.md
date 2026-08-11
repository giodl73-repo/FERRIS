# FSIM-033: Rolling Schema Upgrade

Wave: W09
Revision: 1
State: Simulated
Claim state: simulated

## Question

How does an older projection reader handle a new additive canonical record
during a rolling service upgrade?

## Locked fixture

- application: `forge`
- repositories and workspaces: unchanged
- source and change: no source change; FOREST record schema moves from 3.1 to
  backward-readable 3.2
- contracts and profiles: unchanged
- environment: writer supports 3.2; one reader supports 3.1 plus preserved
  unknown extensions
- policy: unsupported required semantics fail explicitly
- available evidence: 3.2 adds optional owner-qualified diagnostic metadata
- explicit unknowns: none
- negative or matched control: 3.2 record adds a required semantic field the
  old reader cannot interpret

Changing the fixture requires a new revision.

## Governing specifications

- FOREST-002 canonical serialization and versioning;
- FOREST-003 portability; and
- VIEW-001 unsupported machine schema behavior.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Schema evolution record classifies additive compatibility | FOREST-002 |
| Scope | Record and projection version only | No application widening |
| Evidence | Unknown extension bytes and source identity are preserved | Canonical serialization |
| Causality | Version rollout does not imply source change | Separate identity |
| Prediction | None required | Deterministic compatibility |
| Validation | Old reader reproduces required 3.1 semantics | Portability |
| Planning | No work plan changes | Schema-only event |
| Resolution | Additive case is readable; required unknown semantic is unsupported | Explicit classification |
| Trust/action | No mutation authority follows from readability | Authority separation |
| Public view | Shows reader/writer versions and preserved omitted metadata | VIEW-001 |

## Assertions

- [x] optional unknown fields are preserved safely;
- [x] the old reader does not guess new semantics;
- [x] required unknown semantics fail unsupported;
- [x] source and root identities remain explicit; and
- [x] rolling upgrade does not silently migrate historical evidence.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original schema evolution rules distinguish additive and unsupported
changes.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
