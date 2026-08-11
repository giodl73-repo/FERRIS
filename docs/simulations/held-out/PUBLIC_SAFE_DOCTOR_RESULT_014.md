# Public-Safe Replacement Doctor Result 014

Manifest revision: 10
Fixture: FHIF-022
Private fixture manifest revision: `FHIF-022-r01`
Package SHA-256: `796121976ef30cefcc3526f8030df0119d002e9745322ee0788c9066038580cc`
Cutoff: `a8ca67daccc1076c385c1d1679b660432f83a30c`
Tag: `ferris-canonical-command-result-pulse-10-cutoff`
Requirement: `P10-CANONICAL-COMMAND-RESULT`
Disposition: Invalid harness; quarantined without implementation score

Full exact-path branch and oracle-rule qualification preceded freeze. Inputs
and oracle were frozen before execution. All 33 expected process records were
captured. The public run observed 16 success/0, 16 invalid/2, and one blocked/7
result. Aggregate public-output SHA-256:
`31baa989db76ccf73bb7eb819b114af51f1530264b99e4e171e6421b4d47b80f`.

The custodian classified the result as a scoring-path JSON record-parser
defect.

Public remediation: qualify complete JSON-envelope parsing and seal a new
fixture.

No prohibited material was read. No hidden input, script, seeded value,
command detail, expected record, or oracle predicate is disclosed. FHIF-022
MUST NOT be rerun or rescored.
