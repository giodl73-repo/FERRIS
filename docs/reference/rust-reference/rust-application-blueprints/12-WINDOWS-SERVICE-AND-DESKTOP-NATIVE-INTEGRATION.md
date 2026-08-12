---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:windows-service-and-desktop-native-integration
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Windows Service and Desktop Native Integration Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/12-WINDOWS-SERVICE-AND-DESKTOP-NATIVE-INTEGRATION.md
canonical_path: rust-application-blueprints/12-WINDOWS-SERVICE-AND-DESKTOP-NATIVE-INTEGRATION.md
backsource_ids: [mdloom-backfill:rust-application-blueprints:12-windows-service-and-desktop-native-integration]
concepts: [windows service, desktop application, win32, com, scm, native integration, installer rollback]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Windows Service and Desktop Native Integration Blueprint

## The Big Picture

```
+============================================================================+
| WINDOWS HOST AUTHORITIES                                                   |
| SCM/session; window dispatcher; COM apartment; registry; installer         |
+============================================================================+
             | service callbacks                 | UI/native callbacks
             v                                   v
       service host                        desktop/native host
       start/stop/control                  messages/callbacks/UI thread
             \                                   /
              \                                 /
               v                               v
+----------------------------------------------------------------------------+
| neutral application core: use cases | domain policy | ports                |
+----------------------------------------------------------------------------+
             |                                   |
             v                                   v
       Windows adapters                    portable adapters
       Win32/COM/registry/events            store/network/files
```

The universal pattern is host integration: an external lifecycle authority owns
the thread, callback, identity, and shutdown rules, while a neutral application
core owns semantic behavior. Windows supplies several distinct hosts; combining
service, desktop, COM, and installer concerns into one crate hides those rules.

## Workspace Layout

```
device-manager/
|-- Cargo.toml
|-- crates/
|   |-- device-domain/
|   |-- device-application/
|   |-- windows-api-adapter/
|   |-- windows-service-host/
|   |-- windows-desktop-host/
|   `-- installer-contract/
|-- apps/
|   |-- device-service/
|   `-- device-desktop/
|-- installer/
`-- tests/
    `-- windows-integration/
```

```toml
# Cargo.toml (virtual workspace root)
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]
```

```toml
# crates/windows-api-adapter/Cargo.toml
[target.'cfg(windows)'.dependencies]
windows-sys = { version = "0.61.2", features = ["Win32_Foundation"] }
```

Keep `cfg(windows)` and native bindings near adapters. Portable domain crates
should compile and test on a host target without importing Win32 or COM types.
The binding version/feature above is a valid narrow example as of writing, not a
module-wide dependency promise; pin the repository-selected binding and only the
API feature sets it uses.

## Service Lifecycle

```
SCM start
   |
register control handler
   |
report start-pending -> initialize bounded dependencies
   |
report running -> admit work
   |
stop/shutdown control -> stop admission -> drain/cancel
   |
report stopped with observable result
```

| Service concern | Owner |
|-----------------|-------|
| Installation/account/start mode | installer/operator |
| SCM status/control | service host |
| Domain work | application core |
| Recovery actions | SCM/operator policy plus idempotent application startup |
| Credentials/ACLs | deployment/security authority |
| Logs/events | application telemetry contract plus Windows adapter |

Use a least-privilege service identity and explicit service/IPC object ACLs.
Define trusted executable, DLL, plugin, and configuration search paths; avoid
ambient current-directory loading. Installer elevation, COM registration,
named-pipe access, registry writes, and update signing are separate privileged
surfaces with separate audit and rollback requirements.

Services run outside an interactive desktop session. Do not put user prompts or
desktop UI in a service. Use authenticated IPC or another explicit protocol
between service and desktop applications.

## Desktop, Win32, and COM Integration

```
UI/message thread
    |
    +--> translate event to application command
    |
    +<-- render state/result
    |
    `--> marshal callbacks/results back to owning thread
```

| Boundary | Explicit rule |
|----------|---------------|
| Window dispatcher | thread affinity and non-blocking handlers |
| COM | apartment initialization, interface lifetime, marshaling |
| Native handles | owning wrapper, close semantics, invalid/null cases |
| Strings | UTF-16 conversion and invalid input policy |
| Callbacks | lifetime, reentrancy, panic containment |
| Registry/files | hive/path, virtualization, ACL, transaction/recovery |

Wrap unsafe calls in narrow adapters whose safe API states ownership and
threading invariants. A successful native return code is not automatically a
valid domain result; convert errors at the adapter boundary while preserving
diagnostic codes.

## Testing and Rollback

```
portable core tests
   -> Windows adapter unit/contract tests
   -> service lifecycle tests on Windows
   -> desktop/COM thread-affinity scenarios
   -> packaged installer upgrade/repair/uninstall tests
```

```text
cargo test --workspace --all-targets
cargo build --workspace --release --target x86_64-pc-windows-msvc
# install in an isolated Windows test environment, exercise start/stop/upgrade,
# verify service identity, IPC, files/registry, and rollback
```

Artifact signing, installer technology, target triples, and Windows versions are
repository/deployment decisions. State and test the supported matrix rather than
implying universal compatibility. The MSVC target command requires a Windows
host with the matching Visual Studio Build Tools/Windows SDK environment; it is
not a promise that another host can cross-link an MSVC application.

| Change | Rollback requirement |
|--------|----------------------|
| Service binary | stop/drain, replace, restart under same compatible config |
| IPC contract | old/new service and desktop versions interoperate during window |
| COM registration | side-by-side or atomic registration repair plan |
| Registry/state | old version reads current schema or migration is forward-repaired |
| Installer | previous package retained; upgrade/uninstall custom actions reversible |

Removal must stop and drain the service, unregister SCM/COM/protocol/UI hooks,
remove or migrate IPC clients, revoke service credentials and ACL grants,
preserve/export state under policy, and validate installer uninstall/repair from
mixed old/new versions.

## Universal Bridge First

The universal bridge is inversion of control at an OS host boundary. GUI event
loops, Unix daemons, mobile activities, and Windows services all receive
lifecycle callbacks they do not own; correctness comes from translating them
into a host-neutral state machine.

Supplementally, .NET Windows Services, WinForms/WPF dispatchers, P/Invoke, and
COM interop supply close conceptual analogues. Rust requires more explicit
native ownership and panic/FFI boundaries because there is no CLR host enforcing
those contracts.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| Background machine-wide process | service host plus neutral application core |
| Interactive per-user UI | desktop host; communicate with service via explicit IPC |
| Native API call | narrow adapter with owning handle types |
| COM integration | apartment-aware adapter and thread-affinity tests |
| Cross-platform behavior | portable core plus target-specific adapter crates |
| Simple scheduled maintenance | scheduled job [05] may be preferable to a service |
| Upgrade with state | installer plus expand/contract state and IPC compatibility |

## Common Confusion Points

- **A service is not a hidden desktop app.** Session isolation and identity are
  fundamental.
- **`Send` does not override OS thread affinity.** COM apartments and UI objects
  impose external rules beyond Rust traits.
- **RAII wrappers need correct ownership provenance.** Not every borrowed native
  handle may be closed by the wrapper.
- **Catching panic across FFI is not optional design polish.** Unwinding across
  unsupported boundaries can violate contracts; contain it before callbacks
  return.
- **Installer success is not application readiness.** Validate service startup,
  credentials, IPC, and state compatibility.
- **Registry and COM registration are durable state.** Binary rollback alone may
  not restore them.
- **A service account is a security boundary.** LocalSystem or broad object ACLs
  are not harmless defaults.

## Primary Sources

- Microsoft Windows service documentation: https://learn.microsoft.com/windows/win32/services/services
- Microsoft COM documentation: https://learn.microsoft.com/windows/win32/com/component-object-model--com--portal
- Microsoft Win32 API documentation: https://learn.microsoft.com/windows/win32/api/
- Rust for Windows: https://github.com/microsoft/windows-rs
- Rust Nomicon, FFI: https://doc.rust-lang.org/nomicon/ffi.html
- Rust platform support: https://doc.rust-lang.org/rustc/platform-support.html

## Related Guides

- Shared contract: [01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md](01-BLUEPRINT-CONTRACT-ANATOMY-AND-CROSS-CUTTING-CONCERNS.md)
- Scheduled alternative: [05-SCHEDULED-AND-BATCH-JOB.md](05-SCHEDULED-AND-BATCH-JOB.md)
