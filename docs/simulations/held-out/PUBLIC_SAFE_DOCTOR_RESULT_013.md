# Public-Safe Replacement Doctor Result 013

Manifest revision: 9
Fixture: FHIF-021
Private fixture manifest revision: `FHIF-021-r01`
Package SHA-256: `7d731340b7004373e3425bda0c89556168374213eeb3e2a2a6707194b6b4a8b2`
Cutoff: `a8ca67daccc1076c385c1d1679b660432f83a30c`
Tag: `ferris-canonical-command-result-pulse-10-cutoff`
Requirement: `P10-CANONICAL-COMMAND-RESULT`
Disposition: Invalid scoring path; quarantined without implementation score

Qualification passed before freeze, and inputs and oracle were frozen before
execution. All 28 expected process records were captured. The public run
observed 18 success/0 and ten invalid/2 results. Aggregate public-output
SHA-256:
`06438606d4d25be644f0c85e6bf38f5ba6cc9b51f183f4d8ccfa8fe348108d27`.

Post-score custody review classified the result confidently as a fixture,
oracle, or scoring-path defect rather than an implementation-contract
mismatch.

Public remediation: validate the complete scoring path before freeze, then use
a newly sealed fixture without rerunning FHIF-021.

No prohibited material was read. No hidden input, script, seeded value,
command detail, expected record, or oracle predicate is disclosed. FHIF-021
MUST NOT be rerun or rescored.
