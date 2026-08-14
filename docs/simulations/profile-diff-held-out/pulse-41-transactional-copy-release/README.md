# Pulse 41 transactional copy and post-copy verifier release

This standard-library Python release is public infrastructure only. It copies
the exact eight-file Pulse 39 public release tree from one explicit absolute
source release root to one explicit absolute final root. It neither executes
FERRIS nor a diagnostic, build, preflight, seed, corpus, candidate, custody,
or private-data workflow. It creates no diagnostic, product, fix, score,
certification, support, or PLATFORM-001 authority.

Pulse 40 remains permanently `invalid`, non-retryable, and null-conclusion at
`pulse-39-release-custody`. This release does not retry, resume, reinterpret,
or infer that result. The exact private cause is not publicly provable. The
bounded public classes are a stale staging reference after rename, a duplicated
or omitted release root, a wrong cwd or relative root, and verification before
the final synchronization point. They can each produce the published
`8/8 copied, 0/8 post-copy verified` shape.

`transactional_copy.py` hard-binds the following Pulse 39 files, sizes, and
raw SHA-256 identities: `README.md` (1786,
`sha256:9e19afae44aa5c112ddcde67fbdaf501903b5cb39ce3757e5bc6fea8554c7989`),
`checkout_verifier.py` (9685,
`sha256:783283fd127170460ce52106a7a1158054cdc2608475e53899ff45a7a6a31d12`),
`public-manifest.json` (1387,
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c`),
`qualification-receipt.json` (2057,
`sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8`),
`release-seal.json` (1901,
`sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c`),
`root-cause-report.json` (1266,
`sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd`),
`root-cause-report.md` (1727,
`sha256:9cfedd9a239bc869c35b728564267c206db981126c502121bce43a68b533b92e`),
and `tests/test_checkout_verifier.py` (11991,
`sha256:02a57858dbb65cb678b614e0a906a8bab6f9437d69efd2cbc60fac0d4b689440`).

The adapter rejects a non-absolute or traversal-bearing root, source/final
overlap, an existing final root, unsafe parent, missing or extra tree entry,
symlink, and non-regular file. It exclusively creates an absolute sibling
staging tree. Every destination file is flushed and `os.fsync`ed before its
descriptor closes; a write, flush, sync, or close failure is a stage-copy
failure that cleans staging before publication. It then verifies the complete
canonical final-relative path set and raw bytes.

Before exactly one `os.replace` publication, it syncs every created staging
directory bottom-up: `tests`, then the staging root. Its deterministic public
aggregate records directory and attempt counts plus `synced` and `unsupported`
counts. The adapter then discards staging path objects, reconstructs final
paths independently from the final-root input, and verifies the published tree
before syncing the final parent. There are zero logical retries and no
fallback publication operation.

Directory-sync posture is always explicit: `synced` records
`os.open+os.fsync-directory-v1`; `unsupported` records the same attempted
mechanism and a bounded portability/error category, not a durability claim.
An operational stage-sync failure cleans staging before publication. Any
post-rename final verification or sync failure removes the final tree and then
syncs the final parent. Rollback is proven absent only when the final path is
absent and that rollback-parent sync is `synced` or explicitly `unsupported`;
removal, parent-sync, or absence-establishment failure emits
`P41-INDETERMINATE-PUBLICATION`, never the original failure or success.

Run one invocation from any cwd:

```console
python transactional_copy.py --source-root C:\absolute\pulse39 --final-root C:\absolute\pulse39-copy
```

Output is deterministic compact JSON with no input paths: source/stage/final
counts, rename attempts, retries, final-parent and rollback-parent sync
postures, the staging aggregate, rollback flags, indeterminate-publication
flag, and a public failure code. The release tree has eight files;
`public-manifest.json` intentionally binds the five payload files (`README.md`,
adapter, both root-cause reports, and unit test), while the manifest,
qualification receipt, and seal are separately named and sealed.

Qualification runs 17 Python test methods, including 20 isolated exact-source
success cycles, 11 failure-control methods, and the Rust integration validator.
