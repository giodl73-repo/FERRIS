# FSIM-028: Complete Ferris Removal

Wave: W07
Revision: 1
State: Retraced
Claim state: simulated

## Question

What evidence is required before a federated application may claim that Ferris
has been completely removed without harming ordinary Cargo correctness?

## Locked fixture

- application: `forge`
- repositories and workspaces: three independent Cargo workspaces and CI
- source and change: organization ends Ferris adoption
- contracts and profiles: owner-native contracts and profiles remain
- environment: developer Windows host, Unix CI, and two external connectors
- policy: retain audit roots; remove active integrations, credentials, hooks,
  caches, plans, and automation
- available evidence: manifests and lockfiles do not require Ferris; one CI
  hook and one connector credential remain at first verification
- explicit unknowns: whether any workstation cache remains
- negative or matched control: second verification after inventory closure

Changing the fixture requires a new revision.

## Governing specifications

- PRODUCT-001 adoption and removal and Removal Record;
- APPLICATION-001 lifecycle;
- CONNECTOR-001 disablement and removal; and
- CONFORMANCE-001 C-REMOVE.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Organization creates one scoped Removal Record | FSIM-SCR-014 |
| Scope | Repositories, hosts, CI, connectors, credentials, data, and retained audit | Removal inventory |
| Evidence | CI hook, credential, and workstation uncertainty remain | Residual state |
| Causality | Passing Cargo once does not remove integrations elsewhere | Multi-surface proof |
| Prediction | Unknown workstation cache prevents complete claim | No hidden residual default |
| Validation | Cargo and owner-native build/test commands run on Windows and Unix after cleanup | C-REMOVE |
| Planning | Freeze, drain, export, disable, clean, verify, and retain evidence | Removal phases |
| Resolution | First pass is partial; second may complete with retained audit | Completion invariant |
| Trust/action | Credential revocation and connector removal are separately evidenced | TRUST-001 |
| Public view | Shows completed with named retained evidence only after all active residuals close | PRODUCT-001 |

## Assertions

- [x] repository deletion of Ferris metadata alone is insufficient;
- [x] retained immutable audit evidence is compatible with removal;
- [x] active hooks, credentials, and unknown caches block completion;
- [x] ordinary Cargo and owner commands are verified on Windows and Unix; and
- [x] no hidden Ferris correctness state must be recreated.

## Simulation issues

- `FSIM-SI-015`.

## Specification changes

- `FSIM-SCR-014`.

## Retrace

The first verification remains partial. The matched second pass may claim
completion only after all active residuals are resolved and owner-native
workflows pass with retained audit explicitly named.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
