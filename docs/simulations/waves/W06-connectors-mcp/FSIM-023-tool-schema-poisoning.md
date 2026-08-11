# FSIM-023: MCP Tool-Schema Poisoning

Wave: W06
Revision: 1
State: Retraced
Claim state: simulated

## Question

Can an approved Action Plan use an MCP tool whose input schema or semantic
mapping changed after discovery while the server reports the same version?

## Locked fixture

- application: `forge`
- repositories and workspaces: one release workspace
- source and change: approved evidence-packet publication request
- contracts and profiles: unchanged
- environment: MCP server endpoint `S1`
- policy: action approval binds exact connector capabilities and destination
- available evidence: discovery snapshot `C7` maps `publish(packet)` to one
  repository; preflight observes an added `broadcast` field and changed schema
  digest under the same advertised server version
- explicit unknowns: whether drift is compromise or faulty deployment
- negative or matched control: unchanged schema and digest

Changing the fixture requires a new revision.

## Governing specifications

- CONNECTOR-001 capability snapshot and MCP security;
- EXECUTION-001 approval binding and preflight; and
- TRUST-001 connector trust.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Capability snapshot differs at preflight | FSIM-SCR-013 |
| Scope | Approved repository destination remains exact | Action Plan |
| Evidence | Schema and digest drift are retained | Capability snapshot |
| Causality | Same display version does not prove same capability | Identity separation |
| Prediction | Intent and safety of new field remain unknown | No inference |
| Validation | Prior connector conformance no longer covers the observed schema | Versioned surface |
| Planning | Existing plan cannot silently adopt the new field | Snapshot binding |
| Resolution | Rediscover, revalidate, replan, and renew approval or reject | FSIM-SCR-013 |
| Trust/action | Preflight blocks before publication | EXECUTION-001 |
| Public view | Shows exact changed fields, digest, and blocked action | VIEW-001 |

## Assertions

- [x] advertised server version alone is insufficient;
- [x] changed tool schema invalidates the bound snapshot;
- [x] unknown `broadcast` behavior cannot receive default arguments;
- [x] no packet is published; and
- [x] the unchanged control may pass this capability check.

## Simulation issues

- `FSIM-SI-014`.

## Specification changes

- `FSIM-SCR-013`.

## Retrace

The fixture now binds the discovered tool surface by canonical identity and
blocks material drift before action.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
