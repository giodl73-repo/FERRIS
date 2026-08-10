# VALIDATION-001: Ferris Validation Coverage Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: PLATFORM-001, SCOPE-001, FOREST-003, PERF-Q35, and the Crates Series

## Purpose

This specification defines validation requirements, selection, coverage,
mandatory gates, capability preservation, selected-versus-full evidence,
unknown fallback, and renewal.

Validation planning is distinct from package impact, command execution, and a
claim of complete correctness.

## Validation requirement

Every requirement MUST identify:

- requirement ID, owner, and authority;
- originating application, contract, profile, repository, policy, support, or
  deployment record;
- subject and SCOPE-001 coordinates;
- activity and command family;
- features, targets, profiles, platforms, environments, and providers;
- runtime, data, service, native, ABI, contract, and deployment dimensions;
- capability or risk protected;
- mandatory, conditional, advisory, or full-reference class;
- frequency and trigger;
- evidence expiry;
- failure owner; and
- removal or replacement rule.

## Validation dimensions

Coverage MUST retain applicable:

- format and generated-source checks;
- Cargo check and build;
- lints;
- unit, integration, documentation, property, mutation, fuzz, and runtime
  tests;
- examples and test-target compilation;
- all-features and feature-combination checks;
- debug and release profiles;
- host and cross targets;
- native compile, link, load, ABI, and deployment checks;
- contract and projection conformance;
- security, provenance, license, policy, compliance, signing, and attestation;
- packaging, installation, upgrade, migration, rollback, and removal;
- service, database, network, and operational validation; and
- repository and release gates.

Package selection MUST NOT collapse these dimensions.

## Validation plan

A Validation Plan MUST contain:

- triggering FOREST-002 Change Record and prior root;
- required owner-native anchors;
- affected package and non-package closures;
- selected activities and exact configuration;
- mandatory gates;
- explicit omissions and reasons;
- unknown or unmapped inputs;
- capability coverage;
- full-reference comparison;
- expected cost and resource envelope;
- fallback and widening;
- evidence to collect; and
- expiry.

The plan is non-executable until incorporated into a later approved Action
Plan.

## Selection rule

The minimum defensible selected plan is:

```text
affected owner closures
  + required validation dimensions
  + mandatory repository and policy gates
  + explicit finalization work
  + conservative fallback for unknown inputs
```

Direct-package tests alone are insufficient. Reverse dependency closure MAY
narrow owned Rust package scope, but inputs outside Cargo ownership require
reviewed mappings or safe fallback.

AI MAY propose validation selection. It MUST NOT remove mandatory gates,
unmapped inputs, required capability checks, or full-reference obligations
without deterministic policy or explicit approval.

Any prediction-based reduction from the deterministic Validation Plan MUST
reference a current PREDICTION-001 Prediction Admission Record. The admitted
minimum floor, excluded populations, false-omission threshold, full-reference
frequency, expiry, and disable triggers MUST remain visible in the Validation
Plan and Coverage Ledger.

## Coverage ledger

The Validation Coverage Ledger MUST record:

- requirement;
- selected and full-reference scope;
- planned, executed, passed, failed, expected-rejection, skipped, unsupported,
  unavailable, stale, not-observed, and unknown states;
- command, toolchain, environment, and evidence;
- capability preserved, reduced, unverified, or lost;
- false omission discovered later;
- exemption, owner, reason, approval, and expiry;
- periodic full-reference result; and
- renewal trigger.

Selected-plan success MUST NOT be labeled full-suite, release, platform, or
application success.

## Mandatory gates and exceptions

Mandatory gates MUST come from an identified owner authority. An exception
MUST identify:

- exact requirement and scope;
- requester and approver;
- reason and evidence;
- compensating validation;
- start and expiry;
- prohibited uses; and
- renewal or removal.

An expired, missing, denied, or conflicting exception restores the mandatory
gate or blocks the plan.

## Full-reference and held-out controls

Every narrower policy MUST define:

- periodic full-reference frequency;
- held-out changes or mutation classes;
- false-omission threshold;
- unsupported and unknown behavior;
- comparison retention;
- disable or widening threshold; and
- owner review cadence.

Full-reference runs are evidence about the selection policy. They do not
retroactively make a missed selected run sufficient.

## Failure and fallback

Unknown files, generated inputs, runtime data, native dependencies,
environment changes, policy changes, failed mappings, stale evidence, and
unsupported tools MUST widen to a named safe boundary or block.

If the owner freshness model does not observe the changed input, wider
selection does not validate existing owner artifacts. Required validation MUST
use a renewed owner declaration, an approved isolated empty-state rebuild, or
another explicitly sufficient owner method before those artifacts become
eligible.

Unavailable required validation MUST NOT become pass. A fallback activity MUST
state which capability it does and does not cover.

## Acceptance criteria

VALIDATION-001 may advance to Proposed only when:

1. every validation dimension has an exact requirement and fixture;
2. direct-only, reverse-closure, conservative, and full-reference policies are
   compared against seeded failures;
3. unknown and non-Cargo inputs widen safely;
4. selected and full-reference evidence remain distinct;
5. mandatory gates and expiring exceptions are enforced;
6. capability loss cannot be hidden by passing compilation or tests;
7. periodic full-reference and held-out controls disable unsafe narrowing;
8. removal restores the repository's documented owner-native validation; and
9. all nine roles record a disposition.
