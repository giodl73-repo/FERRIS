# Pulse 10b: Canonical Evidence Locations

Status: Public contract clarified; replacement fixture pending
Implementation authority: Documentation only; no executable change

## Trigger

FHIF-023 completed all frozen processes but its oracle qualification did not
cover contract-equivalent evidence layouts. It is invalid evaluation evidence
and MUST NOT be rerun or rescored.

## Clarification

- success owner-output evidence is under `record.evidence`;
- success configured limits and framing are under `record.bounds`;
- non-success bounded owner-output evidence is under
  `diagnostics[*].bounded_output`;
- non-success command-specific `record` is null; and
- JSON member order and pretty-print layout are non-semantic.

This clarification changes no executable behavior or implementation cutoff.
