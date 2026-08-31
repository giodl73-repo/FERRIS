# Wave: Owner Validation Domains

Status: Active; Pulse 01 implemented for closeout review

## Product outcome

Let a Ferris user map explicit Cargo-workspace-root-relative path prefixes to
opaque repository-owned validation entrypoint IDs while Cargo remains
authoritative and every uncertain Cargo effect retains visible full-workspace
fallback.

## Classification

One bounded, non-executable validation-planning wave.

## Budget

- exactly one production pulse covering FERRIS-DOMAIN-001 through
  FERRIS-DOMAIN-003;
- implementation budget already consumed before this retroactive authority
  record;
- remaining work limited to review findings, local proof, and closeout; and
- no successor, external adopter, hosted CI, or execution work in this wave.

## Completion condition

- an optional closed owner-domain contract selects only opaque entrypoint IDs;
- existing filesystem-backed Cargo anchors preserve package selection;
- lexical missing paths may select declared owner domains but never narrow
  Cargo package scope;
- unknown or ambiguous Cargo effects retain full-workspace fallback;
- no-contract serialization and the pinned validation-plan identity remain
  unchanged;
- published schemas have positive and negative structural and semantic proof;
  and
- targeted core, CLI, schema, role, autoreview, and diff checks pass.

## Abandonment condition

Stop rather than expanding the pulse if the capability requires owner command
semantics, workflow parsing, Git discovery, execution, unverifiable
caller-asserted Cargo narrowing, another architectural layer, or a second
production pulse.

## Next product priority

Close this local pulse. Revision-bound evidence and external adopter migration
remain separate, explicitly authorized follow-on decisions.

## Pulse table

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Strict owner-domain selection | Implemented for closeout review | Optional path-prefix declarations compose opaque owner entrypoints with conservative Cargo planning |
