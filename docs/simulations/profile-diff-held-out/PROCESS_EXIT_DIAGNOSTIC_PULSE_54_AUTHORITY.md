# Pulse 54 independent witness-preserving diagnostic authority

Status: `authorized-unexecuted`
Immutable self-excluding cutoff:
`42a16e298c5af55b05df5ceb8e3477d0dd45c814`
Declaration identity:
`sha256:44420f3496067b0422c4146bd4b51354c72c45f7a2758677cf501a683d702d49`

Pulse 54 is one fresh independent diagnostic authority. It is not a retry,
resume, reconstruction, reseed, reuse, correlation, or inference of Pulse 48,
Pulse 49, or Pulse 50. Pulse 48 remains permanently
`invalid-publication-integrity` and null-conclusion; Pulse 49 remains
permanently withdrawn `invalid-prelaunch-authority-integrity`, zero-launch,
and null-conclusion; Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`, zero-launch, and
null-conclusion. None is consumed, cured, or reinterpreted here.

The cutoff is the exact public Pulse 53 release commit and excludes this
authority, its schema, mutation registry, review, and validator. No runtime,
seed, descriptor, candidate, result, or witness artifact exists at
authorization.

## Exact public binding

The canonical declaration binds complete exact cutoff path sets, raw hashes,
manifest, qualification-receipt, and release-seal identities for:

- [Pulse 27 exact adapter](pulse-27-preflight-adapter-release/README.md);
- Pulse 31's exact contract, schema, six positive fixtures, and mutation
  registry;
- [Pulse 33 build freeze](pulse-33-build-freeze-release/README.md);
- [Pulse 35 materializer](pulse-35-corpus-materializer-release/README.md)
  plus its external machine schema;
- [Pulse 37 checkout normalization](pulse-37-checkout-normalization/README.md);
- [Pulse 39 checkout verifier](pulse-39-checkout-verifier-release/README.md)
  and [Pulse 41 transactional copier](pulse-41-transactional-copy-release/README.md);
- [Pulse 43 publisher](pulse-43-ordered-result-publisher-release/README.md),
  [Pulse 44 retained custody](pulse-44-retained-binary-custody-release/README.md),
  [Pulse 45 bridge](pulse-45-binary-custody-event-bridge-release/README.md),
  and [Pulse 47 witness](pulse-47-publication-outcome-witness-release/README.md);
- [Pulse 51 executor](pulse-51-diagnostic-executor-release/README.md) at
  `d09c923c1e2cd2be003026597f4ad2a0e2d3764f`;
- [Pulse 52 ordered executor](pulse-52-ordered-materialization-executor-release/README.md)
  at `e4ef9617f227670f3911be42ca63df4b2e66d24f`; and
- [Pulse 53 witness-preserving executor](pulse-53-witness-preserving-ordered-executor-release/README.md)
  at `42a16e298c5af55b05df5ceb8e3477d0dd45c814`.

The Rust validator reads every bound file from both the current tree and the
cutoff blob, rejects a missing, extra, symlinked, modified, or post-cutoff
release file, and checks all bound callable source hashes and signatures.

## Sole runtime operation

The only production runtime call permitted by Pulse 54 is the exact exported
Pulse 53 callable:

```python
witness_preserving_ordered_executor.run_witness_preserving_ordered_executor(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    retained_custodies: Mapping[str, object],
) -> WitnessPreservingOrderedResult
```

It is invoked exactly once. That invocation consumes this authority even if it
fails before creating the private seed. Public-only prerequisite preparation
does not consume authority. The callable accepts no authority token, callback,
seed, descriptor, materializer, verifier, launcher, fake binary, expectation,
terminal root, or trust-mode injection.

Pulse 54 MUST NOT directly call Pulse 51
`run_diagnostic_executor`, Pulse 52 `run_ordered_materialization_executor`,
Pulse 47 `witness_pulse_43`, or Pulse 43 `publish_result`. P53 alone retains
the sealed Pulse 51+52 chain and the one-use terminal route. There are no
retries, fallback executors, alternate callables, or republication.

## Pre-call public prerequisites

Before the one P53 invocation, the custodian MUST complete only public
preparation:

1. create fresh anonymous Windows and Ubuntu cutoff checkouts at the exact
   cutoff with `core.autocrlf=false`;
2. rebuild the exact Pulse 33 Windows binary with Rust/Cargo 1.95.0 and
   `RUSTFLAGS=-C link-arg=/Brepro`, then verify size `1436672`, hash
   `sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`,
   and receipt
   `sha256:b1d42470ca709406c5869bae9e677334539e745faf8e0f400e4ae93f34cf7d7a`;
3. rebuild the exact Pulse 33 Ubuntu binary in `Ubuntu-24.04` WSL with
   Rust/Cargo 1.97.1, then verify size `1945448`, hash
   `sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4`,
   and receipt
   `sha256:b01e45259e340309772e1d5d5c947cff163ada8dad8b9ddf19775f0a537c4cae`;
4. perform exact Pulse 44 custody once for each platform and supply the
   retained roots and complete summaries to P53;
5. prepare a fresh Pulse 39 checkout root, a fresh absent Pulse 41 final-copy
   root, and fresh runtime and Pulse 27 cycle roots; and
6. validate every release, API, signature, gate identifier, toolchain, and
   root precondition before P53 is called.

No seed, descriptor root, candidate process, result root, or witness root may
be created during these prerequisites.

## Fixed execution and public catalog

The P43-safe gate catalog is exact and ordered:

1. `pulse-41-pulse-39-public-custody`
2. `windows-retained-binary-custody`
3. `ubuntu-retained-binary-custody`
4. `exact-adapter-preflight`
5. `pulse-31-public-input`
6. `pulse-35-pulse-37-normalization`
7. `bounded-materialization`
8. `bounded-process-exit-search`

The exact public validation IDs are `public-catalog-prevalidation` and
`public-input-contract`. Every gate and validation ID is prevalidated against
the P43 identifier grammar and forbidden standalone token set:
`candidate`, `corpus`, `credential`, `home`, `password`, `private`, `seed`,
`secret`, `token`, `user`, and `workspace`.

Inside P53, the required order is P39/P41 custody; Windows P44/P45; Ubuntu
P44/P45; the exact P27 callable once; P31 contract validation; P35/P37
custody; exactly one fresh CSPRNG 32-byte seed; exactly one P35 materializer
and verifier; then 70 dispositions per platform. Each platform has 69 OS
processes and one final no-launch disposition, for `140/138/2` total
dispositions/processes/no-launches. The first semantic-projection mismatch
stops later work. Exactly one terminal P47-to-P43 route follows; no direct
P43 or P47 call is permitted.

## Terminal public transfer

P53's terminal disposition controls the only permitted public transfer:

| P53 disposition | Required transfer | Conclusion |
|---|---|---|
| `published-result` | Copy and verify the exact two-file P43 result tree to `pulse-54-public-result/` and the exact two-file P47 witness tree to `pulse-54-publication-witness/`. | All authority-level diagnostic, category, and product conclusions remain null. |
| `published-failure-witness` | The P43 result path remains absent. Copy and verify only the exact two-file P47 witness tree to `pulse-54-publication-witness/`. | Permanent valid publication-integrity closeout; all conclusions are null. |
| `invalid-witness-publication` or cleanup-indeterminate | Make no success claim and no public tree copy. Use only the bounded authority-permitted invalid/null posture. | Permanent invalid/null. |

No private record, private root path, source root path, seed, descriptor,
token, case ID, or candidate detail may appear in a transfer record.

## Controls and validation

All launch and publication counters begin at zero. The canonical declaration,
strict recursively closed Draft 2020-12 schema, and deterministic exhaustive
mutation registry contain `13485` controls; the monotonic registry total is
`81321` from the prior `67836`.

The test-only Rust validator verifies the complete release/API/signature chain,
catalog, topology, transfer contract, prelaunch toolchains, zero state, cutoff
self-exclusion, absence of Pulse 54 output roots, and rejection of every
mutation. It invokes no Pulse runtime callable or diagnostic.

See the [Pulse 54 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-54.md)
and the [nine-role review](../../plans/reviews/PULSE-54-WITNESS-PRESERVING-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md).
