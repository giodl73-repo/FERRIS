---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:webassembly-and-component-application
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: WebAssembly and Component Application Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/10-WEBASSEMBLY-AND-COMPONENT-APPLICATION.md
canonical_path: rust-application-blueprints/10-WEBASSEMBLY-AND-COMPONENT-APPLICATION.md
backsource_ids: [mdloom-backfill:rust-application-blueprints:10-webassembly-and-component-application]
concepts: [webassembly, component model, wasi, browser wasm, host imports, capability security, interface versioning]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# WebAssembly and Component Application Blueprint

## The Big Picture

```
+============================================================================+
| Rust source -> target/toolchain -> core Wasm module or component artifact  |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| host runtime                                                               |
| instantiate | validate imports | allocate resources | invoke | interrupt   |
+----------------------+----------------------+------------------------------+
                       v                      v
                host capabilities       component/module exports
          clock/files/network/random           |
                       +-----------+------------+
                                   v
                         result + state + telemetry
```

WebAssembly defines a portable instruction and embedding substrate. Its
isolation properties depend on the runtime, host imports, resource controls, and
embedding. Browser APIs, WASI capabilities, component interfaces, memory limits,
and persistence are host contracts that must be selected and tested against an
exact runtime/toolchain combination.

## Workspace Layout

```
rules-component/
|-- Cargo.toml
|-- crates/
|   |-- rules-core/             # portable policy, minimal platform assumptions
|   |-- rules-component/        # exported component/module boundary
|   |-- rules-host/             # native host and capability implementations
|   `-- interface-model/        # interface definitions/fixtures
|-- apps/
|   `-- rules-host-cli/
|-- wit/                        # when using component interfaces
`-- tests/
    |-- component-integration/
    `-- browser-integration/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]
```

Keep portable policy independently testable on the native host. The Wasm-facing
crate should own representation conversion and exported/imported functions, not
all domain behavior.

## Select the Execution Contract

| Environment | Inputs/outputs | Capability source |
|-------------|----------------|-------------------|
| Browser core Wasm | JavaScript/Web APIs and linear-memory bindings | embedding page/runtime |
| WASI command/reactor style | WASI imports provided by runtime | explicit preopened/host resources |
| Component Model | typed imported/exported interfaces | component runtime and interface definitions |
| Embedded host | application-specific imports | host application |

Common rustc targets name different contracts:

| Example target | Boundary |
|----------------|----------|
| `wasm32-unknown-unknown` | minimal core Wasm; browser/host bindings supplied separately |
| `wasm32-wasip1` | WASI Preview 1 command-style environment |
| `wasm32-wasip2` | WASI 0.2/component-oriented target; verify runtime and binding path |

The Component Model and WASI ecosystem continues to evolve. Do not state that a
specific proposal, target, adapter, or runtime feature is universally available;
pin and verify the versions named by the repository.

## Interface, Memory, and Capability Ownership

```
domain value
    |
    v
interface representation
    |
    v
canonical/binding conversion
    |
    v
host resource or guest function
```

| Boundary | Contract decision |
|----------|-------------------|
| Strings/bytes | encoding, size limit, malformed input |
| Records/variants | versioning and unknown-case behavior |
| Resources/handles | ownership, drop/close, host lifetime |
| Calls | sync/async model, deadline, reentrancy |
| State | guest memory, host store, external service, snapshot |
| Errors | semantic categories versus trap/runtime failure |

The host owns capability grant, resource budgets, instantiation, and interruption.
The component owns exported behavior and private state. Interface owners define
versioning. A module cannot access a filesystem or network merely because its
source code requests one; the host must provide the relevant imports.

Enforce memory/table/instance/handle counts, execution time or fuel/epoch
budget, recursion/stack limits, and outbound capability scope in the runtime.
These controls are runtime-specific. Wasm does not remove denial-of-service,
side-channel, confused-deputy, or vulnerable-host-import risks.

## Testing and Rollback

Evidence layers:

```
native rules-core tests
      -> binding/interface fixture tests
      -> exact runtime instantiation tests
      -> capability-denial and resource-limit tests
      -> packaged browser/host smoke tests
```

```text
cargo test -p rules-core
cargo test -p rules-host
rustup target add wasm32-unknown-unknown
cargo build -p rules-component --target wasm32-unknown-unknown
# For WASI/component output, substitute the pinned wasip1/wasip2 target,
# binding tool, and runtime, then run the same interface fixtures.
```

Browser, WASI Preview 1, WASI 0.2/components, and custom hosts do not share one
universal build or invocation command. Pin the exact target, binding generator,
runtime, and feature set in repository instructions.

Rollback usually means switching an immutable artifact or component graph:

| Change | Safe rollback condition |
|--------|-------------------------|
| Guest logic | prior guest still satisfies current host imports |
| Interface additive | prior host tolerates absent optional export/capability |
| Host import change | old and new guests covered by negotiation/window |
| Persistent state | old guest can read current state or state is versioned |
| Runtime upgrade | retained runtime/artifact matrix has been tested |

Removal must stop new instantiation, revoke capability grants, drain or
interrupt active calls under the lifecycle contract, remove component graph
references, and migrate/export/delete guest or host state before discarding the
artifact.

## Universal Bridge First

The universal bridge is a virtual ISA plus capability-based linking: code is
portable because it imports a declared environment rather than assuming the
host OS ABI. The component layer raises those imports/exports from raw functions
and memory to typed interfaces.

Supplementally, this resembles CLR bytecode plus hosting APIs, but Wasm does not
imply the CLR's object model, BCL, GC, reflection, or ambient OS services.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| Browser computation/UI helper | browser Wasm boundary plus JS/Web integration |
| Portable server-side capability unit | WASI/component, after runtime matrix verification |
| Tiny same-process extension | ordinary Rust crate may be simpler |
| Third-party isolated extension | component/plugin host [09] with explicit capabilities |
| Shared policy across native and Wasm | portable core crate plus thin bindings |
| Direct OS/device control | native/embedded [11] unless host exposes bounded capability |
| Fast rollback | immutable guest artifacts and versioned interfaces/state |

## Common Confusion Points

- **Wasm is not automatically portable source.** Dependencies and target
  assumptions may still require OS facilities unavailable to the guest.
- **Linear memory is not a stable high-level interface.** Prefer generated or
  deliberately specified bindings over shared layout guesses.
- **Sandboxing depends on host grants.** Broad imports can recreate broad
  authority.
- **Wasm is not a complete hostile-code policy.** Resource exhaustion,
  side-channels, and vulnerable host imports remain embedding responsibilities.
- **A trap is not a domain error.** Keep semantic errors in the interface where
  callers can handle them.
- **Browser Wasm and WASI are different hosts.** Commands, APIs, and deployment
  assumptions should not be blended.
- **Runtime support claims age quickly.** Pin versions and verify the exact
  component/toolchain path in CI.

## Primary Sources

- WebAssembly specifications: https://webassembly.org/specs/
- Rust and WebAssembly book: https://rustwasm.github.io/docs/book/
- WebAssembly Component Model: https://component-model.bytecodealliance.org/
- WASI: https://wasi.dev/
- Rust platform support: https://doc.rust-lang.org/rustc/platform-support.html
- Rust `wasm32-wasip2` target: https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip2.html

## Related Guides

- Plugin host: [09-PLUGIN-AND-EXTENSION-HOST.md](09-PLUGIN-AND-EXTENSION-HOST.md)
- Embedded target: [11-EMBEDDED-AND-EDGE-DEVICE.md](11-EMBEDDED-AND-EDGE-DEVICE.md)
