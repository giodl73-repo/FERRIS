# Pulse 61 independent witness-preserving capability/materialization diagnostic authority

Status: `authorized-unexecuted` historical declaration; permanently withdrawn
before launch
Immutable self-excluding cutoff: `70ed752359c04e4aac77a49280c37f2cf6b8d012`
Declaration identity: `sha256:d3016922f4bcc09b739b0e71f0317edd54d14975edee103bc3ad1cfecb67ec5d`

Pulse 61 remains the exact historical one-shot authority declaration. It is
not amended or reissued here. Pulse 46 and Pulse 48 remain permanently
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

## Exact immutable historical binding

The canonical declaration binds the complete exact P27/P31/P35/P37/P39/P41/
P43/P47/P51/P52/P56/P57/P58/P59 release chain, including full cutoff-tree path
sets, canonical identities, manifests, receipts, seals, source files, and the
exact P56/P57/P58/P59 callable signatures. Every canonical identity is derived
from the immutable Git blob at cutoff `70ed752359c04e4aac77a49280c37f2cf6b8d012`;
local working-tree bytes are never an identity source. The cutoff itself proves
Pulse 60's permanent withdrawal before this historical authority was introduced.

The validator separately validates runtime materialization against those exact
canonical identities. It accepts an alternate materialization only when it is
an explicitly declared complete-file CRLF/LF identity, size, CR/LF count, and
newline framing supported by the sealed chain. Pulse 35 retains exactly the
Pulse 51 sealed P35/P37 custody variants and the Pulse 37-normalized canonical
LF identities. No generic normalization or arbitrary working-tree-only hash
rule is permitted.

## Historical sole runtime operation and safe-parent contract

The only production runtime call Pulse 61 ever authorized was the exact
exported Pulse 59 callable:

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

It would have been invoked exactly once and consumed the authority on
attempt. Pulse 61 could not directly call Pulse 58, Pulse 57, Pulse 56,
Pulse 52, Pulse 51, Pulse 47, or Pulse 43. There were no retries, resumes,
fallback executors, or republication routes.

Pulse 61 tightened Pulse 60's predecessor contract by requiring the runtime
root to exist and be empty, the P27 cycle root to be an absent direct child,
the Pulse 41 final/stage/rollback paths to be absent and non-overlapping,
the Pulse 59 terminal sibling to be absent, and `ubuntu_runtime_parent` to
be a safe native Linux directory outside `/mnt/*`. Independent prelaunch
review later proved those safe-parent predicates still did not establish the
exact child-creation, restrictive-permission, reversible-cleanup,
same-filesystem-rename, path-length, and executable/noexec prerequisites the
exact helper stack requires.

## Independent prelaunch root-creatability review (2026-08-16)

The exact cutoff Pulse 59/P58/P57/P56/P52/P41/P51 helpers were re-read
without executing any authority or diagnostic callable. That review proved
the sealed Pulse 61 contract still contradicted the exact callable stack in
four ways:

1. Pulse 61 required `private_runtime_root` to exist and be empty, but it did
   not prove creatability/removability of Pulse 58's exact
   `.pulse58-private-launch` namespace child or Pulse 56's exact Windows
   `.p56-*` custody child under that root before any seed;
2. Pulse 61 required the Pulse 41 final parent and absent stage/final/
   rollback paths, but it did not prove same-filesystem stage→final rename,
   available path-length headroom for the exact bound tree, or complete
   reversible cleanup of those exact descendants;
3. Pulse 61 required the Pulse 59 terminal sibling to be absent, but it did
   not prove the runtime parent can create, sync, and remove that exact
   sibling child safely; and
4. Pulse 61 required `ubuntu_runtime_parent` to be a safe native Linux path,
   but it did not prove creation/removal of exact Pulse 57 `.p57-*` bundle
   and Pulse 56 Ubuntu `.p56-*` custody children, nor immediately auditable
   restrictive-permission/executable/noexec prerequisites relevant to the
   native Linux route.

No Pulse 59 callable was invoked. No direct Pulse 58, Pulse 57, Pulse 56,
Pulse 47, Pulse 43, Pulse 41, Pulse 39, or Pulse 27 callable was invoked. No
runtime root, probe, seed, descriptor, candidate process, publication root,
result tree, witness tree, or transfer artifact was created.

## Terminal public transfer and permanent closeout

`published-result` would have permitted transfer of the verified path-free
Pulse 59 public descriptor plus the known Pulse 61 result and witness custody
roots: verified P43 `2/2` result and verified P47 `2/2` witness trees.
`published-failure-witness` would have required the result path to remain
absent and transferred only the verified path-free descriptor plus the known
Pulse 61 witness root for the exact P47 `2/2` witness tree.

Pulse 61 is now permanently withdrawn before launch under
`P61-ROOT-CREATABILITY-CALLABLE-CONTRACT`. Every call, seed, descriptor,
process, publication, transfer, result, and witness count remains zero.
Category, diagnostic, and product conclusions remain null. Retry, resume,
amendment, reinterpretation, and same-authority reuse are prohibited. Any
successor must use a new immutable cutoff that contains this withdrawal and
predates the successor authority.

## Exhaustive control surface

The closed declaration, recursive Draft 2020-12 schema, and exhaustive
deterministic mutation registry remain unchanged at `20058` controls. The
monotonic registry total remains `139725` from the prior `119667`.

See the [Pulse 61 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-61.md).