# Pulse 01: Owner-Native Enterprise Baselines

Status: Authorized
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

Pulse 01 ends after owner-baseline tags and evidence custody. It does not
automatically authorize Ferris onboarding, consumer mutation, or Pulse 02.
