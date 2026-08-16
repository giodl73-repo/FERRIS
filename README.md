# Ferris

**The cross-workspace enterprise build system for Rust.**

FERRIS, formerly FERRIUM, is a research and engineering platform for the
unfinished parts of enterprise Rust: supported crate profiles, versioned
contracts, compiler-grounded AI assistance, fast builds, trustworthy language
boundaries, supply-chain assurance, concurrency observability, and portable
native execution.

Historical `FERRIUM-*` findings remain stable citation identifiers. New
findings use `FERRIS-*`.

## Enterprise platform architecture

FERRIS combines five replaceable layers:

1. idiomatic Rust crate APIs and Cargo SemVer;
2. [RUNE](https://github.com/giodl73-repo/RUNE) semantic descriptors,
   registries, compatibility reports, profiles, and adapters;
3. explicit C ABI, WIT/component, or wire-schema contracts at independently
   versioned boundaries;
4. renewable enterprise crate profiles with support, security, platform,
   stewardship, renewal, removal, and rollback evidence; and
5. Ferris application modeling plus dependency, build, validation, and change
   intelligence.

RUNE remains a product-neutral standards repository. FERRIS consumes and
contributes to it rather than copying it into this repository.
FERRIS recognizes exact RUNE revision
`194449444624fb10add4137cb0da8d0327164fa7` as the accepted RUNE v1 contract
and release-readiness baseline. This is not a Cargo SemVer `1.0.0` or Git v1
tag claim; the RUNE workspace remains `0.1.0`, and the controlled collection
and neutral profile remain `v0`.

## Ferris

Ferris is a Cargo-native cross-workspace build and application-control system.
One semantic engine has two entrypoints:

```console
ferris
cargo ferris
```

`ferris` exposes complete application, repository, multi-workspace, contract,
profile, policy, CI, deployment, root, and ref scope. `cargo ferris`, provided
by `cargo-ferris`, defaults to the current Cargo workspace through Cargo's
external-subcommand convention.

Ferris defines the missing application layer above Cargo packages and
workspaces. Blueprint is its internal normalized model and planning engine:

```text
Cargo graph truth
  + application definition
  + RUNE contracts
  + platform and support profile
  + validation and lifecycle evidence
  -> FERRIS Application Contract
```

For each proposed change, Blueprint may generate a non-executable **Blueprint
Plan**: a dynamic, application-level DAG that composes the affected Cargo,
compiler, contract, native, link, validation, cache, and resource closures.
The plan is global; the work is local. Cargo and every other owner retain their
own resolver, graph, freshness, scheduling, and execution rules.

Blueprint scope is a coordinate set rather than one tree. Package, target,
activity, feature, profile, platform, compilation, runtime test, validation,
contract, service, native, deployment, lifecycle, and evidence scopes remain
distinct and are joined through typed mappings. AI may propose finer scopes,
but deterministic policy controls narrowing and unknowns widen safely.

Cargo remains authoritative for packages, targets, features, sources, and
resolution. Blueprint adds consumer-owned application intent, component and
service relationships, contracts, providers, platforms, validation, support,
renewal, removal, and rollback. The Query Forest remains its internal evidence
model.

Query Forest roots are immutable. Blueprint uses typed branches, write-once
tags, promotion channels, local aliases, and retention pins to navigate them;
leases and tombstones are policy records, while labels are metadata only.
These refs support compare, promotion, rollback, and retention but are never
cache keys or correctness evidence.

See the [Ferris program](docs/plans/FERRIS_PROGRAM.md),
[seven-program architecture](docs/plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md),
[Microsoft enterprise integration](docs/plans/FERRIS_MICROSOFT_ENTERPRISE_INTEGRATION.md),
[enterprise Rust application-platform plan](docs/plans/ENTERPRISE_RUST_APPLICATION_PLATFORM.md)
and
[Rust contract and interface strategy](docs/research/2026-08-10-rust-contract-interface-strategy.md).

## Initial research lanes

| Lane | Question |
|---|---|
| Boundary | How can Rust enter C and C++ systems without weakening safety at the boundary? |
| Hammer | How can build causality, caching, linking, and workspace structure reduce iteration time? |
| Temper | How can generated native code carry auditable safety, provenance, and compliance evidence? |
| Lens | How can async and concurrent Rust become easier to observe, explain, and replay? |
| Furnace | How can ownership-aware native code target CPUs, GPUs, and accelerators portably? |

These are research lanes, not promised products or separate repositories.
FERRIS promotes a lane into implementation only after a cited research note,
measurable baseline, and bounded validation contract exist.

## Microsoft Rust leadership package

- [Microsoft Rust investment brief](docs/leadership/MICROSOFT_RUST_INVESTMENT_BRIEF.md)
- [Upstream and differentiated opportunity map](docs/leadership/MICROSOFT_RUST_UPSTREAM_OPPORTUNITY_MAP.md)
- [Leadership package scorecard](docs/leadership/MICROSOFT_RUST_LEADERSHIP_PACKAGE_SCORECARD.md)
- [Leadership PowerPoint](docs/leadership/MICROSOFT_RUST_INVESTMENT_DECK.pptx)
- [PowerPoint source](docs/leadership/MICROSOFT_RUST_INVESTMENT_DECK_SOURCE.ps1)

## Rust reference library

Ferris carries a generated mirror of MAXIM's reviewed Rust references so
research, specifications, and blueprint work can cite a repository-local
source:

- [Compact Rust card](docs/reference/rust-reference/languages/09-RUST.md)
- [Rust language guide](docs/reference/rust-reference/rust-language/00-OVERVIEW.md)
- [Rust implementation architecture](docs/reference/rust-reference/rust-architecture/00-OVERVIEW.md)
- [Rust application blueprints](docs/reference/rust-reference/rust-application-blueprints/00-OVERVIEW.md)
- [Rust production engineering](docs/reference/rust-reference/rust-production-engineering/00-OVERVIEW.md)
- [Rust crate ecosystem](docs/reference/rust-reference/rust-crate-ecosystem/00-OVERVIEW.md)
- [Rust interop and migration](docs/reference/rust-reference/rust-interop-migration/00-OVERVIEW.md)
- [Rust security assurance](docs/reference/rust-reference/rust-security-assurance/00-OVERVIEW.md)
- [Rust performance](docs/reference/rust-reference/rust-performance/00-OVERVIEW.md)
- [Mirror policy and synchronization](docs/reference/rust-reference/README.md)

MAXIM remains canonical. Mirrored files are synchronized and hash-checked
rather than edited independently in Ferris.

## Rust engineering library

Ferris owns the applied operating guidance that joins Rust code to application
intent, platform support, renewable profiles, generated-change evidence,
upstream ownership, and conformance:

- [AI-assisted Rust engineering](docs/engineering/ai-assisted-rust/00-OVERVIEW.md)
- [Platform and target engineering](docs/engineering/platform-target-engineering/00-OVERVIEW.md)
- [Validated stack profiles](docs/engineering/validated-stack-profiles/00-OVERVIEW.md)
- [Maintainer and upstream contribution](docs/engineering/maintainer-upstream/00-OVERVIEW.md)
- [Reference implementations](docs/engineering/reference-implementations/00-OVERVIEW.md)
- [Engineering library index](docs/engineering/README.md)
- [Placement and gap-closure decision](docs/research/2026-08-12-ferris-rust-engineering-gaps.md)
- [Nine-role engineering-library review](docs/engineering/FERRIS-RUST-ENGINEERING-LIBRARY-ROLE-REVIEW.md)

These guides translate the existing research and Draft specifications into
repeatable workflows. They do not authorize product code or turn examples,
profiles, or AI proposals into correctness or support claims.

## Foundation state

FERRIS has completed the separately approved read-only implementation wave
through Pulse 19's ordinary-Cargo preservation control. The bounded product
surface includes
local `plan`, `explain`, declared-workspace `graph`, passive local `doctor`,
and non-executable `profile-diff` over two explicit experimental evidence
files. Pulse 13 adds a typed single-threaded process boundary for catchable
panics and output write failures. Its immutable cutoff passed the sealed
FHIF-030 held-out score; no held-out profile-diff claim is made.

The research corpus and 22-specification spine remain at Draft status.
Affected-only scope, query, execution, mutation, connectors, MCP, AI narrowing,
approval, deployment, remote evidence, and production claims remain
unauthorized. Profile diffing does not generate profiles, invoke Cargo or
owner tools, interpret evidence states, expose raw section values, or establish
compatibility, support, certification, or approval. Profile identifiers,
revisions, consumers, and JSON object keys are validated output-visible
metadata and must not contain secrets.

The initial command boundaries are recorded in
[`Pulse 01: Local Plan and Explain`](context/waves/2026-08-11-read-only-planning/pulses/pulse-01.md),
[`Pulse 02: Declared Workspace Graph`](context/waves/2026-08-11-read-only-planning/pulses/pulse-02.md),
and [`Pulse 04: Passive Doctor`](context/waves/2026-08-11-read-only-planning/pulses/pulse-04.md).
The current process boundary and held-out result are recorded in
[`Pulse 13`](context/waves/2026-08-11-read-only-planning/pulses/pulse-13.md)
and
[`Pulse 14`](context/waves/2026-08-11-read-only-planning/pulses/pulse-14.md).
The nine-family development conformance matrix and its role review are
recorded in
[`Pulse 15`](context/waves/2026-08-11-read-only-planning/pulses/pulse-15.md),
the
[fixture matrix](tests/fixtures/profile-evidence/MATRIX.md), and the
[Pulse 15 review](docs/plans/reviews/PULSE-15-ROLE-REVIEW.md).
Profile-diff held-out contract revision 3 passed independent Stage A at cutoff
`4371f4f6eb54097bff9badb29278c530d49e2f36`. It freezes LF-only
normative bytes, exact human stream mappings, typed collection outcomes,
strict Draft 2020-12 schemas, repository evidence, exhaustive lifecycle
branches, and the three-public-repository workflow. The
[public repository-selection binding](docs/simulations/profile-diff-held-out/REPOSITORY_SELECTION_BINDING.md)
now freezes the exact hosted, cross-target/`no_std`, and native-bound URLs,
full commits, commands, bounds, evidence, and identities without binding
hidden inputs or claiming execution.
The design is recorded in
[`Pulse 16`](context/waves/2026-08-11-read-only-planning/pulses/pulse-16.md),
the
[held-out program](docs/simulations/profile-diff-held-out/README.md), its
[identity contract](docs/simulations/profile-diff-held-out/IDENTITY.md),
[schemas](docs/simulations/profile-diff-held-out/schemas/README.md),
[repository workflow](docs/simulations/profile-diff-held-out/THREE_REPOSITORY_WORKFLOW.md),
and the
[Pulse 16 review](docs/plans/reviews/PULSE-16-ROLE-REVIEW.md). The later
[public-safe Stage B/C result](docs/simulations/profile-diff-held-out/PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
records a valid implementation failure at cutoff
`8cbb5356fd7b3acca435bc9fad4e97dabab66bb5`; it discloses no hidden
material and is not a pass.

The successor
[Platform Profile Conformance wave](context/waves/2026-08-12-platform-profile-conformance/WAVE.md)
has sealed infrastructure through Pulse 59. Pulse 64 is now permanently
withdrawn `invalid-prelaunch-unbound-wsl-qualification-contract` under
`P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION`: independent prelaunch review proved
its sealed WSL qualification still permitted a nonexistent/unbound optional
`qualify_exact_p57_wsl_bootstrap_contract` branch and literal `%SystemRoot%`
placeholders instead of exact `SystemRoot`/`SYSTEMROOT` source-precedence
derivation, concrete path comparison, and regular non-reparse identity proof,
so no authority callable or diagnostic ran and calls, seeds, descriptors,
processes, publications, and transfers remain zero while all conclusions remain
null. Pulse 63 is now also permanently withdrawn `invalid-prelaunch-wsl-bootstrap-contract` under
`P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT`: independent prelaunch review proved its
declared WSL preflight still underbound and partially contradicted the exact
Pulse 57 `_NativeWslSession` bootstrap path. It bound smaller generic
input/output/timeout limits instead of Pulse 57's exact payload/protocol
bounds, pointed to a misleading Pulse 59 schema path instead of the actual
staged Pulse 56 release tree under
`repository/.../pulse-56-retained-build-custody-release/`, exposed path-bearing
probe outputs instead of the exact canonical `bundle_root` stage JSON, and did
not fully bind the exact worker bootstrap/source-loader assumptions or a
private compare-only `ubuntu_runtime_parent` record. No authority callable or
diagnostic ran, so calls, seeds, descriptors, processes, publications,
transfers, and all conclusions remain zero or null. Pulse 62 is now also
permanently withdrawn `invalid-prelaunch-path-route-contract` under
`P62-REAL-PATH-WSL-ROUTE-CONTRACT`: independent prelaunch review proved its
sealed reversible-probe contract still underbound actual caller-supplied
root basenames and deepest exact Pulse 41/P56/P57/P58/P59 paths, and its
Ubuntu contract omitted the exact harmless WSL `wsl.exe --distribution
Ubuntu-24.04 --exec /usr/bin/python3` gate-3 preflight route, so calls,
seeds, descriptors, processes, publications, and transfers remain zero and
all conclusions remain null. Pulse 61 is now also permanently withdrawn `invalid-prelaunch-root-creatability-contract` under
`P61-ROOT-CREATABILITY-CALLABLE-CONTRACT`: independent prelaunch review
proved its safe-existing parent contract still did not prove the exact child
creation, reversible cleanup, restrictive-permission, same-filesystem-rename,
and path-length prerequisites required by the exact Pulse 41/P56/P57/P58/P59
callable stack, so calls, seeds, descriptors, processes, publications, and
transfers remain zero and all conclusions remain null. Pulse 60 is now permanently
withdrawn `invalid-prelaunch-runtime-root-contract` under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`: independent prelaunch review proved its
sealed `private_runtime_root`, `p27_cycle_root`, and Pulse 41 root-separation
contract contradicted the exact Pulse 58/Pulse 59 helper stack, so calls,
seeds, descriptors, processes, publications, and transfers remain zero and
all conclusions remain null. Any successor must use a new immutable cutoff
containing this withdrawal and exact final Pulse 59. Pulse 59 is sealed
witness-preserving terminal infrastructure over exact Pulse 58: it preserves
Pulse 53's terminal classes after exact P58 completion, derives a fresh
sibling terminal custody root, and leaves pre-execution P58 failure
publication not-attempted. Pulse 54 is permanently withdrawn
`invalid-prelaunch-checkout-variant-integrity`, zero-launch and
null-conclusion, after its validator failed in the required fresh
`core.autocrlf=false` checkout. Pulse 53 is sealed
witness-preserving ordered-executor infrastructure over exact Pulse 52: it
retains a verified Pulse 47 witness of a bounded Pulse 43 publication failure
without granting authority. Pulse 52 remains sealed ordered-materialization
infrastructure that runs Pulse 51 public gates before one exact P35 private
materialization. Pulse 49 is permanently withdrawn
`invalid-prelaunch-authority-integrity` before execution. Pulse 50 is
permanently `invalid-prelaunch-infrastructure-integrity`, withdrawn before
launch with a null conclusion under
`P50-EXECUTOR-RELEASE-OUTSIDE-AUTHORITY-CUTOFF`. Pulse 51 is sealed public
prelaunch executor infrastructure published after Pulse 50 authority/cutoff;
it cannot retroactively make Pulse 50 executable. Pulses 46 and 48 are
permanently `invalid-publication-integrity`, non-retryable, and null-conclusion.
Pulse
22 closed as an invalid diagnostic run after one Windows process and a
collector durability failure before the required Ubuntu partner launch; it
produced no category conclusion. Pulse 23 then qualified repaired collector
durability with 20 synthetic Windows/Ubuntu pairs and zero failures, without
executing a Ferris candidate. Pulse 24 opened one new independent
replacement package at cutoff
`cef0daabc349ac2333869959f21b9a3106e10484` but stopped
`invalid-before-candidates` because no exact inspectable collector source copy
was available from permitted public material. It ran zero preflight or
candidate processes and produced no category conclusion. Pulse 25 now
publishes the
[exact qualified collector source bundle](docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/README.md)
with machine-verified file and aggregate digests. The public copy passed 20
unit tests and 20 synthetic cross-platform pairs; it authorizes no diagnostic
search. Pulse 26 then opened
[one new independent public-bundle diagnostic authority](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_BUNDLE.md)
at cutoff `e01130a5c1fc5b8e58e13bbde03dfc39b8f1bf60`. A new custodian must
copy only the nine public files into an isolated workspace, independently
recompute every file hash and all three aggregates, and run exactly two
harmless zero-retry atomic Windows/Ubuntu preflight pairs before candidates.
Its independent execution verified every bundle and cutoff binding but
stopped
[`invalid during preflight`](docs/simulations/profile-diff-held-out/pulse-26-public-result/README.md)
when the second required pair failed exact-cardinality fresh-process reload.
No Ferris candidate was generated or launched.
Pulse 27 now publishes the
[exact-two-pair public adapter](docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/README.md)
with an unchanged Pulse 25 collector copy. The root cause was pair-local
expected cardinality `1` supplied to a whole-store verifier after pair two
existed. Qualification passed 50 of 50 cycles, 200 process rows, 100 pair
seals, 100 fresh reloads, zero retries, and zero residue without executing a
diagnostic candidate. Pulse 28 then authorized
[one new independent public-adapter diagnostic program](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_ADAPTER.md)
at immutable cutoff `2935f44475b811e619f2ef62e0d408f39c7e8149`.
New custody must copy exactly the 20 Pulse 27 manifest-listed public files,
recompute every file and adapter/test/collector/release aggregate, run exactly
one adapter invocation producing two Windows/Ubuntu pairs, four rows, and two
seals, then run exactly two fresh platform verifiers enforcing whole-store
cardinality `2/2/2`, with zero retries and zero residue. Its
[public result](docs/simulations/profile-diff-held-out/pulse-28-public-result/README.md)
closed `invalid-before-candidates`: 60 binding checks produced 10 passes and
50 failures. The Pulse 25 public manifest was expected at
`sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75`
and observed at
`sha256:03322e9fe6a3df6c71161e5f3916c51cc66c9453e9f1f3141bcc703bd02d7a0d`.
Git worktree EOL conversion at checkout, not corrupted Git blobs, caused the
mismatch. No build, preflight, generation, candidate, pair, seal, retry, or
category conclusion exists.
Pulse 29 now records
[public-artifact checkout normalization](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-29.md).
Anchored recursive `.gitattributes` rules force LF for every file below the
Pulse 25 and Pulse 27 release roots. A disposable resulting-index
materialization on Windows with `core.autocrlf=true` verified 36 LF files and
76 of 76 exact manifest, raw-file, aggregate, and collector-identity checks.
It adds no diagnostic or product authority.
Pulse 30 froze
[one final independent normalized public-adapter authority](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_NORMALIZED_PUBLIC_ADAPTER.md)
at cutoff `cf6b3309c31e5da37d4a8e6655a781f4e92ef603`. Before copy, custody must
materialize the cutoff with `core.autocrlf=true`, verify `text=set` and
`eol=lf` plus LF bytes for all 36 release files, and pass all 76 normalized
bindings. It then permits only the exact 20-file copy and fixed one-invocation,
two-pair, four-row, two-seal, two-verifier `2/2/2` preflight before wholly new
diagnostic material. The
[public result](docs/simulations/profile-diff-held-out/pulse-30-public-result/README.md)
passed 36/36 normalization, 76/76 bindings, the complete package and binary
freeze, and the exact preflight with zero retry/residue, then stopped
`invalid` before case materialization because the public input schema was
unavailable. It produced zero candidates/processes and a null conclusion.
The raw result digest is
`sha256:f75d33f054002cdd1b066678163ef926f62ec95ba826fef7273bc614c348f090`;
the receipt ID is
`sha256:8f08b0cf27f1b1bb97bcea0591b92c2143cf324736e2112744122838ca58dc30`.
Pulse 31 now publishes the complete
[`ferris.profile-evidence/v0` input contract](docs/simulations/profile-diff-held-out/INPUT_PROFILE_EVIDENCE.md),
recursive Draft 2020-12 schema, six positive fixtures, 33 negative controls,
exact byte/framing/duplicate/classification rules, nine-role review, and a
test-only validator without changing production or reopening Pulse 30.
Pulse 32 now freezes
[one new independent public-input authority](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_INPUT.md)
at immutable cutoff `29517d732db13cc2ffa304684b344f3538ab587d`.
It inherits every Pulse 30 normalization, package, adapter preflight,
freshness, coverage, oracle, search, collection, minimization, and publication
rule; pins the exact Pulse 31 contract, schema, six fixture, mutation-file,
and 33 per-control public digests; and requires 39/39 public-only
self-validation before generator/classifier freeze. Ferris source/tests are
outside the authorized generation scope. The authority is governance/test-only
and its
[public result](docs/simulations/profile-diff-held-out/pulse-32-public-result/README.md)
is permanently `invalid` at `cutoff-build-freeze`. Checkout and package gates
passed, but the required Ubuntu executable was unavailable. There were zero
preflight operations, public-input classifications, generated cases, or
candidate processes, and the category conclusion is null.
Pulse 33 now publishes the
[public build-freeze release](docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/README.md).
Its root-cause evidence records exit 127, `cargo: command not found`, because
an Ubuntu 24.04 WSL2 non-login shell omitted the ordinary rustup Cargo
directory from `PATH`. The exact cutoff compiles when Cargo is addressed
explicitly. The external adapter resolves Cargo explicitly, uses Cargo JSON
artifact output, and freezes deterministic Windows and Ubuntu hashes.
Qualification passed 14 unit tests, 20 synthetic checks, four clean rebuilds,
and all 37 manifest files without diagnostic execution or product changes.
Pulse 34 now freezes
[one new independent public authority](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_AUTHORITY.md)
at immutable cutoff `5df7492fa759c415f6ce540a33a4e89c46714348`.
It inherits every Pulse 32 public gate, pins the exact Pulse 33 manifest,
aggregate, seal, adapter, and public receipts, and requires explicit WSL
non-login Cargo discovery, Cargo `compiler-artifact` JSON, and exact
Windows/Ubuntu cutoff binary sizes, hashes, and receipts before inherited
adapter preflight. Its declaration identity is
`sha256:8975e07b9dd417604d06be12a24a448e8ae1834991aca9db086ae7c11b0b1e34`;
704 mutations protect the authority. Its
[public result](docs/simulations/profile-diff-held-out/pulse-34-public-result/README.md)
passed checkout `36/36`, bindings `76/76`, all public package/build gates,
exact `2/2/2` adapter preflight, and input self-validation `39/39`, then
closed `invalid` at `generation-materialization`. The frozen generator did
not complete an isolated corpus before candidate launch, so there were zero
valid cases, candidates, search processes, minimization, or reproducers and
the category conclusion is null. Further launches are prohibited.
Pulse 35 now publishes the
[public corpus-materializer release](docs/simulations/profile-diff-held-out/pulse-35-corpus-materializer-release/README.md).
It requires a 32-byte CSPRNG regular-file seed for materialization and
verification, releasing only its domain-separated commitment and HMAC-SHA256
pseudorandom case/order/profile tokens. Its exactly 70 descriptors include
valid, invalid, unsupported, incomplete, explicit no-launch blocked, and
target-bound path witnesses. An independent public-rule verifier derives every
semantic witness, the 17 value domains plus interaction-requirements domain
(`18/18`), and all eight exact tuple catalogs (`8/8`) from bytes, descriptors,
path state, and the private seed rather than trusting coverage labels. Twenty
isolated cycles include private-seed fresh reload, seed-length, same/different
seed, semantic-tamper, replay, residue, extra-output, staging-sync cleanup,
and publication-failure rollback controls with zero logical retries and
recorded `synced`/`unsupported` directory status. It runs no FERRIS candidate
or diagnostic, changes no product code, does not reopen Pulse 34, and grants
no new diagnostic authority.
Pulse 36's independent [public result](docs/simulations/profile-diff-held-out/pulse-36-public-result/README.md)
is permanently `invalid-before-pulse35-materialization` at
`pulse35-release-copy-verification`. Under authority
`2bf480459614dc56ee2bd744302e79f20a571092` and cutoff
`48697c8da0e93b92fa633e353925ca05707bf9ed`, all inherited gates passed:
`36/36` LF, `76/76` bindings, Pulse 33's 37 files/two platforms/two
binaries/two receipts, exact `1/2/4/2/2` preflight with zero retries/residue,
and Pulse 31's nine artifacts/`39/39`. The sole Pulse 35 copy matched 2/8
files and mismatched 6/8 (405,414 expected versus 403,316 observed bytes);
no seed, materialization, descriptors, candidates, pairs, or seals exist.
Cutoff blobs independently show the six text files were sealed as CRLF
working-tree bytes despite LF `.gitattributes` blobs; the JSON envelopes
match. This is checkout/binding infrastructure evidence, not product
evidence. The conclusion is null, further launches are prohibited, and this
result is permanently non-retryable. Its raw digest is
`sha256:735353e311dc63cd0cdef85c112bd60fd2c50c18f29858929a58f886b34009cc`
and receipt is
`sha256:d1f6f648ae8bb9a1fc44def2d392b72b76446b49439ff8f31e4124ad1fafc628`.
The closed authority declaration identity remains
`sha256:f4d83498f780e6d35bd0073f8d8ddeaa67d99fb2426978190f7af25fff746952`.
[`Pulse 37`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-37.md)
then records public-artifact checkout normalization and rebinding only. It
preserves Pulse 36's historical CRLF-derived manifest/seal identities and
permanent invalid result, while the normalized Pulse 35 successor binds
Git-clean LF manifest
`sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1`,
aggregate
`sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`,
and total `403316`. Its disposable Windows `core.autocrlf=true` clean-filter
materialization passed 8/8 file size/hash bindings and zero CR bytes for the
six text files, without qualification rerun, FERRIS or diagnostic execution,
product change, or new diagnostic authority.
[`Pulse 38`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-38.md)
is a new governance/test-only independent authority at immutable cutoff
`6807bd68aa01cbf0c819198765b7d6b5aa443328`, containing the complete Pulse 37
normalization. It preserves Pulse 36 and every earlier invalid program as
permanently invalid, non-retryable, and null-conclusion. It binds normalized
Pulse 35 manifest `sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1`,
aggregate `sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`,
403316 bytes, normalized seal, and Pulse 37 receipt before a new private
seed/materialization/fresh verification. Only then may one inherited
transactional search run at <=70 cases/processes per platform and <=140
processes total. Declaration
`sha256:a3317422e8c34d4e08d7c5e577e3539820f1376d7fba2ef38d262d1f967031b4`
was exercised once. Cutoff binding and one materialization passed, but the
first required attribute check did not complete. The program stopped before
package custody, seed creation, materialization, candidate execution, or
search; its conclusion is null and further launches are prohibited. The
public result raw digest is
`sha256:d3e74d220a9de9da4f2fff72812443de42272c9a8f78b0efad37573ab33b1c9c`
and receipt is
`sha256:56ddacc0e3043b327b8ce2d6ce869e9662a564faee9ce4f9a2c3d783a390bdad`.
[`Pulse 39`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-39.md)
is a public infrastructure-only checkout-verifier release, not a Pulse 38
retry. It records the independently reproduced below-root cwd ambiguity and
uses exactly 1 root-anchored NUL-framed
`git -C <checkout-root> check-attr -z --stdin text eol` invocation and exactly
1 separate root-anchored read-only Git version probe: 2 total Git processes, 0
retries, and no fallback check-attr form. A disposable Windows Git `2.55.0.windows.3` cutoff checkout with
`core.autocrlf=true` passes 36/36 `text=set,eol=lf` attributes, 36 LF files
with zero CR bytes, and the public Pulse 29 76/76 binding receipt. The public
manifest raw/aggregate are
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`;
the release seal raw/payload are
`sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c` /
`sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b`.
It grants no diagnostic, product, fix, certification, support, score, or
PLATFORM-001 authority and executes no FERRIS, candidate, build, preflight,
seed/corpus materialization, or private custody data.
Pulse 40 is a new governance/test-only independent authority at immutable
cutoff `65d1eec688f53bf7263ecfc8094ac849f9d3be4c`, not a Pulse 38 retry,
resume, reseed, reuse, correlation, or inference. Pulse 38 remains permanently
invalid, non-retryable, and null-conclusion. Before package copy, custody must
exactly copy and independently recompute the complete eight-file LF Git-clean Pulse 39 verifier release tree
manifest raw/aggregate
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c` /
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`,
five manifest payload files totaling 26455 bytes, and every raw release-tree byte. One fresh below-root `core.autocrlf=true` cutoff
checkout must receive one NUL-framed root-anchored check-attr invocation and
one version probe: two Git processes, zero retries, a deterministic 36/36
attribute/LF, zero-CR, safe-path report, and retained 76/76 binding proof.
Only then do inherited gates proceed. Declaration
`sha256:9ff14e5083ed4222f23e0ba68d945515225911633435b73c6c2fe4e6d9680a52`
has 9076 controls. Its sole run passed `8/8` cutoff tree and raw bindings plus
`5/5` manifest payload bindings, copied `8/8` files, then stopped when the
post-copy raw-binding transaction completed `0/8`. It created no cutoff
checkout and ran no verifier, later gate, seed, materialization, candidate, or
search. The result is permanently invalid, non-retryable, and
null-conclusion; raw digest
`sha256:b91ca8ed81a17ddcdb819044e2fa42be53a319a0dec71aaef2ca59b22f9352ca`,
receipt
`sha256:6e78c4e808c24c42f6dbe1df1565768b53a3f71549b82e65621c2e72f4e62237`.

Pulse 41 publishes a public infrastructure-only
[transactional copier and post-copy verifier](docs/simulations/profile-diff-held-out/pulse-41-transactional-copy-release/README.md).
It hard-binds the exact eight-file/31800-byte Pulse 39 source tree, verifies
`8/8` source/stage/final paths and raw bytes around one rename with zero
retries, flushes and fsyncs each staged destination before close, and records
bottom-up aggregate staging-directory posture without claiming unsupported
durability. It proves a post-rename rollback only after final-path absence and
a `synced` or explicit `unsupported` rollback-parent sync; otherwise the
outcome is indeterminate publication. Pulse 40 remains permanently invalid,
non-retryable, and null-conclusion at `pulse-39-release-custody`; its exact
private cause is not provable. Stale stage, duplicate/omitted root,
cwd/relative-root, and pre-final-sync verification are bounded public classes,
not a claimed private cause. No diagnostic, private-custody, product, fix,
score, certification, support, or PLATFORM-001 authority is added.

Pulse 43 publishes a public
[ordered-result publisher](docs/simulations/profile-diff-held-out/pulse-43-ordered-result-publisher-release/README.md)
that closes Pulse 42's missing-result and mixed-counter defects without
reopening a diagnostic. Closed event records must classify as
`public-artifact-self-validation` or `ordered-execution`; only the latter may
advance a predeclared ordered gate catalog. One terminal stop and no later
execution record make an early Pulse 33 stop incompatible with later Pulse 31
or Pulse 35 execution values. The standard-library publisher stages, fsyncs,
hashes, and verifies result plus receipt before one rename and final
verification. It returns success only when the two final files are present and
their raw/payload hashes recompute; errors are explicitly absent, rolled-back,
or indeterminate with zero retry/fallback. Windows directory sync is
`unsupported`, not a durability claim. This release accesses no private data
and grants no diagnostic, custody, product, category, or fix authority.

Pulse 44 publishes public
[retained-binary custody infrastructure](docs/simulations/profile-diff-held-out/pulse-44-retained-binary-custody-release/README.md).
Pulse 45 publishes a public
[binary-custody event bridge](docs/simulations/profile-diff-held-out/pulse-45-binary-custody-event-bridge-release/README.md)
that validates a complete sealed Pulse 44 result before translating it to a
platform-specific intermediate Pulse 43 gate. Pulse 44 failure posture remains
terminal, and neither release grants diagnostic, custody, product, category,
or fix authority.
It pins exact Pulse 33 identities before one retention-enabled build freeze,
requires fresh absolute work/final roots, verifies and file-fsyncs an exact
executable/receipt pair in a sibling stage, makes one rename, independently
verifies final `2/2`, and records honest directory synchronization. Its
Pulse-43-compatible completion event exists only after final verification;
failures are terminal `absent`, `rolled-back`, or `indeterminate` with zero
retry/fallback. Windows qualification rejected one dirty clone whose
normalization was changed after checkout, then independently passed from a
clone fixed to `core.autocrlf=false` before checkout: final pair `2/2`, one
rename, zero retries, size `1436672`, and exact Pulse 33 artifact SHA-256
`sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`.
Runtime roots were removed afterward. This remains infrastructure evidence
only, not a diagnostic, product, category, or fix conclusion.

The RUNE v1 contract-baseline dependency is satisfied, and
PLATFORM-001 remains Draft solely after the valid independent Pulse 17 fail.
Its
[program map](docs/engineering/validated-stack-profiles/07-PLATFORM-PROPOSED-PROGRAM.md)
records the sequence of all nine controlled families, lifecycle controls,
independently owned held-out gates, and the PLATFORM-001 Proposed review. It
adds no current product, owner-execution, support, or status authority.
Pulse 02 now freezes the controlled-fixture
[`ferris.platform-profile/v1` schema](docs/schemas/platform-profile/README.md)
and exact negative controls. The schema is not a generated profile, support
catalog, completed family, product parser, RUNE v1 claim, or PLATFORM-001
status change.
Pulse 03 adds only a dependency-free Rust integration harness for those
controls. Its
[Windows and Unix receipt](docs/plans/validation/PULSE-03-SCHEMA-HARNESS.md)
records exact valid, unsupported, invalid, and blocked outcomes without adding
a production parser or family evidence.
Pulse 04 completes the first controlled v1 family:
[pure data](docs/plans/validation/PULSE-04-PURE-DATA-FAMILY.md). Its two
zero-dependency revisions preserve locked/offline owner Cargo workflows and
exact profile digests on Windows and Unix; no other family or lifecycle gate
is implied.
Pulse 05 completes the controlled
[CLI/configuration family](docs/plans/validation/PULSE-05-CLI-CONFIG-FAMILY.md)
with exact process precedence, bounded explicit-file failures, owner workflow
preservation, and stable profile digests on Windows and Unix.
Pulse 06 completes the controlled
[hosted-service family](docs/plans/validation/PULSE-06-HOSTED-SERVICE-FAMILY.md)
with in-process health, malformed-request, cancellation, readiness, and
unavailable evidence while preserving owner workflows and excluding network
and deployment authority.
Pulse 07 completes the controlled
[embedded/`no_std` family](docs/plans/validation/PULSE-07-EMBEDDED-NO-STD-FAMILY.md)
with host behavior tests and exact `thumbv7em-none-eabi` compilation while
retaining device execution as unavailable.
Pulse 08 completes the controlled
[browser-WASM family](docs/plans/validation/PULSE-08-BROWSER-WASM-FAMILY.md)
with exact escaping, language-metadata rejection, and
`wasm32-unknown-unknown` compilation while retaining browser execution as
unavailable.
Pulse 09 completes the controlled
[WebAssembly-component family](docs/plans/validation/PULSE-09-WASM-COMPONENT-FAMILY.md)
with exact WIT revisions and non-empty `wasm32-wasip2` artifacts while
retaining component-runtime execution as unavailable.
Pulse 10 completes the controlled
[native-dependency family](docs/plans/validation/PULSE-10-NATIVE-DEPENDENCY-FAMILY.md)
through exact Windows `kernel32` and Unix libc process APIs while retaining
ambient native installation and servicing outside Cargo ownership.
Pulse 11 completes the controlled
[identity/provider family](docs/plans/validation/PULSE-11-IDENTITY-PROVIDER-FAMILY.md)
with bounded synthetic credentials, secret-redacted values, and explicit
non-security provider selection.
Pulse 12 completes the controlled
[assurance, packaging, and deployment family](docs/plans/validation/PULSE-12-ASSURANCE-DEPLOYMENT-FAMILY.md)
with actual Cargo package construction and exact rollback planning while
retaining signing and deployment as unavailable.
The platform-profile conformance wave completed its implementation-owned work
through renewal, substitution, emergency containment, adoption, and removal.
The [final review](docs/plans/reviews/PLATFORM-001-PROPOSED-ROLE-REVIEW.md)
keeps PLATFORM-001 at Draft: the independent held-out program produced a
valid implementation failure. Pulse 21 separately closes the RUNE v1
contract-baseline dependency without changing fixture bytes, identities,
digests, versions, or product behavior.
The [Pulse 17 independent handoff](docs/plans/validation/PULSE-17-INDEPENDENT-HANDOFF.md)
records the independent Stage A pass, corrected repository-selection binding,
and completed Stage B/C result. At cutoff
`8cbb5356fd7b3acca435bc9fad4e97dabab66bb5`, fixture
`P17-R3-D6B553CBC3B1240B673B8190` collected exactly 112 processes without
collection-integrity or privacy failure. All three repository workflows
passed, but the valid first score failed only in the public-safe category
`process-exit-agreement`. The
[immutable public-safe result](docs/simulations/profile-diff-held-out/PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
is not invalid custody or a held-out pass. The one-score program is closed;
retry, rescore, and reuse are prohibited.
The platform-profile
[`Pulse 19`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-19.md)
and its
[pre-implementation review](docs/plans/reviews/PULSE-19-PROCESS-EXIT-DIAGNOSTIC-ROLE-REVIEW.md)
produced a public/development diagnostic matrix with exactly 26 processes on
Windows and 26 on Ubuntu 24.04.4 WSL2. Every declared branch agreed from core
classification through actual OS exit and human/JSON parity, yielding bounded
`no-reproduction`. The result does not overturn or explain the closed
held-out failure. The fixture remains unavailable, and no CLI behavior change
or product fix is warranted by this evidence.
The prospective
[post-score diagnostic release protocol](docs/simulations/profile-diff-held-out/POST_SCORE_DIAGNOSTIC_RELEASE.md)
allows future programs to precommit a sanitized public reproducer after an
immutable score. It does not apply retroactively to Pulse 17.
The dependency-only
[`Pulse 21`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-21.md),
its
[machine-readable receipt](docs/plans/validation/PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT.json),
and
[nine-role review](docs/plans/reviews/PULSE-21-RUNE-V1-DEPENDENCY-ROLE-REVIEW.md)
recognize the already-bound exact RUNE revision as the accepted v1 contract
baseline. They do not claim SemVer `1.0.0`, a Git v1 tag, runtime-host
behavior, broad compatibility, support, or a change to the Pulse 17 result.
The authorized
[`Pulse 22`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-22.md)
and its
[normative diagnostic replication contract](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_REPLICATION.md)
precommit the `sanitized-reproducer` tier and one fresh independent
public-rule-based search capped at 512 cases per platform and 1,024 direct
process launches, with no candidate retry. The independent run generated 188
cases but became
[`invalid`](docs/simulations/profile-diff-held-out/pulse-22-public-result/README.md)
after one Windows launch when collector durability failed before the Ubuntu
partner launch. No completed pair, category conclusion, reproducer, receipt,
score, product fix, or Pulse 17 access exists, and PLATFORM-001 is unchanged.
The separate
[`Pulse 23`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-23.md)
collector qualification records the root cause and repaired durability path.
Its fixed synthetic controls passed 20 cross-platform pairs; it is
infrastructure evidence only and grants no search authority.
The closed
[`Pulse 24`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-24.md)
and its
[replacement diagnostic contract](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_REPLACEMENT.md)
froze public authority for one independently custodied fresh search. Pulse
22 remains permanently invalid and non-retryable. Pulse 24 then stopped
before preflight because its pinned collector source had not been published
as an inspectable bundle. It adds no product-fix or status
authority.
The separate
[`Pulse 25`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-25.md)
publishes that exact nine-file source/test bundle, manifest, qualification
receipt, and seal. It closes the infrastructure supply-chain gap but grants
no diagnostic search authority.
The later
[`Pulse 26`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-26.md)
opened one new independent public-bundle program.
Pulse 22 and Pulse 24 remain permanently invalid and non-retryable with null
category conclusions. Pulse 26 binds the exact public bundle, fresh custody
and generation, exactly two zero-retry preflight pairs, the complete inherited
coverage and oracle, transactional collection, bounded search and
minimization, and sanitized-reproducer publication requirements. It changes
no product behavior and grants no fix or PLATFORM-001 status authority.
The program closed invalid during its second mandatory synthetic pair, with
zero candidate activity and a null category conclusion.
The separate
[`Pulse 27`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-27.md)
publishes the
[exact-two-pair adapter release](docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/README.md),
its root-cause report, qualification receipt, release seal, and
[nine-role review](docs/plans/reviews/PULSE-27-PREFLIGHT-ADAPTER-RELEASE-ROLE-REVIEW.md).
The adapter uses the byte-for-byte immutable Pulse 25 collector and verifies
the complete two-pair store with whole-store cardinality `2`. Its 50-cycle
qualification produced 200 process rows, 100 pair seals, 100 fresh reloads,
zero retries, and zero residue. It grants no diagnostic authority. Pulses 22,
24, and 26 remain permanently invalid and non-retryable with null category
conclusions.
The closed
[`Pulse 28`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-28.md)
bound that exact release and the complete Pulse 25 collector bindings at
cutoff `2935f44475b811e619f2ef62e0d408f39c7e8149`. It froze one future
independent custody handoff, exact 20-file copying and hash recomputation, one
two-pair adapter invocation, two fresh platform verifiers, `2/2/2`
whole-store cardinality, and the complete inherited Pulse 26
generation/oracle/search/minimization/publication bounds. Its
[public result](docs/simulations/profile-diff-held-out/pulse-28-public-result/README.md)
is permanently `invalid-before-candidates` after a 60-check public binding
audit, with zero candidate activity, no retry, and a null conclusion.
[`Pulse 29`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-29.md)
adds anchored recursive LF checkout rules, coherent normalized release
bindings, a
[public receipt](docs/simulations/profile-diff-held-out/pulse-29-checkout-normalization/README.md),
and a
[nine-role review](docs/plans/reviews/PULSE-29-PUBLIC-ARTIFACT-CHECKOUT-NORMALIZATION-ROLE-REVIEW.md).
It verifies the resulting uncommitted index rather than the current working
tree and grants no diagnostic, fix, or status authority.
[`Pulse 30`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-30.md)
records the final new independent
[normalized public-adapter contract](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_NORMALIZED_PUBLIC_ADAPTER.md),
closed schema, authorized fixture, 322 mutation controls,
[nine-role review](docs/plans/reviews/PULSE-30-NORMALIZED-PUBLIC-ADAPTER-AUTHORITY-ROLE-REVIEW.md),
and test-only validation. It requires cutoff materialization, all 36
attribute/LF checks, all 76 normalized bindings, exact 20-file copying, fixed
preflight, wholly new later diagnostic material, and unchanged inherited
bounds. Its exact
[public result](docs/simulations/profile-diff-held-out/pulse-30-public-result/README.md)
is invalid after the preflight passed: the public input schema was unavailable
before case materialization, zero candidates/processes ran, and the category
conclusion is null.
[`Pulse 31`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-31.md)
publishes the missing public
[`ferris.profile-evidence/v0` contract](docs/simulations/profile-diff-held-out/INPUT_PROFILE_EVIDENCE.md),
[recursive schema](docs/simulations/profile-diff-held-out/schemas/ferris.profile-evidence.v0.schema.json),
six positive fixtures, 33 negative controls, and
[nine-role review](docs/plans/reviews/PULSE-31-PROFILE-EVIDENCE-INPUT-CONTRACT-ROLE-REVIEW.md).
It mirrors current acceptance in governance/test-only artifacts and grants no
production or diagnostic authority. No execution or production change is authorized.
[`Pulse 32`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-32.md)
publishes the
[public-input diagnostic contract](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_INPUT.md),
closed schema, exact declaration, 538 mutation controls, and
[nine-role review](docs/plans/reviews/PULSE-32-PUBLIC-INPUT-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md).
It binds the immutable Pulse 31 cutoff, all nine public input artifacts, 33
per-control digests, public-only generator/classifier scope, and required
39/39 self-validation while retaining every Pulse 30 gate and diagnostic
bound. Its
[public result](docs/simulations/profile-diff-held-out/pulse-32-public-result/README.md)
closed invalid at cutoff build freeze after passed checkout/package gates,
with zero preflight/input validation/cases and a null conclusion.
[`Pulse 33`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-33.md)
records the sealed
[37-file public build-freeze release](docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/README.md),
[root-cause report](docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/root-cause-report.md),
[nine-role review](docs/plans/reviews/PULSE-33-PUBLIC-BUILD-FREEZE-RELEASE-ROLE-REVIEW.md),
and Rust validation. It identifies WSL non-login Cargo `PATH` discovery as the
external blocker and records deterministic Windows/Ubuntu build hashes after
14 unit tests, 20 synthetic checks, and four clean rebuilds. It executes no
diagnostic and changes no product files.
[`Pulse 34`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-34.md)
publishes the
[public-authority diagnostic contract](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_AUTHORITY.md),
closed schema, exact declaration, 704 mutation controls,
[nine-role review](docs/plans/reviews/PULSE-34-PUBLIC-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md),
and test-only validation. It binds immutable cutoff
`5df7492fa759c415f6ce540a33a4e89c46714348`, retains every Pulse 32 public
gate, pins the exact Pulse 33 release, and requires explicit Cargo discovery,
Cargo JSON artifact output, and exact Windows/Ubuntu cutoff binary freezes.
The declaration identity is
`sha256:8975e07b9dd417604d06be12a24a448e8ae1834991aca9db086ae7c11b0b1e34`.
The
[public result](docs/simulations/profile-diff-held-out/pulse-34-public-result/README.md)
is permanently invalid at `generation-materialization` after every public
gate passed. It contains zero valid cases or candidate launches, has a null
category conclusion, and changes no product files.
[`Pulse 36`](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-36.md) publishes the [materialized public diagnostic authority](docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_36_AUTHORITY.md), closed schema, exact declaration, 1998 mutation controls, [nine-role review](docs/plans/reviews/PULSE-36-MATERIALIZED-PUBLIC-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md), and test-only validation. It binds cutoff `48697c8da0e93b92fa633e353925ca05707bf9ed`, inherits every Pulse 34 gate, pins exact Pulse 35 public materialization/verification before one later bounded transactional search, executes nothing, and preserves every prior invalid null conclusion.
Windows and Ubuntu 24.04.4 WSL2 development validation is recorded in
[`Pulse 17`](context/waves/2026-08-11-read-only-planning/pulses/pulse-17.md),
the
[cross-platform receipt](docs/plans/validation/PULSE-17-CROSS-PLATFORM-VALIDATION.md),
and the
[Pulse 17 review](docs/plans/reviews/PULSE-17-ROLE-REVIEW.md). This is not
native Linux support or held-out evidence.
The public-CLI input and working-directory non-mutation proof is recorded in
[`Pulse 18`](context/waves/2026-08-11-read-only-planning/pulses/pulse-18.md),
the
[filesystem immutability receipt](docs/plans/validation/PULSE-18-FILESYSTEM-IMMUTABILITY.md),
and the
[Pulse 18 review](docs/plans/reviews/PULSE-18-ROLE-REVIEW.md). It is not a
whole-system sandbox or complete removal proof.
The representative owner-native before-and-after Cargo control is recorded in
[`Pulse 19`](context/waves/2026-08-11-read-only-planning/pulses/pulse-19.md),
the
[ordinary Cargo preservation receipt](docs/plans/validation/PULSE-19-ORDINARY-CARGO-PRESERVATION.md),
and the
[Pulse 19 review](docs/plans/reviews/PULSE-19-ROLE-REVIEW.md). It is one
zero-dependency development control, not universal lifecycle evidence.
The Pulse 13 held-out result is the
[public-safe FHIF-030 result](docs/simulations/held-out/PUBLIC_SAFE_DOCTOR_RESULT_022.md).

```console
cargo run -p ferris-cli -- plan --workspace-id <PORTABLE_ID> --manifest-path <Cargo.toml>
cargo run -p ferris-cli -- explain --workspace-id <PORTABLE_ID> --manifest-path <Cargo.toml>
cargo run -p ferris-cli -- graph --workspace-id <PORTABLE_ID> --manifest-path <Cargo.toml>
cargo run -p ferris-cli -- doctor --workspace-id <PORTABLE_ID> --manifest-path <Cargo.toml>
cargo run -p ferris-cli -- profile-diff --before <PROFILE_JSON> --after <PROFILE_JSON>
```

## Research

- [What the first seven performance questions established](docs/research/2026-08-08-first-seven-performance-questions.md)
- [rustc startup and metadata loading](docs/research/2026-08-08-rustc-startup-metadata.md)
- [Parsing and tokenization](docs/research/2026-08-08-parsing-tokenization.md)
- [Declarative macro expansion](docs/research/2026-08-08-declarative-macro-expansion.md)
- [Name resolution and HIR lowering](docs/research/2026-08-08-name-resolution-hir-lowering.md)
- [Type inference and type checking](docs/research/2026-08-08-type-inference-checking.md)
- [Trait-solving cost and reuse](docs/research/2026-08-08-trait-solving-cost-reuse.md)
- [Borrow-checking cost and incrementality](docs/research/2026-08-08-borrow-checking-cost-incrementality.md)
- [MIR construction and optimization](docs/research/2026-08-08-mir-construction-optimization.md)
- [Frontend parallelism](docs/research/2026-08-08-frontend-parallelism.md)
- [Query dependency precision and false invalidation](docs/research/2026-08-08-query-dependency-precision.md)
- [Incremental cache overhead and reuse economics](docs/research/2026-08-08-incremental-cache-overhead.md)
- [Early-phase incrementality](docs/research/2026-08-08-early-phase-incrementality.md)
- [Relink-Don't-Rebuild and cross-crate interfaces](docs/research/2026-08-08-relink-dont-rebuild.md)
- [Reuse across check, build, lint, test, and doctest](docs/research/2026-08-08-command-artifact-reuse.md)
- [Procedural-macro cost, inputs, and reuse](docs/research/2026-08-08-procedural-macro-cost-input-reuse.md)
- [Build-script input, output, and rerun precision](docs/research/2026-08-09-build-script-input-output-precision.md)
- [Monomorphization and generic-instance reuse](docs/research/2026-08-09-monomorphization-generic-instance-reuse.md)
- [Codegen-unit partitioning](docs/research/2026-08-09-codegen-unit-partitioning.md)
- [LLVM optimization cost](docs/research/2026-08-09-llvm-optimization-cost.md)
- [Development codegen backends](docs/research/2026-08-09-development-codegen-backends.md)
- [Debug information and object emission](docs/research/2026-08-09-debug-information-object-emission.md)
- [Linking and incremental linking](docs/research/2026-08-09-linking-incremental-linking.md)
- [Remote artifact provenance and Rust Build Forest roots](docs/research/2026-08-09-remote-artifact-provenance.md)
- [Function-level machine-code caching](docs/research/2026-08-09-function-level-machine-code-caching.md)
- [Crate slicing and partial dependency compilation](docs/research/2026-08-09-crate-slicing-partial-compilation.md)
- [System effects on Rust build latency](docs/research/2026-08-09-system-effects-build-latency.md)
- [Workspace modularization and crate boundaries](docs/research/2026-08-09-workspace-modularization-crate-boundaries.md)
- [Impact-aware validation selection](docs/research/2026-08-09-impact-aware-validation-selection.md)
- [Rust performance contribution program closeout](docs/research/2026-08-09-rust-performance-contribution-program-closeout.md)
- [Rust capability coverage](docs/research/2026-08-09-rust-capability-coverage.md)
- [Rust foundational crate census](docs/research/2026-08-09-rust-foundational-crate-census.md)
- [Rust interchange contracts](docs/research/2026-08-09-rust-interchange-contracts.md)
- [Rust async portability](docs/research/2026-08-09-rust-async-portability.md)
- [Rust maintenance and stewardship](docs/research/2026-08-09-rust-maintenance-stewardship.md)
- [Rust security and provenance](docs/research/2026-08-09-rust-security-provenance.md)
- [Rust platform compatibility](docs/research/2026-08-09-rust-platform-compatibility.md)
- [Rust feature and version fragmentation](docs/research/2026-08-09-rust-feature-version-fragmentation.md)
- [Rust native dependency boundary](docs/research/2026-08-10-rust-native-dependency-boundary.md)
- [Rust crate discovery and selection](docs/research/2026-08-10-rust-crate-discovery-selection.md)
- [Rust compatibility-tested stack profiles](docs/research/2026-08-10-rust-compatibility-stack-profiles.md)
- [Rust ecosystem intervention decisions](docs/research/2026-08-10-rust-ecosystem-intervention-decisions.md)
- [Rust contract and interface strategy](docs/research/2026-08-10-rust-contract-interface-strategy.md)
- [Rust build-state references](docs/research/2026-08-10-rust-build-state-references.md)
- [Blueprint federated execution planning](docs/research/2026-08-10-blueprint-federated-execution-planning.md)
- [Blueprint cross-command scope model](docs/research/2026-08-10-blueprint-cross-command-scope-model.md)
- [Blueprint competitive positioning and CLI strategy](docs/research/2026-08-10-blueprint-competitive-positioning.md)
- [Ferris product naming decision](docs/research/2026-08-10-ferris-product-naming.md)
- [Ferris seven-program synthesis](docs/research/2026-08-10-ferris-seven-program-synthesis.md)
- [Ferris Microsoft enterprise integration](docs/research/2026-08-10-ferris-microsoft-enterprise-integration.md)
- [Performance program role checkpoint](docs/research/2026-08-08-performance-program-role-checkpoint.md)
- [Rust Build Forest opportunity](docs/research/2026-08-08-rust-build-forest-opportunity.md)
- [Rust compiler performance: architecture, bottlenecks, and FERRIS opportunities](docs/research/2026-08-07-rustc-compiler-performance.md)
- [Rust latency component roadmap](docs/research/2026-08-07-rust-latency-component-roadmap.md)
- [Rust incremental reuse scopes and contribution boundaries](docs/research/2026-08-07-rust-incremental-reuse-boundaries.md)
- [Rust performance research-question registry](docs/research/questions/README.md)
- [Crates Series research-question registry](docs/research/questions/ecosystem/README.md)
- [Build latency measurement contract](docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)
- [Rust performance contribution packet](docs/specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md)
- [FERRIS specification registry](docs/specs/README.md)
- [Ferris specification simulations](docs/simulations/README.md)
- [Ferris specification simulation method](docs/research/2026-08-10-ferris-specification-simulation-method.md)
- [Query Forest component model](docs/specs/FOREST_COMPONENT_MODEL.md)
- [FOREST-001 nine-role review](docs/specs/reviews/FOREST-001-ROLE-REVIEW.md)
- [Ferris public-contract review](docs/specs/reviews/FERRIS-PUBLIC-CONTRACTS-ROLE-REVIEW.md)
- [Ferris seven-program review](docs/plans/reviews/FERRIS-SEVEN-PROGRAM-ROLE-REVIEW.md)
- [Ferris Microsoft integration review](docs/plans/reviews/FERRIS-MICROSOFT-INTEGRATION-ROLE-REVIEW.md)
- [Build intelligence research program](docs/plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md)
- [Crates Series: ecosystem and library research](docs/plans/ECOSYSTEM_LIBRARY_RESEARCH_PROGRAM.md)
- [Ferris program](docs/plans/FERRIS_PROGRAM.md)
- [Blueprint planning engine program](docs/plans/BLUEPRINT_PROGRAM.md)
- [FERRIS enterprise Rust application platform](docs/plans/ENTERPRISE_RUST_APPLICATION_PLATFORM.md)

## Review model

FERRIS uses the
[ROLES](https://github.com/giodl73-repo/ROLES) `.roles` convention. Rust safety,
compiler performance, interoperability, AI assurance, ecosystem strategy,
scope, validation, and adopter concerns are represented as explicit review
lenses.

The [FERRIS engineering principles](docs/governance/ENGINEERING_PRINCIPLES.md)
define the lab's decision rules, common failure modes, prototype gate, and
initial review disposition from every repository role.

## Repository skills

- `/research` runs hypothesis-led, cited compiler and native-tooling research.
- `/ferrium-wave` plans research-led capability waves.
- `/ferrium-pulse` executes bounded research or implementation pulses.
- `/ferrium-research` remains a compatibility alias for `/research`.

## Operating rules

1. Research before standardizing a language, protocol, benchmark, or product.
2. Treat compiler success as evidence, not proof of behavioral correctness.
3. Keep shared contracts product-neutral.
4. Measure build, runtime, safety, and migration claims.
5. Implement only capabilities explicitly selected by an approved bounded
   pulse.
6. Record non-goals and rejected approaches.

## Non-goals

- Creating a general-purpose Rust replacement before a defensible wedge exists.
- Building another text-only coding assistant without compiler-grounded checks.
- Claiming formal verification, memory safety, or performance without evidence.
- Embedding portfolio-product semantics in shared crates.
- Treating experimental lane names as committed products.

## Validation

```powershell
git grep -n "FERRIS\\|FERRIUM-" -- README.md PRODUCT_PLAN.md docs context
git diff --check
```

## License

MIT.

## Pulse 42 public-result integrity closure

[Pulse 42](docs/simulations/profile-diff-held-out/pulse-42-public-result/README.md)
is permanently `invalid-publication-integrity`, non-retryable, and
null-conclusion at `public-result-publication`. Its authority cutoff
`2a8b7c27ac465ab78a8ec7ca331b9e427a8625c8` and declaration
`sha256:4da4d749892a487e30467b68bf8e35e9f72655dfb3a75414ead10ff40e0868cc`
remain historical authority only.
Pulse 38 and Pulse 40 remain unchanged permanent invalid/null-conclusion
predecessors. Historical public bindings remain Pulse 41
`sha256:600efbbcf0fdb41669d4700fc7bd40f003ec5d9742709f18e9f5658e0a29d4a8`
and Pulse 39
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c`.
The sole public summary's expected result
files were absent (`1`) and claimed paths observed were `0`; its claimed Pulse
33 stop is order-inconsistent with later reported quantities. All such facts
are `reported_unvalidated`, including
`P42-FROZEN-BINARY-UNAVAILABLE`, which is not an established root cause. No
product, diagnostic, category, or fix authority follows.

## Pulse 46 publication-order diagnostic authority

[Pulse 46](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-46.md)
was an independent process-exit diagnostic authority at
cutoff `22ea38e274b882d6e607810382f842b76e483f10`. It does not retry,
resume, or reconstruct permanently invalid Pulse 42. Before execution it
requires exact current-cutoff Pulse 41/Pulse 39/Pulse 43/Pulse 44/Pulse 45
trees, separate public self-validation and ordered execution, one Pulse 45
and Pulse 44 invocation per platform, and controlled non-public retained
binary roots. Its eight ordered gates end with the sole
`bounded-process-exit-search` gate. Pulse 43 publishes the final public result
exactly once only after `2/2` final files, hash recomputation, one rename, and
zero retries; failed publication is `invalid-publication/null` and discloses
only publication posture. Declaration
`sha256:92847e645338fd142710c1afcff5d6ad5540c35e6322ccf59b574f2fd3d61534`
has 9,208 controls and pre-launch zero execution state.

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

[Pulse 47](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-47.md)
releases a public, standard-library-only witness wrapper for the exact Pulse
43 publisher. It invokes Pulse 43 once through an injected or independently
verified callable, validates its complete closed summary, and persistently
publishes only a bounded publication outcome. Published Pulse 43 output
witnesses public hashes, final `2/2`, rename/retry/sync posture, and
ordered/self-validation aggregates. Failed Pulse 43 output witnesses only its
code, absent/rolled-back/indeterminate posture, final-files flag,
rename/retry, and exact three-directory sync posture; it has no gate events,
counts, paths, private data, or executable bytes.

The witness transaction file-fsyncs, staged-verifies, syncs, renames once,
final-rehashes, and parent-syncs exactly
`publication-witness.json` and `release-receipt.json`, with zero retry or
fallback. If that transaction fails, its public output contains only the
bounded witness failure code and posture, never captured Pulse 43 detail.
Pulse 47 does not retry, resume, reconstruct, or infer the permanently closed
Pulse 46 authority. It creates no diagnostic, custody, product, category,
score, certification, support, or fix authority.

## Pulse 48 permanent public closeout

[Pulse 48](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-48.md)
is permanently `invalid-publication-integrity`, non-retryable, and
null-conclusion. Authority commit `5a8d92d211806d0f2940016af6c317878c5fdfc1`,
cutoff `70c8fc2dfa60b6732fa265bb5fcf6326ac97ad2d`, and declaration
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

[Pulse 49](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-49.md)
is permanently `invalid-prelaunch-authority-integrity` at
`prelaunch-authority-validation`, blocker
`P49-P35-CASE-PROCESS-CARDINALITY-CONFLICT`. The exact historical authority
commit is `80f78fa4edb2d3497a830b2879ea9ff9c6f0aea5`.

Pulse 35's exact public materializer creates 70 descriptors per platform:
69 `launch-ready` process cases and one final `no-launch` disposition with
`not-materialized` before/after states and
`external-immutable-binary-freeze`. The authority's 70 processes per platform
and 140 total are therefore impossible while honoring its own descriptors.
It is permanently withdrawn, non-retryable, and null before launch; P47/P43,
private operations/data/artifacts, result/witness roots, and runtime/public
root transfer all remain absent.

Any successor needs fresh explicit authority for 70 case dispositions, 69
processes, and one no-launch disposition per platform, totaling 140 cases,
138 processes, and two no-launch dispositions. The prelaunch declaration,
schema, and mutations remain exact historical artifacts.

## Pulse 50 withdrawn prelaunch authority

[Pulse 50](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-50.md)
historical authority commit
`48fe9fdcdda03378f68781cae342796c9f11720d` at cutoff
`94d473563a1686091be94a72f491b0ff0d903800` is permanently
`invalid-prelaunch-infrastructure-integrity`, non-retryable, and
null-conclusion. The blocker is
`P50-EXECUTOR-RELEASE-OUTSIDE-AUTHORITY-CUTOFF` at
`prelaunch-public-infrastructure`.

The public prelaunch audit found that the sealed executor was absent from the
authority/cutoff: Pulse 35 CRLF/LF schema binding, descriptor/69+1/P43 runner,
Pulse 27 CLI seam, Pulse 31 schema-count consistency, WSL/canonical Ubuntu
mapping, exact Ubuntu Pulse 33 toolchain/hash custody, and Python resolver
detail. Multiple custodians stopped before launch. There was no diagnostic
execution, private material, seed, descriptor, candidate process, P43/P47
invocation, result root, witness root, or inference; all execution values are
zero or false.

## Pulse 51 public diagnostic-executor release

[Pulse 51](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-51.md)
is complete public prelaunch infrastructure only. It binds the full Pulse 35
release tree plus the formerly omitted CRLF/LF machine schema, frozen Pulse 31
artifacts and 33 mutations, exact P27 callable contract, P33
binary/receipt/toolchain identities, P44-to-P45 custody summaries, canonical
platform IDs, and sealed P43/P45/P47 dependencies. Its fixed runtime rejects
caller-selected grants, gate events, launchers, and expectations; authority is
external governance. It launches Windows natively and Ubuntu only through
`wsl.exe --distribution Ubuntu-24.04 --cd ... --exec ...` after verified
private-root path translation, validates 70/69/1 and complete frozen output
contracts, independently derives profile-diff semantics and recomputes all
four frozen output identities, compares only path-free process-exit semantics,
records private hashes in memory, and emits only P43-safe aggregates.

Pulse 51 does not authorize or execute Pulse 50/Pulse 51 diagnostic work,
create seeds/descriptors, invoke a private candidate, or publish a terminal
root. Its exact release commit
`d09c923c1e2cd2be003026597f4ad2a0e2d3764f` is after the Pulse 50 authority
and cutoff, so its sealed public infrastructure cannot cure or execute Pulse
50.

A future successor requires fresh authority binding exact Pulse 51, exact
Pulse 52, and the existing public releases.  It must use Pulse 52's production
ordered-materialization API, which reuses the one-use Pulse 51
`TerminalPulse47Once` seam. It cannot consume or revive withdrawn Pulse 50
authority.

## Pulse 52 ordered-materialization executor release

[Pulse 52](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-52.md)
wraps exact Pulse 51 only to close its prospective ordering gap.  It accepts
concrete P39 checkout and P41 final-copy roots plus P44 custody inputs; binds
and invokes exact P39/P41 custody verification before internally constructing
gate 1; validates gates 1–6 once; then creates one private 32-byte CSPRNG seed
with `O_EXCL`/`fsync`, invokes exact P35 materialization and verification once,
cleans the private namespace, executes the fixed Pulse 51 `70/69/1`
Windows/WSL path, and invokes the fresh one-use P47 seam.  Private execution
completion is published only after exact P43 result and P47 witness success
shapes and final-root verification.  Any P43 or P47 failure closes
`invalid-publication-integrity`, with null product/category/fix conclusions,
no retry or added event, and verified terminal cleanup.  If that cleanup
remains indeterminate, the callable raises only the public-safe unresolved
`terminal-publication-cleanup-indeterminate` posture instead of returning a
completed closeout.  Twenty fake-only cycles passed with one P39/P41 custody
sequence and 138 fake dispatches each, with no seed, descriptor, path, binary,
or private-record disclosure in P43 events.  This is infrastructure only: no
authority, diagnostic, candidate, score, certification, fix, product
behavior, or PLATFORM-001 conclusion is created.  A future authority must
bind exact Pulse 51 **and** Pulse 52.

## Pulse 53 witness-preserving ordered executor release

[Pulse 53](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-53.md)
binds exact Pulse 51 commit `d09c923c1e2cd2be003026597f4ad2a0e2d3764f`
and exact Pulse 52 commit `e4ef9617f227670f3911be42ca63df4b2e66d24f`.
It imports Pulse 52's sealed phase helpers and copies only the bounded
orchestration/terminal branch.  P39/P41 custody, public gates, CSPRNG/P35
materialization, private cleanup, fixed Windows/WSL dispatch, and exact error
boundaries are unchanged.

After exactly one P47-to-P43 terminal call, verified P43 `2/2` plus P47 `2/2`
output is retained as `published-result`.  A verified P47 `2/2` witness of an
exact P43 `absent`, `rolled-back`, or `indeterminate` failure is retained as
`published-failure-witness`; no P43 result root is required or retained and all
conclusions are null.  P47 failure, malformed/hash-mismatched output, or a
missing final shape is `invalid-witness-publication`, with no retry and exact
Pulse 52 bounded verified cleanup.  Public transfer descriptors expose only
tree kind, exact file counts, and verified raw/payload hashes--never terminal
paths, IDs, roots, seeds, descriptors, or private records.

Twenty fake-only alternating cycles passed with ten published results, ten
retained failure witnesses, all three P43 failure postures, and 2,760 fake
dispatches.  No FERRIS binary ran.  This is infrastructure only and does not
create authority, a diagnostic conclusion, product behavior, or a
PLATFORM-001 conclusion.  A future authority must bind exact Pulse 51, Pulse
52, and Pulse 53 and cannot revive withdrawn Pulse 50 authority.

## Pulse 54 independent witness-preserving diagnostic authority

[Pulse 54](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-54.md)
was published as `authorized-unexecuted` authority at exact self-excluding cutoff
`42a16e298c5af55b05df5ceb8e3477d0dd45c814`. It binds complete current public
P27/P31/P33/P35/P37/P39/P41/P43/P44/P45/P47/P51/P52/P53 path, hash,
manifest, receipt, seal, API, and signature identities. It is not a retry,
resume, reconstruction, reseed, reuse, correlation, or inference of the
permanently invalid/null Pulse 48 or withdrawn/null Pulse 49/Pulse 50 records.

Its sole runtime operation, not performed by this pulse, is exactly one
injection-free Pulse 53
`run_witness_preserving_ordered_executor` call after fresh anonymous
`core.autocrlf=false` cutoff checkouts, exact Pulse 33 Windows `/Brepro` and
`Ubuntu-24.04` WSL freezes, one P44 custody operation per platform, fresh
P39/P41/runtime/P27 roots, and P43-safe catalog prevalidation. Its fixed
route is P39/P41; Windows P44/P45; Ubuntu P44/P45; P27; P31; P35/P37; one
32-byte CSPRNG seed/materializer/verifier; `70/69/1` per platform and
`140/138/2` total; then one P47-to-P43 terminal path. A first semantic
projection mismatch stops later work.

`published-result` copies verified P43 `2/2` and P47 `2/2` trees to the
separate Pulse 54 public result and witness destinations. A
`published-failure-witness` leaves the result destination absent and transfers
only the verified P47 `2/2` witness as a permanent null-conclusion
publication-integrity closeout. Invalid or cleanup-indeterminate postures make
no success claim. There are no Pulse 54 runtime, result, or witness artifacts
at authorization.

Independent pre-call custody stopped before any runtime work because the
required validator depended on the authoring checkout's CRLF materialization
and failed against canonical LF bytes in the mandated
`core.autocrlf=false` checkout. Pulse 54 is permanently withdrawn under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; no P53 call, seed, descriptor,
process, publication, transfer, result, or witness occurred. Any successor
requires a fresh cutoff and checkout-materialization-independent validation.

## Pulse 55 immutable-blob successor authority

[Pulse 55](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-55.md)
was published as the fresh `authorized-unexecuted` successor to permanently
withdrawn Pulse 54, not a retry or amendment. At self-excluding cutoff
`47113e444ef3309afec9a844f0cba62775f19f6f`, it binds the same exact
P27/P31/P33/P35/P37/P39/P41/P43/P44/P45/P47/P51/P52/P53 chain and only the
one-shot P53 callable. Its validator derives canonical identities from cutoff
Git blobs and validates the checkout separately using only canonical identities
or explicit sealed materializations. Pulse 35 retains P37-normalized canonical
LF identities and P51 supported CRLF/LF variants with size/newline framing;
fresh anonymous `core.autocrlf=false` is the required runtime posture.

Pulse 54 remains permanently withdrawn
`invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`, with zero calls, artifacts, and
conclusions. It remains distinct from Pulse 55's consumed one-call closeout.

At authority commit `36b3ac6b9692924af57c7c98b0a976835fe778f6`, one fresh
P44 custody operation occurred per platform before the sole P53 call. The call
returned public publication disposition/classification `not-attempted` at
`pulse-41-pulse-39-public-custody`: zero completed gates, seed, descriptors,
processes, no-launch dispositions, P27/P39/P41/materializer/verifier/P47
calls, and result/witness transfers; all conclusions are null. This consumed
Pulse 55 permanently as `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`; it is non-retryable and non-resumable.

The Windows retained artifact hash/size differed from P51's exact expectation.
Ubuntu matched its expected artifact hash/size, while both fresh retained
receipt payload identities differed from P51's published non-retaining
receipt identities. The structural P33 contract issue is that
`build_freeze` `retained_in_public_bundle` changes when the executable is
retained, so an exact published receipt payload identity cannot equal a real
retained-custody receipt; Windows `/Brepro` plus Rust/Cargo version alone also
left the linker/SDK environment underbound. A future authority requires a new
sealed executor chain binding a corrected P33 retained-build/custody contract,
a fully bound Windows linker/SDK environment or qualified deterministic linker
route, semantic receipt verification for retained artifacts, and replacement
ordered/witness layers binding the corrected diagnostic executor. No
replacement implementation is authorized here. See the
[Pulse 55 execution closeout](docs/simulations/profile-diff-held-out/PULSE_55_EXECUTION_RECORD.md).

Pulse 55's immutable historical declaration is
`sha256:45ac35775c34e8a86fdc90ad1554104f2728a676d51ab46125bfcf126db21655`;
its `19261` deterministic controls raise the registry to `100582`.
## Pulse 56 retained deterministic build and custody release

[Pulse 56](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-56.md)
replaces the staged caller-forgeable retained-identity executor with sealed
foundational build/custody infrastructure. It creates two fresh clean
`core.autocrlf=false` cutoff builds, proves byte identity, writes a new
semantic retained receipt, and custody-publishes only that binary/receipt pair
with one rename. Windows binds sysroot rust-lld; Ubuntu WSL binds its measured
cc/GNU-ld route after a bounded rust-lld failure. Both platform probes passed.
The receipt and custody root are evidence only. A future launch requires an
exact live in-process handle with verified bytes and bounded atomic uses; no
caller-supplied summary, binary, receipt, root, builder, runner, environment,
or callback can authorize it. It creates no diagnostic executor or authority
and never executes FERRIS.

## Pulse 57 capability-bound diagnostic executor successor

[Pulse 57](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-57.md)
seals the successor that replaces the caller-summary custody route with exact
live Pulse 56 capabilities while retaining exact Pulse 51 descriptor,
profile-diff, bounded-output, topology, and first-mismatch controls. The
injection-free production callable creates one Windows and one native
`Ubuntu-24.04` WSL capability, dispatches only through Pulse 56
`launch_verified`, and closes every still-live capability on terminal paths.
The privacy-safe catalog names only Windows/Ubuntu capability-build/custody
gates and performed predecessor/P27/P31/P35/P37/descriptor/process controls;
it does not invoke or claim Pulse 44/Pulse 45 or P39/P41 execution. A later
ordered layer must add P39/P41 before private materialization and cannot infer
them from Pulse 57. Qualification uses sealed predecessor sources plus
harmless fakes for 20 cycles and 2,760 launches, preceded by 13 negative
controls. It runs no FERRIS binary and grants no diagnostic authority or
conclusion.

## Pulse 58 ordered capability/materialization executor successor

[Pulse 58](context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-58.md)
combines the sealed P39/P41 public-custody sequence and P52 P35 staging reader
with exact Pulse 57 live P56 capability semantics. Its sole injection-free
callable completes all public gates before one 32-byte private seed and exact
70-descriptor materialization, then uses the already-live Windows and native
`Ubuntu-24.04` WSL capabilities for fixed `70/69/1` semantics. It returns
private accounting and privacy-safe ordered events only; it makes no P44/P45,
publication, authority, FERRIS execution, product, or PLATFORM-001 claim.
P39's caller-supplied checkout is a future-authority precondition; P58 invokes
only P39's exact path/attribute/LF semantics and P41's exact copy validation,
not freshness, anonymity, HEAD, clean-tree, or `core.autocrlf` claims.
Qualification is fake-only: 20 cycles, 2,760 launches, 20 behavioral controls,
and verified cleanup.

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
registry remain unchanged at `24700` controls, keeping the monotonic total at
`209335` until a separately sealed successor is added.

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
registry remain unchanged at `23266` controls; Pulse 64 later raised the
monotonic total to `209335`.

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
schema and mutation registry remain unchanged at `21644` controls; Pulses 63
and 64 later raised the monotonic total to `209335`.
