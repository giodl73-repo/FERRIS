# EVIDENCE-001: Ferris Evidence Adapter Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: GOVERNANCE-001, SCOPE-001, FOREST-002, and the Crates Series

## Purpose

This specification defines how evidence adapters observe one owner system and
emit canonical Forest records without taking policy, planning, approval, or
execution authority.

## Adapter boundary

An adapter MUST have one primary owner boundary, such as:

- Cargo;
- supported rustc evidence;
- editor or language service;
- build script or procedural macro;
- linker, native toolchain, or runtime loader;
- validation runner or repository gate;
- registry, source host, advisory, license, or stewardship source;
- Typebook or other contract owner;
- platform, deployment, or observability system; or
- a later CONNECTOR-001 connector.

Combining sources MAY occur in the normalizer or a projection. An adapter MUST
NOT silently adjudicate conflicts between owners.

## Adapter manifest

Every adapter MUST declare:

- adapter ID, version, owner, and support contact;
- owner system and supported versions;
- observation methods and required permissions;
- input and output schemas;
- emitted node, edge, scope, identity, state, and observation kinds;
- stable, preview, nightly, generated, inferred, or experimental evidence
  classes;
- platform and environment support;
- data classification and retention;
- credentials and network behavior;
- freshness, expiry, and renewal;
- failure and fallback behavior;
- limitations and known blind spots; and
- deprecation, replacement, and removal.

Stable and version-coupled evidence MUST remain separate.

## Observation requirements

Every emitted claim MUST identify:

- claim class: owner-declared, directly observed, externally reported,
  normalized, inferred, or unknown;
- subject and SCOPE-001 coordinates;
- owner authority;
- exact command, query, API, protocol, file, or artifact;
- source revision or response identity;
- adapter and tool versions;
- environment;
- observation time and expiry;
- result and diagnostic;
- confidence where applicable; and
- limitations and unavailable dimensions.

An inferred claim MUST cite the observations and rule used. It MUST NOT be
relabeled as owner-declared or directly observed.

## Read-only default

Evidence collection MUST be read-only by default.

Any command that may generate files, run build scripts or macros, execute
untrusted code, contact a network, access credentials, mutate a source system,
or create durable side effects MUST be classified before use. Such activity
requires the applicable governance and later action contract; it MUST NOT be
hidden inside observation.

## Secrets and sensitive data

Credentials, reusable tokens, private keys, secret values, and unrestricted
environment dumps MUST NOT enter canonical evidence.

Adapters MUST:

- request minimum permissions;
- redact at collection boundaries;
- preserve sensitivity labels;
- enforce tenant and repository isolation;
- bound logs, payloads, and samples;
- record access without recording secrets; and
- fail explicitly when safe redaction cannot be guaranteed.

## Result states

Adapter results MUST distinguish:

- observed;
- observed with limitations;
- expected rejection;
- unsupported owner version;
- unavailable;
- permission denied;
- malformed;
- conflicting;
- stale;
- failed; and
- unknown.

Absence of emitted records MUST NOT mean an empty owner graph or successful
observation.

## Normalization and conflict

Adapters emit owner-shaped evidence. The FOREST-002 normalizer validates and
converts it to canonical records.

Normalization MUST retain:

- original owner identity;
- raw-evidence reference or digest;
- adapter and schema versions;
- lossy or synthesized fields;
- conflicts;
- rejected records; and
- fallback.

Owner-local facts remain governed by the owner system. Governance may deny use
of evidence but MUST NOT rewrite the observed owner fact.

## Ownership and upstream routing

Every adapter field and diagnostic MUST name its maintenance owner. Failures
caused by missing or unstable owner interfaces MUST be routable to:

- the owner project;
- Ecosystem Bridge;
- a connector maintainer;
- Ferris normalization or projection; or
- the consuming repository.

Ferris-specific workarounds MUST be removable and MUST record an upstream or
replacement disposition.

## Conformance

Each adapter requires fixtures for:

- supported positive observation;
- empty but successful owner result;
- expected rejection;
- unsupported and version-skewed owner;
- partial and malformed output;
- permission and isolation failure;
- timeout, cancellation, and interruption;
- stale evidence;
- secret-redaction failure;
- conflicting owner evidence; and
- complete adapter removal.

## Acceptance criteria

EVIDENCE-001 may advance to Proposed only when:

1. Cargo, compiler, native/link, validation, ecosystem, and contract adapters
   have exact manifests;
2. stable and version-coupled evidence are separated;
3. direct, declared, reported, normalized, inferred, and unknown claims cannot
   be confused;
4. side-effecting collection cannot bypass governance;
5. secrets and cross-tenant evidence are rejected;
6. all result states and conformance cases are executable;
7. removal restores direct owner-system workflows; and
8. all nine roles record a disposition.
