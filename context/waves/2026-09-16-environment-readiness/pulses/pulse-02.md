# Pulse 02: Environment Readiness Contract

Status: Complete
Implementation authority: None

## User outcome

Freeze the smallest reviewable contract that can tell developers why one local
environment is ready and another is not before owner tests run.

## Authority

The user's fresh `continue` after Pulse 01 authorizes documentation, schemas,
fixtures, status and exit semantics, provenance and conflict rules, removal and
rollback, and eleven-role review only.

No product code, executable probe, source-format adapter, installation, repair,
shell evaluation, lifecycle execution, network access, secret handling,
resource probe, owner validation, or adopter change is authorized.

## Maximum effort

One draft contract, two primary JSON schemas, one command-envelope
specialization, one bounded fixture family, one role review, and canonical
context updates.

## Completion test

- owner authority and the four V1 requirement kinds are exact;
- requirement, report, observation, aggregate, and process states are exact;
- source precedence and conflict behavior are explicit;
- no environment value or resolved path can enter the schemas;
- positive and exact negative fixtures cover the contract;
- adoption, compatibility, removal, and rollback are explicit;
- JSON, links, fences, and whitespace validate; and
- no Rust product code or adopter changes.

## Result

READINESS-001 is Draft. V1 accepts one explicit owner declaration and passively
checks platform, executable-name resolution, environment-name presence, and
repository-relative path kind. Required and advisory policy is per
requirement. V1 does not interpret owner-native source formats and has no source
precedence.

The report distinguishes `satisfied`, `missing`, `mismatched`,
`not_applicable`, `unsupported`, `unavailable`, `not_observed`, `stale`,
and `unknown`. V1 cannot emit source conflict because it interprets no
owner-native source. Required non-ready states map to existing VIEW-001 result
classes and process codes. Values, resolved paths, content, and secrets are
structurally absent.

## Stop condition

Pulse 02 ends after schema and fixture review. It grants no automatic Pulse 03
implementation authority.
