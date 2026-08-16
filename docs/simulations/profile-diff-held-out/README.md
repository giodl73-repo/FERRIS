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
was governance/test-only at immutable cutoff
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
has 9076 controls. Its sole execution passed all authority and cutoff Pulse 39
bindings (`8/8` tree, `8/8` raw, `5/5` manifest payloads), copied `8/8`
release files, then stopped `invalid` at `pulse-39-release-custody` because
the post-copy raw-binding transaction completed `0/8`. No cutoff checkout,
verifier process, normalized binding check, later gate, seed, materialization,
candidate, or search occurred. The
[public result](pulse-40-public-result/README.md) has raw digest
`sha256:b91ca8ed81a17ddcdb819044e2fa42be53a319a0dec71aaef2ca59b22f9352ca`
and receipt
`sha256:6e78c4e808c24c42f6dbe1df1565768b53a3f71549b82e65621c2e72f4e62237`.
Pulse 40 is permanently invalid, non-retryable, and null-conclusion.

Pulse 41 publishes the
[transactional Pulse 39 copy and post-copy verifier release](pulse-41-transactional-copy-release/README.md).
It is public infrastructure only and neither retries nor reinterprets Pulse
40. The standard-library adapter binds the exact eight-file/31800-byte Pulse
39 release tree, creates and verifies an exclusive sibling stage, flushes and
fsyncs every staged destination before close, and records bottom-up aggregate
staging-directory posture. It makes exactly one rename with zero retries,
discards staging paths, and independently verifies the reconstructed final
root `8/8`. Post-rename verification or operational sync failure is proven
rolled back only after final-path absence and a `synced` or explicit
`unsupported` rollback-parent sync; otherwise it is explicit indeterminate
publication.
The exact private Pulse 40 cause is not provable. Stale-stage,
duplicate/omitted-root, cwd/relative-root, and pre-final-sync-verification are
bounded public reproduction classes only. This adds no diagnostic, custody,
product, fix, score, certification, support, or PLATFORM-001 authority.

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

## Pulse 42 public-result integrity closure

[Pulse 42](pulse-42-public-result/README.md) is permanently
`invalid-publication-integrity`, non-retryable, and null-conclusion at
`public-result-publication`. Its historical authority cutoff is
`2a8b7c27ac465ab78a8ec7ca331b9e427a8625c8`; declaration identity is
`sha256:4da4d749892a487e30467b68bf8e35e9f72655dfb3a75414ead10ff40e0868cc`.
Pulse 38 and Pulse 40 remain unchanged permanent invalid/null-conclusion
predecessors. Historical public bindings remain Pulse 41
`sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8`
and Pulse 39
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c`.
The expected custodian result files were absent (`1`) and claimed result paths
observed were `0`. The committed order makes the reported Pulse 33 stop
inconsistent with later reported quantities, but cannot determine any ordered
gate execution. All custodian claims remain `reported_unvalidated`, including
`P42-FROZEN-BINARY-UNAVAILABLE`, which is not a root cause. No product,
diagnostic, category, or fix authority follows.

## Pulse 43 ordered public-result publisher release

[Pulse 43](pulse-43-ordered-result-publisher-release/README.md) is public
publication infrastructure only. It accepts a bounded public ordered gate
catalog and explicit, closed event records classified as either
`public-artifact-self-validation` or `ordered-execution`. The publisher keeps
their counters separate, rejects duplicate/out-of-order/late execution gates,
and requires one `terminal-stop`; an early Pulse 33 terminal cannot coexist
with later Pulse 31/Pulse 35 execution values. It writes, fsyncs, verifies,
and canonical-hashes the complete two-file result directory in a sibling
stage, makes one rename with zero retries/fallbacks, then recomputes final raw
and payload hashes before returning publication success. Failures state only
`absent`, `rolled-back`, or `indeterminate`, never a success-shaped execution
summary. Its manifest raw/aggregate are
`sha256:8eaca6ebc350a67e493d037132a27749980c16fc79143d69fd0303305b5030a4` /
`sha256:74f6c61913fbfa638f0ba6aae19cb0d2885a47e38b33b19bb80bec1abc870346`;
the qualification receipt raw/payload are
`sha256:3ebc1bfd95dfbfedd1402bb3f3f9f14ea872aec9137a7327b8ca444248091e0c` /
`sha256:9e713bb8f12deced2119fe66028a4c2ab11d6d70d6d0fe90342b996bc1bf25a2`.
Windows directory sync is explicitly `unsupported`, not durable. The release
creates no diagnostic, custody, private-data, product, category, or fix
authority.

## Pulse 44 retained-binary custody release

[Pulse 44](pulse-44-retained-binary-custody-release/README.md) closes the
public retained-executable custody gap without using Pulse 42's invalid
summary as evidence. It pins exact Pulse 33 identities, invokes its immutable
build adapter once with retention enabled, requires fresh absent absolute work
and final roots, validates the logical filename/platform/cutoff/hash/size and
receipt safety fields, then file-fsyncs/verifies a staged executable-receipt
pair before one rename. It independently reconstructs and verifies final
`2/2`, records honest directory synchronization, and emits a
Pulse-43-compatible terminal event only after final verification. Failures are
only terminal `absent`, `rolled-back`, or `indeterminate` states.

The final custody root is non-public runtime state and is never committed.
Windows qualification rejected one dirty clone whose normalization changed
after checkout, then independently passed from a fresh clone fixed to
`core.autocrlf=false` before checkout. The clean invocation published final
executable/receipt `2/2` with one rename, zero retries, size `1436672`, and
exact Pulse 33 artifact SHA-256
`sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`.
All runtime roots were removed afterward. This remains public infrastructure
evidence only and creates no diagnostic, product, category, or fix authority.

## Pulse 45 binary-custody event bridge release

[Pulse 45](pulse-45-binary-custody-event-bridge-release/README.md) composes
the sealed Pulse 44 result into a larger Pulse 43 ordered ledger without
changing Pulse 44. It accepts only the public Windows and Ubuntu platform
identifiers and maps them to distinct stable catalog gates. After exactly one
verified Pulse 44 invocation, it validates the complete closed predecessor
summary. Only published final custody with all `2/2` checks, one rename, zero
retries, and Pulse 44's exact completed terminal event becomes a
platform-specific `gate-complete/passed`. A closed Pulse 44 failure preserves
its public posture as a platform `terminal-stop/failed`; malformed or thrown
predecessor output fails closed. The bridge result is bounded and path-free,
with no private data or executable bytes, no retry/fallback, and no
diagnostic, custody, product, category, or fix authority.

## Pulse 46 publication-order diagnostic authority

[Pulse 46](PROCESS_EXIT_DIAGNOSTIC_PULSE_46_AUTHORITY.md) was an
independent authority at immutable cutoff
`22ea38e274b882d6e607810382f842b76e483f10`. Pulse 42 remains permanently
invalid-publication/null and is not retried, resumed, or reconstructed. The
authority binds complete exact current-cutoff Pulse 41/Pulse 39/Pulse
43/Pulse 44/Pulse 45 trees and their sealed records before any execution.
Its closed catalog distinguishes public-artifact self-validation from ordered
execution, with eight stable gates ending only at
`bounded-process-exit-search`. Each fresh platform checkout fixes
`core.autocrlf=false` before checkout; Pulse 45 calls Pulse 44 once per
platform, and only complete retained-root `2/2` custody may produce a
platform pass.

After both platform passes, inherited public gates govern preflight, Pulse 31
`39/39`, Pulse 35/Pulse 37 normalization, fresh private 32-byte
materialization, and one 70-per-platform/140-total zero-retry search. The
exact Pulse 43 publisher is called once at terminal disposition to an absent
absolute public-result root. No public terminal summary precedes a published
`2/2` final result with recomputed hashes and one rename. Failed publication
is `invalid-publication/null`; it may state only publication posture.

### Pulse 46 permanent public closeout

Authority commit `a80111845f942b75e985c412389bfe6a89ccdc99`, immutable cutoff
`22ea38e274b882d6e607810382f842b76e483f10`, and declaration identity
`sha256:92847e645338fd142710c1afcff5d6ad5540c35e6322ccf59b574f2fd3d61534`
bind the sole launch. It is permanently `invalid-publication-integrity`,
non-retryable, and null-conclusion. The only public custodian statement is:
`Publication posture: indeterminate. The required final public-result directory
is absent.`

The main workspace confirmed that the required final public-result directory
was absent before this closeout. It records one launch and zero retries; all
ordered-gate attempts, completions, terminal-gate, and search details are
indeterminate/null. It records no gate counts or private blocker. This is
not the failed Pulse 43 transactional result; Pulse 43, Pulse 44, and Pulse
45 releases remain unchanged and available for future redesign. The canonical
[public closeout](pulse-46-public-result/README.md) is the sole public record.

## Pulse 47 publication-outcome witness release

[Pulse 47](pulse-47-publication-outcome-witness-release/README.md) releases
only a public persistent witness for one exact Pulse 43 publication outcome.
It pins the sealed Pulse 43 manifest, receipt, seal, and source identities,
calls the publisher exactly once through an injected or verified real callable,
and rejects any malformed, partial, success-shaped incomplete, or thrown
summary before publication.

The witness filters a published predecessor to public result hashes, final
`2/2`, rename/retry/sync posture, and ordered/self-validation aggregates. It
filters a predecessor failure to its code, absent/rolled-back/indeterminate
state, final-files flag, rename/retry values, and exact stage/final-parent/
rollback-parent sync posture. No failed predecessor witness contains ordered
events/counts, paths, private data, or executable bytes.

Exactly `publication-witness.json` and `release-receipt.json` are written,
fsynced, staged-verified, stage-synced, renamed once, final-rehashed, and
parent-synced. A witness failure reveals only its own bounded posture and
code, never captured Pulse 43 material. Pulse 47 does not record, retry,
resume, reconstruct, or infer Pulse 46's permanent closeout and grants no
diagnostic, custody, product, category, score, certification, support, or fix
authority.

## Pulse 48 permanent public closeout

[Pulse 48](PROCESS_EXIT_DIAGNOSTIC_PULSE_48_AUTHORITY.md) is permanently
`invalid-publication-integrity`, non-retryable, and null-conclusion. Authority
commit `5a8d92d211806d0f2940016af6c317878c5fdfc1`, cutoff
`70c8fc2dfa60b6732fa265bb5fcf6326ac97ad2d`, and declaration
`sha256:6c014c640d9184d458a7e750922399fd82fe10eb070b6cf7a4ee8ce409ee5d3e`
bind its sole launch. Blocker `P48-P43-CATALOG-PRIVACY-IDENTIFIER` is at
`public-result-publication`.

The public Pulse 43 result root is absent. The retained Pulse 47 witness root
contains exactly `publication-witness.json` and `release-receipt.json`. It
witnesses `P43-PRIVACY-BEARING-IDENTIFIER`, absent publication, zero rename
attempts/retries, and all sync postures `not-attempted`. Public reproduction
against exact Pulse 43 rejects the committed catalog because
`private-materialization` contains forbidden identifier part `private`.

This establishes only public catalog/publisher incompatibility, not whether or
how far private execution progressed. Category, diagnostic, and product
conclusions are null; no fix authority, private data/gate/search inference,
or rerun exists. A future redesign may use neutral
`bounded-materialization`, but no new authority is created here.

## Pulse 49 withdrawn prelaunch authority

[Pulse 49](PROCESS_EXIT_DIAGNOSTIC_PULSE_49_AUTHORITY.md) is permanently
`invalid-prelaunch-authority-integrity`, non-retryable, and null-conclusion
at `prelaunch-authority-validation`, blocker
`P49-P35-CASE-PROCESS-CARDINALITY-CONFLICT`. Exact authority commit
`80f78fa4edb2d3497a830b2879ea9ff9c6f0aea5` is withdrawn without execution.

The exact Pulse 35 public materializer makes 70 descriptors per platform, not
70 processes: 69 are `launch-ready` and one final `no-launch` case has
`not-materialized` before/after states and
`external-immutable-binary-freeze`. Thus the authority's 70 processes per
platform and 140 total cannot honor the exact descriptor corpus.

No launch, P47/P43 invocation, private operation/data/artifact, result or
witness root, runtime/public-root transfer, or inference exists. The
declaration/schema/mutations remain exact historical artifacts. A future
successor requires new explicit authority for 70 case dispositions, 69
processes, and one no-launch disposition per platform: 140 cases, 138
processes, and two no-launch dispositions total.

## Pulse 50 withdrawn prelaunch authority

[Pulse 50](PROCESS_EXIT_DIAGNOSTIC_PULSE_50_AUTHORITY.md) historical authority
commit `48fe9fdcdda03378f68781cae342796c9f11720d`, with cutoff
`94d473563a1686091be94a72f491b0ff0d903800`, is permanently
`invalid-prelaunch-infrastructure-integrity`, non-retryable, and
null-conclusion. The blocker is
`P50-EXECUTOR-RELEASE-OUTSIDE-AUTHORITY-CUTOFF` at
`prelaunch-public-infrastructure`.

Multiple independent custodians stopped before launch because the authority and
cutoff omitted the sealed executor. Public blockers were Pulse 35's CRLF/LF
schema binding, no descriptor/69+1/P43 runner, Pulse 27 callable-not-CLI
seam, Pulse 31 schema-count drift, WSL/canonical Ubuntu mismatch, incomplete
exact Ubuntu Pulse 33 toolchain/hash custody, and Python resolver detail.
There was no diagnostic execution, private material, seed, descriptor,
candidate process, P43/P47 invocation, result root, witness root, or
inference; all execution state is zero or false.

## Pulse 51 public diagnostic-executor release

[Pulse 51](pulse-51-diagnostic-executor-release/README.md) is a public,
synthetic-qualified prelaunch infrastructure release, not a diagnostic
authority. It completes the P35/P37 custody binding by covering all ten P35
release-tree files and the P35 machine schema's raw CRLF and canonical LF
identities. It also binds exact P27 API use, frozen P31 source artifacts,
P33 binary/receipt/toolchain identities, canonical platform mapping, and
sealed P43/P45/P47 imports.

Its runtime callable consumes already-materialized descriptors, one declared
private runtime root, and exact P44 custody summaries/final roots; it never
materializes a seed or descriptor and accepts no caller-selected authority
grant, gate event, launcher, or P33 expectation. It calls sealed P45 once per
platform, launches Windows natively and Ubuntu through exact
`Ubuntu-24.04` WSL dispatch with verified path translation, validates complete
frozen JSON/human output contracts, independently derives every non-null
profile record from accepted before/after evidence, and recomputes `diff_id`,
selection, invocation, and result identities before comparing only the public
path-free process-exit semantic projection. It validates 70 ordered cases as
69 launch-ready process records plus one no-launch disposition per platform,
retains private process records only in memory, and returns a P43-safe
catalog/event list. Qualification uses public fake executables only through
the same dispatch constructor, keeps all scratch outside the sealed release
tree, and does not invoke terminal publication.

This does not cure or execute Pulse 50. Pulse 51 commit
`d09c923c1e2cd2be003026597f4ad2a0e2d3764f` is the direct child of the
historical Pulse 50 authority and outside its cutoff. Its manifest, receipt,
and seal bind public infrastructure only; they cannot retroactively make Pulse
50 executable.

Pulse 53 bound exact Pulse 51 and exact Pulse 52 to correct terminal
classification only. Pulse 59 later rebinds that exact terminal closeout to
exact Pulse 58 live-capability ordering. Pulse 60 then recorded one fresh
one-shot authority over exact final Pulse 59, but that authority is now
permanently withdrawn before launch: exact helper review proved its sealed
runtime/P27/P41 root contract contradicted the callable stack. Pulse 61 then
corrected the safe-parent wording at cutoff
`70ed752359c04e4aac77a49280c37f2cf6b8d012`, but that authority is now also
permanently withdrawn before launch: safe-existing parents still did not
prove the exact child creation, reversible cleanup, restrictive-permission,
same-filesystem-rename, and path-length prerequisites required by the exact
Pulse 41/P56/P57/P58/P59 stack. Pulse 63 later recorded one fresh exact Pulse
57 WSL qualification authority at cutoff
`5ad78a0623611ad57797ec4e9da34345b40a6e38`, but that authority is now also
permanently withdrawn before launch: its declared WSL preflight still
underbound and partially contradicted the exact Pulse 57 bundle/worker
bootstrap route. Pulse 64 then recorded one fresh exact successor authority
at cutoff `2388b7d9a5fda7f9cbf772e12d1b4c07d22f2161`, but that authority is
now also permanently withdrawn before launch: it still permitted a
nonexistent/unbound optional `qualify_exact_p57_wsl_bootstrap_contract`
branch and literal placeholder strings instead of exact
`SystemRoot`/`SYSTEMROOT` source-precedence derivation, concrete path
comparison, and regular non-reparse identity proof. Pulse 66 is now the new
successor authority at immutable cutoff
`3a99e9e0f383a9821297ef47778fd586b447b7ba`: it preserves exact final Pulse
59, records the Pulse 65 withdrawal, keeps the conservative actual path/root
probes, and requires exactly two harmless bounded WSL process spawns with no
retry and no hidden third spawn before any sole Pulse 59 call. Spawn 1 is the
exact Pulse 57 `subprocess.run` stage-bundle route with exact
`SystemRoot`/`SYSTEMROOT` derivation, payload/stdout bounds, staged set, and
cleanup. Spawn 2 is the exact `subprocess.Popen` worker bootstrap route over a
pre-staged fake-dependency probe bundle that preserves exact worker
bootstrap/source-loader/ready/close cleanup semantics while preventing any
real Pulse 56 capability or FERRIS execution. No Pulse 66 execution has
occurred, and it still cannot consume or revive withdrawn Pulse 50 authority
or permanently closed Pulse 55. Pulse 65 is now permanently withdrawn before
launch because its declared exact WSL preflight still collapsed one harmless
bounded spawn into proof of both stage bundle and worker bootstrap even though
exact source uses separate `subprocess.run` and `subprocess.Popen` spawns.

## Pulse 52 ordered-materialization executor release

[Pulse 52](pulse-52-ordered-materialization-executor-release/README.md)
prospectively stages exact P35 materialization after, rather than before,
Pulse 51 public gates.  It seals exact Pulse 51 source/tree/callables and P35
source plus exact Pulse 39/Pulse 41 source, manifests, receipts, seals, and
callables.  It accepts concrete fresh P39 checkout and P41 final-copy roots
plus P44 custody inputs; invokes and validates exact P39/P41 custody before
constructing gate 1; runs gates 1–6 once with an absent private namespace; and
then creates one 32-byte CSPRNG seed through `O_EXCL`/`fsync`.  It calls exact
P35 materialization and verification once, records the commitment only
privately, removes the seed and descriptor namespace under a verified bounded
policy, then reuses P51's fixed Windows/`Ubuntu-24.04` WSL dispatch and a
fresh one-use P47 seam.
Private dispatch completion is not terminal publication success: only complete
verified P43 result and P47 witness shapes produce `published`.  A P43/P47
failure, malformed return, or missing final root closes
`invalid-publication-integrity` with null product/category/fix conclusions,
no appended event or retry, and verified removal of the terminal parent and
publication residue.  A terminal cleanup or absence-verification failure after
the one-use seam raises only the public-safe unresolved
`terminal-publication-cleanup-indeterminate` posture, not a completed
closeout.

Twenty deterministic fake-only cycles passed at `70/69/1`, one P39
verification and P41 copy/reverification each, 138 fake dispatches each, and
2,760 total.  They prove no seed or descriptor namespace exists while gates
1–6 run, P39/P41 evidence mutations create no seed, materializer failure
consumes the private launch and terminalizes, and public P43 events contain no
raw seed, commitment, path, descriptor, token, or private record.  This is
infrastructure only.  It neither creates authority nor executes a real FERRIS
diagnostic, and any future authority must bind exact Pulse 51 **and** Pulse
52.

## Pulse 53 witness-preserving ordered executor release

[Pulse 53](pulse-53-witness-preserving-ordered-executor-release/README.md)
reuses exact Pulse 52 public-to-private phase helpers but replaces the bounded
terminal classification.  A complete verified P43 result plus P47 witness is
`published-result`.  A complete verified P47 witness of exact P43
`absent`/`rolled-back`/`indeterminate` publication failure is instead the valid
permanent `published-failure-witness` closeout: it retains only the two-file
witness root, verifies no P43 root/stage residue, and has null conclusions.
The transfer descriptor contains only a fixed tree kind, exact counts, and
verified raw/payload hashes; no root path or private value is public.

Failed/malformed/unverifiable P47 output, hash mismatch, unexpected retained
P43 root, or missing final shape is `invalid-witness-publication`.  There is
one P47 call, no retry/republication, and exact Pulse 52 bounded verified
cleanup; unresolved cleanup is a non-returning public-safe fatal state.
Qualification ran 20 alternating fake-only cycles (ten result, ten
failure-witness) with all three failure postures and no FERRIS binary.  This
release is infrastructure only and grants no authority or conclusion.

## Pulse 54 independent witness-preserving diagnostic authority

[Pulse 54](PROCESS_EXIT_DIAGNOSTIC_PULSE_54_AUTHORITY.md) was published as
`authorized-unexecuted` authority at self-excluding cutoff
`42a16e298c5af55b05df5ceb8e3477d0dd45c814`. Its canonical declaration binds
the complete current public P27/P31/P33/P35/P37/P39/P41/P43/P44/P45/P47/P51/
P52/P53 chain, including exact trees, raw hashes, manifests, receipts, seals,
source APIs, and signatures. It preserves the permanent invalid/null Pulse 48
record and the withdrawn invalid/null Pulse 49/Pulse 50 records without retry,
resume, reconstruction, reseed, reuse, correlation, or inference.

The authority creates no runtime artifact. Its one future P53 production call
is injection-free and follows fresh anonymous `core.autocrlf=false` cutoff
checkouts, exact P33 Windows `/Brepro` and `Ubuntu-24.04` WSL binary freezes,
one P44 custody operation per platform, fresh P39/P41/runtime/P27 roots, and
P43-safe gate/validation prechecks. P53's fixed order is P39/P41, Windows
P44/P45, Ubuntu P44/P45, P27, P31, P35/P37, one 32-byte CSPRNG seed, one
materializer/verifier, `70/69/1` per platform, `140/138/2` total, first
semantic-projection mismatch stop, and one P47-to-P43 terminal route.

For `published-result`, exact verified P43 `2/2` and P47 `2/2` trees are
copied and reverified in the separate Pulse 54 result and witness paths. For
`published-failure-witness`, the P43 result path remains absent and only the
exact P47 `2/2` witness is copied as a permanent null-conclusion
publication-integrity closeout. Invalid or cleanup-indeterminate output makes
no success claim and copies no tree. No result, witness, or private runtime
path is public at authorization.

Independent pre-call custody stopped before Pulse 33/Pulse 44 work or the sole
P53 invocation because the required validator compared a CRLF Pulse 35
working-tree identity against canonical LF bytes in the mandated fresh
`core.autocrlf=false` checkout. Pulse 54 is permanently withdrawn
`invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`, with zero execution and null
conclusions. A successor must use a fresh cutoff and
checkout-materialization-independent variant validation.

## Pulse 55 immutable-blob successor authority

[Pulse 55](PROCESS_EXIT_DIAGNOSTIC_PULSE_55_AUTHORITY.md) was published as a
fresh `authorized-unexecuted` successor to permanently withdrawn Pulse 54, not
a retry or amendment. Its cutoff is
`47113e444ef3309afec9a844f0cba62775f19f6f`; it binds the same exact
P27/P31/P33/P35/P37/P39/P41/P43/P44/P45/P47/P51/P52/
P53 chain and sole one-shot P53 callable. Canonical tree and API identities are
read from immutable cutoff Git blobs, then the local runtime checkout is
validated only against canonical values or explicitly sealed variants. P35
keeps exact P37-normalized LF identities and Pulse 51 authorized CRLF/LF
variants with size/newline framing. Fresh anonymous `core.autocrlf=false`
remains mandatory.

Pulse 54 remains permanently withdrawn
`invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY` with zero calls/artifacts/
conclusions. It remains distinct from Pulse 55's consumed one-call closeout.

At authority commit `36b3ac6b9692924af57c7c98b0a976835fe778f6`, fresh P44
custody occurred once per platform before the sole P53 call. It returned public
publication disposition/classification `not-attempted` at
`pulse-41-pulse-39-public-custody`, with zero completed gates, seed,
descriptors, processes, no-launch dispositions,
P27/P39/P41/materializer/verifier/P47 calls, and result/witness transfers;
all conclusions are null. Pulse 55 is permanently
`terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`, non-retryable, and non-resumable.

Windows' retained binary hash/size differed from P51's exact expectation;
Ubuntu matched its expected artifact hash/size, but both fresh retained receipt
payload identities differed from P51's published non-retaining receipt
identities. `build_freeze` changes `retained_in_public_bundle` when retaining
the executable, so exact published receipt payload identity cannot equal a
real retained-custody receipt. Windows `/Brepro` plus Rust/Cargo version alone
also did not reproduce the historic binary, leaving the linker/SDK environment
underbound. A future authority requires a new sealed executor chain with a
corrected P33 retained-build/custody contract, fully bound Windows linker/SDK
environment or qualified deterministic linker route, semantic receipt
verification for retained artifacts, and replacement ordered/witness layers
that bind the corrected diagnostic executor. No replacement implementation is
created here. See the
[Pulse 55 execution closeout](PULSE_55_EXECUTION_RECORD.md).

Pulse 55's immutable historical declaration identity is
`sha256:45ac35775c34e8a86fdc90ad1554104f2728a676d51ab46125bfcf126db21655`;
its `19261` controls retain registry total `100582`.

## Pulse 56 retained deterministic build and custody release

[Pulse 56](pulse-56-retained-build-custody-release/README.md) is a new sealed
build/custody foundation, not a diagnostic executor. It internally creates two
fresh exact-cutoff clean checkouts and distinct targets, binds actual
toolchain/linker/environment identities, proves identical bytes, creates a
new retained receipt, and atomically publishes exactly the binary and receipt.
The Windows rust-lld route and Ubuntu WSL cc/GNU-ld route both passed real
two-build probes. Public evidence cannot authorize launch: only a live,
identity-bound in-process handle can invoke the protected descriptor/image-lock
handoff. No FERRIS binary is executed and no authority or conclusion is created.

## Pulse 58 ordered capability/materialization executor successor

[Pulse 58](pulse-58-ordered-capability-materialization-executor-release/README.md)
joins exact P39/P41 public custody ordering with exact Pulse 57 live Pulse 56
capabilities. Its injection-free callable runs all public checks before one
P35 seed/materialization and reuses source-bound P57 dispatch and semantic
helpers without calling P57's descriptor-root production surface. The truthful
catalog has P39/P41, sealed predecessor, Windows/Ubuntu capability, P27, P31,
P35/P37, materialization, descriptor, and bounded process gates; it makes no
P44/P45 or publication claim. Twenty fake-only cycles made 2,760 harmless
launches with no FERRIS execution, authority, result, or conclusion. The P39
checkout is supplied by a future authority, which must establish fresh
anonymous exact-cutoff/HEAD/clean/`core.autocrlf` posture; P58 invokes only
P39's exact path/attribute/LF semantics and validates exact P41 copy.

## Pulse 59 witness-preserving capability/materialization executor

[Pulse 59](pulse-59-witness-preserving-capability-materialization-executor-release/README.md)
delegates exact Pulse 58 production and fake qualification orchestration
unchanged, then terminalizes only after exact Pulse 58 has completed and
removed its private runtime root. It derives one fresh sibling terminal root,
executes one exact Pulse 51/Pulse 47 terminal route, preserves the
`published-result` / `published-failure-witness` /
`invalid-witness-publication` closeouts, and leaves pre-execution Pulse 58
failure `not-attempted`. No post-completion execution event is added.
Qualification ran 20 fake-only cycles, 2,760 harmless launches, all three
bounded Pulse 43 failure postures, and 14 behavioral controls with no real
FERRIS execution or authority.

## Pulse 60 witnessed capability/materialization diagnostic authority

[Pulse 60](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-60.md)
is now a permanently withdrawn historical authority record. Independent
prelaunch review proved its sealed root contract contradicted the exact Pulse
58/Pulse 59/Pulse 41/Pulse 57/Pulse 56 helper stack: it declared
`private_runtime_root` as `fresh-absent` instead of an existing empty safe
runtime directory, treated `p27_cycle_root` as merely `fresh` instead of an
absent direct runtime child, and underbound the final/stage/terminal
separation the one-call route requires. No authority callable or diagnostic
ran, so calls, seeds, descriptors, processes, publications, transfers, and
all conclusions remain zero or null. Retry and resume are prohibited, and a
later successor had to use a new immutable cutoff containing this withdrawal.

## Pulse 61 witnessed capability/materialization diagnostic authority

[Pulse 61](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-61.md)
is now a permanently withdrawn historical authority record. It had
authorized one fresh independent future diagnostic over exact final Pulse 59
head `6945f5fc96868c97267a1635fbb5219cc398eeb4` from immutable self-excluding
cutoff `70ed752359c04e4aac77a49280c37f2cf6b8d012` with declaration identity
`sha256:d3016922f4bcc09b739b0e71f0317edd54d14975edee103bc3ad1cfecb67ec5d`.
Independent prelaunch review later proved that its corrected safe-existing
parent contract still did not prove the exact child creation and reversible
cleanup required by the exact Pulse 41/P56/P57/P58/P59 callable stack:
`private_runtime_root` did not prove creatability of the Pulse 58 namespace or
Pulse 56 Windows custody child; the Pulse 41 final parent did not prove exact
stage/final/rollback rename topology, same-filesystem availability, or
path-length headroom; the Pulse 59 terminal parent did not prove exact sibling
creation/removal; and the native Linux `ubuntu_runtime_parent` did not prove
exact Pulse 57 `.p57-*` bundle and Pulse 56 Ubuntu custody child creation or
immediately auditable executable/noexec prerequisites. No authority callable
or diagnostic ran, so calls, seeds, descriptors, processes, publications,
transfers, and all conclusions remain zero or null. Retry and resume are
prohibited, and any successor must use a new immutable cutoff containing this
withdrawal.

## Pulse 68 witnessed capability/materialization diagnostic authority

[Pulse 68](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-68.md)
is now the withdrawn historical prelaunch authority at immutable cutoff
`48c26aff381eb66459bf099559f0d44971d46f97`; declaration identity
`sha256:d9f840c32413105e337312363812b8ecec706c1c95a69a8f85ea170ad42e2818`.
Independent prelaunch review withdrew it as
`invalid-prelaunch-predecessor-cleanup-contract` under
`P68-P57-STAGED-BUNDLE-CLEANUP` because exact Pulse 57 stages a `.p57-*`
bundle under caller-native `ubuntu_runtime_parent` and
`_NativeWslSession.close()` never removes `staged.root` or verifies absence,
so exact Pulse 58/Pulse 59 overclaimed cleanup over the final Pulse 59 stack.
No authority callable or diagnostic ran, so calls, seeds, descriptors,
processes, publications, transfers, and all conclusions remain zero or null.
Retry and resume are prohibited. The historical closed schema and mutation
registry remain unchanged at `28830` controls, keeping the monotonic total at
`319332`.

## Pulse 67 witnessed capability/materialization diagnostic authority

[Pulse 67](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-67.md)
is now the withdrawn historical prelaunch authority at immutable cutoff
`3ec6a36009fd34765508f729e795042fd610e5d4`; declaration identity
`sha256:d295759eed523c7c4c5d9efcd2c5f424ef6db03309c544cc31718fbd66eb3f05`.
Independent prelaunch review withdrew it as
`invalid-prelaunch-cutoff-probe-claim-contract` under
`P67-ROOT-CUTOFF-P56-LOADER-CONTRACT` because its historical
current-cutoff authority/P39/repo fields still pointed at the older Pulse 66
cutoff and its dynamic harmless probe claimed the exact
`repo_root`/`load_exact_p56`/`Path(p56.__file__).parent` worker leg without
actually deriving `repo_root`, importing the exact staged P56 module, or
validating its callable identities. No authority callable or diagnostic ran, so
calls, seeds, descriptors, processes, publications, transfers, and all
conclusions remain zero or null. Retry and resume are prohibited. The
historical closed schema and mutation registry remain unchanged at `28196`
controls; Pulse 68 later raised the monotonic total to `319332`.

## Pulse 66 witnessed capability/materialization diagnostic authority

[Pulse 66](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-66.md)
is now the withdrawn historical prelaunch authority at immutable cutoff
`3a99e9e0f383a9821297ef47778fd586b447b7ba`; declaration identity
`sha256:2cf44e16b0c61d79ed5ac889ab6fbfe46ee693ce6d9ccf2b4528bb877db45034`.
Independent prelaunch review withdrew it as
`invalid-prelaunch-wsl-probe-bundle-contract` under
`P66-WORKER-HASH-BUNDLE-LIFETIME` because the exact production worker
validates `worker/sealed_dependencies.py` against the production hash before
`ready`, so the declared fake dependency could never witness exact worker
bootstrap, and because the declared spawn 1 cleanup / absence proof
contradicted the required spawn 2 reuse of the same staged `bundle_root`. No
authority callable or diagnostic ran, so calls, seeds, descriptors,
processes, publications, transfers, and all conclusions remain zero or
null. Retry and resume are prohibited. The historical closed schema and
mutation registry remain unchanged at `27156` controls; Pulse 68 later
raised the monotonic total to `319332`.

## Pulse 65 witnessed capability/materialization diagnostic authority

[Pulse 65](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-65.md)
is now the withdrawn historical prelaunch authority at immutable cutoff
`e3b0b62f6dd62b5071886d32a9eedca85c76b4ae`; declaration identity
`sha256:5bd7c876180a3bfb9f0bcb1518ef68921d1b28210d1f717c904753508e28abb0`.
Independent prelaunch review withdrew it under
`P65-P57-WSL-TWO-SPAWN-CONTRACT` because its declared exact WSL preflight
still collapsed one harmless bounded WSL spawn into proof of both Pulse 57
stage-bundle and worker bootstrap even though exact source uses separate
`subprocess.run` and `subprocess.Popen` spawns with distinct ready/close
cleanup semantics. No authority callable or diagnostic ran, so calls,
seeds, descriptors, processes, publications, transfers, and all
conclusions remain zero or null. The historical closed schema and mutation
registry remain unchanged at `25815` controls; Pulses 66 and 67 later raised the
monotonic total to `290502`.

## Pulse 64 witnessed capability/materialization diagnostic authority

[Pulse 64](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-64.md)
is now the withdrawn historical prelaunch authority at immutable cutoff
`2388b7d9a5fda7f9cbf772e12d1b4c07d22f2161`; declaration identity
`sha256:634e7b3197f5d550c6f3816dbf13770d44738c4f05de6956aa07966548a0be23`.
Independent prelaunch review withdrew it under
`P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION` because its declared WSL
qualification still permitted an unimplemented/unbound optional
`qualify_exact_p57_wsl_bootstrap_contract` branch and used literal
`%SystemRoot%` placeholders instead of exact `SystemRoot`/`SYSTEMROOT`
source-precedence derivation, concrete path comparison, and regular
non-reparse identity proof. No authority callable or diagnostic ran, so
calls, seeds, descriptors, processes, publications, transfers, and all
conclusions remain zero or null. The historical closed schema and mutation
registry remain unchanged at `24700` controls; Pulses 65, 66, and 67 later raised
the monotonic total to `290502`.

## Pulse 63 witnessed capability/materialization diagnostic authority

[Pulse 63](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-63.md)
is now the withdrawn historical prelaunch authority at immutable
cutoff `5ad78a0623611ad57797ec4e9da34345b40a6e38`; declaration identity
`sha256:b8cfea5cc8cb6dc52a7974f4fee35f6351557158943cc92af388c534421915d5`.
Independent prelaunch review withdrew it under
`P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT` because its declared WSL preflight
still underbound and partially contradicted the exact Pulse 57 bundle/worker
bootstrap route: it used smaller generic input/output/timeout limits than the
exact Pulse 57 payload/protocol bounds, pointed to a misleading Pulse 59
schema path instead of the actual staged Pulse 56 release tree, exposed
path-bearing probe outputs instead of exact canonical `bundle_root` stage
JSON, and did not fully bind the exact worker bootstrap/source-loader or
private-parent comparison contract. No authority callable or diagnostic ran,
so calls, seeds, descriptors, processes, publications, transfers, and all
conclusions remain zero or null. The historical closed schema and mutation
registry remain unchanged at `23266` controls; Pulses 64, 65, 66, and 67 later
raised the monotonic total to `290502`.

## Pulse 62 witnessed capability/materialization diagnostic authority

[Pulse 62](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-62.md)
is now the withdrawn historical prelaunch authority at immutable cutoff
`e38dd20f37923e84ac3a3377892c1a5d0954266a`; declaration identity
`sha256:f0db3ddf18a796d0ec107d6d73e9a08cf5e59d47cdad880d584ee8c7e8f61c5a`.
Independent prelaunch review withdrew it under
`P62-REAL-PATH-WSL-ROUTE-CONTRACT` because its reversible path
qualification still underbound actual caller-supplied root basenames and
deepest real Pulse 41/P56/P57/P58/P59 paths, and it did not bind the
exact harmless WSL `wsl.exe --distribution Ubuntu-24.04 --exec
/usr/bin/python3` gate-3 preflight route. No authority callable or
diagnostic ran, so calls, seeds, descriptors, processes, publications,
transfers, and all conclusions remain zero or null. The historical closed
schema and mutation registry remain unchanged at `21644` controls; Pulses 63,
64, 65, 66, and 67 later raised the monotonic total to `290502`.
