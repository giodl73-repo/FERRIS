# RESOLUTION-001: Ferris Plan Resolution Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: CAUSALITY-001, PREDICTION-001, PLANNING-001, and GOVERNANCE-001

## Purpose

This specification defines how Ferris compares candidate Blueprint Plans,
applies hard constraints and preferences, presents alternatives, records human
or policy decisions, and selects a non-executable result.

Resolution chooses a disposition. It does not approve or execute commands.

## Resolution request

Every request MUST identify:

- application and requested outcome;
- candidate Blueprint Plans;
- current Forest root and evidence cutoff;
- principal and tenant;
- applicable governance policies;
- hard correctness, safety, compatibility, support, data, and lifecycle
  constraints;
- resource, cost, latency, and maintenance preferences;
- required reviewers;
- decision deadline and expiry; and
- unavailable or unknown inputs.

## Candidate eligibility

Before ranking, each candidate MUST be evaluated for:

- complete plan identity;
- required owner closures;
- validation and capability preservation;
- contract, profile, platform, and support eligibility;
- policy and tenant eligibility;
- trust and evidence requirements known at resolution time;
- resource-envelope feasibility;
- rollback and removal;
- unsupported, stale, failed, or unknown scope; and
- prohibited actions.

An ineligible candidate MUST remain visible with its rejection reasons. It MUST
NOT be hidden merely because another candidate is preferred.

## Decision ordering

Resolution MUST apply:

1. hard correctness and owner constraints;
2. mandatory validation and capability requirements;
3. governance, privacy, security, support, and lifecycle constraints;
4. explicit user or maintainer requirements;
5. resource and operational feasibility;
6. reversible cost and benefit preferences; and
7. advisory ranking.

Popularity, model preference, convenience, predicted speed, or composite score
MUST NOT override a hard constraint.

## Alternatives

Ferris MUST retain applicable alternatives:

- proceed with the selected plan;
- choose a wider safe plan;
- choose owner-native full-reference work;
- collect additional evidence;
- wait for an observation barrier;
- request an updated plan;
- request owner or human input;
- defer;
- reject; or
- remove the proposed capability.

Every alternative MUST state consequence, remaining unknowns, validation,
resource cost, rollback, and decision owner.

## Resolution record

Every Resolution Record MUST contain:

- record ID, schema, version, and parent;
- request and candidate identities;
- eligible and rejected candidates;
- constraint and policy evaluation;
- causal and prediction evidence used;
- alternatives;
- selected disposition;
- selected Blueprint Plan where applicable;
- reasons and tradeoffs;
- required approval class;
- unresolved conditions;
- deciding principal or deterministic policy;
- time, expiry, and revocation state; and
- limitations.

The record MUST be immutable. A changed plan, policy, principal, constraint,
evidence root, or material environment produces a new resolution.

## AI boundary

AI MAY summarize candidates, identify tradeoffs, or recommend one disposition.
It MUST NOT:

- declare an ineligible plan eligible;
- waive mandatory validation or policy;
- invent absent evidence;
- conceal rejected alternatives;
- make a high-risk human decision;
- create approval; or
- execute the selected plan.

Model identity, prompt reference, evidence, recommendation, confidence, and
human or policy disposition MUST remain attributable.

## Decision classes

The canonical disposition MUST be one of:

- explain;
- diagnose;
- compare;
- validate;
- request approval;
- request evidence;
- request replan;
- use full reference;
- contribute upstream;
- defer;
- reject;
- require owner input; or
- remove.

`Request approval` is not approval. `Validate` identifies required work; it
does not execute it.

## Acceptance criteria

RESOLUTION-001 may advance to Proposed only when:

1. eligible, conditionally eligible, ineligible, stale, failed, and unknown
   candidates have fixtures;
2. hard constraints always precede advisory ranking;
3. rejected candidates and alternatives remain visible;
4. changed plans, policies, principals, evidence, and environments invalidate
   prior resolutions;
5. AI cannot waive constraints or create approval;
6. full-reference, evidence, replan, defer, reject, owner-input, and removal
   dispositions are exercised;
7. resolution removal leaves direct maintainer decision and owner workflows
   intact; and
8. all nine roles record a disposition.
