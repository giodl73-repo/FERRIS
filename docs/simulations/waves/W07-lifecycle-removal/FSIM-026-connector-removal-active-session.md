# FSIM-026: Connector Removal with Active Session

Wave: W07
Revision: 1
State: Retraced
Claim state: simulated

## Question

When is connector removal complete if an authenticated external session and a
pending owner operation still exist?

## Locked fixture

- application: `forge`
- repositories and workspaces: one release workspace
- source and change: organization requests removal of connector `K9`
- contracts and profiles: no replacement connector selected
- environment: connector service, local configuration, and external endpoint
- policy: deny new use immediately and preserve required audit
- available evidence: one active session, one pending non-mutating owner query,
  local cache, endpoint hook, and ephemeral credential class
- explicit unknowns: whether remote session revocation has propagated
- negative or matched control: connector with no active sessions or hooks

Changing the fixture requires a new revision.

## Governing specifications

- PRODUCT-001 Removal Record;
- CONNECTOR-001 disablement and removal;
- TRUST-001 revocation and deletion; and
- EXECUTION-001 cancellation protocol.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Removal request creates phased Removal Record | FSIM-SCR-014 |
| Scope | Connector code, session, credential, hook, cache, audit, and owner fallback | Removal inventory |
| Evidence | Active session and propagation uncertainty remain visible | TRUST-001 |
| Causality | Uninstalling code cannot revoke remote state | Owner boundary |
| Prediction | Revocation completion remains unknown | No success default |
| Validation | Owner-native access is checked before completion | Removal verification |
| Planning | Freeze new calls, resolve session/query, export audit, revoke, clean, verify | Removal phases |
| Resolution | Block or remain partial until residual state is resolved | Completion invariant |
| Trust/action | New calls denied; active operation follows cancellation or safe completion | CONNECTOR-001 |
| Public view | Shows draining or partial, not removed | PRODUCT-001 |

## Assertions

- [x] code uninstall alone is insufficient;
- [x] new connector use is denied first;
- [x] active session and pending operation receive explicit disposition;
- [x] credential and remote revocation are independently verified; and
- [x] the empty-session control may complete after verification.

## Simulation issues

- `FSIM-SI-015`.

## Specification changes

- `FSIM-SCR-014`.

## Retrace

The fixture now remains draining, blocked, or partial until active use,
credentials, hooks, retained data, and owner-native fallback are resolved.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
