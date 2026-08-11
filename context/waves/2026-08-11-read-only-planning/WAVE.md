# Wave: Read-Only Planning Foundation

Status: Complete on Windows and Unix

## Goal

Establish the smallest removable Ferris implementation: local `plan` and
`explain` commands that derive a non-executable workspace view from official
Cargo metadata.

## Classification

Release/readiness wave with one bounded implementation pulse.

## Owner actions

| Repo | Action |
|---|---|
| FERRIS | Implement, validate, review, and retain all product changes locally |
| TRACKER | Defer; do not mix this wave with the existing dirty portfolio state |
| Cargo and public fixture repositories | No-op; owner behavior is consumed through documented commands |

## Pulse table

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Local plan and explain | Complete | Implemented the read-only Cargo-metadata slice |

## Sequence

1. fix VIEW-001 numeric process codes and close FSIM-SI-004;
2. freeze public held-out source revisions, commands, and schema IDs;
3. seal held-out edits and oracles under independent custody;
4. implement only the Pulse 01 local read-only slice;
5. validate development fixtures and negative controls;
6. review outcomes through all nine roles; and
7. push FERRIS before any later TRACKER pointer update.

## Non-goals

- execution or mutation;
- `run`, `check`, `test`, `affected`, `graph`, `query`, or active `doctor`;
- MCP or connectors;
- AI prediction or narrowing;
- approvals, trust decisions, remote evidence, caching, deployment, or
  publication;
- held-out scoring during development; and
- Proposed or Adopted specification status.

## Completion gate

- Pulse 01 tests pass on the recorded Rust toolchain;
- ordinary `cargo metadata` remains the owner baseline;
- invalid, unsupported, incomplete, blocked, and internal results remain
  distinguishable;
- no network, sibling discovery, owner execution, or durable mutation occurs;
- removal is deleting the Ferris binary and generated transient output; and
- the nine-role implementation review records no P0 or P1 objection.
