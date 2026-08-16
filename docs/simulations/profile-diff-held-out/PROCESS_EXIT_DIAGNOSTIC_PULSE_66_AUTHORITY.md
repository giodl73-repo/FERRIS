# Pulse 66 independent witness-preserving capability/materialization diagnostic authority

Status: current `authorized-unexecuted` authority
Immutable self-excluding cutoff: `3a99e9e0f383a9821297ef47778fd586b447b7ba`
Declaration identity: `sha256:2cf44e16b0c61d79ed5ac889ab6fbfe46ee693ce6d9ccf2b4528bb877db45034`

Pulse 66 is one fresh independent prelaunch authority over exact final Pulse
59. It is not a retry, resume, amendment, reconstruction, reseed, reuse,
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
`P62-REAL-PATH-WSL-ROUTE-CONTRACT`; Pulse 63 remains permanently withdrawn
`invalid-prelaunch-wsl-bootstrap-contract` under
`P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT`; Pulse 64 remains permanently
withdrawn `invalid-prelaunch-unbound-wsl-qualification-contract` under
`P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION`; and Pulse 65 remains permanently
withdrawn `invalid-prelaunch-wsl-spawn-cardinality-contract` under
`P65-P57-WSL-TWO-SPAWN-CONTRACT`.

## Exact immutable binding

Pulse 66 binds exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 public release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities derive only from immutable Git blobs at cutoff
`3a99e9e0f383a9821297ef47778fd586b447b7ba`, which contains the Pulse 65
withdrawal, exact final Pulse 59, and the exact
`p66_wsl_probe_sealed_dependencies.py` preflight helper while excluding the
Pulse 66 declaration, schema, mutation registry, record, wave entry, and
validator source. Working-tree bytes are not an identity source except for
explicitly declared complete-file LF/CRLF variants.

## Exact public qualification before the sole Pulse 59 call

Pulse 66 keeps the prior conservative actual-root probes, preserves every
undefined-branch prohibition from Pulse 65, and seals exact two-spawn WSL
preflight cardinality.

### Spawn 1: exact stage-bundle `subprocess.run(...)` proof

Before any seed derivation or sole Pulse 59 invocation, public qualification
must first bind the exact Pulse 57 stage-bundle route:

- exact lookup precedence
  `os.environ.get("SystemRoot") or os.environ.get("SYSTEMROOT")`;
- exact public formulas
  `os.fspath(Path(system_root) / "System32")`,
  `os.fspath(Path(system32) / "cmd.exe")`, and
  `os.fspath(Path(system_root) / "System32" / "wsl.exe")`;
- exact public comparison tokens `<resolved-system-root>`,
  `<derived-system32>`, `<derived-comspec>`, and
  `<derived-wsl-executable>` in place of literal placeholder strings;
- exact `_wsl_environment()` allowlist `ComSpec`, `PATH`, and `SystemRoot`,
  compared against those derived concrete values;
- safe absolute non-reparse Windows-directory validation for `system_root` and
  the derived `system32` directory plus exact regular non-reparse identity
  proof for the derived WSL and cmd executables;
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
  and `schema`, empty stderr, and exact derived equality
  `bundle_root == ubuntu_runtime_parent.rstrip("/") + "/" + name`; and
- exact same-invocation removal of the staged bundle with verified absence.

### Spawn 2: exact worker-bootstrap `subprocess.Popen(...)` proof over a probe bundle

The second and only other permitted WSL spawn must prove the exact Pulse 57
worker-bootstrap route over a pre-staged probe bundle, with no retry and no
hidden third spawn:

- exact inline `_WSL_WORKER_BOOTSTRAP` argv ordering for
  `<probe_bundle_root>/worker/wsl_session_worker.py`,
  `sha256:9b0d91f7c4e2aed57d7dc40b95f5860f017138717364d3399d132884047904cb`,
  `--runtime-parent`, `<ubuntu_runtime_parent>`, `--bundle-root`,
  `<probe_bundle_root>`, `--p56-root`, and exact
  `<probe_bundle_root>/repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release`;
- exact probe bundle staged from immutable cutoff blobs without another WSL
  spawn: exact Pulse 57 `wsl_session_worker.py`, exact
  `docs/simulations/profile-diff-held-out/fixtures/p66_wsl_probe_sealed_dependencies.py`
  staged as `worker/sealed_dependencies.py`, and exact Pulse 56
  `retained_build_custody.py` staged only as the path-valid placeholder under
  the exact `--p56-root` directory;
- exact descriptor-open/hash/compile loading, no worker-path reopen, no
  ambient `PYTHONPATH`, no `sitecustomize`, no `WSLENV`, and no mounted import
  route;
- exact worker proof that
  `p56_root == <probe_bundle_root>/repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release`
  and `repo_root = p56_root.parents[3]`;
- exact ready-message schema
  `ferris.pulse-57-wsl-capability-session/v1`, exact platform
  `ubuntu-24.04-x86_64`, exact ready count `69`, exact `2800000`-byte
  protocol bound, exact ready read bound `2800001`, exact ready timeout `15`,
  exact close request write/flush timeout `15`, exact close/wait/terminate/
  kill/drain timeout `5`, and exact cleanup failure on residual output or
  nonzero exit; and
- exact startup-ready-plus-close handshake only. No launch request is sent
  during preflight.

The staged probe dependency is intentionally fake-only. Its `load_exact_p56`
returns a harmless module whose fake publish and fake close calls create no
real Pulse 56 capability, whose `launch_verified` is forbidden and unreached,
and whose use therefore proves only the exact worker-bootstrap/source-loader/
ready/close/cleanup route. Pulse 66 does **not** claim that this second proof
executes production `sealed_dependencies.py`, real Pulse 56 capability
publication, real Pulse 56 launch, or any FERRIS binary.

No direct Pulse 58, Pulse 57, Pulse 56, or FERRIS execution is permitted
during qualification. Retry is forbidden, undefined callable branches are
forbidden, and the exact WSL preflight must account for exactly two WSL
processes and no hidden third spawn.

## Terminal transfer and closeout

If the sole Pulse 59 call succeeds, the only public transfer is the exact
verified Pulse 43 result tree to
`docs/simulations/profile-diff-held-out/pulse-66-public-result/` plus the
exact verified Pulse 47 witness tree to
`docs/simulations/profile-diff-held-out/pulse-66-publication-witness/`, both
described only by the path-free
`ferris.pulse-59-public-transfer-descriptor/v1` descriptor.

If any prelaunch or runtime failure occurs before a valid transfer, the public
conclusion remains permanent `not-attempted`, category/diagnostic/product
conclusions remain null, retry/resume remain forbidden, and unresolved custody
remains fatal.

## Exhaustive control surface

The recursively closed Draft 2020-12 schema freezes the exact declaration as
its `const` authority. The deterministic mutation registry adds `27156`
controls, raising the monotonic repository total from `235150` to `262306`.

See the [Pulse 66 wave record](../../../context/waves/2026-08-12-platform-profile-conformance/pulses/pulse-66.md).
