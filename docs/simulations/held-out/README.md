# Ferris Held-Out Implementation Fixtures

Status: Structurally frozen
Freeze date: 2026-08-10
Implementation authority: None

## Purpose

This directory freezes the implementation-evaluation fixture classes produced
by the specification simulation program.

They are separate from `FSIM-*` scenarios. Simulation scenarios were used to
amend the specifications. Held-out fixtures MUST NOT be used to tune an
implementation, prompt, mapping, threshold, policy, or fallback before scoring.

## Artifacts

- [Fixture manifest](MANIFEST.md)
- [Executable source and command binding](EXECUTABLE_BINDING.md)
- [Public-safe custody receipt](PUBLIC_SAFE_RECEIPT.md)
- [Held-out score cutoff 001](SCORE_CUTOFF_001.md)
- [Public-safe score receipt 001](PUBLIC_SAFE_SCORE_RECEIPT_001.md)
- [Held-out score cutoff 002](SCORE_CUTOFF_002.md)
- [Public-safe score receipt 002](PUBLIC_SAFE_SCORE_RECEIPT_002.md)
- [Held-out score cutoff 003](SCORE_CUTOFF_003.md)
- [Public-safe score receipt 003](PUBLIC_SAFE_SCORE_RECEIPT_003.md)
- [Oracle custody and anti-leak protocol](ORACLE_CUSTODY.md)

## Freeze level

The fixture classes, distinguishing dimensions, platforms, command surfaces,
failure seeds, and oracle boundaries are frozen.

Exact public repositories, revisions, commands, input archives, expected
machine records, digests, and thresholds remain intentionally unbound until
the Proposed gate selects executable fixtures. Binding those values creates a
new manifest revision and does not authorize implementation.

## Rules

1. Development, calibration, and held-out evidence remain separate.
2. An implementation team receives fixture inputs, not the sealed oracle.
3. Any fixture used for debugging is reclassified and replaced before scoring.
4. Selected-only results are compared with owner-native full reference.
5. Windows and Unix runs are both required where listed.
6. Failures, unsupported states, abstentions, partial effects, and removals
   remain scoreable outcomes.
7. No held-out result advances a specification or authorizes deployment by
   itself.
