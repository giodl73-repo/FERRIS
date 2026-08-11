# Pulse 09: Typed Bounded Failure Evidence

Status: Validated on Windows and Unix; new replacement fixture pending
Implementation authority: Corrective only; no capability expansion

## Trigger

Independent replacement FHIF-016 failed cutoff
`4759fd549991d474c1fc8c6af14f9aef632490b7` under
`P08-UNAMBIGUOUS-BOUNDED-MACHINE-FRAMING`. It is now development evidence and
MUST NOT be rescored.

No hidden input or oracle predicate was released. The public remediation was
to keep passive-doctor machine output contract-complete, unambiguous, bounded,
and on its documented stream.

## Public-contract audit

The bounded-output contract requires retained and omitted/unknown evidence to
remain explicit. Doctor success records exposed configured limits but not
observed stream counts or completion state. Timeout and output-limit failures
discarded retained stream evidence, and failure invocation identity did not
bind the complete emitted diagnostic.

## Corrections

- expose retained, observed, observed-omitted, and unobserved-unknown byte
  state plus completion and truncation for both owner-output streams;
- retain typed bounded-output evidence on completed, timeout, output-bound,
  and post-start stream-read-failure diagnostic paths;
- preserve the length-prefixed digest of the retained stream pair;
- stop a reader deterministically at its configured limit and settle both
  stream captures after process termination; and
- normalize output-bound evidence to the overflowing stream's retained limit
  plus one observed omitted byte while marking non-overflowing peer bytes
  unknown;
- use nonblocking, one-second-bounded direct-child termination cleanup and
  report whether cleanup completed; and
- bind the complete typed diagnostic, including bounded evidence, into doctor
  failure invocation identity.

## Acceptance

- success records state exact observed and retained stream counts;
- bounded failures retain a typed evidence record and digest;
- timeout and output-bound termination remain distinguishable;
- changing a failure diagnostic or its bounded evidence changes invocation
  identity;
- Windows and Unix formatting, tests, lint, and changed-file diff checks pass;
  and
- a newly sealed replacement fixture with a new ID passes before any held-out
  doctor claim is made.
