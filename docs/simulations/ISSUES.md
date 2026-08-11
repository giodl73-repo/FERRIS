# Ferris Simulation Issues and Specification Change Records

Status: Active

## Issue ledger

| ID | Wave | Type | Severity | Summary | Specs | Status |
|---|---|---|---|---|---|---|
| FSIM-SI-001 | W01 | gap | P1 | No canonical Change Record defined the triggering change consumed by scope, causality, prediction, validation, and planning | FOREST-002, PLANNING-001 | Resolved by FSIM-SCR-001 |
| FSIM-SI-002 | W01 | ambiguity | P1 | “Smallest safe owner boundary” lacked deterministic precedence across package, workspace, repository, application, and full-reference scope | SCOPE-001 | Resolved by FSIM-SCR-002 |
| FSIM-SI-003 | W01 | naming or UX | P2 | `check` and `test` did not define whether their default was plan-only, action request, or execution | VIEW-001 | Resolved by FSIM-SCR-003 |
| FSIM-SI-004 | W01 | known Proposed-status blocker | P2 | Fixed numeric exit codes remain unspecified, so exact process results cannot yet be simulated | VIEW-001 | Open; blocks Proposed, not Draft simulation |
| FSIM-SI-005 | W02 | ambiguity | P1 | Mandatory semantic, Rust, projection, and adapter compatibility results lacked a deterministic application eligibility rule | CONTRACT-001, APPLICATION-001 | Resolved by FSIM-SCR-004 |
| FSIM-SI-006 | W02 | unsafe default | P1 | Wider selected scope could appear sufficient even when owner freshness did not observe a changed hidden build or native input | PLANNING-001, VALIDATION-001, EXECUTION-001 | Resolved by FSIM-SCR-005 |
| FSIM-SI-007 | W03 | ambiguity | P1 | Typed refs did not define canonical namespace identity, type mutation, or ambiguous unqualified lookup when names collide | IDENTITY-001 | Resolved by FSIM-SCR-006 |
| FSIM-SI-008 | W03 | unsafe fallback | P1 | Projection inconsistency was detectable but did not explicitly block planning or distinguish owner conflict from a projection-engine invariant violation | FOREST-003, PLANNING-001, VIEW-001 | Resolved by FSIM-SCR-007 |
| FSIM-SI-009 | W04 | unsafe default | P1 | A calibrated or high-confidence prediction lacked an explicit admission record and deterministic policy gate before reducing owner work or advisory validation | PREDICTION-001, VALIDATION-001, PLANNING-001 | Resolved by FSIM-SCR-008 |
| FSIM-SI-010 | W04 | missing failure state | P1 | Partial, truncated, budget-exhausted, refused, or schema-invalid model output could be mistaken for a complete Prediction Record | PREDICTION-001, VIEW-001 | Resolved by FSIM-SCR-009 |
| FSIM-SI-011 | W05 | missing failure state | P1 | A cancellation request lacked distinct acknowledgement, propagation, too-late, owner-deferred, and failed-stop states | EXECUTION-001, VIEW-001 | Resolved by FSIM-SCR-010 |
| FSIM-SI-012 | W05 | ambiguity | P1 | One terminal execution state could not preserve simultaneous execution, rollback, cleanup, and residual-effect outcomes | EXECUTION-001, VIEW-001 | Resolved by FSIM-SCR-011 |
| FSIM-SI-013 | W06 | unsafe default | P1 | Untrusted connector and MCP content lacked a deterministic rule preventing embedded text from becoming Ferris instructions, scope, approval, or tool calls | CONNECTOR-001 | Resolved by FSIM-SCR-012 |
| FSIM-SI-014 | W06 | gap | P1 | Connector version alone did not bind the discovered MCP tool, resource, prompt, and schema surface against post-discovery poisoning or drift | CONNECTOR-001, EXECUTION-001 | Resolved by FSIM-SCR-013 |
| FSIM-SI-015 | W07 | gap | P1 | Removal obligations were distributed across specs without one canonical phased Removal Record or completion rule | PRODUCT-001, APPLICATION-001, CONNECTOR-001 | Resolved by FSIM-SCR-014 |
| FSIM-SI-016 | W07 | ambiguity | P1 | One packet lifecycle state could not preserve historical publication, current eligibility, and partial deletion simultaneously | FERRIS-001 | Resolved by FSIM-SCR-015 |
| FSIM-SI-017 | W08 | unsafe default | P0 | Emergency revocation could affect a running action without a required observation point before subsequent side effects | TRUST-001, EXECUTION-001 | Resolved by FSIM-SCR-016 |
| FSIM-SI-018 | W08 | unsafe default | P1 | Preflight alone did not require an atomic owner-state guard against a concurrent mutation between check and external write | EXECUTION-001 | Resolved by FSIM-SCR-017 |

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

### FSIM-SCR-004: Layered compatibility eligibility

Trigger: FSIM-SI-005

Affected specifications:

- CONTRACT-001; and
- APPLICATION-001.

Decision: retain every layer result and derive application eligibility only
after evaluating each mandatory boundary. Breaking, unsupported, failed,
stale, or unknown mandatory layers cannot be hidden by a compiling projection.
Optional loss requires an explicit optional classification and capability
consequence.

Retrace: FSIM-006 and compatibility assertions in FSIM-002.

Disposition: Applied and retraced.

### FSIM-SCR-005: Owner freshness insufficiency

Trigger: FSIM-SI-006

Affected specifications:

- PLANNING-001;
- VALIDATION-001; and
- EXECUTION-001.

Decision: when evidence shows a changed input that the owner freshness model
does not declare or observe, scope widening alone is insufficient. Planning
must block or offer explicit owner repair, isolated empty-state rebuild,
supported prior environment, or defer alternatives. State invalidation and
cleaning require a later approved Action Plan.

Retrace: FSIM-008 and unknown-input controls in FSIM-003.

Disposition: Applied and retraced.

### FSIM-SCR-006: Typed ref namespace resolution

Trigger: FSIM-SI-007

Affected specification: IDENTITY-001.

Decision: canonical ref identity includes isolation domain, owner namespace,
ref type, and canonical name. Ref type is immutable. Identical display names
may exist across types or owners, but an unqualified lookup with more than one
candidate is unresolved and cannot use type precedence.

Retrace: FSIM-009 and ref assertions in FSIM-004.

Disposition: Applied and retraced.

### FSIM-SCR-007: Material projection inconsistency

Trigger: FSIM-SI-008

Affected specifications:

- FOREST-003;
- PLANNING-001; and
- VIEW-001.

Decision: a material projection inconsistency blocks the affected projection
from planning, resolution, trust, approval, or action. A coarser fallback is
allowed only when independently derived and explicitly excludes the conflict.
Conflicting owner evidence is an incomplete or blocked evidence state; a
projection engine contradicting canonical records or its own equivalent
projection is an internal invariant violation.

Retrace: FSIM-012 and conflicting-evidence controls in FSIM-003.

Disposition: Applied and retraced.

### FSIM-SCR-008: Work-reducing prediction admission

Trigger: FSIM-SI-009

Affected specifications:

- PREDICTION-001;
- VALIDATION-001; and
- PLANNING-001.

Decision: every prediction that would remove owner work or advisory validation
from the deterministic baseline requires a separate admission record.
Admission binds the predictor version, named population, current held-out
calibration, false-omission and capability thresholds, deterministic minimum
floor, fallback, policy, approval requirement, expiry, and disable trigger.
Admission cannot remove mandatory gates, capability requirements, or
full-reference obligations. Model confidence alone cannot admit narrowing,
and the unreduced baseline remains queryable.

Retrace: FSIM-014 and narrowing assertions in FSIM-002 and FSIM-003.

Disposition: Applied and retraced.

### FSIM-SCR-009: Incomplete model-result handling

Trigger: FSIM-SI-010

Affected specifications:

- PREDICTION-001; and
- VIEW-001.

Decision: model invocation outcome distinguishes complete, truncated,
budget-exhausted, refused, tool-failed, provider-failed, schema-invalid, and
unknown. Only a complete, schema-valid, deterministically normalized result
may become a model-produced Prediction Record. Every other result abstains and
uses safe fallback; parseable partial content cannot narrow work.

Retrace: FSIM-016 and AI failure controls in FSIM-014.

Disposition: Applied and retraced.

### FSIM-SCR-010: Cancellation protocol

Trigger: FSIM-SI-011

Affected specifications:

- EXECUTION-001; and
- VIEW-001.

Decision: cancellation becomes an attributable protocol with requested,
denied, acknowledged, propagating, owner-deferred, completed-before-stop,
cancelled, failed, and unknown states. A request or acknowledgement is not
proof of stopped work. Owner interruptibility, safe points, irreversible
effects, remaining work, rollback or compensation, and cleanup remain visible.

Retrace: FSIM-018 and cancellation assertions in FSIM-004.

Disposition: Applied and retraced.

### FSIM-SCR-011: Composite execution outcome

Trigger: FSIM-SI-012

Affected specifications:

- EXECUTION-001; and
- VIEW-001.

Decision: terminal outcome preserves independent execution, rollback, cleanup,
and residual-effect dimensions plus required recovery ownership. Overall
success is prohibited when required rollback or cleanup failed, remains
partial, or has unknown residual effects.

Retrace: FSIM-019 and failure assertions in FSIM-008.

Disposition: Applied and retraced.

### FSIM-SCR-012: Untrusted connector-content boundary

Trigger: FSIM-SI-013

Affected specification: CONNECTOR-001.

Decision: external owner text, tool descriptions, resources, prompts,
comments, issue bodies, logs, and model-visible connector content are data,
never Ferris instructions or authority. They cannot change semantic command,
scope, policy, approval, connector selection, tool arguments, or disclosure.
Derived proposals require explicit provenance, schema validation, and the
normal planning and approval path; suspected injection is retained as a
security diagnostic.

Retrace: FSIM-022 and AI authority assertions in FSIM-014.

Disposition: Applied and retraced.

### FSIM-SCR-013: Connector capability snapshot

Trigger: FSIM-SI-014

Affected specifications:

- CONNECTOR-001; and
- EXECUTION-001.

Decision: every connector or MCP interaction binds a versioned capability
snapshot covering endpoint and transport identity, tool/resource/prompt
schemas and digests, command mappings, authentication audience, permissions,
observation time, expiry, and trust state. Material drift invalidates planning
or action and requires rediscovery, revalidation, replan, and renewed approval
where applicable.

Retrace: FSIM-023 and connector preflight assertions in FSIM-017.

Disposition: Applied and retraced.

### FSIM-SCR-014: Canonical phased removal

Trigger: FSIM-SI-015

Affected specifications:

- PRODUCT-001;
- APPLICATION-001; and
- CONNECTOR-001.

Decision: Ferris removal uses one phased Removal Record covering inventory,
new-action freeze, active action/session disposition, owner configuration
export, connector and credential disablement, integration cleanup, data and
evidence retention or deletion, owner-native verification, residual state,
rollback, and audit. Completion is prohibited while hidden correctness
dependencies, active mutation, reusable credentials, unresolved hooks, or
unknown residual effects remain.

Retrace: FSIM-026, FSIM-028, and removal assertions in FSIM-020.

Disposition: Applied and retraced.

### FSIM-SCR-015: Packet lifecycle facets

Trigger: FSIM-SI-016

Affected specification: FERRIS-001.

Decision: packet publication history, current eligibility, and retention or
deletion state are independent facets. Later revocation or deletion does not
rewrite prior submission or acceptance, and prior acceptance cannot conceal
current revocation or partial deletion.

Retrace: FSIM-027 and packet assertions in FSIM-023.

Disposition: Applied and retraced.

### FSIM-SCR-016: Running-action revocation barrier

Trigger: FSIM-SI-017

Affected specifications:

- TRUST-001; and
- EXECUTION-001.

Decision: every mutable Action Plan declares applicable revocation sources,
observation method, maximum detection interval, and side-effect barriers.
Revocation is checked before each new side-effecting owner operation and at
declared barriers. Applicable revocation stops new work and enters the
cancellation, rollback, compensation, cleanup, and audit protocol. Unknown
revocation status blocks the next side effect.

Ferris Wheel retrace: FSIM-018, FSIM-023, FSIM-026, and FSIM-030.

Disposition: Applied and retraced.

### FSIM-SCR-017: Atomic owner-state mutation guard

Trigger: FSIM-SI-018

Affected specification: EXECUTION-001.

Decision: every external or shared-state mutation binds the expected owner
generation, version, ETag, revision, lease, or equivalent state immediately
before the operation and uses an owner-native conditional mutation when
available. If drift cannot be excluded atomically through a conditional write
or exclusive isolation, the operation is unsupported or blocked. Preflight
alone is not sufficient.

Ferris Wheel retrace: FSIM-010, FSIM-020, FSIM-023, and FSIM-032.

Disposition: Applied and retraced.
