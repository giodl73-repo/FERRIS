# Rust Platform Compatibility

Date: 2026-08-09
Status: Complete
Question: ECOS-Q07
Decision: represent compatibility as renewable evidence for an exact package,
feature and dependency closure, Cargo/rustc pair, host/target pair, target tier,
library capability, provider configuration, native tools, and observed
resolution, check, link, execution, and test stages. Preserve unsupported,
failed, not-observed, and unknown states instead of assigning one portable
label.

## Decision supported

ECOS-Q07 defines the platform evidence required by the OSPREY Ecosystem adapter
and Crate Ecosystem Ledger.

It does not:

- certify any selected crate as portable;
- prove an MSRV below the compiler that was observed;
- treat a top-level `rust-version` as coverage for every feature closure;
- treat a Rust target tier as crate, linker, runtime, ABI, or operational
  support;
- infer `no_std` support from the presence of `#![no_std]`;
- infer cross-compilation from `cargo check --target`;
- infer native compiler or system-library availability from compiling `cc` or
  `pkg-config` as Rust libraries; or
- authorize OSPREY implementation.

## Platform evidence model

| Dimension | Required evidence |
|---|---|
| Package identity | Registry, exact package and version, checksum-covered archive, source revision, and observation time |
| Compiler compatibility | Declared Rust version or policy, observed rustc and Cargo versions, root edition, resolver behavior, lock selection, feature closure, pass/fail stage, and prior-version state |
| Target identity | Host triple, target triple, Rust target tier, installed component, standard-library availability, architecture capabilities, ABI, and target specification |
| Library capability | Required `std`, `alloc`, or `core`; atomic and pointer-width requirements; panic and unwind assumptions; and target-specific cfg |
| Feature and closure | Requested default and explicit features, effective target/dependency-kind features, selected transitive versions, build scripts, macros, native providers, and generated code |
| Provider/backend | Default, selected, custom, unsupported, or externally supplied backend plus owner, configuration scope, safety assumptions, and failure behavior |
| Native prerequisites | Compiler, archiver, linker, SDK, sysroot, system package, `pkg-config` paths, environment, versions, and artifact identity |
| Validation stage | Metadata resolution, package-root check, consumer check, build, link, execute, test, example, doctest, and deployment state |
| Result state | Supported by declaration, observed pass, expected unsupported, unexpected failure, skipped, not observed, stale, or unknown |
| Renewal | Immutable observation, source timestamp, target/toolchain/feature changes, expiration, replacement command, and owner |

Compatibility is a vector across these dimensions, not a scalar score.

## Measured queue

The nineteen ECOS-Q02 exact releases were measured. Commands, per-target
counts, exact MSRV results, feature/provider controls, link results, sources,
and limitations are in
[EXP-01](ecos-q07-platform-compatibility/results/EXP-01-platform-compatibility-matrix.md).

Observed:

- a 190-case package-root matrix produced 149 passes and 41 failures;
- all nineteen default-feature package roots checked on Windows, x86-64 Linux,
  AArch64 Linux, and x86-64 macOS targets;
- package-root default checks passed for 17 of 19 releases on
  `wasm32-unknown-unknown`, 18 of 19 on `wasm32-wasip2`, and 5 of 19 on each
  selected bare-metal target;
- disabling default features increased the selected Thumb bare-metal result
  from 5 to 11 package-root passes;
- two Serde Core package-root WASM failures were package lint failures, while
  minimal consumers compiled on both WASM targets;
- all eighteen releases with a declared Rust version compiled as default
  minimal consumers on that compiler; `tower-service 0.3.3`, which declares no
  Rust version, compiled in the observed Rust 1.31 consumer;
- Serde's default closure compiled on Rust 1.56, while the exact current derive
  closure could not be consumed by Cargo 1.56 because its Syn 3 manifest uses
  newer namespaced feature syntax;
- `bytes 1.12.1` compiled without default features on
  `thumbv7em-none-eabihf`, but a no-CAS `thumbv6m-none-eabi` consumer required
  both `extra-platforms` and an explicit `portable-atomic` provider;
- `getrandom 0.4.3` deliberately rejected default
  `wasm32-unknown-unknown`, accepted `wasm_js`, and compiled for WASIp2;
- `http 1.5.0` deliberately rejected its no-default-features configuration;
- `proc-macro2`, `quote`, and `syn` declare `#![no_std]` but directly import
  `std` and failed the selected bare-metal consumer;
- a representative binary linked for Windows, unknown WASM, and WASIp2, while
  Linux and macOS cross-links failed on the Windows host because required
  linkers and the Apple SDK were absent; and
- the Windows artifact executed, while WASM execution was not observed because
  no matching runtime was installed.

These observations do not approve the queue for any application profile.

## Findings

### FERRIUM-592: compatibility is stage-specific evidence

**Sources:** 190 package-root checks, nineteen MSRV consumer checks, focused
feature controls, and representative link and execution controls in EXP-01.

**Observed behavior:** The same target-feature closure could resolve and check
while linking or execution remained unavailable. Package-root and consumer
checks could also differ because package lint and development policy apply to
different scopes.

**Implication:** OSPREY must record each validation stage independently. A
passing earlier stage must not be promoted to a later-stage support claim.

**Confidence:** High.

### FERRIUM-593: Rust target tiers do not establish crate support

**Sources:** Rust platform-support documentation and EXP-01.

**Observed behavior:** Tier 1 and Tier 2 targets describe Rust project build,
test, host-tool, and standard-library guarantees. They do not guarantee that a
crate enables the right features, selects a provider, links with locally
installed tools, executes, or passes its own tests.

**Implication:** Retain target tier as one upstream claim beside crate,
environment, link, run, and test evidence.

**Confidence:** High.

### FERRIUM-594: the selected default closures met their declared MSRV checks

**Sources:** nineteen edition-2018 minimal consumers and ten installed
historical toolchains in EXP-01.

**Observed behavior:** All eighteen declared Rust versions passed for exact
default dependencies. `tower-service 0.3.3` declared none and passed the
selected Rust 1.31 observation.

**Implication:** Record these as exact observed passes, not proven minima or
future promises. Absence of a declared version remains unknown policy.

**Confidence:** High for the observed closures; low for any unmeasured feature.

### FERRIUM-595: MSRV belongs to a feature closure and Cargo/rustc pair

**Sources:** Serde default and derive controls, Cargo Rust-version and resolver
documentation, and exact package manifests.

**Observed behavior:** Serde's default exact dependency compiled with Cargo and
rustc 1.56. Its current derive closure selects Syn 3, whose normalized manifest
uses namespaced `dep:` features and declares Rust 1.71. Cargo 1.56 rejected the
manifest before rustc could compile it.

**Implication:** Store declared and observed compiler compatibility per feature
closure, selected transitive versions, Cargo client, resolver policy, and
lockfile. A facade's top-level field cannot stand in for optional tooling.

**Confidence:** High.

### FERRIUM-596: package-root and consumer compatibility are different scopes

**Sources:** Serde Core package-root and minimal-consumer WASM checks.

**Observed behavior:** Direct package-root checks failed on both WASM targets
because package lint policy rejected unused target-specific imports. Exact
minimal consumers passed because dependency compilation caps dependency lints.

**Implication:** Record package-self-validation and downstream-consumer
compilation separately. Neither should silently overwrite the other.

**Confidence:** High.

### FERRIUM-597: `no_std` is not a sufficient compatibility label

**Sources:** exact source, default/no-default matrix, and bare-metal consumer
controls.

**Observed behavior:** `proc-macro2`, `quote`, and `syn` contain
`#![no_std]` but also directly import `std`; all three failed the selected
bare-metal consumer. Other crates required default features to be disabled.

**Implication:** Record actual `core`, `alloc`, and `std` requirements for the
effective closure, not only attributes, badges, or crate-level prose.

**Confidence:** High.

### FERRIUM-598: feature policy materially changes embedded reach

**Sources:** selected bare-metal package-root matrix and focused consumer
controls.

**Observed behavior:** Only five default package roots passed both bare-metal
targets. Eleven passed the no-default Thumb check. Exact no-default consumers
for Bytes, Serde, Tracing Core, and Hashbrown passed
`thumbv7em-none-eabihf`.

**Implication:** Embedded evidence must identify requested and effective
features and whether the resulting API still satisfies the consumer.

**Confidence:** High.

### FERRIUM-599: architecture support may require a capability provider

**Sources:** three `bytes 1.12.1` `thumbv6m-none-eabi` controls.

**Observed behavior:** The no-default consumer failed because pointer-width
atomic CAS operations were unavailable. Enabling `extra-platforms` changed the
failure to an explicit provider requirement. Adding the
`portable-atomic/critical-section` provider compiled.

**Implication:** Record architecture capabilities, fallback feature, provider,
configuration, and safety assumptions. `extra-platforms` alone is not a
complete support claim.

**Confidence:** High.

### FERRIUM-600: WASM support includes an environment and provider contract

**Sources:** `getrandom 0.4.3` documentation and focused controls.

**Observed behavior:** WASIp2 compiled with default and no-default dependency
settings. Unknown WASM deliberately failed by default because the target does
not identify whether JavaScript exists; enabling `wasm_js` compiled.

**Implication:** Distinguish browser, worker, Node, WASI, Emscripten, and
non-Web environments. Provider selection belongs at the application/profile
boundary, not as an unconditional library feature.

**Confidence:** High.

### FERRIUM-601: explicit unsupported states are useful contract behavior

**Sources:** `getrandom` and `http` compile errors.

**Observed behavior:** Both crates failed unsupported configurations with
specific instructions rather than compiling a misleading fallback.

**Implication:** Preserve expected-unsupported as a first-class result distinct
from defect, infrastructure failure, and unknown. Profiles may reject or
configure it, but OSPREY must not hide it.

**Confidence:** High.

### FERRIUM-602: `cargo check --target` does not prove cross-compilation

**Sources:** package-root checks and representative binary builds.

**Observed behavior:** All nineteen default roots checked for the selected
Linux and macOS triples. A simple binary then failed to link for both Linux
targets because `cc` was absent and for macOS because both `cc` and the Apple
SDK were absent.

**Implication:** Cross-target evidence must separate Rust compilation, native
linking, artifact production, execution, and tests.

**Confidence:** High.

### FERRIUM-603: host and target tooling are part of platform identity

**Sources:** link failures and Rust target documentation.

**Observed behavior:** Installed target standard libraries enabled code
generation, but did not install target linkers, SDKs, sysroots, runners, or
deployment environments.

**Implication:** Record the host/target pair and exact external tool inventory.
The target triple alone is insufficient.

**Confidence:** High.

### FERRIUM-604: build-helper compilation does not prove downstream native work

**Sources:** `cc 1.4.2` and `pkg-config 0.3.33` documentation.

**Observed behavior:** Both Rust libraries checked on many targets. Their
actual purpose is to run on the host from downstream build scripts, where
`cc` needs a compiler and archiver and `pkg-config` needs an executable,
sysroot, paths, and explicit cross configuration.

**Implication:** Model build helpers as host capabilities and defer actual
native-provider verification to ECOS-Q09.

**Confidence:** High.

### FERRIUM-605: compatibility results need typed negative and unknown states

**Sources:** all ECOS-Q07 controls.

**Observed behavior:** Failures included expected unsupported targets, required
`std`, absent atomic capability/provider, package-self lint policy, historical
Cargo manifest parsing, and missing host link tools. Execution and most tests
were not observed.

**Implication:** Use typed states such as declared, observed-pass,
expected-unsupported, compile-failed, link-infrastructure-missing,
not-observed, stale, and unknown. Do not collapse them into pass/fail.

**Confidence:** High.

### FERRIUM-606: platform evidence must expire with its closure and environment

**Sources:** exact release matrix, Cargo resolver behavior, target tier policy,
and provider/tool controls.

**Observed behavior:** Results depend on exact transitive versions, features,
Cargo/rustc behavior, target components, providers, and external tools. Any of
these can change independently of the top-level package version.

**Implication:** Renew on package, lockfile, feature, target, compiler, Cargo,
provider, SDK, sysroot, native-tool, tier-policy, or validation change.

**Confidence:** High.

## Recommendations

### Adopt now

- Add the platform evidence model to the OSPREY Ecosystem adapter and Crate
  Ecosystem Ledger.
- Record declared and observed MSRV per exact feature closure and Cargo/rustc
  pair.
- Separate package-root, consumer, check, link, run, test, and deployment
  states.
- Preserve explicit unsupported, unavailable infrastructure, not-observed, and
  unknown results.
- Record `core`/`alloc`/`std`, atomic, provider, target-tier, host-tool,
  linker, SDK, sysroot, and runner requirements.

### Prototype behind a compatibility boundary

- renewable consumer probes for selected target-feature profiles;
- historical compiler checks with explicit resolver and lockfile provenance;
- target-feature closure diffs that expose provider, build, macro, and native
  changes;
- host/target prerequisite inventory and actionable missing-tool diagnostics;
  and
- FERRIS evidence packets for reproducible platform observations.

### Reject or defer

- one portable, embedded-ready, WASM-ready, or MSRV score;
- inferred support from `#![no_std]`, target tier, or installed rust-std alone;
- universal `getrandom/wasm_js` enablement by libraries;
- hidden provider or unsafe architecture assumptions;
- automatic installation of linkers, SDKs, sysroots, or native packages;
- native-boundary conclusions before ECOS-Q09; and
- OSPREY implementation before the Crates Series gate.

## Role review

### Rust Safety Steward

Accepts explicit architecture and provider assumptions, especially
`portable-atomic` critical-section and `getrandom` backend selection. Requires
unsafe provider claims to remain consumer-owned and target-specific.

### Compiler Performance Engineer

Accepts bounded consumer probes and stage separation. Requires future renewal
to reuse immutable archives, lockfiles, toolchains, and target artifacts while
measuring matrix cost rather than attaching every probe to every build.

### Interop Boundary Auditor

Accepts host/target, ABI, linker, SDK, sysroot, native tool, runtime, and
provider identity as separate boundaries. Requires ECOS-Q09 to execute actual
native build and link paths.

### AI Assurance Skeptic

Accepts explicit unsupported, failed, not-observed, and unknown states. Rejects
AI inference from crate attributes, target tiers, compile checks, or missing
errors to broader portability claims.

### Ecosystem Strategist

Accepts evidence compatible with Cargo metadata, rustup targets, crate
documentation, and existing provider mechanisms. Requires upstream
contribution rather than a FERRIUM-specific target taxonomy.

### Rust Maintainer

Accepts minimal consumer fixtures that avoid irrelevant dev dependencies.
Requires diagnostics to identify the exact feature, transitive package,
toolchain, target, and missing prerequisite and to remain removable.

### Native Platform Adopter

Accepts check/link/run separation and explicit external-tool inventory.
Requires application profiles to prove required SDK, linker, sysroot, runtime,
deployment, offline, support, and rollback behavior.

### Scope Keeper

Accepts Q07 as an evidence model. Full target coverage, native-provider
certification, automatic environment repair, stack selection, and
implementation remain closed.

### Validation Checker

Accepts 190 package-root cases, nineteen historical consumer checks, focused
feature/provider controls, six link targets, one execution control, exact
commands, sources, and limitations. Requires future renewals to retain raw
results and distinguish fixture failures from product failures.

## Limitations

- Measurements used one Windows host.
- Package-root checks used current Cargo and rustc 1.95.0.
- Historical checks covered one default consumer per selected release, not
  every feature or a compiler immediately below each declaration.
- Only selected WASM and embedded targets were measured.
- Linux and macOS targets were checked but not linked because required tools
  were absent.
- Only the Windows binary was executed.
- WASM artifacts were not executed or browser-tested.
- Bare-metal artifacts were checked as libraries and not linked into firmware.
- Package examples, doctests, test suites, deployment, ABI behavior, and
  operating-system minimums were not exhaustively tested.
- `cc` and `pkg-config` were compiled as dependencies; downstream native
  invocation remains for ECOS-Q09.
- The selected queue is not an approved stack or universal ecosystem sample.

## Primary sources

- Rust platform support:
  <https://doc.rust-lang.org/rustc/platform-support.html>
- Cargo Rust version:
  <https://doc.rust-lang.org/cargo/reference/rust-version.html>
- Cargo dependency resolver:
  <https://doc.rust-lang.org/cargo/reference/resolver.html#rust-version>
- `getrandom 0.4.3` platform and WASM support:
  <https://docs.rs/getrandom/0.4.3/getrandom/#webassembly-support>
- `cc 1.4.2` external tool and cross-compilation configuration:
  <https://docs.rs/cc/1.4.2/cc/>
- `pkg-config 0.3.33` cross-compilation configuration:
  <https://docs.rs/pkg-config/0.3.33/pkg_config/#cross-compilation>
