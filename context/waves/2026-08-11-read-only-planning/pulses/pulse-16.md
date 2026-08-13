# Pulse 16: Profile Diff Held-Out Design

Status: Complete; public contract and custody protocol approved
Implementation authority: Documentation and independent evaluation design only

## Goal and authority

Define a public-safe, independently constructible held-out evaluation for the
existing Pulse 14 `profile-diff` command.

This pulse authorizes only:

- a frozen public scoring contract;
- an independent custody and anti-leak protocol;
- harness and scorer preflight requirements;
- a Windows and Unix process-cardinality contract; and
- a nine-role design review.

It does not authorize hidden fixture construction by the implementation team,
oracle access, scoring, implementation changes, profile generation, owner
execution, PLATFORM-001 advancement, or a held-out pass claim.

## Artifacts

- [Profile diff held-out program](../../../../docs/simulations/profile-diff-held-out/README.md)
- [Public scoring contract](../../../../docs/simulations/profile-diff-held-out/PUBLIC_CONTRACT.md)
- [Custody and preflight protocol](../../../../docs/simulations/profile-diff-held-out/CUSTODY_AND_PREFLIGHT.md)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-16-ROLE-REVIEW.md)

## Frozen design

The contract requires:

- exactly 56 independently constructed cases;
- one Windows and one Unix execution per case;
- exactly 112 durable Ferris process records;
- complete success, difference, invalid, unsupported, incomplete, and blocked
  result coverage;
- section, structure, pointer, validation, bound, identity, privacy, and stream
  controls;
- complete raw section-value canary inspection;
- scorer qualification before oracle release;
- one irreversible first score at an immutable cutoff; and
- permanent quarantine after any valid failure or invalid attempt.

## Acceptance

- the public contract exposes no hidden input or oracle predicate;
- command, platform, process, result, privacy, identity, and bound expectations
  are explicit;
- harness collection failures cannot become implementation scores;
- scorer-layout failures are qualified before hidden scoring;
- no retry, tuning, debugging, or favorable variant selection is allowed;
- public-safe results cannot disclose hidden case detail;
- all nine roles accept the design-only boundary; and
- changed documentation passes repository validation.

## Stop conditions

Stop rather than widening this pulse if work requires:

- creating or viewing hidden inputs, canaries, expected records, or predicates;
- executing Ferris against a candidate held-out package;
- changing product code or development fixtures;
- assigning a pass or fail without independent custody;
- weakening exact cardinality or cross-platform requirements;
- rescoring a failed or invalid package; or
- claiming compatibility, support, approval, production readiness, or
  PLATFORM-001 advancement.

## Remaining gate

An independent validation owner must construct, qualify, freeze, execute, and
score a candidate package under the published protocol. Until then, Ferris
continues to state that no held-out `profile-diff` claim exists.
