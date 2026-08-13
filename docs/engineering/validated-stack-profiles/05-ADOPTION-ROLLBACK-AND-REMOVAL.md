# Adoption, Rollback, and Removal

Status: Guidance
Implementation authority: None

## Lifecycle rule

A profile is acceptable only when adoption is incremental, support is
explicit, renewal is bounded, substitution is possible, rollback is exact,
removal is complete, and ordinary Cargo remains usable.

Expiry, rollback, and removal are not cleanup details. They are anti-lock-in
requirements that prevent an exact profile from becoming a hidden Ferris
distribution. See
[PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md) and the
[compatibility profile research](../../research/2026-08-10-rust-compatibility-stack-profiles.md).

## Adoption preconditions

Before consumer approval, the packet should contain:

- exact profile, requirements, evidence, and support identities;
- independent family and capability boundary;
- mandatory eligibility results and unresolved preferences;
- direct releases, features, manifest, lock, lock universe, and active
  closures;
- contracts, providers, adapters, native and generated boundaries;
- exact toolchain, host, targets, tools, runtime, and deployment environment;
- complete applicable stage matrix with negative and failure cases;
- assurance, stewardship, licensing, advisory, and residual-unknown records;
- support owner, combinations, dates, response, escalation, and exclusions;
- 90-day-or-shorter expiry and earlier renewal triggers;
- adoption changes and validation commands;
- substitution alternatives;
- rollback owner, trigger, exact procedure, validation, and deadline;
- removal owner, procedure, validation, and retained evidence; and
- all required review dispositions.

Adoption approval applies only to the exact revision. It does not float to a
new package, lock, feature, compiler, target, provider, native source,
deployment, or support statement.

## Incremental adoption

Adoption should minimize repository and workflow coupling:

1. preserve existing manifests, lockfile ownership, Cargo commands, editor
   use, CI concepts, and owner-native diagnostics;
2. add profile metadata as a removable projection rather than a parallel
   dependency manifest;
3. avoid mandatory services, registries, wrapper crates, package mirrors, or
   hidden generated state;
4. introduce only the validation needed for the named consumer contract;
5. make new support, training, operations, native, and recovery obligations
   visible;
6. retain the pre-adoption baseline and its exact validation evidence; and
7. define the point after which rollback requires data, ABI, or deployment
   recovery rather than a source-only revert.

For native and desktop families, include installation, signing, packaging,
debugging, runtime-library, and operations changes. For browser WASM, include
JavaScript glue and deployment assets. For embedded, include runner, board,
flash, and recovery. For data/ML/GPU, include model, data, driver, and device
state.

## Support activation

Support begins only when the named owner accepts the exact profile revision
and combinations. Activation should verify:

- contact and escalation paths;
- incident and security-response process;
- diagnostic collection boundaries;
- supported compiler, target, platform, provider, deployment, and native
  combinations;
- update, emergency, and exception policies;
- support end date and successor policy;
- operator and maintainer training; and
- rollback and recovery readiness.

Local validation cannot create support on behalf of an upstream maintainer.
Enterprise support must be clearly attributed to its actual owner.

## Renewal and servicing

The default maximum evidence age should be 90 days. A shorter risk or consumer
policy controls. Renew immediately when:

- direct or active dependencies, sources, or features change;
- contracts, adapters, providers, native modes, or generated code change;
- advisories, incidents, ownership, custody, licensing, or succession change;
- Cargo, rustc, target tier, SDK, compiler, linker, driver, runtime, or
  deployment changes;
- required evidence fails, expires, is revoked, or becomes unavailable;
- support terms or consumer requirements change; or
- an exception approaches expiry.

Renewal is a reviewed diff, not an automatic update. The measured Clap control
changed only two package versions and preserved package count, yet still
required exact validation and lock restoration. Graph size alone cannot
establish unchanged behavior or risk.

## Substitution procedure

Provider or implementation substitution must:

1. identify the consumer contract to preserve or revise;
2. freeze old and candidate profile identities;
3. diff releases, closures, requested and effective features;
4. diff runtime, provider, native, data, wire, ABI, deployment, support,
   assurance, cost, and operational boundaries;
5. execute positive and expected-rejection semantic fixtures;
6. execute migration, coexistence, and negative compatibility fixtures;
7. validate packaging, deployment, operations, emergency response, and
   rollback;
8. identify changed training, diagnostics, and support obligations;
9. record the consumer decision and remaining unknowns; and
10. preserve reversibility for the declared rollback period.

A package-name replacement is not sufficient evidence. Adapters may lose
semantics while compiling. See the
[enterprise platform plan](../../plans/ENTERPRISE_RUST_APPLICATION_PLATFORM.md)
and [Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md).

## Rollback classes

### Source and lock rollback

Applicable only when no external state changed. Restore the exact approved
manifest and lock identities, selected source state, toolchain, targets, and
features. Rebuild and rerun all mandatory validation with locked inputs.

### Artifact rollback

Restore the exact prior artifact, package, signature, configuration, and
runtime prerequisites. Validate artifact identity and representative
operation. Do not assume rebuilding current source reproduces the prior
artifact.

### Data and wire rollback

Restore or migrate persisted data, schema, messages, and protocol state under
an explicit compatibility contract. Validate forward and backward behavior,
partial migration, failure recovery, and data retention. A lockfile restore
cannot undo a schema or wire transition.

### Native and ABI rollback

Restore native libraries, runtime dependencies, generated bindings, headers,
SDK state, installers, and deployment artifacts. Validate ABI, allocation,
threading, error, panic, and cleanup behavior. Account for system package or
driver changes.

### Deployment and operational rollback

Restore the previous deployment, traffic, configuration, secrets references,
service state, monitoring, and support posture using the consumer's owner
process. Validate health, telemetry, incident response, and data consistency.

### Embedded, GUI, and accelerator rollback

Restore board firmware, desktop package, signing state, driver, model, kernel,
or device configuration as applicable. Include safe recovery when the new
state cannot start.

## Rollback record

Every rollback plan should name:

- trigger and decision authority;
- latest safe rollback time;
- exact prior profile, manifest, lock, artifact, data, native, and deployment
  identities;
- commands and owner-native procedures;
- prerequisites and retained assets;
- responsible owner and contacts;
- service, data, security, and compliance consequences;
- validation matrix;
- expected duration and operational impact;
- failure escalation and safe-stop behavior; and
- evidence retention.

Rollback must be tested, not merely described. If rollback is unavailable,
state that limitation before adoption.

## Emergency response

Emergency response may choose containment, update, substitution, rollback,
shutdown, or a time-bounded exception. It must:

- preserve the alert and affected profile identities;
- revoke or stale affected evidence without rewriting history;
- distinguish confirmed from potential impact;
- identify current source, support, consumer, and deployment owners;
- expose unavailable evidence and unknowns;
- require approval for mutations and deployment;
- validate the selected response and rollback; and
- issue a replacement, supersession, exception, or retirement record.

No emergency authorizes Ferris to silently update dependencies, install tools,
switch providers, or deploy.

## Complete removal

Profile cleanup is not by itself the canonical Ferris removal record. When a
consumer partially or completely removes Ferris integration, PRODUCT-001
requires one versioned
[Removal Record](../../specs/FERRIS_PRODUCT_CONTRACT.md#removal-record).
It must inventory the affected Ferris components and integrations, bind
authority and lifecycle phases, record evidence export and cleanup, verify
ordinary Cargo and owner-native behavior, and retain residual effects and
recovery ownership. The profile steps below provide inputs to that record.

Removal should:

1. stop profile-specific schedules, checks, dashboards, alerts, and support
   routing;
2. remove profile metadata and generated projections;
3. remove profile-specific CI, hooks, configuration, credentials references,
   caches, storage, and access grants;
4. replace or remove direct dependencies through consumer-owned changes;
5. clean requested features, adapters, generated code, native prerequisites,
   packages, SDKs, runtime libraries, artifacts, and deployment resources;
6. validate public Rust types, semantic contracts, ABI, wire and persisted
   data, behavior, packaging, deployment, and operations;
7. prove ordinary Cargo and editor workflows remain functional;
8. remove mandatory Ferris services or tooling from the consumer path;
9. retain the approved historical evidence, decisions, incidents, and
   rollback records under retention policy; and
10. record completion, residual state, and owner acceptance.

Removal is incomplete if the repository still requires Ferris to resolve,
build, test, package, deploy, diagnose, or understand the application.

## Removal validation

At minimum:

- run ordinary Cargo commands directly;
- compare required behavior with the adopted-profile baseline;
- verify no profile-only manifest or lock authority remains;
- verify CI and local developer workflows;
- inspect native, generated, package, deployment, credential-reference, and
  storage cleanup;
- exercise negative and unsupported cases;
- verify retained evidence remains readable without operational dependency;
  and
- obtain consumer and support-owner acceptance.

The [Rust Maintainer](../../../.roles/stakeholders/rust-maintainer.md) requires
complete removal without trapping the repository. The
[Native Platform Adopter](../../../.roles/stakeholders/native-platform-adopter.md)
requires recovery and audit to include existing native and operational
constraints.
