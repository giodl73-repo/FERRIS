# Pulse 17: Independent Held-Out Program

Status: Stage A passed; repository binding corrected; blocked on independent resealing and execution
Implementation authority: None for hidden material, scoring, repository modification, or execution

## Completed independent Stage A

Independent validation passed 789 assertions against cutoff
`4371f4f6eb54097bff9badb29278c530d49e2f36`, reported zero public
blockers, and preserved first-score integrity. The exact three-slot public
selection is frozen in the
[repository-selection binding](../../../../docs/simulations/profile-diff-held-out/REPOSITORY_SELECTION_BINDING.md).

## Required external work

An independent custodian must:

1. bind the frozen 56-case public contract to hidden inputs and oracle rules;
2. construct independently sealed changes for the three frozen repositories;
3. qualify the sealed scorer;
4. execute the immutable Ferris binary once per case on Windows and Unix;
5. collect exactly 112 complete process records;
6. execute the three public-repository workflow under separately sealed
   changes; and
7. publish only the allowed score, validity, and limitation record.

The implementation team may not construct hidden cases, replace the frozen
repositories, modify their source, inspect canaries or predicates, retry a
first score, or self-certify this pulse.

## Ready public material

- [Contract revision 3](../../../../docs/simulations/profile-diff-held-out/PUBLIC_CONTRACT.md)
- [Custody and preflight protocol](../../../../docs/simulations/profile-diff-held-out/CUSTODY_AND_PREFLIGHT.md)
- [Exact identity contract](../../../../docs/simulations/profile-diff-held-out/IDENTITY.md)
- [Draft 2020-12 schemas](../../../../docs/simulations/profile-diff-held-out/schemas/README.md)
- [Three-public-repository workflow](../../../../docs/simulations/profile-diff-held-out/THREE_REPOSITORY_WORKFLOW.md)
- [Frozen repository-selection binding](../../../../docs/simulations/profile-diff-held-out/REPOSITORY_SELECTION_BINDING.md)
- [Public scorer qualification fixtures](../../../../docs/simulations/profile-diff-held-out/fixtures/README.md)
- implementation-owned platform family evidence through Pulse 16

## Blocker

The first independent Stage B attempt reached sealed qualification but was
invalidated before execution because the cross-target license digest had been
computed from CRLF-transformed bytes rather than the frozen LF Git blob. The
binding now uses the verified LF digest
`sha256:403c53069750101aeb9df7e15f127056ceaf7e4e92d0b919a1f4c084afd5f1d4`.
The custodian reported zero of 112 scored processes, so first-score integrity
remains intact.

No valid sealed package, executed repository workflow, 112-record collection,
or first score has been supplied. The pulse therefore remains blocked rather
than being converted into development evidence. The Stage A pass and
corrected repository binding are not execution or a held-out pass.
