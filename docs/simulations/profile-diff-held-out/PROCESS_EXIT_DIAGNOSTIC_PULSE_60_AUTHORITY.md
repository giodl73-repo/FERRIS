# Pulse 60 independent witness-preserving capability/materialization diagnostic authority

Status: `authorized-unexecuted` historical declaration; permanently withdrawn
before launch
Immutable self-excluding cutoff: `6945f5fc96868c97267a1635fbb5219cc398eeb4`
Declaration identity: `sha256:13ba3aaa5d61c536a9dd22b3a57816b1b7d93c2e11592c87117190709cbfb40c`

Pulse 60 remains the exact historical one-shot authority declaration. It is
not amended or reissued here. Pulse 46 and Pulse 48 remain permanently
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

## Exact immutable historical binding

The canonical declaration binds the complete exact P27/P31/P35/P37/P39/P41/
P43/P47/P51/P52/P56/P57/P58/P59 release chain, including full cutoff-tree path
sets, canonical identities, manifests, receipts, seals, source files, and the
exact P56/P57/P58/P59 callable signatures. Every canonical identity is derived
from the immutable Git blob at cutoff
`6945f5fc96868c97267a1635fbb5219cc398eeb4`; local working-tree bytes are never
an identity source.

The validator separately validates runtime materialization against those exact
canonical identities. It accepts an alternate materialization only when it is
an explicitly declared complete-file CRLF/LF identity, size, CR/LF count, and
newline framing supported by the sealed chain. Pulse 35 retains exactly the
Pulse 51 sealed P35/P37 custody variants and the Pulse 37-normalized canonical
LF identities. No generic normalization or arbitrary working-tree-only hash
rule is permitted.

## Historical sole runtime operation

The only production runtime call Pulse 60 ever authorized was the exact
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

It would have been invoked exactly once and consumed the authority on attempt.
Pulse 60 could not directly call Pulse 58, Pulse 57, Pulse 56, Pulse 52,
Pulse 51, Pulse 47, or Pulse 43. There were no retries, resumes, fallback
executors, or republication routes.

## Independent prelaunch contract review (2026-08-16)

The exact cutoff Pulse 58/P59/P52/P41/P57/P56 helpers were re-read without
executing any authority or diagnostic callable. That review proved the sealed
Pulse 60 root contract contradicted the exact callable stack:

1. Pulse 60 declared `private_runtime_root` as `fresh-absent`, while Pulse 58
   first calls `p51._safe_runtime_root(...)` and then requires an existing
   empty safe runtime directory;
2. Pulse 60 declared `p27_cycle_root` only as `fresh`, while Pulse 58 requires
   an absent direct child of the runtime root and rejects any other placement;
   and
3. Pulse 60 underbound the exact Pulse 41 final/stage and Pulse 59 terminal
   sibling separation needed for the one-call route to avoid prelaunch or
   post-cleanup `not-attempted` closeout.

No Pulse 59 callable was invoked. No direct Pulse 58, Pulse 57, Pulse 56,
Pulse 47, Pulse 43, Pulse 41, Pulse 39, or Pulse 27 callable was invoked. No
runtime root, seed, descriptor, candidate process, publication root, result
tree, witness tree, or transfer artifact was created.

## Terminal public transfer and permanent closeout

`published-result` would have permitted transfer of the verified path-free
Pulse 59 public descriptor plus the known Pulse 60 result and witness custody
roots: verified P43 `2/2` result and verified P47 `2/2` witness trees.
`published-failure-witness` would have required the result path to remain
absent and transferred only the verified path-free descriptor plus the known
Pulse 60 witness root for the exact P47 `2/2` witness tree.

Pulse 60 is now permanently withdrawn before launch under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`. Every call, seed, descriptor, process,
publication, transfer, result, and witness count remains zero. Category,
diagnostic, and product conclusions remain null. Retry, resume, amendment,
reinterpretation, and same-authority reuse are prohibited. Any successor must
use a new immutable cutoff that contains this withdrawal and predates the
successor authority.

## Exhaustive control surface

The closed declaration, recursive Draft 2020-12 schema, and exhaustive
deterministic mutation registry remain unchanged at `19085` controls. The
monotonic registry total remains `119667` from the prior `100582`.

See the [Pulse 60 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-60.md).
