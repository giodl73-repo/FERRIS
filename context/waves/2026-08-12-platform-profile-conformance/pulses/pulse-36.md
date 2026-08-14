# Pulse 36: Independent Materialized Public Diagnostic Result

Status: Complete: permanently `invalid-before-pulse35-materialization`,
non-retryable, and null-conclusion
Implementation authority: None

## Authority and passed inherited gates

The independent program used authority
`2bf480459614dc56ee2bd744302e79f20a571092` and immutable cutoff
`48697c8da0e93b92fa633e353925ca05707bf9ed`. The cutoff predates the
authority and contains the complete Pulse 35 release.
The closed authority declaration identity remains
`sha256:f4d83498f780e6d35bd0073f8d8ddeaa67d99fb2426978190f7af25fff746952`;
its 1998 mutation controls remain historical authority evidence.

Custody passed every inherited public gate: normalized checkout `36/36` LF
with zero CR bytes, Pulse 25/27 bindings `76/76`, the Pulse 33 37-file
release, two platforms/binaries/receipts, one-invocation/two-pair/four-row/
two-seal/two-verifier `2/2/2` preflight with zero retries and residue, and
Pulse 31's nine artifacts and `39/39` self-validation.

## Stop and public root cause

The sole eight-file Pulse 35 release copy stopped
`invalid-before-pulse35-materialization` at
`pulse35-release-copy-verification`: two file bindings matched and six
mismatched. The copy expected 405,414 bytes and observed 403,316. No seed,
materializer, descriptors, candidates, pairs, or seals were created.

Independent reproduction from cutoff Git blobs shows release checkout/binding
infrastructure—not product—evidence: the six text files were sealed using
CRLF working-tree bytes while the cutoff's `.gitattributes` stores LF Git
blobs. Cutoff minus sealed-byte deltas are `README.md` `-91`,
`corpus_materializer.py` `-970`, `qualify.py` `-188`,
`root-cause-report.md` `-10`, `tests/test_materializer.py` `-203`, and
`verify_materialization.py` `-636`. The JSON qualification and root-cause
envelopes match exactly.

## Evidence

- [Normative authority](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_36_AUTHORITY.md)
- [Custodian public result](../../../../docs/simulations/profile-diff-held-out/pulse-36-public-result/README.md)
- [Machine result](../../../../docs/simulations/profile-diff-held-out/pulse-36-public-result/PULSE-36-PUBLIC-RESULT.json)
- [Authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_36_authority.rs)
- [Result validator](../../../../crates/ferris-cli/tests/pulse_36_public_result.rs)

Result raw SHA-256:
`sha256:735353e311dc63cd0cdef85c112bd60fd2c50c18f29858929a58f886b34009cc`

Receipt ID:
`sha256:d1f6f648ae8bb9a1fc44def2d392b72b76446b49439ff8f31e4124ad1fafc628`

## Decision

The category conclusion is null, and further launches are prohibited. Pulse
36 is permanently invalid and must not be retried, resumed, reseeded,
rescored, reused, continued, correlated, or used to infer product behavior.
It grants no product, score, certification, fix, support, or PLATFORM-001
authority.
