# Pulse 63 independent witness-preserving capability/materialization diagnostic authority

Status: `authorized-unexecuted`
Immutable self-excluding cutoff: `5ad78a0623611ad57797ec4e9da34345b40a6e38`
Declaration identity: `sha256:b8cfea5cc8cb6dc52a7974f4fee35f6351557158943cc92af388c534421915d5`

Pulse 63 is one fresh independent diagnostic authority. It is not a retry,
resume, amendment, reconstruction, reseed, reuse, correlation, or inference of
any permanently closed predecessor. Pulse 46 and Pulse 48 remain permanently
`invalid-publication-integrity`; Pulse 49 remains permanently withdrawn
`invalid-prelaunch-authority-integrity`; Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`; Pulse 54 remains permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; Pulse 55 remains permanently
closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`; Pulse 60 remains permanently withdrawn
`invalid-prelaunch-runtime-root-contract` under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`; Pulse 61 remains permanently withdrawn
`invalid-prelaunch-root-creatability-contract` under
`P61-ROOT-CREATABILITY-CALLABLE-CONTRACT`; and Pulse 62 remains permanently
withdrawn `invalid-prelaunch-path-route-contract` under
`P62-REAL-PATH-WSL-ROUTE-CONTRACT`, with zero calls, seeds, descriptors,
processes, publications, and transfers plus null conclusions. None of those
historical dispositions is cured, consumed, amended, or reinterpreted here.

The cutoff is the exact withdrawal commit containing Pulse 62 closeout and
excluding this authority, its schema, mutation registry, authority record,
wave pulse record, and validator. No Pulse 63 runtime, seed, descriptor,
result, or witness artifact exists at authorization.

## Exact immutable binding and checkout validation

The canonical declaration binds the complete exact P27/P31/P35/P37/P39/P41/
P43/P47/P51/P52/P56/P57/P58/P59 release chain, including full cutoff-tree path
sets, canonical identities, manifests, receipts, seals, source files, and the
exact P56/P57/P58/P59 callable signatures. Every canonical identity is derived
by the validator from the immutable Git blob at cutoff
`5ad78a0623611ad57797ec4e9da34345b40a6e38`; local working-tree bytes are never
an identity source. The cutoff itself proves Pulse 62's permanent withdrawal
before any new authority is introduced.

The validator separately validates runtime materialization against those exact
canonical identities. It accepts an alternate materialization only when it is
an explicitly declared complete-file CRLF/LF identity, size, CR/LF count, and
newline framing supported by the sealed chain. Pulse 35 retains exactly the
Pulse 51 sealed P35/P37 custody variants and the Pulse 37-normalized canonical
LF identities. The actual authorized runtime posture remains fresh anonymous
`core.autocrlf=false` authority and P39 checkouts. No generic normalization or
arbitrary working-tree-only hash rule is permitted.

## Exact path qualification, WSL route preflight, and sole runtime operation

The only production runtime call permitted by Pulse 63 is the exact exported
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

It is invoked exactly once and consumes the authority on attempt. Pulse 63
MUST NOT directly call Pulse 58, Pulse 57, Pulse 56, Pulse 52, Pulse 51,
Pulse 47, or Pulse 43. There are no retries, resumes, fallback executors, or
republication routes.

Before that unperformed call, independent custody MUST satisfy the exact public
root contract proved by the sealed helpers and MUST complete the non-consuming
public preflight contract sealed in the declaration:

1. obtain the authority anonymously and materialize `repo_root` as the same
   fresh anonymous exact-cutoff clean `core.autocrlf=false` Windows checkout as
   the authority checkout at head `5ad78a0623611ad57797ec4e9da34345b40a6e38`;
2. materialize `p39_checkout_root` as a separate fresh anonymous exact-cutoff
   clean `core.autocrlf=false` Windows checkout, validated only against
   immutable cutoff blobs and supported declared checkout variants;
3. use the actual caller-supplied basenames and exact or conservative deepest
   path shapes the bound helpers create:
   - under the actual `private_runtime_root`, prove exact `.pulse58-private-launch`,
     `.pulse58-private-launch/seed.bin`, and
     `.pulse58-private-launch/descriptors/case-manifest.json` creatability plus
     the exact-length Pulse 56 Windows runtime topology, including
     `work/checkout-a`, `work/checkout-b`, both deepest exact
     `docs/simulations/profile-diff-held-out/pulse-59-witness-preserving-capability-materialization-executor-release/schemas/ferris.pulse-59-witness-preserving-capability-materialization-executor.v1.schema.json`
     checkout leaves, conservative `work/target-*` probe paths, the exact
     retained artifact/receipt names, `launches/l-0123456789abcdef01234567`,
     `home`, and `tmp`, or fail before call if no explicit bound can be proven;
   - under the actual `p41_final_root.parent`, reuse the actual
     `p41_final_root.name`, its exact derived
     `.<final-root-name>.pulse-41-stage` stage root, the same rollback/final
     basename, and the deepest real Pulse 39 relative path
     `tests/test_checkout_verifier.py` while proving same-filesystem rename,
     path-length headroom, rollback cleanup, and complete absence recovery; and
   - derive the exact Pulse 59 terminal sibling as
     `<private_runtime_root.name>.pulse59-terminal-publication` under the real
     runtime parent, not a synthetic substitute;
4. run exactly one harmless WSL route preflight before any real Ubuntu
   capability build or Pulse 59 call by resolving Windows
   `%SystemRoot%\\System32\\wsl.exe` and invoking exactly
   `--distribution Ubuntu-24.04 --exec /usr/bin/python3 -I -S -B` with an
   isolated bounded script that:
   - revalidates `ubuntu_runtime_parent` as an absolute native Linux path
     outside `/mnt/*` and never serializes it publicly;
   - verifies platform identity `ubuntu-24.04-x86_64` and exact Python
     identity `/usr/bin/python3`;
   - creates, fsyncs where supported, removes, and re-verifies absence for the
     exact `.p57-0123456789abcdef0123456789abcdef` bundle topology
     (`worker/wsl_session_worker.py`, `worker/sealed_dependencies.py`, and the
     deepest exact bundled repository path) plus the exact-length conservative
     Pulse 56 Ubuntu runtime topology under the real parent; and
   - emits one canonical bounded JSON line on stdout, no stderr, with explicit
     input, output, environment, timeout, and single-spawn limits; and
5. stop before any seed and before the sole Pulse 59 call on any failure in
   the Windows path-topology or native-Linux WSL-route prerequisite families.

No seed, descriptor root, candidate process, result, or witness artifact may
exist first. No real FERRIS execution, Pulse 56 callable execution, or Pulse 59
consumption is permitted during this prelaunch qualification audit.

## Terminal public transfer and permanent closeout

`published-result` alone permits transfer of the verified path-free Pulse 59
public descriptor plus the known Pulse 63 result and witness custody roots:
verified P43 `2/2` result and P47 `2/2` witness trees.
`published-failure-witness` requires the result path to remain absent and
transfers only the verified path-free descriptor plus the known Pulse 63
witness root for the exact P47 `2/2` witness tree.

Any prelaunch or runtime failure that leaves publication `not-attempted`
permanently closes Pulse 63 with null category, diagnostic, and product
conclusions; no result or witness transfer is permitted.
`invalid-witness-publication` also transfers nothing and makes no success
claim. Pulse 59 `terminal-publication-cleanup-indeterminate` is a fatal
unresolved-custody posture: it transfers nothing, carries null conclusions,
and permits no completed diagnostic claim. Pulse 63 never exposes private
seed, descriptor, runtime, or source-path material publicly.

## Exhaustive control surface

The closed declaration, recursive Draft 2020-12 schema, and exhaustive
deterministic mutation registry contain `23266` controls. The monotonic
registry total is `184635` from the prior `161369`.

See the [Pulse 63 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-63.md).
