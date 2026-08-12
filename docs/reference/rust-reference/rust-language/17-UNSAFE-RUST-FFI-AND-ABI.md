---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:unsafe-rust-ffi-and-abi
kind: guide
module: rust-language
section: languages
title: Unsafe Rust, FFI, and ABI
status: source-custody
source_custody: partial
current_path: rust-language/17-UNSAFE-RUST-FFI-AND-ABI.md
canonical_path: rust-language/17-UNSAFE-RUST-FFI-AND-ABI.md
backsource_ids: [mdloom-backfill:rust-language:17-unsafe-rust-ffi-and-abi]
concepts: [unsafe, raw pointers, FFI, extern, ABI, repr, ownership across FFI, panic unwinding, C interop, C++ bridges]
root_concepts: [unsafe]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Unsafe Rust, FFI, and ABI

`unsafe` is not "turn off safety." It is a promise: *"I have personally verified
an invariant the compiler cannot."* Safe Rust proves memory safety; `unsafe`
lets you do five specific things the checker cannot validate, and your job is to
wrap them in a **sound abstraction** whose safe API can never trigger undefined
behavior. FFI (calling C, being called by C) is the most common reason to reach
for it. This is the layer where a C/C++ background is a genuine advantage.

```
+===============================================================================+
|                    UNSAFE = 5 SUPERPOWERS + A PROOF OBLIGATION                |
+===============================================================================+

  THE FIVE THINGS `unsafe` UNLOCKS         SOUND ABSTRACTION
  --------------------------------         -----------------
  1. dereference a raw pointer *const/*mut  safe public API
  2. call an unsafe fn / FFI fn                 |  (callers can't cause UB)
  3. access/modify a mutable static         +---+------------------+
  4. implement an unsafe trait (Send/Sync)  |  unsafe { ... }      |  <- you prove
  5. access a union field                   |  invariant held here |     the invariant
                                            +----------------------+
  unsafe does NOT disable: borrow checker,  Vec, Mutex, etc. are exactly this:
  type checking, lifetimes on safe code       unsafe internals, safe surface.

  FFI + ABI                                REPR (layout control)
  ---------                                ----------------------
  unsafe extern "C" { fn c_fn(...); }      #[repr(C)]      C-style field layout
    declare foreign functions              #[repr(transparent)] newtype = inner ABI
  extern "C" fn rust_fn() { }              #[repr(u8)]     enum tag size
    export (#[unsafe(no_mangle)])           repr(C) still needs FFI-safe fields
  types: *const T, CStr/CString, c_int
```

## The Five Powers (and What `unsafe` Does *Not* Do)

Inside an `unsafe { }` block you may: (1) dereference raw pointers `*const
T`/`*mut T`; (2) call `unsafe fn`s (including all FFI functions); (3) read/write a
`static mut`; (4) implement an `unsafe trait` (like `Send`/`Sync`); (5) read
`union` fields. That is the **entire** list.

Critically, `unsafe` does **not** disable the borrow checker, type checker, or
lifetime rules on the surrounding safe code. It only unlocks those five
operations. The mental model that trips up newcomers: `unsafe` is a *smaller*
hole than they fear — it does not make Rust into C; it lets you do five audited
things while everything else stays checked.

## Raw Pointers and Sound Abstractions

Raw pointer types carry no checked lifetime and may be null. Creating and
carrying one is often safe; dereferencing it or converting it into a reference
requires `unsafe`. That access still must satisfy provenance, alignment,
initialization, dereferenceability, and aliasing requirements. Raw pointers are
less statically constrained than references, not exempt from Rust's memory
model.

```rust
let mut x = 5;
let p: *mut i32 = &mut x;      // creating a raw pointer: SAFE
unsafe { *p += 1; }            // dereferencing: UNSAFE — you assert p is valid
```

The discipline: confine `unsafe` to the smallest region, and expose a **safe
API** that upholds the invariant for all callers. `Vec`, `String`, `Mutex`, and
`Rc` are literally this pattern — unsafe pointer manipulation inside, a safe
interface that cannot be misused from outside. When you write `unsafe`, document
the invariant with a `// SAFETY:` comment (clippy's `undocumented_unsafe_blocks`
enforces this), and make sure no safe caller can violate it.

## FFI: Calling C

Declare foreign functions in an `extern` block and call them inside `unsafe`:

```rust
use std::os::raw::c_int;
unsafe extern "C" {
    fn abs(input: c_int) -> c_int;      // from libc
}
fn main() {
    let n = unsafe { abs(-3) };          // FFI call is unsafe
    println!("{n}");
}
```

The **2024 edition** requires `unsafe extern "C" { ... }`: declaring the foreign
signature is itself a trust assertion. Current compilers accept that explicit
form in older editions too, so it is the portable style for new code.

## FFI: Being Called by C, and the ABI

Export a Rust function with a C ABI and a stable symbol name:

```rust
#[unsafe(no_mangle)]              // Edition 2024 unsafe-attribute syntax
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }   // callable from C as `add`
```

The **ABI string** (`"C"`, `"system"`, `"stdcall"`, etc.) selects the calling
convention. `"C"` is the lingua franca. `#[unsafe(no_mangle)]` disables Rust's
name mangling so the linker sees `add`. For layout, `#[repr(C)]` gives a struct
C-style field order, alignment, and padding for the target ABI. It is necessary
but not sufficient: every field and function parameter must also have a defined
C-compatible representation, and pointer ownership/validity remains a manual
contract. References, `String`, `Vec`, trait objects, and many Rust enums are not
made FFI-safe by wrapping them in an outer `repr(C)`.

```rust
#[repr(C)]                       // mirrors `struct Point { double x, y; }`
pub struct Point { pub x: f64, pub y: f64 }
```

`#[repr(transparent)]` says a single-field newtype has the exact ABI of its inner
type when its representation constraints are met. `#[repr(u8)]` fixes a
fieldless enum's discriminant representation; data-carrying enum layouts require
a deliberately mirrored tagged-union design.

## Strings and Ownership Across FFI

C strings are NUL-terminated byte arrays without length; Rust strings are
length-prefixed UTF-8 without NUL ([10](10-STRINGS-TEXT-AND-UNICODE.md)). Bridge
with `CStr`/`CString`:

```rust
use std::ffi::{CStr, CString};
let owned = CString::new("hello").unwrap();     // adds NUL; owns the buffer
let ptr = owned.as_ptr();                        // *const c_char to hand to C
// receiving from C:
// let s = unsafe { CStr::from_ptr(c_ptr) }.to_str().unwrap();
```

**Ownership across the boundary is a manual contract** — the compiler cannot track
it. Decide explicitly who frees what: if Rust allocates and C must free, provide a
matching `free` export; if C hands Rust a pointer to borrow, do not free it. Use
`Box::into_raw`/`Box::from_raw` to move heap ownership out to C and back, and
`std::mem::forget`/`ManuallyDrop` ([03](03-OWNERSHIP-MOVES-COPY-AND-DROP.md)) to
suppress a Rust-side drop when C took ownership. Mismatches here are the classic
FFI double-free / leak.

## Panics Must Not Cross FFI

`extern "C"` is a **non-unwind ABI**. If a Rust panic escapes such a function,
Rust aborts the process rather than unwinding through C frames. In an unwinding
panic profile, wrap fallible exported bodies in `catch_unwind` and translate the
panic into an error code; `catch_unwind` cannot catch `panic = "abort"`.
`extern "C-unwind"` is the explicit ABI for cases where both sides and the target
toolchain deliberately support cross-language unwinding.

```rust
fn do_work() { /* application logic that may panic */ }

#[unsafe(no_mangle)]
pub extern "C" fn safe_entry() -> i32 {
    std::panic::catch_unwind(|| { do_work(); 0 }).unwrap_or(-1)
}
```

## C++ Bridges

Raw `extern "C"` works for a C-shaped interface, but C++ has name mangling,
templates, exceptions, and non-trivial object layout that plain FFI cannot
express. In practice:

- **`cxx` crate** — the standard tool for safe Rust<->C++ interop; you declare a
  shared bridge and it generates matching glue on both sides, handling
  `std::string`, `std::unique_ptr`, and lifetimes safely.
- **`bindgen`** — auto-generates Rust `extern` declarations from C/C++ headers.
- **`cbindgen`** — the reverse: generates a C/C++ header from your Rust FFI
  surface so C++ callers can link your `#[repr(C)]` API.

For a C++ reader: think of `unsafe`/FFI as the equivalent of dropping to
`reinterpret_cast` and manual lifetime management, but *bounded* — you carry the
same obligations you already carry in C++ (validity, aliasing, ownership), except
Rust makes the boundary explicit and everything outside it stays checked.

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| C pointers everywhere | raw `*const`/`*mut` | Creation/carrying can be safe; access and reference creation require `unsafe` |
| C++ `reinterpret_cast` / manual lifetimes | `unsafe` blocks | Bounded; must uphold documented invariants |
| `extern "C"` in C++ | `extern "C"` in Rust | Same ABI concept |
| `struct` layout (implementation-defined) | `#[repr(C)]` for FFI | Default Rust layout is unspecified |
| P/Invoke / `DllImport` (.NET) | `extern` block + FFI | Same "call native code" role |
| COM / marshaling | manual FFI + `cxx`/`bindgen` | You control marshaling |
| `char*` ownership conventions | `CString`/`CStr` + explicit free | Ownership is a manual contract |
| SEH / exceptions across boundary | `catch_unwind` or explicit `"C-unwind"` | Non-unwind C exports abort if a Rust panic escapes |

## Common Confusion Points

- **`unsafe` is a promise, not an off switch.** It unlocks five operations;
  everything else stays checked. Undefined behavior inside `unsafe` is *your* bug,
  and it can corrupt safe code elsewhere.
- **Keep `unsafe` tiny and wrapped.** Expose a safe API; document invariants with
  `// SAFETY:`. Do not sprinkle `unsafe` to silence errors.
- **`repr(C)` is necessary, not sufficient.** Use it for mirrored C structs and
  ensure every field/signature type is itself FFI-safe.
- **Ownership across FFI is manual.** Decide who frees; use
  `Box::into_raw`/`from_raw`. Double-free and leaks live here.
- **Do not let a panic escape `extern "C"`.** It aborts the process. In unwind
  builds, catch and translate it; use `"C-unwind"` only for a deliberate,
  supported unwind contract.
- **`static mut` is a hazard.** Accessing it is `unsafe` and easy to get wrong;
  prefer atomics or `OnceLock`/`Mutex` ([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md), [18](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)).
- **Exercise with Miri.** Run `cargo +nightly miri test` to detect UB on covered
  paths; it is evidence, not a soundness proof
  ([19](19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md)).

## Decision Cheat Sheet

| Situation | Do |
|-----------|-----|
| Call a C library function | `unsafe extern "C"` block + `unsafe` call |
| Expose Rust to C | `#[unsafe(no_mangle)] pub extern "C" fn` |
| Pass a struct across FFI | mirrored FFI-safe fields + `#[repr(C)]` |
| Newtype with inner ABI | `#[repr(transparent)]` |
| Pass strings to/from C | `CString` / `CStr` |
| Hand heap ownership to C | `Box::into_raw` (and provide a free fn) |
| Prevent a Rust-side drop | `mem::forget` / `ManuallyDrop` |
| Contain panics at a non-unwind boundary | `catch_unwind` in unwind builds; translate to an error |
| Deliberately support cross-language unwind | `"C-unwind"` plus matching platform/toolchain contract |
| Interop with C++ | `cxx` crate (safe) / `bindgen` / `cbindgen` |
| Assert `Send`/`Sync` manually | `unsafe impl` (rare; you own the proof) |
| Check unsafe code for UB | `cargo +nightly miri` |

## Primary Sources

- The Book, Ch. 19.1 (Unsafe Rust): https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
- The Rustonomicon (unsafe & FFI): https://doc.rust-lang.org/nomicon/
- Reference — `extern` blocks & ABI: https://doc.rust-lang.org/reference/items/external-blocks.html
- Reference — Type layout & `repr`: https://doc.rust-lang.org/reference/type-layout.html
- std::ffi (CStr/CString/OsStr): https://doc.rust-lang.org/std/ffi/index.html
- The cxx crate (C++ interop): https://cxx.rs/

## Related Guides

- Previous: [16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)
- Next: [18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md](18-CONST-STATICS-CFG-FEATURES-AND-EDITIONS.md)
- Drop / forget / ManuallyDrop: [03-OWNERSHIP-MOVES-COPY-AND-DROP.md](03-OWNERSHIP-MOVES-COPY-AND-DROP.md)
- Miri & sanitizers: [19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md](19-TESTING-DOCUMENTATION-FUZZING-AND-BENCHMARKING.md)
