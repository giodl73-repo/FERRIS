# Pulse 40: Independent verifier-custody diagnostic authority

Status: Authorized; unexecuted
Implementation authority: Governance, closed fixtures, documentation, and
test-only validation only

## Goal

Authorize one new independent normalized public diagnostic authority only at
immutable cutoff `65d1eec688f53bf7263ecfc8094ac849f9d3be4c`. The cutoff contains
the complete Pulse 39 release and predates this authority.

## Immutable boundary

Pulse 38 remains permanently
`invalid-before-normalized-checkout-verification`, non-retryable, and
null-conclusion. Pulse 40 is not a Pulse 38 retry, resume, reseed, reuse,
continuation, correlation, or inference. All prior conclusions remain
unchanged; this change creates no custody, package copy, build, preflight,
FERRIS execution, seed/corpus/candidate, private-data access, or result.

## Ordered authority

Before any inherited package copy, new custody MUST exactly copy the complete
eight-file LF Git-clean Pulse 39 release tree, reject every missing or extra
path, and independently recompute all eight raw file bindings. Its canonical
release-relative paths are `README.md`, `checkout_verifier.py`,
`public-manifest.json`, `qualification-receipt.json`, `release-seal.json`,
`root-cause-report.json`, `root-cause-report.md`, and
`tests/test_checkout_verifier.py`. The manifest is one tree file and binds
exactly the five payload files; its raw/aggregate values are
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`,
26455 bytes. Custody MUST separately verify the manifest raw binding, report
raw/payload, receipt raw/payload, and seal raw/payload:
`sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd` /
`sha256:fcfdf7c44c0f4084a6b6339d43626e67fa7b5a1e3b268c9262ae3587f9a4c5ab`;
receipt raw/payload
`sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8` /
`sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546`; and
seal raw/payload
`sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c` /
`sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b`.

Custody then invokes that release from below the root against exactly one
fresh `core.autocrlf=true` cutoff checkout. The verifier makes exactly one
root-anchored `git -C <checkout-root> check-attr -z --stdin text eol` call and
one Git version probe: two Git processes, zero retries, no fallback. It must
emit its deterministic pass report for 36 expected/attribute/LF, zero-CR,
safe-relative files and independently retain Pulse 29's 76/76 normalized
binding verification. Only then do all inherited Pulse 38/Pulse 36/Pulse 34
package, freeze, preflight, validation, normalized-copy, fresh-seed,
materialization, search, minimization, and publication gates proceed.

The sole later search remains one launch, <=70 cases/processes per platform,
<=140 processes total, exactly one fresh private 32-byte CSPRNG seed, 70
descriptors, fresh verification, zero retries, and stop-first mismatch.

## Evidence

- [Normative authority](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_40_AUTHORITY.md)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-40-authority.v1.schema.json)
- [Exact declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-40-authority.json)
- [Mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-40-authority-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-40-VERIFIER-CUSTODY-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md)
- [Integration validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_40_authority.rs)

Declaration identity:
`sha256:9ff14e5083ed4222f23e0ba68d945515225911633435b73c6c2fe4e6d9680a52`.
There are 9076 mutation controls. Failure at any gate stops with a null
conclusion and grants no product or PLATFORM-001 authority.
