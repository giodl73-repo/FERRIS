---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:plugin-and-extension-host
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Plugin and Extension Host Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/09-PLUGIN-AND-EXTENSION-HOST.md
canonical_path: rust-application-blueprints/09-PLUGIN-AND-EXTENSION-HOST.md
backsource_ids: [proof-backfill:rust-application-blueprints:09-plugin-and-extension-host]
concepts: [plugin host, extension api, abi, capability boundary, process isolation, compatibility matrix]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Plugin and Extension Host Blueprint

## The Big Picture

```
+============================================================================+
| host application authority                                                 |
| policy | lifecycle | resource budgets | extension registry | user consent  |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| extension boundary                                                         |
| discover -> authenticate -> negotiate version -> grant capabilities        |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| invocation transport                                                       |
| Rust callback | C ABI | subprocess protocol | Wasm/component               |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| plugin implementation -> allowed effects -> result/events                  |
+============================================================================+
```

A plugin system transfers bounded behavior to code with an independent release
lifecycle. The host must own discovery, compatibility negotiation, capability
granting, failure containment, and disable/recovery. "Load a dynamic library" is
only one transport and is often the least portable contract.

## Workspace Layout

```
editor-host/
|-- Cargo.toml
|-- crates/
|   |-- editor-core/
|   |-- extension-contract/     # versioned semantic messages/types
|   |-- extension-host/
|   |-- extension-transport-process/
|   `-- extension-test-kit/
|-- apps/
|   `-- editor/
|-- plugins/
|   `-- sample-formatter/
`-- tests/
    `-- compatibility-matrix/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "plugins/*", "tests/*"]
```

Repository co-location is useful for sample and first-party plugins, but the
contract must still model independent versions if third parties can release
outside the host cadence.

## Choose the Boundary Deliberately

| Boundary | Strength | Main liability |
|----------|----------|----------------|
| Rust trait/callback in one build | simplest and strongly typed | same process/build graph; no security isolation |
| Native C-compatible ABI | low overhead and broad tool support | unsafe ownership/layout/version discipline; no process isolation |
| Subprocess protocol | crash/process isolation and language neutrality | serialization, startup, supervision |
| Wasm/component | portable capability-oriented sandbox model | runtime/component ecosystem evolves; verify exact versions |

Rust does not promise a stable native Rust ABI. Loading independently built Rust
dylibs and passing ordinary Rust types across the boundary is therefore not a
durable cross-version contract. Use one coordinated build, a deliberately
designed C-compatible ABI, a process protocol, or a Wasm boundary.

## Capability and Lifecycle Contract

```
discover candidate
      |
verify identity/provenance
      |
negotiate contract version
      |
grant capability handles
      |
initialize -> ready -> invoke -> quiesce -> shutdown
                  |
               fault -> quarantine/disable
```

| Capability | Host policy question |
|------------|----------------------|
| Filesystem | which roots and operations? |
| Network | which destinations/protocols? |
| UI | which surfaces and thread/dispatcher rules? |
| Data | read, propose mutation, or direct mutation? |
| Secrets | opaque operation or raw secret access? |
| Compute | time, memory, concurrency, cancellation budgets? |

Prefer capability handles over a global host object. The host owns user policy
and authoritative mutation. Plugins own their private state and declared
behavior. The transport owns framing, lifetime, and error conversion.

Capability handles constrain cooperative code only when the execution boundary
enforces them. A native in-process plugin can call OS APIs directly, corrupt
memory through unsafe code, or exhaust the process; use an OS-sandboxed
subprocess or correctly configured Wasm runtime when hostile-code isolation is a
requirement. Signatures/provenance identify an artifact source but do not prove
the artifact is safe.

## Compatibility, Testing, and Rollback

Maintain a matrix, not a single version:

| Host | Plugin contract | Expected |
|------|-----------------|----------|
| current | current | full |
| current | previous supported | compatible/degraded |
| previous supported | current compatible subset | explicit result |
| current | unknown future | reject safely |
| current | malformed/hostile | contain and preserve diagnostics |

```text
cargo test --workspace --all-targets
# build sample plugin(s), launch real host transport, negotiate,
# exercise timeout/crash/malformed input, then disable and restart
```

Native ABI tests need exact target/toolchain/build-policy boundaries. Process and
Wasm protocols need serialized fixture and resource-limit tests.

Rollback mechanisms:

- disable or quarantine one plugin without downgrading the host;
- retain previous plugin artifact and contract support through the window;
- store plugin state with explicit schema/version and migration ownership;
- run side-by-side only when state and capability isolation are real;
- make host startup resilient to one incompatible extension.

Uninstall/removal must quiesce callbacks, revoke capability handles and
credentials, unregister commands/UI/routes, release host resources, and either
migrate, export, or delete plugin state under a declared retention policy.
Deleting a binary while callbacks or persisted host references remain is not
removal.

## Universal Bridge First

The universal bridge is operating-system capability design: independent code is
given handles to specific powers rather than ambient authority. Browser
extensions, database extensions, IDE plugins, and game mods vary mainly in
isolation and compatibility mechanisms.

Supplementally, .NET assembly loading and MEF-style composition resemble
in-process plugins, but a stable CLR metadata/runtime boundary has no direct
equivalent in Rust's native ABI. That difference should drive transport choice.

## Decision Cheat Sheet

| Need | Boundary |
|------|----------|
| First-party extension in same release | ordinary crate/trait callback |
| Third-party native low-latency plugin | carefully versioned C ABI |
| Strong crash/language isolation | subprocess protocol |
| Portable capability sandbox | Wasm/component [10] |
| Untrusted arbitrary code with broad OS access | do not claim safety from a plugin API alone |
| User recovery from bad extension | independent disable/quarantine path |
| Stable long-lived ecosystem | explicit negotiation, test kit, compatibility matrix |

## Common Confusion Points

- **A Rust trait is not a stable binary ABI.** It works across crates in a
  coordinated build, not arbitrary compiler/version boundaries.
- **Sandbox is not synonymous with Wasm.** Host imports and runtime
  configuration determine granted authority.
- **Version numbers do not negotiate behavior by themselves.** Define required,
  optional, and unknown capabilities.
- **Catching a panic is not complete isolation.** Memory corruption, aborts, and
  process-wide resource exhaustion need stronger boundaries.
- **Plugins should not own host truth by default.** Let them propose actions
  through validated host capabilities.
- **Discovery is a security surface.** Search paths, signatures/provenance, and
  precedence need policy.
- **Capabilities need enforcement.** Passing a narrow Rust interface to native
  in-process code does not prevent ambient OS access.

## Primary Sources

- Rust Reference, linkage: https://doc.rust-lang.org/reference/linkage.html
- Rust Nomicon, FFI: https://doc.rust-lang.org/nomicon/ffi.html
- WebAssembly Component Model: https://component-model.bytecodealliance.org/
- WASI: https://wasi.dev/
- Cargo build scripts: https://doc.rust-lang.org/cargo/reference/build-scripts.html

## Related Guides

- Library surface: [08-REUSABLE-LIBRARY-AND-SDK.md](08-REUSABLE-LIBRARY-AND-SDK.md)
- Wasm boundary: [10-WEBASSEMBLY-AND-COMPONENT-APPLICATION.md](10-WEBASSEMBLY-AND-COMPONENT-APPLICATION.md)
