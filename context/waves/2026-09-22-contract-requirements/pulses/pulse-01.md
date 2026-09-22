# Pulse 01: Explicit Contract Requirement Evaluation

Status: Complete

## User outcome

Allow an adapter or release gate to check whether one installed Ferris binary
retains the exact accepted and emitted schema handling it depends on.

## Control record

- Product outcome: replace adopter-specific catalog comparison code with one
  deterministic local check.
- Maximum effort: one requirements record, one report, one optional CLI input,
  two schemas, one synthetic fixture, focused docs, and tests.
- Completion test: strict bounded input, deterministic normalized output,
  success and report-bearing difference, unknown and wrong-direction controls,
  and three-entrypoint parity.
- Abandonment condition: stop if the work requires semantic comparison,
  repository inference, migration, negotiation, or a support-duration claim.
- Product Value Governor: `continue-within-budget`.

## Result

`ferris contracts --requirements <JSON>` evaluates
`ferris.contract-requirements/v1` against the installed catalog and emits
`ferris.contract-compatibility-report/v1`. The five retained synthetic
repositories normalize to four exact direction-qualified requirements, all of
which the current binary satisfies. An unknown schema and a known
wrong-direction requirement produce a deterministic difference with exit code
1 and retain the full report.

This is the first replayable release gate, not a multi-release window. A later
distinct Ferris release must pass the same fixture before such evidence exists.
