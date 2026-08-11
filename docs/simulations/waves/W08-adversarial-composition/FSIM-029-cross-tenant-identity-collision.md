# FSIM-029: Cross-Tenant Identity Collision

Wave: W08
Revision: 1
State: Simulated
Claim state: simulated

## Question

Can records from two tenants join when their package names, domain-local
identifiers, and display root labels are identical?

## Locked fixture

- application: two unrelated applications named `forge`
- repositories and workspaces: tenant `T1` and tenant `T2` each contain
  package `core` and local root label `R1`
- source and change: private edit in `T1`
- contracts and profiles: independently owned
- environment: shared Ferris service with tenant isolation
- policy: no cross-tenant operation is authorized
- available evidence: domain-local identifiers collide; tenant and owner
  namespaces differ
- explicit unknowns: none
- negative or matched control: explicitly authorized aggregate containing
  disclosure-reviewed summaries only

Changing the fixture requires a new revision.

## Governing specifications

- GOVERNANCE-001 tenant isolation;
- FOREST-002 identity domains;
- FOREST-003 consistency rules; and
- IDENTITY-001 canonical ref identity.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | `T1` Change Record remains in `T1` | Tenant scope |
| Scope | Only `T1` application and owner coordinates | SCOPE-001 |
| Evidence | Same local names remain distinct identities | Domain plus tenant |
| Causality | Name equality creates no edge | Identity is not relationship |
| Prediction | No `T2` work is forecast | Isolation |
| Validation | `T2` evidence is unavailable, not empty | Governance |
| Planning | Plan contains only `T1` owner closures | No cross join |
| Resolution | Cross-tenant candidate is ineligible | Policy |
| Trust/action | No `T2` disclosure or action | TRUST-001 |
| Public view | Qualifies tenant and owner or redacts collision safely | VIEW-001 |

## Assertions

- [x] local identifier equality does not establish exact identity;
- [x] no edge, aggregate, or fallback crosses tenants;
- [x] unavailable `T2` evidence is not represented as empty;
- [x] `T2` names and paths are not disclosed; and
- [x] the authorized summary control uses a separate scoped record.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original tenant, identity-domain, and projection rules reject the join.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
