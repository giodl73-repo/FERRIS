---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:dotnet-csharp-interop
kind: guide
module: rust-interop-migration
section: computing-software
title: .NET and C# Interop
status: source-custody
source_custody: partial
current_path: rust-interop-migration/05-DOTNET-CSHARP-INTEROP.md
canonical_path: rust-interop-migration/05-DOTNET-CSHARP-INTEROP.md
backsource_ids: [proof-backfill:rust-interop-migration:05-dotnet-csharp-interop]
concepts: [.NET interop, C# interop, P/Invoke, LibraryImport, SafeHandle, reverse P/Invoke, NuGet RID, native library]
root_concepts: [.NET interop]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# .NET and C# Interop

The durable shape is a native C ABI with an idiomatic managed projection.
P/Invoke is the transport, `SafeHandle` is the ownership membrane, and a NuGet
package plus RID assets is the deployment unit. Rust types remain private.

## The Big Picture

```
+============================================================================+
|                    .NET HOST -> RUST NATIVE CORE                           |
+============================================================================+
|  C# API                                                                    |
|  Span/Memory | SafeHandle | exceptions/Result object | Task                |
|      |                                                                     |
|      v                                                                     |
|  GENERATED MARSHAL STUB                                                    |
|  [LibraryImport] / PInvoke source generation                               |
|      |                                                                     |
|      v                                                                     |
|  VERSIONED C ABI                                                           |
|  fixed-width scalars | ptr+len | opaque handles | status | callbacks       |
|      |                                                                     |
|      v                                                                     |
|  RUST ADAPTER -> RUST CORE                                                 |
|  validate -> borrow/copy -> execute -> translate error/panic               |
|                                                                            |
|  PACKAGE: NuGet                                                            |
|  lib/netX managed assembly + runtimes/<rid>/native/<library>               |
+============================================================================+
```

## Export a C-Shaped Rust API

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rim_sum(
    data: *const i32,
    len: usize,
    out: *mut i64,
) -> i32 {
    if out.is_null() || (data.is_null() && len != 0) {
        return 1;
    }
    let values = if len == 0 {
        &[]
    } else {
        // SAFETY: the foreign contract requires `data` readable for `len`
        // elements for the duration of this call.
        unsafe { std::slice::from_raw_parts(data, len) }
    };
    // SAFETY: the foreign contract requires `out` aligned and writable.
    unsafe { out.write(values.iter().map(|&x| i64::from(x)).sum()) };
    0
}
```

For production, wrap the body in the module's panic policy and document the
pointer contract. Do not export `String`, `Vec`, Rust references, enums without
a fixed C representation, futures, or trait objects.

## Project It with `LibraryImport`

```csharp
using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

internal static partial class Native
{
    [LibraryImport("rim_native", EntryPoint = "rim_sum")]
    [UnmanagedCallConv(CallConvs = new[] { typeof(CallConvCdecl) })]
    internal static unsafe partial int Sum(
        int* data, nuint length, out long result);
}

public sealed class RimException : Exception
{
    public int Status { get; }

    public RimException(int status)
        : base($"rim_native failed with status {status}")
    {
        Status = status;
    }
}

public static class Rim
{
    public static unsafe long Sum(ReadOnlySpan<int> values)
    {
        fixed (int* p = values)
        {
            int rc = Native.Sum(p, (nuint)values.Length, out long result);
            if (rc != 0) throw new RimException(rc);
            return result;
        }
    }
}
```

`LibraryImport` source generation provides inspectable marshalling and trimming/
NativeAOT-friendly benefits for supported signatures. `DllImport` remains valid
for existing code. For hot paths, prefer blittable scalars and pinned spans over
implicit string/array marshalling. The example explicitly selects Cdecl to match
Rust `extern "C"` on its supported targets; if an export uses `extern "system"`
or another ABI, the managed declaration must match it exactly.

## Own Native Resources with `SafeHandle`

```
  Rust: parser_new -> opaque pointer
                   |
                   v
  C#: SafeParserHandle
      owns pointer, prevents premature collection during calls
                   |
                   v
  SafeHandle.ReleaseHandle -> Rust parser_free
```

Derive from `SafeHandle` rather than storing `IntPtr` in an ordinary class.
`SafeHandle` coordinates lifetime with P/Invoke calls and gives deterministic
`Dispose` plus finalization fallback. The Rust destructor must tolerate exactly
the null/invalid cases promised by the handle wrapper and must not unwind.

## Strings, Errors, and Callbacks

| Concern | Recommended contract |
|---------|----------------------|
| Input text | UTF-8 pointer plus byte length; C# uses explicit UTF-8 encoding or custom marshaller |
| Output text | Caller buffer/two-call sizing, or Rust-owned buffer plus Rust free |
| Ordinary error | Stable status code plus copied structured/detail data |
| Managed surface | Translate status to exception, result type, or domain response |
| Rust panic | Catch/map where unwind is enabled; reserve a terminal status and log correlation |
| Callback | Function pointer plus context; pin delegate lifetime and define callback thread |

Reverse calls are the dangerous half. A delegate passed to native code must be
kept alive as long as native code can call it. The callback must not let a
managed exception escape into native frames. In NativeAOT or advanced hosting,
`UnmanagedCallersOnly` can expose managed entry points, but it still needs an
explicit unmanaged signature and exception boundary.

## NuGet and RID Packaging

```text
package/
  lib/net8.0/Rim.Managed.dll
  runtimes/win-x64/native/rim_native.dll
  runtimes/linux-x64/native/librim_native.so
  runtimes/osx-arm64/native/librim_native.dylib
```

Test restore, publish, single-file, trimming, NativeAOT, and self-contained
deployment modes that the product claims to support. Native library resolution
can be customized with `NativeLibrary.SetDllImportResolver`, but avoid
workstation-only PATH fixes. A RID package is a compatibility promise, not just
a folder name.

## Boundary Hazard Register

| Hazard | .NET boundary rule |
|--------|--------------------|
| ABI | P/Invoke a C/system ABI with exact calling convention and blittable layout; never marshal Rust ABI or trait objects. |
| Allocator | Use `SafeHandle`/Rust free for Rust objects and buffers; do not free Rust memory with `Marshal.FreeHGlobal` unless contractually allocated that way. |
| Panic/unwind | Map Rust panic to status; catch managed exceptions before reverse P/Invoke returns. |
| Lifetime | Pin spans only for call duration; root delegates/context for callback lifetime; use `SafeHandle` for native ownership. |
| Threading | State callback thread, synchronization-context behavior, reentrancy, and whether handles are thread-safe. |
| Target | Validate RID, architecture, OS, CRT/libc, .NET runtime, trimming, and NativeAOT combinations. |
| Packaging | Put native assets in correct RID folders; test resolver, signing, symbols, servicing, and dependency closure. |

## Old World -> New World Bridge

| .NET prior art | Rust interop mapping |
|----------------|----------------------|
| COM callable wrapper / RCW | Managed projection around a foreign lifetime contract |
| `SafeHandle` | Canonical owner of an opaque Rust handle |
| HRESULT | Stable native status translated to managed exception/result |
| `Span<T>` | Call-scoped pointer-plus-length view |
| `CancellationToken` | Explicit cancel handle/flag or message; not an implicit Rust future cancellation contract |
| NuGet RID asset | Native target artifact selected by restore/publish |

## Common Confusion Points

- **"`IntPtr` is enough."** It has no ownership, finalization, or in-flight call
  protection. Use `SafeHandle`.
- **"Marshalling copies are harmless."** On hot paths they can dominate; make
  encoding and copy behavior explicit and measure end-to-end.
- **"Pinned means permanently stable."** A `fixed` span is pinned for that
  scope only. Native code must not retain it.
- **"Managed exceptions can cross the native callback."** Catch them and map to
  a native error channel.
- **"Any DLL in the NuGet package will load."** RID selection, dependent native
  libraries, architecture, and publish modes determine load success.
- **"C# interfaces can map to Rust trait objects."** Build a managed interface
  over opaque handles/functions; Rust trait-object ABI is not durable.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Native resource lifetime | `SafeHandle` calling a Rust release export |
| Blittable hot-path input | `ReadOnlySpan<T>` pinned for the call |
| Text | Explicit UTF-8 pointer plus byte length |
| Ordinary failure | Status code/structured detail, translated by managed wrapper |
| Managed callback | Rooted delegate or supported unmanaged function pointer plus context |
| Multi-platform delivery | NuGet RID native assets with publish-mode tests |
| Maximum isolation/rollback | Separate Rust service instead of P/Invoke |

## Primary Sources

- .NET native interoperability best practices: https://learn.microsoft.com/dotnet/standard/native-interop/best-practices
- `LibraryImportAttribute`: https://learn.microsoft.com/dotnet/api/system.runtime.interopservices.libraryimportattribute
- `SafeHandle`: https://learn.microsoft.com/dotnet/api/system.runtime.interopservices.safehandle
- Native library loading: https://learn.microsoft.com/dotnet/standard/native-interop/native-library-loading
- NuGet native files: https://learn.microsoft.com/nuget/create-packages/native-files-in-net-packages

## Related Guides

- Previous: [04-CPP-INTEROP.md](04-CPP-INTEROP.md)
- Next: [06-PYTHON-INTEROP.md](06-PYTHON-INTEROP.md)
- Windows projections: [09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md](09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md)
