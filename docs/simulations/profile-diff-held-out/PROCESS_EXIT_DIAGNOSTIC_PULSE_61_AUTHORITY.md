# Pulse 61 independent witness-preserving capability/materialization diagnostic authority

Status: `authorized-unexecuted`
Immutable self-excluding cutoff: `70ed752359c04e4aac77a49280c37f2cf6b8d012`
Declaration identity: `sha256:d3016922f4bcc09b739b0e71f0317edd54d14975edee103bc3ad1cfecb67ec5d`

Pulse 61 is one fresh independent diagnostic authority. It is not a retry,
resume, amendment, reconstruction, reseed, reuse, correlation, or inference of
any permanently closed predecessor. Pulse 46 and Pulse 48 remain permanently
`invalid-publication-integrity`; Pulse 49 remains permanently withdrawn
`invalid-prelaunch-authority-integrity`; Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`; Pulse 54 remains permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; Pulse 55 remains permanently
closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`; and Pulse 60 remains permanently
withdrawn `invalid-prelaunch-runtime-root-contract` under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`, with zero calls, seeds, descriptors,
processes, publications, and transfers plus null conclusions. None of those
dispositions is cured, consumed, amended, or reinterpreted here.

The cutoff is the exact withdrawal commit containing Pulse 60 closeout and
excluding this authority, its schema, mutation registry, authority record,
wave pulse record, and validator. No Pulse 61 runtime, seed, descriptor,
result, or witness artifact exists at authorization.

## Exact immutable binding and checkout validation

The canonical declaration binds the complete exact P27/P31/P35/P37/P39/P41/
P43/P47/P51/P52/P56/P57/P58/P59 release chain, including full cutoff-tree path
sets, canonical identities, manifests, receipts, seals, source files, and the
exact P56/P57/P58/P59 callable signatures. Every canonical identity is derived
by the validator from the immutable Git blob at cutoff
`70ed752359c04e4aac77a49280c37f2cf6b8d012`; local working-tree bytes are never
an identity source. The cutoff itself proves Pulse 60's permanent withdrawal
before any new authority is introduced.

The validator separately validates runtime materialization against those exact
canonical identities. It accepts an alternate materialization only when it is
an explicitly declared complete-file CRLF/LF identity, size, CR/LF count, and
newline framing supported by the sealed chain. Pulse 35 retains exactly the
Pulse 51 sealed P35/P37 custody variants and the Pulse 37-normalized canonical
LF identities. The actual authorized runtime posture remains fresh anonymous
`core.autocrlf=false` authority and P39 checkouts. No generic normalization or
arbitrary working-tree-only hash rule is permitted.

## Exact root contract and sole runtime operation

The only production runtime call permitted by Pulse 61 is the exact exported
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

It is invoked exactly once and consumes the authority on attempt. Pulse 61
MUST NOT directly call Pulse 58, Pulse 57, Pulse 56, Pulse 52, Pulse 51,
Pulse 47, or Pulse 43. There are no retries, resumes, fallback executors, or
republication routes.

Before that unperformed call, independent custody MUST satisfy the exact public
root contract proved by the sealed helpers:

1. obtain the authority anonymously and materialize `repo_root` as the same
   fresh anonymous exact-cutoff clean `core.autocrlf=false` Windows checkout as
   the authority checkout at head
   `70ed752359c04e4aac77a49280c37f2cf6b8d012`;
2. materialize `p39_checkout_root` as a separate fresh anonymous exact-cutoff
   clean `core.autocrlf=false` Windows checkout, validated only against
   immutable cutoff blobs and supported declared checkout variants;
3. supply `private_runtime_root` as an absolute existing empty safe directory
   accepted by `p51._safe_runtime_root(...)`;
4. supply `p27_cycle_root` as an absent absolute direct child of
   `private_runtime_root`, distinct from Pulse 58's reserved
   `.pulse58-private-launch` namespace;
5. supply `p41_final_root` as an absent absolute Windows path whose exact
   derived stage root `.<final-root-name>.pulse-41-stage` and rollback/final
   publication path remain absent and non-overlapping with `repo_root`, the
   runtime root, the P27 cycle root, the P39 checkout root, and the derived
   Pulse 59 terminal sibling; and
6. supply `ubuntu_runtime_parent` as an absolute native Linux path string that
   is not under `/mnt/*`, is never serialized publicly, and is an existing safe
   directory when Pulse 56 creates its fresh child runtime root.

Pulse 59 derives its terminal custody sibling as
`<private_runtime_root.name>.pulse59-terminal-publication` under the runtime
parent. That sibling path MUST remain absent before the sole callable is
attempted, and its parent MUST remain a safe existing directory.

No seed, descriptor root, candidate process, result, or witness artifact may
exist first.

## Terminal public transfer and permanent closeout

`published-result` alone permits transfer of the verified path-free Pulse 59
public descriptor plus the known Pulse 61 result and witness custody roots:
verified P43 `2/2` result and verified P47 `2/2` witness trees.
`published-failure-witness` requires the result path to remain absent and
transfers only the verified path-free descriptor plus the known Pulse 61
witness root for the exact P47 `2/2` witness tree.

Any prelaunch or runtime failure that leaves publication `not-attempted`
permanently closes Pulse 61 with null category, diagnostic, and product
conclusions; no result or witness transfer is permitted.
`invalid-witness-publication` also transfers nothing and makes no success
claim. Pulse 59 `terminal-publication-cleanup-indeterminate` is a fatal
unresolved-custody posture: it transfers nothing, carries null conclusions,
and permits no completed diagnostic claim. Pulse 61 never exposes private
seed, descriptor, runtime, or source-path material publicly.

## Exhaustive control surface

The closed declaration, recursive Draft 2020-12 schema, and exhaustive
deterministic mutation registry contain `20058` controls. The monotonic
registry total is `139725` from the prior `119667`.

See the [Pulse 61 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-61.md).
