# CONTRACT-001: Contract Identity and Compatibility

Status: Draft after nine-role review
Implementation authority: None
Depends on: ECOS-Q03, ECOS-Q04, ECOS-Q07 through ECOS-Q12, Typebook/RUNE v1

## Purpose

This specification defines how Ferris identifies, compares, validates, and
projects Rust source APIs, Typebook semantic contracts, native ABIs,
WebAssembly components, generated bindings, wire schemas, and durable data
contracts.

## Contract layers

Ferris MUST keep these layers distinct:

| Layer | Owner | Governing identity |
|---|---|---|
| Rust source | crate maintainer and Cargo | package source, release, public API, features, target, toolchain |
| Semantic | Typebook/RUNE contract owner | contract, operation, error, state, invariant, lifecycle, version |
| Native ABI | component owner | ABI family, symbols, calling convention, layout, ownership, allocator, unwind |
| Component | WIT/component owner | package, interface, world, version, runtime, capability |
| Wire/data | schema owner | schema namespace, message/service identity, version, compatibility policy |
| Projection | adapter owner | source contract, target contract, generator, adapter, loss, validation |

One layer MUST NOT be used as proof of another.

Rust source compatibility MUST NOT be described as stable binary
compatibility. C layout or calling convention MUST NOT be described as a
complete semantic contract. Schema shape MUST NOT prove behavior.

## Contract identity

Every contract identity MUST include:

- contract kind;
- namespace and stable contract ID;
- contract version;
- owner;
- source and revision;
- operation or surface selection;
- applicable feature, target, platform, provider, runtime, and deployment
  scope;
- schema or descriptor version;
- evidence date and expiry;
- lifecycle state; and
- supersession or replacement identity when applicable.

Package version, implementation version, interface version, profile revision,
adapter version, and evidence revision MUST remain separate.

## Operation contract

An operation contract MUST describe applicable:

- inputs and outputs;
- preconditions, postconditions, and invariants;
- error and status model;
- ownership and borrowing;
- allocation and deallocation;
- lifetime and retention;
- ordering and idempotency;
- concurrency, synchronization, and thread affinity;
- asynchronous readiness and executor assumptions;
- cancellation and timeout;
- backpressure and flow control;
- panic, exception, unwind, and abort behavior;
- security and data classification;
- observability and audit;
- version negotiation;
- unsupported and degraded behavior; and
- migration, removal, and rollback.

An omitted dimension MUST be represented as not applicable, unsupported,
unobserved, or unknown rather than silently defaulted.

## Rust source contracts

Rust source contracts MUST retain:

- exact package source and release;
- Cargo requirement and lock selection;
- public modules, types, traits, functions, constants, macros, and public
  dependency types;
- requested and effective features;
- target and configuration;
- `rust-version`, Cargo, rustc, and edition;
- implementor policy for traits;
- exhaustiveness and extension policy;
- compile-pass, expected-failure, API-diff, and behavioral fixtures; and
- SemVer policy and known exceptions.

Cargo SemVer and API-analysis tools MAY provide evidence. They MUST NOT be
treated as complete semantic proof.

`extern "Rust"`, rustc metadata, `rlib`, `rmeta`, Rust symbol mangling, and Rust
vtable layout MUST NOT be standardized as stable application ABI.

## Typebook semantic contracts

Typebook/RUNE MUST remain product-neutral and independently useful.

Semantic contracts MAY define:

- concepts and stable identities;
- operations, commands, events, and state transitions;
- structured errors and outcomes;
- invariants and lifecycle;
- sensitivity, authority, and ownership;
- compatibility policy;
- evidence and trace references; and
- projections into Rust, C ABI, WIT, wire, data, or documentation formats.

Ferris-specific support, policy, profile selection, planning, and execution
MUST remain outside the neutral contract.

## Native ABI contracts

A native ABI contract MUST state:

- target ABI and calling convention;
- symbol and versioning policy;
- integer, enum, string, buffer, handle, and aggregate representation;
- alignment and layout;
- ownership and lifetime;
- allocation and release functions;
- nullability and optionality;
- error transport;
- panic, exception, unwind, and abort behavior;
- threading and callback rules;
- library loading and dependency requirements;
- compatibility negotiation;
- debug and diagnostic behavior; and
- removal and rollback.

Opaque handles SHOULD be preferred for independently versioned stateful native
components.

`repr(C)` and `extern "C"` alone are insufficient conformance.

## WIT and component contracts

WIT contracts MUST retain:

- package, interface, world, and version identity;
- imports, exports, resources, and capabilities;
- component runtime and WASI version;
- generated binding tool and version;
- async and cancellation behavior;
- host policy and sandbox assumptions;
- package-resolution and deployment inputs outside WIT; and
- runtime positive, negative, unsupported, and version-skew evidence.

Producing a component binary MUST NOT establish runtime compatibility.

## Wire and durable-data contracts

Wire and data contracts MUST identify:

- namespace, service/message/table/event identity, and version;
- encoding and canonicalization;
- field presence, defaults, unknown-field behavior, and ordering;
- numeric, temporal, locale, and Unicode semantics;
- compatibility direction;
- producer and consumer version ranges;
- persistence and migration;
- partial failure, retries, duplication, and idempotency;
- privacy, retention, and deletion; and
- rollback constraints.

Compilation of generated code MUST NOT establish cross-version behavior or
data migration safety.

## Projections and adapters

Every projection or adapter MUST record:

- source and target contract identity;
- direction;
- generator and implementation version;
- fields and operations preserved;
- fields and operations lost, synthesized, reordered, copied, allocated, or
  made fallible;
- runtime, provider, platform, and feature assumptions;
- ownership and maintenance;
- positive, negative, failure, unsupported, and version-skew fixtures;
- adoption, expiry, removal, and rollback; and
- current upstream owner.

An adapter MUST NOT claim semantic equivalence when it preserves only shape or
nominal types.

## Compatibility result

Compatibility is directional and scoped. A result MUST be one of:

- compatible;
- compatible with named conditions;
- additive but requiring consumer action;
- degraded with preserved minimum capability;
- breaking;
- unsupported;
- failed;
- stale;
- not observed; or
- unknown.

The result MUST identify:

- source and target;
- direction;
- scope and operation;
- evidence and tool versions;
- changed dimensions;
- affected consumers;
- migration and validation requirements;
- expiry; and
- decision owner.

A Boolean compatibility field is prohibited as the canonical result.

## Evolution and migration

Contract changes MUST classify:

- identity-preserving implementation change;
- compatible source evolution;
- compatible projection change;
- additive semantic change;
- conditionally compatible change;
- breaking source, semantic, ABI, component, wire, or data change;
- deprecation;
- supersession; and
- removal.

Every breaking or conditional change MUST name affected consumers, migration
fixtures, rollback, support period, and owner approval.

## Acceptance criteria

CONTRACT-001 may advance to Proposed only when:

1. one contract is expressed through Rust source, Typebook, C ABI, WIT, and
   wire or durable-data projections;
2. identity separation is demonstrated across package, contract, adapter,
   profile, and evidence versions;
3. positive, negative, failure, unsupported, and version-skew fixtures exist;
4. ownership, allocation, error, async, cancellation, panic, threading,
   lifetime, migration, and removal are tested;
5. compatibility results are directional and non-Boolean;
6. Typebook remains usable without Ferris;
7. no stable arbitrary Rust ABI is implied; and
8. all nine roles record a disposition.
