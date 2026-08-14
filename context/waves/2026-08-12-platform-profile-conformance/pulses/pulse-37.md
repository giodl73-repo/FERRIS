# Pulse 37: Public-Artifact Checkout Normalization and Rebinding

Status: Complete
Implementation authority: Public release metadata, normalization evidence,
documentation, review, and test-only validation only

## Goal

Rebind the existing Pulse 35 public corpus-materializer manifest and release
seal to the exact LF Git-clean bytes already required by the release-root
`.gitattributes` rule. This is public-artifact checkout normalization only; it
does not change the six source/documentation/test files semantically, rerun
qualification, execute FERRIS, execute a diagnostic, or authorize a new
diagnostic.

## Historical boundary

Pulse 36 remains immutable and permanently
`invalid-before-pulse35-materialization` at
`pulse35-release-copy-verification`. Its cutoff
`48697c8da0e93b92fa633e353925ca05707bf9ed` historical identities remain:

- manifest raw `sha256:9baef3aa3030d7e8261072b26e7bd40436c362163f9138f929f0e4264fd0289b`;
- eight-file aggregate
  `sha256:585f0caf7aa4cbe821a71dcb60e5a1b7d6ad0650677b715dcbf143456612a0d7`;
- total `405414`;
- release-seal raw
  `sha256:51edf2f2df9210291705332fa8a4c3b55cb2a19a1aff22ecd882434a5ebefef2`;
  and payload
  `sha256:5b5e4383ffe5274f36f355069a5339c1684674aea342229f54f63ef247d21e52`.

The historical mismatch remains six CRLF-sealed text files versus LF cutoff
Git blobs: `README.md` `-91`, `corpus_materializer.py` `-970`,
`qualify.py` `-188`, `root-cause-report.md` `-10`,
`tests/test_materializer.py` `-203`, and `verify_materialization.py` `-636`.
The qualification and root-cause JSON envelopes matched then and remain
unchanged. Pulse 37 neither retries nor reinterprets that closed result.

## Normalized successor identities

The current Pulse 35 release is a normalized successor binding the exact
Git-clean/LF bytes:

- manifest raw `sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1`;
- eight-file aggregate
  `sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`;
- total `403316`;
- release-seal raw
  `sha256:17459123c674f2664d7d09ea03c00dcba72129bb1cf532cfe11f8cf4edeffd23`;
  and payload
  `sha256:834781867ea008dc14a54d7b811002ee1b8fa759c0b1d7f32432ea6c0d5c5375`.

The qualification receipt and root-cause report raw/payload identities and
the complete release limits are preserved. The release root remains covered
by:

```gitattributes
/docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/** text eol=lf
```

## Materialization evidence

A disposable Windows `core.autocrlf=true` alternate index staged the
resulting public release through Git's clean filter and materialized it with
`git checkout-index`, never copying the ambient working tree. The resulting
index release tree `fcc9e21f1adc5cb42c97d47cba8058ad09c77679` passed all
eight size/hash bindings. The six text files were LF with zero CR bytes; both
unchanged JSON envelopes also matched their bindings.

The machine receipt raw SHA-256 is
`sha256:9c6f61340af9d6e7bcd4d294c7916d34c16c226d0c4ccf7d28c812465658bff6`;
its sealed receipt identity is
`sha256:e312d8265c406c6330d537e24913168508cab6dd40018bcb36bbbc1e2116bfae`.

## Evidence

- [Normalization receipt](../../../../docs/simulations/profile-diff-held-out/pulse-37-checkout-normalization/README.md)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-37-PUBLIC-ARTIFACT-CHECKOUT-NORMALIZATION-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/pulse_37_public_artifact_checkout_normalization.rs)

## Decision

Pulse 37 corrects the prospective public release binding for deterministic
Git-clean checkout use only. It creates no diagnostic authority or execution,
no product change, no qualification rerun, no candidate, no score, no fix
authority, and no PLATFORM-001 status change.
