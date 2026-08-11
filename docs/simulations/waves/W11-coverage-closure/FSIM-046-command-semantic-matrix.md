# FSIM-046: Public Command Semantic Matrix

Wave: W11
Revision: 1
State: Simulated
Claim state: simulated

## Question

Do all nine public commands retain stable semantics and authority boundaries
across `ferris`, `cargo ferris`, and governed MCP surfaces?

## Locked fixture

- application: `forge`
- repositories and workspaces: two Cargo workspaces
- source and change: one private body edit and one stale native mapping
- contracts and profiles: exact supported Rust profile; native evidence stale
- environment: identical explicit inputs for CLI and MCP
- policy: read-only commands allowed; actions require exact approval
- available evidence: immutable root, Change Record, plans, mappings, and
  diagnostics
- explicit unknowns: native owner closure
- negative or matched control: `cargo ferris` without explicit sibling scope

Changing the fixture requires a new revision.

## Governing specifications

- PRODUCT-001 one-engine requirement;
- VIEW-001 command model and invocation parity;
- CONNECTOR-001 MCP adapter; and
- CONFORMANCE-001 C-ENTRY.

## Hand-derived trace

| Command | Predicted semantic result |
|---|---|
| `plan` | versioned non-executable Blueprint Plan |
| `run` | action request or exact approved Action Plan only |
| `affected` | typed selected and widened affected scope |
| `graph` | root-bound projection with stale and unknown evidence |
| `query` | typed records with schema and source identities |
| `explain` | evidence-linked selection, omission, unknown, and fallback |
| `check` | plan-only by default; optional action request |
| `test` | plan-only validation selection by default |
| `doctor` | passive diagnosis plus non-executable active Probe Plans |

## Assertions

- [x] semantic command IDs are adapter-independent;
- [x] identical explicit inputs yield equivalent machine records;
- [x] human formatting may differ without semantic loss;
- [x] no read command grants action authority;
- [x] `run`, `check`, `test`, and active doctor probes use normal approval; and
- [x] implicit Cargo-workspace scope never includes a sibling workspace.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

All nine commands now have direct scenario coverage while preserving one
semantic engine and distinct authority states.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
