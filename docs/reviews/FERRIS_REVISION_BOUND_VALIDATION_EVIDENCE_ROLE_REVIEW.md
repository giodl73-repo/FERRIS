# Ferris Revision-Bound Validation Evidence Role Review

Date: 2026-08-30
Stage: pre-implementation

## Rust Maintainer

- Accepted: pin `git diff --no-renames` and publish the exact status mapping so
  ambient rename configuration cannot change evidence.
- Accepted: use a result-preserving bounded Git runner rather than collapsing
  unavailable, failed, timed-out, and truncated states.
- Accepted: require exactly one result from `git merge-base --all`.
- Accepted: reconcile Git-deleted paths with the tested checkout; a path
  reintroduced by the tested merge is classified as changed, not lexical
  missing.

## Native Platform Adopter

- Accepted: use a separate 4,096-input bound for Git-derived ranges while
  retaining the existing 256-input explicit-request bound.
- Accepted: make complete local history an explicit owner precondition and
  retain a dedicated insufficient-history diagnostic.
- Accepted: compare canonicalized Git and Cargo roots and cover linked
  worktrees.

## Scope Keeper

- Accepted: planning must run in the tested checkout in V1; planning from a
  separate checkout is explicitly deferred.
- Retained: `--tested-revision` is necessary because BISECT pull-request jobs
  test GitHub's merge revision while classifying the pull-request head.
- Clarified: federated validation planning is unchanged and does not accept the
  revision triple.

## AI Assurance Skeptic

- Accepted: machine output records clean, dirty, or not-observed working-tree
  state and explicitly limits the binding to committed revision evidence.
- Accepted: identity pre-images are normative and platform-relative; the pinned
  identity must agree between local Windows and hosted Linux proof.
- Accepted: a web-only Git fixture must prove the derived route can remain
  narrower than full workspace.
- Accepted: BISECT PR #44 and its before/after conformance evidence are the named
  deletion proof.

## Product Value Governor

The user outcome, budget, completion condition, deletion target, and
abandonment condition remain concrete. All accepted corrections fit inside the
one-pulse design and avoid an additional provider or evidence layer.

Disposition: `continue-within-budget`

## Post-Implementation Closure

The final implementation review found no blocking or actionable findings after
the following controls were proved:

- canonical Git objects are observed with replacement objects disabled and
  ambient repository/object/history routing removed;
- literal-backslash Git paths fail instead of being rewritten;
- committed tested-tree modes, not dirty filesystem path types, determine Cargo
  anchor eligibility; and
- tested `HEAD` is checked before Cargo metadata and again after planning.

Final dispositions:

| Role | Disposition |
|---|---|
| Rust Maintainer | approve |
| Native Platform Adopter | approve |
| Scope Keeper | approve |
| AI Assurance Skeptic | approve |
| Product Value Governor | `continue-within-budget` |

Focused proof: ferris-core 83 passed and 2 ignored; revision CLI 11 passed;
validation-plan schema 4 passed; `git diff --check` passed. The package is clear
to advance to adopter proof.
