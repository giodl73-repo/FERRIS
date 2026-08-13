# Wave: Read-Only Planning Foundation

Status: Pulses 01 through 14 validated locally; Pulse 13 retains the latest held-out proof

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
| 02 | Declared workspace graph | Complete | Projected bounded Cargo-declared package and dependency structure |
| 03 | Read-only interface hardening | Complete | Corrected identity, rendering, evidence, diagnostics, and CLI envelopes |
| 04 | Passive local doctor | Complete | Validated locally; no existing held-out fixture was applicable |
| 05 | Passive doctor hardening | Complete | Corrected review findings; applicable held-out owner-context fixtures passed |
| 06 | Blind doctor fixture remediation | Validated | Corrected strict evidence and complete identity after FHIF-013 failure |
| 07 | Typed-record doctor identity | Validated | Replaced manual identity lists and tightened canonical evidence after FHIF-014 failure |
| 08 | Bounded machine framing | Validated | Replaced ambiguous owner-output concatenation with domain-separated length framing |
| 09 | Typed bounded failures | Validated | Retained bounded failure evidence and diagnostic-bound result identity |
| 10 | Canonical command results | Validated | Unified command outcomes in a complete typed result envelope |
| 11 | Selection and result relationships | Validated | Separated selection, invocation, and result identities |
| 12 | Universal typed non-success | Validated | Routed parsed and syntax failures through typed stderr envelopes |
| 13 | Panic and output boundary | Validated | Converts catchable internal panics and success-output failures into typed internal results |
| 14 | Local profile evidence diff | Complete | Adds a bounded two-file experimental evidence diff without owner execution or raw section values |

## Sequence

1. fix VIEW-001 numeric process codes and close FSIM-SI-004;
2. freeze public held-out source revisions, commands, and schema IDs;
3. seal held-out edits and oracles under independent custody;
4. implement and score the Pulse 01 local read-only slice;
5. implement only the Pulse 02 bounded declaration graph;
6. validate development fixtures and negative controls;
7. review outcomes through all nine roles; and
8. push FERRIS before any later TRACKER pointer update.

## Non-goals

- execution or mutation;
- `run`, `check`, `test`, `affected`, `query`, or active `doctor`;
- MCP or connectors;
- AI prediction or narrowing;
- approvals, trust decisions, remote evidence, caching, deployment, or
  publication;
- held-out scoring during development; and
- Proposed or Adopted specification status.

## Completion gate

- Pulse 01 tests pass on the recorded Rust toolchain;
- Pulse 02 graph tests pass on Windows and Unix;
- ordinary `cargo metadata` remains the owner baseline;
- invalid, unsupported, incomplete, blocked, and internal results remain
  distinguishable;
- no network, sibling discovery, owner execution, or durable mutation occurs;
- removal is deleting the Ferris binary and generated transient output; and
- the nine-role implementation review records no P0 or P1 objection.
