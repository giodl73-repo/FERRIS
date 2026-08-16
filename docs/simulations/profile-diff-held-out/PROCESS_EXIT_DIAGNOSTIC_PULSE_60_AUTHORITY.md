# Pulse 60 independent witness-preserving capability/materialization diagnostic authority

Status: `authorized-unexecuted`
Immutable self-excluding cutoff: `6945f5fc96868c97267a1635fbb5219cc398eeb4`
Declaration identity: `sha256:13ba3aaa5d61c536a9dd22b3a57816b1b7d93c2e11592c87117190709cbfb40c`

Pulse 60 is one fresh independent diagnostic authority. It is not a retry,
resume, amendment, reconstruction, reseed, reuse, correlation, or inference of
any permanently closed predecessor. Pulse 46 and Pulse 48 remain permanently
`invalid-publication-integrity`; Pulse 49 remains permanently withdrawn
`invalid-prelaunch-authority-integrity`; Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`; Pulse 54 remains permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; and Pulse 55 remains permanently
closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`. None of those dispositions is cured,
consumed, amended, or reinterpreted here.

The cutoff is the exact final Pulse 59 HEAD. It excludes this authority, its
schema, mutation registry, authority record, wave pulse record, and validator.
No Pulse 60 runtime, seed, descriptor, result, or witness artifact exists at
authorization.

## Exact immutable binding and checkout validation

The canonical declaration binds the complete exact P27/P31/P35/P37/P39/P41/
P43/P47/P51/P52/P56/P57/P58/P59 release chain, including full cutoff-tree path
sets, canonical identities, manifests, receipts, seals, source files, and the
exact P56/P57/P58/P59 callable signatures. Every canonical identity is derived
by the validator from the immutable Git blob at cutoff
`6945f5fc96868c97267a1635fbb5219cc398eeb4`; local working-tree bytes are never
an identity source.

The validator separately validates runtime materialization against those exact
canonical identities. It accepts an alternate materialization only when it is
an explicitly declared complete-file CRLF/LF identity, size, CR/LF count, and
newline framing supported by the sealed chain. Pulse 35 retains exactly the
Pulse 51 sealed P35/P37 custody variants and the Pulse 37-normalized canonical
LF identities. The actual authorized runtime posture remains fresh anonymous
`core.autocrlf=false` authority and P39 checkouts. No generic normalization or
arbitrary working-tree-only hash rule is permitted.

## Sole runtime operation

The only production runtime call permitted by Pulse 60 is the exact exported
Pulse 59 callable:

```python
run_witness_preserving_capability_materialization_executor(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    ubuntu_runtime_parent: str,
) -> WitnessPreservingCapabilityMaterializationResult
```

It is invoked exactly once and consumes the authority on attempt. Pulse 60
MUST NOT directly call Pulse 58, Pulse 57, Pulse 56, Pulse 52, Pulse 51,
Pulse 47, or Pulse 43. There are no retries, resumes, fallback executors, or
republication routes.

Before that unperformed call, independent custody MUST obtain the authority
anonymously; validate the authority checkout and the caller-supplied P39 root
as fresh anonymous exact-cutoff clean `core.autocrlf=false` checkouts at head
`6945f5fc96868c97267a1635fbb5219cc398eeb4`; prepare fresh absent
runtime/P27/P41 roots; and supply a native Ubuntu runtime parent. No seed,
descriptor root, candidate process, result, or witness artifact may exist
first.

## Terminal public transfer and permanent closeout

`published-result` alone permits transfer of the verified path-free Pulse 59
public descriptor plus the known Pulse 60 result and witness custody roots:
verified P43 `2/2` result and verified P47 `2/2` witness trees.
`published-failure-witness` requires the result path to remain absent and
transfers only the verified path-free descriptor plus the known Pulse 60
witness root for the exact P47 `2/2` witness tree.

Any prelaunch or runtime failure that leaves publication `not-attempted`
permanently closes Pulse 60 with null category, diagnostic, and product
conclusions; no result or witness transfer is permitted.
`invalid-witness-publication` also transfers nothing and makes no success
claim. Pulse 59 `terminal-publication-cleanup-indeterminate` is a fatal
unresolved-custody posture: it transfers nothing, carries null conclusions,
and permits no completed diagnostic claim. Pulse 60 never exposes private
seed, descriptor, runtime, or source-path material publicly.

## Exhaustive control surface

The closed declaration, recursive Draft 2020-12 schema, and exhaustive
deterministic mutation registry contain `19085` controls. The monotonic
registry total is `119667` from the prior `100582`.

See the [Pulse 60 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-60.md).
