# EXP-01: Rust Capability Coverage Matrix

Date: 2026-08-09
Question: ECOS-Q01
Method: primary-source capability inventory
Result: broad capability availability with distributed governance and several
interchange, provider, data, platform, and native-bound gaps.

## Method

The inventory classified capabilities using five evidence classes:

1. Guaranteed by the language, `core`, `alloc`, or `std`.
2. Official Rust project or toolchain component.
3. Available through external crates.
4. Fragmented across contracts or lifecycle.
5. Materially absent or primarily native-bound.

For external capabilities, one or more primary crate sources were inspected
only to establish availability. This experiment did not rank crates or infer
quality from downloads.

## Source inventory

| Domain | Primary sources |
|---|---|
| Rust foundation | <https://doc.rust-lang.org/core/>, <https://doc.rust-lang.org/alloc/>, <https://doc.rust-lang.org/std/> |
| Official workflow | <https://rust-lang.org/tools/>, <https://doc.rust-lang.org/cargo/>, <https://doc.rust-lang.org/rustdoc/> |
| Serialization | <https://serde.rs/> |
| Async | <https://doc.rust-lang.org/std/future/trait.Future.html>, <https://tokio.rs/> |
| HTTP and RPC | <https://hyper.rs/>, <https://docs.rs/reqwest/>, <https://docs.rs/axum/>, <https://docs.rs/tower/>, <https://docs.rs/tonic/> |
| TLS and URL | <https://rustls.dev/>, <https://docs.rs/url/> |
| Time and i18n | <https://docs.rs/chrono/>, <https://docs.rs/time/>, <https://docs.rs/chrono-tz/>, <https://icu4x.unicode.org/> |
| Crypto and secrets | <https://github.com/RustCrypto>, <https://docs.rs/ring/>, <https://docs.rs/keyring/> |
| Data and messaging | <https://docs.rs/sqlx/>, <https://docs.rs/diesel/>, <https://docs.rs/lapin/>, <https://docs.rs/rdkafka/> |
| Operations | <https://docs.rs/clap/>, <https://docs.rs/config/>, <https://docs.rs/tracing/>, <https://docs.rs/opentelemetry/> |
| GUI, GPU and media | <https://wgpu.rs/>, <https://docs.rs/egui/>, <https://docs.rs/iced/>, <https://gtk-rs.org/>, <https://docs.rs/cpal/>, <https://docs.rs/ffmpeg-next/> |
| Assurance | <https://github.com/rust-fuzz/cargo-fuzz>, <https://docs.rs/proptest/>, <https://bheisler.github.io/criterion.rs/book/>, <https://github.com/taiki-e/cargo-llvm-cov> |
| Platform comparisons | <https://learn.microsoft.com/en-us/dotnet/standard/class-library-overview>, <https://pkg.go.dev/std>, <https://docs.oracle.com/en/java/javase/25/docs/api/index.html> |

## Detailed matrix

| Capability | Guaranteed | Official | External examples | Gap disposition |
|---|---|---|---|---|
| Ownership, borrowing, core types | Yes | Compiler | Specialized crates | None at foundation |
| Allocation-backed collections | `alloc`/`std` | Compiler/library | Specialized collections | None at foundation |
| Sync I/O, files, processes | `std` | Compiler/library | Async/specialized I/O | Async is external |
| Threads, locks, atomics, channels | `std` | Compiler/library | Rayon, async synchronization | Scheduling policies external |
| Blocking TCP/UDP | `std` | Compiler/library | Async networking and protocols | Protocol and async layers external |
| Packages, builds, workspaces | No language API | Cargo | Cargo extensions | Official |
| Unit/integration tests and doctests | Test harness/rustdoc | Cargo/rustdoc | Framework and snapshot tools | Official base |
| Formatting and linting | No language API | rustfmt/Clippy | Additional policy tools | Official base |
| Async executor and I/O | `Future` only | No bundled runtime | Tokio and alternatives | Runtime/I/O/cancellation fragmentation |
| Serialization | No | No | Serde and format crates | External governance |
| HTTP and gRPC | No | No | hyper, reqwest, axum, tower, tonic | Runtime/body/provider contracts |
| TLS | No | No | rustls, native TLS approaches | Crypto/root/provider policy |
| URL | No | No | url | External contract |
| Calendar and formatted time | Primitive clocks only | No | chrono, time | Type and formatting split |
| IANA time zones | No | No | chrono-tz and OS-backed approaches | Data update and deployment |
| Locale and Unicode services | UTF-8/scalar foundation | No full platform | ICU4X and focused crates | Data provider and footprint |
| Regex | No | No | regex | External contract |
| Crypto primitives | No general suite | No | RustCrypto, ring | Provider, algorithm and audit policy |
| Credential storage | No | No | keyring/platform services | Platform-native contract |
| SQL and ORM | No | No | sqlx, Diesel and others | Runtime, query and database choices |
| Messaging | No | No | lapin, rdkafka and others | Protocol and native boundaries |
| CLI parsing/configuration | Args/env primitives | No | clap, config and others | External contract |
| Logging/tracing/metrics | No application contract | No | log, tracing, OTel and metrics crates | Facade/exporter composition |
| Identity/OAuth/OIDC | No | No | Protocol/provider crates | Claims/provider/policy fragmentation |
| GPU | No | No | wgpu | Driver/platform capability |
| GUI | No | No | egui, iced, GTK, web wrappers | Toolkit/model fragmentation |
| Audio | No | No | cpal and playback crates | Platform/native dependency |
| Video/media | No | No | FFmpeg/GStreamer bindings | Primarily native-bound |
| Property testing | No | No | proptest and others | External assurance tool |
| Fuzzing | No portable stable default | Project and ecosystem pieces | cargo-fuzz and others | Nightly/platform workflow |
| Benchmarking | Compiler benchmarking separate | rustc-perf for compiler | Criterion and others | Application benchmark contract external |
| Coverage | Compiler instrumentation exists | No one bundled Cargo workflow | cargo-llvm-cov and others | Tool/version/platform boundary |
| `no_std`/embedded/WASM | `core`, optional `alloc`, targets | Toolchain target support | Per-crate support | Separate closure verification |

## Interpretation

The matrix does not support either extreme:

- Rust is not missing most application capabilities.
- Rust does not provide one bundled, uniformly governed application platform.

The evidence supports a distributed-platform model:

```text
language/core/alloc/std
  + official toolchain
  + ecosystem contracts
  + provider and data choices
  + platform and native dependencies
  + stewardship and assurance lifecycle
```

ECOS-Q02 must now determine which external contracts qualify as foundational
and collect exact versions, owners, licenses, closures, and lifecycle evidence.

## Limitations

- No external crate is approved by this matrix.
- No download or reverse-dependency count was treated as proof.
- The matrix is not exhaustive across crates.io.
- Platform claims were not compiled or executed.
- Native package installation and cross-compilation were not tested.
- Assurance and audit status were not evaluated.
