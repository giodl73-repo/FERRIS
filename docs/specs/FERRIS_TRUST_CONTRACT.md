# TRUST-001: Ferris Trust, Provenance, Privacy, and Lifecycle Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: GOVERNANCE-001, IDENTITY-001, PERF-Q30, and BLUE-Q01

## Purpose

This specification defines provenance, integrity, consumer trust, privacy,
ref authority, retention, revocation, deletion, and trust-decision lifecycle.

Identity, integrity, provenance, trust, compatibility, validation, and
correctness are separate claims. No one claim substitutes for another.

## Trust subjects

Trust policy MUST identify applicable:

- principal or workload;
- producer and builder;
- source and revision;
- toolchain and environment;
- connector or MCP endpoint;
- contract, profile, application, plan, action, root, ref, packet, artifact,
  or attestation;
- tenant and data classification;
- operation and scope;
- time and expiry; and
- consumer.

Trust is consumer-scoped and operation-specific. A producer accepted for one
artifact class, repository, tenant, or action MUST NOT be universally trusted.

## Provenance record

Provenance MUST retain:

- subject identity and digest where applicable;
- producer and delegated identities;
- builder and build type;
- source materials and revisions;
- external parameters;
- commands or semantic operations;
- toolchain, platform, and environment;
- start and completion time;
- isolation and network state;
- outputs;
- signatures, transparency, or attestation references;
- limitations and unresolved execution cones; and
- schema and verification versions.

A valid signature authenticates an assertion. It does not prove that the
assertion is complete or that the subject is correct.

## Integrity

Integrity verification MUST state:

- algorithm and parameters;
- expected digest and size;
- canonicalization or archive rules;
- verified bytes or record set;
- verification time and tool;
- mismatch behavior; and
- fallback.

Content identity MUST remain separate from action, compatibility, and owner
identity.

## Consumer trust decision

Every decision MUST evaluate applicable:

- expected subject and action identity;
- accepted producer, signer, builder, and build type;
- source and dependency expectations;
- platform, ABI, contract, profile, and environment;
- artifact class and execution-cone completeness;
- signature, transparency, attestation, and revocation;
- validation and capability evidence;
- policy, tenant, residency, and data use;
- freshness and expiry; and
- known incidents and unknowns.

The result MUST be trusted for named use, trusted with conditions, denied,
revoked, stale, unsupported, incomplete, failed, or unknown.

Freshness, expiry, lease, retention, and revocation comparisons MUST use
FOREST-002 time evidence. Time uncertainty that overlaps a trust boundary
MUST produce stale, unknown, denied, or blocked according to the consuming
operation; it MUST NOT extend eligibility.

## Refs and roots

Immutable roots retain historical identity. Refs provide navigation and
policy-controlled promotion.

Ref names, channels, labels, trusted producers, or valid signatures MUST NOT
establish compatibility, validation, availability, or correctness.

Moving ref authority MUST use IDENTITY-001 generations, GOVERNANCE-001
authorization, expected prior values, history, expiry, and revocation.

## Secrets and privacy

Credentials, private keys, reusable tokens, credential caches, secret values,
and unrestricted private inputs MUST NOT enter plans, roots, refs, packets,
prompts, logs, or attestations.

Trust processing MUST:

- enforce minimum disclosure;
- preserve classification and tenant isolation;
- redact before model or connector exposure;
- record visible omissions;
- apply residency and transfer policy;
- bound logs and samples;
- support subject-access and deletion obligations; and
- fail closed when safe handling cannot be established.

Redacted evidence MUST NOT appear complete.

## Retention

Retention decisions MUST consider:

- current refs and generations;
- pins and leases;
- application, validation, support, audit, legal, and incident obligations;
- root and packet lineage;
- data classification and residency;
- revocation evidence;
- owner policy;
- cost; and
- deletion requirements.

Age alone MUST NOT authorize deletion. Retention of identity or audit metadata
MUST remain distinguishable from retention of source, artifacts, logs, or
personal data.

## Revocation

Revocation MUST support:

- principal;
- credential class;
- signer or builder;
- connector or MCP endpoint;
- policy exception;
- ref, root, packet, or artifact eligibility;
- trust decision; and
- pending or running action.

Revocation denies future eligible use within its scope. It MUST NOT claim that
historical signatures, observations, or bytes never existed.

Revocation propagation, emergency disablement, cache invalidation, running
action behavior, notification, and audit MUST be explicit.

For every action that may outlive one trust check, applicable revocation
policy MUST define:

- revocation sources and scope;
- observation or subscription method;
- maximum detection interval;
- required checks before side effects and at action barriers;
- behavior for unavailable or unknown revocation status;
- interruptible and non-interruptible owner operations;
- credential invalidation and cache handling;
- rollback or compensation;
- notification and escalation; and
- audit evidence.

Unknown revocation status MUST block the next side-effecting operation unless
an explicit policy defines a narrower safe operation that requires no revoked
authority.

## Deletion

Deletion MUST define:

- subject and exact data classes;
- authority and request;
- tenant and scope;
- dependent records;
- legal or audit constraints;
- tombstone or retained proof;
- connector and replica propagation;
- completion evidence;
- partial or failed deletion; and
- recovery or irreversibility.

Deletion failure MUST remain visible. A tombstone is not proof that every copy
was deleted.

## Artifact use

Before artifact installation or restoration, Ferris MUST separately verify:

- candidate action identity;
- compatibility envelope;
- integrity;
- provenance and consumer trust;
- authorization;
- validation and capability;
- isolation and materialization; and
- net benefit.

Any failure falls back to an owner-native rebuild or a named blocked state.

## Acceptance criteria

TRUST-001 may advance to Proposed only when:

1. identity, integrity, provenance, trust, compatibility, validation, and
   correctness cannot be collapsed;
2. signed, unsigned, incomplete, mismatched, stale, revoked, and unknown
   subjects have fixtures;
3. consumer-scoped trust policies differ by operation and artifact class;
4. ref promotion cannot bypass authorization or validation;
5. secrets and cross-tenant data are excluded from durable and model-visible
   records;
6. pins, leases, expiry, revocation, retention, deletion, and tombstones have
   positive, partial-failure, and negative fixtures;
7. artifact rejection falls back safely without mutating shared immutable
   storage; and
8. all nine roles record a disposition.
