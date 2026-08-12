# Public-Safe Replacement Doctor Result 019

Manifest revision: 15
Fixture: FHIF-027
Private fixture manifest revision: `FHIF-027-r01`
Package SHA-256:
`174e9818847c7785943fcd0a03afd2c0b03df938c5e3540fe2ec3cba0023c772`
Cutoff: `9f93d08db1d6cd5c4f05c9bb33a38db6efc941e1`
Tag: `ferris-universal-non-success-pulse-12-cutoff`
Requirement: `P12-UNIVERSAL-TYPED-NON-SUCCESS`
Disposition: Invalid scorer infrastructure; quarantined without implementation
score

Cardinality-safe collection produced exactly 48 durable records with
`collection_status: complete`. The independent scorer then failed its
public-contract field-layout handling before implementation scoring.
Aggregate public-output SHA-256:
`edfcf05987d2ffceb82533de063a27e988eac02ef9376ac01701378630df4e51`.

Public remediation: correct the independent scorer's public-contract
field-layout handling and use a fresh replacement fixture.

No prohibited material was read. No hidden input, command matrix, seeded
value, expected record, or oracle predicate is disclosed. FHIF-027 MUST NOT be
rerun or rescored.
