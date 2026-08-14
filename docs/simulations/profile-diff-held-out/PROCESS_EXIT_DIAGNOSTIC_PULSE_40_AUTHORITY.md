# Independent Pulse 40 Verifier-Custody Diagnostic Authority Contract

Status: Authorized; unexecuted
Program: `FERRIS-P40-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-VERIFIER-CUSTODY-AUTHORITY`
Schema: `ferris.process-exit-diagnostic-pulse-40-authority/v1`
Disclosure tier: `sanitized-reproducer` (precommitted before generation)

## Purpose and immutable closure

This governance/test-only authority permits one new independent, bounded
`process-exit-agreement` search only after every inherited Pulse 38, Pulse 36,
and Pulse 34 gate passes. It does not create custody, copy a package, build,
freeze a binary, generate a seed/corpus/descriptor/candidate, invoke FERRIS,
or launch a search.

Pulse 38 remains permanently
`invalid-before-normalized-checkout-verification`, non-retryable, and
null-conclusion. Pulse 40 is not a Pulse 38 retry, resume, reseed, reuse,
continuation, correlation, or inference; it preserves every prior conclusion
and every permanently invalid predecessor. It changes no product, score,
certification, support, fix, or PLATFORM-001 status.

## Cutoff and Pulse 39 custody gate

Custody MUST use only immutable cutoff
`65d1eec688f53bf7263ecfc8094ac849f9d3be4c`. It contains the complete Pulse
39 public release and predates this authority. A different or self-containing
cutoff is invalid before custody.

Before any Pulse 25/Pulse 27 package copy, custody MUST copy exactly the
eight-file Pulse 39 verifier release tree, reject every missing or extra file,
independently recompute every raw file binding, and require LF Git-clean
bytes. Its canonical release-relative paths are:

```text
README.md
checkout_verifier.py
public-manifest.json
qualification-receipt.json
release-seal.json
root-cause-report.json
root-cause-report.md
tests/test_checkout_verifier.py
```

The manifest is one of those eight files and binds exactly five payload files.
Its independently verified raw/aggregate values are
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`,
five payload files, and 26455 bytes. Custody MUST separately verify the
manifest raw binding; root-cause report raw/payload
`sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd` /
`sha256:fcfdf7c44c0f4084a6b6339d43626e67fa7b5a1e3b268c9262ae3587f9a4c5ab`;
qualification receipt raw/payload
`sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8` /
`sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546`; and
release seal raw/payload
`sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c` /
`sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b`.

Only then, from a below-root cwd against exactly one fresh
`core.autocrlf=true` cutoff checkout, custody MUST invoke the copied verifier.
It makes exactly one root-anchored NUL-framed
`git -C <checkout-root> check-attr -z --stdin text eol` invocation and one
root-anchored Git version probe: two Git processes total, zero retries, and no
fallback. Its deterministic public report MUST pass 36 expected/attribute/LF
files with zero CR bytes and a safe relative catalog. Custody MUST separately
retain the existing Pulse 29 normalized binding verification at 76/76.

## Inherited gates and later order

The Pulse 39 gate replaces only the ambiguous normalized-checkout operation.
Every Pulse 38/Pulse 36/Pulse 34 package, build-freeze, preflight, Pulse 31,
normalized Pulse 35, Pulse 37 Git-blob proof, privacy, minimization, and
publication bound remains exact. In particular, the normalized Pulse 35
manifest/aggregate remain
`sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1` /
`sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`,
with eight files and 403316 bytes.

After the verifier gate, custody proceeds only through Pulse 25/Pulse 27
package custody, Pulse 33 freezes, exact adapter preflight, Pulse 31
validation, the normalized Pulse 35 copy and Pulse 37 proof, then one new
regular undisclosed 32-byte CSPRNG seed, 70 descriptors, and fresh private
verification. Only then may one transactional cross-platform search run:
at most 70 cases and processes per platform and 140 processes total, one
launch, zero retries, and stop at the first target mismatch. Inherited
sanitized-reproducer publication and minimization rules remain binding.

## Declaration and stop conditions

Declaration identity:
`sha256:9ff14e5083ed4222f23e0ba68d945515225911633435b73c6c2fe4e6d9680a52`.
The closed Draft 2020-12 schema and exact declaration have 9076 mutation
controls (20565 declared controls in total). Every current activity/result
field is zero, false, or null. Stop with a null conclusion on any cutoff,
release, LF, verifier, package, build, preflight, input, seed, materializer,
search, privacy, retry, predecessor-reopening, or scope failure.
