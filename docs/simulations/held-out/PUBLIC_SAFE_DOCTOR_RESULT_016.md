# Public-Safe Replacement Doctor Result 016

Manifest revision: 12
Fixture: FHIF-024
Private fixture manifest revision: `FHIF-024-r01`
Package SHA-256: `8da32ce6049ca998220aa51c50f2311a652892a8315e0dcc09179a480ed86b02`
Implementation cutoff: `a8ca67daccc1076c385c1d1679b660432f83a30c`
Implementation tag: `ferris-canonical-command-result-pulse-10-cutoff`
Contract clarification: `8552d2d9c2a1d0080f0da35c21119e130a7538be`
Contract tag: `ferris-canonical-evidence-contract-pulse-10b`
Requirement: `P10B-CANONICAL-EVIDENCE-LOCATIONS`
Disposition: Fail; reclassified as development evidence

Semantic-layout and full branch/rule qualification preceded freeze. Inputs
and oracle were frozen before execution. All 34 expected process records were
captured. The public run observed seven success/0, nine invalid/2, nine
unsupported/4, seven blocked/7, and two unparsed actual-exit-2 results.
Aggregate public-output SHA-256:
`f89bf975dc1e08d64e46d782d1ed5be42a1ccba73024978d5b9db3e19958f69a`.

Public categories: stream placement, success evidence location, failure
evidence location, bounded counts and unknown semantics, framing and digest,
and identity relationships. The two unparsed outcomes were indeterminate.

Public remediation: align machine-mode envelopes with designated streams,
canonical typed evidence fields, bounded and unknown semantics, stable
framing, and coherent document, result, and invocation relationships.

No prohibited material was read. No hidden input, script, seeded value,
command detail, expected record, or oracle predicate is disclosed. FHIF-024
MUST NOT be rerun or rescored.
