# Pulse 04: Environment-Bound Onboarding Proof

Status: Authorized; implementation in progress
Implementation authority: One fresh environment-bound proof

## Decision

Run one fresh onboarding proof with the existing toolchain locations explicitly
bound into each invoking process environment. This changes no owner command,
consumer file, Cargo configuration, or product behavior.

The owner explicitly authorized this successor on 2026-09-13 with “continue.”

## Product control

| Control | Record |
|---|---|
| Product outcome | Observe whether two owner-native enterprise consumers can adopt and completely remove pinned Ferris without changing owner correctness. |
| Shortest credible path | Preflight the already-installed platform tools, then run the unchanged protocol without another environment-discovery failure. |
| Maximum budget | One verified WSL PowerShell archive installation, two consumers, two local platforms, one preflight and baseline/onboarding/removal cycle, and one public-safe closeout. |
| Completion | Both unchanged owner commands pass before and after onboarding on Windows and WSL; approved Ferris planning/execution verifies; all consumer onboarding files and the WSL prerequisite are removed. |
| Abandonment | Stop on preflight failure, checksum mismatch, owner baseline failure after successful preflight, product/schema change need, consumer owner-file drift, credential need, or non-removable residue. |
| Product Value Governor | `continue-within-budget` |

## Invocation environment

Before any owner command:

- Windows MUST prepend the existing `C:\Users\giodl\.cargo\bin` directory to
  that process's `PATH` and resolve `cargo`, `rustc`, and `pwsh`;
- WSL MUST preserve `/root/.cargo/bin` on `PATH`, install the same official
  PowerShell 7.6.6 Linux x64 archive only after exact digest verification, and
  resolve `cargo`, `rustc`, `git`, and native `pwsh`; and
- both platforms MUST report Cargo and rustc 1.95.0.

The preflight is environment evidence, not an owner baseline. After it passes,
each unchanged owner command MAY be invoked once per required stage.

## Authorized operations

Pulse 04 MAY:

- repeat Pulse 03's exact verified installation under
  `/opt/ferris-pulse-04/powershell/7.6.6` with
  `/usr/local/bin/pwsh`;
- run each unchanged owner command once on Windows and WSL before onboarding;
- freeze and push one immutable baseline tag per consumer only after all four
  owner baselines pass;
- add only Ferris-owned, explicitly removable onboarding files;
- pin public Ferris revision
  `b347dc34d62810f043122d0d279def316b890cf0` and record each executable digest;
- prepare strict owner-controlled planning, Action Plan, entrypoint, and
  approval inputs without command inference;
- run local `plan`, approved `go`, and `verify` on Windows and WSL;
- remove every Ferris-owned onboarding and generated evidence file;
- rerun each unchanged owner command once per platform; and
- remove the WSL PowerShell symlink, exact installation tree, and archive.

Exact private identities, paths, revisions, tags, timings, and raw outputs MUST
remain in custody.

## Stop conditions and claim limits

Stop rather than expand on preflight failure, checksum mismatch, native `pwsh`
launch failure, owner baseline failure after successful preflight, Cargo or
owner-file mutation, product or schema change, credential, reusable secret,
workflow change, third repository, retry requiring changed input, or cleanup
failure.

Pulse 04 MUST NOT persistently alter the Windows user or machine `PATH`, add an
APT source, install a Snap daemon, translate an owner script, use Windows
PowerShell as Linux evidence, or modify a consumer to accommodate a tool.

This pulse grants no hosted-CI, clean-runner, official Ubuntu support,
production, support, affected-only, performance, savings, workflow-replacement,
or external-adopter claim.

## Validation

- both platform tool-resolution preflights pass before owner invocation;
- the PowerShell archive digest and native process identity match the authority;
- each consumer is clean at its custody revision before onboarding;
- all four unchanged owner baseline commands pass;
- Ferris planning, approved execution, and receipt verification pass separately
  on both platforms;
- every consumer onboarding and generated evidence file is removed;
- all four unchanged post-removal owner commands pass;
- the WSL PowerShell symlink and exact installation tree are absent at closeout;
- both consumer worktrees are clean at their immutable baseline revisions;
- changed Markdown local links and fences pass; and
- `git diff --check` passes.
