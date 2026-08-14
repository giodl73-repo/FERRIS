# Pulse 26: Independent Process-Exit Diagnostic Public-Bundle Authority

Status: Complete; invalid during mandatory preflight; no category conclusion
Implementation authority: Public governance, machine-readable declaration,
nine-role review, and test-only validation only

## Goal

Authorize one new independent process-exit diagnostic program using the exact
public Pulse 25 collector source bundle.

Pulse 22 remains permanently `invalid` and non-retryable with a null category
conclusion. Pulse 24 remains permanently `invalid-before-candidates` and
non-retryable with a null category conclusion. Pulse 26 is not a retry,
resume, reseed, rescore, reuse, replay, or continuation of either program.

## Fixed execution target

Any later custody execution is bound to immutable Ferris cutoff
`e01130a5c1fc5b8e58e13bbde03dfc39b8f1bf60`.

The only permitted collector source is the exact nine-file public directory:

`docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/bundle`

The declaration pins:

- manifest:
  `sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75`;
- source aggregate:
  `sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558`;
- test aggregate:
  `sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62`;
- bundle aggregate:
  `sha256:8311268bc5835a6d835da52531d02e450a14992ffb03cafbed7a019003f21bbc`;
- release receipt:
  `sha256:ebd68c4f86f2602ed3af0f84154cec290e878d9b00109a2eb9615e9c69ed3780`;
  and
- release seal:
  `sha256:fa7cd42ff0766f126b7400959429f2c65a32dbeac8c4caeecfca3e5a445979c0`.

## Custody and preflight boundary

The new custodian must:

- establish a new custody identity and isolated workspace;
- copy only the nine public bundle files into that workspace;
- independently recompute all nine file hashes and the source, test, and
  complete-bundle aggregates before preflight;
- run exactly two harmless synthetic atomic Windows/Ubuntu preflight pairs
  from the copied bundle, with zero retries; and
- invalidate before candidates after any copy, digest, launch, stream,
  durability, seal, reload, residue, or cardinality failure.

The program requires a new private seed and commitment, independently frozen
classifier and generator, new case and coverage manifests, and a fresh corpus.
It prohibits access, reuse, correlation, or inference involving Pulse 17
private material, Pulse 22 private material, Pulse 24 custody material, or
Pulse 19 case bytes.

## Search and result bounds

Pulse 26 preserves the complete Pulse 22/Pulse 24 public generation domains,
mandatory interactions, oracle fields, and all six target predicates.

- maximum 512 cases per platform and 1,024 search processes;
- one launch per case per platform, zero retries, and one execution;
- transactional durable records and pair seals;
- fresh-process reload before classification;
- stop after the first completed reproducing cross-platform pair;
- separate minimization bounded to 128 transformations and 256 processes;
- fresh sanitized-reproducer publication with a valid
  `ferris.post-score-diagnostic-release/v1` receipt;
- `bounded no-reproduction; no fix authority` after complete bounded
  non-reproduction; and
- exact blocker, stage, counts, further-launch prohibition, and null category
  conclusion after invalidation.

No outcome is a score, certification, product-fix authority, support claim, or
PLATFORM-001 status change.

## Evidence

- [Normative contract](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_BUNDLE.md)
- [Declaration schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-public-bundle.v1.schema.json)
- [Authorized/unexecuted declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-bundle.json)
- [Mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-bundle-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-26-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-BUNDLE-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_public_bundle.rs)
- [Exact public collector release](../../../../docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/README.md)
- [Public-safe invalid result](../../../../docs/simulations/profile-diff-held-out/pulse-26-public-result/README.md)

## Decision

The independent custodian verified all nine public files and every pinned
manifest, source, test, bundle, receipt, and seal digest. The immutable Ferris
cutoff and both platform binaries were also verified.

The fixed two-pair preflight became invalid when the second pair failed exact
cardinality during fresh-process reload. One pair completed; four harmless
synthetic processes ran; no retry occurred. No seed, classifier, generator,
corpus, candidate, minimization, reproducer, or release receipt exists.

Pulse 26 is permanently closed with a null category conclusion. The preflight
adapter/cardinality boundary requires separate infrastructure authority.

## Stop conditions

Stop rather than widen this pulse if work would:

- execute a preflight or candidate in this governance change;
- change production source, behavior, dependencies, result classes, exits, or
  stream routing;
- reopen, retry, reuse, or infer from Pulse 22 or Pulse 24;
- access prohibited private or custody material;
- change the cutoff or any public bundle binding;
- copy anything except the nine public bundle files into custody;
- weaken exact preflight count, zero retries, coverage, oracle, collection,
  search, minimization, or publication requirements; or
- change PLATFORM-001 from Draft.
