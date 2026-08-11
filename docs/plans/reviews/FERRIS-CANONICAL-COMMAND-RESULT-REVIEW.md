# Ferris Canonical Command Result Review

Date: 2026-08-11
Scope: Pulse 10 remediation after FHIF-020
Disposition: Validated; new replacement fixture pending
Implementation authority: No expansion

## Result

Every command outcome now has one canonical `ferris.command-result/v1`
envelope. Its complete typed content determines `result_identity`; its result
class determines the recorded and actual process exit code.

Invocation identity identifies the normalized request rather than the
outcome. Invalid CLI requests normalize equivalent option forms and remove
executable and checkout roots while retaining privacy-safe distinction among
typed argument values.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed formatting, 28 core tests, 12 CLI
tests, lint with warnings denied, and changed-file diff checks. The two
process-helper tests remain ignored except when invoked by their
timeout/output-bound tests.

## Held-out state

- FHIF-013 through FHIF-018 and FHIF-020: quarantined development evidence;
  never rerun or rescore.
- FHIF-019: no fixture was created and no score exists.
- FHIF-021: invalid fixture, oracle, or scoring-path evidence; never rerun or
  rescore.
- FHIF-022: invalid scoring-path JSON parser; never rerun or rescore.
- next score: requires a newly designed and sealed fixture with a new ID.

Validation Checker withholds a doctor held-out pass.
