# Pulse 05: Isolated WSL Cargo-Home Proof

Status: Authorized; implementation in progress
Implementation authority: One fresh configuration-isolated proof

## Decision

Run one fresh onboarding proof with WSL Cargo configuration isolated from the
Windows-hosted user Cargo home. Preserve the user's configuration unchanged,
reuse the installed Rust 1.95 toolchain, and remove all temporary environment
state at closeout.

The owner explicitly authorized this successor on 2026-09-13 with “continue.”

## Product control

| Control | Record |
|---|---|
| Product outcome | Observe whether two owner-native enterprise consumers can adopt and completely remove pinned Ferris without changing owner correctness. |
| Shortest credible path | Use a temporary empty WSL Cargo home so unrelated host configuration cannot select an unavailable Linux compiler wrapper. |
| Maximum budget | One temporary Cargo home, one verified WSL PowerShell archive installation, two consumers, two local platforms, one baseline/onboarding/removal cycle, and one public-safe closeout. |
| Completion | Both unchanged owner commands pass before and after onboarding on Windows and WSL; approved Ferris planning/execution verifies; consumer onboarding files and all temporary WSL state are removed. |
| Abandonment | Stop on isolated preflight failure, owner baseline failure, product/schema change need, consumer owner-file drift, credential need, or non-removable residue. |
| Product Value Governor | `continue-within-budget` by explicit owner reprioritization |

## Isolated environment

Pulse 05 MAY:

- create exact empty directory `/opt/ferris-pulse-05/cargo-home`;
- set `CARGO_HOME` to that directory only for WSL preflight, owner commands, and
  Ferris execution;
- keep `/root/.cargo/bin` on `PATH` and `/root/.rustup` unchanged;
- repeat the exact checksum-verified PowerShell 7.6.6 archive installation under
  `/opt/ferris-pulse-05/powershell/7.6.6`;
- prepend the existing Windows Cargo directory only to Windows invoking
  processes; and
- remove `/opt/ferris-pulse-05`, `/usr/local/bin/pwsh`, and the archive at
  closeout.

It MUST NOT copy registry state or configuration into the temporary Cargo home,
edit or override the user's Cargo configuration, install dependencies, alter a
consumer to accommodate the environment, or persistently change `PATH`.

## Consumer protocol

After both platform preflights pass, Pulse 05 MAY:

- run each unchanged owner command once per platform before onboarding;
- freeze and push one immutable baseline tag per consumer only after all four
  baselines pass;
- add only Ferris-owned, explicitly removable onboarding files;
- pin public Ferris revision
  `b347dc34d62810f043122d0d279def316b890cf0` and record executable digests;
- prepare strict owner-controlled planning, Action Plan, entrypoint, and
  approval records without command inference;
- run local `plan`, approved `go`, and `verify` on Windows and WSL;
- remove every Ferris-owned onboarding and generated evidence file; and
- rerun each unchanged owner command once per platform.

Exact private identities, paths, revisions, tags, timings, and raw outputs MUST
remain in custody.

## Stop conditions and claim limits

Stop rather than expand on preflight failure, checksum mismatch, owner baseline
failure, missing offline material in the empty Cargo home, Cargo or owner-file
mutation, product or schema change, credential, reusable secret, workflow
change, third repository, retry requiring changed input, or cleanup failure.

This pulse grants no dependency installation, hosted-CI, clean-runner, official
Ubuntu support, production, support, affected-only, performance, savings,
workflow-replacement, or external-adopter claim.

## Validation

- both platform preflights pass before owner invocation;
- WSL proves the temporary Cargo home is empty and the user configuration is
  unchanged;
- all four unchanged owner baselines pass;
- Ferris planning, approved execution, and receipt verification pass separately
  on both platforms;
- every consumer onboarding and generated evidence file is removed;
- all four unchanged post-removal owner commands pass;
- all Pulse 05 WSL files and symlinks are absent at closeout;
- both consumer worktrees are clean at their immutable baseline revisions;
- changed Markdown local links and fences pass; and
- `git diff --check` passes.
