# Ferris Federated Validation Reconciliation Review

Date: 2026-08-19
Scope: Federated Validation Reconciliation Pulse 01
Base: `origin/main` at `cebce42`
Prototype evidence: `application-definition-prototype` commits `02a8337`,
`eca5599`, and `ba3566f`
Disposition: Complete within one implementation pulse and one corrective pass
Implementation authority: No expansion

## Reconciliation decision

The preserved prototype branch remains available as historical design
evidence but is superseded for product integration. The canonical shipped
request-based `federated-plan`, `ferris.federated-plan-request/v0`, and
`ferris.federated-plan/v0` remain unchanged and continue to own
relationship-free application plan collation.

Only the prototype's distinct capability was reconciled: one separate
read-only `federated-validation-plan` command accepts a strict
consumer-owned `ferris.application/v0` definition and composes the existing
single-workspace validation-plan algorithm across explicit `depends_on`
relationships. No existing V0 contract or published validation-plan schema
changed.

## Product Value Governor

Disposition: `continue-within-budget`

Pass. The command deletes manual per-workspace planning and reverse
relationship widening without replacing Cargo or the already shipped
federated-plan. The outcome completed within the authorized pulse and
corrective budget.

Stop condition: delivery is complete. A richer Application Definition,
relationship semantics, execution, or another architecture layer requires new
user authority.

## Rust Safety Steward

Pass. The change adds no `unsafe`. JSON and workspace cardinality are bounded.
Cargo metadata uses the existing federated direct-child timeout and output
limits. Compiler acceptance and tests are not presented as proofs beyond the
measured behavior.

## Compiler Performance Engineer

Pass with no performance claim. Cargo metadata runs sequentially once for
every declared workspace. At 16 workspaces, the timeout ceiling is 480 seconds
plus process startup and cleanup. Caching, parallelism, affected discovery,
and execution remain out of scope.

## Interop Boundary Auditor

Pass. Cargo independently owns membership, packages, resolution, lockfiles,
features, targets, and metadata in each workspace. The implementation uses
Cargo-reported canonical workspace roots, rejects duplicate and nested roots,
and never constructs a shared resolution or lock graph. Application
relationships are explicit consumer assertions used only for conservative
validation propagation.

## AI Assurance Skeptic

Pass. Executable tests cover direct selection, two-hop reverse fallback,
application fallback, unknown workspace and package, invalid qualifier,
strict shape, cycles, duplicates, traversal, outside-application paths,
relocation identity, human/JSON output, adapter parity, path-free failures,
and existing-command preservation. Unknown application compatibility and
owner validation requirements remain explicit.

## Ecosystem Strategist

Pass. The command fills the measured gap between independent federated
collation and one-workspace validation planning. It reuses Cargo and the
existing validation-plan algorithm rather than introducing a resolver,
workflow engine, or replacement manifest.

## Rust Maintainer

Pass. The separate command avoids overloading `validation-plan`. The core
extracts the existing decoded-metadata validation implementation so the
federated path can reuse the exact algorithm and one bounded metadata result
without duplicating generic Cargo or validation helpers. No dependency,
plugin layer, public execution API, or schema publication was added.

Removal remains bounded to the command, prototype-derived application/result
types, fixture additions, focused tests, documentation, and this wave.

## Native Platform Adopter

Pass within the existing local experimental boundary. Definition paths require
portable forward slashes and remain below one canonical definition parent.
Changed paths and output become application- or workspace-relative, and
tested failures disclose no absolute paths. Different-drive grouping remains
unsupported.

## Scope Keeper

Pass. The slice is read-only composition across explicit relationships only.
It adds no relationship inference, Git discovery, owner commands, validation
execution, mutation, connectors, MCP, AI narrowing, approval, deployment,
remote evidence, or full APPLICATION-001 model.

## Validation Checker

Pass. Focused and shared tests, workspace check, targeted Clippy, rustfmt,
changed-JSON parsing, and diff hygiene passed. The core library passed 60 tests
with 2 ignored historical controls. CLI validation passed 10 `ferris` binary
tests, 10 `cargo-ferris` binary tests, 34 shared tests, 10 existing
federated-plan tests, 8 focused federated-validation-plan tests, and 4
published validation-plan schema tests.

The direct embedded workspace record is equality-tested against current
single-workspace `create_validation_plan` behavior. Existing federated-plan
and validation-plan schemas and command IDs are separately regression-tested.

## Autonomy Supervisor

Pass. The user fixed the outcome, base, branch, boundaries, pulse budget,
corrective budget, validation, commit message, and stop condition before work.
One implementation pulse and one corrective pass were consumed. The
corrective pass made package-component whitespace strict and included the new
command in the shared help-parity gate. No successor was started.

Control record:

- product outcome: explicit relationship-aware federated validation planning;
- work completed: strict application input, bounded Cargo-root loading,
  unchanged direct validation records, conservative fallbacks, CLI parity,
  tests, and documentation;
- value obtained: manual planning and reverse relationship widening are
  replaced without changing canonical federated-plan;
- remaining risk: sequential all-workspace metadata cost, direct-child rather
  than process-tree termination, common-parent path constraint, explicit
  relationship trust, unsupported experimental compatibility, and
  evidence-sensitive top-level result identity;
- pulses or retries consumed: one implementation pulse and one corrective
  pass;
- proposed next action: stop after delivery; and
- Product Value Governor disposition: `continue-within-budget`.

## Corrective review record

The reconciliation corrected the prototype by:

- creating a separate semantic command rather than overloading
  `validation-plan`;
- preserving current federated-plan types, request/result contracts, tests,
  and documentation;
- using Cargo-reported workspace roots instead of manifest-parent ownership;
- reusing the current bounded federated Cargo process controls;
- reusing the exact current decoded-metadata validation-plan algorithm;
- rejecting cycles, duplicate/nested Cargo roots, traversal, and outside-root
  paths;
- making qualified package parsing strict and unambiguous;
- excluding location-sensitive Cargo evidence from portable plan identities;
- keeping errors path-free; and
- adding shared direct/Cargo help and execution parity.

No blocking finding remains.

## Decision

Pulse 01 is complete. All eleven roles accept this bounded unsupported V0.
No role grants execution, mutation, relationship inference, shared Cargo
resolution, changes to existing V0 contracts, APPLICATION-001 conformance,
support, production, or successor authority.
