# Pulse 02: Repeatable Enterprise Onboarding and Removal

Status: Complete; invalid before execution
Implementation authority: Exhausted

## Decision

Determine whether both private enterprise consumers can use exact public Ferris
revision `b347dc34d62810f043122d0d279def316b890cf0` for explicit planning,
approved owner-command execution, receipt verification, and complete removal on
Windows and native Linux.

## Product control

| Control | Record |
|---|---|
| Product outcome | Prove a repeatable fresh-enterprise onboarding and removal path without changing owner correctness. |
| Shortest credible path | Use disposable exact clones, generate strict owner-controlled records, execute unchanged owner scripts, remove `.ferris/`, and rerun owner commands. |
| Maximum budget | Two consumers, two platforms, one planning/execution/verification/removal cycle, exact public Ferris revision, and public-safe closeout. |
| Completion | Four planning results, four verified successful receipts, four post-removal owner passes, zero source mutation, and complete temporary cleanup. |
| Abandonment | Stop on identity mismatch, planning failure, owner-command failure, receipt failure, source mutation, secret exposure, product/schema change need, or cleanup failure. |
| Product Value Governor | `continue-within-budget` |

## Materializations

Pulse 02 MAY:

- create one disposable Windows clone and one native-WSL clone of each exact
  custody revision using local origins only;
- create disposable Windows and native-WSL checkouts of exact public Ferris
  revision `b347dc34d62810f043122d0d279def316b890cf0`;
- build the existing `ferris` binary without product changes;
- stage a removable platform-native PowerShell runtime below `.ferris/tools/`;
- add strict planning inputs and generated Action Plan, owner-entrypoint, and
  approval files below `.ferris/`;
- inherit only the non-secret environment names required by the unchanged owner
  scripts;
- run explicit `plan` or `federated-plan`, `go`, and `verify`;
- retain private raw evidence in custody; and
- remove every disposable clone and build at closeout.

Source consumers MUST remain read-only and clean.

## Owner-command boundary

Each Action Plan MUST invoke the unchanged repository-owned validation script
through the staged platform-native PowerShell executable. Ferris MUST NOT parse,
translate, or synthesize the script's Cargo operations.

The Action Plan and approval records MUST bind:

- exact consumer revision;
- exact entrypoint and staged executable identities;
- repository-relative working directory;
- explicit `PATH` inheritance only;
- credential class `none`;
- bounded timeout and output limits; and
- one required owner-validation lane.

## Planning boundary

- `EO-01` MUST run ordinary workspace `plan`.
- `EO-02` MUST run `federated-plan` over its three explicitly declared
  independent workspaces.
- Planning remains non-executable and separate from Action Plan approval.

## Removal proof

After receipt verification, each disposable consumer MUST:

1. delete the complete `.ferris/` tree;
2. prove no other tracked or untracked onboarding file remains;
3. rerun the unchanged owner command; and
4. remain at the exact baseline revision with a clean worktree.

The platform PowerShell prerequisite and all disposable roots MUST then be
removed.

## Stop conditions and non-goals

Stop without retry on product or schema change need, unknown owner command,
credential requirement, identity mismatch, execution failure, receipt
verification failure, source mutation, or cleanup failure.

This pulse does not authorize tags or remote pushes, workflow changes,
affected-only execution, CI narrowing, performance or savings claims,
production support, official Ubuntu support, or external-adopter claims.

## Validation

- exact Ferris and consumer revisions are verified;
- planning succeeds on both platforms for both consumers;
- all four Action Plans execute the unchanged owner scripts successfully;
- all four receipts verify;
- all `.ferris/` material is removed;
- all four post-removal owner commands pass;
- source and disposable consumer worktrees are clean at exact revisions;
- every disposable root and prerequisite is absent;
- changed Markdown local links and fences pass; and
- `git diff --check` passes.

## Result

The exact Ferris revision built successfully on Windows and native Linux after
correcting disposable checkout transport and Windows long-path handling.
Onboarding preparation then exposed two defects:

- `federated-plan` resolves manifest paths relative to the request file, but the
  request below `.ferris/onboarding/` used repository-root-relative paths; and
- Windows PowerShell staging used a literal wildcard path, so no runtime files
  were copied and Action Plan identity generation rejected the missing
  executable.

The planning-failure stop applied. No valid Action Plan, `go`, receipt,
consumer source mutation, owner command, or removal claim followed. All
disposable Windows and WSL roots, native PowerShell files, symlink, archive,
builds, and partial onboarding files were removed. Both source consumers remain
clean.

## Closeout

Pulse 02 is invalid as onboarding evidence. Any separately authorized retry
must preflight request-relative manifests and staged executable presence before
planning or identity generation.
