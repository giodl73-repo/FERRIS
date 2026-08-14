# Profile Diff Held-Out Program

Status: Closed; Stage B/C completed with a valid implementation failure
Implementation authority: None
Oracle access: Prohibited

## Purpose

This program defines an independently constructed held-out evaluation for the
bounded Pulse 14 `profile-diff` command. It is separate from the historical
FHIF implementation-fixture series and from the Pulse 15 development fixture
matrix.

The public repository contains:

- the [public scoring contract](PUBLIC_CONTRACT.md);
- the complete public
  [`ferris.profile-evidence/v0` input contract](INPUT_PROFILE_EVIDENCE.md);
- the [custody and preflight protocol](CUSTODY_AND_PREFLIGHT.md);
- the [exact identity contract](IDENTITY.md);
- the [three-public-repository workflow](THREE_REPOSITORY_WORKFLOW.md);
- the [public repository-selection binding](REPOSITORY_SELECTION_BINDING.md);
- the [immutable public-safe Stage B/C result](PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
  and its
  [machine-readable companion](PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.json);
- complete [Draft 2020-12 schemas](schemas/README.md); and
- public [synthetic vectors and preflight fixtures](fixtures/README.md).

Prospective programs may also opt into the
[post-score diagnostic release protocol](POST_SCORE_DIAGNOSTIC_RELEASE.md).
[Why quarantine exists](WHY_QUARANTINE.md) explains the certification and
debugging tradeoff. Neither document changes this closed program.

The separate
[Pulse 22 process-exit diagnostic replication contract](PROCESS_EXIT_DIAGNOSTIC_REPLICATION.md)
precommitted the `sanitized-reproducer` tier for one fresh independent
public-rule-based diagnostic search. Its
[public-safe result](pulse-22-public-result/README.md) is `invalid`: one
Windows process was retained, the required Ubuntu partner was not launched
after a collector durability failure, and no category conclusion exists. It
does not reopen, retry, rescore, reuse, infer, or otherwise alter this closed
program.

The separate
[Pulse 23 collector qualification](pulse-23-collector-qualification/collector-qualification-report.md)
records the durability root cause and 20 passing synthetic Windows/Ubuntu
pairs for the repaired collector. It executed no Ferris diagnostic candidate
and does not authorize a replacement search.

The separate
[Pulse 24 replacement diagnostic contract](PROCESS_EXIT_DIAGNOSTIC_REPLACEMENT.md)
opened one new independent search at immutable cutoff
`cef0daabc349ac2333869959f21b9a3106e10484`. Its
[public result](pulse-24-public-result/README.md) is
`invalid-before-candidates`: the public collector report verified, but no
exact inspectable source copy was available. It ran zero preflight or
candidate processes and produced no category conclusion.

The
[Pulse 25 collector source release](pulse-25-collector-source-release/README.md)
publishes the exact qualified nine-file infrastructure bundle, manifest,
receipt, and seal. Every file and aggregate digest is repository-tested. The
release contains no diagnostic data and grants no search authority.

The
[Pulse 26 public-bundle diagnostic contract](PROCESS_EXIT_DIAGNOSTIC_PUBLIC_BUNDLE.md)
opened one new independent program at immutable cutoff
`e01130a5c1fc5b8e58e13bbde03dfc39b8f1bf60`. Its
[public result](pulse-26-public-result/README.md) is `invalid`: all bundle and
cutoff bindings verified, but the second required synthetic pair failed
exact-cardinality fresh-process reload. No corpus or candidate was created,
and no category conclusion exists.

The
[Pulse 27 exact-two-pair adapter release](pulse-27-preflight-adapter-release/README.md)
publishes a separate adapter around the byte-for-byte immutable Pulse 25
collector. The root cause was pair-local expected cardinality `1` supplied to
a whole-store verifier after pair two existed; the collector required no
modification. Qualification passed 50 of 50 cycles, 200 process rows, 100 pair
seals, 100 fresh-process reloads, zero retries, and zero residue. This public
infrastructure release executes no diagnostic candidate and grants no search
authority. Pulses 22, 24, and 26 remain permanently invalid and non-retryable
with null category conclusions.

The separate
[Pulse 28 public-adapter diagnostic contract](PROCESS_EXIT_DIAGNOSTIC_PUBLIC_ADAPTER.md)
authorized one new independent program at immutable cutoff
`2935f44475b811e619f2ef62e0d408f39c7e8149`. It pins every Pulse 25
collector binding and the exact Pulse 27 20-file manifest. New custody must
copy exactly those public files, recompute every file and aggregate, run one
two-pair adapter invocation, then two fresh platform verifiers enforcing
whole-store cardinality `2/2/2`, with zero retries and zero residue. The
[public result](pulse-28-public-result/README.md) closed
`invalid-before-candidates`: 60 binding checks produced 10 passes and 50
failures. The Pulse 25 manifest was expected at
`sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75`
and observed at
`sha256:03322e9fe6a3df6c71161e5f3916c51cc66c9453e9f1f3141bcc703bd02d7a0d`.
Git worktree EOL conversion at checkout, not corrupted Git blobs, caused the
mismatch. No package copy, build, preflight, generation, candidate, pair,
seal, retry, or category conclusion exists.

[Pulse 29 checkout normalization](pulse-29-checkout-normalization/README.md)
adds anchored recursive LF rules for every file under the Pulse 25 and Pulse
27 release roots and rebinds their public manifests, raw files, aggregates,
receipts, and seals to stable LF bytes. A disposable resulting-index checkout
on Windows with `core.autocrlf=true` verified 36 LF files and 76 of 76 exact
binding checks. Pulse 29 creates no diagnostic authority and does not modify
the Pulse 28 result or PLATFORM-001 status.

The separate
[Pulse 30 normalized public-adapter contract](PROCESS_EXIT_DIAGNOSTIC_NORMALIZED_PUBLIC_ADAPTER.md)
authorized one final new independent program at immutable cutoff
`cf6b3309c31e5da37d4a8e6655a781f4e92ef603`. Before package copy, custody
materialized that cutoff with `core.autocrlf=true`, verified 36/36 attribute
and LF checks plus 76/76 normalized bindings, copied 20 files, recomputed 20
hashes and four aggregates, verified six report/receipt/seal bindings, and
froze both binaries and both environments. The fixed one-invocation,
two-pair, four-row, two-seal, two-verifier `2/2/2` preflight passed with zero
retries and zero residue.

The [Pulse 30 public result](pulse-30-public-result/README.md) then closed
`invalid` at `generation-before-case-materialization` because the authorized
public read scope did not contain a public `ferris.profile-evidence/v0` input
schema. The exact raw result digest is
`sha256:f75d33f054002cdd1b066678163ef926f62ec95ba826fef7273bc614c348f090`
and its receipt ID is
`sha256:8f08b0cf27f1b1bb97bcea0591b92c2143cf324736e2112744122838ca58dc30`.
There were zero candidates or candidate processes and the category conclusion
is null. Further launches are prohibited.

[Pulse 31](INPUT_PROFILE_EVIDENCE.md) prospectively publishes the missing
complete public input boundary: one recursive Draft 2020-12 schema, six
positive fixtures, 33 negative controls, byte/framing/duplicate companion
rules, exact classifications, a test-only validator, and nine-role review.
It changes no production behavior and does not reopen Pulse 30.

The separate
[Pulse 32 public-input diagnostic contract](PROCESS_EXIT_DIAGNOSTIC_PUBLIC_INPUT.md)
authorizes one new independent program at immutable cutoff
`29517d732db13cc2ffa304684b344f3538ab587d`. It inherits the complete Pulse
30 normalization, package, adapter preflight, freshness, coverage, oracle,
search, collection, minimization, and publication rules. It additionally
pins the Pulse 31 contract and schema raw digests, all six positive fixture
path/size/digest bindings, the mutation-file digest, and all 33 mutation IDs
and canonical public digests. After adapter preflight and before generation,
custody must verify those nine public artifacts and complete 39/39 public-only
contract classifications without reading Ferris source or tests. The
authority was governance/test-only. Its
[public result](pulse-32-public-result/README.md) passed 36/36 attribute and LF
checks, 76/76 bindings, and exact package verification, then closed `invalid`
at `cutoff-build-freeze` because the required Ubuntu executable was
unavailable. There were zero preflight operations, input classifications,
generated cases, or candidate processes, and the category conclusion is
null.

[Pulse 33](pulse-33-build-freeze-release/README.md) publishes the external
build-freeze adapter, root-cause report, receipts, 37-file manifest, and
release seal without executing a diagnostic or changing product code. The
Pulse 32 blocker was exit 127, `cargo: command not found`, because a WSL
non-login shell omitted the ordinary rustup Cargo directory from `PATH`.
Explicit Cargo succeeds at the exact cutoff. The adapter uses that explicit
discovery and Cargo `compiler-artifact` JSON. Qualification passed 14 unit
tests, 20 synthetic checks, and four clean deterministic rebuilds.

The separate
[Pulse 34 public-authority diagnostic contract](PROCESS_EXIT_DIAGNOSTIC_PUBLIC_AUTHORITY.md)
authorizes one new independent program at immutable cutoff
`5df7492fa759c415f6ce540a33a4e89c46714348`. It inherits every Pulse 32
public gate and additionally pins the exact Pulse 33 37-file manifest,
aggregate, seal, build adapter, and public receipts. Before inherited adapter
preflight, custody must use explicit WSL non-login Cargo discovery and Cargo
`compiler-artifact` JSON to freeze exact Windows and Ubuntu binaries from the
Pulse 34 cutoff. The declaration identity is
`sha256:8975e07b9dd417604d06be12a24a448e8ae1834991aca9db086ae7c11b0b1e34`;
704 mutations reject gate weakening. Its
[public result](pulse-34-public-result/README.md) passed checkout `36/36`,
bindings `76/76`, every public package and build gate, exact `2/2/2`
preflight, and public-input self-validation `39/39`, then closed `invalid` at
`generation-materialization` because the frozen generator did not complete an
isolated corpus before candidate launch. There were zero valid cases,
candidates, search processes, or reproducers; the category conclusion is
null. Further launches are prohibited.

[Pulse 35](pulse-35-corpus-materializer-release/README.md) prospectively
releases a standalone public-rule corpus materializer without reopening Pulse
34. It requires an exact 32-byte CSPRNG seed for both materialization and
verification, publishes only a domain-separated commitment, and derives its
public case IDs/order/profile tokens with keyed HMAC-SHA256—not raw seed
slices. Its exactly 70 descriptors bind every request spelling through a
platform namespace/template/substitution contract to the declared target,
including UNC-preserving lexical resolution. Exact tuple catalogs close all
20 metadata-boundary, 12 metadata-character, 54 input/path, 6 input-size, 33
value/order, 20 duplicate/failure, 6 result/route, and 4 result/format
requirements; the independently derived `18/18` domains and `8/8`
interactions are not labels. Pair change counts and `9,999`/`10,000`/`10,001`
boundary witnesses are explicit and recomputed. Qualification runs 20
isolated cycles with private-seed fresh reload, same/different seed,
seed-length, semantic-tamper, replay, extra-output, residue, one-attempt
publication, staging-sync cleanup, and final-sync rollback/re-entry controls
at zero logical retries. Directory-sync receipts state `synced` or
`unsupported` with mechanism/error. The release changes no product code,
accesses no private custody material, and authorizes no new diagnostic.

The separate [Pulse 36 materialized public authority](PROCESS_EXIT_DIAGNOSTIC_PULSE_36_AUTHORITY.md)
executed independently under authority
`2bf480459614dc56ee2bd744302e79f20a571092` at cutoff
`48697c8da0e93b92fa633e353925ca05707bf9ed`. Its
[public result](pulse-36-public-result/README.md) passed inherited checkout
`36/36` LF, bindings `76/76`, the Pulse 33 37-file/two-platform/two-binary/
two-receipt gate, exact `1/2/4/2/2` preflight with zero retries/residue, and
Pulse 31's nine artifacts/`39/39`. It stopped permanently
`invalid-before-pulse35-materialization` at
`pulse35-release-copy-verification`: 2/8 Pulse 35 files matched, 6/8
mismatched, and 405,414 expected bytes became 403,316 observed bytes. No
seed, materializer, descriptors, candidates, pairs, or seals exist; the
category conclusion is null and further launches are prohibited.

Independent cutoff-blob reproduction establishes public-safe checkout/binding
infrastructure evidence, not product evidence. The six text files were sealed
with CRLF working-tree bytes while `.gitattributes` stores LF Git blobs:
`README.md` `-91`, `corpus_materializer.py` `-970`, `qualify.py` `-188`,
`root-cause-report.md` `-10`, `tests/test_materializer.py` `-203`, and
`verify_materialization.py` `-636`; JSON qualification and root-cause
envelopes match. The result raw digest is
`sha256:735353e311dc63cd0cdef85c112bd60fd2c50c18f29858929a58f886b34009cc`
and receipt ID is
`sha256:d1f6f648ae8bb9a1fc44def2d392b72b76446b49439ff8f31e4124ad1fafc628`.
Pulse 36 is permanently invalid and non-retryable.

[Pulse 37](pulse-37-checkout-normalization/README.md) records public-artifact
checkout normalization and rebinding only. It preserves Pulse 36's historical
CRLF-derived Pulse 35 identities and permanently invalid result, while the
current normalized successor binds the LF manifest
`sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1`,
aggregate
`sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`,
and total `403316`. A disposable Windows `core.autocrlf=true`
resulting-index clean-filter checkout passed 8/8 file size/hash bindings and
zero CR bytes in the six text files. It runs no FERRIS or diagnostic, creates
no diagnostic authority, reruns no qualification, and changes no product
code.

The separate [Pulse 38 normalized public authority](PROCESS_EXIT_DIAGNOSTIC_PULSE_38_AUTHORITY.md)
was governance/test-only at immutable cutoff
`6807bd68aa01cbf0c819198765b7d6b5aa443328`, which contains Pulse 37. It
preserves Pulse 36 and all prior invalid programs as permanently invalid,
non-retryable, and null-conclusion; it is not a retry, resume, reseed, reuse,
correlation, or inference. It binds normalized manifest
`sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1`,
aggregate `sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`,
403316 bytes, seal, and Pulse 37 receipt before new private
seed/materialization/fresh verification. One inherited transactional search
is capped at 70 cases/processes per platform and 140 processes total. Its
identity is
`sha256:a3317422e8c34d4e08d7c5e577e3539820f1376d7fba2ef38d262d1f967031b4`;
its sole execution bound and materialized the cutoff once, then stopped
`invalid-before-normalized-checkout-verification` because the first required
attribute check did not complete. No package, build, preflight, input, seed,
corpus, candidate, process, search, or minimization activity occurred. The
[public result](pulse-38-public-result/README.md) is permanently non-retryable
and has raw digest
`sha256:d3e74d220a9de9da4f2fff72812443de42272c9a8f78b0efad37573ab33b1c9c`
and receipt
`sha256:56ddacc0e3043b327b8ce2d6ce869e9662a564faee9ce4f9a2c3d783a390bdad`.
Its category conclusion is null.

[Pulse 39 checkout-verifier release](pulse-39-checkout-verifier-release/README.md)
is public infrastructure only and does not retry or alter the permanently
invalid Pulse 38 result. It fixes the reproduced below-root cwd ambiguity
with exactly 1 root-anchored NUL-framed `git -C <checkout-root> check-attr -z
--stdin text eol` invocation and exactly 1 separate root-anchored read-only Git
version probe: 2 total Git processes, 0 retries, and no fallback check-attr form.
The disposable
Windows Git `2.55.0.windows.3` `core.autocrlf=true` cutoff checkout passes
36/36 attributes, 36 LF files with zero CR bytes, and independently binds the
existing Pulse 29 76/76 receipt. Its manifest raw/aggregate are
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`;
its receipt raw/payload are
`sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8` /
`sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546`;
and its seal raw/payload are
`sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c` /
`sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b`.
It executes no FERRIS, diagnostic, build, preflight, seed/corpus
materialization, or private custody data, and grants no new authority.

The separate [Pulse 40 verifier-custody authority](PROCESS_EXIT_DIAGNOSTIC_PULSE_40_AUTHORITY.md)
is governance/test-only and unexecuted at immutable cutoff
`65d1eec688f53bf7263ecfc8094ac849f9d3be4c`. It preserves Pulse 38 as
permanently invalid, non-retryable, and null-conclusion; it is not a retry,
resume, reseed, reuse, correlation, or inference. Before package copy, custody
must exactly copy and independently recompute the complete eight-file LF Git-clean Pulse 39 release tree, reject missing or
extra paths, and recompute all raw bytes: manifest raw/aggregate
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`,
26455 payload bytes, report, receipt, and seal. One fresh below-root
`core.autocrlf=true` checkout then makes one NUL-framed root-anchored
check-attr call and one version probe: two Git processes, zero retries,
36/36 attributes/LF files, zero CR bytes, safe paths, and retained 76/76
bindings. Declaration
`sha256:9ff14e5083ed4222f23e0ba68d945515225911633435b73c6c2fe4e6d9680a52`
has 9076 controls and grants no product authority.

It contains no hidden profile inputs, privacy canaries, expected records,
expected digests, scorer predicates, acceptance thresholds derived from a
candidate run, or private fixture identifiers.

## Current state

Independent Stage A passed against immutable cutoff
`4371f4f6eb54097bff9badb29278c530d49e2f36`: 789 assertions, 28 LF
artifacts, 11 schemas and 69 closed objects, 41 positive records plus two
nested diffs, 38 mutations, 31 vectors and 161 identities, 59 evidence joins,
10 process archetypes, 112 slots, 26 workflow records, exactly 40 branches,
and 48 links. It reported no public blocker and preserved first-score
integrity.

The three public repository selections are now frozen in
[`repository-selections/`](repository-selections/) by that result.

Independent Stage B/C then completed against immutable cutoff
`8cbb5356fd7b3acca435bc9fad4e97dabab66bb5` with opaque fixture
`P17-R3-D6B553CBC3B1240B673B8190`. Exactly 112 processes were collected
without collection-integrity or privacy failure, and all three repository
workflow slots passed. The valid first score failed only in the public-safe
category `process-exit-agreement`. This is a valid implementation failure,
not invalid custody, and it is not a held-out pass.

The [public-safe result](PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
contains only permitted aggregates. The fixture is sealed in quarantine. The
one-score program is closed: retry, rescore, and reuse are prohibited.
