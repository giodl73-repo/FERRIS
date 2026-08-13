# Pulse 17: Independent Held-Out Program

Status: Complete; valid implementation failure; one-score program closed
Implementation authority: None for hidden material, scoring, repository modification, or execution

## Completed independent Stage A

Independent validation passed 789 assertions against cutoff
`4371f4f6eb54097bff9badb29278c530d49e2f36`, reported zero public
blockers, and preserved first-score integrity. The exact three-slot public
selection is frozen in the
[repository-selection binding](../../../../docs/simulations/profile-diff-held-out/REPOSITORY_SELECTION_BINDING.md).

## Completed independent Stage B/C

Independent custody completed the immutable Stage B/C program against cutoff
`8cbb5356fd7b3acca435bc9fad4e97dabab66bb5` with fixture
`P17-R3-D6B553CBC3B1240B673B8190`.

- exactly 112 processes were collected: Windows 56 and Ubuntu 24.04 56;
- missing, duplicate, retried, extra, launch-failure, abnormal-termination,
  stream-failure, and privacy-hit counts were all zero;
- aggregate result classes summed to 112;
- the hosted, cross-target/`no_std`, and native-bound repository workflows
  passed, with 28 owner commands and zero owner, comparison, or lifecycle
  failures;
- repository cleanup completed; and
- first-score attempt 1 and scorer attempt 1 produced a valid implementation
  failure with the sole public-safe category `process-exit-agreement`.

The [public-safe result](../../../../docs/simulations/profile-diff-held-out/PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
contains the permitted aggregates and immutable seals. Repository workflows
passed, but the command score did not. This is a valid implementation failure,
not invalid custody, and it cannot be converted into a pass.

## Ready public material

- [Contract revision 3](../../../../docs/simulations/profile-diff-held-out/PUBLIC_CONTRACT.md)
- [Custody and preflight protocol](../../../../docs/simulations/profile-diff-held-out/CUSTODY_AND_PREFLIGHT.md)
- [Exact identity contract](../../../../docs/simulations/profile-diff-held-out/IDENTITY.md)
- [Draft 2020-12 schemas](../../../../docs/simulations/profile-diff-held-out/schemas/README.md)
- [Three-public-repository workflow](../../../../docs/simulations/profile-diff-held-out/THREE_REPOSITORY_WORKFLOW.md)
- [Frozen repository-selection binding](../../../../docs/simulations/profile-diff-held-out/REPOSITORY_SELECTION_BINDING.md)
- [Public scorer qualification fixtures](../../../../docs/simulations/profile-diff-held-out/fixtures/README.md)
- implementation-owned platform family evidence through Pulse 16

## Historical invalid attempt

The first independent Stage B attempt reached sealed qualification but was
invalidated before execution because the cross-target license digest had been
computed from CRLF-transformed bytes rather than the frozen LF Git blob. The
binding now uses the verified LF digest
`sha256:403c53069750101aeb9df7e15f127056ceaf7e4e92d0b919a1f4c084afd5f1d4`.
The custodian reported zero of 112 scored processes, so first-score integrity
remained intact for the later completed program.

That invalid custody attempt is distinct from the later valid implementation
failure. The completed fixture and custody artifacts are sealed in quarantine.
Retry, rescore, and reuse are prohibited. No per-case or hidden material is
disclosed.
