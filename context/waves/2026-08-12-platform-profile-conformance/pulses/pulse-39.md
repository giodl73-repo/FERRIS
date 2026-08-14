# Pulse 39: Public checkout-verifier release

Status: Complete
Implementation authority: Public infrastructure, documentation, and test-only
validation only

## Goal

Release the smallest inspectable standard-library verifier that closes the
independently reproduced Pulse 38 checkout-orchestration ambiguity. It
qualifies only the public Pulse 25/Pulse 27 release roots at immutable cutoff
`6807bd68aa01cbf0c819198765b7d6b5aa443328`.

## Immutable boundary

Pulse 38 remains permanently `invalid`, non-retryable, and null-conclusion.
Pulse 39 is not a retry, resume, replacement diagnostic, reseed, reuse,
correlation, or inference. It creates no diagnostic authority and changes no
historical result, product behavior, score, certification, support claim,
fix authority, or PLATFORM-001 status.

## Released control

The verifier accepts an explicit checkout root and the two canonical
repository-relative Pulse 25/Pulse 27 roots. It permits exactly the fixed
36-file catalog; rejects absolute, traversal, symlink/out-of-root, missing,
unexpected, duplicate, and wrong-cardinality paths; and makes exactly 1 root-anchored check-attr invocation:

```text
git -C <checkout-root> check-attr -z --stdin text eol
```

It separately makes exactly 1 root-anchored read-only
`git -C <checkout-root> --version` probe to produce the required tool-version
output. The release therefore performs exactly 2 total Git processes: 1
check-attr invocation and 1 Git version probe. It exactly parses NUL triples,
requires `text=set` and `eol=lf` for every path, rejects `unspecified`,
malformed, missing, duplicate, or unexpected records and Git failures, then
rejects all CR bytes. There are 0 retries and no fallback check-attr form.

## Qualification and identities

A disposable Windows Git `2.55.0.windows.3` checkout with
`core.autocrlf=true`, invoked from below the checkout root, passed 36/36
attributes, 36 LF files with zero CR bytes, and the independent Pulse 29
76/76 public binding receipt.

- manifest raw / aggregate:
  `sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
  `sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`;
- root-cause report raw / payload:
  `sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd` /
  `sha256:fcfdf7c44c0f4084a6b6339d43626e67fa7b5a1e3b268c9262ae3587f9a4c5ab`;
- qualification receipt raw / payload:
  `sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8` /
  `sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546`; and
- release seal raw / payload:
  `sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c` /
  `sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b`.

## Evidence

- [Public verifier release](../../../../docs/simulations/profile-diff-held-out/pulse-39-checkout-verifier-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-39-checkout-verifier-release/qualification-receipt.json)
- [Root-cause report](../../../../docs/simulations/profile-diff-held-out/pulse-39-checkout-verifier-release/root-cause-report.md)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-39-CHECKOUT-VERIFIER-RELEASE-ROLE-REVIEW.md)
- [Rust integration test](../../../../crates/ferris-cli/tests/pulse_39_checkout_verifier_release.rs)
