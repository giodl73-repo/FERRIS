# Public-Safe Replacement Doctor Result 012

Manifest revision: 8
Fixture: FHIF-020
Private fixture manifest revision: `FHIF-020-r01`
Package SHA-256: `e4809cc159d2549ae6e0eba5b8d4475bee7e7ebe9ed39f041414021c780fb957`
Cutoff: `a0c2a5ba991f76ba51ecf59061bf1abf0c256a3c`
Tag: `ferris-typed-bounded-failure-pulse-09-cutoff`
Requirement: `P09-TYPED-BOUNDED-FAILURE-EVIDENCE`
Disposition: Fail; reclassified as development evidence

Harness qualification passed before freeze. Inputs and oracle were frozen
before execution. All 18 expected Ferris process records were captured. The
public run observed 17 success/0 and one invalid/2 result. Aggregate
public-output SHA-256:
`473247a496c13156660653549787c71190a7d5c0f5a5910188862d1bf6c4ac5a`.

Public contract categories: identity binding, success evidence completeness,
bounded failure evidence completeness, result classification, and
determinism. The release custodian classified the failure as an
implementation-contract mismatch.

Public remediation: emit one stable, complete result record per invocation
and derive classification, identity binding, bounded evidence, and exit
behavior consistently from it.

No prohibited material was read. No hidden input, script, seeded value,
command detail, expected record, or oracle predicate is disclosed. FHIF-020
MUST NOT be rerun or rescored.
