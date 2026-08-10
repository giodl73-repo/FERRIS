# FSIM-011: Revoked Historical Evidence

Wave: W03
Revision: 1
State: Simulated
Claim state: simulated

## Question

How does Ferris treat an immutable root containing evidence that was accepted
when observed but whose signer or adapter is later revoked?

## Locked fixture

- application: `forge`
- repositories and workspaces: one producer and one consumer workspace
- source and change: no source change
- contracts and profiles: an artifact profile was previously eligible
- environment: unchanged
- policy: signer `S1` is revoked for future artifact use at time `T2`
- available evidence: root `R30` contains a valid-at-`T1` signed observation
  and a trust decision that accepted it for restore
- explicit unknowns: whether the underlying bytes are malicious
- negative or matched control: historical query asking what was accepted at
  `T1`

Changing the fixture requires a new revision.

## Governing specifications

- TRUST-001 revocation and refs and roots;
- IDENTITY-001 immutable roots and reuse boundary;
- EVIDENCE-001 observation requirements; and
- PLANNING-001 replan triggers.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Revocation record for signer `S1` becomes effective at `T2` | TRUST-001 |
| Scope | Named signer, artifact class, consumer, and future operation | Consumer-scoped trust |
| Evidence | `R30` still records the `T1` signature and decision | Historical facts are not rewritten |
| Causality | Revocation changes future eligibility, not historical existence | TRUST-001 |
| Prediction | Any prior reuse expectation becomes stale | PLANNING-001 replan trigger |
| Validation | Prior validation remains historical but cannot override revocation | Claims remain separate |
| Planning | A plan relying on `S1` is stale and must rebuild or block | Reuse boundary |
| Resolution | Candidate restore is denied for the named future use | Revoked trust result |
| Trust/action | Running or pending action follows explicit revocation policy | TRUST-001 propagation |
| Public view | Historical query shows accepted-at-`T1`; current eligibility shows revoked | Time and operation scope |

## Assertions

- [x] `R30` is not rewritten;
- [x] revocation denies future eligible use within scope;
- [x] historical validity does not imply current trust;
- [x] current denial does not claim the old signature never existed; and
- [x] unknown maliciousness remains unknown.

## Simulation issues

- None.

## Specification changes

- None.

## Retrace

The original specifications distinguish immutable history from future
eligibility without ambiguity.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
