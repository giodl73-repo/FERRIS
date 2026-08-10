# FERRIS Enterprise Rust Application Platform

Status: Architecture planning
Implementation authority: None
Contract dependency: [RUNE](https://github.com/giodl73-repo/RUNE)

## Mission

FERRIS will define an enterprise application platform for Rust without
creating a Rust fork, a replacement standard library, a permanent crate
distribution, or a stable Rust ABI that the language does not provide.

The platform joins:

- idiomatic Rust crates and Cargo source compatibility;
- RUNE-authored semantic contracts and compatibility evidence;
- explicit C ABI, WIT, and wire-schema boundaries where source compatibility
  is insufficient;
- renewable, compatibility-tested crate profiles;
- coordinated security, stewardship, platform, native-tool, and lifecycle
  evidence;
- Blueprint application, build, validation, dependency, and change
  intelligence;
- immutable Query Forest roots with typed comparison, promotion, retention,
  rollback, and revocation references; and
- FERRIS evidence packets for adoption, renewal, removal, and rollback.

## Naming and repository decision

The FERRIUM lab becomes **FERRIS**. The existing `FERRIUM-*` finding IDs and
historical research documents remain unchanged so citations and evidence do
not break. New research uses the `FERRIS-*` prefix.

RUNE remains a separate product-neutral standards and protocol repository.
FERRIS should consume its versioned descriptor, registry, profile, adapter,
evidence, and compatibility contracts rather than copy the implementation into
the platform repository. FERRIS-specific adapters, profiles, support policy,
and application-platform decisions belong here.

Code movement may be reconsidered only if:

1. RUNE can no longer remain useful outside FERRIS;
2. separate release and compatibility policies create measured consumer harm;
3. ownership, licensing, migration, rollback, and downstream impact are
   reviewed; and
4. a dedicated cross-repository pulse approves the move.

## Native Rust contract boundary

Rust provides strong source-level contracts but not a complete enterprise
interface system:

- public items and traits define source APIs;
- crate versions and Cargo requirements express compatibility expectations;
- Cargo features select additive build capabilities;
- `rust-version`, targets, and profiles constrain compilation;
- `rustc` metadata allows crates built by the same compiler toolchain to
  compose; and
- the type system verifies a selected graph at compile time.

Those mechanisms do not provide:

- an independently versioned interface separate from the crate;
- a stable Rust ABI between independently compiled components;
- runtime interface negotiation;
- a language-neutral type library;
- semantic compatibility for ownership, errors, async behavior, cancellation,
  panic, threading, or lifecycle; or
- coordinated support guarantees across multiple crate owners.

FERRIS must not describe `extern "Rust"`, Rust vtables, symbol mangling,
`rlib`, `dylib`, or compiler metadata as stable application contracts.

## Layered contract model

### Layer 1: Rust source contracts

Use ordinary Rust crates for components released and rebuilt together:

- explicit public API and public-dependency policy;
- Cargo SemVer compatibility;
- Edition 2024 and resolver 3 where eligible;
- declared `rust-version`, target, feature, and provider support;
- sealed traits where external implementations are not a supported contract;
- `#[non_exhaustive]` and extension objects where evolution requires them;
- compatibility checks with `cargo-semver-checks`, public-API inspection, and
  source fixtures; and
- negative tests for trait, version, feature, runtime, and target boundaries.

This is the default crate-to-crate contract. It is a source compatibility
contract, not a stable binary interface.

### Layer 2: RUNE semantic contracts

Use RUNE to retain the application-facing meaning that Rust and Cargo do not
encode as one durable record:

- stable contract and operation identity;
- contract version independent of implementation release;
- fields, commands, events, errors, invariants, lifecycle, and trace links;
- ownership, capability, sensitivity, stability, and authority metadata;
- explicit registries and deterministic collection identity;
- compatibility reports across descriptors, profiles, adapters, and runtimes;
- evidence and conformance references; and
- projections into boundary-specific schemas.

RUNE descriptors supplement Rust APIs. They do not promise ABI stability,
replace compiler checking, scrape arbitrary source, or make compatibility
decisions without evidence.

### Layer 3: Stable external boundaries

Choose the boundary by deployment need:

| Boundary | Preferred contract | Use |
|---|---|---|
| Rebuilt Rust crates | Rust API + Cargo SemVer + RUNE evidence | Ordinary in-process composition |
| Native independently versioned component | C ABI with opaque handles | Stable native library/plugin base |
| Rust/C++ source integration | CXX over an explicit bridge | Closely coordinated Rust/C++ builds |
| Mobile or language SDK | UniFFI or Diplomat projection | Generated foreign-language bindings |
| Sandboxed/polyglot plugin | WIT + WebAssembly Component Model | Versioned capability-oriented components |
| Remote service or durable message | Protobuf/gRPC or another explicit wire IDL | Network and persisted data contracts |

Every boundary must state ownership, allocation, errors, async behavior,
cancellation, panic/unwind, threading, lifetime, version negotiation,
unsupported states, and removal.

### Layer 4: Supported application profiles

FERRIS may publish renewable enterprise profiles that name:

- exact crate releases and features;
- compiler, host, target, and provider pairs;
- lock and active target closures;
- contract and adapter versions;
- native prerequisites and deployment artifacts;
- supported validation stages;
- assurance and stewardship snapshots;
- support and security-response periods;
- renewal date, successor, removal, substitution, and rollback.

A profile is a coordinated support statement for exact evidence. It is not a
claim that included crates are universally best, safe, certified, or the only
valid Rust stack.

### Layer 5: Evidence and operations

Blueprint and the Crate Ecosystem Ledger explain:

- what contract, dependency, feature, platform, provider, native, assurance,
  stewardship, profile, and validation evidence changed;
- whether the change is compatible, degraded, blocked, stale, unsupported, or
  unknown;
- which owner must decide or act; and
- what adoption, renewal, removal, and rollback work remains.

Observation never grants mutation, publication, installation, deployment, or
approval authority.

## Cargo Application Model

Blueprint is the public Cargo component:

```console
cargo blueprint generate
cargo blueprint inspect
cargo blueprint compare
cargo blueprint refs
cargo blueprint validate
cargo blueprint update
```

Cargo remains authoritative for packages, targets, features, source identity,
resolution, and lock state. Blueprint MUST consume Cargo metadata and MUST NOT
reimplement the resolver or create a competing dependency manifest.

The model has three distinct records:

1. **Application definition:** consumer-authored components, services,
   requirements, RUNE contracts, providers, platforms, validation, and
   lifecycle intent.
2. **Blueprint model:** normalized application intent joined with Cargo graph
   truth, exact active closures, contracts, profiles, native prerequisites,
   evidence, and owner state.
3. **FERRIS Application Contract:** resolved and validated output carrying
   compatibility, support, expiry, renewal, substitution, removal, and
   rollback evidence.

Initial declaration may use a versioned Cargo metadata namespace:

```toml
[workspace.metadata.blueprint]
application = "payments-service"
profile = "ferris.service.v1"
contracts = ["rune:payments.v2"]
```

The canonical schema and file projections remain specification work. The
metadata example is not yet a stable format.

Blueprint roots are immutable evidence manifests. Branches are moving
development refs, tags are write-once, channels are policy-controlled
promotion refs, aliases are local conveniences, and pins retain roots. Leases
and tombstones are policy records; labels are searchable metadata only.
References improve comparison, promotion, retention, and rollback but never
replace Cargo freshness, compatibility, integrity, trust, validation, or
restore-economics checks. See
[Rust build-state references](../research/2026-08-10-rust-build-state-references.md).

## Initial enterprise profile families

The first specification work should define profiles, not select permanent
implementations:

1. service and hosted application;
2. CLI and configuration;
3. data and serialization;
4. native Windows and Linux integration;
5. WebAssembly component and browser;
6. embedded and `no_std`;
7. telemetry and diagnostics;
8. identity, credentials, TLS, and cryptographic providers; and
9. testing, fuzzing, assurance, packaging, and deployment.

Each profile must permit provider substitution and document the contract
boundary that makes substitution possible.

## Upstream and governance model

FERRIS should:

- fund and contribute to existing crate and tool maintainers;
- contribute compatibility fixtures and actionable diagnostics;
- work through the Rust Project, Rust Foundation, Bytecode Alliance, crate
  owners, and relevant standards bodies;
- prefer neutral conformance suites over FERRIS-only traits;
- publish support criteria, expiry, exceptions, and conflict disclosures; and
- separate Microsoft or enterprise support commitments from ecosystem-wide
  Rust guarantees.

FERRIS should not:

- declare itself the owner of Rust, Cargo, crates.io, WIT, or upstream crates;
- require all implementations to share one crate or runtime;
- standardize unstable compiler metadata or a Rust ABI;
- use popularity as a compatibility or support criterion;
- hide native, provider, runtime, target, or licensing constraints; or
- mutate consumer manifests, hosts, providers, or deployments automatically.

## Potential Microsoft participation

This plan does not claim Microsoft sponsorship or support. If Microsoft adopts
the program, its strongest contribution would be a transparent support layer:

- coordinated support and security-servicing windows for exact profiles;
- Windows, Linux, Azure, container, WebAssembly, native, accessibility,
  identity, cryptographic-provider, and regulated-environment validation;
- engineering and funding for current upstream maintainers;
- neutral conformance suites and diagnostics contributed through existing
  owners;
- published exceptions, end-of-support, successor, migration, and rollback
  policy; and
- governance that permits non-Microsoft implementations and providers.

Microsoft-specific support commitments should be represented as one profile
and policy owner. They must not be mislabeled as guarantees from the Rust
Project, Rust Foundation, Bytecode Alliance, crates.io, or individual crate
maintainers.

## Required specifications

Before an application-platform implementation:

1. CONTRACT-001 defines Rust API, RUNE, C ABI, WIT, and wire-contract
   identities and compatibility rules.
2. PLATFORM-001 defines profile selection, support, servicing, renewal,
   substitution, removal, and rollback.
3. APPLICATION-001 defines application definitions, the normalized Blueprint
   model, Cargo metadata integration, typed root references, and FERRIS
   Application Contracts.
4. EVIDENCE-001 defines source adapters and ownership.
5. VALIDATION-001 defines conformance and capability preservation.
6. TRUST-001 defines provenance, security, privacy, ref authority, retention,
   revocation, and deletion.
7. CONFORMANCE-001 defines held-out positive, negative, failure, unsupported,
   stale, version-skew, migration, and removal tests.
8. A separately approved pulse selects one bounded consumer proof.

## Validation expectations

- At least three independently released crates exercise source-level evolution.
- At least one C ABI, WIT component, and wire-schema projection is tested.
- Compatible, additive, breaking, degraded, unsupported, stale, and unknown
  changes are distinguished.
- Windows and Unix native boundaries execute.
- One provider and one implementation are substituted without changing the
  consumer contract.
- Adoption, renewal, exact rollback, and complete removal are demonstrated.
- RUNE remains usable by a non-FERRIS consumer.

## Non-goals

- A replacement Rust standard library.
- A stable ABI for arbitrary Rust types and traits.
- One universal runtime, TLS provider, database, GUI, or async stack.
- A global lockfile or permanent distribution.
- A quality, maintenance, safety, or portability score.
- Automatic migration or best-effort conversion.
- Product code before the specification and held-out gates pass.
