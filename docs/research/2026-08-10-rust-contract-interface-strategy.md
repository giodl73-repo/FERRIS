# Rust Contract and Interface Strategy

Date: 2026-08-10
Status: Complete
Decision: use Rust public APIs and Cargo SemVer for rebuilt crate composition,
RUNE for durable semantic contracts, C ABI for independently versioned native
components, WIT for sandboxed polyglot components, and explicit wire IDLs for
services and persisted messages. Do not attempt to standardize Rust compiler
metadata or a stable ABI for arbitrary Rust types.

## Decision supported

This research decides how the FERRIS enterprise Rust application platform
should represent versioned contracts, how RUNE participates, and what should
remain owned by Rust, Cargo, boundary standards, and implementation-specific
generators.

It informs:

- [FERRIS enterprise Rust application platform](../plans/ENTERPRISE_RUST_APPLICATION_PLATFORM.md);
- CONTRACT-001, PLATFORM-001, and APPLICATION-001;
- Blueprint identity, evidence, validation, and conformance specifications; and
- future RUNE adoption without moving RUNE code into FERRIS.

## What Rust provides natively

### Public APIs and traits

Rust public modules, types, functions, constants, traits, implementations, and
macros form a source-level API. Traits are the native abstraction contract, but
their evolution is constrained:

- adding a required trait item is breaking;
- changing an item signature is breaking;
- adding a defaulted item can still create method ambiguity;
- object-compatible trait APIs have additional restrictions; and
- blanket implementations, auto traits, associated types, generic bounds, and
  public dependency types can widen the compatibility surface.

Sources:

- [Rust Reference: traits](https://doc.rust-lang.org/reference/items/traits.html)
- [Cargo SemVer compatibility guide](https://doc.rust-lang.org/cargo/reference/semver.html)
- [RFC 1105: API evolution](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

### Cargo versions, resolution, and features

Cargo supplies package versions, dependency requirements, lockfiles, feature
selection, resolver behavior, source identity, and `rust-version` handling.
Caret requirements are the default compatibility expression. Features are
additive build-time selections and are commonly unified across a graph;
resolver 2 narrows some unification and resolver 3 is the Edition 2024 default
with incompatible-Rust-version fallback behavior.

Cargo does not define an interface version distinct from the crate version.
Features are not runtime capability negotiation and should not encode mutually
exclusive semantic contracts.

Sources:

- [Cargo dependency requirements](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [Cargo resolver](https://doc.rust-lang.org/cargo/reference/resolver.html)
- [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)

### Compiler metadata and ABI

`rlib`, `dylib`, and `rmeta` contain compiler-specific metadata. The rustc
development guide states that `rlib` is specific to rustc and may change, and
metadata carries compiler-version compatibility. The Rust Reference provides
no stability guarantee for `extern "Rust"`.

`repr(C)` and `extern "C"` supply explicit layout and calling-convention
mechanics. They do not define ownership, allocation, errors, panic/unwind,
threading, lifetime, semantic compatibility, or version negotiation.

Sources:

- [rustc libraries and metadata](https://rustc-dev-guide.rust-lang.org/backend/libs-and-metadata.html)
- [Rust Reference: type layout](https://doc.rust-lang.org/reference/type-layout.html)
- [Rust Reference: external blocks and ABI](https://doc.rust-lang.org/reference/items/external-blocks.html)
- [Rust Reference: linkage](https://doc.rust-lang.org/reference/linkage.html)

## Current compatibility tools

`cargo-semver-checks` and `cargo-public-api` provide useful CI guardrails over
public API changes. They are tools rather than language guarantees. Their
analysis also inherits limitations from Rust API extraction; official rustdoc
JSON remains experimental.

Sources:

- [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks)
- [cargo-public-api](https://github.com/cargo-public-api/cargo-public-api)
- [rustdoc unstable features](https://doc.rust-lang.org/nightly/rustdoc/unstable-features.html)

FERRIS should use these tools with compile-pass, compile-fail, behavioral,
target, feature, and migration fixtures rather than treating one API diff as
complete compatibility proof.

## External interface choices

### WIT and the WebAssembly Component Model

WIT explicitly describes interfaces and worlds with versioned package
identities. It is the strongest current standardized option for sandboxed,
polyglot, capability-oriented components. WIT does not define package
resolution. Rust component documentation now favors native tooling with
`wit-bindgen`; official WASI 0.3 was released on 2026-06-11 with native async
support and requires current compatible runtimes such as Wasmtime 43 or later.

Sources:

- [Component Model documentation](https://component-model.bytecodealliance.org/)
- [WIT overview](https://component-model.bytecodealliance.org/design/wit.html)
- [WIT worlds](https://component-model.bytecodealliance.org/design/worlds.html)
- [Rust component support](https://component-model.bytecodealliance.org/language-support/building-a-simple-component/rust.html)
- [WASI roadmap](https://wasi.dev/roadmap)
- [WASI releases](https://github.com/WebAssembly/WASI/releases)

### Native and language projections

- [UniFFI](https://mozilla.github.io/uniffi-rs/) generates foreign-language
  bindings from UDL or proc-macro contracts.
- [CXX](https://cxx.rs/) defines a checked Rust/C++ bridge for coordinated
  source builds.
- [Diplomat](https://rust-diplomat.github.io/diplomat/) generates bindings for
  several target languages from a Rust-first model.
- Protobuf with [prost](https://github.com/tokio-rs/prost) and
  [tonic](https://github.com/hyperium/tonic) provides a mature schema and RPC
  boundary.

These are projections or boundary-specific contract systems. None is a
universal stable in-process Rust ABI.

## RUNE disposition

[RUNE](https://github.com/giodl73-repo/RUNE) already implements the needed
product-neutral semantic layer:

- neutral descriptors and collections;
- derive-authored metadata;
- deterministic crate-owned registries;
- versioned profiles and adapters;
- retained evidence, state graphs, and read-first agent protocol records; and
- explicit compatibility reports that fail closed without automatic migration
  or runtime-host authority.

RUNE should remain separate from FERRIS because:

1. its contract model can serve consumers that do not adopt the FERRIS
   platform;
2. neutral descriptors should not inherit FERRIS product vocabulary;
3. independent compatibility policy makes the standards layer replaceable;
4. FERRIS can own profiles and adapters without owning the neutral format; and
5. moving code now would add migration risk without solving a measured
   contract problem.

FERRIS should contribute upstream to RUNE when a missing neutral contract is
demonstrated. Product-specific support policy, crate selection, Blueprint
evidence, and enterprise profile behavior stay in FERRIS.

## Recommended crate solution

### Adopt now

1. Treat a Rust crate's public API plus Cargo SemVer as its native source
   contract.
2. Declare `rust-version`, targets, features, providers, public dependencies,
   and support periods.
3. Use RUNE identities for stable application concepts, operations, errors,
   lifecycle, compatibility evidence, and projections.
4. Use C ABI with opaque handles for independently versioned native binaries.
5. Use WIT components for sandboxed or polyglot plugins.
6. Use explicit wire IDLs for remote services and durable messages.
7. Run API, compile, behavioral, feature, target, migration, renewal, removal,
   and rollback conformance tests.

### Prototype behind a compatibility boundary

Prototype one RUNE-authored contract expressed through:

- an idiomatic Rust trait/API;
- a C ABI projection;
- a WIT interface and component;
- one generated language binding or wire schema; and
- a FERRIS compatibility and evidence report.

The proof must distinguish shape compatibility from ownership, error, async,
cancellation, panic, threading, lifecycle, and behavioral compatibility.

### Reject or defer

- a stable ABI for arbitrary Rust types or traits;
- versioning `rustc` metadata as an application standard;
- runtime reflection that bypasses explicit Rust authoring;
- one universal interface crate for unrelated capabilities;
- automatic best-effort migration;
- Cargo features as interface negotiation;
- merging RUNE into FERRIS before a measured ownership or release problem; and
- claiming WIT, C ABI, or generated bindings preserve semantics without
  conformance tests.

## Findings

### FERRIS-696: Rust's native crate contract is source-level

**Sources:** Rust Reference traits; Cargo SemVer guide; RFC 1105.

**Observed behavior:** Rust verifies selected public APIs and trait contracts
at compile time, while their compatibility evolves with the containing crate.

**Implication:** FERRIS should strengthen source contracts rather than promise
an independent native interface version that Rust does not define.

**Confidence:** High.

### FERRIS-697: Cargo versions packages, not interfaces

**Sources:** Cargo dependency, resolver, and feature references.

**Observed behavior:** Cargo resolves package releases and additive features;
it does not negotiate a separately versioned runtime interface.

**Implication:** contract identity and package identity must remain separate.

**Confidence:** High.

### FERRIS-698: Rust compiler metadata is not an enterprise ABI

**Sources:** rustc libraries and metadata guide; Rust Reference ABI and
linkage sections.

**Observed behavior:** Rust-native library metadata is compiler-specific and
`extern "Rust"` has no stability guarantee.

**Implication:** independently deployed native components require an explicit
stable boundary.

**Confidence:** High.

### FERRIS-699: C ABI is a base mechanism, not a complete contract

**Sources:** Rust Reference type-layout and external-block sections.

**Observed behavior:** C representation and calling conventions do not encode
ownership, panic, threading, lifetime, error, or semantic evolution.

**Implication:** FERRIS must pair C ABI projections with RUNE metadata and
conformance tests.

**Confidence:** High.

### FERRIS-700: traits require conservative evolution

**Sources:** Cargo SemVer guide and Rust trait reference.

**Observed behavior:** required items and signatures are breaking; even
defaulted additions can create ambiguity.

**Implication:** enterprise extension points need explicit implementor policy,
sealed traits or extension objects, and negative compatibility fixtures.

**Confidence:** High.

### FERRIS-701: WIT is the preferred portable component contract

**Sources:** Component Model, WIT, Rust component support, and WASI release
documentation.

**Observed behavior:** WIT provides versioned package/interface/world identity
for polyglot components but leaves package resolution outside the format.

**Implication:** use WIT for component boundaries while FERRIS profiles retain
resolution, runtime, support, and deployment evidence.

**Confidence:** High.

### FERRIS-702: binding generators are projections, not universal standards

**Sources:** UniFFI, CXX, Diplomat, prost, and tonic documentation.

**Observed behavior:** each tool targets a particular language or deployment
boundary and preserves a different semantic subset.

**Implication:** RUNE should describe common meaning while boundary adapters
remain replaceable and independently validated.

**Confidence:** High.

### FERRIS-703: RUNE already occupies the neutral contract layer

**Sources:** RUNE README, v1 release-readiness policy, and Mission 2.0.

**Observed behavior:** RUNE has versioned descriptors, registries, profiles,
adapters, retained evidence, and compatibility reports with explicit
non-goals.

**Implication:** adopt and contribute to RUNE rather than duplicating or moving
it into FERRIS now.

**Confidence:** High.

### FERRIS-704: the enterprise platform is a support contract

**Sources:** ECOS-Q01 through ECOS-Q12 and this interface review.

**Observed behavior:** Rust already supplies strong compilation and package
mechanisms; the missing layer joins semantic contracts, exact profiles,
support, conformance, assurance, stewardship, renewal, removal, and rollback.

**Implication:** FERRIS should standardize and support layered contracts, not
create a replacement language, ABI, or universal crate stack.

**Confidence:** High.

## Limitations

- Rust and Cargo behavior will continue to evolve.
- Tool-based API checks do not prove semantics.
- WIT and WASI runtime support remains versioned and operationally dependent.
- RUNE has not yet been evaluated as the contract source for a complete FERRIS
  application profile.
- No cross-language proof in this repository has exercised all recommended
  projections.
- Microsoft, Rust Project, Rust Foundation, Bytecode Alliance, and upstream
  owner commitments have not been negotiated.
