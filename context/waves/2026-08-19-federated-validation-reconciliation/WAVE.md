# Wave: Federated Validation Reconciliation

Status: Complete
Implementation authority: One bounded pulse
Corrective authority: At most one review-driven pass
Successor authority: None

## Frame

Current `origin/main` at `cebce42` already ships the canonical request-based
`federated-plan` product. It groups 2-16 independent Cargo workspaces and
retains one unchanged Blueprint Plan per workspace without relationships or
validation composition.

The preserved `application-definition-prototype` branch records commits
`02a8337`, `eca5599`, and `ba3566f`. Its distinct useful capability is an
explicit consumer-owned Application Definition with `depends_on`
relationships and conservative validation propagation. The prototype is
preserved as design evidence but is superseded for product integration by
this reconciliation against current main.

Product outcome:

```text
one strict ferris.application/v0 definition
+ explicit changed paths and WORKSPACE_ID:PACKAGE inputs
+ existing single-workspace validation-plan logic
+ explicit reverse depends_on propagation
-> one non-executable ferris.federated-validation-plan/v0 record
```

The deletion target is manual per-workspace validation planning plus manual
reverse-relationship widening.

## Approved slice

Add a separate command:

```console
ferris federated-validation-plan \
  --application <APPLICATION_JSON> \
  --changed-path <PATH>... \
  --changed-package <WORKSPACE_ID:PACKAGE>... \
  --format human|json
```

Direct `cargo-ferris` and Cargo-style `cargo ferris` invocations must have the
same semantics. The command never discovers an application or changed input.

The Application Definition:

- uses strict schema `ferris.application/v0`;
- contains one portable application ID and 2-16 explicit workspaces;
- gives each workspace one portable ID, one forward-slash
  request-parent-relative Cargo manifest, and an optional explicit
  `depends_on` list;
- rejects unknown fields, duplicate IDs, manifests, Cargo workspace roots,
  nested roots, self/duplicate/unknown relationships, cycles, traversal,
  absolute or non-portable paths, and roots outside the definition parent;
- remains consumer-owned and is not the full APPLICATION-001 model.

The result:

- uses schema `ferris.federated-validation-plan/v0`;
- is read-only and non-executable;
- retains the unchanged `ferris.validation-plan/v0` record only for directly
  affected workspaces;
- widens every transitive reverse application dependent to a full-workspace
  fallback without fabricating changed inputs;
- widens all workspaces when an in-application path has no declared workspace
  owner;
- preserves independent Cargo resolution, lockfiles, metadata evidence, and
  owner commands per workspace; and
- contains no absolute path.

## Reconciliation corrections

The prototype must not be copied blindly. This wave corrects it by:

- creating a separate command instead of overloading `validation-plan`;
- preserving current `federated-plan` types, command IDs, schemas, and tests;
- using Cargo-reported canonical workspace roots rather than manifest-parent
  assumptions;
- applying current bounded federated Cargo metadata process controls;
- rejecting relationship cycles;
- keeping semantic identities independent of checkout location, Application
  Definition display filename, and Cargo metadata path bytes;
- using normalized definition content plus portable changed inputs for
  semantic request identity, with a non-revealing non-semantic placeholder
  only when no supported definition can be loaded;
- validating both components of `WORKSPACE_ID:PACKAGE`; and
- retaining current path-free diagnostic and adapter conventions.

## Stop conditions

Stop rather than widen scope if reconciliation requires:

- changing `ferris.federated-plan-request/v0`,
  `ferris.federated-plan/v0`, `ferris.validation-plan/v0`, or published
  validation-plan schemas;
- shared Cargo resolution, lock identity, or inferred relationships;
- owner-command or validation execution;
- Git discovery, mutation, networking, connectors, MCP, AI narrowing,
  approval, deployment, or remote evidence;
- the full APPLICATION-001 model or a second architecture layer;
- a new dependency, unsafe code, another pulse, or a second corrective pass.

## Completion

Completion requires focused core and CLI tests for direct selection,
transitive reverse fallback, application fallback, invalid workspace/package,
definition graph and path rejection, relocation identity, human/JSON and
adapter parity, existing-command preservation, and path-free failures.

The measured implementation result belongs in
[`Pulse 01`](pulses/pulse-01.md). The all-eleven-role closeout belongs in
`docs/plans/reviews/FERRIS-FEDERATED-VALIDATION-RECONCILIATION-REVIEW.md`.

## Removal

Delete the separate command, its two prototype-derived V0 data models, fixture
definition, tests, and this wave. Existing commands, schemas, Cargo workflows,
and consumer contracts remain unchanged.
