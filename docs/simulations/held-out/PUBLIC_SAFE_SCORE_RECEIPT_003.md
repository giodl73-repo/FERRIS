# Held-Out Public-Safe Score Receipt 003

Cutoff: `c3590a39fd053a66996909b87eaf7ca7ac73ded4`
Tag: `ferris-read-only-hardening-pulse-03-cutoff`
Disposition: All applicable Pulse 03 fixtures passed

This receipt exposes no hidden input, edit, failure seed, expected output,
oracle predicate, or private scoring note.

## Verification

- cutoff and tag: matched;
- environment and required CLI binding: matched; and
- sealed package digests: 12 of 12 matched.

## Aggregate

- applicable: 2;
- pass: 2;
- fail: 0;
- applicable blocked or invalid: 0; and
- out of scope and not executed: 10.

## Applicable results

| Fixture | Requirement | Disposition | Observed class / exit | Public-output SHA-256 |
|---|---|---|---|---|
| FHIF-009 | `P03-LOCAL-PLAN-HARDENING` | pass | success / 0 | `6499b9ea2d3695c3a5e4fcb7316be0ce29047bb73fc01273652ebe182897f03f` |
| FHIF-012 | `P03-BOUNDED-READONLY-PARITY` | pass | success / 0; expected invalid control / 2 | `1f327a5fcfde279db8d8024e72f2069b6e95218cf3564908cb0bf0a4b2552ffe` |

Out of scope and not executed: FHIF-001 through FHIF-008, FHIF-010, and
FHIF-011. They count as neither passes nor failures.
