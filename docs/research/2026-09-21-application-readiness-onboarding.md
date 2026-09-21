# Application Readiness Onboarding

Date: 2026-09-21
Status: Complete
Decision: Prototype only a narrow explicit request binder under separate
authority

## Decision supported

Ferris should keep its existing Cargo installation path and strict
APP-READINESS-001 consumer. A later pulse may prototype one non-executable
binder that takes:

- an explicit `ferris.application/v0` file;
- an explicit requirements-file path for each named workspace; and
- an explicit output request path whose parent is the application root.

The binder may validate the existing owner records, compute their exact
digests, and write only the existing
`ferris.application-readiness-request/v1` record. It must not discover
workspaces, author or infer requirements, invoke Cargo, change an owner record,
observe the environment, prepare an Action Plan, or execute work.

This is a research decision, not implementation authority.

## Research question

Can documentation and existing strict validators make application-readiness
onboarding repeatable, or is one narrow deterministic request binder justified?

## Starting hypothesis

Installation is not the immediate blocker. The concrete friction is manually
repeating owner-authored identities and paths while computing exact byte
digests for a record whose purpose is binding rather than new owner truth.

Competing hypotheses were:

1. documentation alone is sufficient;
2. a broad `init` or onboarding generator is required; and
3. readiness preparation should be combined with Action Plan preparation.

## Local evidence

### Installation and command surface

The repository already documents one locked Cargo installation command for both
entrypoints in [`README.md`](../../README.md#L1060-L1067). Cargo installation
therefore has a concrete owner-invoked path; this pulse found no evidence that a
second installer would remove the APP-READINESS authoring work.

The CLI already consumes an explicit request at
`doctor --application-readiness` in
[`entrypoint.rs`](../../crates/ferris-cli/src/entrypoint.rs#L118-L155). The
option conflicts with workspace readiness inputs rather than inferring or
combining them.

### Frozen-fixture authoring measurement

The public three-workspace fixture consists of:

- one owner Application Definition in
  [`application.json`](../../tests/fixtures/application-readiness/application.json);
- three owner requirements declarations in
  [`tests/fixtures/application-readiness`](../../tests/fixtures/application-readiness/);
  and
- one binding request in
  [`request-valid.json`](../../tests/fixtures/application-readiness/request-valid.json).

This dependency-free measurement parsed those records and verified every
published digest:

```powershell
@'
import hashlib, json
from pathlib import Path

root = Path("tests/fixtures/application-readiness")
application = json.loads((root / "application.json").read_text())
request = json.loads((root / "request-valid.json").read_text())
requirements = {
    path.name: json.loads(path.read_text())
    for path in sorted(root.glob("requirements-*.json"))
}
application_workspaces = {
    workspace["workspace_id"]: workspace
    for workspace in application["workspaces"]
}
request_workspaces = {
    workspace["workspace_id"]: workspace
    for workspace in request["workspaces"]
}
assert request["application_id"] == application["application_id"]
assert set(application_workspaces) == set(request_workspaces)
manifest_equalities = sum(
    application_workspaces[key]["manifest_path"]
    == request_workspaces[key]["manifest_path"]
    for key in application_workspaces
)
requirements_equalities = sum(
    requirements[request_workspaces[key]["requirements_path"]]["workspace_id"]
    == key
    for key in request_workspaces
)
bound_files = [
    (
        request["application_definition"]["path"],
        request["application_definition"]["digest"],
    ),
    *[
        (workspace["requirements_path"], workspace["requirements_digest"])
        for workspace in request["workspaces"]
    ],
]
for name, expected in bound_files:
    actual = "sha256:" + hashlib.sha256((root / name).read_bytes()).hexdigest()
    assert actual == expected
print(
    {
        "cross_record_equalities": (
            1
            + len(application_workspaces)
            + manifest_equalities
            + requirements_equalities
        ),
        "exact_digest_bindings": len(bound_files),
        "requirements_associations": len(request_workspaces),
    }
)
'@ | python -
```

The measured authoring surface is:

| Binding work | Count |
|---|---:|
| Application ID equality | 1 |
| Application-to-request workspace ID equalities | 3 |
| Application-to-request manifest-path equalities | 3 |
| Request-to-requirements workspace ID equalities | 3 |
| **Total cross-record equalities** | **10** |
| Exact application and requirements digest bindings | 4 |
| Explicit workspace-to-requirements path associations | 3 |

The count scales with the contract's two-to-sixteen workspace range. The
requirements associations are new user choices; the ten equalities and four
digests are mechanical bindings that Ferris later validates.

### Existing contract boundary

APP-READINESS-001 requires the Application Definition to be a direct child of
the request root, requires complete workspace coverage, and requires exact
application and requirements digests in
[`FERRIS_APPLICATION_READINESS_COMPOSITION_CONTRACT.md`](../specs/FERRIS_APPLICATION_READINESS_COMPOSITION_CONTRACT.md#request).
The request schema already defines the complete output shape in
[`ferris.application-readiness-request.v1.schema.json`](../schemas/application-readiness/ferris.application-readiness-request.v1.schema.json).

The private enterprise shadow established that this binding has product value:
two ready workspaces did not conceal one missing-Cargo workspace. That evidence
and its limits are retained in
[`2026-09-19-private-enterprise-application-readiness-shadow.md`](2026-09-19-private-enterprise-application-readiness-shadow.md).

## Option assessment

| Option | Decision | Reason |
|---|---|---|
| Documentation only | Reject as complete solution | Documentation can explain the format but cannot remove 10 mechanical equalities and 4 byte-digest calculations. |
| Narrow explicit binder | Select for a later bounded prototype | It can reuse owner records and the existing schema while automating only mechanical validation and binding. |
| Broad `init` or generator | Reject | It would need to invent application topology, requirements, naming, or repository policy and would create competing owner truth. |
| Combined readiness and Action Plan preparation | Reject | It would collapse observation, planning, approval, and execution authorities and make a passive diagnostic appear execution-eligible. |
| Installation automation | Defer | The repository already has one locked Cargo installation path; no measured installation defect explains request authoring friction. |

## Findings

### FERRIS-803: Installation and application binding are separate onboarding problems

**Sources**

- [`README.md` installation command](../../README.md#L1060-L1067)
- [`DoctorArgs` explicit request input](../../crates/ferris-cli/src/entrypoint.rs#L118-L155)

**Observation**

Both Ferris entrypoints have one documented locked Cargo installation command.
Application readiness nevertheless requires a separately authored explicit
binding request.

**Implication**

Ferris should not add an installer to solve request authoring. Installation,
input preparation, observation, and execution remain separate concerns.

**Confidence:** High for the repository's current documented workflow.

### FERRIS-804: The frozen request contains fourteen mechanical bindings

**Sources**

- [`application.json`](../../tests/fixtures/application-readiness/application.json)
- [`request-valid.json`](../../tests/fixtures/application-readiness/request-valid.json)
- [`requirements-alpha.json`](../../tests/fixtures/application-readiness/requirements-alpha.json)
- measured comparison and digest command recorded above

**Observation**

The fourteen mechanical bindings are ten equality-constrained values repeated
across owner records plus four exact byte digests. The three
workspace-to-requirements paths are the actual new associations the owner must
choose.

**Implication**

Documentation alone cannot eliminate the error-prone mechanical portion. A
binder can reduce work without acquiring authority to choose requirements.

**Confidence:** High; every equality and digest was checked against the exact
public fixture bytes.

### FERRIS-805: A narrow binder can reuse the existing owner and schema boundaries

**Sources**

- [APP-READINESS-001 request and application binding](../specs/FERRIS_APPLICATION_READINESS_COMPOSITION_CONTRACT.md#request)
- [Existing request schema](../schemas/application-readiness/ferris.application-readiness-request.v1.schema.json)
- [Engineering principles FP-05 and FP-07](../governance/ENGINEERING_PRINCIPLES.md#fp-05-preserve-ordinary-rust-workflows)

**Observation**

The existing contract already names every input, equality, digest, path rule,
and output field needed by a binder. No new record is required.

**Implication**

A future prototype should consume explicit owner files and emit exactly the
existing request at an explicit path. Removal is deletion of that derived
request and optional helper; owner records remain unchanged.

**Confidence:** High for the contract boundary; usability remains unmeasured
until a separately authorized prototype.

### FERRIS-806: Broad generation and execution coupling would cross owner authority

**Sources**

- [Ferris program governing rule](../plans/FERRIS_PROGRAM.md#product-statement)
- [APP-READINESS-001 authority](../specs/FERRIS_APPLICATION_READINESS_COMPOSITION_CONTRACT.md#authority)
- [Repeatable enterprise onboarding result](2026-09-14-repeatable-enterprise-onboarding.md)

**Observation**

Application topology, requirements, owner commands, approval, and execution
already have separate owners and records. The successful enterprise onboarding
proof required explicit approved Action Plans and retained owner commands.

**Implication**

The binder must not become `init`, generate requirements, prepare execution, or
infer that readiness makes an Action Plan eligible.

**Confidence:** High; combining these layers would contradict existing
authority contracts rather than merely change ergonomics.

## Model evolution

The starting hypothesis was confirmed and narrowed. The useful candidate is not
a general onboarding generator. It is a derived-record binder with an explicit
output root, because request-relative paths and the direct-child Application
Definition invariant make an implicit current-directory base unsafe.

## Adopt now

- Document the implemented `doctor --application-readiness <REQUEST_JSON>`
  command alongside other development commands.
- Keep the existing locked Cargo installation command.
- Keep the frozen request schema and strict consumer unchanged.

## Prototype behind a compatibility boundary

A separately approved prototype may accept an explicit Application Definition,
an explicit mapping from every workspace ID to one requirements file, and an
explicit output request path. It should:

- reject incomplete, duplicate, extra, mismatched, nested, linked, or
  out-of-root inputs;
- compute exact digests over bounded bytes;
- preserve Application Definition workspace order or emit the contract's
  required workspace-ID order;
- write one deterministic request without modifying owner files; and
- run no readiness observation during binding.

The prototype must define atomic output and existing-file behavior before code
is authorized.

## Reject or defer

- broad `init`;
- requirement templates or inferred requirements;
- Cargo workspace discovery;
- environment observation during binding;
- Action Plan generation or approval;
- owner-command execution;
- private-adopter mutation;
- installation automation;
- support, production, performance, or savings claims.

## Contribution path

Build only as an optional Ferris preparation boundary because it binds
Ferris-specific existing records. Continue using Cargo's installation and
external-subcommand workflow; do not create a package manager or upstream Cargo
feature for this local record-authoring concern.

## Open questions

- exact CLI spelling;
- whether output must be absent or may require an explicit replace flag;
- atomic write and cleanup behavior;
- human versus JSON diagnostics for a successful derived record; and
- whether the helper should emit to stdout in addition to an explicit file
  without weakening request-root identity.

## Role review

The
[application-readiness onboarding review](../plans/reviews/FERRIS_APPLICATION_READINESS_ONBOARDING_RESEARCH_REVIEW.md)
records all eleven dispositions and keeps implementation closed.
