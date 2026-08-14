# Pulse 41: Transactional Pulse 39 copy and post-copy verifier release

Status: Complete
Implementation authority: Public infrastructure, documentation, and test-only
validation only

## Goal

Publish a standard-library transactional copier and post-copy verifier for the
bounded Pulse 40 `8/8` copied, `0/8` post-copy-verified custody class. The
release copies only the explicit absolute Pulse 39 public release root to an
explicit absolute final root and does not execute FERRIS or a diagnostic.

## Immutable boundary

Pulse 40 remains permanently `invalid`, non-retryable, and null-conclusion at
`pulse-39-release-custody`. Pulse 41 is not a retry, resume, replacement
launch, custody execution, or inference. It has no private custody input and
does not create diagnostic, product, fix, score, certification, support, or
PLATFORM-001 authority.

The exact private cause is not provable. The public report states only bounded
reproductions: stale staging path after rename, duplicated or omitted release
root, wrong cwd or relative root, and verification before final synchronization.
Each can yield the same published count shape; none is claimed as the private
cause.

## Released control and qualification

The release hard-binds the eight canonical Pulse 39 paths, 31800 release-tree
bytes, all raw sizes/digests, and the separately explicit 26455-byte
five-file Pulse 39 manifest payload. It rejects missing/extra entries,
symlinks/nonregular files, traversal, overlap, existing final roots, and
unsafe parents. It exclusively creates an absolute sibling stage, verifies
stage `8/8`, records directory-sync posture as `synced` or `unsupported`,
performs exactly one rename/replace and zero retries, then reconstructs final
paths independently and verifies final `8/8`.

Post-rename verification or operational final-sync failure rolls the final
tree back to proven absence. An unprovable rollback emits
`P41-INDETERMINATE-PUBLICATION`, never success. Unsupported directory sync is
an explicit portability posture rather than a durability claim.

Every staged destination is flushed and fsynced before close. Before its one
rename, the adapter synchronizes all created staging directories bottom-up
(`tests`, then the staging root) and publicly aggregates two directory sync
attempts as `synced` or `unsupported`. A post-rename failure proves rollback
only after final-path absence and a `synced` or explicit `unsupported`
rollback-parent sync; a removal, absence, or rollback-parent-sync operational
failure is `P41-INDETERMINATE-PUBLICATION`.

The public release has eight files. Its five-file manifest payload and
release-tree counts are explicit. Qualification has 17 Python test methods, 11
failure-control methods, 20/20 isolated exact-source cycles, one rename/zero
retries per invocation, and a Rust integration validator. The observed
Windows directory-sync posture is `unsupported` with
`os.open+os.fsync-directory-v1` and
`unsupported-by-platform-or-filesystem`.

## Evidence

- [Public transactional release](../../../../docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release/README.md)
- [Bounded class report](../../../../docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release/root-cause-report.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release/qualification-receipt.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-41-TRANSACTIONAL-COPY-RELEASE-ROLE-REVIEW.md)
- [Rust integration validator](../../../../crates/ferris-cli/tests/pulse_41_transactional_copy_release.rs)

Manifest raw / aggregate:
`sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8` /
`sha256:2efa8a1bb63444798f0e368029f81b33147ef313db98fb871b65936d4e2b2755`.
Root-cause report raw / payload:
`sha256:e16b84700318b3bedb82b283d9af2df8ae963fe63465fd2056a43839dfcfd8ee` /
`sha256:fbcfd656024e3fd6b3e24d18cf8991532fd77284611ab81f578e1985f6a5b4cc`.
Qualification receipt raw / payload:
`sha256:add7af06ef6527182f1b76f1596168d76b8d8b21d4d78543b1406717351cd07c` /
`sha256:77914324290230da0be37021837c32a2feffeae72dee076155dba91b57f99d3f`.
Release seal raw / payload:
`sha256:8e6c49ee7f903d10659a82552d93a267b8ca40c3e379ad97e947c3f365b1c95a` /
`sha256:851a347f7cd6ab3ca38315718589caa6ace2e2e1b7803d53453ea2db7ae8efcf`.
