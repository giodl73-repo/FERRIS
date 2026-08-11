# Ferris Universal Non-Success Review

Date: 2026-08-11
Scope: Pulse 12 remediation after FHIF-025
Disposition: Validated; new replacement fixture pending
Implementation authority: No expansion

## Result

Every non-success CLI invocation now emits exactly one complete UTF-8
`ferris.command-result/v2` envelope on stderr, including Clap syntax failures
and parsed commands that requested human success rendering.

The process exit is taken from that emitted record. Help and version remain
successful informational displays.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed formatting, 29 core tests, 13 CLI
tests, lint with warnings denied, and changed-file diff checks. The two
process-helper tests remain ignored except when invoked by their
timeout/output-bound tests.

## Held-out state

- FHIF-013 through FHIF-025, excluding nonexistent FHIF-019, are quarantined
  development or invalid fixture evidence; never rerun or rescore.
- next score: requires a newly designed and sealed fixture with a new ID.

Validation Checker withholds a doctor held-out pass.
