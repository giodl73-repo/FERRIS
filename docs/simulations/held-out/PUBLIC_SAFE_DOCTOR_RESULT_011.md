# Public-Safe Replacement Doctor Result 011

Manifest revision: 7
Fixture: FHIF-018
Private fixture manifest revision: `FHIF-018-r01`
Package SHA-256: `2cfa05c8524fc544bba93f5bc8d67f0e4644f40a7430b107c2dcf04eea5f8b30`
Cutoff: `a0c2a5ba991f76ba51ecf59061bf1abf0c256a3c`
Tag: `ferris-typed-bounded-failure-pulse-09-cutoff`
Requirement: `P09-TYPED-BOUNDED-FAILURE-EVIDENCE`
Disposition: Fail; reclassified as development evidence

The transport preflight passed, and inputs and oracle were frozen before first
Ferris execution. The public run observed one invalid/2 result. Aggregate
public-output SHA-256:
`d786e6a6d64e558c67c1dfed6071f1533961ad2635dcdda6a243f33c8d8bb528`.

Public remediation: align the passive doctor machine interface with the
documented contract and resubmit a newly sealed fixture.

No prohibited material was read. No hidden input, script, seeded value,
command detail, expected record, or oracle predicate is disclosed. FHIF-018
MUST NOT be rerun or rescored.
