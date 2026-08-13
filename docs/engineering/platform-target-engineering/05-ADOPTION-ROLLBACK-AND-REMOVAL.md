# Platform Adoption, Rollback, and Removal

Status: Guidance
Implementation authority: None

## Purpose

A platform capability is acceptable only when it can be introduced,
supported, reversed, replaced, and removed without making Ferris a permanent
runtime dependency or trapping the repository. This guide defines the
lifecycle record. It does not authorize dependency, CI, toolchain,
environment, signing, device, or deployment changes.

## Lifecycle

```text
evaluate -> approve exact profile -> integrate -> qualify -> observe
    -> renew or constrain
    -> rollback, substitute, or remove
```

Adoption is not "the build passed." It is a consumer-owned decision to support
an exact profile under named operational and servicing obligations.

## Adoption entry criteria

Before adoption, record:

- consumer and business/engineering purpose;
- profile ID/revision, owner, approval, support start/end, expiry;
- exact source, manifest, lock, feature, closure, Cargo/rustc, host/target,
  target tier, `core`/`alloc`/`std`, architecture, provider, native and
  deployment identities;
- stage results for resolve, check, build, link, execute, tests, package,
  signing/attestation, deploy, debug, operational validation, and rollback;
- every `expected-rejection`, `unsupported`, `unavailable`, `not-observed`,
  `stale`, and `unknown` item;
- security, licensing, provenance, native servicing, and support owners;
- CI qualification matrix and cost;
- training, diagnostics, incident, and escalation path;
- substitution alternatives; and
- exact rollback and removal plans.

An exception must name its owner, rationale, affected scope, expiry, renewal
condition, and safe fallback.

## Integration boundary

Prefer a narrow repository-owned adapter or profile boundary. Preserve:

- ordinary Cargo commands and manifests as owner truth;
- owner-native target, linker, runner, package, signing, and deployment
  workflows;
- explicit provider configuration;
- reviewable CI jobs rather than hidden automation;
- no reusable credentials in repository evidence; and
- an off switch that does not require Ferris to interpret or repair the
  repository.

Ferris metadata may describe support and evidence. It must not become a second
resolver, a hidden manifest, or the only way to build or remove the capability.

## Rollout strategy

Adopt in bounded rings:

1. development fixture or non-production consumer;
2. representative CI qualification;
3. packaging and deployment test environment;
4. limited observation ring where applicable;
5. supported production or field scope only after all required evidence.

At each ring, define stop criteria, rollback owner, retained artifact, data or
protocol compatibility, monitoring period, and next approval. A green earlier
ring does not authorize the next.

## CI adoption

The supported profile must map to explicit CI jobs:

- host-native jobs for each supported host;
- cross-check jobs where they add evidence;
- real link jobs with SDK/sysroot/native prerequisites;
- target execution/test jobs on native hosts, runtimes, emulators, simulators,
  browsers, devices, or hardware;
- package, sign/attest, install/deploy, debug-symbol, rollback, and removal
  jobs as required;
- negative jobs for unsupported and missing-prerequisite behavior; and
- scheduled renewal jobs that cannot approve adoption by themselves.

Do not add every target to every change without a cost model. Use risk-based
tiers and mandatory release/renewal gates, but never delete required coverage
solely for speed.

## Rollback design

Rollback is a prepared transition to a previously acceptable state. Inventory:

| Layer | Rollback identity |
|---|---|
| Source and policy | Commit/revision, approval, profile revision |
| Cargo | Manifest requirements/features/patches and `Cargo.lock` |
| Toolchain | Cargo/rustc, target components, linker, SDK, sysroot, native tools |
| Provider/native | Provider configuration, system/bundled mode, native version and artifacts |
| Contract/data | API, ABI, wire, schema, persisted data, migration compatibility |
| Package/sign | Prior package, signature/attestation, symbols and metadata |
| Deployment | Prior releasable artifact, configuration, ring, health and routing state |
| Device/firmware | Prior image, bootloader/slot policy, flashing/recovery method |

Reverting `Cargo.lock` is insufficient when features, provider, source,
generated code, ABI, system package, data, signing, package, deployment, or
device state changed.

Rollback validation must prove the prior state still builds, links, executes,
passes required tests, packages, deploys, and operates under current policy.
Do not roll back to a known vulnerable or unsupported state merely because it
is familiar.

## Substitution

A provider, runtime, native mode, linker, SDK, runner, or package substitution
must:

- retain or explicitly revise the consumer contract;
- identify changed feature and dependency closure;
- compare architecture, ABI, unsafe, crypto, entropy, allocator, threading,
  native, license, advisory, build, artifact, and support consequences;
- run positive, negative, migration, deployment, and rollback fixtures;
- keep prior evidence and the owner decision; and
- remain reversible for the declared rollback period.

Examples include system versus bundled SQLite, Schannel versus OpenSSL,
different Rustls crypto providers, browser JavaScript versus WASI providers,
or one RTOS/HAL/runner combination versus another. Equal API shape is not
equivalent operational behavior.

## Removal plan

Capability cleanup and Ferris removal are related but distinct. If the
consumer partially or completely removes Ferris integration, PRODUCT-001
requires one canonical, versioned
[Removal Record](../../specs/FERRIS_PRODUCT_CONTRACT.md#removal-record).
That record binds the removal authority, complete Ferris inventory, action
freeze and drain state, evidence export, cleanup, owner-native verification,
retained evidence, residual effects, recovery owner, and final disposition.
The platform-specific checklist below supplies evidence to that canonical
record; it does not replace it.

Removal is complete only after checking:

- direct, transitive, target, build, and development dependency edges;
- features, patches, source replacement, Cargo configuration, toolchain pins,
  target components, and local overrides;
- build scripts, procedural macros, generators, generated source/bindings, and
  vendored tools;
- native source, prebuilt objects, system packages, SDKs, sysroots, link flags,
  runtime libraries, and provider configuration;
- CI jobs, caches, images, runners, emulators, simulators, browser harnesses,
  devices, secrets references, signing policy, and deployment automation;
- API/ABI/wire/data contracts and migrations;
- packages, installers, containers, firmware, store artifacts, notices, SBOMs,
  debug symbols, crash symbol services, and support records;
- dashboards, alerts, incident procedures, training, and documentation; and
- retained immutable historical evidence.

Removal evidence must be collected across every supported profile. A reverse
dependency query for one host/feature selection is not sufficient.

## Platform-specific rollback and removal

| Family | Additional concerns |
|---|---|
| Linux | System package and image rollback, libc/loader compatibility, service unit, container base, debug packages |
| Windows | MSI/MSIX or enterprise installer rollback, CRT/runtime, registry/service state, Authenticode, PDB retention |
| macOS | App/package rollback, entitlements, notarization, architecture slices, dSYM and update channel |
| Android/iOS | App data/schema compatibility, store/enterprise rollout, signing/provisioning, device OS floor, crash symbols |
| WASM/browser | JS glue and asset rollback, cache/CDN invalidation, browser/runtime compatibility, component/interface version |
| Embedded/RTOS | Bootloader slot/recovery, flash procedure, persistent data, hardware revision, field update, physical recovery |

## Renewal and removal triggers

Renew or consider removal when:

- upstream support, ownership, license, advisory, or security response changes;
- compiler, Cargo, target tier, target specification, SDK, sysroot, provider,
  native tool, runner, packaging, signing, or deployment changes;
- required validation fails or becomes unavailable;
- evidence expires;
- build or operational cost exceeds the profile budget;
- the capability becomes duplicated, unused, or strategically unsupported; or
- rollback can no longer be demonstrated.

## Exit acceptance criteria

Removal is accepted when:

1. no required runtime or build path depends on the removed capability;
2. all supported profile matrices pass without it or are explicitly retired;
3. native, generated, package, signing, deployment, and servicing residue is
   removed or assigned;
4. ordinary Cargo and owner-native workflows remain functional;
5. replacement and rollback state are documented;
6. historical evidence remains immutable and discoverable; and
7. support owners agree that the capability is no longer promised.

## Sources

- [Ferris context](../../../CONTEXT.md)
- [PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md)
- [Seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)
- [Native dependency research](../../research/2026-08-10-rust-native-dependency-boundary.md)
- [Mirrored supported profile lifecycle guide](../../reference/rust-reference/rust-crate-ecosystem/15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md)
