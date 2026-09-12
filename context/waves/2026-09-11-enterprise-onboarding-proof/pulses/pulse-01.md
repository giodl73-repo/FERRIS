# Pulse 01: Owner-Native Enterprise Baselines

Status: Complete; stopped incomplete at hosted capability gate
Implementation authority: Two private repositories only
Budget: Two repository creations, one owner baseline per repository, and one
hosted Ubuntu owner workflow per repository

## User outcome

Create fresh enterprise-shape repositories whose correctness, topology, and
removal invariants are established before Ferris adoption, so a later onboarding
pulse can measure what Ferris adds without manufacturing the baseline around
the product.

## Authorized consumers

- `EO-01`: ordinary multi-package Cargo workspace plus one non-Cargo owner
  concern.
- `EO-02`: three independent Cargo workspaces plus one explicit application
  relationship graph.

Exact repository owner, names, URLs, revisions, paths, and run identities remain
in private custody.

## Required owner baselines

Each repository MUST:

- be private and contain no credential or reusable secret;
- use ordinary Cargo manifests and locked dependencies;
- define explicit repository-owned formatting, lint, and test commands;
- run those commands in hosted Ubuntu CI;
- document topology, owner authority, baseline scenarios, and complete removal;
- keep any generated topology deterministic and byte-verifiable;
- remain independently useful without Ferris; and
- freeze one immutable owner-baseline tag only after Windows and hosted Ubuntu
  checks pass.

`EO-02` MUST keep its three Cargo workspaces independently resolved and tested.
Its application relationship graph is owner data and MUST NOT become a shared
Cargo resolver or hidden dependency source.

## Validation

- repository cleanliness and default-branch checks;
- formatting, Clippy with denied warnings, and locked/offline owner tests;
- deterministic topology regeneration or strict checked-in integrity;
- hosted Ubuntu owner workflow success;
- exact topology and scenario review;
- immutable baseline tag binding; and
- private custody plus public-safe closeout.

## Stop conditions

Stop on any need for Ferris code or contracts, consumer Ferris files, Action
Plans, cross-repository credentials, a third repository, changed owner semantics,
or a performance, support, production, affected-only, or savings claim.

## Closeout

Pulse 01 created exactly two private repositories without Ferris files,
contracts, adapters, outputs, or dependencies.

`EO-01` retained:

- one ordinary three-package Cargo workspace;
- one explicit non-Cargo owner concern;
- formatting, denied-warning Clippy, locked/offline tests, metadata cardinality,
  and deterministic topology verification; and
- owner scenarios plus a removal invariant.

`EO-02` retained:

- three independently resolved and tested Cargo workspaces;
- one explicit owner application relationship graph that does not combine Cargo
  resolution;
- one application-owner concern;
- formatting, denied-warning Clippy, locked/offline tests, per-workspace
  metadata cardinality, and deterministic topology verification; and
- owner scenarios plus a removal invariant.

Both complete owner commands passed on Windows and both repositories were clean
on their default branches. The enterprise host recognized each checked-in
workflow but reported repository Actions disabled. Attempts to enable the
repository setting did not change that enforced state. Zero hosted jobs ran.

The hosted Ubuntu completion condition was therefore unavailable, not failed.
The pulse stopped without alternate CI, privileged policy change,
cross-repository credentials, a third consumer, or Ferris onboarding. No
immutable owner-baseline tag was created.

The aggregate record is
[`Private enterprise onboarding baseline attempt`](../../../../docs/research/2026-09-11-private-enterprise-onboarding-baselines.md).
Exact host, repository, revision, path, and raw-output evidence remains in
private custody.

Completed role dispositions are recorded in
[`FERRIS Enterprise Onboarding Baselines Role Review`](../../../../docs/plans/reviews/FERRIS_ENTERPRISE_ONBOARDING_BASELINES_ROLE_REVIEW.md).

Pulse 01 is exhausted and incomplete. It does not authorize Ferris onboarding,
consumer mutation, alternate hosted infrastructure, or Pulse 02.
