# Crates Series: Ecosystem and Library Research Program

Status: Next research program

## Series identity

The twelve ECOS questions are the **Crates Series**. They are the next FERRIUM
research sequence after PERF-Q01 through PERF-Q36.

The Crates Series is also Phase 0 of the
[OSPREY program](OSPREY_PROGRAM.md). No OSPREY implementation package,
executable, service, repository adapter, or production integration may begin
until ECOS-Q01 through ECOS-Q12 are complete and receive a final nine-role
review.

OSPREY architecture planning may continue during the series, but it remains
specification-only.

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

FERRIUM should investigate that application-platform contract before creating
another standard library, package manager, framework, or curated distribution.

## Decision this program informs

Determine whether FERRIUM should:

1. explain the ecosystem through an evidence-backed capability map;
2. define compatibility and assurance profiles for common crate stacks;
3. contribute missing traits, adapters, metadata, tests, or governance
   mechanisms upstream;
4. prototype a bounded selection or certification tool; or
5. reject an apparent gap because an established crate or ecosystem program
   already owns it.

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
  stewardship over a FERRIUM-owned replacement stack.
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
sequence. Findings continue the global `FERRIUM-XX` sequence.

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
    permanent FERRIUM distribution or lock-in?
12. **ECOS-Q12: Intervention decision.** For each verified gap, should FERRIUM
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

The Crates Series is complete only when:

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
9. all nine FERRIUM roles accept the synthesis.

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

The first plausible FERRIUM wedge is an evidence-backed Rust application
platform map:

- capability and interchange taxonomy;
- dependency-closure and lifecycle evidence;
- compatibility-tested stack profiles;
- explicit consumer requirements and tradeoffs;
- upstream ownership and gap disposition.

It is not a replacement standard library and does not automatically approve,
install, upgrade, or reject dependencies.

## Non-goals

- Forking Cargo or crates.io.
- Reimplementing mature foundational crates to create a FERRIUM namespace.
- Declaring one async runtime, web stack, database layer, GUI, or crypto
  provider universally correct.
- Treating popularity, age, downloads, funding, or one audit as a complete
  trust score.
- Promising that dependency metadata proves safety, security, correctness, or
  future maintenance.
