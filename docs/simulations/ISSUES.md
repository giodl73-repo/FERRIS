# Ferris Simulation Issues and Specification Change Records

Status: Active

## Issue ledger

| ID | Wave | Type | Severity | Summary | Specs | Status |
|---|---|---|---|---|---|---|
| FSIM-SI-001 | W01 | gap | P1 | No canonical Change Record defined the triggering change consumed by scope, causality, prediction, validation, and planning | FOREST-002, PLANNING-001 | Resolved by FSIM-SCR-001 |
| FSIM-SI-002 | W01 | ambiguity | P1 | “Smallest safe owner boundary” lacked deterministic precedence across package, workspace, repository, application, and full-reference scope | SCOPE-001 | Resolved by FSIM-SCR-002 |
| FSIM-SI-003 | W01 | naming or UX | P2 | `check` and `test` did not define whether their default was plan-only, action request, or execution | VIEW-001 | Resolved by FSIM-SCR-003 |
| FSIM-SI-004 | W01 | known Proposed-status blocker | P2 | Fixed numeric exit codes remain unspecified, so exact process results cannot yet be simulated | VIEW-001 | Open; blocks Proposed, not Draft simulation |

## Specification Change Records

### FSIM-SCR-001: Canonical Change Record

Trigger: FSIM-SI-001

Affected specifications:

- FOREST-002;
- SCOPE-001;
- CAUSALITY-001;
- PREDICTION-001;
- VALIDATION-001; and
- PLANNING-001.

Decision: define one canonical Change Record in FOREST-002 and require
downstream records to reference it rather than relying on an informal
“triggering change.”

Retrace: FSIM-001 through FSIM-004.

Disposition: Applied and retraced.

### FSIM-SCR-002: Safe widening precedence

Trigger: FSIM-SI-002

Affected specification: SCOPE-001.

Decision: define ordered candidate boundaries and select the first boundary
whose owner mappings and mandatory coverage establish safety. Unknown safety
continues widening.

Retrace: FSIM-001 through FSIM-003.

Disposition: Applied and retraced.

### FSIM-SCR-003: Plan-first check and test commands

Trigger: FSIM-SI-003

Affected specification: VIEW-001.

Decision:

- `check` and `test` are plan-only by default;
- `--request-action` creates an action request from the displayed plan;
- `run --action-plan <id>` is the only initial execution form; and
- neither action request nor approved plan is execution.

Retrace: FSIM-001 through FSIM-004.

Disposition: Applied and retraced.
