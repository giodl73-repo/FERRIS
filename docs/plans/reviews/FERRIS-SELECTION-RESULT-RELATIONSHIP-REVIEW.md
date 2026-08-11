# Ferris Selection and Result Relationship Review

Date: 2026-08-11
Scope: Pulse 11 remediation after FHIF-024
Disposition: Validated; new replacement fixture pending
Implementation authority: No expansion

## Result

`ferris.command-result/v2` envelopes now expose explicit selection,
invocation, and result identities. Doctor success and post-read failures
derive request identity from the same portable selection identity, while
outcome fields remain bound only to result identity.

Pre-selection and invalid-CLI paths use privacy-safe portable selection
identities without retaining executable or checkout roots.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed formatting, 29 core tests, 12 CLI
tests, lint with warnings denied, and changed-file diff checks. The two
process-helper tests remain ignored except when invoked by their
timeout/output-bound tests.

## Held-out state

- FHIF-013 through FHIF-024, excluding nonexistent FHIF-019, are quarantined
  development or invalid fixture evidence; never rerun or rescore.
- next score: requires a newly designed and sealed fixture with a new ID.

Validation Checker withholds a doctor held-out pass.
