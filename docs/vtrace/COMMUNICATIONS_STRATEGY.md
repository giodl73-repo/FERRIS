# Communications Strategy

## Purpose

Explain Ferris Go from controlled requirements and evidence without turning
projections into promises or user documentation into a substitute for the
specification.

## Surface Plan

| Surface ID | Source IDs | Audience | User Question | Generated Docs | Cadence | Owner | Status |
|---|---|---|---|---|---|---|---|
| COMMS-README-001 | GO-REQ-003/015; GO-SPEC-003/010 | contributors and leads | What does Ferris Go do and why does it matter? | `README.md` | every claim change | Product Value Governor | updated |
| COMMS-CONCEPTS-001 | GO-REQ-001 through 013 | maintainers and owners | Who owns planning, execution, policy, and proof? | `FERRIS_GO_EXECUTION_PLAN.md` | every architecture change | Ferris maintainer | updated |
| COMMS-EVIDENCE-001 | GO-REQ-014/015 | reviewers and leaders | Where did the hour target come from? | research evidence and VTRACE evidence | every cohort change | Evidence owner | updated |
| COMMS-DELIVERY-001 | GO-WP-000 through 007 | implementers | What can be built next and what is blocked? | wave package and dashboard | every pulse decision | Change Control Reviewer | updated |
| COMMS-PR-001 | all accepted IDs | PR reviewers | What decision does this PR request? | `FERRIS_GO_PR_PACKAGE.md` | every PR boundary change | Ferris maintainer | ready |

## Audience And Message

| Audience | Lead Message | Required Evidence | Avoid |
|---|---|---|---|
| Contributor | One command can run the minimum sufficient owner work before push and explain it. | Exact owner entrypoint and source identity. | "Ferris replaces CI." |
| Engineering lead | The target is roughly an hour back on a failed PR. | One prevented iteration plus separately measured remaining-tail reduction. | Adding the two projections without disjoint cohorts. |
| Repository owner | Ferris preserves owner topology and can be removed without workflow redesign. | Reconciliation and removal drill. | "Ferris knows better than the pipeline." |
| CI/platform owner | Balanced scheduling changes timing, never required scope. | Cross-profile conformance and cancellation receipts. | "Fail fast means cancel everything." |
| Reviewer/security | Plans are approved before execution; receipts fail closed. | Mutation tests and authority review. | "AI decides what is safe." |

## Approved Launch Language

> Ferris Go is a deterministic front door for working repository build systems.
> Our evaluation target is one fewer failed CI iteration per failed PR - about
> 41 minutes at the combined cohort median - plus 22–36 minutes removed from
> the remaining failure loop. Replay and shadow adoption must prove those
> benefits before they become product claims.

## Claim Ladder

| Level | Allowed Language | Required Gate |
|---|---|---|
| Design | "targets", "projects", "hypothesis" | Current evidence package. |
| Replay | "replay observed" with cohort and denominator | GO-VAL-004/005 pass. |
| Shadow | "shadow observed" with repository and period | GO-WP-007 owner acceptance. |
| Production | "saves" with population-weighted interval | Required adoption plus monitoring window. |

## Pull Request Story

1. Existing build systems work; the missing layer is deterministic
   coordination and evidence.
2. Failed PRs are the value center: 7.8 iterations and 4.06 failed iterations
   on average in the sampled failed-PR cohort.
3. Ferris attacks two different costs: prevent one eligible failed iteration
   before push, then shorten eligible remaining tails.
4. Owner commands, policy, human gates, and publication remain authoritative.
5. The change is delivered in separate, reversible, evidence-gated waves.

The ready-to-use title, body, reviewer guide, and proposed PR stack are in
[`FERRIS_GO_PR_PACKAGE.md`](../plans/reviews/FERRIS_GO_PR_PACKAGE.md).

## Release And Adoption Communications

- Publish no hour-saved headline before shadow evidence.
- Every metric includes repository, time window, eligibility, denominator,
  uncertainty, and whether it is projected, replayed, shadowed, or production.
- Report negative results and narrowed scope with the same prominence as wins.
- Keep technical completion distinct from merge readiness in CLI and docs.
