# Public-Safe Replacement Doctor Result 008

Manifest revision: 4
Fixture: FHIF-015
Package SHA-256: `3c6d53f2dba44b8520b8caa495eeafad01c8f6aea23ed3039ab9d2213ad29104`
Cutoff: `01c9e52e620769e98bf96d06ebf2ea0f96575ee8`
Tag: `ferris-typed-doctor-identity-pulse-07-cutoff`
Requirement: `P07-TYPED-DOCTOR-COMPLETE-BOUND-IDENTITY`
Disposition: Fail; reclassified as development evidence

Inputs and oracle were frozen before first execution. The public run observed
22 invalid/2 results. Aggregate public-output SHA-256:
`fc068ac13b3ffeb2ca472e9e49bb8ab72e515afb78e8c20a3e8678446c2feb4e`.

Public remediation: correct bounded machine framing before evaluating complete
typed identity and binding.

No hidden input, script, seeded value, command detail, expected record, or
oracle predicate is disclosed. FHIF-015 MUST NOT be rescored.
