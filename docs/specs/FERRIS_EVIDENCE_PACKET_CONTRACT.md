# FERRIS-001: Ferris Evidence Packet Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: FOREST-003, RESOLUTION-001, EXECUTION-001, TRUST-001,
GOVERNANCE-001, and CONNECTOR-001

## Purpose

This specification defines the portable FERRIS Evidence Packet and its
adaptation into bounded owner-aligned upstream contribution packets.

A packet is a review artifact. It is not a correctness certificate, approval,
trust decision, deployment record, or authorization to post externally.

## Packet identity

Every packet MUST identify:

- packet ID, schema, version, parent, and lifecycle;
- packet kind and audience;
- application and tenant scope;
- source and revision;
- originating Forest root;
- packet owner and maintainer;
- creation and verification times;
- data classification;
- retention and deletion policy; and
- supersession or replacement.

## Required sections

A complete packet MUST retain applicable:

1. application, contract, profile, platform, toolchain, and environment
   identities;
2. triggering change and requested outcome;
3. selected Query Forest slices and source references;
4. causal explanation and limitations;
5. Prediction Record and prediction-versus-observation error;
6. Blueprint Plan, alternatives, and Resolution Record;
7. Action Plan, approval, policy, and exceptions;
8. execution, deviations, failures, cancellation, rollback, and cleanup;
9. validation coverage, full-reference comparison, and capabilities;
10. trust, provenance, integrity, privacy, retention, revocation, and deletion
    state;
11. connector and external-owner interactions;
12. Outcome Record and measured user or operational impact;
13. omitted, unsupported, stale, unavailable, failed, and unknown scope; and
14. upstream, deferred, rejected, removed, or retained disposition.

Sections that do not apply MUST say why. Missing evidence MUST NOT be replaced
with success-shaped defaults.

## Packet kinds

The first schema MUST support:

- maintainer investigation;
- plan review;
- action audit;
- validation and capability review;
- profile renewal;
- incident and rollback;
- adoption and removal;
- connector or MCP audit;
- upstream contribution; and
- application support review.

Kinds MAY select different views. They MUST retain the same canonical evidence
identities and authority boundaries.

## Portability

Packets MUST:

- use versioned product-neutral schemas for canonical records;
- preserve owner identifiers and source links;
- include a manifest and bounded file inventory;
- define canonical serialization;
- support detached large or classified evidence by reference;
- preserve redaction and omitted-evidence markers;
- declare required viewers or adapters;
- reject unsupported versions without guessing; and
- remain reviewable without a live Ferris service where policy permits.

A packet digest or signature proves integrity of the packet bytes and
authenticates an assertion. It does not prove correctness or completeness.

## Privacy and redaction

Packet creation MUST apply TRUST-001 and GOVERNANCE-001 before serialization.

Private source, repository names, paths, dependency names, logs, personal
data, credentials, tokens, prompts, and model inputs MUST be omitted, redacted,
aggregated, or retained by authorized reference according to policy.

Every redaction MUST record:

- field or section;
- reason and authority;
- audience;
- effect on interpretation;
- retained source owner; and
- whether a less-redacted authorized packet exists.

## Completeness

Packet completeness MUST be evaluated against:

- packet kind;
- required sections;
- applicable scope;
- source availability;
- policy-required redaction;
- evidence freshness;
- owner review; and
- unresolved unknowns.

Canonical completeness state MUST be complete, complete with named
limitations, incomplete, blocked, stale, revoked, failed, or unknown.

## Upstream adaptation

An upstream contribution packet MUST:

- identify one upstream owner and maintainer question;
- use the owner's vocabulary and intake format;
- provide the smallest licensed reproducer preserving positive and negative
  controls;
- record exact environment and copyable commands;
- separate observation, inference, prediction, and unknowns;
- preserve correctness and capability frontiers;
- request one bounded upstream action;
- name maintenance ownership; and
- satisfy the adopted Rust Performance Contribution Packet where applicable.

The upstream packet MUST be smaller than the internal evidence archive.

## External publication

Packet generation, submission-ready status, connector availability, or MCP
tool discovery MUST NOT authorize:

- issue creation;
- comments;
- branches;
- pull requests;
- benchmark submissions;
- package publication;
- deployment;
- promotion; or
- funding commitments.

External publication requires EXECUTION-001 action approval and the external
owner's process.

## Lifecycle

Packets MUST support draft, assembled, reviewed, submission-ready, submitted,
accepted, external, superseded, revoked, expired, and retired states.

Submitted and accepted states require an external owner reference. Revocation
denies future eligible use but retains required historical audit. Deletion
follows TRUST-001 and MUST record partial failures.

## Acceptance criteria

FERRIS-001 may advance to Proposed only when:

1. every packet kind has a canonical fixture;
2. required sections, not-applicable reasons, limitations, and unknowns are
   machine-checkable;
3. packet serialization and manifests are deterministic;
4. public, private, redacted, stale, incomplete, revoked, and deletion-failed
   packets have fixtures;
5. two viewers reproduce the required canonical packet subset;
6. one packet adapts to a licensed owner-aligned upstream format;
7. no packet state can authorize external action; and
8. all nine roles record a disposition.
