# PRODUCT-001: Ferris Product Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: BLUE-Q01 through BLUE-Q05 and Crates Series

## Purpose

This specification defines the public identity, category, namespace,
entrypoints, authority boundary, compatibility obligations, and removal
contract for Ferris.

## Normative identity

1. The public product name MUST be **Ferris**.
2. The primary executable MUST be named `ferris`.
3. The Cargo-native executable package SHOULD be named `cargo-ferris` and MUST
   be invokable as `cargo ferris`.
4. **Blueprint** MUST refer to the internal normalized model and non-executable
   plan, not the public product or required command namespace.
5. FERRIS MAY remain the repository, research-program, finding-prefix, and
   application-contract identifier.
6. Public material MUST NOT imply official Rust Project or Rust Foundation
   affiliation or endorsement.

## Product category

Ferris MUST be described as:

> A cross-workspace enterprise build system for Rust.

That category means Ferris owns:

- application and repository discovery;
- cross-workspace planning;
- affected-scope composition;
- policy and approval;
- validation coverage;
- resource coordination;
- explanation and evidence;
- lifecycle and removal; and
- integration of owner-local activities.

The category MUST NOT imply that Ferris owns Cargo resolution, Cargo unit
construction, rustc semantics, linker semantics, test-runner semantics,
contract-system semantics, native-tool semantics, or deployment-provider
semantics.

## Public namespace

The canonical command surfaces are:

```console
ferris <command>
cargo ferris <command>
```

The previously considered public forms `ferris blueprint` and
`cargo blueprint` MUST NOT be required by an implementation.

The crates.io package name `ferris` is occupied by an unrelated library.
Published Ferris packages MUST therefore use qualified package names. Package
names MUST NOT be treated as product identity, command identity, compatibility
identity, or evidence identity.

## One-engine requirement

`ferris` and `cargo ferris` MUST invoke one semantic command engine.

They MUST share:

- semantic command IDs;
- configuration and schema versions;
- Blueprint Model and Plan semantics;
- scope selectors;
- policy and approval behavior;
- output schemas and exit classes;
- evidence and audit records; and
- conformance fixtures.

They MAY differ in discovery defaults:

- `cargo ferris` defaults to the current Cargo workspace;
- `ferris` may select an application, repository set, or multiple workspaces.

The adapters MUST NOT produce different plans for the same explicit inputs,
configuration, scope, tool versions, and evidence.

## Authority boundaries

Ferris MUST delegate local semantics to the owning system.

- Cargo owns package resolution, features, targets, units, freshness, and
  compiler invocation.
- rustc owns language acceptance and compilation.
- test systems own test enumeration and execution.
- linkers and native tools own their local operations.
- contract and deployment systems own their domain semantics.

Ferris MAY compose, constrain, schedule, explain, approve, and record those
operations. It MUST NOT silently substitute its own approximation where owner
truth is available.

Compiler acceptance, a successful build, or a passing selected test set MUST
NOT be described as proof of behavioral correctness, safety, soundness,
security, ABI compatibility, or complete validation. Each such claim requires
dedicated evidence from its owning specification and toolchain.

Ferris planning MUST NOT introduce an `unsafe` implementation boundary.
Any later adapter requiring `unsafe`, FFI, process injection, compiler-private
state, or artifact restoration requires a separately reviewed specification
and implementation pulse.

Unknown, unsupported, stale, incompatible, corrupt, revoked, or uneconomic
inputs MUST widen, defer, reject, or fall back. They MUST NOT become
success-shaped defaults.

## Adoption and removal

Ferris adoption MUST preserve:

- ordinary Cargo commands;
- existing manifests and lockfiles;
- editor and rust-analyzer workflows;
- repository correctness without Ferris;
- explicit full-reference validation; and
- a documented removal path.

Adoption material MUST identify supported tools, platforms, ABIs, deployment
models, training, support ownership, compliance and audit effects, failure
diagnosis, recovery, rollback, and removal cost.

The initial proof MUST NOT require BUILD-file migration, shared writable target
directories, compiler-private state restoration, source rewriting, CI
replacement, or external posting.

Removing Ferris MUST NOT require source-code changes to recover ordinary Cargo
correctness.

Canonical schemas and shared libraries SHOULD remain product-neutral where
their contracts are generally reusable. Repository- or consumer-specific
workflow semantics MUST remain in adapters, application definitions, or
policy.

## Branding and artwork

Ferris branding SHOULD be distinct and independent.

- It MUST NOT use “official,” “original,” or equivalent endorsement language.
- Specific third-party mascot artwork MUST NOT be used without satisfying its
  license.
- Film references MAY be informal wordplay but MUST NOT use protected artwork,
  character likenesses, quotations, or endorsement claims.
- Commercial release SHOULD receive a professional trademark clearance.

## Acceptance criteria

PRODUCT-001 may advance to Proposed only when:

1. every public document uses Ferris for the product and Blueprint for the
   internal model;
2. command examples use `ferris` or `cargo ferris`;
3. package-name conflicts and dated availability observations are recorded;
4. one-engine parity is testable by CONFORMANCE-001;
5. ordinary Cargo preservation and removal are normative;
6. unsupported competitive claims are prohibited; and
7. all nine roles record a disposition.
