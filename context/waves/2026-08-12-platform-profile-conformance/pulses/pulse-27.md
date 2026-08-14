# Pulse 27: Exact-Two-Pair Preflight Adapter Release

Status: Complete; exact public adapter and immutable collector copy released
Implementation authority: Public infrastructure source, tests, evidence,
nine-role review, and test-only validation only

## Goal

Publish the already-qualified exact-two-pair public preflight adapter that
closes the Pulse 26 orchestration boundary without executing a Ferris
diagnostic candidate or modifying production behavior.

Pulses 22, 24, and 26 remain permanently invalid, non-retryable, and unable
to produce category conclusions. This pulse does not reopen, retry, resume,
reseed, rescore, replay, reuse, or continue any of them.

## Root cause

The Pulse 26 orchestration incorrectly supplied pair-local expected
cardinality `1` to a whole-store verifier after pair two already existed in
the growing store. The verifier correctly rejected the extra row.

The immutable Pulse 25 collector required no modification. Pulse 27 adds a
separate adapter that writes exactly two Windows records, two Ubuntu records,
and two joined pair seals, then verifies the complete six-file store with
whole-store expected cardinality `2` from fresh Windows and Ubuntu Python
processes.

## Exact release bindings

The public package is
`docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/`.
It pins:

- 20-file release aggregate:
  `sha256:31f38a79629d6b5da1fab9cb335450a95a1763f1ac80b1d8d851b103a318e540`;
- public manifest:
  `sha256:449851e7b917f474fb1829b2d9f89a3f08a886733c476889dfad1ae27d097154`;
- root-cause report:
  `sha256:9bd5ac7aa29b1f621df09eefc2ff33369c5ba5810e4e5f76f4e7e15aab57f478`;
- qualification receipt:
  `sha256:94ec4237fd046281b9971e7eea67dc1ae7208996ed69b50ce8430a78cd0b6886`;
  and
- release seal:
  `sha256:6bff93434170ee79a4b5210ee26b647ab6dba351dccc4ccf3a5e224de56ced38`.

The manifest also records the unchanged nine-file collector copy, five
adapter source files, one adapter test file, every per-file SHA-256 digest,
and deterministic adapter-source, adapter-test, collector-copy, and complete
release aggregates.

## Qualification

The fixed harmless synthetic qualification passed:

- 50 of 50 cycles;
- 200 process rows;
- 100 pair seals;
- 100 fresh-process reloads;
- zero retries; and
- zero residue.

The copied immutable collector suites passed 10 tests per platform and the
adapter suite passed 9 tests per platform. The public reproducer demonstrates
only the generic cardinality-scope error and contains no diagnostic input,
seed, corpus, candidate, or retained diagnostic stream.

## Evidence

- [Public adapter release](../../../../docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/README.md)
- [Public manifest](../../../../docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/public-manifest.json)
- [Root-cause report](../../../../docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/root-cause-report.json)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/qualification-receipt.json)
- [Release seal](../../../../docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/release-seal.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-27-PREFLIGHT-ADAPTER-RELEASE-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/preflight_adapter_release.rs)

## Decision

The exact-two-pair adapter is public, inspectable, reproducible, and qualified
as infrastructure only. No diagnostic preflight or candidate was executed by
this governance pulse, and no fix, score, certification, support, production,
or PLATFORM-001 status authority follows.

Pulse 28 is intentionally deferred. Its immutable execution cutoff must be a
later commit that already contains the complete Pulse 27 public release and
authority. An uncommitted Pulse 27 change cannot name that cutoff, and a Pulse
28 authority commit must not use a cutoff containing its own authority.

## Stop conditions

Stop rather than widen this pulse if work would:

- execute a diagnostic preflight, corpus, candidate, or minimization;
- modify the immutable Pulse 25 collector or any production source;
- reopen or infer a conclusion from Pulse 22, Pulse 24, or Pulse 26;
- access private custody, seed, corpus, stream, oracle, or held-out material;
- authorize Pulse 28 before a committed Pulse 27 cutoff exists;
- use a cutoff containing its own diagnostic authority; or
- change PLATFORM-001 from Draft.
