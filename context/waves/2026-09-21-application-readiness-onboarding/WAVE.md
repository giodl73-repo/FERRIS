# Wave: Application Readiness Onboarding

Status: Complete through Pulse 01
Implementation authority: None

## Systems-development gap

APP-READINESS-001 has public conformance proof and bounded private enterprise
evidence, but an owner must still hand-assemble a digest-bound request that
duplicates identities and manifest paths from an existing Application
Definition. Ferris validates the completed request but offers no owner-neutral
preparation surface.

The product must not solve this friction by discovering workspaces, inventing
requirements, interpreting owner commands, or creating another source of
application truth.

## Measurable decision

Decide whether documentation and strict consuming commands are enough for
repeatable onboarding, or whether Ferris should later expose one narrow,
non-executable request-binding helper over explicit owner inputs.

The direction advances only if a helper can:

1. preserve the Application Definition and READINESS-001 declarations as owner
   truth;
2. perform no workspace, requirement, command, or environment inference;
3. emit only the existing deterministic removable request;
4. fail closed before observation when bindings are invalid; and
5. remain separate from Action Plan preparation, approval, and execution.

## Pulse sequence

| Pulse | Scope | Status |
|---|---|---|
| 01 | Inventory onboarding friction and select or reject a bounded preparation surface | Complete |

Every implementation or adopter pulse requires separate explicit approval.

## Pulse 01 result

The public three-workspace fixture requires ten cross-record equality bindings,
four exact digest bindings, and three explicit requirements associations.
Installation already has one locked Cargo command; it does not remove this
record-authoring work.

The
[research decision](../../../docs/research/2026-09-21-application-readiness-onboarding.md)
selects only a future narrow binder over one explicit Application Definition,
explicit workspace-to-requirements paths, an explicit output request path, and
the existing request schema. Documentation-only, broad `init`, inferred
requirements, Cargo discovery, and Action Plan coupling are rejected. The
[eleven-role review](../../../docs/plans/reviews/FERRIS_APPLICATION_READINESS_ONBOARDING_RESEARCH_REVIEW.md)
accepts the decision and withholds implementation.

## Reviewers

All eleven repository roles apply. Product Value Governor decides whether the
observed friction justifies a product surface. Ecosystem Strategist prevents a
new owner-truth format. Rust Maintainer and Native Platform Adopter assess
authoring and removal cost. Scope Keeper and Autonomy Supervisor keep Action
Plan generation, execution, and adopter mutation closed.

## Non-goals

- product code, schema, or dependency changes;
- generation or inference of requirements;
- Cargo discovery or metadata execution;
- Action Plan generation, approval, or execution;
- installation automation or environment repair;
- private consumer changes or raw private evidence;
- support, production, performance, or savings claims.
