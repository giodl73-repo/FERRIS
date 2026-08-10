# FSIM-009: Ref Namespace and Root Switching

Wave: W03
Revision: 1
State: Retraced
Claim state: simulated

## Question

How does Ferris switch a moving branch association to a new immutable root
when another typed ref has the same display name?

## Locked fixture

- application: `forge`
- repositories and workspaces: one Git repository and one Cargo workspace
- source and change: Git branch `release` moves from revision `A` to `B`
- contracts and profiles: unchanged
- environment: unchanged supported host
- policy: branch updates are allowed; published tags are write-once
- available evidence: root `R10` observes `A`; root `R11` observes `B`; a
  Ferris tag also has display name `release` and points to `R9`
- explicit unknowns: none
- negative or matched control: an unqualified lookup for `release`

Changing the fixture requires a new revision.

## Governing specifications

- IDENTITY-001 typed refs, ref updates, and generations;
- FOREST-002 immutable Forest root; and
- FOREST-003 snapshot and generation behavior.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Git revision association changes from `A` to `B` | Git retains source authority |
| Scope | One repository and workspace association | SCOPE-001 exact owner coordinates |
| Evidence | `R10` and `R11` remain immutable | FOREST-002 root identity |
| Causality | No claim that root movement changes source | Identity is association, not authority |
| Prediction | None required | Ref update is deterministic |
| Validation | Root assembly evidence for `R11` remains separate | Identity does not prove validation |
| Planning | Branch ref update expects `R10` and its generation | IDENTITY-001 compare-and-set |
| Resolution | Qualified branch ref selects `R11`; tag remains `R9` | FSIM-SCR-006 |
| Trust/action | Update requires actor authority but does not mutate either root | IDENTITY-001 |
| Public view | `branch:release` and `tag:release` are distinct; bare `release` is unresolved | FSIM-SCR-006 |

## Assertions

- [x] root `R10`, root `R11`, and tag target `R9` remain immutable;
- [x] moving the branch creates a new generation;
- [x] the tag does not move;
- [x] ref type is part of canonical identity; and
- [x] unqualified ambiguous lookup does not choose by precedence.

## Simulation issues

- `FSIM-SI-007`.

## Specification changes

- `FSIM-SCR-006`.

## Retrace

The locked fixture now resolves qualified refs deterministically and rejects
the ambiguous unqualified control without changing Git authority.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
