# Pulse 28: Independent Process-Exit Diagnostic Public-Adapter Authority

Status: Complete; invalid before candidates
Implementation authority: Closed public-result recording and test-only
validation only

## Goal

Record the closed outcome of the independently owned
`process-exit-agreement` diagnostic program that was authorized to use the
exact public Pulse 25 collector and Pulse 27 two-pair adapter.

Pulses 22, 24, and 26 remain permanently invalid, non-retryable, and unable
to produce category conclusions. Pulse 28 is not their retry, resume, reseed,
rescore, reuse, continuation, correlation, or inference.

## Fixed execution target

Any later custody execution is bound to immutable Ferris cutoff
`2935f44475b811e619f2ef62e0d408f39c7e8149`.

That cutoff already contains the complete Pulse 27 release and does not
contain this Pulse 28 authority.

## Exact public infrastructure

Pulse 28 pins every Pulse 25 collector binding already frozen by Pulse 26:
the nine public files, manifest, qualification report, source/test/bundle
aggregates, release receipt, and release seal.

It also pins the exact Pulse 27 release directory, complete 20-entry manifest,
every per-file path/kind/size/digest, and:

- release aggregate
  `sha256:31f38a79629d6b5da1fab9cb335450a95a1763f1ac80b1d8d851b103a318e540`;
- root-cause report
  `sha256:9bd5ac7aa29b1f621df09eefc2ff33369c5ba5810e4e5f76f4e7e15aab57f478`;
- qualification receipt
  `sha256:94ec4237fd046281b9971e7eea67dc1ae7208996ed69b50ce8430a78cd0b6886`;
  and
- release seal
  `sha256:6bff93434170ee79a4b5210ee26b647ab6dba351dccc4ccf3a5e224de56ced38`.

New custody may copy exactly the 20 manifest-listed public files into a new
isolated workspace. It may not access a private or prior custody workspace.
It must independently recompute every file digest and the adapter-source,
adapter-test, collector-copy, and complete-release aggregates, and confirm
that the nine collector files are byte-identical to Pulse 25.

## Exact preflight

After package and executable verification, custody must run exactly one
adapter invocation. That invocation must deterministically create exactly two
atomic Windows/Ubuntu pairs: two Windows rows, two Ubuntu rows, four total
rows, and two pair seals.

After the complete store exists, custody must run exactly two fresh read-only
verifier processes, one on Windows and one on Ubuntu. Both must enforce exact
whole-store cardinality `2/2/2`: two Windows rows, two Ubuntu rows, and two
pair seals. Adapter, pair, and verifier retries are zero; residue must be zero.
Any failure makes the package `invalid-before-candidates`.

## Public result

The independent custodian stopped at the public-package binding audit before
copy, build, preflight, generation, or candidate execution. The audit
evaluated all 60 declared bindings: 10 passed and 50 failed. Pulse 25
accounted for 18 failures; Pulse 27 accounted for 42 checks, with 10 passes
and 32 failures.

The first mismatch was the Pulse 25 public-manifest raw SHA-256:

- expected
  `sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75`;
- observed
  `sha256:03322e9fe6a3df6c71161e5f3916c51cc66c9453e9f1f3141bcc703bd02d7a0d`.

The root cause is Git worktree EOL conversion at checkout. The committed Git
blobs were not corrupted. The mismatch existed before custody package copy,
so the result is `invalid-before-candidates`: zero binaries, preflight
invocations, verifier processes, generated cases, Ferris candidate launches,
pairs, seals, or retries; null category conclusion; and no authorized
reproducer. Further launches are prohibited.

## Fresh program and inherited bounds

The custody identity and isolated workspace must be new before public-package
copy. Only after successful preflight may that custodian freeze a new private
seed and commitment, new independent classifier and generator, new case and
coverage manifests, and a fresh corpus.

Pulse 28 preserves the complete Pulse 26 public generation domains, eight
mandatory interactions, eight oracle fields, six target predicates,
transactional durable collection, 512-case/platform and 1,024-process search
bounds, zero retries, one execution, 128-transformation and 256-process
minimization bounds, and sanitized-reproducer or bounded no-reproduction
publication requirements.

## Evidence

- [Normative contract](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_ADAPTER.md)
- [Declaration schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-public-adapter.v1.schema.json)
- [Authorized declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-adapter.json)
- [Mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-adapter-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-28-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-ADAPTER-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_public_adapter.rs)
- [Public result](../../../../docs/simulations/profile-diff-held-out/pulse-28-public-result/README.md)
- [Exact Pulse 25 collector release](../../../../docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/README.md)
- [Exact Pulse 27 adapter release](../../../../docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/README.md)

## Decision

The public contract, closed schema, authorized fixture, mutation controls,
nine-role review, result receipt, and test-only validator now form a closed
historical record. The binding audit ran, failed before package copy, and
permanently ended the authority. No adapter invocation, verifier launch,
generation, Ferris candidate execution, minimization, or retry occurred.

No product behavior, dependency, production source, score, certification,
support claim, fix authority, or PLATFORM-001 status changes.

## Stop conditions

Pulse 28 is closed. Stop rather than widen or retry it if work would:

- execute the adapter, a verifier, a preflight, or a Ferris candidate;
- use a cutoff other than
  `2935f44475b811e619f2ef62e0d408f39c7e8149`;
- modify or substitute any Pulse 25 or Pulse 27 public file;
- copy an unlisted or private-workspace file into custody;
- run other than one adapter invocation or two fresh verifier processes;
- accept cardinality other than `2/2/2`, any retry, or any residue;
- reopen, retry, reuse, correlate, or infer from Pulse 22, 24, or 26;
- weaken inherited generation, oracle, search, collection, minimization, or
  publication bounds; or
- change PLATFORM-001 from Draft.
