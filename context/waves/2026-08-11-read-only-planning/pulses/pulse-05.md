# Pulse 05: Passive Doctor Hardening

Status: Validated on Windows and Unix; replacement scoring pending
Implementation authority: Corrective only; no capability expansion

## Goal

Correct all four findings from the post-completion Pulse 04 review without
expanding the read-only command surface.

## Required behavior

- run Cargo metadata and passive doctor from the selected manifest directory;
- preserve inherited owner toolchain selection while setting
  `CARGO_NET_OFFLINE=true`, `RUSTUP_AUTO_INSTALL=0`, and
  `RUSTUP_NO_UPDATE_CHECK=1`;
- bind those owner-toolchain semantics into evidence and invocation identity;
- read at most 1 MiB from the selected doctor manifest;
- stop the doctor probe after five seconds;
- retain at most 64 KiB from each doctor stdout and stderr stream;
- return `blocked` rather than success when any resource bound is exceeded;
- bind framed Cargo stdout and stderr digest into doctor report and invocation
  identities;
- use the manifest digest, rather than checkout path, for doctor failures that
  occur after the manifest is read; and
- preserve existing privacy, fixed result classes, human/JSON parity, and
  passive-command boundaries.

## Prohibited behavior

- new commands or schemas;
- owner execution beyond existing Cargo metadata and `cargo --version`;
- network, rustup installation/update, active probes, mutation, sibling
  discovery, arbitrary public executable selection, or environment dumps;
- affected scope, query, execution, connectors, MCP, AI, approval, or
  deployment; or
- reuse of superseded held-out results for the corrected executable.

## Acceptance

- doctor and metadata commands share the same owner toolchain context;
- manifest, process-time, stdout, and stderr bounds have negative tests;
- different owner output with the same Cargo semantic version changes doctor
  report and invocation identities;
- equivalent post-read failures in different checkouts share invocation
  identity;
- Windows and Unix formatting, tests, lint, and diff checks pass;
- no post-fix review blocker remains; and
- a new immutable cutoff is independently classified and scored.
