# Pulse 35: Public Corpus-Materializer Release

Status: Complete; public synthetic infrastructure release
Implementation authority: Public release source, machine contract,
documentation, review, and test-only validation only

## Goal

Prospectively close the Pulse 34 generation-materialization gap without
reopening Pulse 34, executing a FERRIS candidate, changing product code,
accessing private custody material, or authorizing a new diagnostic.

The release is
[`pulse-35-corpus-materializer-release/`](../../../../docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/README.md).
It is a standalone Python standard-library adapter. It reads a supplied
regular private seed file containing exactly 32 bytes of CSPRNG material,
publishes only the explicit domain-separated seed commitment
`sha256("ferris-p35-seed-commitment-v1\0" || seed)`, and deterministically
materializes 70 concrete public-rule descriptors. Case IDs, order tokens, and
profile tokens are full HMAC-SHA256 pseudorandom outputs keyed by that
mandatory seed over a domain/purpose/counter message; profile IDs therefore
contain seed-derived pseudorandom values, never raw seed bytes or source
paths.

## Public-only boundary

The adapter binds only:

- the exact Pulse 31 public contract, schema, six positive fixtures, and 33
  mutation controls;
- the Pulse 34 public authority's 18 coverage domains, eight interactions,
  and 512-logical-case maximum; and
- the immutable Pulse 34 public result receipt
  `sha256:dca0ad1579257a6f265ada501533a4034070963267ef7c25478bf38267ee1588`.

Each descriptor declares concrete role state, output-relative regular/
directory/missing target, raw size and digest where applicable, expected input
classification, expected result posture, a host-independent request-template/
namespace/substitution contract, and semantic witnesses. It includes valid
pairs plus actual malformed, duplicate, unsupported, boundary, oversized,
directory, missing, path, ordering, pointer, value, number, and change-count
witnesses. `blocked` is separately represented as a no-launch external-
prerequisite descriptor. The independent verifier privately requires the seed,
recomputes every HMAC value, all raw witnesses, UNC-preserving lexical
resolution, and all exact tuple catalogs: 20 metadata-boundary, 12
metadata-character, 54 input/path, 6 input-size, 33 value/order, 20
duplicate/failure, 6 result/route, and 4 result/format tuples. Thus `18/18`
domains and `8/8` interactions are derived closures, not labels.

Atomic same-directory staging, file sync, one replacement with zero logical
retries, and confirmed rollback after final-sync failure protect the output
boundary. Every directory-sync event records `synced` or `unsupported` with
mechanism/error; unexpected failures propagate, and post-creation staging-sync
failure cleans its residue. Existing, partial, extra, replayed, or residual
output is rejected.

Pulse 34 remains immutable, `invalid` at `generation-materialization`,
non-retryable, null-conclusion, and closed. The only recorded root-cause fact
is that its frozen generator did not complete isolated materialization before
candidate launch. This release infers no private cause or detail.

## Qualification and identities

Synthetic qualification runs 20 isolated complete-coverage cycles, private
seed fresh-process reloads, same-seed identity, different-seed divergence,
exact 31/32/33-byte seed controls, semantic fake-coverage rejection, replay,
extra-output, residue, rename-failure, staging-sync cleanup, and final-sync
rollback controls. It records actual directory-sync posture, zero logical
retries, zero residue, no diagnostic
execution, no product-file modification, no private-path disclosure, and no
seed-byte disclosure.

- manifest raw SHA-256:
  `sha256:9baef3aa3030d7e8261072b26e7bd40436c362163f9138f929f0e4264fd0289b`;
- eight-file aggregate:
  `sha256:585f0caf7aa4cbe821a71dcb60e5a1b7d6ad0650677b715dcbf143456612a0d7`;
- qualification receipt raw SHA-256:
  `sha256:4c4f4ad1d9fa437e23f655083eb74c754114c5bea43ae111d2127fc7f051a037`;
- qualification payload SHA-256:
  `sha256:7f1154ca94009cef966ab2f43ba74a9f017989ed5dbbdbfd8c3ce8fe64fe5cee`;
- root-cause report raw SHA-256:
  `sha256:02f3a34195858b1f82acd4b9c2ea9abc42413306e40caea3b9594ed0492b6ffe`;
- root-cause payload SHA-256:
  `sha256:26d1a9a9051f5c4656da62f3743df19c371297634dbfdaf898ae76ed37b623ce`;
- release-seal raw SHA-256:
  `sha256:51edf2f2df9210291705332fa8a4c3b55cb2a19a1aff22ecd882434a5ebefef2`;
  and
- release-seal payload SHA-256:
  `sha256:5b5e4383ffe5274f36f355069a5339c1684674aea342229f54f63ef247d21e52`.

## Evidence

- [Public release](../../../../docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/README.md)
- [Public manifest](../../../../docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/public-manifest.json)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/qualification-receipt.json)
- [Release seal](../../../../docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/release-seal.json)
- [Closed machine schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.pulse-35-corpus-materializer.v1.schema.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-35-CORPUS-MATERIALIZER-RELEASE-ROLE-REVIEW.md)
- [Rust release validator](../../../../crates/ferris-cli/tests/pulse_35_corpus_materializer_release.rs)

## Stop conditions

Stop rather than execute a FERRIS binary, candidate, diagnostic, owner
command, or build; change `crates/ferris-cli/src`; access a private custody
directory, hidden corpus, seed, or prior material; disclose a seed or private
path; modify a prior result or declaration; treat coverage labels as score
evidence; weaken the 512-case, public-input, duplicate/framing/key/size,
atomicity, no-extra-output, zero-retry, or privacy limits; or change
PLATFORM-001 status.
