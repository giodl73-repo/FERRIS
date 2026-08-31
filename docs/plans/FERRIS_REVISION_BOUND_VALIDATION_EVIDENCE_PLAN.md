# Ferris Revision-Bound Validation Evidence Plan

Status: Authorized bounded V1 slice

## Frame

BISECT already has a working owner system: its workflow chooses the comparison
revisions, owns `npm ci` and `npm run build`, and decides whether those commands
succeeded. Ferris already produces a deterministic non-executable validation
plan from explicit paths and owner-domain declarations.

The missing shared capability is a trustworthy link between those two facts.
BISECT currently carries a repository-local Git status parser and manually
copies revision fields into its owner receipt. That code can drift from the
paths passed to Ferris or from the revision actually checked out by CI.

V1 adds optional local revision binding to `ferris validation-plan`:

```text
working owner workflow
+ Ferris-local revision and change-set binding
-> one auditable plan tied to the exact compared and tested commits
```

Ferris remains read-only. It does not execute owner entrypoints or interpret
their outcomes.

## Falsifiable thesis

Given an owner-selected base, head, and tested revision in a complete local Git
checkout whose repository root is the Cargo workspace root, Ferris can:

1. resolve the base and head to immutable commit identities;
2. compute the merge base and exact merge-base-to-head changed/deleted paths;
3. reject a checkout whose current `HEAD` is not the declared tested revision;
4. reject a tested revision that does not contain the selected head;
5. produce the existing validation plan from those paths; and
6. emit a deterministic binding over the revisions, normalized change set, and
   validation plan identity.

The thesis is disproved if BISECT still needs its own Git path classifier for
the actual plan, if stale tested revisions are accepted, if old explicit-path
plan identities change, or if Ferris must understand the owner command.

## Owner boundaries

Ferris owns:

- local bounded Git observation;
- rename/copy/delete path classification;
- normalized change-set identity;
- revision relationship validation;
- the non-executable validation-plan binding and diagnostics.

The repository owner retains:

- selection of base, head, and tested revisions;
- repository fetch depth and checkout policy;
- owner-domain declarations and opaque entrypoint mapping;
- commands, dependencies, environments, matrices, and artifacts;
- success, required-check, rollout, and rollback policy;
- any execution receipt layered on the Ferris planning evidence.

Git remains authoritative for commit resolution, ancestry, merge-base, and
diff status. Cargo remains authoritative for workspace/package metadata.

## V1 contract

`validation-plan` accepts all three of:

- `--base-revision REVISION`
- `--head-revision REVISION`
- `--tested-revision REVISION`

The three options are atomic and mutually exclusive with `--changed-path`,
`--deleted-path`, and `--changed-package`. Ferris derives the path inputs from
the resolved merge base through the resolved head.

The checkout MUST contain the selected history; Ferris does not fetch missing
objects. CI owners therefore retain fetch-depth policy. Insufficient local
history is reported separately from an invalid revision or Git-process failure.
The roots are compared after filesystem canonicalization so linked worktrees,
symlinks, and platform path aliases do not create textual false mismatches.
All Git observations disable replacement objects so recorded object IDs, trees,
ancestry, and diffs refer to canonical repository objects. Git tree paths with
literal backslashes are rejected as non-portable instead of being rewritten.
Repository- and object-routing Git environment variables are removed before
observation, so `git -C` discovers the selected checkout rather than an ambient
repository. Ferris verifies the tested checkout before Cargo metadata and again
after all worktree-dependent planning to reject concurrent checkout changes.

Ferris invokes Git with rename detection disabled and
`--ignore-submodules=none`. This makes path classification independent of
ambient rename, rename-limit, and submodule-ignore configuration:

| Git status | V1 classification |
|---|---|
| `A`, `M`, `T` | changed path |
| `D`, when absent from the tested checkout | deleted path |
| `D`, when present in the tested checkout | changed path |
| rename | deterministic `D` + `A` because `--no-renames` is pinned |
| copy | deterministic added path because `--no-renames` is pinned |
| conflict, unknown, malformed, or non-UTF-8 | structured hard failure |

Ferris reads the tested revision's committed tree to determine whether that
commit retained, deleted, or reintroduced each head-change path and whether the
committed entry is a regular file, symbolic link, or submodule. Only regular
files may become Cargo anchors; all path kinds remain eligible for owner-domain
selection. Uncommitted files never change that classification; they affect only
the separate working-tree observation. A committed path missing from the
working tree fails with a revision-specific diagnostic.

The derived path route has a separate 4,096-input bound under a bounded Git
output capture. The existing 256-input limit for caller-supplied paths and
packages is unchanged. Derived ranges beyond the higher bound fail with an
actionable diagnostic; V1 does not silently collapse or batch away evidence.

The success record adds optional `revision_binding` with:

- schema `ferris.validation-revision-binding/v1`;
- deterministic `revision_binding_id`;
- exact resolved base, merge-base, head, and tested revisions;
- relationship `tested_is_head` or `tested_contains_head`;
- deterministic `change_set_id`;
- changed and deleted path counts.
- working-tree observation `clean`, `dirty`, or `not_observed`.

The existing `validation_plan_id` remains selection-semantic and independent
of revision context. `revision_binding_id` binds that plan identity to the
revision context and change set. Existing calls without revision options retain
their JSON shape and identities.

`change_set_id` is SHA-256 over sorted records framed as
`kind NUL workspace/path NUL`, where kind is `changed` or `deleted`, paths use
`/`, and no absolute checkout path participates. `revision_binding_id` is over
the schema, resolved revisions, relationship, change-set identity, working-tree
observation, and `validation_plan_id`.

The working-tree field is observation, not attestation. The binding covers
committed revisions and the selected committed diff only. A dirty or
not-observed tree remains explicit in machine output and makes no claim that
uncommitted contents were tested.

## Failure semantics

V1 fails closed when:

- only part of the revision triple is supplied;
- manual inputs and revision inputs are mixed;
- Git or a revision is unavailable;
- local history is insufficient to resolve exactly one merge base;
- the Cargo workspace root differs from the Git root;
- current `HEAD` differs from the tested revision;
- the tested revision does not equal or contain the head;
- merge-base or diff observation fails or exceeds bounds;
- the diff is empty, malformed, non-UTF-8, or contains an unsupported status;
- the derived input set exceeds the existing validation input bound.

All failures use structured diagnostics and request-bound identities. Ferris
does not fetch missing objects, mutate the checkout, or silently fall back to
caller-provided paths.

No merge base and multiple merge bases are distinct failures. Ferris uses
`git merge-base --all` and accepts exactly one result.

## Comparison

### Internal analogues

| Analogue | Disposition | Reason |
|---|---|---|
| Action Plan `source_revision` check | Reuse | Exact current-HEAD equality and lowercase Git object validation already exist. |
| Revision-skew local Git helpers | Adapt | Prompt-free local Git behavior fits, but validation planning needs a result-preserving runner and binary NUL diff parsing so unavailable, failed, timed-out, and truncated states remain distinct. |
| Validation plan identity projection | Reuse | Keep plan semantics stable; add a separate revision binding to avoid compatibility drift. |
| Repository-local BISECT receipt | Replace in part | Keep owner execution outcomes, delete duplicate revision/path facts. |

### External comparators

in-toto Statement v1 binds claims to digest-addressed subjects, while SLSA
provenance separates build definition from resolved dependencies. Ferris V1
adopts the useful semantic precedent—bind evidence to immutable inputs—without
claiming a signed attestation, build provenance, or supply-chain level.

Primary references:

- <https://in-toto.io/Statement/v1>
- <https://slsa.dev/provenance/v1>

## Budget and stop conditions

Budget: one production pulse, one BISECT migration, and at most two
review/fix rounds.

Completion requires:

- accepted and stale/mismatch controls;
- no-contract identity compatibility;
- published structural and semantic schema controls;
- a derived web-only fixture that remains narrower than full-workspace fallback;
- identical pinned identities despite ambient rename configuration and from
  local Windows plus hosted Linux proof;
- BISECT PR #44 deletion of its actual-change parser with before/after
  conformance evidence;
- local and hosted proof with owner execution still repository-controlled;
- clean Rust Maintainer, Native Platform Adopter, Scope Keeper, AI Assurance
  Skeptic, and Product Value Governor dispositions.

Stop if Git semantics require a generalized repository provider, a remote
fetcher, a signing system, or a second evidence layer. Those are separate
product decisions.

## Non-goals

- remote Git hosting APIs or fetch;
- arbitrary source-control systems;
- dirty-working-tree attestation;
- planning from a checkout other than the tested checkout;
- signed attestations or transparency logs;
- owner command declaration or execution;
- owner result verification;
- workflow generation or required-check changes;
- cache keys, reuse decisions, savings, or CI-equivalence claims;
- federated/multi-workspace revision binding.

The existing `federated-validation-plan` command remains unchanged and does not
accept the revision triple.

## Deletion and rollback

BISECT can delete `parseChanges` and `inputArguments` for its actual plan once
the native Ferris path is green. Scenario-only explicit-path calls and the
independent owner-domain oracle remain.

Rollback is to remove the three revision options and use the existing explicit
changed/deleted path interface. No Cargo manifest, owner command, workflow, or
required-check semantics depend on the new record.
