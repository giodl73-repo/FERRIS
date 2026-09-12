# Pulse 02: Pinned Ferris Onboarding and Removal

Status: Complete; incomplete at WSL PowerShell capability gate
Implementation authority: Exhausted

## Decision

Determine whether the frozen `EO-01` and `EO-02` owner baselines can adopt one
pinned public Ferris revision, prepare explicit owner-controlled planning and
Action Plan inputs, exercise `plan`, `go`, and `verify` on Windows and local
WSL 2 Ubuntu, retain all owner checks, and remove Ferris without changing owner
correctness.

The owner explicitly authorized this replacement for the administratively
unavailable hosted Ubuntu runner. The selected public Ferris revision is
`b347dc34d62810f043122d0d279def316b890cf0`.

## Required approval inputs

- completed Pulse 01 owner-baseline revisions, with tags frozen only after both
  newly authorized local platform baselines pass;
- public Ferris revision
  `b347dc34d62810f043122d0d279def316b890cf0`;
- exact non-secret installation and removal procedures;
- explicit owner commands and approval records;
- measurable setup effort separated from command runtime; and
- stop conditions for any product, schema, credential, or workflow expansion.

## Authorized operations

Pulse 02 MAY:

- run each unchanged owner command on Windows and local WSL 2 Ubuntu;
- freeze and push one immutable baseline tag per repository only after both
  local platform baselines pass;
- add only Ferris-owned, explicitly removable onboarding files;
- prepare strict owner-controlled planning, Action Plan, owner-entrypoint, and
  approval inputs without command inference;
- run required local `plan`, approved `go`, and `verify` operations on Windows
  and local WSL 2 Ubuntu;
- remove every Ferris-owned onboarding and generated evidence file; and
- rerun the unchanged owner commands on both local platforms after removal.

The public Ferris revision and each platform executable identity MUST be
recorded. Exact private identities, paths, revisions, tags, timings, and raw
output MUST remain in custody.

## Implementation budget

- two existing custody-bound repositories;
- one immutable baseline tag per repository;
- Ferris-owned removable files only;
- no Cargo manifest, owner script, source, topology, policy, or workflow
  modification;
- one baseline, one onboarding execution, one verification, one removal, and
  one post-removal validation per repository and local platform; and
- public-safe aggregate governance, research, and review records.

## Stop conditions and claim limits

Stop rather than expand if the proof requires a product or schema change,
translated owner command, dependency installation, credential, reusable
secret, workflow change, remote fetch, hosted execution, retry after a
materially changed input, third repository, or owner-native correctness change.

This pulse grants no hosted-CI, clean-runner, support, production,
affected-only, performance, savings, workflow-replacement, or external-adopter
claim.

## Removal and validation

Removal MUST delete every Ferris-owned onboarding and generated evidence file
and leave the owner repository at its tagged source state. The unchanged owner
command MUST then pass on Windows and local WSL 2 Ubuntu. Public closeout also
requires changed Markdown link/fence validation, eleven-role review, and
`git diff --check`.

## Result

The read-only WSL preflight observed:

- WSL 2 Ubuntu on Linux `x86_64`;
- Cargo 1.95.0 and rustc 1.95.0;
- Git 2.53.0; and
- no Linux `pwsh` in `PATH`, standard installation locations, or the package
  database.

Both owner commands are PowerShell scripts. Running them unchanged on Linux
therefore remained `unavailable`. Invoking a Windows executable through WSL
would not establish Linux execution, translating the scripts would violate
owner-command immutability, and installing PowerShell was outside the approved
budget.

The pulse stopped before any Windows or WSL baseline command, private
repository mutation, tag, Ferris-owned onboarding file, planning input, Action
Plan, approval, `plan`, `go`, `verify`, receipt, removal operation, or
post-removal run. Both private repositories remain at their Pulse 01 clean
revisions.

## Closeout

The onboarding and removal hypothesis remains not observed. This result is a
capability unavailability, not an owner-command failure and not a Ferris
product failure. The pulse is exhausted and grants no retry, dependency
installation, translated command, hosted runner, successor pulse, product
change, support, production, affected-only, performance, or savings authority.
