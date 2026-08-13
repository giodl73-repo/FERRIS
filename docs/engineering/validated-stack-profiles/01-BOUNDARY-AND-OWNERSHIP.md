# Boundary and Ownership

Status: Guidance
Implementation authority: None

## Governing boundary

A profile is an exact, renewable, consumer-scoped evidence and support record.
It describes one accepted capability boundary under one set of requirements.
It is not:

- a Ferris distribution, registry, lockfile mandate, package manager, or
  installation mechanism;
- a certification, compliance verdict, safety claim, maintenance score, or
  universal recommendation;
- a transfer of Cargo, rustc, crate, platform, provider, or deployment
  authority to Ferris;
- proof that an unobserved target, stage, feature, provider, or environment
  works; or
- permission to edit manifests, update locks, switch providers, install tools,
  approve exceptions, publish upstream, or deploy.

This boundary is normative in
[PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md) and is
supported by the measured
[profile research](../../research/2026-08-10-rust-compatibility-stack-profiles.md)
and [intervention decision](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md).

## Ownership map

| Concern | Primary owner | Ferris/profile responsibility |
|---|---|---|
| Requirements and non-goals | Consumer | Preserve exact scope and decision authority |
| Package sources, resolution, lock, targets, features | Cargo and consumer manifests | Observe exact owner truth; never reimplement resolution |
| Compiler behavior and target support | Rust Project and toolchain owners | Record exact versions, components, commands, and outcomes |
| Crate API, releases, security response | Upstream maintainers | Cite releases, contracts, support statements, and current owners |
| Registry release and checksum data | Registry owner | Record dated source identity without claiming source reproduction |
| Advisories and policy results | Advisory and policy tool owners | Record tool, database, time, scope, result, and limitations |
| Native tools, SDKs, ABI, system packages | Platform and native owners | Record discovery, versions, source mode, artifacts, and failures |
| Provider behavior | Provider owner and consumer | Record exact provider contract and substitution boundary |
| Deployment and operations | Consumer and deployment owner | Record environment, support, incident, rollback, and removal procedures |
| Profile schema and evidence method | Profile author | Maintain schema, evidence identity, expiry, and diff rules |
| Adoption and exceptions | Consumer approval authority | Record decision; never infer it from observations |
| Support commitment | Named support owner | State combinations, dates, contact, response, exclusions, and escalation |

Ferris may enforce declared policy only when a later specification and pulse
authorize that action. Successful local observation does not create a support
commitment. A profile author cannot promise behavior controlled by another
owner without an explicit support agreement.

## Consumer scope

Consumer scope must be narrow enough to answer:

- Which repository, application, component, environment, and operation is
  covered?
- Which target, feature, provider, native, deployment, and lifecycle
  combinations are eligible?
- Which combinations are explicitly unsupported or outside the profile?
- Which authority may adopt, renew, except, substitute, revoke, or retire it?

Profile names such as "service stack" or "desktop stack" are labels, not
identity. Two consumers using identical packages may require different
profiles because their operations, targets, support windows, providers,
deployment systems, risk policies, and rollback obligations differ.

## Family-specific boundaries

### Hosted service

Scope the request operation, runtime, transport, TLS and identity providers,
shutdown, cancellation, observability, deployment, and operational checks.
Do not promote an in-process request test to bound-network, production,
capacity, or failover support.

### CLI and configuration

Scope argument sources, config formats, filesystem and terminal assumptions,
diagnostic contracts, packaging, and supported hosts. Compilation for WASM
does not establish a usable browser CLI.

### Pure data

Scope schemas, encodings, time and locale sources, numeric and error behavior,
resource limits, persistence, and output compatibility. Broad compilation
does not establish runtime semantics on each target.

### Embedded and `no_std`

Scope `core`, `alloc`, or no-allocation assumptions; architecture; panic
behavior; board or emulator; runner; memory; timing; interrupts; and
transport. Host tests and cross-builds do not prove device execution.

### Browser WASM

Scope the WASM target, JavaScript glue, browser versions, APIs, bundler,
origin, storage, networking, and runtime execution. A `.wasm` artifact alone
is build evidence only.

### Native dependency

Scope bundled versus system source, native version, ABI, compiler, generator,
SDK, discovery, linker, runtime libraries, packaging, signing, and cleanup.
Bundling changes custody and build behavior; it does not remove native risk.

### Desktop and GUI

Scope event loop, graphics, input, accessibility, window system, threading,
native integration, installer, signing, update, diagnostics, and recovery.
Desktop support must be expressed per platform and packaging route.

### Networking and protocol

Scope protocol and wire identity, transport, runtime, TLS, timeouts,
backpressure, cancellation, retry, version negotiation, negative cases, and
remote compatibility. A client and server may require separate profiles.

### Data, ML, and GPU

Scope data and model identities, preprocessing, kernels, precision, device,
driver, runtime, memory, determinism, CPU fallback, packaging, and serving.
Hardware availability and model quality are distinct from build success.

Each family remains independently versioned and renewable. Shared crates do
not justify collapsing contracts.

## Contract boundary and substitution

Substitution is possible only when the consumer boundary is explicit. A
profile should identify the Rust API, Typebook/RUNE contract, C ABI, WIT
interface, wire schema, data format, or other consumer-facing contract that
must remain stable or be deliberately revised.

For every adapter or provider, record direction, ownership, allocation,
copying, fallibility, field loss, ordering, backpressure, cancellation, panic,
threading, feature, target, and runtime consequences. This follows the
[enterprise platform plan](../../plans/ENTERPRISE_RUST_APPLICATION_PLATFORM.md)
and the
[Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md).

Substitution must not be represented as a package-name swap. It changes an
identified set of dependencies, features, runtime assumptions, native tools,
data or wire behavior, operations, support, cost, and rollback obligations.

## Support boundary

A support statement must name:

- support owner and contact;
- supported profile revisions;
- supported compiler, target, platform, provider, native, and deployment
  combinations;
- start and end dates;
- security and emergency response;
- diagnostics and escalation;
- servicing and exception policy;
- operational and training prerequisites; and
- unsupported combinations.

Upstream maintenance and consumer support are different. A support owner may
support an exact profile without speaking for the Rust Project or crate
maintainers. Conversely, upstream activity does not create enterprise support.

## Ordinary Cargo is the escape path

Ordinary Cargo workflows must remain functional before, during, and after
profile use. Profile metadata must be removable without changing Cargo's
source, resolution, feature, target, or build authority. Consumer repositories
must retain understandable manifests, lockfiles, commands, and diagnostics.

This is both an ownership rule and an anti-lock-in control. The
[Rust Maintainer](../../../.roles/stakeholders/rust-maintainer.md) and
[Scope Keeper](../../../.roles/editorial/scope-keeper.md) require adoption to
remain incremental and removable.

## Decision authority

Evidence collection, recommendation, approval, execution, and support are
separate records. Humans or existing policy owners approve adoption,
exceptions, renewal, substitution, and retirement. AI may summarize diffs or
propose validation but cannot establish owner truth, erase unknowns, approve
policy, or mutate dependencies. See the
[AI Assurance Skeptic](../../../.roles/parliament/ai-assurance-skeptic.md).

