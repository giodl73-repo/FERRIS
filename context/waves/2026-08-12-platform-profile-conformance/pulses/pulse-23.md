# Pulse 23: Collector Durability Qualification

Status: Complete; synthetic infrastructure qualification passed
Implementation authority: Public root-cause record, synthetic qualification,
test-only validation, and role review only

## Goal

Diagnose the Pulse 22 collector durability failure and qualify repaired
collector infrastructure without executing or replaying a Ferris diagnostic
candidate.

This pulse does not reopen Pulse 22, authorize a replacement search, change
Ferris production behavior, or create evidence about
`process-exit-agreement`.

## Root cause

The failed writer closed its writable temporary-file handle, reopened the
file read-only, and requested a durability synchronization operation through
that read-only handle. Windows rejected that access mode before atomic
replacement. The old path also lacked an explicit durable
containing-directory synchronization result.

## Repair boundary

The independent infrastructure custodian:

- keeps the temporary file writable through userspace flush and file sync;
- closes it only after file synchronization;
- atomically replaces the destination with a same-directory temporary file;
- synchronizes the containing directory where supported and records
  unsupported semantics explicitly;
- detects and rejects interrupted-write residue; and
- verifies retained records through a fresh read-only process.

The closed Pulse 22 workspace remained byte-for-byte unchanged.

## Synthetic qualification

- unit tests: 20 passed, 0 failed;
- synthetic Windows/Ubuntu pairs: 20 passed, 0 failed;
- command observations: 40 passed, 0 failed;
- success exits: 20;
- nonzero exits: 20;
- retained records: 20 Windows, 20 Ubuntu, and 20 pair seals;
- fresh-process reload checks: 2 passed, 0 failed;
- exact cardinality: yes; and
- interrupted-write residue: 0.

All payloads and commands were fixed harmless collector controls. No Ferris
binary, diagnostic case, private seed, candidate corpus, Pulse 17 material,
or Pulse 22 retained stream was executed, replayed, or published.

## Evidence

- [Public qualification report](../../../../docs/simulations/profile-diff-held-out/pulse-23-collector-qualification/collector-qualification-report.md)
- [Machine-readable qualification](../../../../docs/simulations/profile-diff-held-out/pulse-23-collector-qualification/collector-qualification-report.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-23-COLLECTOR-DURABILITY-QUALIFICATION-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/collector_durability_qualification.rs)

## Decision

The repaired collector is qualified only for later review. A new diagnostic
search requires a separate pulse, new independent custody, new private
commitments, a new corpus, and explicit authority. Pulse 22 remains invalid
and permanently non-retryable.

