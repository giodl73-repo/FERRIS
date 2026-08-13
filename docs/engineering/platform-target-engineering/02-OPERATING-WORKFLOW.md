# Platform Engineering Operating Workflow

Status: Guidance
Implementation authority: None

## Purpose

This workflow turns a consumer platform need into a reviewable, renewable
profile without allowing Ferris to create owner truth or mutate an environment.
The workflow produces plans and evidence records. Owner-native tools perform
any separately approved work.

## Workflow summary

```text
intent
  -> boundary and owner inventory
  -> immutable input snapshot
  -> profile candidate
  -> qualification plan
  -> approved owner-local observations
  -> evidence join and review
  -> adopt, reject, constrain, or defer
  -> monitor and renew
  -> rollback, replace, or remove
```

Observation, planning, approval, execution, validation, outcome, and support
decision are separate records. A plan is not approval. An executed command is
not validation. A passing validation is not adoption.

## Step 1: define consumer intent

Name:

- application, repository, workspace, component, and owner;
- platform family and exact deployment class;
- required operations and non-goals;
- required `core`, `alloc`, or `std` layer;
- performance, memory, startup, image-size, power, availability, and support
  constraints;
- packaging, signing, debugging, deployment, update, rollback, and servicing
  obligations; and
- deadline, evidence age, risk class, and approval authority.

Examples of sufficiently bounded intent are "Windows x86-64 MSVC service
installed by the enterprise MSI channel" and "Cortex-M4F firmware for board X
under RTOS Y." "Desktop", "mobile", "WASM", and "embedded" are not sufficient.

## Step 2: inventory owners and immutable inputs

Record before proposing work:

- source revision, manifests, lockfile, Cargo configuration, toolchain file,
  profile configuration, and relevant generated-source policy;
- Cargo and rustc identities, host triple, target triple, target tier and
  target-specification source;
- lock universe and target-active normal, build, and development closures;
- requested/effective features, build scripts, procedural macros, `links`,
  generated code, unsafe and FFI boundaries;
- native source mode, provider, SDK, sysroot, compiler, linker, archiver,
  generator, package discovery, runner, runtime, and deployment system;
- policy owners for security, licensing, signing, release, operations, and
  support; and
- current result states, including missing and unknown evidence.

Do not run installation or active probing merely to complete this inventory.
Passive records may say `unavailable`, `not-observed`, or `unknown`.

## Step 3: construct a profile candidate

The candidate follows
[PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md). It must include
an exact profile ID and revision, consumer scope, owner, support state, evidence
dates, expiry, previous/replacement identities, required validation stages,
unsupported combinations, and removal/rollback path.

Separate:

- mandatory eligibility from preference;
- lockfile universe from active target closure;
- declared MSRV from observed Cargo/rustc behavior;
- target tier from crate and product support;
- provider capability from provider implementation;
- system-requested native identity from discovered and linked artifact; and
- expected future work from observed evidence.

## Step 4: build a qualification plan

The plan names owner-native commands or procedures without executing them. For
each stage, specify:

| Field | Requirement |
|---|---|
| Stage | Resolve, check, build, link, execute, test, package, sign, deploy, operate, rollback, or remove |
| Scope | Exact packages, targets, features, profiles, test binaries/cases, devices, or deployment ring |
| Environment | Host/target pair, image, tools, SDK, sysroot, runner, provider, credentials boundary |
| Expected result | Pass, expected rejection, unsupported, unavailable, or another typed outcome |
| Evidence | Commands, versions, logs, hashes, artifacts, environment identity, timestamps |
| Owner | Person/team/system authorized to perform and interpret the stage |
| Stop condition | Safety, cost, policy, unsupported, or unexpected-state boundary |
| Fallback | Wider owner-native validation or full reference path |

Qualification plans must include positive, negative, unavailable, and
not-observed cases. They must never delete required gates because a model
predicts success.

## Step 5: approve owner-local execution

Execution requires authority outside these guides. Approval must cover:

- the exact commands or system actions;
- allowed environment changes, if any;
- data, network, secrets, certificates, signing, device, and deployment scope;
- cost and time bounds;
- artifact retention and redaction;
- rollback and cleanup; and
- who may stop or override the run.

Ferris must not convert a read-only plan into `rustup target add`, SDK/package
installation, Cargo mutation, CI edits, signing, notarization, flashing, or
deployment without this authority.

## Step 6: run stage-specific qualification

### Resolution and closure

Use Cargo-owned evidence to capture exact resolution, lock universe, active
target/dependency-kind closure, features, build scripts, procedural macros, and
native edges. A resolved graph is not a compiled graph.

### Cross-check

Cross-check asks whether selected Rust units type-check for the target under the
recorded closure. Capture package-root and consumer scope separately. Do not
claim a linker, runtime, or deployment result.

### Cross-build and cross-link

Cross-build captures target code generation and artifact production.
Cross-link additionally requires the actual target linker, SDK/sysroot, native
libraries, CRT/runtime objects, and complete link inputs. Preserve the actual
tool discovered, even when it was found outside `PATH`.

### Cross-run and cross-test

Execution requires a target host, container, emulator, simulator, browser,
WASI runtime, device, RTOS runner, or hardware probe. Record runner identity,
transport, configuration, test binary/case identity, timeouts, and result.
Compiling a test harness is not running it.

### Packaging, signing, and deployment

Record package format and contents, debug-symbol package, dependency inventory,
signing or attestation identity, notarization/provisioning where applicable,
deployment target, installation result, health checks, rollback marker, and
servicing registration. Credentials must remain outside plans and durable
evidence.

### Debugging and operations

Qualify required source locations, symbols, locals/types, panic/unwind or crash
diagnosis, mixed-language debugging, telemetry, dump/symbol retrieval, and
field/service workflows. PDB, DWARF, dSYM, source maps, probe metadata, and
WASM debugging are separate platform artifacts.

## Step 7: join evidence without promotion

Create a result per stage. Do not infer:

- link from check;
- run from link;
- tests from one example execution;
- deploy from package creation;
- support from a local pass;
- reproducibility from one rebuild;
- security or safety from compiler acceptance; or
- universal portability from one host/target pair.

If evidence is absent, record `not-observed`; if prerequisites are missing,
record `unavailable`; if an owner rejects the combination, record
`unsupported`; if the cause is unresolved, record `unknown`.

## Step 8: review and decide

The decision is one of:

- adopt the exact profile;
- adopt with explicit constraints and unsupported combinations;
- defer pending named evidence;
- reject because mandatory eligibility failed;
- retain an experimental profile without support; or
- replace/remove an existing profile.

Review includes all applicable nine-role concerns. A role disposition is not
an implementation pulse and a Draft specification is not product-code
authority.

## Step 9: monitor, renew, and retire

Trigger renewal when packages, features, lock state, compiler/Cargo, tier,
target specification, architecture, provider, native component, SDK, sysroot,
linker, runner, packaging/signing, deployment, policy, ownership, advisories,
or evidence age changes. Renewal produces a reviewed diff.

Adoption must enter with a rollback and removal plan. Retirement must prove
that package, feature, generated, native, CI, signing, deployment, support, and
documentation residue is handled while retaining immutable historical
evidence.

## Platform-specific workflow additions

| Family | Required workflow addition |
|---|---|
| Linux | Test supported libc/distro or image floors, package source, dynamic loader, container/host deployment, service and debug workflow |
| Windows | Qualify SDK/toolset discovery, MSVC/GNU ABI, CRT, installer, PDB, signing, minimum OS, servicing |
| macOS | Qualify SDK/deployment target, architectures, codesigning/notarization, entitlements, dSYM, package/update |
| Android/iOS | Separate simulator/emulator and device, API/deployment levels, package, signing/provisioning, store/enterprise release, crash symbols |
| WASM/WASI/browser | Name runtime class, JS/WASI/component providers, browser/runtime matrix, glue/bundling, source maps, hosting/deployment |
| Embedded/bare metal/RTOS | Qualify final image, memory map/linker script, startup/panic/allocator, probe/device, timing/resource budgets, update/recovery |

## Sources

- [Ferris context](../../../CONTEXT.md)
- [Ferris product plan](../../../PRODUCT_PLAN.md)
- [PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md)
- [Platform compatibility research](../../research/2026-08-09-rust-platform-compatibility.md)
- [Native dependency research](../../research/2026-08-10-rust-native-dependency-boundary.md)
- [Mirrored native tools guide](../../reference/rust-reference/rust-architecture/18-CARGO-BUILD-SCRIPTS-PROC-MACROS-AND-NATIVE-TOOLS.md)
- [Mirrored supported profile lifecycle guide](../../reference/rust-reference/rust-crate-ecosystem/15-SUPPORTED-PROFILES-RENEWAL-AND-REMOVAL.md)

