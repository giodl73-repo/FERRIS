# Pulse 08: Unambiguous Bounded Machine Framing

Status: Validated on Windows and Unix; new replacement fixture pending
Implementation authority: Corrective only; no capability expansion

## Trigger

Independent replacement FHIF-015 failed cutoff
`01c9e52e620769e98bf96d06ebf2ea0f96575ee8` under
`P07-TYPED-DOCTOR-COMPLETE-BOUND-IDENTITY`. It is now development evidence
and MUST NOT be rescored.

No hidden input or oracle predicate was released. The public remediation was
to correct bounded machine framing before evaluating complete typed identity
and binding.

## Correction

- replace ambiguous `stdout + NUL + stderr` evidence framing;
- prefix the frame with the `ferris.command-output/v1` domain;
- encode each retained stream length as an unsigned 64-bit little-endian
  value immediately before that stream;
- expose `length-prefixed-stdout-stderr/v1` as the report framing contract;
- bind the revised framing into success and failure identities; and
- align the executable binding with the implemented read-only CLI and stream
  contract.

## Acceptance

- distinct stdout/stderr byte pairs cannot collide by moving NUL bytes across
  the stream boundary;
- all existing manifest and process bounds remain unchanged;
- machine invocations emit one complete typed JSON value on exactly one
  documented stream;
- Windows and Unix formatting, tests, lint, and diff checks pass; and
- a newly sealed replacement fixture with a new ID passes before any held-out
  doctor claim is made.
