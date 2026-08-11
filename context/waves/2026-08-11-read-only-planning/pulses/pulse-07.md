# Pulse 07: Typed-Record Doctor Identity

Status: Validated on Windows and Unix; new replacement fixture pending
Implementation authority: Corrective only; no capability expansion

## Trigger

Independent replacement FHIF-014 failed cutoff
`69c1f5529c2a98e235ca09be02fdf72082093a39` under
`P06-PASSIVE-DOCTOR-CANONICAL-BOUNDED-IDENTITY`. It is now development
evidence and MUST NOT be rescored.

No hidden input or oracle predicate was released. A direct public-contract
audit identified remaining avoidable ambiguity in manually enumerated report
identity inputs and canonical commit/date validation.

## Corrections

- construct the complete typed `DoctorReport` with an empty identity field;
- calculate `report_id` from that entire typed record;
- bind invocation identity to the resulting complete-record identity;
- require canonical lowercase Cargo commit evidence of the supported short or
  full length;
- validate release dates against Gregorian month lengths and leap years; and
- give oversized manifests a portable bounded-prefix selection identity
  rather than checkout-path identity.

## Acceptance

- changing any typed doctor record field changes report identity;
- owner-output, bounds, checks, unknowns, limitations, fallback, and evidence
  cannot be omitted accidentally from record identity;
- uppercase, noncanonical-length, and invalid-calendar Cargo evidence is
  unsupported;
- oversized-manifest failures have checkout-portable request identity;
- Windows and Unix formatting, tests, lint, and diff checks pass; and
- a newly sealed replacement fixture with a new ID passes before any held-out
  doctor claim is made.
