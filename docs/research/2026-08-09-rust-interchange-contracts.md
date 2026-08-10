# Rust Interchange Contracts

Date: 2026-08-09
Status: Complete
Question: ECOS-Q03
Decision: model interchange as layered evidence rather than a Boolean
compatibility claim. Exact package identity, re-export identity, trait
coherence, feature resolution, conversion policy, semantic preservation, and
runtime behavior are separate contracts.

## Decision supported

ECOS-Q03 determines where otherwise capable Rust crates compose directly,
where an explicit adapter can join them, and where version, trait, feature, or
semantic boundaries require upstream coordination or profile-level control.

The decision does not select an async runtime, replace ecosystem contracts,
ban duplicate versions, or claim that an adapter preserves all behavior.

## Interchange taxonomy

| State | Meaning | Required evidence |
|---|---|---|
| **Exact identity** | Producer and consumer use the same package ID and item identity | resolved package source/version plus compile probe |
| **Re-export identity** | One facade publicly re-exports the same underlying item | source/re-export evidence plus compile probe |
| **Trait compatible** | Required implementations exist for the exact trait identity | compile probe including generic and trait-object use where relevant |
| **Convertible** | An owned, explicit conversion exists | conversion owner, direction, fallibility, allocation, and field mapping |
| **Wrapped** | A local newtype or local trait supplies an allowed coherence boundary | wrapper ownership and public API impact |
| **Serialized** | Values can cross through a wire or data representation | schema, fidelity, validation, allocation, and error evidence |
| **Feature compatible** | One resolved feature set satisfies all consumers | resolver, target, dependency kind, effective feature, and negative-case evidence |
| **Semantic compatible** | Meaning, invariants, error behavior, ordering, cancellation, metadata, and side effects survive | behavioral tests, not type checking alone |
| **Runtime compatible** | Executors, I/O, timers, task locals, threading, and shutdown assumptions compose | ECOS-Q04 runtime evidence |
| **Incompatible or unknown** | A required layer fails or has not been observed | explicit failed probe or unknown state |

Two crates with similar names or method shapes are not compatible unless the
required identity, implementation, conversion, semantics, and runtime layers
are demonstrated.

## Measured probes

Eight isolated fixtures were run with Rust 1.95.0 and Cargo 1.95.0. Exact
sources, package checksums, commands, and diagnostics are recorded in
[EXP-01](ecos-q03-interchange-contracts/results/EXP-01-interchange-contract-probes.md).

| Probe | Result | Boundary demonstrated |
|---|---|---|
| `serde` derive accepted by `serde_core::Serialize` bound | Pass | facade and core share re-exported trait identity |
| `http` 0.2.12 `Request` passed to `http` 1.5.0 consumer | Expected compile failure, E0308 | duplicate package versions create distinct nominal types |
| `rand_core` 0.10.1 `RngCore` bound passed to 0.6.4 bound | Expected compile failure, E0277 | same trait name across versions is a different trait identity |
| `syn` 3.0.3 `DeriveInput` passed to syn 2.0.119 consumer | Expected compile failure, E0308 | construction-plane AST versions do not interchange |
| direct `From<http 0.2 Request>` for `http 1 Request` downstream | Expected compile failure, E0117 | orphan rules prevent foreign-trait/foreign-type repair |
| local wrapper converting an HTTP request | Pass | newtype restores coherence but owns conversion policy |
| typed `thiserror` error propagated through `anyhow` and downcast | Pass | `std::error::Error` supports application aggregation while retaining a recoverable source type |

The feature fixture passed, but its effective features changed:

```text
feature-low alone:
serde = ["alloc"]

feature-low + feature-high in one resolver-3 application:
serde = ["alloc", "default", "derive", "serde_derive", "std"]
```

The lower-level crate requested `default-features = false`, yet the shared
Serde package instance gained `std` and derive-related features because another
normal dependency requested them.

## Contract-family dispositions

| Family | Current disposition | Primary risk passed forward |
|---|---|---|
| Serde facade/core | Re-export identity verified at 1.0.229 | derive belongs to `serde`; family/version/feature identity must remain explicit |
| HTTP types | Same-major use preferred at public boundaries; 0.2/1.x require conversion | field, extension, body, header, URI, and version fidelity |
| Rand traits | Version-specific trait contracts | implementation must target each required `rand_core` identity or an upstream adapter |
| syn AST | Version-specific construction contract | proc-macro graphs can carry simultaneous AST families |
| log/tracing | Adapter-compatible, not identity-compatible | structured-field loss, direction, filtering, recursion, and global initialization |
| Error types | `std::error::Error` is the common base; typed and erased APIs serve different layers | source chain, downcast, context, `Send`/`Sync`, backtrace, and public API stability |
| Cargo features | Additive within one package resolution, subject to resolver rules | effective features differ from each dependency declaration |
| Build/platform crates | API composition is secondary to environment and target effects | ECOS-Q07 and ECOS-Q09 |

## Findings

### FERRIUM-527: compatibility is layered, not Boolean

**Sources:** measured probes, Cargo feature and resolver documentation, Rust
coherence rules.

**Observed behavior:** One pair can compile because it shares type identity yet
fail semantically at runtime; another can have equivalent data shapes yet fail
at compile time because package identities differ.

**Implication:** OSPREY must record the exact compatibility layer observed and
must not turn build success into semantic or runtime compatibility.

**Confidence:** High.

### FERRIUM-528: re-exported identity is stronger than duplicated shape

**Sources:** Serde 1.0.229 source, serde_core documentation, and the passing
Serde fixture.

**Observed behavior:** A type deriving `serde::Serialize` satisfied a
`serde_core::Serialize` bound because `serde` re-exports the same underlying
trait identity. Serde documents that derive users must depend on `serde`, while
handwritten implementations or trait-bound-only crates may use `serde_core`.

**Implication:** Facade/core relationships need explicit re-export edges; name
similarity alone cannot infer them.

**Confidence:** High.

### FERRIUM-529: package version participates in nominal type identity

**Sources:** HTTP and syn compile-fail fixtures.

**Observed behavior:** Rust emitted E0308 for `http::Request<()>` and
`syn::DeriveInput` values whose displayed names matched but whose package
versions differed. Diagnostics explicitly identified multiple crate versions.

**Implication:** Public API exposure of a crate type makes its package family
and compatible version range part of the consumer contract.

**Confidence:** High.

### FERRIUM-530: same-named traits across versions are separate obligations

**Sources:** rand_core 0.6.4/0.10.1 fixture.

**Observed behavior:** A generic bound on rand_core 0.10.1's `RngCore` did not
satisfy rand_core 0.6.4's `RngCore`; rustc emitted E0277 and identified the two
trait definitions.

**Implication:** Implementations, trait objects, blanket impls, and generic
bounds must be tracked against exact trait identities, not display names.

**Confidence:** High.

### FERRIUM-531: coherence prevents arbitrary downstream repair

**Sources:** Rust Reference orphan rules and the E0117 fixture.

**Observed behavior:** A downstream crate could not implement the foreign
`From` trait directly between two foreign HTTP request types. Rust requires a
local trait or at least one local type to preserve coherent implementations.

**Implication:** Adapter ownership is architectural. Upstream conversion, a
dedicated adapter crate, a local newtype, or an explicit function must own the
bridge.

**Confidence:** High.

### FERRIUM-532: a wrapper solves coherence, not semantics

**Sources:** passing local HTTP wrapper fixture.

**Observed behavior:** A local wrapper allowed a conversion from HTTP 0.2 to
HTTP 1.x. The bounded fixture rebuilt only method, URI, and body; it did not
claim complete preservation of headers, extensions, version, or other
semantics.

**Implication:** Every adapter needs a field and invariant disposition,
direction, loss model, fallibility, allocation behavior, and negative tests.

**Confidence:** High.

### FERRIUM-533: Cargo features are an effective graph property

**Sources:** Cargo features/resolver documentation and feature-unification
fixture.

**Observed behavior:** The isolated low-level branch enabled only Serde
`alloc`. In the combined application, the one resolved Serde version also
enabled default, std, derive, and serde_derive features requested elsewhere.

**Implication:** A manifest declaration is insufficient evidence. OSPREY must
record requesting edges and the effective package/target/dependency-kind
feature set.

**Confidence:** High.

### FERRIUM-534: disabling defaults is not a global negative feature

**Sources:** Cargo feature documentation and feature-unification fixture.

**Observed behavior:** `default-features = false` suppressed defaults for one
dependency declaration but did not prevent another normal dependency from
enabling Serde's defaults on the shared package instance.

**Implication:** Profiles requiring `no_std`, reduced code, provider exclusion,
or security-sensitive feature absence need resolved-graph assertions, not
local manifest inspection.

**Confidence:** High.

### FERRIUM-535: standard error composition preserves a typed/erased choice

**Sources:** anyhow and thiserror documentation plus the passing runtime probe.

**Observed behavior:** Thiserror generated ordinary `std::error::Error`
implementations without appearing in the public API. An application propagated
the typed library error through `anyhow::Error` and successfully downcast to
the original error type.

**Implication:** Libraries can expose typed errors while applications aggregate
them. Erasure, context, downcast, source-chain, and backtrace behavior remain
explicit API and operational choices.

**Confidence:** High.

### FERRIUM-536: adapters between logging contracts are directional and lossy

**Sources:** tracing-log documentation.

**Observed behavior:** tracing-log converts `log::Record` values into tracing
events but documents that unstructured format arguments do not become
structured fields. It also warns that enabling conversions in both directions
can recurse indefinitely without filtering.

**Implication:** An adapter's existence does not establish equivalent data,
filtering, initialization, or runtime behavior.

**Confidence:** High.

### FERRIUM-537: serialization is a boundary protocol, not free type compatibility

**Sources:** Serde data model and ECOS-Q03 taxonomy.

**Observed behavior:** Serialization can decouple nominal Rust types, but it
introduces schema, format, validation, allocation, error, unknown-field,
versioning, and fidelity policy.

**Implication:** FERRIUM should classify serialization as a protocol bridge and
never as exact type interchange.

**Confidence:** High.

### FERRIUM-538: duplicate versions are harmful only when their identities or effects meet

**Sources:** Cargo resolver documentation and compile-fail fixtures.

**Observed behavior:** Cargo can resolve multiple semver-incompatible versions.
They become an interchange failure when values, trait bounds, global resources,
generated code, native links, or feature expectations cross the boundary.

**Implication:** Version duplication should be evaluated by exposed contract
and effect, not prohibited categorically.

**Confidence:** High.

### FERRIUM-539: construction dependencies can leak into public maintenance cost

**Sources:** syn 2.0.119/3.0.3 fixture and Q02 construction-plane census.

**Observed behavior:** Procedural macros can carry multiple syn families
without runtime type exchange, but macros or helper crates that expose syn AST
types create direct version coupling.

**Implication:** ECOS-Q08 must distinguish private construction duplication from
public AST contracts and measure compile cost separately from API
compatibility.

**Confidence:** High.

### FERRIUM-540: public dependency identity is a SemVer concern

**Sources:** Cargo dependency guidance and the measured public-type failures.

**Observed behavior:** Changing the major family of a type or trait appearing
in a public signature can force downstream migration even when the containing
crate's own API names are unchanged.

**Implication:** Compatibility profiles must inventory external types and
traits in public APIs, not only direct dependency declarations.

**Confidence:** High.

### FERRIUM-541: shape-only traits need explicit convergence or adapters

**Sources:** Rust nominal trait identity and coherence rules.

**Observed behavior:** Traits with similar methods remain distinct contracts.
Downstream blanket bridging can be forbidden or create coherence conflicts.

**Implication:** Shared minimal traits, upstream implementations, local
wrappers, and explicit adapter crates are preferable to assuming structural
typing.

**Confidence:** High.

### FERRIUM-542: runtime compatibility remains open after compile success

**Sources:** ECOS-Q03 scope and async/runtime boundaries identified in ECOS-Q01.

**Observed behavior:** These probes establish compile-time identity, feature,
coherence, conversion, and error behavior only. They do not test executors,
cancellation, timers, task locals, I/O readiness, shutdown, or blocking work.

**Implication:** ECOS-Q04 must evaluate runtime contracts independently.

**Confidence:** High.

### FERRIUM-543: trait identity does not encode the full call protocol

**Sources:** tower-service 0.3.3 `Service` documentation.

**Observed behavior:** Tower documents a readiness protocol in which callers
must obtain `Poll::Ready(Ok(()))` from `poll_ready` before `call`; services may
panic if wrappers clone or call the wrong instance without preserving that
readiness. The method signatures alone do not enforce this history.

**Implication:** Trait compatibility evidence must include ordering, ownership,
backpressure, panic, and future-execution semantics where the contract defines
a protocol.

**Confidence:** High.

### FERRIUM-544: globally additive features can alter unrelated observability

**Sources:** log 0.4.33 compile-time filter documentation and Cargo feature
unification.

**Observed behavior:** log's `max_level_*` and `release_max_level_*` features
remove disabled log levels from the binary. The crate explicitly warns
libraries not to enable them because they are global and cannot be changed once
set.

**Implication:** Effective-feature evidence must classify behavioral features,
especially those that suppress diagnostics, select providers, change
serialization, or alter global process behavior.

**Confidence:** High.

## Decision

### Adopt now

- Adopt the layered interchange taxonomy.
- Treat package source/version as part of nominal type and trait identity.
- Record facade/re-export relationships separately from similar names.
- Record effective features with their requesting dependency edges.
- Require adapter direction, owner, loss, fallibility, allocation, and semantic
  tests.
- Keep typed library errors and application error aggregation as distinct
  choices.
- Begin compatibility profiles with public API contract inventories.

Owner: FERRIUM.

Expected validation: ECOS-Q04 runtime probes, ECOS-Q07 target and `no_std`
builds, ECOS-Q08 simultaneous-version and feature-cost measurements, ECOS-Q09
native/provider boundaries, and ECOS-Q11 representative stack profiles.

Non-goals: changing dependencies, publishing adapters, choosing one runtime,
eliminating all duplicates, or claiming wire/runtime compatibility from a
compile pass.

### Prototype behind a compatibility boundary

- Disposable compile-pass and compile-fail fixtures for exact version pairs.
- Newtype and explicit-function adapters with negative semantic tests.
- Public-API dependency inventories.
- Effective-feature diff views by consumer, target, and dependency kind.
- Logging and telemetry adapter probes with recursion and field-loss controls.
- Service-wrapper probes preserving readiness and backpressure.
- Negative checks for globally behavioral features such as log level removal.

### Reject or defer

- global bans on duplicate crate versions;
- structural-typing assumptions for same-shaped traits;
- automatic foreign-type adapters that violate coherence;
- serialization as a silent universal adapter;
- assuming `default-features = false` guarantees feature absence;
- erasing library errors solely for convenience;
- calling an adapter lossless without field and behavior tests; and
- runtime conclusions before ECOS-Q04.

## Role review

### Rust Safety Steward

Accepts explicit identity, coherence, and adapter ownership. Requires adapter
tests to preserve invariants, ownership, pinning, concurrency, error, and
unsafe-boundary behavior.

### Compiler Performance Engineer

Accepts separation of compatibility from duplication cost. Requires ECOS-Q08
to measure proc-macro versions, feature expansion, build scripts, compile time,
and binary impact before deduplication recommendations.

### Interop Boundary Auditor

Accepts the layered taxonomy and negative fixtures. Requires every bridge to
record direction, loss, allocation, fallibility, lifecycle, and unsupported
cases.

### AI Assurance Skeptic

Accepts exact package identities and expected failures. Rejects synthesized
adapters unless their semantic mapping and negative tests are reviewable.

### Ecosystem Strategist

Accepts upstream contracts and adapter ownership as the preferred path.
Requires intervention decisions to avoid replacing mature facades or creating
FERRIUM-only types.

### Rust Maintainer

Accepts compile-time diagnostics and local wrappers as understandable tools.
Requires public API and feature consequences to be visible without specialized
infrastructure.

### Native Platform Adopter

Accepts that Cargo-level compatibility excludes provider, native, target, and
runtime behavior. Requires those boundaries to remain explicit unknowns.

### Scope Keeper

Accepts Q03 as interchange only. Async execution, security, stewardship,
platform breadth, fragmentation cost, native behavior, and stack selection
remain assigned to later questions.

### Validation Checker

Accepts passing and expected-failure controls, exact versions, checksums, and
commands. Requires future profiles to preserve both positive and negative
fixtures.

## Limitations

- The probes cover representative boundaries, not every selected Q02 crate.
- Only one toolchain and host were used.
- HTTP adapter behavior was intentionally bounded and not complete.
- Feature resolution used one resolver-3 normal-dependency workspace.
- Build, dev, target-specific, weak dependency feature, and command-line
  feature cases remain for ECOS-Q08.
- Logging adapter behavior was source-reviewed rather than executed.
- No async runtime behavior was tested.
- No native, WASM, embedded, or `no_std` target was compiled.
