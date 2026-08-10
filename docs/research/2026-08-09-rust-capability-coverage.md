# Rust Capability Coverage

Date: 2026-08-09
Status: Complete
Question: ECOS-Q01
Decision: adopt a five-class capability taxonomy and an evidence-backed
capability map. Rust has broad application capability, but much of it is
versioned and governed outside `std`. The leading gap is not a wholesale lack
of libraries; it is the absence of one renewable contract joining capability,
interchange, provider, platform, data, stewardship, security, and lifecycle
evidence. Do not rank or bless individual crates until ECOS-Q02.

## Decision supported

This research determines the taxonomy used by the remaining Crates Series:

1. what Rust guarantees through the language, `core`, `alloc`, or `std`;
2. what the official Rust project and toolchain provide;
3. what is available through external crates;
4. where capability exists but contracts or governance are fragmented; and
5. where a capability remains materially absent or primarily native-bound.

It does not select a universal async runtime, TLS provider, date-time type,
database layer, GUI toolkit, cryptography stack, or application framework.
Those decisions require the foundational-crate, interchange, stewardship,
security, platform, and stack-profile questions.

## Coverage classes

| Class | Meaning | Evidence required |
|---|---|---|
| **Guaranteed** | Language, `core`, `alloc`, or `std` contract available on its documented targets | Rust language or standard-library documentation |
| **Official** | Rust-project tool or component, but not a language or `std` API guarantee | Official Rust project documentation and release channel |
| **Ecosystem available** | One or more external crates credibly implement the capability | Primary crate documentation plus later ECOS-Q02 verification |
| **Fragmented** | Implementations exist, but runtime, type, provider, data, platform, feature, governance, or lifecycle contracts do not compose cleanly | Competing primary contracts and measured integration evidence |
| **Material gap** | No credible portable Rust implementation, or practical use primarily requires native/system capability | Primary source review plus later platform and native-boundary verification |

`Ecosystem available` is not the same as foundational, recommended, safe,
secure, maintained, or compatible. ECOS-Q02 through ECOS-Q09 establish those
properties.

## Evidence reviewed

### Rust guarantees and official tools

- Rust describes `std` as "a set of minimal and battle-tested shared
  abstractions" and the foundation of portable Rust software:
  <https://doc.rust-lang.org/std/>
- `core` is the dependency-free foundation and `alloc` provides allocation-
  backed types when a global allocator is available:
  <https://doc.rust-lang.org/core/> and
  <https://doc.rust-lang.org/alloc/>
- Rust's official tools describe Cargo build and package management, unified
  tests, documentation, and broad target support:
  <https://rust-lang.org/tools/>
- Cargo is the official package manager and build system:
  <https://doc.rust-lang.org/cargo/>
- Clippy, rustfmt, rustdoc, rustup, and Miri are official project components or
  tools with distinct support and release boundaries:
  <https://github.com/rust-lang/rust-clippy>,
  <https://github.com/rust-lang/rustfmt>,
  <https://doc.rust-lang.org/rustdoc/>,
  <https://rust-lang.github.io/rustup/>, and
  <https://github.com/rust-lang/miri>

### Capability examples

These sources establish that a capability exists. They do not complete the
ECOS-Q02 foundational-crate decision:

- Serde's trait-based data model and format ecosystem:
  <https://serde.rs/>
- Tokio's async runtime and I/O platform:
  <https://tokio.rs/>
- the `Future` contract in `std`, separate from an executor and I/O runtime:
  <https://doc.rust-lang.org/std/future/trait.Future.html>
- HTTP and middleware examples:
  <https://hyper.rs/>, <https://docs.rs/reqwest/>, <https://docs.rs/axum/>,
  <https://docs.rs/tower/>, and <https://docs.rs/tonic/>
- TLS, URL, and regular-expression examples:
  <https://rustls.dev/>, <https://docs.rs/url/>, and
  <https://docs.rs/regex/>
- date, time-zone, Unicode, and internationalization examples:
  <https://docs.rs/chrono/>, <https://docs.rs/time/>,
  <https://docs.rs/chrono-tz/>, and <https://icu4x.unicode.org/>
- cryptography and secure-storage examples:
  <https://github.com/RustCrypto>, <https://docs.rs/ring/>, and
  <https://docs.rs/keyring/>
- database, messaging, CLI, configuration, and observability examples:
  <https://docs.rs/sqlx/>, <https://docs.rs/diesel/>,
  <https://docs.rs/lapin/>, <https://docs.rs/rdkafka/>,
  <https://docs.rs/clap/>, <https://docs.rs/config/>,
  <https://docs.rs/tracing/>, and <https://docs.rs/opentelemetry/>
- GPU, GUI, audio, and media examples:
  <https://wgpu.rs/>, <https://docs.rs/egui/>, <https://docs.rs/iced/>,
  <https://gtk-rs.org/>, <https://docs.rs/cpal/>,
  <https://docs.rs/ffmpeg-next/>, and <https://gstreamer.freedesktop.org/>
- testing and measurement examples:
  <https://github.com/rust-fuzz/cargo-fuzz>,
  <https://docs.rs/proptest/>, <https://bheisler.github.io/criterion.rs/book/>,
  and <https://github.com/taiki-e/cargo-llvm-cov>

### Bundled-platform comparisons

- .NET's core libraries include data structures, I/O, security checks, data
  access, and GUI surfaces:
  <https://learn.microsoft.com/en-us/dotnet/standard/class-library-overview>
- Go's standard packages include HTTP, JSON and other encodings, cryptography,
  SQL interfaces, templates, testing, fuzzing, benchmarking, and coverage:
  <https://pkg.go.dev/std>
- Java SE defines a broad platform including HTTP, security, JDBC, XML,
  internationalization, GUI, audio, imaging, diagnostics, packaging, and
  management APIs:
  <https://docs.oracle.com/en/java/javase/25/docs/api/index.html>

## Capability matrix

| Capability area | Guaranteed or official baseline | Ecosystem availability | Coverage disposition |
|---|---|---|---|
| Primitive types, ownership, `Option`, `Result`, iterators | Language and `std` | Specialized abstractions exist | Guaranteed |
| Collections and allocation-backed containers | `alloc` and `std` | Specialized and concurrent collections exist | Guaranteed foundation |
| Synchronous I/O, files, processes, environment | `std` | Async and specialized I/O exist | Guaranteed synchronous foundation |
| Threads, atomics, locks, channels | `std` | Work stealing, data parallelism, async synchronization exist | Guaranteed primitives; ecosystem policies |
| Blocking TCP/UDP and addresses | `std` | Async networking, DNS, protocols, and service frameworks exist | Guaranteed primitive; protocol layer external |
| Package, build, workspace, test, docs | Cargo, rustc, rustdoc | Cargo extensions add policy and workflow | Official |
| Format and lint | rustfmt and Clippy | Additional linters and policy tools exist | Official with ecosystem extensions |
| Debugging and interpreter-style checking | rust-gdb/rust-lldb wrappers and Miri have official project ownership | Platform debuggers, sanitizers, coverage, and profilers vary | Official plus platform/tool fragmentation |
| Async language contract | `Future`, `async`, and `.await` | Executors, reactors, timers, networking, files, and cancellation are external | Fragmented runtime and I/O contract |
| Serialization and data formats | No general bundled application data model | Serde demonstrates a broad shared trait/data-format contract | Ecosystem available; governance external |
| HTTP, URL, middleware, gRPC | No bundled application protocol stack | Multiple mature, often composable crates exist | Ecosystem available; runtime/provider choices |
| TLS and certificate handling | No bundled TLS API | Pure-Rust and native-provider approaches exist | Ecosystem available; provider and root-policy boundary |
| Date, calendar, time zone | `Duration`, `Instant`, and `SystemTime` only | Calendar, formatting, and IANA-zone crates exist | Fragmented types and data-update lifecycle |
| Locale and Unicode services | UTF-8 strings and scalar operations; no full locale platform | ICU4X and focused Unicode crates exist | Ecosystem available; explicit data-provider lifecycle |
| Regex | No bundled regex engine | A prominent Rust regex implementation exists | Ecosystem available |
| Cryptographic primitives | No broad bundled crypto suite | RustCrypto, ring, and provider-backed stacks exist | Ecosystem available; algorithm/provider/audit policy |
| Secure credential storage | No portable bundled contract | Platform keychain wrappers exist | Fragmented and platform-native |
| SQL and ORM | No bundled database interface | Sync and async drivers, query layers, and ORMs exist | Ecosystem available; runtime/database/tooling choices |
| Messaging | No bundled broker interface | AMQP, Kafka, NATS and other clients exist | Ecosystem available; protocol and native boundaries |
| CLI and configuration | Process arguments and environment in `std` | Parsing, layered configuration, and completion crates exist | Ecosystem available |
| Logging, tracing, metrics | No bundled application observability contract | `log`, `tracing`, metrics and OpenTelemetry stacks exist | Ecosystem available; facade/exporter composition |
| Identity, OAuth and OIDC | No bundled identity platform | Protocol and provider crates exist | Fragmented claims, provider and policy contracts |
| GPU | No bundled GPU API | wgpu provides a portable WebGPU-oriented surface | Ecosystem available; driver/platform capability |
| GUI | No bundled GUI | Several distinct immediate, retained, native-binding, and web-wrapper approaches exist | Fragmented toolkit and application model |
| Audio and video | No bundled media platform | Audio wrappers exist; mature video commonly uses FFmpeg or GStreamer bindings | Native-bound; pure-Rust material gap for broad media |
| Unit, integration and documentation tests | Official test harness and rustdoc doctests | Snapshot, mocking, property, integration and framework tools exist | Official base plus ecosystem |
| Property testing and fuzzing | No stable bundled property framework; fuzzing is not a portable stable default | proptest and cargo-fuzz demonstrate availability | Ecosystem available; toolchain/platform boundary |
| Benchmarking and coverage | Compiler/project benchmarking exists; application workflows are not one stable bundled contract | Criterion and LLVM-based Cargo tools exist | Fragmented official/ecosystem tooling |
| `no_std`, embedded and WASM | `core`, optional `alloc`, target/toolchain support | Capability varies by crate features and transitive closure | Separate platform profiles required |

The detailed source-to-class mapping is recorded in
[EXP-01](ecos-q01-capability-coverage/results/EXP-01-capability-coverage-matrix.md).

## Findings

### FERRIUM-498: Rust capability is distributed across five governance classes

**Sources:** Rust `std`, official tools, and the capability matrix.

**Observed behavior:** Some capabilities are language or standard-library
guarantees, some are official tools, many are external crates, some have
multiple incompatible contracts, and a smaller set remains primarily
native-bound or absent.

**Implication:** "Does Rust have this?" is insufficient. FERRIUM must ask who
owns the contract, how it is versioned, which platforms it covers, and what
evidence renews the claim.

**Confidence:** High.

### FERRIUM-499: Rust deliberately standardizes a foundation, not an application platform

**Sources:** Rust standard-library introduction and official tools page.

**Observed behavior:** `std` explicitly describes itself as minimal shared
abstractions. It includes fundamental collections, synchronous I/O,
multithreading, files, processes, and blocking networking. It does not bundle
general HTTP, TLS, JSON, database, GUI, locale, or async-runtime platforms.

**Implication:** External crates are part of the intended ecosystem model, not
automatically evidence of immaturity. The architectural gap is how their
contracts compose and renew.

**Confidence:** High.

### FERRIUM-500: the official toolchain is a strong integrated platform layer

**Sources:** Cargo, Rust tools, rustdoc, Clippy, rustfmt, rustup, and Miri.

**Observed behavior:** Package resolution, builds, workspaces, tests,
documentation, formatting, linting, toolchain management, and interpreter-style
undefined-behavior checking have official project surfaces, although support
levels and platform coverage differ.

**Implication:** The Crates Series must not treat every non-`std` capability as
equally external. Official tools have different ownership and lifecycle from
third-party crates.

**Confidence:** High.

### FERRIUM-501: most common application capabilities have credible Rust implementations

**Sources:** the primary crate sources in the capability matrix.

**Observed behavior:** Serialization, HTTP, TLS, URL handling, regex,
date-time, Unicode, cryptography, SQL, messaging, CLI, configuration,
observability, GPU access, property testing, fuzzing, and benchmarking all have
documented Rust implementations.

**Implication:** ECOS-Q02 should evaluate which implementations are
foundational. FERRIUM should not begin by reimplementing these capabilities.

**Confidence:** High for availability; no maturity or selection claim is made.

### FERRIUM-502: ecosystem contracts can become platform infrastructure without entering `std`

**Sources:** Serde's data model and format separation; HTTP middleware and
service examples.

**Observed behavior:** Serde lets data structures and formats interoperate
through shared traits. HTTP stacks can share request, body, and service
abstractions across multiple crates. These contracts coordinate ecosystems
without standard-library ownership.

**Implication:** A missing `std` API does not imply a missing standardizing
mechanism. ECOS-Q03 must identify de facto interchange contracts and their
governance.

**Confidence:** High for the examples; generality requires ECOS-Q03.

### FERRIUM-503: async portability is the largest cross-cutting contract gap

**Sources:** `Future`, Tokio, HTTP, gRPC, SQL, and messaging crate documents.

**Observed behavior:** Rust standardizes futures and syntax, but executors,
reactors, timers, async I/O traits, spawning, cancellation, and task-local
behavior are supplied externally. Application stacks frequently expose
runtime assumptions through features or APIs.

**Implication:** Async capability cannot be represented as one Boolean.
OSPREY must record runtime, I/O, timer, cancellation, task, synchronization,
and blocking-work contracts. ECOS-Q04 receives dedicated investigation.

**Confidence:** High.

### FERRIUM-504: data-bearing capabilities require update and deployment policy

**Sources:** chrono-tz and ICU4X documentation; TLS provider and certificate
root choices.

**Observed behavior:** Time-zone rules, locale data, Unicode data, certificate
roots, and related policy are versioned data dependencies. They can be embedded,
generated, loaded from the OS, or supplied at runtime.

**Implication:** Capability evidence must include data source, version, update,
size, deployment, fallback, and expiry—not merely a crate version.

**Confidence:** High.

### FERRIUM-505: provider choice is part of capability identity

**Sources:** rustls provider model, native TLS alternatives, keyring platform
stores, database drivers, and messaging clients.

**Observed behavior:** Two crates can expose "TLS," "secrets," "SQL," or
"messaging" while relying on different cryptographic providers, root stores,
OS facilities, native libraries, runtimes, protocols, and operational
assumptions.

**Implication:** The Crate Ecosystem Ledger must record provider and capability
identity separately from the top-level crate name.

**Confidence:** High.

### FERRIUM-506: duplicated capability types create real integration cost

**Sources:** multiple date-time, error, async I/O, HTTP body, TLS, database,
logging, and metrics contracts in the capability inventory.

**Observed behavior:** Capable crates may use different date-time types,
duration types, error models, I/O traits, runtime handles, request bodies,
provider types, or feature conventions. Adapters can exist while semantic
conversion and lifecycle remain application-owned.

**Implication:** ECOS-Q03 must measure interchange cost rather than comparing
feature lists alone.

**Confidence:** Medium-high pending measured stack combinations.

### FERRIUM-507: native-bound capability remains a material portability frontier

**Sources:** keyring, rdkafka, cpal, FFmpeg, GStreamer, GTK, and platform TLS
documentation.

**Observed behavior:** Secure storage, Kafka, audio, video, native widgets,
system TLS, and other capabilities can depend on C/C++ libraries, OS services,
drivers, package managers, or platform APIs.

**Implication:** "Available on crates.io" does not establish Cargo-only
installation or cross-compilation. ECOS-Q09 must expose native ownership,
ABI, packaging, licensing, update, and recovery boundaries.

**Confidence:** High.

### FERRIUM-508: assurance tooling has an official core and an external edge

**Sources:** Cargo tests, rustdoc doctests, Clippy, Miri, cargo-fuzz, proptest,
Criterion, and cargo-llvm-cov.

**Observed behavior:** Unit, integration, documentation testing, linting, and
some interpreter-based checks have official ownership. Property testing,
portable fuzz workflows, statistical application benchmarking, and coverage
typically add external tools, nightly requirements, or platform constraints.

**Implication:** Stack profiles must include development and assurance tools,
not only runtime dependencies, and must record toolchain and CI requirements.

**Confidence:** High.

### FERRIUM-509: `std`, `no_std`, embedded, WASM, and desktop-server profiles are different platforms

**Sources:** `core`, `alloc`, Rust targets, and capability crate feature
documentation.

**Observed behavior:** A capability may work with `std` but not `no_std`, need
allocation, use threads or sockets unavailable on a target, require JavaScript
bindings in WASM, or depend on native code during cross-compilation.

**Implication:** Platform coverage cannot be inherited from the top-level crate
or from successful desktop compilation. ECOS-Q07 must evaluate complete
feature and dependency closures.

**Confidence:** High.

### FERRIUM-510: broader bundled libraries mainly change governance and versioning

**Sources:** official .NET, Go, and Java platform documentation.

**Observed behavior:** .NET, Go, and Java bundle more application capabilities
under one platform release and documentation system. Rust often provides the
same capability through separately versioned crates and maintainers.

**Implication:** Comparisons must distinguish missing implementation from
distributed governance, update independence, substitution, and integration
cost. A larger standard library is not automatically superior, but it provides
a stronger common lifecycle contract.

**Confidence:** High.

### FERRIUM-511: the leading gap is a renewable application-platform contract

**Sources:** all evidence above and the Crates Series plan.

**Observed behavior:** Broad implementation availability coexists with
fragmented ownership, runtime and provider choice, target support, data
updates, advisories, native dependencies, feature closures, maintenance, and
interchange.

**Implication:** FERRIUM should build the evidence model for capability and
compatibility before considering a curated distribution or new foundational
crate. OSPREY must represent these properties in its Ecosystem adapter and
Crate Ecosystem Ledger.

**Confidence:** High.

### FERRIUM-512: capability mapping must precede crate ranking

**Sources:** ECOS-Q01 scope and the capability inventory.

**Observed behavior:** One crate can cover several capabilities, and one
capability can require several crates, data sources, providers, tools, or
native components. Popularity cannot resolve consumer requirements or closure
risk.

**Implication:** ECOS-Q02 will classify foundational crates against the frozen
capability taxonomy rather than starting from download rankings.

**Confidence:** High.

## Decision

### Adopt now

- Use the five coverage classes throughout the Crates Series.
- Maintain a capability map separate from the foundational-crate census.
- Record provider, runtime, data, platform, native, assurance, stewardship, and
  lifecycle dimensions for every capability.
- Treat official tools separately from language and `std` guarantees.
- Add capability, coverage class, owner, provider, data source, platform
  profile, and gap disposition to OSPREY's future Ecosystem adapter.

### Prototype behind a compatibility boundary

- A read-only capability matrix with source dates and explicit unknowns.
- Disposable stack probes used by later ECOS questions.
- Cross-platform and `no_std`/WASM profile checks after exact crate candidates
  are selected.

No crate installation, ranking, certification, automatic dependency change, or
OSPREY implementation is authorized.

Owner: FERRIUM. Expected validation: ECOS-Q02 exact-candidate census followed
by the closure, platform, security, and stack probes assigned to ECOS-Q03
through ECOS-Q11. Non-goals: selecting crates, publishing a compatibility
profile, changing dependencies, or implementing OSPREY.

### Reject or defer

- a larger FERRIUM standard library;
- a FERRIUM namespace that reimplements available capability;
- universal async, TLS, database, GUI, cryptography, or observability choices
  before consumer profiles and closure evidence exist;
- treating downloads, GitHub activity, age, one audit, or `std` inclusion as a
  complete quality score;
- treating top-level documentation as transitive platform proof; and
- calling a stack standard, blessed, certified, safe, or portable before its
  criteria and renewal model are defined.

## Role review

### Rust Safety Steward

Accepts the separation between capability availability and safety evidence.
Requires ECOS-Q06 to inspect unsafe, macros, build scripts, native code,
provider identity, advisories, and complete closures before trust claims.

### Compiler Performance Engineer

Accepts capability mapping without performance ranking. Requires ECOS-Q08 and
later stack profiles to measure feature/version multiplication, proc-macro and
build-script work, compile time, binary size, runtime, and assurance-tool cost.

### Interop Boundary Auditor

Accepts provider and native boundaries as first-class. Requires ECOS-Q03 and
ECOS-Q09 to test type conversion, ABI, ownership, panic, threading,
allocation, generated bindings, and negative cases.

### AI Assurance Skeptic

Accepts the five classes and the explicit statement that availability is not
quality proof. Requires source dates, exact versions, unsupported cases,
failed probes, and human approval before selection or certification.

### Ecosystem Strategist

Accepts the evidence-backed application-platform map as a defensible gap.
Rejects replacing mature crates and requires ECOS-Q02 to identify owners,
standards, adoption paths, and upstream intervention opportunities.

### Rust Maintainer

Accepts a capability-first taxonomy that avoids premature crate mandates.
Requires later outputs to explain tradeoffs in ordinary terms and avoid
dependency churn or framework lock-in.

### Native Platform Adopter

Accepts bundled-versus-external, provider, data, native, and platform
distinctions. Requires measured Windows, Linux, macOS, container, WASM,
embedded, installation, update, rollback, compliance, and support evidence
where relevant.

### Scope Keeper

Accepts ECOS-Q01 as taxonomy only. Individual crate ranking, async selection,
security scoring, platform verification, compatibility profiles, and OSPREY
implementation remain assigned to later questions or gates.

### Validation Checker

Accepts the primary-source capability inventory and limitations. Requires
exact-version and measured-closure evidence before any candidate moves beyond
`Ecosystem available`.

## Limitations

- This is a broad source review, not a complete crates.io census.
- Examples demonstrate capability availability but do not establish
  foundational status, maintenance quality, safety, security, or portability.
- Exact versions, dependency closures, features, licenses, owners, and release
  histories are intentionally deferred to ECOS-Q02.
- No stack was built across the full platform matrix.
- Identity and GUI areas are fragmented and were sampled rather than
  exhaustively enumerated.
- Security audit currency was not evaluated.
- Official support levels differ across Rust project tools.
