# FSIM-001: Private Body Edit in a Shared Library

Wave: W01
Revision: 1
State: Retraced
Claim state: simulated

## Question

Can Ferris distinguish source API stability from possible behavioral impact
and produce a conservative cross-workspace validation plan?

## Locked fixture

- Application `ledger-cli` contains two repositories.
- Repository `ledger-lib` has workspace `lib`, package `ledger-core`.
- Repository `ledger-app` has workspace `app`, package `ledger-cli`.
- `ledger-cli` consumes released source from `ledger-core`.
- The change modifies one private function body in `ledger-core`.
- Cargo metadata, lock state, package ownership, and the cross-workspace
  consumer relationship are available.
- No behavioral contract maps the private function to individual tests.
- Full-reference validation is the documented validation of both workspaces.
- No action or execution is requested.

Negative control: a comment-only change with no generated, policy, or
repository-gate effect.

## Governing specifications

- FOREST-002 Change Record;
- SCOPE-001 owner anchors, mappings, narrowing, and widening;
- CAUSALITY-001 stage-specific causality;
- PREDICTION-001 safe fallback;
- VALIDATION-001 selection rule;
- PLANNING-001 owner-specific closures; and
- VIEW-001 plan-first `check`.

## Initial hand trace

The baseline specs named a “triggering change” but did not define one canonical
record. They also did not deterministically choose among package, workspace,
repository, and application fallback boundaries.

Initial issues:

- FSIM-SI-001;
- FSIM-SI-002.

## Retraced expected behavior

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Canonical Change Record classifies a Rust body edit, source owner, file/item scope, prior/new revisions, and unknown behavioral consumers | FOREST-002 |
| Scope | Exact changed item and package are selected; absent a behavioral test mapping, safety widens through the consuming application relationship | SCOPE-001 |
| Evidence | Cargo and application evidence are observed; individual behavioral test mapping is not observed | EVIDENCE-001 |
| Causality | Source body change is observed; downstream behavioral effect is unknown; source API change is not observed | CAUSALITY-001 |
| Prediction | `ledger-core` owner work is predicted; consumer runtime validation is conservatively selected; exact downstream compiler work is uncertain | PREDICTION-001 |
| Validation | Package checks plus required consumer tests and repository gates are selected; selected/full-reference difference is shown | VALIDATION-001 |
| Planning | Separate Cargo invocation plans exist for each workspace; no universal unit graph is created | PLANNING-001 |
| Resolution | Disposition is `validate`; no approval or action exists | RESOLUTION-001 |
| Public view | `ferris check` shows a non-executable plan and explains widening from missing behavioral mapping | VIEW-001 |

The comment-only control produces no Rust compilation closure when owner
evidence proves no generated, policy, documentation, or gate consequence.
Mandatory repository gates remain explicit.

## Assertions

- [x] Source API stability does not prove behavioral non-impact.
- [x] Missing behavioral mappings do not disappear.
- [x] Cross-workspace consumer validation remains visible.
- [x] `check` does not execute by default.
- [x] No performance or correctness claim is made.

## Specification changes

- FSIM-SCR-001;
- FSIM-SCR-002; and
- FSIM-SCR-003.

## Claim boundary

This predicts specification behavior only. No Cargo command or Ferris
implementation ran.
