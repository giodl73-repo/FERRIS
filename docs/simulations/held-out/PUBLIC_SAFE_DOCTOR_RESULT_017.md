# Public-Safe Replacement Doctor Result 017

Manifest revision: 13
Fixture: FHIF-025
Private fixture manifest revision: `FHIF-025-r01`
Package SHA-256: `9ea532ca7617461b8d3a6d93b1c2b3795dbed5a9144d5e30ddd974aa7a2b72fe`
Cutoff: `5158e12fa5bd939bd3c056bb317022601ece1735`
Tag: `ferris-selection-result-relationship-pulse-11-cutoff`
Requirement: `P11-SELECTION-INVOCATION-RESULT-RELATIONSHIP`
Disposition: Fail; reclassified as development evidence

All 48 qualification checks preceded freeze. Inputs and oracle were frozen
before execution. All 40 expected process records were captured. The public
run observed 27 success/0, ten invalid/2, one unsupported/4, one blocked/7,
and one unparsed actual-exit-2 result. Aggregate public-output SHA-256:
`dedd428ac86f84760f4ca6cddee7e79c6a312a8c1f0808c2196a4f0d4eae9a71`.

The custodian classified the unparsed result confidently as an implementation
machine-output violation. Public categories were v2 envelope completeness,
result classification, and exit consistency.

Public remediation: ensure every non-success invocation emits one valid UTF-8
`ferris.command-result/v2` envelope containing required identities,
classification, recorded exit, diagnostics, and null record, with class and
exit matching the process outcome.

No prohibited material was read. No hidden input, script, seeded value,
command detail, expected record, or oracle predicate is disclosed. FHIF-025
MUST NOT be rerun or rescored.
