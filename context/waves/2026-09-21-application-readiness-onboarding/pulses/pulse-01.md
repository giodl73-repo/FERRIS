# Pulse 01: Application Readiness Onboarding Research

Status: Complete

## User outcome

Reduce the manual setup needed to obtain fail-closed multi-workspace readiness
without taking ownership of application topology or environment requirements.

## Control record

- Product outcome: identify the smallest reversible preparation improvement for
  APP-READINESS-001.
- Work completed: V1 implementation and public controls pass; a bounded private
  three-workspace shadow also passes.
- Value obtained: Ferris catches one blocked workspace at application scope
  before owner work.
- Remaining risk: owners must manually duplicate bindings and calculate exact
  file digests before Ferris can validate the request.
- Pulses consumed: eight environment-readiness pulses; zero onboarding-surface
  research pulses.
- Proposed next action: one local evidence inventory and adopt/wrap/build
  decision.
- Product Value Governor: `continue-within-budget`.

## Research question

Can documentation and existing strict validators make application-readiness
onboarding repeatable, or is one narrow deterministic request binder justified?

## Starting hypothesis

Installation is not the immediate blocker. The smallest credible product
candidate is a non-executable binder that consumes one explicit Application
Definition plus explicit workspace-to-requirements paths, validates them, and
emits the existing request schema. It must not author requirements or discover
workspace truth.

## Competing hypotheses

1. Documentation and examples are sufficient; no product helper is needed.
2. A broad `init` or onboarding generator is needed.
3. Action Plan preparation should be combined with readiness onboarding.

## Authority

The user's fresh `go` after Environment Readiness Pulse 08 authorizes one
research-and-documentation-only onboarding-surface decision:

- inventory current installation and CLI entrypoints;
- trace exact APP-READINESS-001 authoring duplication on public fixtures;
- use prior anonymized enterprise onboarding and shadow evidence;
- compare documentation-only, narrow binding, broad generation, and combined
  execution-preparation options;
- record stable findings and all eleven role dispositions; and
- correct directly related current command documentation.

No product behavior, schema, dependency, private consumer, Action Plan,
approval, execution, installation automation, support, production,
performance, savings, or successor pulse is authorized.

## Maximum effort

One public-source inventory, one public-fixture authoring analysis, one
decision note, one role review, directly related command documentation, and one
commit.

## Completion test

- every material conclusion cites a repository file, line range, or measured
  command;
- installation friction is distinguished from input-authoring friction;
- duplicated fields and exact-byte bindings are counted on the frozen fixture;
- the selected option preserves owner truth and record separation;
- rejected options and remaining risks stay explicit;
- no product, schema, dependency, or private repository changes;
- changed Markdown links and code fences and `git diff --check` pass; and
- all eleven roles accept or explicitly block the decision.

## Stop condition

Stop if the decision requires private source disclosure, owner-command
interpretation, automatic requirement discovery, a new schema, implementation,
or adopter mutation. Any prototype requires a separately approved pulse.

## Result

The exact public three-workspace fixture contains ten equality-constrained
cross-record values, four exact SHA-256 bindings, and three explicit
workspace-to-requirements associations. The repository already documents one
locked Cargo installation command, so installation automation does not address
the measured authoring friction.

Findings `FERRIS-803` through `FERRIS-806` select one future narrow binder
candidate. It may consume an explicit Application Definition, explicit
workspace-to-requirements paths, and an explicit output request path, then emit
only the existing request schema. Broad `init`, requirement inference, Cargo
discovery, environment observation, and Action Plan coupling are rejected.

The
[research note](../../../../docs/research/2026-09-21-application-readiness-onboarding.md)
and
[eleven-role review](../../../../docs/plans/reviews/FERRIS_APPLICATION_READINESS_ONBOARDING_RESEARCH_REVIEW.md)
record the decision and remaining gates. Pulse 01 is complete and exhausted.
No implementation, adopter, installation, execution, support, production,
performance, savings, or successor authority follows.
