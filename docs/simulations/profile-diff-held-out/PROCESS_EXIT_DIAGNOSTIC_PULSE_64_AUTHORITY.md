# Pulse 64 independent witness-preserving capability/materialization diagnostic authority

Status: current `authorized-unexecuted` authority
Immutable self-excluding cutoff: `2388b7d9a5fda7f9cbf772e12d1b4c07d22f2161`
Declaration identity: `sha256:634e7b3197f5d550c6f3816dbf13770d44738c4f05de6956aa07966548a0be23`

Pulse 64 is one fresh independent prelaunch authority over exact final Pulse 59.
It is not a retry, resume, amendment, reconstruction, reseed, reuse,
correlation, or inference of any permanently closed predecessor.

## Permanent predecessor closure

Pulse 46 and Pulse 48 remain permanently `invalid-publication-integrity`.
Pulse 49 remains permanently withdrawn `invalid-prelaunch-authority-integrity`;
Pulse 50 remains permanently withdrawn
`invalid-prelaunch-infrastructure-integrity`; Pulse 54 remains permanently
withdrawn `invalid-prelaunch-checkout-variant-integrity` under
`P54-CHECKOUT-VARIANT-VALIDATOR-INTEGRITY`; Pulse 55 remains permanently
closed `terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`; Pulse 60 remains permanently withdrawn
`invalid-prelaunch-runtime-root-contract` under
`P60-RUNTIME-ROOT-CALLABLE-CONTRACT`; Pulse 61 remains permanently withdrawn
`invalid-prelaunch-root-creatability-contract` under
`P61-ROOT-CREATABILITY-CALLABLE-CONTRACT`; Pulse 62 remains permanently
withdrawn `invalid-prelaunch-path-route-contract` under
`P62-REAL-PATH-WSL-ROUTE-CONTRACT`; and Pulse 63 remains permanently withdrawn
`invalid-prelaunch-wsl-bootstrap-contract` under
`P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT`.

## Exact immutable binding

Pulse 64 binds exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 public release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities derive only from immutable Git blobs at cutoff
`2388b7d9a5fda7f9cbf772e12d1b4c07d22f2161`, which contains the Pulse 63
withdrawal and excludes the Pulse 64 declaration, schema, mutation registry,
record, wave entry, and validator source. Working-tree bytes are not an
identity source except for explicitly declared complete-file LF/CRLF variants.

## Exact public qualification before the sole Pulse 59 call

Pulse 64 keeps the prior conservative actual-root probes and additionally
seals the full exact Pulse 57 WSL bootstrap route. Before any seed derivation
or sole Pulse 59 invocation, public qualification must:

1. prove actual caller-supplied basenames and conservative deepest exact
   topology for:
   - Pulse 41 stage/final/rollback rename topology over
     `tests/test_checkout_verifier.py`;
   - Pulse 56 Windows checkout/target/custody paths;
   - the Pulse 58 private namespace leaf names; and
   - the Pulse 59 terminal sibling derived from
     `<private_runtime_root.name>.pulse59-terminal-publication`;
2. invoke exactly one harmless bounded proof of the exact Pulse 57 WSL route,
   either by using the same exact stage-bundle bootstrap route or by calling
   the public qualification callable
   `qualify_exact_p57_wsl_bootstrap_contract`;
3. stop before the Pulse 59 callable on any path, route, source-loader,
   privacy, cleanup, or bound failure.

### Exact Pulse 57 stage-bundle route

The WSL qualification must bind the exact source route:

- exact `%SystemRoot%\\System32\\wsl.exe`;
- exact `_wsl_environment()` allowlist `ComSpec`, `PATH`, `SystemRoot` with
  exact values `%SystemRoot%\\System32\\cmd.exe`, `%SystemRoot%\\System32`,
  and `%SystemRoot%`;
- exact `--distribution Ubuntu-24.04 --exec /usr/bin/python3 -I -S -B -c`
  argv shape with inline `_WSL_BUNDLE_BOOTSTRAP` and exact post-bootstrap
  arguments `(<ubuntu_runtime_parent>, <name>)`;
- exact canonical payload schema `ferris.pulse-57-wsl-bundle/v1` bounded by
  `1048576` bytes;
- exact bundle-name rule `.p57-` plus 32 lowercase hex characters;
- exact staged file set of 12 files:
  `worker/wsl_session_worker.py`, `worker/sealed_dependencies.py`, and the
  full exact Pulse 56 release under actual
  `repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/`;
- exact canonical stdout schema
  `ferris.pulse-57-wsl-bundle-staged/v1` with only `bundle_root`, `python`,
  and `schema`, no stderr, and exact derived equality
  `bundle_root == ubuntu_runtime_parent.rstrip("/") + "/" + name`; and
- exact same-invocation removal of the probe bundle with verified absence.

The authority does not permit a misleading staged Pulse 59 schema path in this
contract.

### Exact worker-bootstrap, source-loader, and private-parent contract

The same exact proof must also bind:

- exact inline `_WSL_WORKER_BOOTSTRAP` argv ordering for
  `<bundle_root>/worker/wsl_session_worker.py`,
  `sha256:9b0d91f7c4e2aed57d7dc40b95f5860f017138717364d3399d132884047904cb`,
  `--runtime-parent`, `<ubuntu_runtime_parent>`, `--bundle-root`,
  `<bundle_root>`, `--p56-root`, and exact
  `<bundle_root>/repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release`;
- exact ready-message schema
  `ferris.pulse-57-wsl-capability-session/v1`, exact platform
  `ubuntu-24.04-x86_64`, and exact ready count `69`;
- exact descriptor-open/hash/compile loading, no worker-path reopen, no
  ambient `PYTHONPATH`, no `sitecustomize`, no `WSLENV`, and no mounted import
  route;
- exact bundled `sealed_dependencies.py` load only;
- exact worker proof that
  `p56_root == <bundle_root>/repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release`
  and `repo_root = p56_root.parents[3]`; and
- a custodian-private compare-only record that may hold the actual caller
  `ubuntu_runtime_parent` path and compare it to the observed path while the
  durable public record emits only status or digest material if needed.

No direct Pulse 58, Pulse 57, Pulse 56, or FERRIS execution is permitted
during qualification. Retry is forbidden.

## Terminal transfer and closeout

If the sole Pulse 59 call succeeds, the only public transfer is the exact
verified Pulse 43 result tree to
`docs/simulations/profile-diff-held-out/pulse-64-public-result/` plus the
exact verified Pulse 47 witness tree to
`docs/simulations/profile-diff-held-out/pulse-64-publication-witness/`, both
described only by the path-free
`ferris.pulse-59-public-transfer-descriptor/v1` descriptor.

If any prelaunch or runtime failure occurs before a valid transfer, the public
conclusion remains permanent `not-attempted`, category/diagnostic/product
conclusions remain null, retry/resume remain forbidden, and unresolved custody
remains fatal.

## Exhaustive control surface

The recursively closed Draft 2020-12 schema freezes the exact declaration as
its `const` authority. The deterministic mutation registry adds `24700`
controls, raising the monotonic repository total from `184635` to `209335`.

See the [Pulse 64 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-64.md).
