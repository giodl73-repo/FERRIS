# Pulse 65: witnessed capability/materialization diagnostic authority

Status: Authorized-unexecuted exact one-shot authority at immutable cutoff
`e3b0b62f6dd62b5071886d32a9eedca85c76b4ae`

## Goal

Authorize one fresh independent future diagnostic over exact final Pulse 59
while permanently preserving the Pulse 64 withdrawal, removing every
undefined qualification branch, and sealing only the implemented exact Pulse
57 WSL stage-bundle and worker-bootstrap route before any sole Pulse 59 call.

## Authority

Pulse 65 binds exact final Pulse 59 HEAD
`6945f5fc96868c97267a1635fbb5219cc398eeb4`, the complete exact P27/P31/P35/
P37/P39/P41/P43/P47/P51/P52/P56/P57/P58/P59 release chain, and only
`run_witness_preserving_capability_materialization_executor`. Canonical
identities are derived only from immutable cutoff Git blobs at self-excluding
cutoff `e3b0b62f6dd62b5071886d32a9eedca85c76b4ae`, which contains the Pulse 64
withdrawal and predates the authority. Runtime materialization is limited to
those identities or explicitly declared complete-file LF/CRLF variants.

The sealed declaration requires fresh anonymous exact-cutoff
`core.autocrlf=false` authority and Pulse 39 checkouts, separate independent
checkout validation, one exact Pulse 59 production call, path-free terminal
custody transfer, permanent null-conclusion `not-attempted` closeout for any
prelaunch failure, and fatal unresolved-custody cleanup posture. It is not a
retry, resume, amendment, reconstruction, reseed, reuse, correlation, or
inference of any permanently closed predecessor.

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
`P63-P57-EXACT-WSL-BOOTSTRAP-CONTRACT`; and Pulse 64 remains permanently
withdrawn `invalid-prelaunch-unbound-wsl-qualification-contract` under
`P64-WSL-QUALIFIER-SYSTEMROOT-DERIVATION`. Pulse 65 does not revive,
reinterpret, or consume any of those historical closures.

## Exact pre-call public qualification

Before any seed derivation or sole Pulse 59 invocation, Pulse 65 requires all
public prerequisites to succeed and to stop the authority immediately on any
failure:

1. actual caller-supplied basenames plus conservative deepest exact topology
   proofs for the Pulse 41 final root, the Pulse 56 Windows custody/runtime
   stack, the Pulse 58 namespace, and the Pulse 59 terminal sibling route;
2. exact Pulse 57 WSL qualification only through the same implemented
   stage-bundle and worker-bootstrap route, proving all of the following:
   - exact `SystemRoot`/`SYSTEMROOT` source-precedence lookup
     `os.environ.get("SystemRoot") or os.environ.get("SYSTEMROOT")`;
   - exact public formulas
     `os.fspath(Path(system_root) / "System32")`,
     `os.fspath(Path(system32) / "cmd.exe")`, and
     `os.fspath(Path(system_root) / "System32" / "wsl.exe")`, compared only
     through declared derived tokens rather than literal placeholder strings;
   - safe absolute non-reparse Windows-directory proof for `system_root` and
     the derived `system32` directory, plus exact regular non-reparse identity
     proof for the derived WSL and cmd executables;
   - exact `_wsl_environment()` allowlist `ComSpec`, `PATH`, and `SystemRoot`
     with public comparison tokens bound to the derived concrete values;
   - exact `--distribution Ubuntu-24.04 --exec /usr/bin/python3 -I -S -B -c`
     shape with inline `_WSL_BUNDLE_BOOTSTRAP` and post-bootstrap argv
     `(<ubuntu_runtime_parent>, <name>)`;
   - exact `.p57-` plus 32 lowercase hex bundle-name rule;
   - exact staged file set `worker/wsl_session_worker.py`,
     `worker/sealed_dependencies.py`, and the complete exact Pulse 56 release
     under actual
     `repository/docs/simulations/profile-diff-held-out/pulse-56-retained-build-custody-release/`;
   - exact canonical stdout fields `bundle_root`, `python`, and `schema` with
     `bundle_root == ubuntu_runtime_parent.rstrip("/") + "/" + name`;
   - exact `_WSL_WORKER_BOOTSTRAP` argv ordering for worker source, worker
     SHA-256, `--runtime-parent`, `--bundle-root`, and exact `--p56-root`;
   - exact descriptor-open/hash/compile source loading, bundled
     `sealed_dependencies.py`, exact `p56_root` equality, and
     `repo_root = p56_root.parents[3]`;
   - a custodian-private compare-only record for the actual
     `ubuntu_runtime_parent` path plus concrete Windows derivation values and
     SHA-256 path digests, with no durable public path serialization; and
   - exact probe-bundle removal plus verified absence in the same invocation;
3. no direct Pulse 58/Pulse 57/Pulse 56/FERRIS execution during qualification,
   no retry, no undefined callable branch, and no alternate schema-path
   substitution.

## Terminal transfer and closeout

If the sole Pulse 59 call succeeds, the only public transfer is the exact
verified Pulse 43 result tree to
`docs/simulations/profile-diff-held-out/pulse-65-public-result/` and the exact
verified Pulse 47 witness tree to
`docs/simulations/profile-diff-held-out/pulse-65-publication-witness/`,
described only by the path-free
`ferris.pulse-59-public-transfer-descriptor/v1` descriptor. Any prelaunch or
runtime failure remains permanent `not-attempted`, non-retryable, and
non-resumable with null category/diagnostic/product conclusions.

## Evidence

- [Authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-65-authority.json)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-65-authority.v1.schema.json)
- [Mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-65-authority-mutations.json)
- [Authority record](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_65_AUTHORITY.md)
- [Authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_65_authority.rs)

Declaration identity:
`sha256:5bd7c876180a3bfb9f0bcb1518ef68921d1b28210d1f717c904753508e28abb0`.
The declaration adds `25815` deterministic controls, raising the monotonic
registry total to `235150`.
