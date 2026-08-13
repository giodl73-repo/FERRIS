# Platform Validation Roadmap

Status: Guidance
Implementation authority: None

## Purpose

This roadmap defines the evidence sequence for Ferris platform profiles. It is
a conformance plan, not authorization to implement a runner, mutate CI, install
toolchains or SDKs, operate signing systems, deploy software, or claim support.

## Validation principles

1. Qualify exact consumer profiles, not platform brands.
2. Separate host, target, provider, native, runner, packaging, signing, and
   deployment identities.
3. Record every stage independently.
4. Include positive, expected-rejection, unsupported, unavailable,
   not-observed, stale, unknown, version-skew, rollback, and removal cases.
5. Preserve full owner-native fallback when mapping or evidence is uncertain.
6. Measure matrix cost and variance without weakening mandatory correctness,
   safety, or release gates.
7. Require a separate support decision after validation.

## Matrix dimensions

Every qualification matrix selects explicit values from:

- consumer/profile revision;
- source/manifest/lock revision;
- Cargo/rustc/toolchain and declared/observed compiler floor;
- host OS, version, architecture, image, and filesystem;
- target triple, target specification, Rust tier, target component/sysroot;
- `core`, `alloc`, `std`, panic, allocator, atomics, SIMD, thread, entropy,
  clock, I/O, and other capabilities;
- requested/effective features and active normal/build/dev closure;
- provider and source mode;
- native compiler, linker, archiver, SDK, sysroot, headers/libraries, package
  discovery, generator, and generated output;
- runner/runtime/device/browser/RTOS;
- debug, optimization, and link profile;
- package, signing/attestation, deployment, and servicing channel; and
- stage, expected result, evidence date, and owner.

Avoid a Cartesian product without purpose. Define mandatory representative
rows and pairwise/risk-based expansions, then retain a full-reference release
or renewal matrix where the support contract requires it.

## Baseline platform rows

These rows are starting categories, not support promises:

| Family | Minimum distinct rows |
|---|---|
| Linux | x86-64 GNU native; AArch64 GNU native or target host; musl if claimed; supported distro/image floors |
| Windows | x86-64 MSVC native; ARM64 if claimed; GNU target only if separately supported; minimum OS/SDK/toolset rows |
| macOS | Apple silicon native; Intel if claimed; universal/package row if shipped; minimum deployment target |
| Android | Each supported ABI/API floor; emulator and at least one representative device class |
| iOS | Simulator and physical device; each supported architecture/deployment floor; signing/package row |
| Browser/WASM | Each supported browser class; worker/main-thread where relevant; bundler/glue/provider row |
| WASI/component | Each supported runtime and interface/version; package/component validation |
| Embedded/bare metal/RTOS | Each supported MCU/board/hardware revision and RTOS/runtime; probe/device and recovery row |

Cross-check rows may run from additional hosts, but they do not replace native
or target execution rows.

## Stage qualification matrix

### Stage A: schema and identity fixtures

Create fixtures for:

- exact host/target identity;
- target tier and component availability;
- `core`, `alloc`, and `std` distinctions;
- architecture capability present/absent;
- provider default, explicit, custom, external, and unsupported states;
- system, bundled, prebuilt, generated, and external native modes;
- every top-level result state; and
- expiry, supersession, rollback, and removal.

Acceptance: deterministic records, correct joins, no secrets, and round-trip
schema tests. This stage alone establishes no platform behavior.

### Stage B: resolution and check

For each selected profile:

- capture lock universe and active target/dependency-kind closures;
- check default and approved explicit feature sets;
- test compiler floor with the exact Cargo/rustc pair;
- separate package-root and minimal-consumer checks;
- include expected unsupported `std`, provider, architecture, and target cases.

Acceptance: typed diagnostics and no promotion beyond check.

### Stage C: build and link

Add:

- target code generation and artifact identity;
- actual linker, SDK/sysroot, CRT/libc, frameworks/import libraries, linker
  script/startup objects, and native providers;
- positive links and missing-tool/package/ABI negative fixtures;
- static/dynamic and debug/release artifact inspection; and
- generated/native/final artifact reproducibility observations.

Acceptance: link claims require final target images and exact external-tool
evidence.

### Stage D: execute and test

Run on the supported execution substrate:

- native host, VM/container, emulator, simulator, browser, WASI runtime,
  device, RTOS, or hardware;
- unit, integration, doctest, contract, FFI, provider, failure, and recovery
  cases as applicable;
- architecture capability and resource-limit tests;
- representative cross-language calls for each supported ABI; and
- runner-unavailable and runtime-failure controls.

Acceptance: test scope and non-executed cases remain explicit.

### Stage E: package, sign, deploy, and debug

Qualify:

- package contents, licenses/notices, SBOM/provenance, native runtime
  dependencies, symbols, and installation;
- signing or attestation policy without retaining reusable secrets;
- macOS notarization, mobile provisioning, Windows signing, firmware image
  metadata, or other platform release controls when required;
- deployment, health, restart/update, crash collection, symbol retrieval,
  source-level debugging, and support escalation; and
- unavailable signing/deployment infrastructure as explicit cases.

Acceptance: a package/build pass cannot substitute for deploy and operational
evidence.

### Stage F: renewal, rollback, substitution, and removal

Execute fixtures for:

- package/feature/toolchain/target/provider/native/SDK changes;
- evidence expiry and revocation;
- rollback across Cargo, toolchain, native, data, package, deployment, and
  device layers;
- provider or source-mode substitution; and
- complete removal with ordinary Cargo and owner-native workflows remaining
  functional.

Acceptance: at least one exact profile must demonstrate each lifecycle path
before a general lifecycle claim advances.

## CI qualification tiers

| Tier | Purpose | Typical cadence |
|---|---|---|
| Change gate | Fast host-native checks and required negative controls | Every relevant change |
| Integration gate | Representative links, runs, tests, and native/provider rows | Merge or scheduled |
| Release gate | Full supported package/sign/deploy/debug matrix | Release candidate |
| Renewal gate | Full profile diff, version skew, support and lifecycle evidence | Expiry or material change |
| Incident gate | Focused vulnerable/failing boundary plus rollback/substitution | Event driven |

Each tier records what it omits. Scheduled green runs may refresh evidence but
must not approve dependency changes, support expansion, or environment
mutation.

## Negative and state fixtures

Every platform family should include:

- explicitly unsupported target/feature/provider;
- missing target component;
- missing linker or SDK/sysroot;
- missing system package or native library;
- architecture capability absence;
- generator or discovery failure;
- runner/device/runtime unavailable;
- package/sign/deploy rejection;
- stale evidence;
- unknown mapping that widens to full owner scope;
- rollback failure; and
- removal residue detection.

Expected failures must assert typed diagnostics, stage, identity, remediation
owner, and absence of false success.

## Performance and cost controls

Record cold and warm duration, CPU, memory, storage, network, queue time,
device scarcity, flake/retry rate, and variance for each matrix class. Use the
data to schedule and shard qualification, not to erase required coverage.
Performance recommendations must identify causality and preserve
reproducibility and correctness.

## Advancement gates

Guidance may advance toward a normative or implementation proposal only when:

1. platform identities and state vocabulary are complete;
2. at least Linux, Windows, macOS, mobile, WASM/WASI/browser, and
   embedded/bare-metal/RTOS families have bounded fixtures or explicit deferral;
3. check, link, run, test, package, sign, deploy, debug, service, rollback, and
   removal are not collapsed;
4. target-tier and consumer-support claims are separate;
5. `core`/`alloc`/`std`, architecture capabilities, providers, native tools,
   SDKs, sysroots, linkers, and runners are represented;
6. all typed negative and missing states have fixtures;
7. ordinary Cargo operation and removal are demonstrated;
8. all nine roles record dispositions and remaining blockers; and
9. a separately approved pulse names implementation scope and stop criteria.

Until then, implementation authority remains none.

## Roadmap order

```text
R1 identity/state fixtures
  -> R2 resolve/check matrices
  -> R3 build/link and native boundaries
  -> R4 target execution/test
  -> R5 package/sign/deploy/debug/service
  -> R6 renewal/rollback/substitution/removal
  -> R7 held-out cross-platform conformance
  -> separately approved bounded implementation
```

Held-out evidence must be designed independently, protect any oracle boundary,
and preserve invalid or failed runs rather than rescoring them into success.

## Sources

- [Ferris context](../../../CONTEXT.md)
- [Ferris agent instructions](../../../AGENTS.md)
- [Ferris product plan](../../../PRODUCT_PLAN.md)
- [PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md)
- [Seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)
- [Platform compatibility research](../../research/2026-08-09-rust-platform-compatibility.md)
- [Native dependency research](../../research/2026-08-10-rust-native-dependency-boundary.md)
- [Mirrored compiler CI and distribution guide](../../reference/rust-reference/rust-architecture/20-BOOTSTRAP-CI-TESTING-PERF-AND-DISTRIBUTION.md)
- [Mirrored target compatibility guide](../../reference/rust-reference/rust-crate-ecosystem/11-TARGET-PLATFORM-COMPATIBILITY-AND-NO-STD.md)

