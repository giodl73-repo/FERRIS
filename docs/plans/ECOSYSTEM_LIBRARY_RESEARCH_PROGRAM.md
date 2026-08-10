# Crates Series: Ecosystem and Library Research Program

Status: Complete

## Series identity

The twelve ECOS questions are the **Crates Series**. They completed the FERRIS
research sequence after PERF-Q01 through PERF-Q36.

The Crates Series is also Phase 0 of the
[OSPREY program](OSPREY_PROGRAM.md). The series and final nine-role review
completed on 2026-08-10, satisfying one prerequisite for OSPREY. Separate
specification, conformance, held-out workflow, adoption, rollback, and
approved-pulse gates still prohibit an OSPREY implementation package,
executable, service, repository adapter, or production integration.
OSPREY architecture work remains specification-only.

## Opportunity thesis

Rust has a deliberately compact standard library and a large crates.io
ecosystem. The standard library describes itself as a set of
"minimal and battle-tested shared abstractions," while the .NET core libraries
ship a much broader application-platform surface:

- [Rust standard library](https://doc.rust-lang.org/std/)
- [.NET core library overview](https://learn.microsoft.com/en-us/dotnet/standard/class-library-overview)

The resulting Rust gap is not simply that libraries are missing. Mature crates
exist for serialization, asynchronous execution, HTTP, TLS, regular
expressions, command-line parsing, databases, identifiers, URLs, time zones,
telemetry, cryptography, and user interfaces. The unresolved question is
whether maintainers can select and combine those crates with predictable
governance, compatibility, security, portability, maintenance, and lifecycle
properties.

FERRIS should investigate that application-platform contract before creating
another standard library, package manager, framework, or curated distribution.

## Decision this program informed

FERRIS will explain the ecosystem through an evidence-backed capability map,
define renewable compatibility and assurance profiles, and prepare
owner-aligned upstream contribution packets. It will advance a read-only
ecosystem-ledger/profile diff only to specification and held-out design.
Certification, universal ranking, automatic mutation, a curated distribution,
and replacement capabilities already owned by established projects are
rejected or deferred.

## Research principles

- Separate an unavailable capability from one that is available but difficult
  to discover, combine, trust, or sustain.
- Treat download counts and repository popularity as signals, not quality or
  security proof.
- Distinguish language and `std` guarantees from de facto ecosystem
  conventions.
- Evaluate complete dependency and feature closures, not only top-level crate
  APIs.
- Preserve MSRV, target, `no_std`, WASM, native-library, licensing, unsafe-code,
  and maintenance requirements.
- Prefer shared traits, compatibility tests, documentation, and upstream
  stewardship over a FERRIS-owned replacement stack.
- Do not label a stack "blessed," "standard," or "certified" without published
  criteria, reproducible evidence, renewal rules, and an ownership model.

## Capability census

The initial comparison should cover:

| Capability | Rust `std` baseline | Ecosystem questions |
|---|---|---|
| Collections, I/O, files, threads, synchronization, TCP/UDP | Substantial portable foundation | Platform consistency, async boundaries, specialized data structures |
| Async execution and I/O | Language futures without one bundled runtime | Runtime coupling, portable traits, cancellation, testing, observability |
| Serialization and data formats | Not bundled | Shared data model, format fidelity, derive cost, schema evolution |
| HTTP, TLS, URLs, identity | Mostly external crates | Type interoperability, backend selection, native roots, policy |
| Date, time zone, locale, Unicode services | Limited foundation | Data updates, platform behavior, serialization, globalization |
| Cryptography and secure storage | Not bundled | Algorithm policy, provider boundaries, audits, platform integration |
| Databases and messaging | Not bundled | Runtime coupling, pooling traits, migrations, protocol compatibility |
| CLI, configuration, logging, telemetry | Not bundled | Common conventions, composition, generated code, operational evidence |
| GUI, media, GPU, accelerator access | Not bundled | Platform coverage, native dependencies, lifecycle, portability |
| Testing, fuzzing, mocking, property testing | Mixed toolchain and crates | Common evidence contracts and maintenance expectations |

This census compares Rust with .NET, Go, Java, C++, and relevant native package
ecosystems without assuming that a larger standard library is automatically
better.

## Research questions

The ecosystem sequence is independent of the `PERF-Qxx` compiler-performance
sequence. Historical findings retain the global `FERRIUM-XX` sequence; new
post-rename findings use `FERRIS-XX`.

Detailed question files and status are maintained in the
[Crates Series research-question registry](../research/questions/ecosystem/README.md).

ECOS-Q01 is complete. It freezes five coverage classes—Guaranteed, Official,
Ecosystem available, Fragmented, and Material gap—and establishes that broad
capability availability does not provide one renewable application-platform
contract. See
[Rust capability coverage](../research/2026-08-09-rust-capability-coverage.md).

ECOS-Q02 is complete. Foundational status now means a cross-repository contract,
construction, platform, build, or implementation-substrate role with material
replacement consequences. Nineteen exact releases form the deeper verification
queue; none is approved for adoption by the census. See
[Rust foundational crate census](../research/2026-08-09-rust-foundational-crate-census.md).

ECOS-Q03 is complete. Interchange is now classified by exact and re-export
identity, trait coherence, conversion, wrappers, serialization, effective
features, semantic preservation, and runtime behavior. Compile-pass and
expected-failure fixtures establish representative boundaries. See
[Rust interchange contracts](../research/2026-08-09-rust-interchange-contracts.md).

ECOS-Q04 is complete. Async portability is now represented per operation:
Future, executor, spawn, I/O, time, cancellation, blocking, synchronization,
context, and platform. Ten exact fixtures establish representative positive,
compile-fail, and context-panic controls. See
[Rust async portability](../research/2026-08-09-rust-async-portability.md).

ECOS-Q05 is complete. Stewardship is now renewable evidence across registry
authority, publication provenance, source custody, crate-path work,
responsiveness, succession, lifecycle declarations, and replacement lineage.
Release age and activity remain review triggers rather than abandonment
verdicts. See
[Rust maintenance and stewardship](../research/2026-08-09-rust-maintenance-stewardship.md).

ECOS-Q06 is complete. Assurance is now joined evidence across archive
integrity, package and repository revision, publication authority, advisory
snapshot, lockfile and active closure, compile-time execution, unsafe and
native boundaries, audit criteria, licensing, and renewal. Zero matches and
zero direct syntax remain bounded observations rather than safety verdicts.
See
[Rust security and provenance](../research/2026-08-09-rust-security-provenance.md).

ECOS-Q07 is complete. Compatibility is now renewable evidence across exact
feature closure, Cargo/rustc pair, host/target pair, target tier,
`core`/`alloc`/`std` and architecture capability, provider, native tools, and
independently observed resolution, check, link, execution, and test stages.
Expected unsupported, failed, not-observed, stale, and unknown states remain
explicit. See
[Rust platform compatibility](../research/2026-08-09-rust-platform-compatibility.md).

ECOS-Q08 is complete. Fragmentation is now typed evidence across package and
requirement identity, duplicate and shared closure, public exposure, requested
and effective features, resolver/target/dependency-kind scope, compiler and
artifact cost, interchange consequences, remediation ownership, and renewal.
Duplicate counts and feature expansion remain diagnostics rather than automatic
rewrite authority. See
[Rust feature and version fragmentation](../research/2026-08-09-rust-feature-version-fragmentation.md).

ECOS-Q09 is complete. Native integration is now renewable evidence across
source mode, provider, host and target tools, system discovery, ABI, generated
code and bindings, Cargo directives, native component identity, artifacts,
assurance, reproducibility, and deployment. System, bundled, prebuilt,
vendored, generated, and external modes shift ownership without removing the
boundary. See
[Rust native dependency boundary](../research/2026-08-10-rust-native-dependency-boundary.md).

ECOS-Q10 is complete. Discovery and selection are now separate records across
consumer intent, retrieval source and ranking policy, candidate role and exact
identity, evidence coverage, eligibility, tradeoffs, decision, and renewal.
Search, downloads, recency, curation, and composite scores generate attributed
candidates; mandatory consumer requirements filter before preferences. See
[Rust crate discovery and selection](../research/2026-08-10-rust-crate-discovery-selection.md).

ECOS-Q11 is complete. Compatibility profiles are now renewable,
consumer-scoped evidence records rather than crate lists or a FERRIS
distribution. Six exact lanes established hosted-server, CLI/configuration,
pure-data, embedded `no_std`, browser-WASM, and bundled-native boundaries.
Profiles retain exact releases and features, lock and active-target closures,
compiler and target pairs, separate validation stages, provenance, advisory
scope, owner, expiry, renewal, removal, and rollback. See
[Rust compatibility-tested stack profiles](../research/2026-08-10-rust-compatibility-stack-profiles.md).

ECOS-Q12 is complete. Every verified gap now has an owner, intervention class,
validation gate, and non-goals. FERRIS adopts the product-neutral Crate
Ecosystem Ledger evidence vocabulary, routes contributions and stewardship
through existing owners, and advances a read-only ecosystem-ledger/profile
diff only to specification and held-out design. A FERRIS distribution,
certification, universal score, automatic dependency or environment mutation,
and unproven foundational crates are rejected or deferred. See
[Rust ecosystem intervention decisions](../research/2026-08-10-rust-ecosystem-intervention-decisions.md).

1. **ECOS-Q01: Capability coverage.** Which common application capabilities are
   in Rust `std`, the official toolchain, de facto foundational crates, or still
   materially absent?
2. **ECOS-Q02: Foundational crate census.** Which crates function as ecosystem
   infrastructure, and what evidence supports that classification?
3. **ECOS-Q03: Interchange contracts.** Where do competing types, traits, error
   models, and feature policies prevent otherwise mature crates from composing?
4. **ECOS-Q04: Async portability.** Which runtime, I/O, cancellation, timing,
   and task-local assumptions leak through library boundaries?
5. **ECOS-Q05: Maintenance and stewardship.** How can users detect maintainer
   concentration, stalled releases, ownership transfer, forks, and abandonment?
6. **ECOS-Q06: Security and provenance.** What evidence covers advisories,
   integrity, unsafe code, build scripts, procedural macros, native code,
   licensing, and release identity?
7. **ECOS-Q07: Platform compatibility.** How consistently do foundational
   crates support MSRV, operating systems, architectures, `no_std`, WASM,
   embedded targets, and cross-compilation?
8. **ECOS-Q08: Feature and version fragmentation.** How much graph duplication,
   compile cost, binary cost, and incompatibility comes from feature policies
   and simultaneous major versions?
9. **ECOS-Q09: Native dependency boundary.** Where do C/C++ libraries, system
   packages, TLS providers, code generation, and build scripts reduce
   portability or reproducibility?
10. **ECOS-Q10: Discovery and selection.** Can evidence improve crate selection
    beyond keyword search, popularity, anecdote, and stale recommendation lists?
11. **ECOS-Q11: Compatibility-tested stack profiles.** Can representative
    application stacks be tested as renewable profiles without creating a
    permanent FERRIS distribution or lock-in?
12. **ECOS-Q12: Intervention decision.** For each verified gap, should FERRIS
    document, adapt, standardize, contribute upstream, steward, prototype, or
    defer?

## Evidence model

Each question should record:

- exact crate versions, source revisions, owners, licenses, and release dates;
- complete dependency, feature, build-script, procedural-macro, native-code,
  and unsafe-code closures;
- MSRV and supported target claims compared with measured builds;
- advisories, audit or review evidence, ownership transitions, and maintenance
  signals;
- API and type compatibility across representative stack combinations;
- compile time, binary size, runtime, and operational behavior when relevant;
- negative cases, unsupported configurations, and unknowns;
- the existing owner and upstream path for every proposed intervention.

## Stage gates

### Series completion gate

The Crates Series completion gate was satisfied on 2026-08-10:

1. ECOS-Q01 through ECOS-Q12 each have a cited decision note;
2. exact crate versions, revisions, features, dependency closures, licenses,
   owners, and maintenance evidence are recorded;
3. representative application profiles cover server, CLI, data, embedded or
   `no_std`, WASM, and native-integration needs;
4. interchange, async, security, platform, feature, version, and native
   boundaries have measured controls;
5. every gap names its current owner and intervention path;
6. compatibility-tested profiles have renewal, removal, and rollback rules;
7. no crate or stack is called standard, blessed, or certified without a
   published and renewable contract;
8. the final synthesis states what OSPREY must represent about crates and
   dependency governance; and
9. all nine FERRIS roles accept the synthesis.

### Stage A: Map

Inventory capability areas and candidate crates without ranking them.

**Gate:** published taxonomy, source criteria, and at least three representative
application profiles.

### Stage B: Verify

Test dependency closures, feature combinations, target claims, interchange
boundaries, and lifecycle evidence.

**Gate:** reproducible evidence distinguishes documented capability from
observed compatibility.

### Stage C: Compare

Evaluate competing stacks against explicit consumer profiles rather than one
universal score.

**Gate:** rankings expose weighting, uncertainty, security boundaries, and
maintenance assumptions.

### Stage D: Intervene

Choose documentation, adapters, shared contracts, upstream contributions,
stewardship, or a bounded tool.

**Gate:** named consumer, owner, renewal model, rollback, validation contract,
and non-goals.

## Leading bounded opportunity

The selected FERRIS wedge is an evidence-backed, read-only Rust ecosystem
ledger and renewable profile diff:

- capability and interchange taxonomy;
- dependency-closure and lifecycle evidence;
- compatibility-tested stack profiles;
- explicit consumer requirements and tradeoffs;
- upstream ownership and gap disposition.

It advances to OSPREY specification and held-out workflow design. It is not a
replacement standard library and does not automatically approve, install,
upgrade, reject, or rewrite dependencies.

## Non-goals

- Forking Cargo or crates.io.
- Reimplementing mature foundational crates to create a FERRIS namespace.
- Declaring one async runtime, web stack, database layer, GUI, or crypto
  provider universally correct.
- Treating popularity, age, downloads, funding, or one audit as a complete
  trust score.
- Promising that dependency metadata proves safety, security, correctness, or
  future maintenance.
