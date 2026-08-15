# Pulse 55 independent witness-preserving diagnostic authority

Status: `authorized-unexecuted`
Immutable self-excluding cutoff: `47113e444ef3309afec9a844f0cba62775f19f6f`
Declaration identity: `sha256:45ac35775c34e8a86fdc90ad1554104f2728a676d51ab46125bfcf126db21655`

Pulse 55 is one fresh independent diagnostic authority. It is not a retry,
amendment, resume, reconstruction, reseed, reuse, correlation, or inference of
any permanently closed predecessor. Pulse 46 and Pulse 48 remain permanently
`invalid-publication-integrity`; Pulse 49 remains permanently withdrawn
`invalid-prelaunch-authority-integrity`; Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`; and Pulse 54 remains permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`. Pulse 54 made zero authority or P53
calls, created zero runtime, seed, result, or witness artifacts, and has null
category, diagnostic, and product conclusions. None of those dispositions is
cured, consumed, amended, or reinterpreted here.

The cutoff is the public Pulse 54 closeout commit. It excludes this authority,
its schema, mutation registry, review, and validator. No Pulse 55 runtime,
seed, descriptor, candidate, result, or witness artifact exists at
authorization.

## Exact immutable binding and checkout validation

The canonical declaration binds the complete exact P27/P31/P33/P35/P37/P39/
P41/P43/P44/P45/P47/P51/P52/P53 release chain, including full cutoff-tree path
sets, canonical identities, manifests, receipts, seals, and callable source
signatures. Every canonical identity is derived by the validator from the
immutable Git blob at cutoff `47113e444ef3309afec9a844f0cba62775f19f6f`; local
working-tree bytes are never an identity source.

The validator separately validates a runtime working tree against those exact
canonical identities. It accepts an alternate materialization only when it is
an explicitly declared complete-file CRLF/LF identity, size, CR/LF count, and
newline framing supported by a sealed release. Pulse 35 retains exactly the
Pulse 51 sealed P35/P37 custody variants and the Pulse 37-normalized canonical
LF identities. The actual authorized runtime posture remains a fresh anonymous
`core.autocrlf=false` checkout. No generic normalization or arbitrary
hash-choice rule is permitted.

## Sole runtime operation

The only production runtime call permitted by Pulse 55 is the exact exported
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

It is invoked exactly once and consumes the authority on attempt. Pulse 55
MUST NOT directly call Pulse 51, Pulse 52, Pulse 47, or Pulse 43. There are no
retries, fallback executors, alternate callables, or republication.

Before that unperformed call, independent custody MUST create fresh anonymous
Windows and Ubuntu exact-cutoff `core.autocrlf=false` checkouts; freeze the
exact P33 Windows `/Brepro` and `Ubuntu-24.04` WSL binaries; perform P44 once
per platform; provide fresh P39/P41/runtime/P27 roots; and prevalidate every
public catalog identifier. No seed, descriptor, candidate process, result, or
witness artifact may exist first.

The sealed P53 route remains P39/P41; Windows P44/P45; Ubuntu P44/P45; P27;
P31; P35/P37; one 32-byte CSPRNG seed; one P35 materializer and verifier; then
`70/69/1` per platform and `140/138/2` total, with first semantic-projection
mismatch stop and exactly one P47-to-P43 terminal route.

## Terminal public transfer

`published-result` alone permits copying and verifying exact P43 `2/2` and
P47 `2/2` public trees to `pulse-55-public-result/` and
`pulse-55-publication-witness/`. `published-failure-witness` requires the
result path to remain absent and transfers only the P47 `2/2` witness to the
Pulse 55 witness path. Invalid or cleanup-indeterminate output permits no
success claim or transfer. Every category, diagnostic, and product conclusion
remains null.

The closed declaration, recursive Draft 2020-12 schema, and exhaustive
deterministic mutation registry contain `19261` controls. The monotonic
registry total is `100582` from the prior `81321`.

See the [Pulse 55 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-55.md)
and [nine-role review](../../plans/reviews/PULSE-55-WITNESS-PRESERVING-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md).