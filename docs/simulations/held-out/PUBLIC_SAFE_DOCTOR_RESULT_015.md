# Public-Safe Replacement Doctor Result 015

Manifest revision: 11
Fixture: FHIF-023
Private fixture manifest revision: `FHIF-023-r01`
Package SHA-256: `39dda2dd18c5b12de5f76a2dece221369ab87b98228918cf0975306d93e66640`
Cutoff: `a8ca67daccc1076c385c1d1679b660432f83a30c`
Tag: `ferris-canonical-command-result-pulse-10-cutoff`
Requirement: `P10-CANONICAL-COMMAND-RESULT`
Disposition: Invalid harness; quarantined without implementation score

Full-stream parser and oracle branch qualification preceded freeze. Inputs and
oracle were frozen before execution. All 32 expected process records were
captured. The public run observed 14 success/0, 17 invalid/2, and one
unparsed actual-exit-0 result. Aggregate public-output SHA-256:
`bd079a0f5d4e125248b4c7a41d90ed69cb6014993d248c7f4838fe8c85253493`.

The custodian classified the result as a fixture-oracle qualification defect.

Public remediation: qualify all contract-equivalent evidence layouts and
bounded controls before replacement freeze.

No prohibited material was read. No hidden input, script, seeded value,
command detail, expected record, or oracle predicate is disclosed. FHIF-023
MUST NOT be rerun or rescored.
