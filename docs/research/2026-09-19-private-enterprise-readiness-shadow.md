# Private Enterprise Readiness Shadow

Date: 2026-09-19
Status: Complete
Decision: Accept explicit readiness as useful pre-execution evidence
Ferris revision: `1cee93da0572ddb5d9c9f69629ce0bb6852590f1`

## Scope

Environment Readiness Pulse 05 ran the unchanged public Ferris product against
the two custody-bound private enterprise consumers, identified publicly only
as `EO-01` and `EO-02`. Exact consumer revisions remain in private session
custody.

Temporary READINESS-001 declarations and raw command outputs remained outside
both consumers. No owner command ran. The shadow compared:

1. the inherited Windows process environment, in which Cargo was not visible;
2. the same environment with only the existing Cargo directory prepended to
   `PATH` for the Ferris child process; and
3. an identical repeat of the ready observation.

The declarations covered only owner-reviewed prerequisite names and
repository-relative path kinds. They retained no environment values or
resolved paths.

## Results

| Consumer | Inherited result | Required gap | Process-local result | Ready observations | Repeat |
|---|---|---|---|---:|---|
| `EO-01` | `blocked`, exit 7 | Cargo executable `missing` | `success`, exit 0 | 7 | Byte-identical |
| `EO-02` | `blocked`, exit 7 | Cargo executable `missing` | `success`, exit 0 | 6 | Byte-identical |

All other declared observations were satisfied in both inherited runs. In both
ready repeats, report, selection, invocation, and result identities matched.
The missing and ready reports preserved the same selection and invocation
identities while result and report evidence changed with the environment.

A deny-list scan found no private root, session path, user name, machine name,
Cargo path, PowerShell path, or process `PATH` value in any retained command
output. Every observation kept `value_retained`, `resolved_path_retained`, and
`content_retained` false.

Both private source trees were clean at their bound revisions before and after
the shadow. Every temporary declaration and raw output was removed; only the
private revision custody record remains outside the repository.

## Findings

### FERRIS-794: Readiness exposed the missing Cargo prerequisite before owner work

**Sources**

- [Environment Readiness Pulse 05](../../context/waves/2026-09-16-environment-readiness/pulses/pulse-05.md)
- [READINESS-001 aggregate mapping](../specs/FERRIS_ENVIRONMENT_READINESS_CONTRACT.md)
- Private custody-bound command results summarized above

**Observation**

Both enterprise consumers returned one report-bearing `blocked` result with
exactly one non-satisfied required observation: `executable-cargo` was
`missing`. No owner validation command ran.

**Implication**

The implemented explicit readiness path addresses a demonstrated enterprise
failure mode: it converts an otherwise downstream command failure into an
up-front typed prerequisite diagnosis.

**Confidence:** High for these two declarations and this Windows environment;
this is not a support or completeness claim.

### FERRIS-795: Process-local visibility changed readiness without mutation

**Sources**

- [Environment Readiness Pulse 05](../../context/waves/2026-09-16-environment-readiness/pulses/pulse-05.md)
- Private custody-bound command results summarized above

**Observation**

Prepending the already installed Cargo directory only to the Ferris process
changed both reports from `blocked` to `ready`. The consumers, persistent
environment, dependencies, and Ferris product remained unchanged.

**Implication**

Readiness can distinguish an environment-visibility defect from a repository
defect and can confirm a process-local correction without becoming an
installer or repair tool.

**Confidence:** High for the observed transition.

### FERRIS-796: V1 readiness root scope is narrower than an application root

**Sources**

- [Environment Readiness Pulse 05](../../context/waves/2026-09-16-environment-readiness/pulses/pulse-05.md)
- [READINESS-001 workspace selection](../specs/FERRIS_ENVIRONMENT_READINESS_CONTRACT.md)
- [Application model contract](../specs/FERRIS_APPLICATION_MODEL_CONTRACT.md)

**Observation**

`EO-02` is an explicit multi-workspace application without one root Cargo
manifest. Requirements mode binds repository-path observation to one selected
Cargo manifest parent, so its shadow covered shared executable and environment
requirements plus that selected workspace's manifest, not application-root
files.

**Implication**

Ferris MUST NOT present one workspace-bound V1 report as complete
application-level readiness. A future application-level readiness design would
need explicit roots and composition rather than Cargo-root inference.

**Confidence:** High; the limitation follows directly from the implemented
selection boundary and the observed consumer shape.

### FERRIS-797: Determinism, privacy, and removal held in the private shadow

**Sources**

- [Environment Readiness Pulse 05](../../context/waves/2026-09-16-environment-readiness/pulses/pulse-05.md)
- Private custody-bound command results summarized above

**Observation**

Equivalent ready runs were byte-identical. Typed identities remained stable,
the privacy scan passed, both source trees remained clean, and all temporary
requirements and raw outputs were removed.

**Implication**

The explicit-file workflow is suitable for further bounded evaluation without
embedding private paths or environment values in public evidence. This does
not establish production support.

**Confidence:** High for the retained checks.

## Decision

Keep explicit READINESS-001 input as an implemented, useful pre-execution
diagnostic. The private shadow advances evidence from synthetic fixtures to two
enterprise-shaped consumers, but only on local Windows and only for declared
V1 properties.

Do not add automatic repair, infer undeclared prerequisites, claim owner-command
success, or treat workspace-bound readiness as application-wide readiness.

## Limitations

- No owner command ran, so readiness did not prove owner validation success.
- No Linux shadow ran.
- No version, component, service, credential, capacity, or ancestor
  configuration requirement was observed.
- The declarations were temporary evaluation inputs, not adopted owner files.
- `EO-02` was represented by one selected Cargo workspace, not its complete
  application root.
- No support, production, performance, savings, or CI-replacement claim
  follows.

## Role review

The
[Pulse 05 eleven-role review](../plans/reviews/FERRIS_PRIVATE_ENTERPRISE_READINESS_SHADOW_REVIEW.md)
accepts this evidence and closes the pulse.
