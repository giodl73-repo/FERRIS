# Ferris Specification Simulation Program

Status: Active
Implementation authority: None
Depends on: complete 22-specification Draft spine

## Purpose

The program pressure-tests Ferris specifications by asking AI and human
reviewers to hand-derive expected behavior from frozen scenarios before code.

It seeks:

- contradictions;
- ambiguous authority;
- missing records or fields;
- unsafe defaults;
- ownership leaks;
- hidden scope narrowing;
- missing failure states;
- unverifiable claims;
- public UX ambiguity; and
- incomplete removal or rollback.

## Non-goals

- writing Ferris product code;
- creating executable test harnesses;
- claiming runtime conformance;
- benchmarking;
- contacting connectors or external owner systems;
- advancing any specification to Proposed; or
- replacing later held-out implementation testing.

## Scenario artifact

Each scenario MUST contain:

- scenario ID, title, wave, revision, and state;
- locked fixture and explicit unknowns;
- question and expected distinguishing behavior;
- governing specification set;
- hand-derived record and state trace;
- predicted human and machine view;
- negative or matched control where applicable;
- assertions and claim boundary;
- Simulation Issues;
- Specification Change Records applied; and
- retrace disposition.

## Claim states

Simulation uses:

| State | Meaning |
|---|---|
| declared | fixed directly by the fixture |
| spec-derived | exact consequence of a cited normative rule |
| simulated | AI or human trace produced from declared and spec-derived inputs |
| ambiguous | two or more conforming traces remain possible |
| unsupported | the specs explicitly do not support the fixture |
| blocked | a required record, owner, evidence, policy, or decision is absent |
| contradicted | cited specifications require incompatible outcomes |
| superseded | replaced by a later fixture revision or spec amendment |

`Simulated` MUST NOT be relabeled `observed`.

## Simulation issue vocabulary

Simulation issues use `FSIM-SI-NNN` and one primary type:

- gap;
- ambiguity;
- contradiction;
- unsafe default;
- missing failure state;
- ownership leak;
- unverifiable requirement;
- naming or UX;
- known Proposed-status blocker; or
- scenario defect.

Severity is:

- **P0:** permits unsafe, unauthorized, secret-bearing, cross-tenant, or
  correctness-corrupting behavior;
- **P1:** permits materially different conforming behavior or loses required
  work, validation, identity, trust, or rollback;
- **P2:** impairs diagnostics, portability, maintenance, or predictable UX;
- **P3:** editorial or low-impact precision.

Research findings retain `FERRIS-*` IDs. GitHub issues are not required for
simulation bookkeeping.

## Specification Change Records

Normative fixes use `FSIM-SCR-NNN` and MUST record:

- triggering issue IDs;
- affected specs;
- before and after rule;
- authority and compatibility consequence;
- scenarios requiring retrace;
- role concerns;
- validation; and
- disposition.

One SCR MAY resolve several related issues. A scenario issue may close without
a normative change only when the ledger records why the existing rule is
sufficient or the scenario was defective.

## Wave protocol

1. **Select a bounded concern.**
2. **Freeze three to six fixtures**, including a negative or adversarial case.
3. **Lock the specification versions** and evidence cutoff.
4. **Trace independently** from source records through public view.
5. **Compare traces** and classify disagreements.
6. **Record issues** without editing the fixture to fit the preferred answer.
7. **Review the wave** through applicable `.roles`.
8. **Approve SCRs** and amend specifications.
9. **Retrace all affected scenarios.**
10. **Close the wave** only when open issues are resolved, deferred with owner
    and gate, or converted into a later-wave fixture.

## Ferris Wheel

The cross-wave regression cycle is named the **Ferris Wheel**.

Each approved Specification Change Record rotates every affected earlier
fixture through:

```text
reselect affected scenarios
  -> retrace from the frozen fixture
    -> compare prior and current outcomes
      -> retain intended change and detect regression
        -> update issue and change-record disposition
```

One Wheel turn is complete only when all affected scenarios have an
unambiguous current trace, intended differences are attributed to the exact
Specification Change Record, and no new P0 or P1 regression remains open.

## Complexity progression

| Wave class | Focus |
|---|---|
| Foundation | one change, one owner boundary, read-only planning |
| Composition | multiple workspaces, contracts, profiles, and validation |
| Evidence | identity, roots, refs, adapters, projections, and stale state |
| Predictive | causality, held-out prediction, AI, and economics |
| Controlled action | governance, trust, approval, execution, rollback |
| Integration | connectors, MCP, external owners, and packets |
| Lifecycle | renewal, revocation, deletion, removal, and incident recovery |
| Adversarial | races, partial failure, cross-tenant, unsupported, and combined faults |

## Coverage ledger

The wave registry MUST track coverage across:

- all 22 specifications;
- all nine roles;
- Windows and Unix assumptions;
- positive, negative, failure, unsupported, stale, version-skew, permission,
  tenant, rollback, deletion, and removal classes;
- `ferris`, `cargo ferris`, and MCP views;
- selected and full-reference scope;
- AI and non-AI decisions; and
- no-change, accepted-change, rejected-change, and superseded-change cases.

## Completion rule

Simulation may establish Draft consistency and reveal defects. It cannot
establish runtime behavior or satisfy CONFORMANCE-001 implementation fixtures.

The program is complete only when:

1. every specification has direct and compositional scenarios;
2. every public command and canonical state is exercised;
3. every failure class has a frozen fixture;
4. every P0 and P1 issue is resolved or explicitly blocks progression;
5. all SCRs have been retraced;
6. all nine roles review the final simulation corpus; and
7. the resulting held-out implementation fixture set is frozen separately.
