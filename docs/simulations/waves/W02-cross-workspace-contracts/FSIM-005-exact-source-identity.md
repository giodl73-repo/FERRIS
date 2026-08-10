# FSIM-005: Exact Cross-workspace Source Identity

Wave: W02
Revision: 1
State: Retraced
Claim state: simulated

## Question

Does Ferris avoid connecting workspaces merely because they contain or consume
the same package name?

## Locked fixture

- Workspace `engine-next` contains local package `domain-core` version 2.0.0.
- Workspace `reporting` consumes registry package `domain-core` version 1.4.2.
- The lockfile source is the registry checksum, not the local path or Git
  revision.
- A private body edit occurs in local `domain-core` 2.0.0.
- No Application Definition mapping declares the local package as a provider
  for `reporting`.

Matched control: a second fixture changes the consumer lock and Application
Definition to use the exact local Git revision.

## Governing specifications

- APPLICATION-001 Cargo authority and exact owner identities;
- IDENTITY-001 source, release, lock, and unit domains;
- SCOPE-001 owner-native anchors; and
- PLANNING-001 owner-specific closures.

## Expected trace

| Stage | Predicted result |
|---|---|
| Change | Change Record identifies local path/Git source identity for `domain-core` 2.0.0 |
| Identity | Registry 1.4.2 and local 2.0.0 are distinct package-source identities |
| Scope | No mapping reaches `reporting` in the primary fixture |
| Validation | `reporting` is outside selected change scope, while its mandatory independent gates remain governed by its own policy |
| Planning | Only `engine-next` owner closures are added because of the change |
| Control | Exact Git revision and application-provider mapping create cross-workspace consumer scope |

## Assertions

- [x] Package name equality is not source or compatibility equality.
- [x] A local checkout does not shadow Cargo lock authority.
- [x] The matched control produces fan-out only after exact source linkage.
- [x] No global package-name namespace is created.

## Simulation issues

None. Existing identity and Cargo authority rules are sufficient.

## Claim boundary

No Cargo resolution ran. Source identity and scope are simulated from the
locked manifest and application declarations.
