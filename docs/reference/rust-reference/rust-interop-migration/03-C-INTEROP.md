---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:c-interop
kind: guide
module: rust-interop-migration
section: computing-software
title: C Interop
status: source-custody
source_custody: partial
current_path: rust-interop-migration/03-C-INTEROP.md
canonical_path: rust-interop-migration/03-C-INTEROP.md
backsource_ids: [mdloom-backfill:rust-interop-migration:03-c-interop]
concepts: [C interop, C ABI, opaque handle, bindgen, cbindgen, pkg-config, ownership across FFI, panic containment]
root_concepts: [C interop]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# C Interop

C is the native interop substrate for Rust, not because C types are inherently
safe, but because operating systems, linkers, and language runtimes converge on
C-compatible calling conventions and layouts. A good Rust/C boundary is small,
opaque, versioned, and explicit about every byte and lifetime.

## The Big Picture

```
+============================================================================+
|                         RUST <-> C BOUNDARY                                |
+============================================================================+
|  C header / ABI contract                                                   |
|  scalars | ptr+len | repr(C) records | opaque handles | callbacks          |
|      |                                                                     |
|      v                                                                     |
|  DIRECTION A - C CALLS RUST                                                |
|  cdylib/staticlib; cbindgen/manual header; controlled exports              |
|                                                                            |
|  DIRECTION B - RUST CALLS C                                                |
|  extern declarations; bindgen/manual declarations; native link inputs      |
|      |                                                                     |
|      v                                                                     |
|  BOUNDARY ADAPTER                                                          |
|  validate pointers -> copy/borrow -> Rust core -> translate error/panic    |
|      |                                                                     |
|      v                                                                     |
|  RUST CORE                                                                 |
|  String/Vec/enums/traits/async stay behind the adapter                     |
+============================================================================+
```

## Design the Header First

Prefer fixed-width integers, explicit lengths, and opaque incomplete types:

```c
#include <stddef.h>
#include <stdint.h>

typedef struct rim_parser rim_parser;

typedef uint32_t rim_status;
#define RIM_OK ((rim_status)0u)
#define RIM_INVALID_ARGUMENT ((rim_status)1u)
#define RIM_PARSE_ERROR ((rim_status)2u)
#define RIM_PANIC ((rim_status)255u)

#ifdef __cplusplus
extern "C" {
#endif
uint32_t rim_api_version(void);
rim_status rim_parser_new(rim_parser **out);
rim_status rim_parser_parse(
    rim_parser *parser,
    const uint8_t *data,
    size_t len,
    uint64_t *record_count);
void rim_parser_free(rim_parser *parser);
#ifdef __cplusplus
}
#endif
```

The incomplete `rim_parser` prevents C callers from depending on Rust layout.
The API uses caller-owned inputs and scalar outputs, with allocation confined to
an opaque handle that Rust also frees.

## Implement an Opaque Handle

```rust
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    ptr,
    slice,
};

pub struct Parser {
    records: u64,
}

#[unsafe(no_mangle)]
pub extern "C" fn rim_api_version() -> u32 {
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rim_parser_new(out: *mut *mut Parser) -> u32 {
    if out.is_null() {
        return 1;
    }
    // SAFETY: the C contract requires `out` to be aligned and writable.
    unsafe { out.write(ptr::null_mut()) };
    let raw = Box::into_raw(Box::new(Parser { records: 0 }));
    // SAFETY: same `out` contract; ownership of `raw` transfers to the caller.
    unsafe { out.write(raw) };
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rim_parser_parse(
    parser: *mut Parser,
    data: *const u8,
    len: usize,
    out: *mut u64,
) -> u32 {
    if parser.is_null() || out.is_null() || (data.is_null() && len != 0) {
        return 1;
    }
    let result = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: for nonzero length, the C contract requires `data` readable
        // for `len` bytes. Rust slices require a non-null pointer even at zero,
        // so the empty case is handled without calling `from_raw_parts`.
        let bytes = if len == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(data, len) }
        };
        // SAFETY: the contract requires a live handle created by
        // `rim_parser_new`, exclusive access for this call, and writable `out`.
        let parser = unsafe { &mut *parser };
        parser.records += bytes.iter().filter(|&&b| b == b'\n').count() as u64;
        unsafe { out.write(parser.records) };
        0
    }));
    result.unwrap_or(255)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rim_parser_free(parser: *mut Parser) {
    if parser.is_null() {
        return;
    }
    // SAFETY: the contract permits exactly one release of a live handle
    // returned by `rim_parser_new`; `Parser::drop` must not panic.
    unsafe { drop(Box::from_raw(parser)) };
}
```

`catch_unwind` is useful only with an unwinding panic strategy and only for
unwinding Rust panics. It does not validate foreign pointers, catch process
aborts, or make arbitrary foreign exceptions safe. The ordinary contract
remains: no panic or foreign unwind crosses this C ABI. The exports are marked
`unsafe` for Rust callers because their raw-pointer preconditions cannot be
checked by the type system; that qualifier does not alter the C ABI.

## Calling C from Rust

```rust
use std::ffi::{c_char, c_int, CString};

#[link(name = "legacy")]
unsafe extern "C" {
    fn legacy_open(path: *const c_char) -> c_int;
}

pub fn open(path: &str) -> Result<(), i32> {
    let path = CString::new(path).map_err(|_| -1)?;
    let rc = unsafe { legacy_open(path.as_ptr()) };
    if rc == 0 { Ok(()) } else { Err(rc) }
}
```

The safe wrapper owns NUL validation, pointer lifetime, and return-code
translation. It should also document whether the C function stores the pointer;
if it does, this wrapper is unsound because `CString` dies on return.

## Binding and Build Tools

| Tool | Direction | Use | Boundary warning |
|------|-----------|-----|------------------|
| `bindgen` | C/C++ headers -> Rust declarations | Large existing headers | Generated output mirrors unsafe source contracts; wrap it |
| `cbindgen` | Rust exports -> C/C++ header | Rust-owned C API | Review generated ABI and publish the header as a versioned artifact |
| `cc` crate | Compile C/C++ in `build.rs` | Bundled native source | Host/target compiler and flags must match Cargo target |
| `pkg-config` crate/tool | Discover Unix native libraries | System dependencies | Not a Windows-universal packaging story |
| `vcpkg` crate/tool | Discover vcpkg libraries | Common Windows native dependencies | Triplet and linkage mode are part of target matrix |

Keep generated bindings either reproducibly generated in CI or intentionally
checked in with the generating tool/version recorded. Do not let a workstation's
header search path silently define production ABI.

## Data Layout Rules

| C contract | Rust representation |
|------------|---------------------|
| `uint32_t` | `u32` |
| `size_t` | `usize` for same-target calls |
| `const uint8_t *p, size_t n` | `*const u8, usize`, validated then borrowed |
| Plain C struct | `#[repr(C)]` with FFI-safe fields |
| Tagged union | Fixed-width integer tag plus explicit `repr(C)` union/payload contract |
| NUL string | `CStr`/`CString`; encoding stated separately |
| Opaque object | Forward declaration in C, pointer to private Rust type |
| Callback | `Option<unsafe extern "C" fn(...)>` plus context pointer |

`bool`, `long`, `wchar_t`, C enum representation, bitfields, variadic
functions, packed structs, and compiler-specific constructs require special
care. Prefer fixed-width integer status/tag fields and avoid platform-sized or
compiler-specific constructs unless they are the point of the platform API.

## Boundary Hazard Register

| Hazard | C boundary rule |
|--------|-----------------|
| ABI | Use explicit `extern "C"`/`extern "system"` and `repr(C)` where appropriate; export stable symbols; never expose Rust ABI or trait objects. |
| Allocator | Rust frees Rust allocations via exported destructors; C frees C allocations; do not exchange `malloc`/`Vec` ownership implicitly. |
| Panic/unwind | Catch and map at Rust exports when applicable; prohibit C++ exceptions, `longjmp`, or Rust panic unwinding through ordinary frames. |
| Lifetime | State pointer validity, mutability, aliasing, and retention for every parameter and callback context. |
| Threading | Mark handles thread-safe, thread-confined, or externally synchronized; specify callback reentrancy. |
| Target | Verify compiler family, architecture, endianness, alignment, libc/CRT, and calling convention. |
| Packaging | Ship header, import/static/shared library, transitive native dependencies, symbols/debug files, license, and loader instructions together. |

## Old World -> New World Bridge

| C/C++ practice | Rust boundary equivalent |
|----------------|--------------------------|
| PIMPL / incomplete struct | Opaque Rust handle |
| Constructor/destructor pair | `new`/`free` exports owning `Box::into_raw`/`from_raw` internally |
| HRESULT/errno result | Stable status enum plus optional error-detail retrieval |
| SAL annotations | Machine-readable header annotations plus prose ownership/thread contract |
| RAII wrapper around C handle | Safe Rust newtype with `Drop` |
| DEF/export map | Controlled symbol surface for `cdylib`/`staticlib` |

## Common Confusion Points

- **"A non-null pointer is valid."** It must also be aligned, initialized,
  dereferenceable for the stated size, and satisfy aliasing for the access.
- **"`size_t` is always 64-bit."** It follows the target pointer width.
- **"C can free Rust memory with `free`."** Only if the implementation
  deliberately allocated with the same compatible allocator contract. Do not
  make that assumption; export a Rust release function.
- **"`catch_unwind` makes FFI safe."** It addresses one failure path, not pointer
  validity, allocator identity, abort panics, or foreign exceptions.
- **"Generated bindings are safe wrappers."** They are usually unsafe
  declarations. Build a small safe adapter.
- **"A shared object is self-contained."** Loader paths, CRTs, dependent shared
  libraries, and symbol versions remain packaging concerns.

## Decision Cheat Sheet

| Need | Pattern |
|------|---------|
| Stable native object boundary | Opaque handle plus versioned C functions |
| Borrow input only during call | Pointer plus length, validated before slice creation |
| Return variable data | Caller-provided buffer/two-call sizing or Rust object plus Rust free |
| Call a broad existing C library | `bindgen` for declarations, handwritten safe facade |
| Publish a Rust C API | `cbindgen` or reviewed manual header plus ABI tests |
| Cross-thread use | Explicit thread-safe handle contract; internal synchronization if promised |
| Rich polymorphic API | Keep polymorphism behind opaque handle; do not export Rust trait objects |

## Primary Sources

- Rustonomicon, FFI: https://doc.rust-lang.org/nomicon/ffi.html
- Rust Reference, external blocks: https://doc.rust-lang.org/reference/items/external-blocks.html
- Rust Reference, type layout: https://doc.rust-lang.org/reference/type-layout.html
- `std::ffi`: https://doc.rust-lang.org/std/ffi/
- bindgen user guide: https://rust-lang.github.io/rust-bindgen/
- cbindgen documentation: https://github.com/mozilla/cbindgen

## Related Guides

- Previous: [02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md](02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md)
- Next: [04-CPP-INTEROP.md](04-CPP-INTEROP.md)
- Ownership and unwind details: [12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md](12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md)
