# Ferris Bounded Machine Framing Review

Date: 2026-08-11
Scope: Pulse 08 remediation after FHIF-015
Disposition: Validated; new replacement fixture pending
Implementation authority: No expansion

## Result

The former `stdout + NUL + stderr` digest frame was ambiguous because retained
owner output may contain NUL bytes. Ferris now domain-separates the evidence
and length-prefixes both bounded byte streams before hashing them.

The executable binding now names only implemented read-only commands and
defines machine-output stream ownership. Success emits one complete JSON value
on stdout; non-success emits one complete JSON value on stderr.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed formatting, 25 core tests, 12 CLI
tests, lint with warnings denied, and diff checks. The two process-helper tests
remain ignored except when invoked by their timeout/output-bound tests.

## Held-out state

- FHIF-013: failed development evidence; never rescore.
- FHIF-014: failed development evidence; never rescore.
- FHIF-015: failed development evidence; never rescore.
- next score: requires a newly designed and sealed fixture with a new ID.

Validation Checker withholds a doctor held-out pass.
