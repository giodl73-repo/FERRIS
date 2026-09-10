# Wave: Private Enterprise-Shape Corpus Qualification

Status: Complete; Pulse 01 stopped incomplete at Windows owner-integrity gate
Implementation authority: One evidence-only qualification pulse
Successor authority: None

## Systems-development gap

Ferris has separately demonstrated owner-first behavior in public repositories
and deterministic behavior in private synthetic repositories, but the complete
private enterprise-shape corpus has not been qualified against the current
integrated Ferris product revision. The retained synthetic scenario ledgers
bind an older Ferris revision and therefore cannot establish regression status
for current `main`.

The measurable decision is whether the current Ferris revision preserves every
already-declared selection, fallback, rejection, deterministic-identity, and
selected-versus-full outcome across all five private corpus families on local
Windows and hosted Ubuntu.

## Research basis

- [`Ferris Current Strategy and Feature Set`](../../../docs/plans/FERRIS_CURRENT_STRATEGY_AND_FEATURES.md)
  requires materially different adopter and failure evidence while preserving
  owner workflows.
- [`Ferris Program`](../../../docs/plans/FERRIS_PROGRAM.md) keeps Cargo and
  repository owners authoritative for local work and forbids converting package
  counts into savings claims.
- [`Build latency measurement contract`](../../../docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)
  permits authorized private fixtures only when private names, paths,
  dependencies, source, and identifiable timing records do not enter public
  artifacts.
- Public real-history evaluation in
  [PR #28](https://github.com/giodl73-repo/FERRIS/pull/28) found that broader
  history alone did not provide enough precise, shape-preserving execution
  evidence to promote affected-only work.

No new capability lane is selected. This wave qualifies existing behavior
against existing controlled fixtures.

## Corpus families

Private custody maps five repository identities to these public-safe labels:

| Label | Controlled shape | Ferris surface |
|---|---|---|
| `EC-01` | Linear reverse-dependency chain | `validation-plan` closure and conservative fallback |
| `EC-02` | Shared-library fanout | Reverse fanout, isolated leaves, and test-only inputs |
| `EC-03` | Fail-closed boundaries | Manifest, lockfile, unknown path, owner domain, mixed input, and deletion behavior |
| `EC-04` | High-cardinality dependency mesh | Determinism, closure scale, matched failure, and bounded fallback |
| `EC-05` | Sixteen-workspace application mesh | `federated-validation-plan`, propagation, maximum inputs, and typed rejection |

The labels are not public repository aliases and must not be published with a
mapping.

## Affected surfaces

- Ferris crates: none modified.
- Ferris tools: existing `ferris` binary built from one frozen revision.
- Fixtures: five existing private repositories and their existing immutable
  scenario tags.
- Candidate consumers: private synthetic enterprise-shape corpus only.
- Public records: this wave, Pulse 01, one aggregate report, and one role
  review.
- Private records: exact repository mapping, revisions, commands, raw output,
  and identifiable timing evidence under owner custody.

## Budget

- one qualification pulse;
- five existing private repositories;
- one frozen Ferris revision;
- one local Windows campaign and one hosted Ubuntu campaign;
- one repeated deterministic planning pass per declared scenario;
- existing scenario commands and owner oracles only;
- at most one corrective rerun for an infrastructure-only failure before any
  scenario command starts; and
- no product code, schema, dependency, fixture-scenario, or owner-command
  change.

One temporary validation branch per private repository MAY add only a hosted
CI adapter that invokes the existing scenario harness. Such branches MUST NOT
merge and MUST be removed after evidence custody.

## Completion condition

- exact private revisions and scenario tags are frozen before execution;
- every declared scenario is attempted on both platforms or receives a typed
  unavailable/not-attempted disposition;
- repeated plan and selection identities are compared;
- selected and full owner outcomes remain distinct from planning results;
- all fallback and rejection cases remain visible;
- public evidence contains only aggregate, non-identifying results;
- detailed evidence remains in private custody;
- all nine technical and stakeholder roles plus Product Value Governor and
  Autonomy Supervisor record dispositions; and
- documentation links, code fences, and `git diff --check` pass.

## Stop conditions

Stop rather than expand if qualification requires:

- a product-code, schema, dependency, corpus scenario, or owner-command change;
- workflow parsing, relationship inference, remote fetch by Ferris, or checkout
  mutation by Ferris;
- credentials in plans, output, logs, or retained evidence;
- publication of private repository identities or identifiable measurements;
- treating synthetic results as production representativeness; or
- a second corrective rerun, additional corpus, or successor feature.

## Reviewers

Required reviewers are Rust Safety Steward, Compiler Performance Engineer,
Interop Boundary Auditor, AI Assurance Skeptic, Ecosystem Strategist, Rust
Maintainer, Native Platform Adopter, Scope Keeper, Validation Checker, Product
Value Governor, and Autonomy Supervisor.

## Pulse table

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Current-main private corpus qualification | Complete; incomplete | Four family preflights passed; one deterministic-generation check failed before any Ferris scenario or Ubuntu run |

## Non-goals

- production support or enterprise representativeness;
- affected-only workflow gating or CI deletion;
- realized latency, cost, or prevented-iteration claims;
- new selection logic, execution logic, schemas, adapters, or limits;
- changes to any immutable scenario;
- public disclosure of private corpus identity or evidence; and
- automatic continuation after Pulse 01.
