# Held-Out Public-Safe Score Receipt 005

Cutoff: `95a0b905fb31c908a241d57ae17d984e16d8c053`
Tag: `ferris-passive-doctor-hardening-pulse-05-cutoff`
Disposition: All applicable Pulse 05 fixtures passed

This receipt exposes no hidden input, edit, failure seed, expected output,
oracle predicate, or private scoring note.

## Verification

- cutoff and tag: matched;
- checkout and command bindings: matched; and
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
| FHIF-009 | `P05-LOCAL-PLAN-OWNER-CONTEXT` | pass | success / 0 | `4384719cb4299be1a802300ac78ef0429e88f460e7af782800573e6346e04ad5` |
| FHIF-012 | `P05-BOUNDED-READONLY-OWNER-CONTEXT` | pass | success / 0; expected invalid control / 2 | `14fb52b748974939b8164f0861adfeb9a02e8df434c06b117fbdad098348a006` |

Out of scope and not executed: FHIF-001 through FHIF-008, FHIF-010, and
FHIF-011.

No existing sealed fixture specifically covers passive doctor. This receipt
therefore makes no held-out doctor claim.
