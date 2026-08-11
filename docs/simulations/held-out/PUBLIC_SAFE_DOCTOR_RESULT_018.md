# Public-Safe Replacement Doctor Result 018

Manifest revision: 14
Fixture: FHIF-026
Private fixture manifest revision: `FHIF-026-r01`
Package SHA-256: `d9f51b41b48a8fe7942d793b33d55dc15d763a5fe6cd8cb06a689bd1d7538e26`
Cutoff: `9f93d08db1d6cd5c4f05c9bb33a38db6efc941e1`
Tag: `ferris-universal-non-success-pulse-12-cutoff`
Requirement: `P12-UNIVERSAL-TYPED-NON-SUCCESS`
Disposition: Invalid harness; quarantined without implementation score

Qualification preceded freeze, and inputs and oracle were frozen before
execution. The harness captured 43 of 48 expected process records. Observed
public results were 12 success/0, 24 invalid/2, one unsupported/4, four
blocked/7, and two unparsed actual-exit-0 outcomes. Aggregate public-output
SHA-256:
`1862ca4654dbaec6e1b2607ce0f22b50a2d81d2abc20df2a2c66462a4d51fd57`.

Public remediation: qualify collection cardinality and uninterrupted
aggregation in a new fixture.

No prohibited material was read. No hidden input, script, seeded value,
command detail, expected record, or oracle predicate is disclosed. FHIF-026
MUST NOT be rerun or rescored.
