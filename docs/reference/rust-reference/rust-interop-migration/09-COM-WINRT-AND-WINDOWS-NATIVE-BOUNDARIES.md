---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:com-winrt-windows-native-boundaries
kind: guide
module: rust-interop-migration
section: computing-software
title: COM, WinRT, and Windows-Native Boundaries
status: source-custody
source_custody: partial
current_path: rust-interop-migration/09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md
canonical_path: rust-interop-migration/09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md
backsource_ids: [mdloom-backfill:rust-interop-migration:09-com-winrt-windows-native-boundaries]
concepts: [COM, WinRT, Windows API, windows crate, HRESULT, IUnknown, apartment threading, HSTRING, Windows packaging]
root_concepts: [Windows-native interop]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# COM, WinRT, and Windows-Native Boundaries

Windows offers durable platform ABIs that Rust can consume or implement:
Win32's C/system ABI, COM's `IUnknown`-based binary contract, and WinRT's
metadata-projected interface system. Use them when Windows identity is truly
part of the product contract. They supplement, rather than replace, universal
C, schema, WIT, or process boundaries.

## The Big Picture

```
+============================================================================+
|                       WINDOWS-NATIVE BOUNDARIES                            |
+============================================================================+
|  UNIVERSAL CORE CONTRACT                                                   |
|  process/protocol or versioned C ABI around Rust core                      |
|      |                                                                     |
|      v                                                                     |
|  PROJECTION A - WIN32 C/SYSTEM ABI                                         |
|  handles, BOOL, HRESULT, windows crate                                     |
|                                                                            |
|  PROJECTION B - COM: IUnknown, GUID, vtable, AddRef/Release, apartment     |
|  PROJECTION C - WINRT: metadata, HSTRING, async, capabilities              |
|      |                                                                     |
|      v                                                                     |
|  WINDOWS TARGET/PACKAGE                                                    |
|  MSVC/GNU choice | x64/arm64 | DLL search | registration | MSIX/signing    |
+============================================================================+
```

## Use Generated Windows Bindings

The `windows` crate generates bindings from Windows metadata and projects many
Win32, COM, and WinRT APIs into Rust types. It reduces declaration errors; it
does not remove platform semantics.

```rust
use std::{marker::PhantomData, rc::Rc};
use windows::{
    core::Result,
    Win32::System::Com::{
        CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED,
    },
};

struct ComApartment(PhantomData<Rc<()>>);

impl Drop for ComApartment {
    fn drop(&mut self) {
        // SAFETY: paired with the successful `CoInitializeEx` on this thread.
        unsafe { CoUninitialize() };
    }
}

fn main() -> Result<()> {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok()?; }
    let _apartment = ComApartment(PhantomData);
    // Call COM APIs on this thread under the selected apartment model.
    Ok(())
}
```

The exact crate features and method projections depend on the pinned `windows`
crate version. Generate only the namespaces needed and keep unsafe platform
calls behind a safe domain adapter. Every successful `CoInitializeEx` call,
including `S_FALSE`, requires a matching `CoUninitialize` on the same thread;
the guard makes that balance explicit. Its `Rc` marker prevents moving the guard
to another thread before destruction.

## COM Contract

```
  interface pointer
       |
       v
  +--------------------------------------------------------+
  | vtable                                                 |
  | QueryInterface -> discover IID/interface               |
  | AddRef         -> increment owner count                |
  | Release        -> decrement; destroy at zero           |
  | methods        -> HRESULT plus ABI parameters          |
  +--------------------------------------------------------+

  identity + interface versioning + reference counting + apartment rules
```

COM interfaces are binary contracts defined by IID and vtable order. Extending
an existing interface by appending methods is not a safe versioning strategy for
arbitrary consumers; define a new IID/interface and expose it through
`QueryInterface`. Rust implementations must make reference counting, identity,
aggregation (if any), and threading model coherent.

Do not reinterpret a Rust trait object as a COM interface. Although both have a
data/vtable flavor, Rust trait-object layout and ABI are not stable. Use a
proper generated or explicitly defined COM implementation.

## Apartments, Marshaling, and Callbacks

COM threading is not merely "thread-safe or not." An STA object is associated
with an apartment and often relies on message dispatch; an MTA object may be
called concurrently. Interface pointers crossing apartments may require COM
marshaling. Callbacks/events can reenter while the original call stack is
active.

| Question | Contract |
|----------|----------|
| Initialization | Which thread calls `CoInitializeEx`, with which apartment model? |
| Pointer transfer | Is interface marshaled, agile, or apartment-bound? |
| Reentrancy | Can outgoing calls pump messages or invoke callbacks? |
| Shutdown | Which thread releases final references and unadvises events? |
| Error | How are `HRESULT`, restricted error info, and Rust errors mapped? |

## WinRT

WinRT builds on COM-like ABI principles with metadata-driven projections,
`HSTRING`, standardized interfaces, async operations, and capability/package
integration. Use metadata/projection tools rather than hand-authoring vtables.
For external contracts, preserve interface/version semantics and do not leak
Rust collection, enum, future, or trait layouts.

Initialize the Windows Runtime through its supported API/projection helper
(`RoInitialize` or the pinned `windows` crate equivalent) and balance that
initialization on the same thread. The COM-only `CoInitializeEx` example above
must not be treated as proof that every WinRT activation context is ready.

WinRT async objects are not Rust futures. An adapter may await/project them, but
must define cancellation, completion thread/context, and lifetime. Likewise,
Rust async work exposed as WinRT requires a projection that owns the operation
state after the initiating call returns.

## Allocators and Windows Value Types

| Value | Owner/release |
|-------|---------------|
| `HANDLE` | API-specific close function such as `CloseHandle`, not `free` |
| COM interface pointer | `Release` |
| `CoTaskMem` buffer | `CoTaskMemFree` |
| `BSTR` | `SysFreeString` |
| `HSTRING` | Windows string API/projection ownership |
| Rust `Box`/buffer | Rust-exported release function |

"Windows allocator" is not one allocator. Match every value to its documented
release API and encode that in a Rust RAII wrapper.

## Boundary Hazard Register

| Hazard | Windows boundary rule |
|--------|-----------------------|
| ABI | Use Win32 `system` ABI, COM/WinRT defined interfaces, or C ABI; never substitute Rust ABI or trait objects. |
| Allocator | Pair each HANDLE/BSTR/HSTRING/CoTaskMem/COM/Rust value with its exact close/release function. |
| Panic/unwind | Translate Rust panic/error to `HRESULT`/failure; do not unwind through Windows callbacks or COM frames. |
| Lifetime | Honor AddRef/Release, borrowed parameter duration, event subscription, and apartment-bound pointer rules. |
| Threading | Initialize apartments, state agility/marshaling, handle reentrancy, and release on valid contexts. |
| Target | Test Windows version, x64/arm64, MSVC/GNU where claimed, SDK contract, and packaged/unpackaged context. |
| Packaging | Define DLL search, COM registration/manifest, MSIX identity/capabilities, signing, symbols, and servicing. |

## Old World -> New World Bridge

| Windows prior art | Rust migration mapping |
|-------------------|------------------------|
| HRESULT boundary | Rust error translated to stable platform status |
| `CComPtr`/smart pointer | Generated Rust COM wrapper with `Drop`/clone reference counting |
| IDL projection | Metadata/header remains canonical; Rust code is one projection |
| STA message pump | Scheduler/affinity contract that Rust workers must respect |
| Registration-free COM | Deployment-owned activation manifest |
| MSIX | Signed package identity and native dependency closure |

## Common Confusion Points

- **"COM is just a C++ class ABI."** It is a specified interface ABI with
  identity, IIDs, reference counting, and marshaling rules.
- **"A Rust trait object can implement COM directly."** Use a real COM layout/
  projection; trait-object ABI is compiler-private.
- **"`Send` means a COM pointer can cross apartments."** Rust's marker traits do
  not override COM marshaling and agility rules.
- **"All Windows memory uses `CoTaskMemFree`."** Release is API/type-specific.
- **"`CoInitializeEx` is process initialization."** It is per-thread and every
  successful call must be balanced on that same thread.
- **"WinRT async equals a Rust future."** It requires an ownership, completion,
  context, and cancellation adapter.
- **"A DLL beside the EXE always loads."** Packaged apps, Safe DLL search,
  dependencies, architecture, and activation model affect resolution.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Call Win32/COM/WinRT from Rust | `windows` crate with minimal namespace features |
| Long-lived public native core | Universal C ABI, then optional COM/WinRT facade |
| Existing COM ecosystem/activation | Proper COM interface implementation and new IID for evolution |
| Modern packaged Windows projection | WinRT metadata/projection if product model fits |
| Cross-apartment callback | Explicit marshaling/agility plus reentrancy plan |
| Rust-owned buffer in Windows API | Keep Rust release export; do not guess platform allocator |
| Cross-platform product | Keep Windows bridge supplemental to a universal core boundary |

## Primary Sources

- `windows` crate documentation: https://microsoft.github.io/windows-docs-rs/
- COM technical overview: https://learn.microsoft.com/windows/win32/com/com-technical-overview
- Processes, threads, and apartments: https://learn.microsoft.com/windows/win32/com/processes--threads--and-apartments
- Windows Runtime design: https://learn.microsoft.com/windows/uwp/winrt-cref/winrt-type-system
- Dynamic-link library security: https://learn.microsoft.com/windows/win32/dlls/dynamic-link-library-security

## Related Guides

- Previous: [08-JVM-INTEROP.md](08-JVM-INTEROP.md)
- Next: [10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md](10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md)
- .NET projection: [05-DOTNET-CSHARP-INTEROP.md](05-DOTNET-CSHARP-INTEROP.md)
