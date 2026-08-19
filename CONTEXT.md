# Ferris Context

Ferris is the cross-workspace enterprise build system for Rust.

This repository contains a bounded experimental Ferris implementation in
addition to its research, specification, and governance records. Its
22-specification spine is complete at Draft status.

The specification simulation program is complete at Draft after 11 waves and
46 frozen scenarios. It resolved all 25 Simulation Issues through 25 applied
Specification Change Records and froze a separate structural held-out
implementation fixture set.

The closed bounded read-only wave in
`context/waves/2026-08-11-read-only-planning/` authorizes local `plan`,
`explain`, bounded declared-workspace `graph`, passive local `doctor`, and the
Pulse 14 two-file experimental `profile-diff` behavior over explicit local
inputs and development fixtures. Pulse 15 adds only a nine-family development
fixture matrix and conformance test for that existing behavior. Pulse 16 adds
only a public held-out scoring contract and independent custody protocol.

The separately closed bounded
`context/waves/2026-08-17-conservative-validation-plan/` wave authorizes only
local read-only `validation-plan` over one explicit workspace manifest,
explicit local changed workspace paths/packages, and ordinary Cargo metadata.
It does not authorize validation-command execution, repository-owned
validation declarations, affected-only git discovery, query, mutation, active
probes, connectors, MCP, AI narrowing, approval, deployment, or remote
evidence.

The separately closed bounded
`context/waves/2026-08-18-federated-application-plan/` wave authorizes only
local read-only `federated-plan` over one strict request naming 2-16 explicit
Cargo workspaces below one request-parent ancestor. It preserves one existing
non-executable Blueprint Plan per workspace, uses one bounded offline and
locked Cargo metadata invocation per workspace, and rejects duplicate Cargo
workspace roots. It does not authorize cross-workspace dependency inference,
affected scope, validation composition, execution, mutation, hidden
discovery, connectors, MCP, AI narrowing, approval, deployment, remote
evidence, or the full APPLICATION-001 Application Definition.

The separately bounded
`context/waves/2026-08-19-federated-validation-reconciliation/` wave
reconciles the preserved `application-definition-prototype` commits
`02a8337`, `eca5599`, and `ba3566f` with canonical current main at `cebce42`.
It authorizes only the separate read-only `federated-validation-plan` command
over one strict consumer-owned `ferris.application/v0` definition and
explicit changed inputs. Directly affected workspaces retain the unchanged
single-workspace `ferris.validation-plan/v0` record; explicit transitive
reverse `depends_on` relationships widen to workspace fallback, and an
application-level path widens all declared workspaces. Cargo resolution and
lock authority remain independent per workspace. No relationship inference,
validation execution, owner command, mutation, shared lock graph, existing V0
contract change, or full APPLICATION-001 authority is granted. The prototype
branch remains preserved but is superseded for product integration; the
request-based `federated-plan` remains canonical for relationship-free plan
collation.

The measurement-only
`context/waves/2026-08-19-federated-validation-value-proof/` wave authorizes
one synthetic eight-workspace development fixture, deterministic scope
assertions, and an opt-in local planning-overhead report for the unchanged
`federated-validation-plan`. It adds no product behavior, schema, dependency,
execution, relationship inference, or production-value claim. Workspace
scope reduction is reported separately from planning latency and is not
treated as a build-time, command-count, or cost saving.

The follow-on measurement-only
`context/waves/2026-08-19-federated-validation-scenario-matrix/` wave reuses
that fixture unchanged to validate graph-depth response, independent-branch
union, mixed path/package inputs, nested workspace-manifest fallback, and
input-order identity. It adds no product behavior, timing claim, inferred
relationship, schema, dependency, or execution authority.

The measurement-only
`context/waves/2026-08-19-federated-validation-scaling-boundary/` wave uses
disposable generated applications to validate accepted 2-, 4-, 8-, and
16-workspace sizes, typed rejection at 17, full-chain propagation, and local
sequential metadata scaling. It changes no product code, public fixture,
process control, dependency, timeout, caching, parallelism, or execution
authority.

The measurement-only
`context/waves/2026-08-19-public-portfolio-validation-pilot/` wave runs the
unchanged planner across clean exact public FERRIS, PARLOR, RUNE, and ICELINES
revisions. PARLOR and RUNE are explicit conservative contract-migration
consumers, not inferred or live Cargo dependents; ICELINES is an unrelated
control. The pilot retains owner-local Cargo authority, mutates no child repo,
executes no validation, and creates no production or support claim.

The successor
`context/waves/2026-08-12-platform-profile-conformance/` wave has no active
authority. Pulse 85 permanently closes the sole consumed Pulse 84 invocation
after exact Pulse 82 stopped `not-attempted` at Ubuntu capability build
custody with `P57-WSL-BUNDLE`. Pulse 86 now seals prospective capability-only
infrastructure over exact Pulse 78: it derives the operational WSL username
from the native runtime parent's owner and binds every WSL spawn explicitly,
without filtering stderr, retrying Pulse 84, or granting authority. Pulse 87
now preserves exact Pulse 81 ordered behavior over exact Pulse 86, including
the parent-owner binding and indeterminate-stage terminal disposition. Pulse
88 now preserves exact Pulse 82/Pulse 59 witness semantics over exact Pulse
87 and proves the indeterminate-stage disposition remains non-publishable.
The capability, ordered, and witness-preserving successor chain is complete,
but no diagnostic authority exists. Pulse 68 is
now permanently withdrawn under
`P68-P57-STAGED-BUNDLE-CLEANUP`, with zero calls, seeds, descriptors,
processes, publications, and transfers plus null conclusions. Pulses 69
through 82 are sealed predecessor infrastructure over the exact Pulse 57/Pulse
58/Pulse 59 chain. The latest capability, ordered, and witness-preserving
stack terminates truthfully at Pulse 88, including Pulse 86's WSL parent-owner
binding, Pulse 87's exact Pulse 35 release-tree binding, and Pulse 82's
terminal witness semantics. It is
ready for any separate new-authority review, but no authority has been
created and no real FERRIS diagnostic has executed. Pulse 83 records a static
governance/test-only readiness proof at merged cutoff `dfc889b`; it maps every
public Pulse 68 blocker to exact sealed successors and grants no authority.
Independent prelaunch review proved
exact Pulse 57 stages a `.p57-*` bundle under caller-supplied native
`ubuntu_runtime_parent`, `_NativeWslSession.close()` closes
worker/capability but never removes `staged.root` or verifies absence, and
exact Pulse 58/Pulse 59 therefore overclaimed complete cleanup and zero
residue across the final callable stack. Pulse 67 is now
permanently withdrawn
`invalid-prelaunch-cutoff-probe-claim-contract` under
`P67-ROOT-CUTOFF-P56-LOADER-CONTRACT`: independent prelaunch review proved
its current-cutoff authority/P39/repo fields still pointed at the old Pulse 66
cutoff and its dynamic harmless probe overclaimed the exact
`repo_root`/`load_exact_p56`/`Path(p56.__file__).parent` worker leg. Pulse 66 is now permanently withdrawn
`invalid-prelaunch-wsl-probe-bundle-contract` under
`P66-WORKER-HASH-BUNDLE-LIFETIME`: independent prelaunch review proved the
exact worker rejects the fake sealed dependency bytes before `ready` and the
declared spawn1 cleanup contradicted the required spawn2 bundle handoff.
Pulse 65 is now permanently withdrawn
`invalid-prelaunch-wsl-spawn-cardinality-contract` under
`P65-P57-WSL-TWO-SPAWN-CONTRACT`: independent prelaunch review proved
its declared exact WSL preflight still collapsed one harmless bounded
WSL spawn into proof of both Pulse 57 `_stage_wsl_bundle` and worker
bootstrap even though exact source uses separate `subprocess.run` and
`subprocess.Popen` spawns with distinct ready/close cleanup obligations. Pulse 64 is now
permanently withdrawn
`invalid-prelaunch-unbound-wsl-qualification-contract` under
`P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION`: independent prelaunch review proved
its sealed WSL qualification still permitted a nonexistent/unbound optional
`qualify_exact_p57_wsl_bootstrap_contract` branch and literal placeholder
strings instead of exact `SystemRoot`/`SYSTEMROOT` source-precedence
derivation, concrete path comparison, and regular non-reparse identity proof,
so no authority callable or diagnostic ran and calls, seeds, descriptors,
processes, publications, and transfers remain zero while all conclusions
remain null. Pulse 63 is now also permanently withdrawn
`invalid-prelaunch-wsl-bootstrap-contract` under
`P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT`: independent prelaunch review proved
its declared WSL preflight still underbound and partially contradicted the
exact Pulse 57 `_NativeWslSession` bootstrap path. It bound smaller generic
input/output/timeout limits instead of Pulse 57's exact payload/protocol
bounds, pointed to a misleading Pulse 59 schema path instead of the actual
staged Pulse 56 release tree under
`repository/.../pulse-56-retained-build-custody-release/`, exposed
path-bearing probe outputs instead of the exact canonical `bundle_root` stage
JSON, and did not fully bind the exact worker bootstrap/source-loader
assumptions or a private compare-only `ubuntu_runtime_parent` record. No
authority callable or diagnostic ran, so calls, seeds, descriptors,
processes, publications, transfers, and all conclusions remain zero or null.
Pulse 62 is now also permanently withdrawn
`invalid-prelaunch-path-route-contract` under
`P62-REAL-PATH-WSL-ROUTE-CONTRACT`: independent prelaunch review proved its
sealed reversible-probe contract still underbound actual caller-supplied
root basenames and deepest exact Pulse 41/P56/P57/P58/P59 paths, and its
Ubuntu contract omitted the exact harmless WSL `wsl.exe --distribution
Ubuntu-24.04 --exec /usr/bin/python3` gate-3 preflight route, so calls,
seeds, descriptors, processes, publications, and transfers remain zero and
all conclusions remain null. Pulse 61 is now also permanently withdrawn
`invalid-prelaunch-root-creatability-contract` under
`P61-ROOT-CREATABILITY-CALLABLE-CONTRACT`: independent prelaunch review
proved its safe-existing parent contract still did not prove the exact child
creation, reversible cleanup, restrictive-permission, same-filesystem-rename,
and path-length prerequisites required by the exact Pulse 41/P56/P57/P58/P59
callable stack, so calls, seeds, descriptors, processes, publications, and
transfers remain zero and all conclusions remain null. Pulse 60 is now
permanently withdrawn `invalid-prelaunch-runtime-root-contract` under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`: independent prelaunch review proved its
sealed `private_runtime_root`, `p27_cycle_root`, and Pulse 41 root-separation
contract contradicted the exact Pulse 58/Pulse 59 helper stack, so calls,
seeds, descriptors, processes, publications, and transfers remain zero and
all conclusions remain null.
Pulse 59 is sealed
witness-preserving terminal infrastructure over exact Pulse 58: it preserves
Pulse 53's terminal classes after exact P58 completion, derives a fresh
sibling terminal custody root, and leaves pre-execution P58 failure
publication not-attempted. Pulse 69 is sealed cleanup-owning infrastructure
over exact Pulse 57: it
retains exact staged `.p57-*` bundle identity through the worker lifetime,
closes worker/capability first, removes only its owned native bundle through a
bounded no-follow tree cleanup, verifies absence, and makes cleanup
uncertainty fatal with precedence, without modifying frozen Pulse 57 release
artifacts or executing real FERRIS. Pulse 70 is sealed ordered capability/
materialization infrastructure over exact Pulse 69: it preserves Pulse 58's
public-before-private ordering, delegates the fixed cleanup-owning capability
layer, and adds no publication or authority. Pulse 71 is sealed
witness-preserving capability/materialization infrastructure over exact Pulse
70: it preserves Pulse 59's terminal classes after exact Pulse 70 completion,
binds fresh exact Pulse 70/Pulse 52/Pulse 69/Pulse 51/Pulse 43/Pulse 47
modules behind a cross-instance kernel lock, keeps publication
not-attempted until Pulse 70 completes, and adds no authority. Pulses 72
through 74 close the stage-to-identity race and rebase the ordered and
witness-preserving layers over that hardened capability route. Pulses 75
through 77 add stage-bootstrap cleanup ownership, worker-launch identity
binding, and matching ordered/witness successors. Pulses 78 through 80 close
the remaining mkdir-to-open ownership-capture and bootstrap argv/dependency
loader gaps, then preserve the ordered and terminal publication semantics over
that route. Pulse 81 binds the exact Pulse 35 manifest, receipt, seal, file
set, sizes, hashes, and sole manifest source digests into the ordered chain.
Pulse 82 rebases the witness-preserving terminal layer over exact Pulse 81.
All of Pulses 72 through 82 are fake-qualified sealed infrastructure only:
they create no authority and perform no real FERRIS diagnostic. Pulse 83
statically binds the merged cutoff, exact Pulse 82 tree/manifest/seal/source,
and the complete blocker-to-successor map as
`ready-for-separate-authority-drafting`. It invokes no callable and creates no
authority. Pulse 84 now records one fresh self-excluding diagnostic authority
at cutoff `f874ebf` for exactly one later independent invocation attempt of
the exact Pulse 82 callable. Pulse 85 permanently closes that authority after
the sole call passed Pulse 39/Pulse 41, sealed predecessor, and Windows
capability custody, then stopped `not-attempted` at Ubuntu capability build
custody with `P57-WSL-BUNDLE`. No seed, candidate, publication, or transfer
occurred; cleanup completed and all conclusions remain null. Pulse 55
consumed its sole P53
call and
permanently closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`: it returned `not-attempted` at
`pulse-41-pulse-39-public-custody`, with null conclusions and no transfer.
It is non-retryable and non-resumable. Pulse 54 is separately permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity`, zero-launch and
null-conclusion, after its required validator failed in the exact fresh
`core.autocrlf=false` custody posture. Pulse 52 is a sealed
ordered-materialization infrastructure release: it binds exact Pulse 51, runs
public gates before one private P35 materialization, and grants no authority.
Pulse 49 is permanently withdrawn
`invalid-prelaunch-authority-integrity` before execution. Pulse 50 is
permanently `invalid-prelaunch-infrastructure-integrity`, withdrawn before
launch with a null conclusion under
`P50-EXECUTOR-RELEASE-OUTSIDE-AUTHORITY-CUTOFF`. Pulse 51 is a sealed public
prelaunch executor release published after Pulse 50 authority/cutoff and
cannot retroactively make it executable. Pulses 46 and 48 are permanently
`invalid-publication-integrity`, non-retryable, and null-conclusion.
PLATFORM-001
remains Draft solely because the valid Pulse 17 first score failed
`process-exit-agreement`. Pulse 21 closes the separate RUNE dependency by
recognizing the already-bound exact revision as RUNE's accepted v1 contract
and release-readiness baseline, without claiming Cargo SemVer `1.0.0`, a Git
v1 tag, profile v1, or product behavior. Pulse 22 closed as `invalid` after
one retained Windows process and a collector durability failure before the
required Ubuntu partner launch. It produced zero completed pairs, zero
retries, no minimization, and no category conclusion. Each pulse retains its
own bounded authority.

Pulse 23 independently diagnosed and repaired the Pulse 22 collector
durability path without executing or replaying a Ferris candidate. The
repaired writable-handle, atomic-replacement, directory-sync, and residue
checks passed 20 synthetic Windows/Ubuntu pairs and 20 unit tests with zero
failures. This qualifies infrastructure only; it does not authorize a
replacement search or change any diagnostic or product conclusion.

Pulse 24 opened one new independent replacement diagnostic search at
immutable cutoff `cef0daabc349ac2333869959f21b9a3106e10484` and closed
`invalid-before-candidates`. The custodian verified the Pulse 23 public
report and its declared bindings but could not obtain an exact inspectable
collector source copy from permitted public material. No preflight, corpus
generation, candidate, minimization, or category conclusion exists. The
missing public collector source bundle is an infrastructure supply-chain gap,
not product evidence.

Pulse 25 closes that gap by releasing the exact qualified nine-file
collector source/test bundle with deterministic source, test, and bundle
aggregates. The copied public bundle passed 20 unit tests, 20 synthetic
Windows/Ubuntu pairs, 40 harmless commands, four fresh-process reloads, and
zero residue. It contains no diagnostic data, authorizes no search, and grants
no production, score, certification, fix, or PLATFORM-001 status authority.

Pulse 26 opened one new independent diagnostic program
at immutable cutoff `e01130a5c1fc5b8e58e13bbde03dfc39b8f1bf60`, using only
the exact public Pulse 25 bundle. Its independent execution verified every
public binding and immutable cutoff, then stopped `invalid` during mandatory
synthetic preflight. Two pairs were attempted, one completed, four harmless
processes ran, and the second pair failed exact-cardinality fresh-process
reload. No generation, candidate, retry, minimization, or category conclusion
exists. Pulses 22, 24, and 26 are permanently invalid, non-retryable, and
unable to produce category conclusions.

Pulse 27 publishes the exact public two-pair adapter and an unchanged copy of
the Pulse 25 collector. The root cause was pair-local expected cardinality
`1` incorrectly supplied to a whole-store verifier after pair two existed;
the immutable collector required no modification. Qualification passed 50 of
50 cycles, 200 process rows, 100 pair seals, 100 fresh-process reloads, zero
retries, and zero residue. The original checkout-dependent 20-file release
aggregate was
`sha256:31f38a79629d6b5da1fab9cb335450a95a1763f1ac80b1d8d851b103a318e540`.
This is infrastructure evidence only and executes no diagnostic candidate.

Pulse 28 authorized one new independent diagnostic program at immutable
cutoff `2935f44475b811e619f2ef62e0d408f39c7e8149`, using only the exact
public Pulse 25 collector bindings and exact Pulse 27 20-file adapter
manifest. New isolated custody must copy exactly those 20 public files,
independently recompute every file and adapter/test/collector/release
aggregate, run exactly one adapter invocation producing two Windows/Ubuntu
pairs, four rows, and two seals, then run exactly two fresh platform
verifiers enforcing whole-store cardinality `2/2/2`, with zero retries and
zero residue.

The public binding audit stopped `invalid-before-candidates`: 60 checks
produced 10 passes and 50 failures. The first mismatch was the Pulse 25
manifest expected at
`sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75`
and observed at
`sha256:03322e9fe6a3df6c71161e5f3916c51cc66c9453e9f1f3141bcc703bd02d7a0d`.
Git worktree EOL conversion at checkout, not corrupted Git blobs, caused the
mismatch. No package copy, build, preflight, generation, candidate, pair,
seal, retry, or category conclusion exists, and further launches are
prohibited.

Pulse 29 forces LF checkout for every file under the byte-bound Pulse 25 and
Pulse 27 public release roots through anchored recursive `.gitattributes`
rules. The normalized Pulse 25 bundle aggregate is
`sha256:e296329ff56fad14eba2274d928f45c0fdf6a281db3d2d554c1cee3814d4b406`;
the normalized Pulse 27 release aggregate is
`sha256:531113c7c8a50f1c71c446bc708e44549702623114625ea46f5aa874b6aea721`.
A disposable resulting-index checkout on Windows with
`core.autocrlf=true` verified 36 LF files and 76 of 76 exact binding checks.
This checkout normalization changes no production behavior, diagnostic
authority, closed result, or PLATFORM-001 status.

Pulse 30 authorized one final new independent diagnostic program at immutable
cutoff `cf6b3309c31e5da37d4a8e6655a781f4e92ef603`, which contains the
Pulse 29 attributes and receipt but predates the authority. Before package
copy, custody must materialize that cutoff with `core.autocrlf=true`, require
`text=set` and `eol=lf` for all 36 byte-bound files, verify 36/36 LF files and
all 76/76 normalized bindings, then copy exactly the 20 Pulse 27
manifest-listed files and recompute every digest. The unchanged exact
preflight requires one adapter invocation, two pairs, four rows, two seals,
two fresh verifiers, `2/2/2` cardinality, zero retries, and zero residue.
Only after pass may wholly new diagnostic material be frozen under inherited
Pulse 26 bounds.

The independent Pulse 30 result passed 36/36 attribute and LF checks, 76/76
normalized bindings, exact package verification for 20 files, 20 hashes, four
aggregates, and six report/receipt/seal bindings, both binary/environment
freezes, and the exact one-invocation/two-pair/four-row/two-seal/two-verifier
preflight with `2/2/2`, zero retries, and zero residue. It then stopped
`invalid` at `generation-before-case-materialization` because the authorized
public read scope lacked a public `ferris.profile-evidence/v0` input schema.
Its raw result digest is
`sha256:f75d33f054002cdd1b066678163ef926f62ec95ba826fef7273bc614c348f090`
and receipt ID is
`sha256:8f08b0cf27f1b1bb97bcea0591b92c2143cf324736e2112744122838ca58dc30`.
It created zero cases, candidates, or candidate processes and has a null
conclusion. Further launches are prohibited.

Pulse 31 publishes the complete current `ferris.profile-evidence/v0` public
input contract without production changes: a closed five-member root, closed
twelve-member `sections`, recursive JSON values with 1-through-256 visible
ASCII object member names, strict duplicate rejection, an inclusive
1,048,576-byte file limit, exact classification precedence, six positive
fixtures, 33 negative controls, a recursive Draft 2020-12 schema, test-only
validator, and nine-role review. It closes the public documentation gap
prospectively but does not reopen Pulse 30 or add product, score,
certification, fix, support, or PLATFORM-001 authority.

Pulse 32 authorizes one new independent diagnostic program at immutable
cutoff `29517d732db13cc2ffa304684b344f3538ab587d`, which contains Pulse 31
but predates the authority. It inherits every Pulse 30 normalization,
package, adapter preflight, freshness, coverage, oracle, search, collection,
minimization, and publication rule. The declaration pins the exact Pulse 31
input contract and schema raw digests, all six positive fixture path/size/
digest bindings, the mutation-file digest, and all 33 per-control canonical
public digests. After adapter preflight and before generation, independent
custody must verify all nine public artifacts and complete six positive
acceptances plus 33 exact negative classifications using only those public
rules. Ferris source/tests, prior custody, and hidden material are prohibited.
The authority is governance/test-only and executes nothing.

The independent Pulse 32 result passed 36/36 attribute and LF checks, 76/76
bindings, and exact package verification for 20 files, 20 hashes, four
aggregates, and six report/receipt/seal bindings. The Windows direct
executable built, but the Ubuntu direct executable was unavailable, so the
program stopped `invalid` at `cutoff-build-freeze` before environment freeze.
There were zero preflight operations, public-input classifications, generated
cases, candidate processes, retries, search executions, or minimization
processes, and the category conclusion is null. The raw result digest is
`sha256:27ff0f0c2a4768628fdcdfa7916efa7fe12217faa7bec20f65dbde8e526f88fd`;
the receipt ID is
`sha256:cf48f0ddc7102d29084529b1ffe5b8812acd6b2d5cf75ec544265a1b3c0238cd`.
Further launches are prohibited.

Pulse 33 publishes a public external build-freeze release for that exact
cutoff without diagnostic execution or product changes. The initiating
Ubuntu failure was exit 127, `cargo: command not found`, because WSL non-login
shell orchestration omitted the ordinary rustup Cargo directory from `PATH`.
The same cutoff compiles when Cargo is addressed explicitly. The adapter
checks `PATH` and the ordinary rustup location, then uses Cargo
`compiler-artifact` JSON rather than a guessed target path. Qualification
passed 14 unit tests, 20 synthetic checks, two actual build freezes, four
clean deterministic rebuilds, and all 37 manifest files. The Ubuntu artifact
is `sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4`;
the Windows `/Brepro` artifact is
`sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`.
No FERRIS product change is required.

Pulse 34 authorized one new independent diagnostic program at immutable
cutoff `5df7492fa759c415f6ce540a33a4e89c46714348`, which contains the complete
Pulse 33 release and predates the authority. It inherits every Pulse 32
normalization, package, adapter-preflight, public-input, fresh-generation,
oracle, search, minimization, and publication gate. It additionally pins the
exact Pulse 33 37-file manifest, aggregate, release seal, build adapter, and
public receipts and requires explicit WSL non-login Cargo discovery, Cargo
`compiler-artifact` JSON, and exact Windows/Ubuntu cutoff binary sizes,
hashes, and receipts before adapter preflight. The declaration identity is
`sha256:8975e07b9dd417604d06be12a24a448e8ae1834991aca9db086ae7c11b0b1e34`;
704 mutations protect the authority.

Independent custody passed checkout `36/36`, bindings `76/76`, all public
package gates, deterministic Windows and Ubuntu cutoff freezes, the exact
two-pair preflight with `2/2/2`, zero retries and residue, and Pulse 31 input
self-validation `39/39`. It then closed `invalid` at
`generation-materialization` because the frozen generator did not complete an
isolated corpus before any candidate launch. There were zero valid cases,
coverage domains or interactions, candidate pairs or processes, search or
minimization executions, and reproducers. The category conclusion is null and
further launches are prohibited. The receipt ID is
`sha256:dca0ad1579257a6f265ada501533a4034070963267ef7c25478bf38267ee1588`.

Pulse 35 prospectively closes the Pulse 34 generation-materialization gap
without reopening that immutable invalid result. Its public standard-library
adapter requires a readable regular file containing exactly 32 CSPRNG bytes
for both materialization and verification and publishes only
`sha256("ferris-p35-seed-commitment-v1\0" || seed)`. Case IDs, order tokens,
and profile tokens are full HMAC-SHA256 pseudorandom values keyed by that seed;
profile IDs therefore contain seed-derived pseudorandom outputs, not raw seed
slices. It requires exactly 70 concrete descriptors under the inherited
512-case maximum. Descriptors bind raw artifacts or directory/missing state,
expected public classification/result posture, host-independent request
resolution, and semantic witnesses including actual malformed, duplicate,
unsupported, boundary, path, pair, change-count, and no-launch blocked cases.
An independent verifier derives public input precedence, lexical
transformations, pair result, every witness, the 17 value domains plus
interaction-requirements (`18/18`), and all eight exact tuple catalogs
(`8/8`) directly from descriptors, states, bytes, and the private seed;
coverage labels are not trusted. Publication makes one replacement with zero
logical retries, records `synced`/`unsupported` directory status, cleans a
post-creation staging-sync failure, and rolls back confirmed output on final
parent-sync failure. Qualification runs 20 isolated complete-coverage cycles
with private-seed fresh-process reload, same/different-seed, 31/32/33-byte
seed, semantic-tamper, replay, extra-output, residue, rename-failure,
staging-sync cleanup, and final-sync rollback controls,
without executing Ferris, a candidate, a diagnostic, or a product change.
The sealed release identities are recorded by the current manifest and receipt.
The limited public root-cause fact remains only that Pulse 34's frozen
generator did not complete isolated corpus materialization before candidate
launch. No private detail is inferred, and no diagnostic authority is added.

Pulse 36 independently executed under authority
`2bf480459614dc56ee2bd744302e79f20a571092` at immutable cutoff
`48697c8da0e93b92fa633e353925ca05707bf9ed`. It passed inherited checkout
`36/36` LF, bindings `76/76`, Pulse 33's 37-file/two-platform/two-binary/
two-receipt build gate, exact one-invocation/two-pair/four-row/two-seal/
two-verifier `2/2/2` preflight with zero retries/residue, and Pulse 31's nine
artifacts and `39/39` validation. The sole Pulse 35 eight-file copy then
stopped permanently `invalid-before-pulse35-materialization` at
`pulse35-release-copy-verification`: two bindings matched, six mismatched,
and 405,414 expected bytes became 403,316 observed bytes. No seed,
materializer, descriptors, candidates, pairs, or seals were created; the
category conclusion is null and further launches are prohibited. Independent
cutoff-blob reproduction proves checkout/binding infrastructure—not product—
evidence: six text files were sealed with CRLF working-tree bytes while
`.gitattributes` stores LF blobs, with cutoff deltas `-91`, `-970`, `-188`,
`-10`, `-203`, and `-636` for `README.md`, `corpus_materializer.py`,
`qualify.py`, `root-cause-report.md`, `tests/test_materializer.py`, and
`verify_materialization.py`; the JSON qualification and root-cause envelopes
match. The result raw digest is
`sha256:735353e311dc63cd0cdef85c112bd60fd2c50c18f29858929a58f886b34009cc`
and receipt ID is
`sha256:d1f6f648ae8bb9a1fc44def2d392b72b76446b49439ff8f31e4124ad1fafc628`.
The closed Pulse 36 authority declaration identity remains
`sha256:f4d83498f780e6d35bd0073f8d8ddeaa67d99fb2426978190f7af25fff746952`.
Pulse 36 is permanently invalid, non-retryable, and cannot create product,
score, certification, fix, support, or PLATFORM-001 authority.

Pulse 37 normalizes and rebinds the current Pulse 35 public release to its
already-required LF Git-clean bytes. It preserves the cutoff's historical
CRLF-sealed manifest `sha256:9baef3aa3030d7e8261072b26e7bd40436c362163f9138f929f0e4264fd0289b`,
aggregate `sha256:585f0caf7aa4cbe821a71dcb60e5a1b7d6ad0650677b715dcbf143456612a0d7`,
405,414-byte total, and release-seal identities as Pulse 36 evidence, while
publishing the normalized successor manifest
`sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1`,
aggregate `sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`,
and 403,316-byte total. A disposable Windows `core.autocrlf=true`
resulting-index clean-filter checkout passed all 8/8 file size/hash bindings
and zero CR bytes in the six text files. This is public-artifact
normalization only: it changes no product code, runs no qualification,
FERRIS, or diagnostic, creates no diagnostic authority, and does not reopen
Pulse 36.

Pulse 38 created one governance/test-only independent diagnostic authority
at immutable cutoff `6807bd68aa01cbf0c819198765b7d6b5aa443328`, which
contains complete Pulse 37 normalization and predates the authority. It
preserves all invalid predecessors, including Pulse 36, as permanently
invalid, non-retryable, and null-conclusion; it is not a retry, resume,
reseed, reuse, correlation, or inference. It inherits every Pulse 36/Pulse 34
gate and bound while binding the normalized Pulse 35 manifest
`sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1`,
aggregate `sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`,
403316-byte total, seal raw/payload, and Pulse 37 receipt raw/identity.
Custody order is inherited checkout/package/build/preflight/Pulse 31
validation, exact normalized copy and Pulse 37 proof, new private
seed/materialization/fresh verification, then one transactional
cross-platform search bounded to 70 cases/processes per platform and 140
processes total. Its declaration identity is
`sha256:a3317422e8c34d4e08d7c5e577e3539820f1376d7fba2ef38d262d1f967031b4`.
The sole execution bound the cutoff and materialized it once, then stopped
permanently at the first required attribute check before package copy or any
diagnostic activity. The raw public result is
`sha256:d3e74d220a9de9da4f2fff72812443de42272c9a8f78b0efad37573ab33b1c9c`;
receipt
`sha256:56ddacc0e3043b327b8ce2d6ce869e9662a564faee9ce4f9a2c3d783a390bdad`.
Pulse 38 is invalid, non-retryable, and null-conclusion.

Pulse 39 is a public infrastructure-only checkout-verifier release that
corrects the independently reproduced Pulse 38 cwd orchestration ambiguity
without retrying or altering its permanently invalid result. At the same
immutable cutoff `6807bd68aa01cbf0c819198765b7d6b5aa443328`, a disposable
Windows Git `2.55.0.windows.3` `core.autocrlf=true` checkout passed 36/36
root-anchored NUL-framed `text=set,eol=lf` checks, 36 LF files with zero CR
bytes, and the independent public Pulse 29 76/76 binding receipt. The
standard-library verifier permits only the two canonical release roots,
rejects unsafe paths and incomplete Git output, makes exactly 1
`git -C <checkout-root> check-attr -z --stdin text eol` invocation and exactly
1 separate root-anchored read-only Git version probe: 2 total Git processes, 0
retries, and no fallback check-attr form. Manifest raw/aggregate are
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`;
receipt raw/payload are
`sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8` /
`sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546`;
seal raw/payload are
`sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c` /
`sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b`.
It executes no FERRIS, diagnostic, build, preflight, seed/corpus
materialization, or private custody data, and creates no diagnostic, product,
fix, support, score, certification, or PLATFORM-001 authority.

Pulse 40 is a new governance/test-only independent authority at immutable
cutoff `65d1eec688f53bf7263ecfc8094ac849f9d3be4c`, not a Pulse 38 retry,
resume, reseed, reuse, correlation, or inference. Pulse 38 remains permanently
invalid, non-retryable, and null-conclusion. Before package copy, custody MUST
exactly copy and independently recompute the complete eight-file LF Git-clean Pulse 39 release tree, whose
manifest binds five payload files: manifest raw/aggregate
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`,
26455 payload bytes, all eight raw file bytes, report, receipt, and seal. Exactly one fresh below-root
`core.autocrlf=true` checkout then uses one root-anchored NUL-framed
check-attr invocation and one Git version probe: two Git processes, zero
retries, 36 expected/attribute/LF files, zero CR bytes, safe relative paths,
and retained 76/76 bindings. Declaration
`sha256:9ff14e5083ed4222f23e0ba68d945515225911633435b73c6c2fe4e6d9680a52`
has 9076 controls. Its sole execution passed authority and cutoff release
verification (`8/8` tree, `8/8` raw, `5/5` manifest payload bindings), copied
`8/8` files, then stopped permanently when the post-copy raw-binding
transaction completed `0/8`. No cutoff checkout, verifier process, later
gate, seed, materialization, candidate, or search occurred. Raw result
`sha256:b91ca8ed81a17ddcdb819044e2fa42be53a319a0dec71aaef2ca59b22f9352ca`;
receipt
`sha256:6e78c4e808c24c42f6dbe1df1565768b53a3f71549b82e65621c2e72f4e62237`.
Pulse 40 is invalid, non-retryable, and null-conclusion.

Pulse 41 releases public infrastructure only: a standard-library transactional
copier that binds the exact eight-file/31800-byte Pulse 39 release tree,
flushes and fsyncs every staged destination file before close, verifies
`8/8` source/stage/final files around exactly one rename and zero retries, and
records bottom-up aggregate staging-directory posture. A post-rename failure
is proven rolled back only after final-path absence and a `synced` or explicit
`unsupported` rollback-parent sync; otherwise it is indeterminate publication.
Pulse 40 remains permanently invalid, non-retryable, and null-conclusion at
`pulse-39-release-custody`. Its exact private cause is not provable; stale
stage references, duplicate/omitted roots, cwd/relative-root errors, and
pre-final-sync verification are bounded public classes only. No diagnostic,
private-custody, product, fix, score, or PLATFORM-001 authority is added.

Pulse 02 freezes only the `ferris.platform-profile/v1` controlled-fixture
schema, its canonicalization and projection boundary, one incomplete schema
exemplar, exact negative-control mutations, an exact RUNE fixture revision,
and the schema review. It adds no parser, harness, owner execution,
completed family, generation, product behavior, support, held-out access, or
PLATFORM-001 status change.

Pulse 03 authorizes only one dependency-free, test-only Rust integration
harness that executes the nine frozen schema controls, including duplicate,
size, schema-version, top-level-member, metadata, source-location, and state
checks. It does not authorize production schema types, owner commands,
completed families, generation, semantic decisions, or product behavior.

Pulse 04 authorizes only the controlled pure-data family: two zero-dependency
library revisions, locked/offline owner Cargo evidence in isolated target
directories, deterministic negative behavior, source immutability, and
test-only materialization of complete v1 profile values and digests. It does
not authorize production generation, other families, external dependencies,
native or provider claims, support, approval, or held-out access.

Pulse 05 authorizes only the controlled CLI/configuration family and reusable
integration-test support for deterministic family manifests, in-memory
profiles, digests, snapshots, and owner commands. The two zero-dependency CLI
revisions add explicit bounded config behavior only in `r2`. No production
parser, discovery, installation, mutation, other family, support, or held-out
authority is granted.

Pulse 06 completes the controlled in-process zero-dependency hosted-service
family with exact request, readiness, unavailable, cancellation, runtime, and
operational states. Windows and Ubuntu 24.04.4 WSL2 passed the Rust/Cargo
1.95.0 owner and workspace gates at cutoff
`de5b5242a26ed5ce15d1dae2d3ec333a3a7663d2`. It grants no socket, network,
database, TLS, credential, deployment, production operation, other family,
support, or held-out authority.

Pulse 07 completes two safe-Rust zero-dependency `#![no_std]` library
fixtures, host behavior tests, and exact `thumbv7em-none-eabi` compilation on
Windows and Ubuntu 24.04.4 WSL2 at cutoff
`ed214488aa19d025a9c9565dbe6db828b43582ac`. Device execution and
operational validation remain unavailable. The pulse grants no board, runner,
device I/O, interrupt, register, linker script, allocator, unsafe code,
firmware, flashing, deployment, hardware support, other family, or held-out
authority.

Pulse 08 completes two zero-dependency browser-WASM library fixtures, host
rendering tests, and exact `wasm32-unknown-unknown` compilation on Windows and
Ubuntu WSL2 at cutoff `dedd439fe1bb7eb693f1af4e3d851973ae44ae52`.
Browser execution remains unavailable. It grants no JavaScript binding, DOM,
browser runtime, automation, network, storage, bundling, publishing,
deployment, support, other family, or held-out authority.

Pulse 09 completes exact local WIT contracts, host semantic tests, and
`wasm32-wasip2` artifact compilation on both development hosts at cutoff
`f565270ac61d68bb18347bf0c05b5a0f49463a3f`. Runtime execution remains
unavailable. It grants no generated binding, component runtime, composition,
registry, network, deployment, support, another family, or held-out authority.

Pulse 10 completes minimal conditional FFI to Windows `kernel32` and Unix
libc process-identity APIs, safe wrappers, and exact system-native evidence on
both development hosts at cutoff
`41b7086cb43bc6b9a37b7ba5920cfdec39950f4a`. It grants no arbitrary FFI,
native discovery, package-manager control, bundled source, dynamic loading,
credential, deployment, support, other family, or held-out authority.

Pulse 11 completes bounded synthetic credential parsing, secret-redacted
values, and explicit non-security provider selection on both development
hosts at cutoff `3039cdb70247546ca8d53a0b318ecf2d81b778c3`. It grants no real
credential, authentication, authorization, TLS, cryptography, key storage,
network, external provider, deployment, support, other family, or held-out
authority.

Pulse 12 completes deterministic release records, actual Cargo package
construction and content inspection, deployment planning, and rollback
identity at cutoff `e60d67e`. It grants no signing, attestation, installation, deployment,
credential, remote-system, approval, support, other family, or held-out
authority.

Pulse 13 completes a test-only exact census across the nine controlled
families and eighteen revisions. It grants no merged-family semantics,
lifecycle action, product behavior, support, or held-out authority.

Pulse 14 completes one real renewal and exact rollback using only an isolated
temporary copy of the pure-data fixtures. It grants no committed-fixture
mutation, product lifecycle command, deployment, support, or held-out
authority.

Pulse 15 completes typed synthetic-provider substitution, emergency
containment, invalid-transition rejection, and exact provider rollback in
tests. It grants no production provider, credential, cryptography, incident,
deployment, support, or held-out authority.

Pulse 16 completes one explicit-marker adoption and complete removal in an
isolated ordinary Cargo consumer, plus one canonical Removal Record. It grants
no product adoption/removal command, committed-consumer mutation, registry,
deployment, support, or held-out authority.

Pulse 17 independent Stage A passed against public contract revision 3 at
cutoff `4371f4f6eb54097bff9badb29278c530d49e2f36` and froze the public
three-repository selection. A first Stage B custody attempt was invalidated
before execution by a CRLF-derived license digest and ran zero scored
processes. After the public binding was corrected, independent Stage B/C
completed against cutoff `8cbb5356fd7b3acca435bc9fad4e97dabab66bb5`
with fixture `P17-R3-D6B553CBC3B1240B673B8190`. Exactly 112 processes were
collected without collection-integrity or privacy failure, and all three
repository workflows passed. The valid first score failed only in the
public-safe category `process-exit-agreement`. This is a valid implementation
failure, not invalid custody or a held-out pass. The fixture is sealed in
quarantine and cannot be retried, rescored, or reused.

Pulse 18 completes the final review. It originally recorded the valid Pulse 17
failure and an unreconciled RUNE dependency as separate blockers. Pulse 21
later closes the RUNE blocker while preserving the exact fixture revision,
workspace `0.1.0`, controlled collection `v0`, neutral profile `v0`, and
no-tag facts. PLATFORM-001 remains Draft solely for the valid Pulse 17
failure.

Platform-profile Pulse 19 completed a test-only public diagnostic matrix with
23 JSON branches and three human-format pairs on Windows and Ubuntu 24.04.4
WSL2. Both platforms retained exactly 26 processes with zero retries and
reported `no-reproduction`: core classification, envelope construction,
actual OS exit, stream routing, and format parity agreed for every declared
public branch. This does not explain or overturn the valid Pulse 17 failure.
Pulse 19 grants no hidden-material access, retry, rescore, fixture reuse, CLI
behavior change, or fix. The old Pulse 17 fixture remains permanently closed,
and any fix requires a separately approved later product pulse.

Platform-profile Pulse 20 freezes a prospective opt-in
sanitized-reproducer protocol for future held-out programs. It preserves an
immutable original score while allowing an independent custodian to publish a
fresh bounded reproducer after scoring, subject to zero-overlap gates and
permanent retirement from certification. Pulse 17 cannot opt in retroactively.

Platform-profile Pulse 21 recognizes RUNE revision
`194449444624fb10add4137cb0da8d0327164fa7`, already bound by the controlled
semantic fixtures, as satisfying CONTRACT-001's Typebook/RUNE v1
contract-baseline dependency. RUNE v1 here means the accepted contract and
release-readiness baseline with eight accepted specification rows. The Cargo
workspace remains `0.1.0`, descriptor collection and neutral profile versions
remain `v0`, and no Git `v1.0.0` tag is present or claimed. The pulse changes
documentation, machine-readable evidence, review, and test-only validation
only. It does not alter RUNE, FERRIS production behavior, fixtures, Pulse 17,
Pulse 19, or Pulse 20.

Platform-profile Pulse 22 executed one fresh independent diagnostic
replication search for the released `process-exit-agreement` category at
cutoff `94890e53631d9110128bb420bf0cbbb074187e7c`. Its precommitted
`sanitized-reproducer` tier and private commitments remained intact. The
custodian generated 188 cases but a collector durability failure occurred
after one Windows process and before the Ubuntu partner launch. The result is
`invalid`: zero completed cross-platform pairs, zero retries, no
minimization, no reproducer, no receipt, and no category conclusion. Pulse 22
is not certification, a score, a product-fix pulse, or authority to access,
modify, retry, rescore, reuse, reconstruct, correlate, or infer Pulse 17.
PLATFORM-001 remains Draft solely for the immutable Pulse 17 failure.

Pulses 01 and 02 established local `plan`, `explain`, and declared `graph`.
Pulse 03 hardens their explicit portable workspace identity, invocation
identity, evidence representation, human output completeness, diagnostic
redaction, and JSON-mode CLI parse failures. Its applicable held-out fixtures
passed. Earlier held-out cutoffs remain historical evidence for their frozen
commits.

Pulse 04 authorizes only a passive `doctor` command that validates a portable
workspace identity, reads the explicitly selected manifest, and invokes
`cargo --version`. It does not invoke Cargo metadata or owner work. Windows
and Unix development gates passed; all 12 existing held-out fixtures were
independently classified out of scope and were not executed.

Pulse 05 corrects the Pulse 04 review findings. Cargo metadata and passive
doctor now use the same selected-manifest directory and inherited owner
toolchain context with offline, no-update, and no-auto-install guards. Doctor
adds a 1 MiB manifest bound, five-second process bound, 64 KiB per-stream
output bounds, owner-output-bound identities, and manifest-digest failure
identity after the manifest is read. Both applicable replacement held-out
owner-context fixtures passed. No dedicated passive-doctor fixture exists, so
no held-out doctor claim is made.

An independently designed blind doctor fixture, FHIF-013, then found a strict
Cargo-evidence and post-read identity gap at the Pulse 05 cutoff. It is now
development evidence and cannot be rescored. Pulse 06 tightens the canonical
Cargo version grammar, exposes safe commit/date evidence, and binds command,
working-directory, every resource bound, framing, and owner evidence into
doctor identity. A separately sealed replacement fixture is required.

The first replacement, FHIF-014, also failed its blind score and is now
development evidence. Pulse 07 removes manual doctor report identity field
lists by hashing the complete typed record, tightens canonical Cargo commit
and Gregorian release-date validation, and gives oversized manifests a
portable bounded-prefix selection identity. A new replacement ID is required.

Pulses 08 through 12 subsequently established unambiguous owner-output
framing, typed bounded-failure evidence, canonical command-result records,
explicit selection/invocation/result relationships, and typed stderr
envelopes for parsed and syntax failures. FHIF-026 was invalid because its
harness collected only 43 of 48 expected records.

Cardinality-safe replacement infrastructure then collected 48 of 48 records
for FHIF-027, which was invalidated by an independent scorer-layout defect.
After public-contract scorer correction, FHIF-028 collected and conformed all
48 records before producing a valid implementation failure in the public-safe
category `universal typed non-success coverage`. FHIF-027 and FHIF-028 are
quarantined permanently.

Pulse 13 is corrective only. It constructs command output before stream
emission, catches unwind-safe internal panics at the single-threaded CLI
boundary, suppresses default panic prose during guarded execution, emits a
typed internal result with exit 11, and converts failed success-output writes
to an internal process result.

FHIF-029 collected 48 of 48 records but was invalidated before oracle release
because success-output declarations were not carried into durable scorer
records. Repaired infrastructure passed a mixed 48-case preflight. FHIF-030
then collected and conformed 48 of 48 records and passed its sealed score
against immutable cutoff `15145eb24358a7d06db01bb0b7366d7899f310fa`.
Pulse 13 therefore has a valid held-out pass. FHIF-029 and FHIF-030 are
permanently quarantined.

Pulse 14 authorizes one local `profile-diff` command over two explicit
`ferris.profile-evidence/v0` experimental fixture files. It compares caller
evidence without invoking Cargo or owner tools, interpreting evidence states,
exposing raw section values, or establishing compatibility, support,
certification, or approval. Profile identifiers, revisions, consumers, and
JSON object keys are validated output-visible metadata and must not contain
secrets.

Pulse 15 adds synthetic before/after development fixtures for all nine
independent profile families required by Draft PLATFORM-001 and executes them
through the existing CLI. It demonstrates typed family-specific differences
and raw section-value redaction only. The fixtures are not owner observations,
canonical profiles, support statements, approvals, held-out evidence, or a
gate for advancing PLATFORM-001.

Pulse 16 freezes a public-safe `profile-diff` held-out contract requiring 56
independently constructed cases, Windows and Unix execution, 112 complete
process records, qualified collection and scoring, sealed privacy canaries,
one first score, and permanent quarantine after failure or invalidation. No
executable fixture or held-out claim exists yet.

Pulse 17 records Windows and Ubuntu 24.04.4 WSL2 development validation at
cutoff `f9305bdb5696da4889864b9c885ab4e18a56cdba` with Rust and Cargo
1.95.0. Both environments passed the workspace suite and nine-family CLI
matrix. This is development evidence only; it is not native Linux support and
does not satisfy either platform run in the independently sealed Pulse 16
program.

Pulse 18 adds a public-CLI integration test that proves all nine development
fixture pairs retain exact bytes, lengths, modification times, and directory
membership while an isolated working directory remains empty. Windows and
Ubuntu 24.04.4 WSL2 passed at cutoff
`ecb10e7ed82009e1a7cf46eb585f97e3769102b8`. The evidence is bounded
to those locations and is not a syscall, sandbox, network, ordinary-Cargo,
PRODUCT-001 removal, or held-out proof.

Pulse 19 adds one locked zero-dependency Cargo consumer control. Exact offline
Cargo metadata and one owner unit test pass before and after `profile-diff`,
separate external target directories prevent cache dependence, and the
consumer workspace remains unchanged. Windows and Ubuntu 24.04.4 WSL2 passed
at cutoff `e1b9e9d427b8bfcca7f21ce7f177fd31d6cf8960`. This is not
universal Cargo, adoption, removal, or held-out evidence.

## Product boundary

Ferris owns the global application plan, policy, approval, explanation,
evidence, lifecycle, and cross-workspace coordination.

Cargo remains authoritative for:

- package sources and dependency resolution;
- lock state;
- workspace membership;
- targets, features, profiles, and platform conditions;
- build-unit construction and freshness; and
- compiler invocation and local scheduling.

Ferris does not replace Cargo, rustc, linkers, test runners, Typebook/RUNE,
native tools, deployment systems, or their owner-local semantics.

The governing rule is:

> **The plan is global; the work is local.**

## Names

- **Ferris** is the public product.
- `ferris` is the primary command.
- `cargo ferris` is the Cargo external-subcommand entrypoint.
- **Blueprint** is the internal application model and planning engine.
- **Query Forest** is the canonical typed evidence model and immutable-root
  history.
- **Typebook/RUNE** is a separate, product-neutral semantic-contract system.

Public documentation should say Ferris unless it specifically means the
Blueprint Model or Blueprint Plan.

## Program architecture

The work is divided into seven bounded programs:

1. Ferris - public command, governance, approvals, lifecycle, and evidence.
2. Typebook - product-neutral semantic contracts.
3. Profiles - exact, renewable compatibility and support commitments.
4. Blueprint - normalized application model and non-executable planning.
5. Query Forest - scope, evidence, identity, causality, roots, refs, and
   history.
6. Conformance - executable positive, negative, failure, unsupported,
   version-skew, rollback, and removal proof.
7. Ecosystem Bridge - owner-aligned adapters, connectors, MCP, and upstream
   contribution packets.

Microsoft services are replaceable connectors, never canonical dependencies.

## Authority and gates

The canonical specification sequence is in
[`docs/specs/README.md`](docs/specs/README.md). A Draft, Proposed, or Adopted
specification does not by itself authorize implementation.

Implementation requires:

1. completed research dependencies;
2. a complete normative specification;
3. all applicable `.roles` reviews;
4. measurable acceptance and stop criteria;
5. adoption, support, removal, rollback, and maintenance plans; and
6. a separately approved implementation pulse.

Until that gate exists, work is limited to research, plans, specifications,
fixtures, and review records that do not create product code or hidden runtime
commitments.

## Core invariants

- Keep observation, normalization, projection, identity, prediction,
  resolution, execution, validation, outcome, and evidence responsibilities
  distinct.
- Keep Application Definition, Blueprint Model, Blueprint Plan, approved
  Action Plan, and FERRIS Application Contract as separate records.
- Keep Rust source, semantic, ABI, component, wire/data, and projection
  identities separate.
- Scope is multi-dimensional; package, target, activity, compilation, runtime,
  validation, contract, native, platform, lifecycle, and evidence scope must
  not collapse into one hierarchy or Boolean.
- Unknown mappings widen to the smallest safe owner boundary.
- AI may propose plans, mappings, and explanations but cannot establish owner
  truth, remove mandatory work, approve policy, or execute actions.
- Query Forest roots are immutable.
- Branches, tags, channels, aliases, pins, leases, tombstones, and labels have
  distinct semantics.
- Refs never prove compatibility, integrity, trust, validation, availability,
  or reuse.
- Credentials and reusable secrets never enter plans, prompts, roots, refs,
  logs, or durable evidence.
- Ordinary Cargo and owner-system workflows must remain functional after
  Ferris removal.

## Review model

Use the nine repository roles for architecture, specification, and gate
reviews:

- Rust Safety Steward;
- Compiler Performance Engineer;
- Interop Boundary Auditor;
- AI Assurance Skeptic;
- Ecosystem Strategist;
- Rust Maintainer;
- Native Platform Adopter;
- Scope Keeper; and
- Validation Checker.

Role files live under `.roles/`. Reviews record each role's disposition,
required revisions, remaining blockers, and whether implementation is
authorized.

## Key files

- [`README.md`](README.md) - public product and research overview.
- [`docs/plans/FERRIS_PROGRAM.md`](docs/plans/FERRIS_PROGRAM.md) - governing
  product, commands, sequencing, and implementation gates.
- [`docs/plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md`](docs/plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)
  - program ownership and cross-program contracts.
- [`docs/plans/BLUEPRINT_PROGRAM.md`](docs/plans/BLUEPRINT_PROGRAM.md) -
  internal model and planning architecture.
- [`docs/specs/README.md`](docs/specs/README.md) - canonical specification
  registry and review status.
- [`docs/specs/FOREST_COMPONENT_MODEL.md`](docs/specs/FOREST_COMPONENT_MODEL.md)
  - Query Forest component boundaries.
- [`docs/simulations/README.md`](docs/simulations/README.md) - no-code
  specification simulation waves, issues, and change records.
- [`docs/research/questions/README.md`](docs/research/questions/README.md) -
  research-question registry.
- `.roles/` - review responsibilities and stakeholder perspectives.

## Repository workflow

- Make Ferris research, specification, fixture, and future implementation
  commits in this repository.
- Keep Ferris commits separate from TRACKER submodule-pointer updates.
- Commit and push Ferris work before an explicit TRACKER portfolio snapshot.
- Do not push unless requested.
- Do not amend commits unless explicitly requested.
- Do not rewrite or remove historical `FERRIUM-*` finding identifiers; new
  findings use `FERRIS-*`.

## Validation

For documentation and specification changes:

- check local Markdown links;
- check balanced code fences;
- run `git diff --check`;
- inspect the specification dependency graph for cycles;
- confirm all nine role dispositions where a review gate is claimed; and
- stage only files belonging to the current logical change.

Implementation validation commands will be added only when an implementation
pulse authorizes product code.

## Pulse 42 public-result integrity closure

[Pulse 42](docs/simulations/profile-diff-held-out/pulse-42-public-result/README.md)
is permanently `invalid-publication-integrity`, non-retryable, and
null-conclusion at `public-result-publication`. Its historical authority cutoff
is `2a8b7c27ac465ab78a8ec7ca331b9e427a8625c8` and declaration identity is
`sha256:4da4d749892a487e30467b68bf8e35e9f72655dfb3a75414ead10ff40e0868cc`.
Pulse 38 and Pulse 40 remain unchanged permanent invalid/null-conclusion
predecessors. Historical public bindings remain Pulse 41
`sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8`
and Pulse 39
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c`.
The expected custodian result files were absent (`1`) and claimed result paths
observed were `0`. The public summary's claimed Pulse 33 stop is inconsistent
with its later ordered reported quantities under the committed stop-on-failure
authority. Those facts are `reported_unvalidated`; they do not establish
`P42-FROZEN-BINARY-UNAVAILABLE`, execution of any later gate, a diagnostic or
product conclusion, or fix authority.

## Pulse 43 ordered public-result publisher release

[Pulse 43](docs/simulations/profile-diff-held-out/pulse-43-ordered-result-publisher-release/README.md)
releases only public standard-library result-publication infrastructure. Its
closed catalog and explicit event records distinguish
`public-artifact-self-validation` from `ordered-execution`; self-validation
counters cannot advance ordered gates. Execution consumes its predeclared
catalog once and in order with one `terminal-stop`, rejecting the
Pulse42-shaped early Pulse 33 stop followed by later Pulse 31 or Pulse 35
execution records. It stages, file-fsyncs, canonicalizes, hashes, verifies,
renames exactly once, final-verifies, and parent-syncs a two-file result
directory before returning `published`. Failure posture is only `absent`,
`rolled-back`, or `indeterminate`; retries and fallbacks are zero.

The sealed manifest raw/aggregate identities are
`sha256:8eaca6ebc350a67e493d037132a27749980c16fc79143d69fd0303305b5030a4` /
`sha256:74f6c61913fbfa638f0ba6aae19cb0d2885a47e38b33b19bb80bec1abc870346`.
The qualification receipt raw/payload identities are
`sha256:3ebc1bfd95dfbfedd1402bb3f3f9f14ea872aec9137a7327b8ca444248091e0c` /
`sha256:9e713bb8f12deced2119fe66028a4c2ab11d6d70d6d0fe90342b996bc1bf25a2`;
the release seal raw/payload identities are
`sha256:4445a0b181419b303c28f1d91e1700a594d2a040fbae1cd0dc526fa7fc8e8f05` /
`sha256:b51b408cb7e93fccc3c4f92e1d29956f3c107e500a4e947548273ca01ea525a1`.
Windows directory synchronization is honestly `unsupported` with
`os.open+os.fsync-directory-v1`; it is not a durability claim. Pulse 43
accesses no private diagnostic data and grants no diagnostic, custody,
product, category, or fix conclusion.

## Pulse 44 retained-binary custody release

[Pulse 44](docs/simulations/profile-diff-held-out/pulse-44-retained-binary-custody-release/README.md)
is public retained-binary custody infrastructure only. It pins the immutable
Pulse 33 manifest, aggregate, build adapter, and cutoff before one
retention-enabled build-freeze call for one platform. Fresh absent absolute
work and final roots, exact `2/2` executable/receipt verification, file fsync,
honest stage/final-parent sync, one rename, final path reconstruction, and
rollback posture prevent a successful-looking partial custody result. A
Pulse-43-compatible ordered event is completed only after final `2/2`
verification; every failure is terminal and `absent`, `rolled-back`, or
`indeterminate`. The controlled non-public final custody tree is never
committed. Windows qualification first rejected a dirty clone created before
`core.autocrlf=false` was fixed, then independently passed from a clone
normalized before checkout: final executable/receipt `2/2`, one rename, zero
retries, size `1436672`, and Pulse 33 SHA-256
`sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`.
All runtime roots were removed afterward. This remains infrastructure basis
only, not diagnostic, product, category, or fix evidence.

## Pulse 45 binary-custody event bridge release

[Pulse 45](docs/simulations/profile-diff-held-out/pulse-45-binary-custody-event-bridge-release/README.md)
is public ordered-ledger composition infrastructure only. It pins the exact
sealed Pulse 44 manifest, receipt, seal, and adapter source identities before
the CLI imports and invokes `retain_binary_custody` once with its unchanged
arguments. Only `windows-x86_64` and `ubuntu-24.04-x86_64` map to distinct
stable Pulse 43 gates. It validates the entire closed Pulse 44 summary:
published output requires final-files presence, work/stage/final `2/2`,
one rename, zero retries, and the exact Pulse 44
`retained-binary-custody` `terminal-stop/completed` event before translation
to `gate-complete/passed`. Closed Pulse 44 failures preserve their
`absent`, `rolled-back`, or `indeterminate` posture as platform
`terminal-stop/failed`; malformed or thrown output is a bounded Pulse 45
terminal failure. Its deterministic public result exposes no filesystem path,
private data, or executable bytes and has zero retry/fallback. This creates
no diagnostic, custody, product, category, or fix authority.

## Pulse 46 publication-order diagnostic authority

Pulse 46 authority was approved at immutable
cutoff `22ea38e274b882d6e607810382f842b76e483f10`; it is not a Pulse 42
retry, resume, or reconstruction. It preserves Pulse 42's permanent
invalid-publication/null closure and the valid immutable Pulse 17 score
baseline. Before any ordered execution, fresh immutable cutoff custody
verifies full exact Pulse 41/Pulse 39/Pulse 43/Pulse 44/Pulse 45 trees and
their sealed manifest, receipt, seal, and raw-file bindings. It fixes the
eight-gate catalog from Pulse 41/Pulse 39 custody through sole terminal
`bounded-process-exit-search`, keeps public self-validation nonadvancing, and
makes later ordered counts indeterminate after a stop.

Each platform checkout fixes `core.autocrlf=false` before checkout, invokes
Pulse 45 once, and permits its one Pulse 44 invocation only to translate a
complete controlled retained `2/2` root into a platform
`gate-complete/passed`; a platform failure is terminal. Only after both pass
do inherited preflight, Pulse 31 `39/39`, Pulse 35/Pulse 37 normalization,
fresh private 32-byte materialization, and one 70-per-platform/140-total,
zero-retry/fallback search proceed. Pulse 43 publishes once at terminal
disposition to a fresh absent absolute result root; absent, rolled-back, or
indeterminate publication is `invalid-publication/null` and may expose only
that posture. The declaration is
`sha256:92847e645338fd142710c1afcff5d6ad5540c35e6322ccf59b574f2fd3d61534`;
its pre-launch execution state is zero.

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
[public closeout](docs/simulations/profile-diff-held-out/pulse-46-public-result/README.md)
is the sole public record.

## Pulse 47 publication-outcome witness release

[Pulse 47](docs/simulations/profile-diff-held-out/pulse-47-publication-outcome-witness-release/README.md)
is a public standard-library-only wrapper around the exact sealed Pulse 43
publisher. It verifies the exact Pulse 43 manifest, receipt, seal, and source
identities, invokes the publisher once through an injected or verified real
callable, validates the complete returned closed summary, then creates a
separate persistent public witness. A published Pulse 43 result exposes only
public result hashes, final `2/2`, rename/retry/sync posture, and
ordered/self-validation aggregates. A Pulse 43 failure exposes only its
failure code, absent/rolled-back/indeterminate posture, final-files flag,
rename/retry values, and exact stage/final-parent/rollback-parent sync
postures.

Pulse 47 stages, file-fsyncs, verifies, directory-syncs, renames once,
re-opens/re-hashes, and parent-syncs exactly
`publication-witness.json` and `release-receipt.json`; it has zero retry or
fallback. Any witness-publication failure returns only its bounded
absent/rolled-back/indeterminate witness posture and code, with no captured
Pulse 43 material. It neither records, retries, resumes, reconstructs, nor
infers Pulse 46, accesses no private diagnostic data, and grants no
diagnostic, custody, product, category, score, certification, support, or fix
authority.

## Pulse 48 permanent public closeout

[Pulse 48](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_48_AUTHORITY.md)
is permanently `invalid-publication-integrity`, non-retryable, and
null-conclusion. Authority commit `5a8d92d211806d0f2940016af6c317878c5fdfc1`,
cutoff `70c8fc2dfa60b6732fa265bb5fcf6326ac97ad2d`, and declaration
`sha256:6c014c640d9184d458a7e750922399fd82fe10eb070b6cf7a4ee8ce409ee5d3e`
bind its sole launch. Blocker `P48-P43-CATALOG-PRIVACY-IDENTIFIER` is at
`public-result-publication`.

The public Pulse 43 result root is absent. The retained Pulse 47 witness root
contains exactly `publication-witness.json` and `release-receipt.json`, with
`P43-PRIVACY-BEARING-IDENTIFIER`, absent publication, zero rename attempts and
retries, and every sync posture `not-attempted`. Public reproduction against
exact Pulse 43 rejects the committed catalog because
`private-materialization` contains forbidden identifier part `private`.

This establishes only public catalog/publisher incompatibility, not whether or
how far private execution progressed. Category, diagnostic, and product
conclusions are null; no fix authority, private data/gate/search inference,
or rerun exists. A future redesign may use neutral
`bounded-materialization`, but no new authority is created here.

## Pulse 49 withdrawn prelaunch authority

[Pulse 49](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_49_AUTHORITY.md)
is permanently `invalid-prelaunch-authority-integrity`, non-retryable, and
null-conclusion at `prelaunch-authority-validation`, blocker
`P49-P35-CASE-PROCESS-CARDINALITY-CONFLICT`. Authority commit
`80f78fa4edb2d3497a830b2879ea9ff9c6f0aea5` is withdrawn before execution.

The exact Pulse 35 public materializer yields 70 descriptors per platform,
not 70 processes: 69 are `launch-ready` process cases and one final
`no-launch` descriptor has `not-materialized` before/after states and
`external-immutable-binary-freeze`. The historical assertion of 70 processes
per platform and 140 total cannot honor that exact corpus.

No launch, P47/P43 invocation, private operation/data/artifact, result or
witness root, runtime/public-root transfer, or inference exists. The
historical declaration, schema, and mutations remain exact. A future successor
requires fresh authority for 70 case dispositions, 69 processes, and one
no-launch disposition per platform: 140 cases, 138 processes, and two
no-launch dispositions total.

## Pulse 50 withdrawn prelaunch authority

[Pulse 50](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-50.md)
historical authority commit
`48fe9fdcdda03378f68781cae342796c9f11720d` at cutoff
`94d473563a1686091be94a72f491b0ff0d903800` is permanently
`invalid-prelaunch-infrastructure-integrity`, non-retryable, and
null-conclusion. The blocker is
`P50-EXECUTOR-RELEASE-OUTSIDE-AUTHORITY-CUTOFF` at
`prelaunch-public-infrastructure`.

Independent custodians stopped before launch after the public audit found that
the sealed executor was missing from the authority/cutoff: Pulse 35's CRLF/LF
schema binding, descriptor/69+1/P43 runner, Pulse 27 CLI seam, Pulse 31
schema-count consistency, WSL/canonical Ubuntu mapping, exact Ubuntu Pulse 33
toolchain/hash custody, and Python resolver detail. There was no diagnostic
execution, private material, seed, descriptor, candidate process, P43/P47
invocation, result root, witness root, or inference; all execution values are
zero or false.

## Pulse 51 public diagnostic-executor release

[Pulse 51](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-51.md)
closes public prelaunch infrastructure blockers only. It adds a synthetic-only
standard-library executor release with full P35 release-tree and machine-schema
Git-clean custody, frozen P31 exact-contract validation, exact one-call P27
cleanup/retention binding, independently enforced P33
binary/receipt/toolchain identities, sealed P44-to-P45 custody bridging,
native Windows plus exact `Ubuntu-24.04` WSL dispatch, full frozen output
validation with independently recomputed profile-diff semantics and
identities, canonical Windows/Ubuntu IDs, sealed-tree-clean qualification,
and sealed P43/P45/P47 imports.

The release creates no Pulse 50/Pulse 51 authority or execution. It does not
create private seeds/descriptors, invoke a private candidate, or create a
result/witness root. Its exact release commit is
`d09c923c1e2cd2be003026597f4ad2a0e2d3764f`, a direct post-authority child and
therefore outside Pulse 50's cutoff. Its sealed identities qualify public
infrastructure only and cannot cure Pulse 50.

A future successor requires fresh authority binding exact Pulse 51, exact
Pulse 52, and the existing public releases.  It must use Pulse 52's production
ordered-materialization API, which reuses the one-use Pulse 51
`TerminalPulse47Once` seam. It cannot consume or revive withdrawn Pulse 50
authority.

## Pulse 52 ordered-materialization executor release

[Pulse 52](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-52.md)
seals the prospective ordering adapter around exact Pulse 51 and exact Pulse
35 source.  It binds exact Pulse 39/Pulse 41 source, manifests, receipts,
seals, and callable signatures; receives a concrete fresh P39 checkout root,
fresh P41 final-copy root, and P44 custody inputs; invokes and rechecks P39
and P41 before internally constructing gate 1; executes gates 1–6 once with
no private namespace; then and only then creates one 32-byte
`secrets.token_bytes(32)` seed through `O_EXCL`/`fsync`, invokes exact P35
materialization and verification once, cleans private material, and performs
the fixed Pulse 51 `70/69/1` Windows/WSL dispatch plus a fresh one-use P47
seam.

Twenty fake-only cycles qualified one P39 verification, one P41 copy/final
reverification, one P27 call, one P35 materializer, one P35 verifier, 138 fake
dispatches, 2,760 total dispatches, and bounded cleanup per cycle.  Permanent
terminal cleanup uncertainty raises only the public-safe unresolved
`terminal-publication-cleanup-indeterminate` posture rather than returning a
completed closeout.  Public results remain exact P43 catalog/events; seed
values, commitments, paths, descriptors, tokens, binaries, and private records
are not public.  This is infrastructure only.  It creates no authority,
diagnostic, score, candidate, product result, certification, fix, or
PLATFORM-001 conclusion.  Any future authority must bind exact Pulse 51 and
Pulse 52 and cannot revive withdrawn Pulse 50 authority.

## Pulse 53 witness-preserving ordered executor release

[Pulse 53](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-53.md)
is sealed synthetic closeout infrastructure.  It binds exact Pulse 51 commit
`d09c923c1e2cd2be003026597f4ad2a0e2d3764f` and exact Pulse 52 commit
`e4ef9617f227670f3911be42ca63df4b2e66d24f`, verifies Pulse 52's complete
release tree/source/signatures, and uses Pulse 52's own exact Pulse 51 loader.
It copies only bounded orchestration/terminal classification; P39/P41 custody,
public gates, CSPRNG/P35 materialization, private cleanup, fixed P51 dispatch,
and error boundaries remain exact Pulse 52 behavior.

After one P47-to-P43 call, it retains complete independently verified P43/P47
`2/2` roots as `published-result`.  It also retains a complete independently
verified P47 `2/2` witness as `published-failure-witness` when the captured P43
publication is an exact bounded `absent`, `rolled-back`, or `indeterminate`
failure and P43 root/stage residue is absent.  This permanent public closeout
has null product/category/fix conclusions and a path-free descriptor containing
only expected tree kind, exact file counts, and verified raw/payload hashes.
P47 failure, malformed/unverifiable/hashing-mismatched output, P43 residue, or
missing final shape is `invalid-witness-publication`: no retry/republication,
only exact bounded verified cleanup, with unresolved cleanup raising the
public-safe `terminal-publication-cleanup-indeterminate` fatal posture.
Twenty fake-only alternating cycles retained ten result and ten failure-witness
closeouts at 2,760 fake dispatches; no FERRIS binary or authority was used.
Any future authority must bind exact Pulse 51, Pulse 52, and Pulse 53 and
cannot consume or revive withdrawn Pulse 50 authority.

## Pulse 54 independent witness-preserving diagnostic authority

[Pulse 54](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-54.md)
was published as `authorized-unexecuted` authority at exact self-excluding cutoff
`42a16e298c5af55b05df5ceb8e3477d0dd45c814`. It binds complete public
P27/P31/P33/P35/P37/P39/P41/P43/P44/P45/P47/P51/P52/P53 release identities,
including path/hash/manifest/receipt/seal and source/API/signature binding.
Pulse 48 remains permanent invalid/null; Pulse 49 and Pulse 50 remain
permanently withdrawn invalid/null. Pulse 54 is not their retry, resume,
reconstruction, reseed, reuse, correlation, or inference.

Before its sole unperformed P53 production call, independent custody must make
fresh anonymous exact-cutoff `core.autocrlf=false` checkouts, recreate exact
P33 Windows `/Brepro` and `Ubuntu-24.04` WSL binary freezes, run P44 once per
platform, provide fresh P39/P41/runtime/P27 roots, and prevalidate every P43
gate and validation ID. No seed, descriptor, candidate, runtime, result, or
witness artifact may exist first. P53 then fixes P39/P41; Windows P44/P45;
Ubuntu P44/P45; P27; P31; P35/P37; one 32-byte CSPRNG seed/materializer/verifier;
`70/69/1` per platform; `140/138/2` total; first mismatch stop; and one
P47-to-P43 terminal route. No direct P51/P52/P47/P43 path, retry, fallback,
or republication is authorized.

The only public transfer is P43 `2/2` plus P47 `2/2` to distinct Pulse 54
destinations for `published-result`, or exactly P47 `2/2` alone with an absent
P43 result destination for `published-failure-witness`. The latter is a
permanent null-conclusion publication-integrity closeout. Invalid or cleanup
indeterminacy permits no success claim or tree copy. The authority records
zero launch/publication counters and null conclusions.

Independent pre-call custody stopped before Pulse 33 or Pulse 44 work because
the required validator compared a CRLF Pulse 35 working-tree identity against
canonical LF bytes in the mandated `core.autocrlf=false` checkout. No P53 call
or private/runtime artifact occurred. Pulse 54 is permanently withdrawn under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; it is non-retryable and cannot be
amended or resumed. A successor must use a fresh cutoff and
checkout-materialization-independent variant validation.

## Pulse 55 immutable-blob successor authority

[Pulse 55](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-55.md)
was published as a fresh `authorized-unexecuted` authority at immutable
self-excluding cutoff `47113e444ef3309afec9a844f0cba62775f19f6f`. It binds
the unchanged exact P27/P31/P33/P35/P37/P39/P41/P43/P44/P45/P47/P51/P52/P53
chain and the sole
one-shot P53 `run_witness_preserving_ordered_executor` callable. It is neither
a retry nor amendment of permanently withdrawn Pulse 54.

Pulse 55 corrects only authority validation: every canonical release-tree and
callable identity is derived from cutoff Git blobs, while a working checkout is
validated only against those identities or explicitly sealed complete-file
CRLF/LF variants. Pulse 35 keeps its Pulse 37-normalized canonical LF
identities and exact Pulse 51 custody variants with size/newline framing; a
fresh anonymous `core.autocrlf=false` checkout remains the mandatory runtime
posture. No generic alternate-hash rule is authorized.

Pulse 54 remains permanently withdrawn
`invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`, with zero calls, artifacts, and
conclusions. It remains distinct from Pulse 55's consumed one-call closeout.

At authority commit `36b3ac6b9692924af57c7c98b0a976835fe778f6`, fresh P44
custody occurred once per platform before the one and only P53 call. That call
returned public publication disposition/classification `not-attempted` at
`pulse-41-pulse-39-public-custody`: zero completed gates, seed, descriptors,
processes, no-launch dispositions, P27/P39/P41/materializer/verifier/P47
calls, or result/witness transfers; all conclusions are null. Pulse 55 is
permanently `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`, non-retryable, and non-resumable.

The Windows retained artifact hash/size differed from P51's exact expectation;
Ubuntu matched its expected artifact hash/size, but both fresh retained
receipt payload identities differed from P51's published non-retaining receipt
identities. The structural issue is that `build_freeze`
`retained_in_public_bundle` changes when the executable is retained, so an
exact published receipt payload identity cannot equal a real retained-custody
receipt. Windows `/Brepro` plus Rust/Cargo version alone also left the
linker/SDK environment underbound. A successor needs a new sealed executor
chain binding a corrected P33 retained-build/custody contract, a fully bound
Windows linker/SDK environment or qualified deterministic linker route,
semantic retained-artifact receipt verification, and replacement
ordered/witness layers bound to the corrected diagnostic executor. No
replacement implementation is authorized here. See the
[Pulse 55 execution closeout](docs/simulations/profile-diff-held-out/PULSE_55_EXECUTION_RECORD.md).

Pulses 46/48/49/50 retain their prior permanent dispositions. Pulse 55's
immutable historical declaration identity is
`sha256:45ac35775c34e8a86fdc90ad1554104f2728a676d51ab46125bfcf126db21655`;
its `19261` controls bring the monotonic registry total to `100582`.
## Pulse 56 retained deterministic build and custody release

[Pulse 56](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-56.md)
is sealed foundational infrastructure only, not a diagnostic executor. It
internally makes two fresh clean `core.autocrlf=false` checkout/target builds
at `29517d732db13cc2ffa304684b344f3538ab587d`, requires byte identity, creates
a new semantic retained receipt, and publishes exactly the binary/receipt pair
through one staged rename with zero retries. Windows binds toolchain-shipped
rust-lld and deterministic controls; Ubuntu WSL binds its measured cc/GNU-ld
route after shipped rust-lld could not resolve system libraries. Both current
platform probes passed distinct-root reproducibility. The public receipt and custody root are auditable evidence only, never launch
authorization. A future launch requires the exact live private
object-identity `CustodyHandle`, whose bounded use count and verified bytes are
held in-process. No FERRIS binary, diagnostic, authority, or conclusion is
executed or created; Pulse 55 remains closed.

## Pulse 57 capability-bound diagnostic executor successor

Pulse 57 is a sealed infrastructure successor that binds the complete exact
Pulse 51 diagnostic semantics and complete exact Pulse 56 retained
build/custody release. Its one production callable accepts no receipt, custody
root, binary, executable path, process runner, callback, environment, or
arbitrary control. It creates one opaque Windows capability and one opaque
native-WSL Ubuntu capability internally, retains 70/69/1 and 140/138/2
accounting, validates full frozen profile-diff output identities, and stops on
the first semantic mismatch. The old Pulse 44/Pulse 45 caller-summary route is
not invoked or claimed; P43-safe gates instead name only performed
sealed-predecessor, capability, P27/P31/P35/P37, descriptor, and bounded
process controls. P57 neither executes nor infers P39/P41. A future ordered
layer must add those controls before private materialization. Qualification is
fake-only: 20 cycles and 2,760 fake launches plus 13 negative controls, with
no FERRIS execution, authority, seed, materialization, result, witness,
product, category, score, certification, support, fix, or PLATFORM-001
conclusion.

## Pulse 58 ordered capability/materialization executor successor

[Pulse 58](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-58.md)
seals infrastructure that orders exact P39/P41 custody and all public P57
gates before one private P35 materialization, then applies exact P57 semantics
to already-live Pulse 56 capabilities. Qualification is fake-only (20 cycles,
2,760 launches); it creates no authority, real FERRIS execution, result, or
PLATFORM-001 conclusion. A future authority must supply the fresh anonymous
exact-cutoff P39 checkout and independently establish its HEAD, clean-tree,
and `core.autocrlf` posture; P58 invokes exact P39 semantics only.

## Pulse 59 witness-preserving capability/materialization executor

[Pulse 59](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-59.md)
binds exact Pulse 58's six-input production surface and exact fake-only
qualification orchestration rather than reimplementing ordering. After exact
Pulse 58 completes and removes its private runtime root, Pulse 59 derives a
fresh sibling terminal custody root and executes one exact Pulse 51/Pulse 47
terminal route with no post-completion execution event. It preserves
`published-result`, `published-failure-witness`, and
`invalid-witness-publication`; pre-execution Pulse 58 failure remains
`not-attempted`. Qualification is fake-only: 20 cycles, 2,760 launches, all
three bounded Pulse 43 failure postures, 14 behavioral controls, and no real
FERRIS execution.

## Pulse 60 witnessed capability/materialization diagnostic authority

[Pulse 60](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-60.md)
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

[Pulse 61](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-61.md)
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

[Pulse 68](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-68.md)
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

[Pulse 67](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-67.md)
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

[Pulse 66](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-66.md)
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

[Pulse 65](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-65.md)
is now the withdrawn historical prelaunch authority at immutable cutoff
`e3b0b62f6dd62b5071886d32a9eedca85c76b4ae`; declaration identity
`sha256:5bd7c876180a3bfb9f0bcb1518ef68921d1b28210d1f717c904753508e28abb0`.
Independent prelaunch review withdrew it under
`P65-P57-WSL-TWO-SPAWN-CONTRACT` because its declared exact WSL preflight
still collapsed one harmless bounded WSL spawn into proof of both Pulse
57 stage-bundle and worker bootstrap even though exact source uses
separate `subprocess.run` and `subprocess.Popen` spawns with distinct
ready/close cleanup semantics. No authority callable or diagnostic ran,
so calls, seeds, descriptors, processes, publications, transfers, and
all conclusions remain zero or null. The historical closed schema and
mutation registry remain unchanged at `25815` controls; Pulses 66 and 67 later
raised the monotonic total to `290502`.

## Pulse 64 witnessed capability/materialization diagnostic authority

[Pulse 64](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-64.md)
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

[Pulse 63](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-63.md)
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

[Pulse 62](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-62.md)
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
