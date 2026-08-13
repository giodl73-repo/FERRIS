# Validated Stack Profiles

Status: Guidance
Implementation authority: None

## Purpose

This guide set explains how Ferris should describe validated stack profiles
without creating a Ferris distribution, certification program, package
registry, universal recommendation, or installation authority.

A validated stack profile is an **exact, renewable, consumer-scoped evidence
record**. It joins one consumer's requirements to one identified selection,
closure, environment, validation matrix, assurance snapshot, support
commitment, and lifecycle plan. The record says what was observed, for whom,
under which conditions, by which owner, and until when.

A profile does not say that its crates are universally best, safe, secure,
maintained, portable, approved, or future-proof. A successful build does not
promote unobserved execution, deployment, operations, safety, or support
stages. Ferris coordinates and preserves evidence; Cargo, rustc, upstream
maintainers, native tools, deployment systems, and consumers retain their
authority.

These boundaries follow the repository's product rule:

> The plan is global; the work is local.

See [Ferris context](../../../CONTEXT.md), the
[seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md),
and [PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md).

## Guide map

1. [Boundary and ownership](01-BOUNDARY-AND-OWNERSHIP.md) defines what a
   profile is, what it is not, and which owner controls each decision.
2. [Operating workflow](02-OPERATING-WORKFLOW.md) defines candidate,
   observation, review, adoption, renewal, and retirement flow.
3. [Evidence and identity](03-EVIDENCE-AND-IDENTITY.md) defines exact
   selection, closure, environment, stage, and evidence identities.
4. [Failure modes and controls](04-FAILURE-MODES-AND-CONTROLS.md) prevents
   success promotion, hidden fallback, stale evidence, and owner confusion.
5. [Adoption, rollback, and removal](05-ADOPTION-ROLLBACK-AND-REMOVAL.md)
   preserves ordinary Cargo and makes every profile reversible.
6. [Validation roadmap](06-VALIDATION-ROADMAP.md) identifies the evidence
   needed before any read-only profile tooling can be implemented.

## PLATFORM-001 profile families

PLATFORM-001 requires the first profile schema to represent these nine
independently scoped families. They are contract shapes, not selected stacks.
Each needs its own consumer, requirements, exact revision, environment,
validation matrix, support owner, expiry, and lifecycle contract.

| Required family | Contract focus | Boundaries that must remain explicit |
|---|---|---|
| Hosted service | Request handling and service operation | Runtime, I/O, cancellation, TLS, provider, deployment, operations |
| CLI and configuration | Argument and configuration processing | Terminal, filesystem, encoding, config format, diagnostics, packaging |
| Pure data | Parse, transform, validate, and serialize | Data schema, time, locale, numeric behavior, persistence, resource limits |
| Embedded and `no_std` | Bounded operation without `std` | `core`/`alloc`, architecture, panic, allocator, board, runner, transport |
| Browser WASM | Browser-hosted module behavior | JS glue, browser API, bundler, runtime, origin, storage, network |
| WebAssembly component | Component-model operation | WIT identity, component tooling, runtime, capability and projection boundaries |
| Bundled or system-native dependency | Native integration | Source mode, ABI, compiler, SDK, discovery, linker, artifacts, deployment |
| Identity, credential, TLS, and cryptographic provider | Security-sensitive provider operation | Provider identity, key/credential custody, policy, algorithm, runtime, substitution |
| Testing, assurance, packaging, and deployment | Delivery and proof operation | Test scope, assurance tools, package identity, signing, deployment, rollback |

## Later extension families

The following applied families may be added as independently scoped extensions
after the required PLATFORM-001 families remain represented:

| Extension family | Contract focus | Boundaries that must remain explicit |
|---|---|---|
| Desktop and GUI | Interactive native application | GUI runtime, event loop, accessibility, graphics, packaging, signing |
| Networking and protocol | Client, server, or protocol behavior | Wire identity, transport, TLS, async runtime, timeouts, compatibility |
| Data, ML, and GPU | Data pipeline, model, or accelerator use | Dataset/model identity, kernels, drivers, device, precision, memory, fallback |

Required and extension families must not be merged into one universal stack.
For example,
browser WASM execution cannot be inferred from a produced `.wasm` file;
embedded architecture builds do not establish board behavior; and bundled
native source moves the native boundary into the build rather than removing
it. The measured research profiles demonstrate why lock universes,
target-active closures, compiler floors, and stage outcomes must remain
separate. See
[Rust compatibility-tested stack profiles](../../research/2026-08-10-rust-compatibility-stack-profiles.md).

## Minimum profile shape

Every profile should contain:

- stable profile ID, exact revision, schema version, owner, consumer, and
  application scope;
- mandatory requirements, preferences, alternatives, and explicit non-goals;
- exact direct releases, sources, requested features, lock identity, complete
  lock universe, and target-active closures;
- effective features, build scripts, procedural macros, `links`, generated
  code, unsafe boundaries, native code, public dependencies, and adapters;
- exact Cargo, rustc, toolchain, host, targets, components, linkers, runners,
  SDKs, native tools, providers, and deployment assumptions;
- independent results for resolve, check, lint, build, link, execute, test,
  conformance, package, sign, deploy, operational validation, and rollback;
- provenance, advisory, licensing, stewardship, support, limitations,
  residual unknowns, source dates, and evidence expiry;
- adoption authority, support period, exception process, escalation path,
  renewal triggers, substitution path, removal procedure, and exact rollback;
  and
- predecessor, successor, supersession, revocation, and retained historical
  evidence links.

The default maximum evidence age should be 90 days. A shorter consumer or risk
policy takes precedence. Material changes or revoked evidence trigger earlier
renewal.

## State vocabulary

Each stage and evidence source must retain a typed state. At minimum:

- pass;
- fail;
- expected rejection;
- unsupported;
- unavailable;
- not observed;
- stale; and
- unknown.

The ecosystem research also uses conflicting where sources disagree. These
states are not interchangeable. Unavailable is not unsupported; not observed
is not pass; stale is not current; and unknown must not be converted into a
recommendation. See
[ecosystem intervention decisions](../../research/2026-08-10-rust-ecosystem-intervention-decisions.md).

## Review lenses

The nine repository roles supply complementary review constraints:

- [Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md):
  compiler acceptance and advisory results are not safety proof.
- [Compiler Performance Engineer](../../../.roles/parliament/compiler-performance-engineer.md):
  distinguish check, build, test, link, cache state, causality, and variance.
- [Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md):
  make ABI, allocation, lifetime, panic, threading, and migration explicit.
- [AI Assurance Skeptic](../../../.roles/parliament/ai-assurance-skeptic.md):
  separate observed evidence from generated assertions and human decisions.
- [Ecosystem Strategist](../../../.roles/parliament/ecosystem-strategist.md):
  compose current owners rather than duplicate their tools or authority.
- [Rust Maintainer](../../../.roles/stakeholders/rust-maintainer.md):
  keep diagnostics actionable, adoption incremental, and removal simple.
- [Native Platform Adopter](../../../.roles/stakeholders/native-platform-adopter.md):
  retain platform, ABI, tooling, operations, support, audit, and recovery cost.
- [Scope Keeper](../../../.roles/editorial/scope-keeper.md):
  keep the capability bounded, product-neutral, and visibly non-authoritative.
- [Validation Checker](../../../.roles/editorial/validation-checker.md):
  require reproducible commands, representative fixtures, negative cases, and
  recorded environments.

Role guidance informs reviews but does not authorize implementation. A future
implementation still requires complete specifications, held-out evidence,
acceptance and stop criteria, lifecycle plans, all applicable role
dispositions, and a separately approved pulse.

## Current authority

These files are engineering guidance only. They do not define a stable schema,
select packages, create support obligations, authorize repository mutation, or
open product implementation. The current Ferris implementation authority is
limited to the bounded read-only work described in
[CONTEXT.md](../../../CONTEXT.md); profile generation and diffing remain future
read-only candidates behind a separate pulse.
