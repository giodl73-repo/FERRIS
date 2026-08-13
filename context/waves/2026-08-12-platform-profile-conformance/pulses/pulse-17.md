# Pulse 17: Independent Held-Out Program

Status: Blocked on independent validation custodian
Implementation authority: None for hidden material, scoring, or repository selection

## Required external work

An independent custodian must:

1. bind the frozen 56-case public contract to hidden inputs and oracle rules;
2. select and freeze three public repositories without implementation-author
   influence;
3. execute the immutable Ferris binary once per case on Windows and Unix;
4. collect exactly 112 complete process records;
5. execute the three public-repository workflow under separately sealed
   changes; and
6. publish only the allowed score, validity, and limitation record.

The implementation team may not construct hidden cases, select favorable
repositories, inspect canaries or predicates, retry a first score, or
self-certify this pulse.

## Ready public material

- [Frozen 56-case contract](../../../../docs/simulations/profile-diff-held-out/PUBLIC_CONTRACT.md)
- [Custody and preflight protocol](../../../../docs/simulations/profile-diff-held-out/CUSTODY_AND_PREFLIGHT.md)
- implementation-owned platform family evidence through Pulse 16

## Blocker

No independent custodian, sealed package, three frozen repository selections,
112-record collection, or first score has been supplied. The pulse therefore
remains blocked rather than being converted into development evidence.
