---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:nodejs-javascript-interop
kind: guide
module: rust-interop-migration
section: computing-software
title: Node.js and JavaScript Interop
status: source-custody
source_custody: partial
current_path: rust-interop-migration/07-NODEJS-JAVASCRIPT-INTEROP.md
canonical_path: rust-interop-migration/07-NODEJS-JAVASCRIPT-INTEROP.md
backsource_ids: [mdloom-backfill:rust-interop-migration:07-nodejs-javascript-interop]
concepts: [Node.js interop, JavaScript interop, Node-API, napi-rs, wasm-bindgen, event loop, native addon, npm packaging]
root_concepts: [Node.js interop]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Node.js and JavaScript Interop

For Node.js, prefer Node-API through a maintained binding such as `napi-rs`
when native performance is required. Prefer WebAssembly when browser reach,
sandboxing, or easier cross-platform delivery outweighs native integration.
Either way, keep blocking work off the JavaScript event-loop thread.

## The Big Picture

```
+============================================================================+
|                   JAVASCRIPT HOST -> RUST OPTIONS                          |
+============================================================================+
|  JavaScript / TypeScript API                                               |
|  objects | Buffer/TypedArray | Promise | Error | AbortSignal               |
|      |                                                                     |
|      v                                                                     |
|  OPTION A - NODE-API ADDON                                                 |
|  napi-rs; native threads/OS APIs; npm prebuild matrix                      |
|                                                                            |
|  OPTION B - WEBASSEMBLY MODULE                                             |
|  wasm-bindgen/component host; sandbox target; JS glue + wasm artifact      |
|      |                                                                     |
|      v                                                                     |
|  RUST ADAPTER -> HOST-NEUTRAL RUST CORE                                    |
|  convert/copy/borrow -> schedule -> map error/panic -> settle/callback     |
+============================================================================+
```

## A Native Addon with `napi-rs`

```rust
use napi::bindgen_prelude::*;
use napi_derive::napi;

pub struct CountTask {
    bytes: Vec<u8>,
}

impl Task for CountTask {
    type Output = u32;
    type JsValue = u32;

    fn compute(&mut self) -> Result<Self::Output> {
        Ok(self.bytes.iter().filter(|&&b| b == b'\n').count() as u32)
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

#[napi]
pub fn count_records_async(input: Buffer) -> AsyncTask<CountTask> {
    AsyncTask::new(CountTask { bytes: input.to_vec() })
}
```

Treat this as scoped shape, not a version-free promise: use the exact `napi-rs`
APIs documented by the pinned release. The synchronous form is appropriate only
for bounded fast work. CPU-heavy or blocking work belongs on a worker, with
completion marshalled back to JavaScript. `AsyncTask` uses host-managed worker
capacity; bound concurrency so native work cannot starve unrelated libuv work.
The `#[napi]` attribute belongs on the exported function, not on the `Task`
trait implementation.

## Node-API versus WebAssembly

| Criterion | Node-API native addon | WebAssembly |
|-----------|-----------------------|-------------|
| Node integration | Strong: Buffer, Promise, classes, threadsafe functions | Through JS glue/runtime |
| Browser reuse | No | Yes, with environment-specific APIs |
| Native OS/library access | Direct | Restricted/imported capabilities |
| Artifact matrix | OS/arch/libc prebuilds | Often fewer binary variants, but runtime/features vary |
| Peak native performance | Best for suitable workloads | Good, with boundary/copy/runtime constraints |
| Crash isolation | Same process | Sandbox helps memory isolation; host/process failures still matter |
| ABI story | Node-API version contract | WebAssembly/module/component contract |

Node-API is designed to provide ABI stability across supported Node versions for
its API level. That does not make the addon independent of OS, architecture,
libc, dependent shared libraries, or the binding crate's support matrix.

## Event Loop, Workers, and Callbacks

```
  JS event-loop thread
      |
      +-- quick native call ----------> return value
      |
      +-- schedule Rust work ---------> worker/runtime
                                          |
                                          v
                                      completion
                                          |
                                          v
                              settle Promise on JS-safe path
```

Never call JavaScript values directly from an arbitrary Rust thread. Use the
binding's thread-safe function/channel mechanism to marshal work to a valid
JavaScript execution context. Define whether cancellation stops queued work,
cooperatively signals running work, or merely ignores its eventual result.

## Buffers and Object Lifetimes

`Buffer` and typed-array views can avoid copies, but the native code must respect
the host object's backing-store lifetime and mutation rules. If work outlives
the call, use a binding-supported retained reference or copy the bytes. A raw
pointer retained after the JavaScript object becomes collectible is invalid.

For Rust-owned external buffers, define the finalizer and ensure destruction
runs safely even during environment shutdown. Do not assume finalizers run on a
convenient worker thread or that JavaScript can be called during teardown.

## npm Packaging

| Strategy | Shape |
|----------|-------|
| Platform packages | Main JS package selects `@scope/pkg-win32-x64`, `...-linux-x64-gnu`, etc. |
| One package with optional dependencies | Installer resolves platform prebuild |
| Build from source | Requires Rust/native toolchain; use only when product supports it |
| WASM package | JS wrapper plus `.wasm`, with browser/Node export conditions |

Test `require` and ESM import paths, package `exports`, TypeScript declarations,
Node versions, bundlers where claimed, and clean installation without a Rust
toolchain. Linux glibc and musl builds are different targets and package
identities.

## Boundary Hazard Register

| Hazard | Node/JavaScript boundary rule |
|--------|-------------------------------|
| ABI | Use Node-API or WebAssembly contract; native core may use C ABI; never expose Rust ABI or trait objects. |
| Allocator | JS runtime owns JS buffers/objects; Rust frees Rust allocations/finalizers; avoid implicit cross-heap ownership. |
| Panic/unwind | Map panic/error to rejected Promise or JS Error; never unwind through Node/V8 frames. |
| Lifetime | Retain host objects through supported references or copy before async work; define external-buffer finalization. |
| Threading | JavaScript access stays on valid host context; marshal worker completion/callbacks; state reentrancy. |
| Target | Validate Node-API level, Node versions, OS, architecture, libc, and WASM runtime features. |
| Packaging | Publish correct prebuild selection, exports, dependent libraries, symbols, provenance, and source-build policy. |

## Old World -> New World Bridge

| Familiar model | Rust interop mapping |
|----------------|----------------------|
| Native Node addon | Node-API adapter over a host-neutral core |
| Browser Web Worker | Off-main-thread computation with message-shaped completion |
| .NET `Task` returned from native wrapper | JavaScript Promise settled on the host-safe thread |
| `Buffer`/typed array | Borrowed or retained byte view with GC-backed lifetime |
| npm optional platform dependency | RID-like native artifact selection |

## Common Confusion Points

- **"Node-API means one binary for every platform."** It reduces Node-version
  ABI churn, not OS/architecture/libc variation.
- **"`async fn` automatically avoids blocking Node."** Only if the binding
  schedules work appropriately; Rust async can still execute blocking code.
- **"A Buffer pointer can be retained."** Only with a supported retained
  lifetime or copy.
- **"WASM is always portable and zero-install."** Runtime features, JS glue,
  filesystem/network imports, and bundler behavior still vary.
- **"Dropping a Promise cancels native work."** JavaScript Promises have no
  intrinsic cancellation; design `AbortSignal` or an explicit cancel handle.
- **"A Rust trait can be a JS class vtable."** Project a concrete JS API over
  opaque state; trait-object ABI is not stable.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Deep Node integration and native libraries | Node-API via `napi-rs` |
| Browser and Node reuse | WebAssembly plus environment adapters |
| CPU-heavy operation | Worker/AsyncTask; settle Promise on host-safe path |
| Long-lived input buffer | Supported retained reference or copy |
| Cancellation | Explicit `AbortSignal`/cancel handle and cooperative checks |
| Broad package install success | Prebuilt per-target npm packages plus clean-install tests |
| Maximum crash isolation | Separate process/service |

## Primary Sources

- Node-API documentation: https://nodejs.org/api/n-api.html
- `napi-rs` documentation: https://napi.rs/
- Node.js addon context awareness: https://nodejs.org/api/addons.html
- wasm-bindgen guide: https://rustwasm.github.io/docs/wasm-bindgen/
- npm package specification: https://docs.npmjs.com/cli/configuring-npm/package-json

## Related Guides

- Previous: [06-PYTHON-INTEROP.md](06-PYTHON-INTEROP.md)
- Next: [08-JVM-INTEROP.md](08-JVM-INTEROP.md)
- Async and cancellation: [13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md](13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md)
