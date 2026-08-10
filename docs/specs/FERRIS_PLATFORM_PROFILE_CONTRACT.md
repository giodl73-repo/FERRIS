# PLATFORM-001: Renewable Platform Profile Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: CONTRACT-001 and ECOS-Q05 through ECOS-Q12

## Purpose

This specification defines renewable, consumer-scoped compatibility and
support profiles over exact Rust application stacks, environments, contracts,
validation, assurance, and lifecycle.

## Profile identity

Every profile MUST include:

- stable profile ID and revision;
- owner and approval authority;
- consumer and application scope;
- requirements and explicit non-goals;
- creation, observation, approval, expiry, and supersession times;
- prior and replacement profile identities;
- support state; and
- schema version.

A profile name without an exact revision and evidence identity is not
sufficient selection.

## Requirements

Requirements MUST distinguish mandatory eligibility from preference.

They MAY cover:

- operations and semantics;
- Rust API and Typebook contracts;
- deployment and runtime;
- compiler and MSRV;
- host, target, architecture, and target tier;
- `std`, `alloc`, `core`, embedded, WASM, or component capability;
- async runtime, I/O, cancellation, and context;
- providers, native libraries, SDKs, and tools;
- security, licensing, provenance, and stewardship;
- performance and resource budgets;
- observability, debugging, crash, sanitizer, coverage, and profiling;
- packaging, signing, deployment, rollback, and servicing; and
- policy, compliance, data, and support obligations.

Candidate ranking, popularity, downloads, recency, and composite scores MUST
NOT override mandatory eligibility.

## Exact selection and closure

A profile selection MUST retain:

- direct package source, release, and requested features;
- manifest and Cargo lock identity;
- complete lockfile package universe;
- target-active normal, build, and development closures as applicable;
- requested and effective features;
- public dependency exposure;
- build scripts, procedural macros, `links`, native code, generated code, and
  unsafe boundaries;
- contract and adapter versions;
- provider and source mode; and
- accepted alternatives.

Lock universe and active target closure MUST remain separately queryable.

## Environment

Every profile environment MUST identify:

- Cargo, rustc, toolchain channel, and exact compiler;
- host and target triples;
- installed target and components;
- linker, archiver, debugger, and runner;
- native compilers, SDKs, generators, package managers, and system packages;
- runtime and provider;
- container, VM, filesystem, and execution substrate;
- deployment target; and
- environment evidence date.

A declared `rust-version` MAY select a compiler-floor test. It MUST NOT replace
execution with the exact compiler and closure.

## Validation stages

Each stage MUST be recorded independently:

- resolve;
- check;
- lint;
- build;
- link;
- execute;
- unit test;
- integration test;
- doctest;
- contract conformance;
- package;
- sign or attest;
- deploy;
- operational validation; and
- rollback.

Each stage result MUST be:

- pass;
- fail;
- expected rejection;
- unsupported;
- unavailable;
- not observed;
- stale; or
- unknown.

One passing stage MUST NOT promote another stage.

## Assurance and stewardship

A profile MUST retain applicable:

- registry and source identity;
- archive checksum and packaged revision;
- publication authority;
- license and policy decision;
- advisory database, tool, time, and scope;
- unsafe, build-script, macro, and native review scope;
- owner and succession evidence;
- release and repository activity;
- support and security-response commitments;
- known incidents, exceptions, and residual unknowns; and
- evidence expiry.

Zero advisory matches, recent activity, or stable package count MUST NOT be
described as security or maintenance certification.

## Support and servicing

Support MUST identify:

- owner and contact;
- supported profile revisions;
- supported compiler, target, platform, provider, and deployment combinations;
- support start and end;
- security response;
- update and emergency policy;
- diagnostic and escalation path;
- exception policy;
- training and operational requirements; and
- unsupported combinations.

Ferris MAY enforce declared support policy. It MUST NOT infer support from
successful local observation.

## Renewal

A profile MUST expire. The default maximum evidence age SHOULD be 90 days
unless a shorter consumer or risk policy applies.

Renewal MUST occur earlier when applicable:

- direct or active dependencies change;
- requested or effective features change;
- contracts or adapters change;
- advisories or security notices change;
- owners, custody, licensing, or succession changes;
- compiler, Cargo, target tier, SDK, native tools, provider, or deployment
  changes;
- required validation fails or becomes unavailable;
- policy or consumer requirements change; or
- evidence is revoked.

Renewal MUST produce a reviewed diff across identity, closure, features,
contracts, environment, validation, assurance, support, limitations, and
rollback.

A scheduled green run MAY refresh evidence. It MUST NOT approve adoption or
merge an update without policy authority.

## Substitution

Provider or implementation substitution MUST:

- preserve or explicitly revise the consumer contract;
- identify changed dependencies, features, runtime, native, data, and
  deployment boundaries;
- execute positive, negative, migration, and rollback fixtures;
- compare support, assurance, cost, and operational consequences;
- retain the prior selection and owner decision; and
- remain reversible during the declared rollback period.

## Removal and rollback

Every profile MUST define:

- capability boundary;
- replacement or removal path;
- metadata and automation cleanup;
- dependency and feature cleanup;
- contract, public type, data, wire, ABI, native artifact, deployment, and
  credential consequences;
- validation commands;
- retained historical evidence; and
- exact rollback owner and procedure.

Restoring a lockfile is insufficient when contract, data, native, or deployment
state changed.

Ordinary Cargo operation MUST remain available without the profile.

## Initial profile families

The first schema MUST support independently scoped:

1. hosted service;
2. CLI and configuration;
3. pure data processing;
4. embedded and `no_std`;
5. browser WASM;
6. WebAssembly component;
7. bundled or system-native dependency;
8. identity, credential, TLS, and cryptographic provider; and
9. testing, assurance, packaging, and deployment.

These families MUST NOT be merged into one universal stack.

## Acceptance criteria

PLATFORM-001 may advance to Proposed only when:

1. at least six independent profile families have exact fixture revisions;
2. lock and target-active closures are separately verified;
3. compiler-floor, host, target, native, and runtime states are explicit;
4. stage-specific positive, expected-rejection, unsupported, unavailable, and
   unknown cases exist;
5. assurance and stewardship evidence has date, source, owner, scope, and
   expiry;
6. one renewal, substitution, removal, and exact rollback are executed;
7. ordinary Cargo and non-Ferris profile consumers remain functional; and
8. all nine roles record a disposition.
