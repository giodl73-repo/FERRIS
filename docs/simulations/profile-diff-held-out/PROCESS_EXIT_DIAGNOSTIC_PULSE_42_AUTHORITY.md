# Independent Pulse 42 Transactional-Copy Diagnostic Authority Contract

Status: Authorized; unexecuted
Program: `FERRIS-P42-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-TRANSACTIONAL-COPY-AUTHORITY`
Schema: `ferris.process-exit-diagnostic-pulse-42-authority/v1`
Disclosure tier: `sanitized-reproducer` (precommitted before generation)

## Purpose and immutable closure

This governance/test-only authority permits one new, independent, bounded
`process-exit-agreement` search only after its ordered public gates pass. It
does not execute FERRIS, a candidate, a diagnostic, a custody workflow, or
private data while authoring this authority.

Its immutable cutoff is
`2a8b7c27ac465ab78a8ec7ca331b9e427a8625c8`. That commit contains the
complete Pulse 41 public release and predates this authority. Custody MUST
independently verify the commit and use a fresh immutable read-only checkout
that exposes public artifacts only.

Pulse 38 remains permanently
`invalid-before-normalized-checkout-verification`, non-retryable, and
null-conclusion. Pulse 40 remains permanently `invalid` at
`pulse-39-release-custody`, non-retryable, and null-conclusion. Pulse 42 is
not a retry, resume, reseed, reuse, correlation, or inference of either
Pulse 38 or Pulse 40. All other closed predecessors, results, product
boundaries, Cargo authority, and PLATFORM-001 status remain unchanged.

## First gate: verify and directly execute the cutoff Pulse 41 adapter

Before any inherited gate, fresh custody MUST verify the exact eight-file
Pulse 41 release tree, reject missing or extra paths, and independently
recompute all LF Git-clean raw bindings. The exact canonical paths are:

```text
README.md
public-manifest.json
qualification-receipt.json
release-seal.json
root-cause-report.json
root-cause-report.md
tests/test_transactional_copy.py
transactional_copy.py
```

The Pulse 41 manifest has raw identity
`sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8`,
aggregate
`sha256:2efa8a1bb63444798f0e368029f81b33147ef313db98fb871b65936d4e2b2755`,
five manifest payload files, eight release-tree files, and 49120 payload
bytes. The report raw/payload identities are
`sha256:e16b84700318b3bedb82b283d9af2df8ae963fe63465fd2056a43839dfcfd8ee` /
`sha256:fbcfd656024e3fd6b3e24d18cf8991532fd77284611ab81f578e1985f6a5b4cc`;
the receipt raw/payload identities are
`sha256:add7af06ef6527182f1b76f1596168d76b8d8b21d4d78543b1406717351cd07c` /
`sha256:77914324290230da0be37021837c32a2feffeae72dee076155dba91b57f99d3f`;
and the seal raw/payload identities are
`sha256:8e6c49ee7f903d10659a82552d93a267b8ca40c3e379ad97e947c3f365b1c95a` /
`sha256:851a347f7cd6ab3ca38315718589caa6ace2e2e1b7803d53453ea2db7ae8efcf`.

Only after that verification, custody MUST directly execute the verified
cutoff path
`docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release/transactional_copy.py`.
It MUST set `PYTHONDONTWRITEBYTECODE=1`. It MUST pass exactly one absolute
Pulse 39 source root from that immutable cutoff and one fresh, absent,
absolute custody final root. It MUST NOT copy Pulse 41 through a different
copier first, use a relative root, retry, or invoke an alternate publication
operation.

The one deterministic invocation MUST pass source/stage/final `8/8`,
independently recompute final `8/8`, perform eight destination file fsyncs,
attempt two bottom-up staging-directory syncs with an honest `synced` or
`unsupported` posture, perform exactly one rename, and make zero retries.
Final-parent sync MUST be honestly `synced` or `unsupported`; rollback,
indeterminate publication, stage residue, and final residue MUST all be
absent. Any mismatch stops this authority with a null conclusion.

## Second gate: copied Pulse 39 verifier

Only after the direct Pulse 41 final `8/8` recomputation, custody MUST create
exactly one separate fresh `core.autocrlf=true` cutoff checkout. From below
the copied/custodied Pulse 39 final root, it MUST invoke the copied
`checkout_verifier.py` exactly once. It MUST make exactly one root-anchored
NUL-framed `git -C <checkout-root> check-attr -z --stdin text eol` call and
one Git version probe: two Git processes, zero retries, and zero fallback.
It MUST pass 36/36 expected attributes/LF files, zero CR bytes, a safe
catalog, and a separately recomputed 76/76 normalized binding proof.

The exact Pulse 39 manifest raw/aggregate identities are
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`.
The report raw/payload identities are
`sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd` /
`sha256:fcfdf7c44c0f4084a6b6339d43626e67fa7b5a1e3b268c9262ae3587f9a4c5ab`;
receipt raw/payload are
`sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8` /
`sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546`;
and seal raw/payload are
`sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c` /
`sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b`.

## Inherited order and bounds

Only after both new gates pass, custody proceeds through the unchanged Pulse
25/27 package gate, Pulse 33 freezes, exact adapter preflight, Pulse 31
`39/39` validation, normalized Pulse 35 eight-file copy and Pulse 37 Git-blob
proof, then a fresh regular private 32-byte seed, 70 descriptors, independently
derived `18/18` domains and `8/8` tuple catalogs. Only then may one search
launch, limited to 70 cases/processes per platform and 140 processes total.
Retries remain zero and the first target mismatch stops the authority.
Publication and minimization retain their inherited sanitized-reproducer
requirements.

## Declaration and stop condition

Declaration identity:
`sha256:4da4d749892a487e30467b68bf8e35e9f72655dfb3a75414ead10ff40e0868cc`.
The closed schema fixes all members, arrays, and values. The declaration has
9046 comprehensive rejection controls; the held-out fixture registry has
29611 total declared mutations. Every Pulse 41 copy/invocation/sync/rename/
rollback field, Pulse 39 verifier field, and later-gate execution field is
currently zero, false, or null. Any scope, predecessor, cutoff, binding,
copy, verifier, package, build, preflight, input, seed, materializer,
privacy, retry, or search failure stops with a null conclusion and grants no
product or PLATFORM-001 authority.
