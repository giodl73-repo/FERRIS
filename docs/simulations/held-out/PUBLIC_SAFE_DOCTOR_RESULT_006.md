# Public-Safe Blind Doctor Fixture Result 006

Manifest revision: 2
Fixture: FHIF-013
Package SHA-256: `28b17e77a3ebc978bd724ecd4efced608c41f946d12bad196e9e628959d57114`
Cutoff: `95a0b905fb31c908a241d57ae17d984e16d8c053`
Tag: `ferris-passive-doctor-hardening-pulse-05-cutoff`
Requirement: `P05-PASSIVE-DOCTOR-BOUNDED-IDENTITY`
Disposition: Fail; reclassified as development evidence

The custodian verified the cutoff, tag, unchanged source, and sealed package
before execution. The public run observed five success/0, two unsupported/4,
and four blocked/7 cases. Aggregate public-output SHA-256:
`abbe53bc0c73e36cea77f497f55ed283032675bae0251f9aabe3d6cf90197669`.

No hidden input, script, seed, command detail, expected record, or oracle
predicate is disclosed. The only released remediation was:

- tighten Cargo evidence classification; and
- complete post-read report identity.

FHIF-013 MUST NOT be rescored. A replacement fixture requires a new opaque ID
and independently frozen oracle.
