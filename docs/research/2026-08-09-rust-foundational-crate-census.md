# Rust Foundational Crate Census

Date: 2026-08-09
Status: Complete
Question: ECOS-Q02
Decision: use a role-based foundational-crate test and carry nineteen exact
crate releases into ECOS-Q03 through ECOS-Q09. The set is a verification queue,
not an approved stack or dependency recommendation.

## Decision supported

ECOS-Q02 determines which external crates have enough cross-project contract,
construction, platform, build, or implementation leverage to require deeper
Crates Series verification.

It does not decide:

- which crates an application should adopt;
- whether any crate is safe, secure, maintained, portable, or compatible;
- which async runtime, TLS provider, database, GUI, or application framework
  should be preferred; or
- whether a high-download crate should become a FERRIUM standard.

## Foundational test

A crate is foundational when its replacement or failure crosses repository
boundaries because it performs at least one structural role:

1. **Contract foundation:** exports types or traits that independently authored
   crates expose in public APIs.
2. **Construction foundation:** supplies the token, syntax, code-generation, or
   derive substrate used to construct many other crates.
3. **Platform foundation:** normalizes an operating-system or target capability
   below application policy.
4. **Build foundation:** controls compilation, discovery, linking, or generated
   native inputs during a Cargo build.
5. **Implementation substrate:** is embedded beneath many unrelated crates and
   carries material safety, performance, or replacement consequences even
   without defining a public interchange contract.

The role must also have evidence of cross-domain reach and material blast
radius. Download or reverse-dependency counts corroborate reach but cannot
establish correctness, governance, safety, or foundational status alone.

The following are not sufficient:

- popularity;
- being useful in many applications;
- having a large feature set;
- being the leading implementation in one domain;
- being a convenient derive or error helper; or
- appearing transitively without creating cross-project replacement cost.

## Selected verification queue

### Contract foundations

| Crate | Exact release | Shared contract |
|---|---:|---|
| `serde_core` | 1.0.229 | serialization and deserialization traits |
| `serde` | 1.0.229 | user-facing serialization facade and trait re-exports |
| `log` | 0.4.33 | logging facade, metadata, records, and logger trait |
| `tracing-core` | 0.1.36 | structured event, span, metadata, and subscriber contracts |
| `bytes` | 1.12.1 | byte-buffer types and `Buf`/`BufMut` contracts |
| `http` | 1.5.0 | request, response, URI, status, method, and header types |
| `tower-service` | 0.3.3 | request-to-response service trait |
| `futures-core` | 0.3.33 | stream and core asynchronous traits |
| `rand_core` | 0.10.1 | random-generator and seed contracts |

### Construction foundations

| Crate | Exact release | Construction role |
|---|---:|---|
| `proc-macro2` | 1.0.107 | token-stream types outside compiler procedural-macro context |
| `quote` | 1.0.47 | token interpolation and `ToTokens` contract |
| `syn` | 3.0.3 | parsed Rust syntax tree used by procedural macros |

### Platform and build foundations

| Crate | Exact release | Boundary |
|---|---:|---|
| `libc` | 0.2.189 | C and operating-system ABI declarations |
| `cfg-if` | 1.0.4 | target-conditional source selection |
| `getrandom` | 0.4.3 | target entropy acquisition |
| `cc` | 1.4.2 | C/C++ compiler invocation from build scripts |
| `pkg-config` | 0.3.33 | native library discovery and linker metadata |

### Implementation substrates

| Crate | Exact release | Substrate role |
|---|---:|---|
| `hashbrown` | 0.17.1 | hash-table implementation with Rust-project integration |
| `memchr` | 2.8.3 | optimized byte-search implementation beneath parsers and regex engines |

These nineteen releases are fully identified by crates.io checksums in
[EXP-01](ecos-q02-foundational-crate-census/results/EXP-01-foundational-crate-census.md).

## Measured evidence

The census queried the crates.io API on 2026-08-09 and resolved each exact
release in an isolated Cargo manifest with default features:

```text
cargo metadata --format-version 1 \
  --filter-platform x86_64-pc-windows-msvc \
  --manifest-path <isolated-probe>/Cargo.toml
```

Environment:

```text
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
target filter: x86_64-pc-windows-msvc
```

| Crate | Reverse dependencies | Recent downloads, millions | Declared Rust version | Default resolved packages |
|---|---:|---:|---:|---:|
| `serde` 1.0.229 | 113,469 | 260.1 | 1.56 | 2 |
| `log` 0.4.33 | 29,763 | 238.9 | 1.71.0 | 1 |
| `syn` 3.0.3 | 17,037 | 444.5 | 1.71 | 4 |
| `quote` 1.0.47 | 16,641 | 313.1 | 1.71 | 3 |
| `bytes` 1.12.1 | 15,228 | 212.5 | 1.57 | 1 |
| `proc-macro2` 1.0.107 | 14,720 | 298.1 | 1.71 | 2 |
| `libc` 0.2.189 | 13,594 | 316.2 | 1.65 | 1 |
| `http` 1.5.0 | 7,075 | 210.1 | 1.57.0 | 3 |
| `getrandom` 0.4.3 | 3,880 | 492.3 | 1.85 | 2 |
| `cc` 1.4.2 | 3,814 | 244.7 | 1.64.0 | 3 |
| `cfg-if` 1.0.4 | 2,696 | 272.3 | 1.32 | 1 |
| `rand_core` 0.10.1 | 2,266 | 367.6 | 1.85 | 1 |
| `hashbrown` 0.17.1 | 2,126 | 584.0 | 1.85.0 | 4 |
| `futures-core` 0.3.33 | 1,961 | 195.6 | 1.36 | 1 |
| `memchr` 2.8.3 | 1,788 | 298.7 | 1.61 | 1 |
| `pkg-config` 0.3.33 | 1,150 | 143.6 | 1.31 | 1 |
| `tower-service` 0.3.3 | 663 | 127.8 | Not declared | 1 |
| `tracing-core` 0.1.36 | 315 | 171.3 | 1.65.0 | 2 |
| `serde_core` 1.0.229 | 203 | 214.3 | 1.56 | 1 |

Reverse dependencies are direct crates.io relationships across published
versions. They undercount transitive exposure and can be distorted by facade
crates, version families, generated releases, and optional dependencies.

Default closure size is also not typical-stack size. For example, Tokio's
default feature set resolves only two packages in the same probe, while a
network application enables additional runtime, I/O, macro, synchronization,
and platform features. ECOS-Q08 will measure representative feature closures.

## Comparison cohort

| Disposition | Crates | Reason |
|---|---|---|
| Domain foundation | `tokio`, `regex`, `url`, `rustls` | Material within async, text, web, or TLS domains, but each supplies domain policy or implementation rather than a universal contract |
| Application or framework choice | `clap`, `sqlx`, `wgpu` | Consumer profile and feature selection dominate; evaluate in ECOS-Q11 stacks |
| Widely reused utility | `bitflags`, `smallvec`, `once_cell`, `indexmap` | High reach and public types, but replacement remains more local than the selected contract and construction hubs |
| Focused helper | `thiserror`, `anyhow` | Error construction or aggregation helpers; they do not define a shared ecosystem error identity |

Exclusion from the foundational queue is not a quality judgment. These crates
remain candidates where their capability is present in a representative stack.

## Findings

### FERRIUM-513: foundational status is a structural role, not a rank

**Sources:** ECOS-Q01 taxonomy, crates.io API measurements, and exact Cargo
closures in EXP-01.

**Observed behavior:** High-download crates include contract hubs, build tools,
domain implementations, application frameworks, and small helpers. Their
replacement consequences differ even when popularity is similar.

**Implication:** FERRIUM must classify role before comparing popularity,
maintenance, security, or performance.

**Confidence:** High.

### FERRIUM-514: shared public types create the strongest ecosystem lock

**Sources:** primary documentation for Serde, log, tracing-core, bytes, http,
tower-service, futures-core, and rand_core.

**Observed behavior:** These crates define types or traits that independently
authored libraries expose and implement. Replacing one requires coordinated
changes across producer and consumer repositories.

**Implication:** ECOS-Q03 should begin with type identity, trait compatibility,
error conversion, feature policy, and version coexistence for these nine
contract crates.

**Confidence:** High.

### FERRIUM-515: facade and core crates need separate identities

**Sources:** `serde` 1.0.229 and `serde_core` 1.0.229 package manifests and
crates.io dependency relationships.

**Observed behavior:** `serde` is the established facade while `serde_core`
contains the lower-level trait contract. Their reverse-dependency counts,
features, build behavior, and adoption histories differ.

**Implication:** OSPREY must not collapse a facade family into one crate name.
It needs package, contract-family, re-export, and feature identities.

**Confidence:** High.

### FERRIUM-516: procedural macros have a shared construction plane

**Sources:** proc-macro2, quote, and syn package documentation and measured
closures.

**Observed behavior:** The three crates provide token, interpolation, and Rust
syntax substrates used by many unrelated procedural macros. They are not an
application capability, yet changes can multiply compile work and version
duplication across the graph.

**Implication:** ECOS-Q03, ECOS-Q05, ECOS-Q06, and ECOS-Q08 must treat code-
generation infrastructure as a first-class dependency system.

**Confidence:** High.

### FERRIUM-517: build foundations carry effects outside the Rust type graph

**Sources:** cc and pkg-config primary documentation and manifests.

**Observed behavior:** `cc` invokes native compilers; `pkg-config` discovers
system libraries and emits linker metadata. Their one-to-three-package default
closures do not represent compilers, headers, environment variables, package
databases, linkers, or native libraries they activate.

**Implication:** Dependency count cannot measure build-boundary risk.
ECOS-Q09 must record tools, environment, generated outputs, native packages,
ABI, licensing, and cross-compilation behavior.

**Confidence:** High.

### FERRIUM-518: platform foundations encode target policy

**Sources:** libc, cfg-if, and getrandom package sources and target-specific
manifests.

**Observed behavior:** These crates normalize ABI declarations, conditional
compilation, and entropy acquisition across targets. Their behavior depends on
the selected target and, for some configurations, explicit provider features.

**Implication:** ECOS-Q07 must evaluate target-feature closures rather than
assuming that a successful host build establishes portability.

**Confidence:** High.

### FERRIUM-519: implementation substrates deserve review without becoming contracts

**Sources:** hashbrown and memchr package metadata, source, and crates.io reach.

**Observed behavior:** Both crates sit beneath many unrelated crates and expose
material performance and unsafe-code review surfaces, but their primary role is
implementation rather than cross-ecosystem type interchange.

**Implication:** FERRIUM needs an implementation-substrate category so safety
and performance review does not falsely elevate every optimized utility into a
standard contract.

**Confidence:** High.

### FERRIUM-520: reverse-dependency counts are useful but semantically incomplete

**Sources:** crates.io reverse-dependency API.

**Observed behavior:** `serde` reports 113,469 direct reverse dependencies,
while the newer `serde_core` reports 203 despite being the lower-level trait
package. `tracing-core` similarly sits behind higher-level facades.

**Implication:** Reach evidence must include facade, re-export, transitive,
optional, version-family, and public-API relationships.

**Confidence:** High.

### FERRIUM-521: default dependency closure can hide ordinary usage

**Sources:** isolated Cargo metadata probes.

**Observed behavior:** Minimal default closures range from one package for
contract crates to 132 for SQLx and 83 for wgpu on the Windows target filter.
Tokio resolves only two packages by default because major runtime capabilities
are feature-selected.

**Implication:** ECOS-Q08 must measure named consumer profiles and enabled
features. Default-feature closure cannot stand in for ecosystem cost.

**Confidence:** High.

### FERRIUM-522: MSRV is already a selection boundary within infrastructure

**Sources:** crates.io `rust_version` fields for exact selected releases.

**Observed behavior:** Declared Rust versions in the queue range from 1.31 for
pkg-config and 1.32 for cfg-if to 1.85 for getrandom, rand_core, and hashbrown.
Tower-service does not declare one in the published metadata.

**Implication:** ECOS-Q07 must record declared, resolved, and measured MSRV by
feature and target; "foundational" cannot imply one portfolio-wide MSRV.

**Confidence:** High.

### FERRIUM-523: package ownership data is not stewardship proof

**Sources:** crates.io owners endpoints and repository URLs in EXP-01.

**Observed behavior:** The queue includes Rust-project teams, ecosystem
organizations, named users, and publish teams. The API establishes publishing
authority, not review capacity, succession, funding, bus factor, or incident
response.

**Implication:** ECOS-Q05 must inspect governance and maintenance history rather
than converting owner counts into risk scores.

**Confidence:** High.

### FERRIUM-524: package checksum is the available release identity

**Sources:** crates.io version API.

**Observed behavior:** Every selected release has a registry checksum, version,
publisher, release time, and repository URL. A corresponding repository commit
is not guaranteed by the registry record.

**Implication:** The Crate Ecosystem Ledger must preserve registry checksum and
source origin separately from an optional repository revision.

**Confidence:** High.

### FERRIUM-525: common utility and domain success do not create universal infrastructure

**Sources:** comparison-cohort metadata and primary documentation.

**Observed behavior:** Error helpers, ordered maps, lazy initialization, CLI,
SQL, GPU, TLS, URL, regex, and async runtime crates can be widely used while
remaining substitutable utilities, domain foundations, or consumer choices.

**Implication:** ECOS-Q11 should evaluate them inside representative profiles,
not impose them as global foundations.

**Confidence:** High.

### FERRIUM-526: the census is a verification queue, not adoption authority

**Sources:** ECOS-Q02 decision and Crates Series gates.

**Observed behavior:** Q02 measured identity, reach, declared MSRV, owners, and
minimal closures. It did not establish unsafe closure, security, maintenance,
platform support, interoperability, native reproducibility, or representative
stack compatibility.

**Implication:** No selected crate may be called approved, blessed, certified,
or portfolio-standard before ECOS-Q03 through ECOS-Q11 close the relevant
evidence.

**Confidence:** High.

## Decision

### Adopt now

- Adopt the five structural roles and the nineteen-release verification queue.
- Preserve registry checksum, owner set, repository, license, release date,
  declared Rust version, features, and measured closure as separate evidence.
- Begin ECOS-Q03 with contract and construction foundations.
- Carry platform, build, and implementation foundations into the later
  platform, security, fragmentation, and native-boundary questions.

Owner: FERRIUM.

Expected validation: exact-version type and feature probes in ECOS-Q03,
runtime tests in ECOS-Q04, governance history in ECOS-Q05, source and
provenance review in ECOS-Q06, target/MSRV builds in ECOS-Q07, feature/version
measurements in ECOS-Q08, native-boundary probes in ECOS-Q09, and
representative stack profiles in ECOS-Q11.

Non-goals: adding dependencies, ranking application crates, selecting a
runtime/provider/framework, or implementing OSPREY.

### Prototype behind a compatibility boundary

- Disposable public-API compatibility fixtures for the nine contract crates.
- Isolated procedural-macro graphs for syn 2/3 and related version families.
- Target-specific platform and native build probes.
- Read-only family/re-export/version views for the future Ecosystem adapter.

### Reject or defer

- dependency changes based on this census;
- "safe to adopt" conclusions from downloads, owners, age, or a small closure;
- treating crates.io owner count as bus-factor evidence;
- treating registry version as a repository commit;
- treating a no-release period as abandonment without maintenance evidence;
- universal selection of Tokio, rustls, clap, SQLx, wgpu, or any other domain
  implementation; and
- expanding the queue merely because another crate is popular.

## Role review

### Rust Safety Steward

Accepts the queue only as prioritization. Requires ECOS-Q06 to distinguish
unsafe token indicators from unsafe blocks, APIs, invariants, generated code,
dependencies, and soundness evidence.

### Compiler Performance Engineer

Accepts the construction and default-closure measurements. Requires ECOS-Q08
to measure proc-macro execution, build scripts, feature closures, simultaneous
versions, compile time, and binary effects in named profiles.

### Interop Boundary Auditor

Accepts the contract-first ordering. Requires ECOS-Q03 to test type identity,
trait implementation, adapters, errors, ownership, pinning, threading, and
negative version combinations.

### AI Assurance Skeptic

Accepts exact versions, checksums, commands, unknowns, and exclusions. Rejects
automatic quality, security, or adoption conclusions from census metrics.

### Ecosystem Strategist

Accepts the role-based test and upstream-owner preservation. Requires later
interventions to prefer documentation, compatibility, contribution, and
stewardship over replacement.

### Rust Maintainer

Accepts a bounded queue rather than a dependency mandate. Requires ordinary
Cargo workflows, feature control, MSRV visibility, and removal paths.

### Native Platform Adopter

Accepts platform and build foundations as separate from Rust-only contracts.
Requires Windows, Linux, macOS, container, cross-compilation, toolchain, and
native-package evidence where applicable.

### Scope Keeper

Accepts Q02 as census and prioritization only. Interchange, async, governance,
security, platform, fragmentation, native behavior, discovery, and stack
selection remain assigned to later questions.

### Validation Checker

Accepts crates.io snapshots and reproducible default-closure commands.
Requires exact features and representative target matrices before any broader
claim.

## Limitations

- The census is bounded, not exhaustive across crates.io.
- Reverse dependencies and downloads are volatile point-in-time signals.
- Default features do not represent typical application profiles.
- Cargo metadata resolves packages but does not prove successful compilation.
- The source scan counted lexical `unsafe` tokens only for triage and is not a
  safety audit.
- Crates.io ownership does not establish governance quality.
- Repository commits corresponding to registry releases were not established.
- Security advisories, audit currency, maintenance history, complete unsafe
  closure, all targets, all features, and simultaneous versions are deferred.
