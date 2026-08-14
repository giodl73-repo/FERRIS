# Pulse 25: Qualified Collector Source Release

Status: Complete; exact public source bundle released
Implementation authority: Public infrastructure source, tests, manifest,
qualification receipt, role review, and test-only validation only

## Goal

Publish the exact Pulse 23 qualified collector source and tests so a later
independent custodian can verify and execute the infrastructure without
accessing a private workspace.

This pulse addresses the supply-chain blocker that made Pulse 24 invalid
before candidates. It does not reopen Pulse 24 or authorize a diagnostic
search.

## Released bundle

The release contains nine immutable Python files:

- seven collector, qualification, and verification source files; and
- two unit-test files.

The bundle uses Python 3.10 or newer and only the standard library. The
repository root `LICENSE` applies to the published files.

## Exact bindings

- source aggregate:
  `sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558`;
- test aggregate:
  `sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62`;
- complete bundle aggregate:
  `sha256:8311268bc5835a6d835da52531d02e450a14992ffb03cafbed7a019003f21bbc`;
- public manifest:
  `sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75`;
- release receipt:
  `sha256:ebd68c4f86f2602ed3af0f84154cec290e878d9b00109a2eb9615e9c69ed3780`;
  and
- release seal:
  `sha256:fa7cd42ff0766f126b7400959429f2c65a32dbeac8c4caeecfca3e5a445979c0`.

## Independent release validation

- qualification bindings: 3 passed, 0 failed;
- forbidden-content checks: 6 categories passed, 0 findings;
- runtime checks: 4 passed, 0 failed;
- byte-for-byte copies: 9 passed, 0 failed;
- unit tests: 20 passed, 0 failed;
- synthetic cross-platform pairs: 20 passed, 0 failed;
- harmless command observations: 40 passed, 0 failed;
- fresh-process reload verifications: 4 passed, 0 failed; and
- interrupted-write residue: 0.

No Ferris binary or diagnostic workload ran. No private seed, candidate,
corpus, stream, hidden identifier, private path, or private-file dependency
is present.

## Evidence

- [Public source bundle](../../../../docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/README.md)
- [Manifest](../../../../docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/public-manifest.json)
- [Release receipt](../../../../docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/release-receipt.json)
- [Release seal](../../../../docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/release-seal.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-25-COLLECTOR-SOURCE-RELEASE-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/collector_source_release.rs)

## Decision

The exact qualified collector is now publicly inspectable and reproducible.
A later diagnostic search still requires a separately authorized new program,
new custody, new generation material, and a preflight against this exact
bundle.

