# VIEW-001: Ferris Command and View Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: PRODUCT-001; final semantics depend on FOREST-003 through FERRIS-001

## Purpose

This specification defines the shared command vocabulary, scope defaults,
plan and explanation views, output envelope, and exit semantics for `ferris`,
`cargo ferris`, and governed MCP adapters.

## Command model

The initial semantic commands are:

| Command | Required behavior |
|---|---|
| `plan` | Produce a versioned, non-executable Blueprint Plan |
| `run` | Execute only an explicitly approved Action Plan |
| `affected` | Resolve changed scopes relative to a revision or root |
| `graph` | Project declared and discovered relationships |
| `query` | Select typed model, plan, root, ref, and evidence records |
| `explain` | Explain included, omitted, reused, rebuilt, waiting, unknown, and fallback work |
| `check` | Plan and optionally execute the declared check activity |
| `test` | Plan and optionally execute declared test and validation activities |
| `doctor` | Diagnose configuration, tools, mappings, environment, and evidence support |

Commands MUST have stable semantic IDs independent of display spelling.
Aliases MAY be added but MUST resolve to a canonical semantic ID.

## Invocation parity

For any command supported by both adapters:

```console
ferris <command> --workspace <path> ...
cargo ferris <command> ...
```

MUST produce equivalent semantic results when the explicit workspace, inputs,
configuration, tools, and evidence are identical.

`cargo ferris` MUST NOT silently discover or operate on sibling repositories
or workspaces. `ferris` MUST NOT silently expand beyond the selected
application or repository set.

## Scope display

Every plan, query, and explanation MUST identify applicable coordinates:

- owner;
- subject;
- activity;
- configuration;
- platform;
- lifecycle;
- evidence state; and
- fallback boundary.

Package selection, compilation scope, runtime test scope, validation coverage,
contract scope, native scope, deployment scope, and evidence scope MUST remain
distinguishable.

Unknown mappings MUST be visible and MUST widen to a named safe boundary.

## Plan view

A plan view MUST include:

- plan identity and schema version;
- application, repository, workspace, and revision selection;
- owner-specific closures;
- one Cargo invocation plan per Cargo activity;
- required validation and mandatory gates;
- expected reuse and invalidation;
- resource envelope and concurrency constraints;
- uncertainty, unsupported inputs, and observation barriers;
- fallback and replan triggers;
- approval state; and
- expected root and evidence outputs.

A Blueprint Plan MUST be visibly non-executable until projected into an
approved Action Plan.

## Explanation view

Every explanation MUST answer:

1. what was selected;
2. why it was selected;
3. what was omitted;
4. what remains unknown;
5. which owner supplied each material fact;
6. what fallback applies; and
7. what evidence would change the decision.

An explanation MUST distinguish observed, inferred, predicted, resolved,
approved, executed, and yielded statements.

Human explanations MUST use maintainer-facing Cargo, package, target, command,
test, contract, native, and platform terms before internal graph vocabulary.
Every material reason MUST link to or identify its source evidence.

## Output envelope

Human and machine-readable output MUST represent the same semantic record.

Machine output MUST include:

- schema and command versions;
- semantic command ID;
- invocation and selection identity;
- result class;
- diagnostics;
- warnings and unknowns;
- plan, action, root, ref, and evidence identifiers when applicable; and
- compatibility or unsupported-version information.

Human output MAY summarize but MUST NOT hide failures, unknowns, omitted
mandatory scope, fallback, or unsupported states present in machine output.

Sensitive source paths, environment values, credentials, and model inputs MUST
follow TRUST-001 redaction and retention policy. Redaction MUST be explicit and
MUST NOT make an incomplete record appear complete.

## Exit classes

Implementations MUST distinguish at least:

| Class | Meaning |
|---|---|
| success | Requested operation completed with required evidence |
| difference | Query or comparison completed and found a material difference |
| denied | Policy or approval prohibited execution |
| invalid | Input, configuration, or selection is invalid |
| unsupported | Required owner capability or version is unsupported |
| incomplete | Evidence or required validation is missing |
| failed | An owner-local action failed |
| internal | Ferris violated an invariant or could not process valid evidence |

Numeric exit codes are assigned before implementation and MUST be identical
between both adapters.

## Safety defaults

- Planning MUST be the default for work-reducing or mutating operations.
- `run`, `check`, and `test` MUST show the selected plan before first execution
  unless an explicit reviewed policy permits non-interactive approval.
- Work-reducing AI recommendations MUST NOT execute without deterministic
  policy or human approval.
- Unsupported machine-output schema versions MUST fail explicitly.
- No command may convert missing evidence into success.

## Acceptance criteria

VIEW-001 may advance to Proposed only when:

1. all commands have stable semantic definitions;
2. adapter defaults and scope boundaries are explicit;
3. plan and explanation fixtures cover included, omitted, unknown, and
   fallback work;
4. human and machine output consistency is testable;
5. exit classes are mapped to fixed numeric codes;
6. approval and non-interactive policy are specified; and
7. all nine roles record a disposition.
