# Private Enterprise Application Readiness Shadow

Date: 2026-09-19
Status: Complete
Decision: Accept bounded application-composition evidence
Ferris revision: `9aeb5c55318ce7899c55d3cece15d09f6ee5c4b1`

## Scope

Environment Readiness Pulse 08 ran the unchanged APP-READINESS-001
implementation against the custody-bound private multi-workspace consumer
identified publicly only as `EO-02`. The exact consumer revision remains in
private session custody.

One disposable local Windows clone received five temporary evaluation inputs:
an Application Definition, an application-readiness request, and one
READINESS-001 declaration for each of three owner-declared workspace roots.
Workspace IDs and manifest paths came only from the existing owner topology.
No owner command ran.

The shadow compared:

1. the inherited process environment, in which Cargo was not visible;
2. the same environment with only the existing Cargo directory prepended to
   `PATH` for the Ferris process; and
3. an identical repeat of the ready observation.

Only one child declaration required Cargo. The other two children were
independently ready, making the inherited run a direct aggregate-precedence
control rather than three copies of the same blocked result.

## Results

| State | Exit | Application aggregate | Blocked children | Ready children | Repeat |
|---|---:|---|---:|---:|---|
| Inherited environment | 7 | `blocked` | 1 | 2 | Not applicable |
| Process-local Cargo visibility | 0 | `ready` | 0 | 3 | Byte-identical |

Selection and invocation identities remained stable across the environment
change. The two ready runs also preserved result and report identities.

The retained-output deny-list found no private root, session path, username,
machine name, resolved Cargo path, or process `PATH`. The source consumer
remained clean at its exact revision. The disposable clone had no tracked
mutation, and its only untracked files were the five expected evaluation
inputs. All raw outputs, temporary inputs, and the disposable clone were
removed; only a private custody summary remains outside the repository.

## Findings

### FERRIS-799: One blocked workspace remains visible at application scope

**Sources**

- [Environment Readiness Pulse 08](../../context/waves/2026-09-16-environment-readiness/pulses/pulse-08.md)
- [APP-READINESS-001](../specs/FERRIS_APPLICATION_READINESS_COMPOSITION_CONTRACT.md)
- Private custody-bound command results summarized above

**Observation**

Two workspace reports were ready while one reported the required Cargo
executable missing. The application returned `blocked`, exit 7, rather than
allowing the two ready children to conceal the blocked child.

**Implication**

The implemented aggregate closes the false application-wide readiness gap that
the earlier single-workspace `EO-02` shadow exposed.

**Confidence:** High for this exact three-workspace Windows shape and
declaration; this is not owner-command or support evidence.

### FERRIS-800: Process visibility changes the aggregate without repository change

**Sources**

- [Environment Readiness Pulse 08](../../context/waves/2026-09-16-environment-readiness/pulses/pulse-08.md)
- Private custody-bound command results summarized above

**Observation**

Prepending the existing Cargo directory only to the Ferris process changed the
single blocked child to ready and the application from `blocked` to `ready`.
The private source, dependencies, persistent environment, and Ferris product
did not change.

**Implication**

Application composition preserves the useful READINESS-001 distinction between
an environment-visibility defect and a repository defect.

**Confidence:** High for the observed transition.

### FERRIS-801: Determinism, privacy, and removal held at application scope

**Sources**

- [Environment Readiness Pulse 08](../../context/waves/2026-09-16-environment-readiness/pulses/pulse-08.md)
- Private custody-bound privacy, identity, and cleanup checks

**Observation**

Equivalent ready runs were byte-identical. Portable command identities
remained stable, the privacy scan passed, no tracked consumer file changed, and
all disposable inputs and raw outputs were removed.

**Implication**

The explicit application-readiness workflow can be evaluated or adopted
removably without retaining private machine state in its public report.

**Confidence:** High for this bounded local shadow.

### FERRIS-802: Ready composition remains narrower than owner validation

**Sources**

- [APP-READINESS-001 limitations](../specs/FERRIS_APPLICATION_READINESS_COMPOSITION_CONTRACT.md)
- [READINESS-001 contract](../specs/FERRIS_ENVIRONMENT_READINESS_CONTRACT.md)

**Observation**

The ready aggregate covered only three explicit workspace declarations. The
harness invoked no Cargo candidate or repository-owned validation command, and
APP-READINESS-001 intentionally did not observe application-root requirements.

**Implication**

Ferris can make declared preconditions clear before work starts, but must not
present readiness as proof that compilation, tests, application-wide policy, or
owner validation will succeed.

**Confidence:** High for the boundary; no broader readiness claim follows.

## Decision

Keep APP-READINESS-001 as the implemented application-level pre-execution
diagnostic. The private shadow advances its evidence from public fixtures to
the enterprise multi-workspace shape that motivated the contract.

Do not infer undeclared requirements, run owner work, add automatic repair, or
claim support, production readiness, performance, savings, or CI replacement.

## Limitations

- The shadow ran only on local Windows.
- Temporary evaluation declarations were not adopted owner files.
- No application-root requirement was observed.
- No version, component, service, credential, capacity, or hidden
  configuration requirement was observed.
- No owner command ran, so readiness did not prove owner validation success.
- No support, production, performance, savings, or CI-replacement claim
  follows.

## Role review

The
[Pulse 08 eleven-role review](../plans/reviews/FERRIS_PRIVATE_ENTERPRISE_APPLICATION_READINESS_SHADOW_REVIEW.md)
accepts this bounded evidence and closes the pulse.
