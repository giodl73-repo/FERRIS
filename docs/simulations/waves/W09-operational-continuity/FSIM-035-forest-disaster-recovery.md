# FSIM-035: Query Forest Disaster Recovery

Wave: W09
Revision: 1
State: Retraced
Claim state: simulated

## Question

Which identities survive when a Query Forest store is lost and recovery uses
a verified backup plus incomplete evidence packets?

## Locked fixture

- application: `forge`
- repositories and workspaces: three workspaces
- source and change: no source change; primary Forest store is lost
- contracts and profiles: unchanged
- environment: verified backup through root `R50`; packets contain later
  records but omit one classified observation
- policy: never invent missing evidence or current refs
- available evidence: backup digests verify; packet manifests verify; current
  branch generation after `R50` is unavailable
- explicit unknowns: omitted observation and latest ref generation
- negative or matched control: complete byte-identical replica through `R52`

Changing the fixture requires a new revision.

## Governing specifications

- FOREST-002 Recovery Record and immutable roots;
- IDENTITY-001 lineage and refs; and
- FERRIS-001 packet completeness.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Recovery incident and sources are recorded | FSIM-SCR-019 |
| Scope | Roots, records, refs, generations, retention, and packets | Recovery inventory |
| Evidence | Verified backup restores through `R50`; later packet evidence is incomplete | Recovery result |
| Causality | Packet omission prevents exact later-root reconstruction | Canonical set |
| Prediction | Latest branch target remains unknown | No invented ref |
| Validation | `R50` digest and independent reader verify | Byte-identical restore |
| Planning | Use historical `R50`, recollect owner evidence, or block | Safe recovery |
| Resolution | `R50` retains identity; reconstructed later state gets a new root | FSIM-SCR-019 |
| Trust/action | No current action uses unresolved ref generation | Identity boundary |
| Public view | Shows restored, reconstructed, missing, and unknown material | Recovery Record |

## Assertions

- [x] verified `R50` retains its root identity;
- [x] incomplete packet data cannot recreate an identical later root;
- [x] current refs are not inferred from historical roots;
- [x] missing classified evidence remains visible; and
- [x] the complete-replica control may restore `R52` byte-identically.

## Simulation issues

- `FSIM-SI-020`.

## Specification changes

- `FSIM-SCR-019`.

## Retrace

The fixture now separates verified restore, partial reconstruction, and
unresolved mutable navigation.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
