# FSIM-021: CLI and MCP Planning Parity

Wave: W06
Revision: 1
State: Simulated
Claim state: simulated

## Question

Do `ferris plan` and `ferris.plan` produce the same semantic plan when every
explicit input, version, scope, policy, and evidence identity matches?

## Locked fixture

- application: `forge`
- repositories and workspaces: two Cargo workspaces
- source and change: one canonical Change Record
- contracts and profiles: identical explicit contract and profile identities
- environment: identical supported host identity
- policy: read-only planning is authorized for both principals
- available evidence: one immutable root and one connector capability snapshot
- explicit unknowns: none
- negative or matched control: MCP request omits the selected workspace

Changing the fixture requires a new revision.

## Governing specifications

- PRODUCT-001 one-engine requirement;
- CONNECTOR-001 MCP adapter; and
- VIEW-001 invocation parity and output envelope.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Both invocations reference the same Change Record | FOREST-002 |
| Scope | Explicit coordinates are identical | SCOPE-001 |
| Evidence | Same root and cutoff | Snapshot isolation |
| Causality | Same evidence path | CAUSALITY-001 |
| Prediction | Same predictor inputs or no prediction | PREDICTION-001 |
| Validation | Same Validation Plan | VALIDATION-001 |
| Planning | Same semantic Blueprint Plan identity | One engine |
| Resolution | None requested | Planning remains non-executable |
| Trust/action | No action request or approval | MCP read boundary |
| Public view | Human formatting may differ; machine semantic record is equivalent | VIEW-001 |

## Assertions

- [x] command spelling does not change plan semantics;
- [x] MCP tool discovery does not add authority;
- [x] the same unknowns and fallbacks appear;
- [x] no action is created; and
- [x] the omitted-workspace control has a distinct invocation identity.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The one-engine and parity rules produce one unambiguous semantic result.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
