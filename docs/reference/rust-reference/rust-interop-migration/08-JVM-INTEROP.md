---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:jvm-interop
kind: guide
module: rust-interop-migration
section: computing-software
title: JVM Interop
status: source-custody
source_custody: partial
current_path: rust-interop-migration/08-JVM-INTEROP.md
canonical_path: rust-interop-migration/08-JVM-INTEROP.md
backsource_ids: [proof-backfill:rust-interop-migration:08-jvm-interop]
concepts: [JVM interop, Java interop, JNI, Foreign Function and Memory API, direct ByteBuffer, native library, JAR packaging]
root_concepts: [JVM interop]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# JVM Interop

The JVM offers two principal native routes: JNI, the long-lived universal
contract, and the newer Foreign Function and Memory API on modern JDKs. Both
should target a narrow C-shaped Rust surface. Java/Kotlin objects and Rust
objects remain private to their runtimes.

## The Big Picture

```
+============================================================================+
|                       JVM HOST -> RUST CORE                                |
+============================================================================+
|  Java / Kotlin API                                                         |
|  records | ByteBuffer | AutoCloseable handle | exception | future          |
|      |                                                                     |
|      v                                                                     |
|  OPTION A - JNI ADAPTER                                                    |
|  native methods, JNIEnv, jni crate/generated glue                         |
|                                                                            |
|  OPTION B - FFM API ADAPTER                                                |
|  Linker, MemorySegment, Arena; modern JDK-specific surface                 |
|      |                                                                     |
|      v                                                                     |
|  VERSIONED C ABI -> RUST ADAPTER -> HOST-NEUTRAL RUST CORE                 |
|                                                                            |
|  PACKAGE: JAR/module + per-OS/arch native library + extraction/load policy |
+============================================================================+
```

## Select JNI, FFM, or a Process

| Option | Use when | Cost |
|--------|----------|------|
| JNI | Broad JDK compatibility; existing native methods | Verbose handles, local/global refs, classloader/thread rules |
| Foreign Function and Memory API | Controlled modern JDK baseline; C-shaped native library | JDK-version policy and newer ecosystem tooling |
| JNA/JNR | Lower-volume calls and faster wrapper development | Dynamic mapping overhead and less compile-time checking |
| Process/service | Strong failure, heap, and rollout isolation | Serialization and operations |

The FFM API was finalized in JDK 22 (JEP 454). If a product supports older JDKs,
JNI remains the portable baseline. A migration can publish one Java facade with
different internal adapters, but the supported JDK matrix must be explicit.

## JNI Shape

```rust
use jni::{
    objects::{JByteArray, JClass},
    sys::jlong,
    JNIEnv,
};

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_maxim_Parser_countRecords<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JByteArray<'local>,
) -> jlong {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let bytes = env.convert_byte_array(&input)?;
        Ok::<jlong, jni::errors::Error>(
            bytes.iter().filter(|&&b| b == b'\n').count() as jlong
        )
    }));
    match result {
        Ok(Ok(value)) => value,
        _ => {
            let _ = env.throw_new("java/lang/IllegalStateException", "native failure");
            0
        }
    }
}
```

This example copies a byte array, trading throughput for a simple lifetime.
It corresponds to a Java declaration such as
`public static native long countRecords(byte[] input);`; overloaded native
methods need JNI's long symbol form or explicit `RegisterNatives` registration.
Production code should distinguish ordinary domain errors from panics and JNI
errors rather than collapsing them.

## References, Threads, and Classloaders

JNI local references are valid only within their native frame and have bounded
capacity. A value retained after return needs a global reference, later deleted
through JNI. A native worker thread must attach to the JVM before using JNI and
detach according to the embedding contract. `JNIEnv` is thread-affine; do not
cache and reuse it on another thread.

Class lookup is also a classloader problem. Native threads may not have the same
context loader as the Java call site. Cache a global class/reference or pass a
loader/factory explicitly rather than assuming `FindClass` behaves identically
from every thread.

## FFM Shape

```
  Java Linker.downcallHandle(symbol, descriptor)
          |
          v
  MemorySegment / Arena controls native memory lifetime
          |
          v
  extern "C" Rust export
```

FFM gives Java-side lifetime scopes (`Arena`) and typed layouts for C-compatible
calls. It does not stabilize Rust layout or allocate Rust objects in Java's
arena. Keep opaque Rust handles and provide Rust destructors; use `MemorySegment`
for call-scoped buffers or explicitly shared native memory.

FFM downcalls/upcalls use restricted native-access operations. Configure the
exact JDK launch explicitly, for example
`--enable-native-access=com.example.module` or `ALL-UNNAMED` for classpath code.
Warning/denial policy has evolved across modern JDK releases and launch modes,
so package and test the option with the supported JDK rather than relying on a
developer's permissive default. Modern JDKs also apply native-access policy to
JNI; JNI remains broadly compatible, but it is not configuration-free.

## Errors and Async

| Native event | JVM projection |
|--------------|----------------|
| Domain error | Return status/result data, then throw a documented Java exception |
| Rust panic | Catch/map to terminal native failure; never unwind into JVM |
| Blocking call | Run on a Java executor or native worker; do not block sensitive JVM threads |
| Native completion | Complete `CompletableFuture` through an attached JVM thread |
| Cancellation | Explicit cancel handle/flag; define whether work stops or result is ignored |

Do not hold JNI critical array/string access across blocking work. Such APIs can
restrict GC progress; use them only for short bounded operations.

## Packaging Native Libraries

Common strategies are per-platform classifier artifacts, dedicated platform
JARs, or extraction of a resource to a controlled cache followed by
`System.load` with an absolute path. Name and select by OS, architecture, and
libc where relevant. Avoid writing a shared predictable temporary filename:
concurrent processes, upgrades, permissions, and tampering make that unsafe.
The Java package or launch scripts must also carry the native-access options
required by the exact JNI/FFM/JDK profile.

## Boundary Hazard Register

| Hazard | JVM boundary rule |
|--------|-------------------|
| ABI | JNI/system ABI or FFM-to-C ABI only; never expose Rust ABI, `repr(Rust)`, or trait objects. |
| Allocator | JVM owns Java objects; Rust frees Rust handles/buffers; FFM arenas do not free arbitrary Rust allocations. |
| Panic/unwind | Catch/map Rust panic; Java exceptions remain pending/handled through JVM APIs and never unwind through Rust. |
| Lifetime | Local/global JNI refs, array pins, `MemorySegment` scopes, and opaque handles each need explicit duration. |
| Threading | `JNIEnv` is thread-affine; attach workers, define callback executor/reentrancy, and respect GC-sensitive regions. |
| Target | Test JDK range, JNI/FFM adapter, OS, architecture, libc, and JVM distribution constraints. |
| Packaging | Ship/select/extract native libraries securely; include dependencies, symbols, signing, and classloader behavior. |

## Old World -> New World Bridge

| JVM prior art | Rust interop mapping |
|---------------|----------------------|
| JNI native method | Runtime adapter over a C-shaped Rust operation |
| Direct `ByteBuffer` | Explicit off-heap view with lifetime and position/limit semantics |
| `AutoCloseable` | Managed owner of an opaque Rust handle |
| `CompletableFuture` | Completion projected from native work on a JVM-safe path |
| JAR classifier | Per-target native artifact selection |
| Java interface | Java projection over functions/handles, not a Rust trait-object vtable |

## Common Confusion Points

- **"`JNIEnv` is a global JVM handle."** It is valid for the current attached
  thread and native frame.
- **"A local reference can be cached."** Promote to a global reference and
  release it explicitly.
- **"Direct ByteBuffer means ownership transferred."** It is a view; the native
  memory owner and valid interval remain separate contracts.
- **"FFM makes Rust types callable."** It describes foreign C-compatible
  layouts/calls, not Rust ABI.
- **"FFM/JNI needs no launch policy once the JAR compiles."** Modern JDK native
  access is explicit and version-sensitive; test the packaged launch mode.
- **"Throwing a Java exception returns automatically."** Native code must stop
  normal work and return with the exception pending.
- **"One Linux native library covers Linux."** glibc/musl, architecture, and
  minimum runtime policy can require separate artifacts.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Broad JDK compatibility | JNI with a narrow wrapper |
| Controlled JDK 22+ estate and C-shaped API | FFM API |
| Long-lived native state | `AutoCloseable` wrapper around opaque handle |
| Large call-scoped bytes | Direct buffer/segment with explicit lifetime, or copy |
| Native worker callback | Attach thread, use global refs, marshal to chosen executor |
| Easy rollback and crash isolation | Separate process/service |
| Portable distribution | Platform artifacts plus clean JVM load tests |

## Primary Sources

- Java Native Interface specification: https://docs.oracle.com/en/java/javase/25/docs/specs/jni/
- JEP 454, Foreign Function and Memory API: https://openjdk.org/jeps/454
- Java FFM API: https://docs.oracle.com/en/java/javase/25/core/foreign-function-and-memory-api.html
- Java restricted methods/native access: https://docs.oracle.com/en/java/javase/25/core/restricted-methods.html
- `jni` crate documentation: https://docs.rs/jni/

## Related Guides

- Previous: [07-NODEJS-JAVASCRIPT-INTEROP.md](07-NODEJS-JAVASCRIPT-INTEROP.md)
- Next: [09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md](09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md)
- Thread and callback contracts: [13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md](13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md)
