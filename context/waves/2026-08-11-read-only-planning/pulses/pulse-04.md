# Pulse 04: Passive Local Doctor

Status: Complete on Windows and Unix; no existing held-out fixture applicable
Implementation authority: Bounded to this document

## Goal

Implement:

```console
ferris doctor --workspace-id <portable-id> --manifest-path <Cargo.toml> [--format human|json]
```

The command passively diagnoses whether the explicit local Ferris planning
prerequisites are present. It reads the selected manifest and invokes only:

```console
cargo --version
```

The probe runs from the system temporary directory with
`RUSTUP_TOOLCHAIN=stable`, `RUSTUP_AUTO_INSTALL=0`,
`RUSTUP_NO_UPDATE_CHECK=1`, and `CARGO_NET_OFFLINE=true`. Any stderr output
blocks a successful passive report.

## Required behavior

- reuse the validated portable workspace-ID contract;
- require an existing readable file named exactly `Cargo.toml`;
- retain only the manifest SHA-256 digest, never its contents or absolute path;
- report a strictly parsed Cargo semantic version and SHA-256 digest over
  framed stdout and stderr;
- emit `ferris.doctor-report/v0`;
- bind report and invocation identity to command, workspace ID, manifest
  digest, and the passive Cargo-version probe;
- expose checks, unknowns, limitations, evidence digests, and fallback in both
  human and JSON output;
- return `invalid` for invalid workspace or manifest input;
- return `blocked` when Cargo cannot start or exits unsuccessfully;
- return `unsupported` when successful Cargo output cannot be safely parsed;
  and
- preserve the existing JSON CLI parse envelope.

## Prohibited behavior

- Cargo metadata, dependency resolution, build scripts, rustc, owner code,
  compilation, tests, linkers, generators, or active probes;
- network, credentials, environment dumps, PATH disclosure, sibling discovery,
  or arbitrary executable selection from the public CLI;
- mutation, durable output, caches, Query Forest state, or support claims;
- affected scope, query, execution, connectors, MCP, AI, approval, or
  deployment; or
- hidden held-out inputs or oracles.

## Acceptance

- successful JSON and human reports preserve the same checks and limitations;
- successful public output contains no absolute checkout path or manifest
  contents;
- Cargo-unavailable and malformed-version negative controls preserve fixed
  result classes and source digests;
- non-manifest files, leading-zero versions, and successful probes with stderr
  are rejected;
- probe configuration tests retain the stable toolchain, offline, no-update,
  no-auto-install, and isolated-working-directory controls;
- invocation and report identities are stable across checkout paths when
  workspace identity, manifest bytes, and Cargo version are equal;
- changed manifest bytes or workspace identity change the report identity;
- Windows and Unix validation, lint, and diff checks pass; and
- an independent review finds no unresolved blocker.

## Removal

Removal is deleting the Ferris binary and transient build output. No manifest,
source, lockfile, configuration, or owner workflow changes are required.

## Held-out classification

The independent custodian verified cutoff
`ba2a055735a5c6bc8530570e270b77684f996d5b`, its tag, and all 12
sealed package digests. All 12 existing fixtures were outside Pulse 04, so
none was executed and no held-out pass is claimed.
