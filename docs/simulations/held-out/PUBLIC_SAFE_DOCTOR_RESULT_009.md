# Public-Safe Replacement Doctor Result 009

Manifest revision: 5
Fixture: FHIF-016
Private fixture manifest revision: 1
Package SHA-256: `f076076bf32737539e63dd2252c5824be763e8969468050828ad2f7c478d42d2`
Cutoff: `4759fd549991d474c1fc8c6af14f9aef632490b7`
Tag: `ferris-bounded-machine-framing-pulse-08-cutoff`
Requirement: `P08-UNAMBIGUOUS-BOUNDED-MACHINE-FRAMING`
Disposition: Fail; reclassified as development evidence

Inputs and oracle were frozen before first execution. The public run observed
three success/0, one invalid/2, and five blocked/7 results. Aggregate
public-output SHA-256:
`15af6a9edc21591261e7cf321668f28f141161f370de4353e3aec1b4e5711d7a`.

Public remediation: ensure passive doctor machine output remains
contract-complete, unambiguous, bounded, and on its documented stream.

No prohibited material was read. No hidden input, script, seeded value,
command detail, expected record, or oracle predicate is disclosed. FHIF-016
MUST NOT be rescored.
