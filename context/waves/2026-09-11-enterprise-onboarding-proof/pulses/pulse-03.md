# Pulse 03: WSL Prerequisite and Resumed Onboarding

Status: Complete; incomplete at Windows Cargo PATH gate
Implementation authority: Exhausted

## Decision

Install one checksum-verified, removable Linux PowerShell prerequisite in the
existing local WSL 2 Ubuntu environment, then resume the exact owner-native
baseline, pinned Ferris onboarding, approved execution, verification, complete
consumer removal, and post-removal validation that Pulse 02 could not begin.

The owner explicitly authorized this successor on 2026-09-13 with “lets do it.”

## Research basis

Microsoft's
[`alternate installation guidance`](https://learn.microsoft.com/en-us/powershell/scripting/install/alternate-install-methods?view=powershell-7.6)
states that the binary archive method can install PowerShell on Linux versions
that are not officially supported and defines manual removal. The official
PowerShell
[`v7.6.6 release`](https://github.com/PowerShell/PowerShell/releases/tag/v7.6.6)
publishes `powershell-7.6.6-linux-x64.tar.gz`. The release API records SHA-256
digest `ddbc4a2d113bbd46d283cfedcbcd117a70caefd7673f41f2b4e0000badf103bc`.

## Product control

| Control | Record |
|---|---|
| Product outcome | Observe whether two owner-native enterprise consumers can adopt and completely remove pinned Ferris without changing owner correctness. |
| Shortest credible path | Add the one missing Linux command prerequisite without changing either owner command, then execute the already-defined proof. |
| Maximum budget | One verified PowerShell archive installation, two consumers, two local platforms, one baseline/onboarding/removal cycle, and one public-safe closeout. |
| Completion | Both unchanged owner commands pass before and after onboarding on Windows and WSL; approved Ferris planning/execution verifies; all consumer onboarding files and the WSL prerequisite are removed. |
| Abandonment | Stop on checksum mismatch, prerequisite launch failure, owner baseline failure, product/schema change need, consumer owner-file drift, credential need, or non-removable residue. |
| Product Value Governor | `continue-within-budget` |

## Authorized environment change

Pulse 03 MAY:

- download only the official PowerShell 7.6.6 Linux x64 archive;
- verify its exact published SHA-256 digest before extraction;
- install it under `/opt/ferris-pulse-03/powershell/7.6.6`;
- create `/usr/local/bin/pwsh` pointing to that exact executable;
- remove the downloaded archive immediately after verification and extraction;
- retain the installed prerequisite only through post-removal owner validation;
  and
- remove the symlink and exact `/opt/ferris-pulse-03` tree at closeout.

It MUST NOT add an APT source, install a Snap daemon, change a system package,
translate an owner script, use Windows PowerShell as Linux evidence, modify a
consumer to accommodate PowerShell, or claim official Ubuntu 26.04 support.

## Resumed consumer authority

After `pwsh` launches natively on Linux, Pulse 03 MAY:

- run the unchanged `EO-01` and `EO-02` owner commands once on Windows and once
  on local WSL 2 Ubuntu;
- freeze and push one immutable baseline tag per repository only after all four
  owner baselines pass;
- add only Ferris-owned, explicitly removable onboarding files;
- pin public Ferris revision
  `b347dc34d62810f043122d0d279def316b890cf0` and record each executable digest;
- prepare strict owner-controlled planning, Action Plan, entrypoint, and
  approval inputs without command inference;
- run local `plan`, approved `go`, and `verify` on Windows and WSL;
- remove every Ferris-owned onboarding and generated evidence file; and
- rerun each unchanged owner command once per platform.

Exact private identities, paths, revisions, tags, timings, and raw outputs MUST
remain in custody.

## Stop conditions and claim limits

Stop rather than expand on any checksum mismatch, native `pwsh` launch failure,
owner baseline failure, Cargo or owner-file mutation, product or schema change,
credential, reusable secret, remote fetch beyond the one public archive,
workflow change, third repository, retry requiring changed input, or cleanup
failure.

This pulse grants no hosted-CI, clean-runner, official Ubuntu support,
production, support, affected-only, performance, savings, workflow-replacement,
or external-adopter claim.

## Validation

- the PowerShell archive digest and native process identity match the authority;
- each consumer remains clean at its custody revision before onboarding;
- all four unchanged owner baseline commands pass;
- Ferris planning, approved execution, and receipt verification pass separately
  on both platforms;
- every consumer onboarding and generated evidence file is removed;
- all four unchanged post-removal owner commands pass;
- the WSL PowerShell symlink and exact installation tree are absent at closeout;
- both consumer worktrees are clean at their immutable baseline revisions;
- changed Markdown local links and fences pass; and
- `git diff --check` passes.

## Result

The official PowerShell 7.6.6 Linux x64 archive matched published SHA-256
digest `ddbc4a2d113bbd46d283cfedcbcd117a70caefd7673f41f2b4e0000badf103bc`.
The extracted executable was an ELF x86-64 binary and launched natively with
PowerShell reporting platform `Unix`, operating system `Ubuntu 26.04 LTS`, and
architecture `X64`.

The first two Windows owner baseline invocations both exited at their initial
`Get-Command cargo` statement because `cargo` was absent from the inherited
`PATH`. No formatting, Clippy, test, metadata, or topology command ran. The
consumer worktrees remained clean.

The changed-input retry stop applied. Pulse 03 did not alter `PATH`, rerun
either command, run a WSL baseline, create a tag, mutate a consumer, create an
Action Plan, invoke Ferris in a consumer, or test removal. The temporary
`/usr/local/bin/pwsh` symlink, exact `/opt/ferris-pulse-03` tree, and downloaded
archive were removed and verified absent.

## Closeout

The PowerShell prerequisite was proven viable and removable, but the
onboarding and removal hypothesis remains not observed. The Windows result is
an invocation-environment unavailability, not an owner correctness failure and
not a Ferris product failure.

Pulse 03 is exhausted and grants no retry with a changed `PATH`, environment
setup, successor pulse, consumer mutation, product change, hosted-CI, official
Ubuntu support, production, support, affected-only, performance, savings, or
workflow-replacement authority.
