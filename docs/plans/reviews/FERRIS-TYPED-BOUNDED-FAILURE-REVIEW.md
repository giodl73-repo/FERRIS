# Ferris Typed Bounded Failure Review

Date: 2026-08-11
Scope: Pulse 09 remediation after FHIF-016
Disposition: Validated; new replacement fixture pending
Implementation authority: No expansion

## Result

Doctor now preserves typed retained-output evidence across success and
bounded-failure paths. The evidence names retained, observed,
observed-omitted, and unobserved-unknown bytes plus completion, truncation,
read failure, framing, digest, and termination reason for both streams.

Failure invocation identity now binds the complete typed diagnostic rather
than only the manifest selection and static process contract.

Output-limit evidence is scheduler-independent: the overflowing stream is
reported as the retained limit plus one observed omitted byte, while an
non-overflowing peer stream is explicitly unknown. Direct-child cleanup is
nonblocking and bounded to one second, and its completion state is retained.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed formatting, 26 core tests, 12 CLI
tests, lint with warnings denied, and changed-file diff checks. The two
process-helper tests remain ignored except when invoked by their
timeout/output-bound tests.

## Held-out state

- FHIF-013 through FHIF-016: failed development evidence; never rescore.
- FHIF-017: invalid harness execution; quarantined without implementation
  score; never rerun.
- next score: requires a newly designed and sealed fixture with a new ID.

Validation Checker withholds a doctor held-out pass.
