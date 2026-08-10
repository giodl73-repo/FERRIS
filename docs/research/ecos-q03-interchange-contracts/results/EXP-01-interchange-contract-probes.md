# EXP-01: Interchange Contract Probes

Date: 2026-08-09
Question: ECOS-Q03
Method: exact-version compile-pass, compile-fail, runtime, and Cargo feature
fixtures
Result: eight expected outcomes observed; compatibility depends on identity,
coherence, effective features, and explicit conversion policy.

## Environment

```text
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
host: Windows
edition: 2024
workspace feature resolver: 3
```

## Exact package identities

| Crate | Version | Released | License | Declared Rust | crates.io checksum |
|---|---:|---:|---|---:|---|
| `http` | 0.2.12 | 2024-03-04 | MIT OR Apache-2.0 | 1.49.0 | `601cbb57e577e2f5ef5be8e7b83f0f63994f25aa94d673e54a92d5c516d101f1` |
| `http` | 1.5.0 | 2026-07-29 | MIT OR Apache-2.0 | 1.57.0 | `918d3568bebf352712bc2ef3d46a8bcf1a75b373be6539de198e9105cbbf9ce0` |
| `rand_core` | 0.6.4 | 2022-09-15 | MIT OR Apache-2.0 | Not declared | `ec0be4795e2f6a28069bec0b5ff3e2ac9bafc99e6a9a7dc3547996c5c816922c` |
| `rand_core` | 0.10.1 | 2026-04-13 | MIT OR Apache-2.0 | 1.85 | `63b8176103e19a2643978565ca18b50549f6101881c443590420e4dc998a3c69` |
| `syn` | 2.0.119 | 2026-07-15 | MIT OR Apache-2.0 | 1.71 | `872831b642d1a07999a962a351ed35b955ea2cfc8f3862091e2a240a84f17297` |
| `syn` | 3.0.3 | 2026-07-22 | MIT OR Apache-2.0 | 1.71 | `53e9bae58849f64dfa4f5d5ae372c8341f7305f82a3868709269343628b659a3` |
| `serde` | 1.0.229 | 2026-07-18 | MIT OR Apache-2.0 | 1.56 | `4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba` |
| `serde_core` | 1.0.229 | 2026-07-18 | MIT OR Apache-2.0 | 1.56 | `67dca2c9c51e58a4791a4b1ed58308b39c64224d349a935ab5039aa360942a48` |
| `thiserror` | 2.0.20 | 2026-08-08 | MIT OR Apache-2.0 | 1.71 | `ec86235f5fcc2a73650310756d2ac5b138a5780bbbdfae3eeccec992c435ba4f` |
| `anyhow` | 1.0.104 | 2026-07-18 | MIT OR Apache-2.0 | 1.68 | `330a5ed07fa54e4702c9d6c4174f74427fc0ef6e214bbd677ae50a5099946470` |

Registry source:
<https://crates.io/api/v1/crates/>.

Current package-level owners and repositories:

| Family | crates.io owners | Repository |
|---|---|---|
| `http` | `carllerche`, `seanmonstar` | <https://github.com/hyperium/http> |
| `rand_core` | `dhardy`, `github:rust-random:maintainers` | <https://github.com/rust-random/rand_core> |
| `syn` | `dtolnay` | <https://github.com/dtolnay/syn> |
| `serde` / `serde_core` | `dtolnay`, `github:serde-rs:publish` | <https://github.com/serde-rs/serde> |
| `thiserror` | `dtolnay` | <https://github.com/dtolnay/thiserror> |
| `anyhow` | `dtolnay` | <https://github.com/dtolnay/anyhow> |

Registry checksums identify the exact published sources. Corresponding
repository revisions were not established.

## Result matrix

| Fixture | Command | Expected | Observed |
|---|---|---:|---:|
| Serde facade/core | `cargo check --quiet --manifest-path pass-serde-core/Cargo.toml` | 0 | 0 |
| Typed/erased error | `cargo run --quiet --manifest-path pass-error-boundary/Cargo.toml` | 0 | 0 |
| Local HTTP wrapper | `cargo run --quiet --manifest-path pass-local-adapter/Cargo.toml` | 0 | 0 |
| HTTP duplicate type | `cargo check --quiet --manifest-path fail-http-version/Cargo.toml` | nonzero | 101, E0308 |
| Rand duplicate trait | `cargo check --quiet --manifest-path fail-rand-core-version/Cargo.toml` | nonzero | 101, E0277 |
| syn duplicate AST | `cargo check --quiet --manifest-path fail-syn-version/Cargo.toml` | nonzero | 101, E0308 |
| Foreign conversion impl | `cargo check --quiet --manifest-path fail-orphan-adapter/Cargo.toml` | nonzero | 101, E0117 |
| Unified feature workspace | `cargo check --quiet --manifest-path feature-unification/app/Cargo.toml` | 0 | 0 |

## Probe A: Serde facade/core identity

```rust
#[derive(serde::Serialize)]
struct Item {
    value: u32,
}

fn accepts_core<T: serde_core::Serialize>(value: &T) -> &T {
    value
}
```

Result: pass. Serde documents that it re-exports the serde_core traits:
<https://docs.rs/serde_core/1.0.229/serde_core/>.

## Probe B: HTTP version identity

```rust
fn accepts_v1(_: http1::Request<()>) {}

let request = http02::Request::new(());
accepts_v1(request);
```

Result: E0308:

```text
expected `http::Request<()>`, found a different `http::Request<()>`
note: there are multiple different versions of crate `http`
```

## Probe C: Rand trait identity

```rust
fn accepts_old<T: rand_core06::RngCore>(_: &mut T) {}

fn bridge<T: rand_core10::RngCore>(rng: &mut T) {
    accepts_old(rng);
}
```

Result: E0277. The 0.10.1 trait bound did not imply the 0.6.4 trait bound.
The 0.10.1 `RngCore` name was also deprecated in favor of its newer trait
organization, increasing the semantic distance beyond package identity.

## Probe D: syn AST identity

```rust
fn accepts_v2(_: syn2::DeriveInput) {}

fn bridge(input: syn3::DeriveInput) {
    accepts_v2(input);
}
```

Result: E0308:

```text
expected `syn::DeriveInput`, found a different `syn::DeriveInput`
note: there are multiple different versions of crate `syn`
```

## Probe E: orphan-rule boundary

```rust
impl From<http02::Request<()>> for http1::Request<()> {
    // ...
}
```

Result: E0117:

```text
only traits defined in the current crate can be implemented for types
defined outside of the crate
note: define and implement a trait or new type instead
```

Rust Reference:
<https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules>.

## Probe F: local wrapper

```rust
struct LocalRequest(http1::Request<()>);

impl From<http02::Request<()>> for LocalRequest {
    // Explicitly reconstruct method, URI, and body.
}
```

Result: pass. The local type satisfies coherence. The fixture intentionally
does not claim complete HTTP metadata preservation.

## Probe G: typed and erased errors

```rust
#[derive(Debug, thiserror::Error)]
enum LibraryError {
    #[error("input was rejected")]
    Rejected,
}

fn application_call() -> anyhow::Result<()> {
    library_call()?;
    Ok(())
}

let error = application_call().unwrap_err();
assert!(error.downcast_ref::<LibraryError>().is_some());
```

Result: pass. Thiserror documents that its derive does not appear in public API,
and anyhow documents `std::error::Error` propagation and downcasting:
<https://docs.rs/thiserror/2.0.20/thiserror/> and
<https://docs.rs/anyhow/1.0.104/anyhow/>.

## Probe H: feature unification

The low branch requested:

```toml
serde = { version = "=1.0.229", default-features = false, features = ["alloc"] }
```

The high branch requested:

```toml
serde = { version = "=1.0.229", features = ["derive"] }
```

Isolated low branch:

```text
serde v1.0.229
└── serde feature "alloc"
```

Combined resolver-3 application:

```text
serde v1.0.229
├── serde feature "alloc"
├── serde feature "default"
├── serde feature "derive"
├── serde feature "serde_derive"
└── serde feature "std"
```

Command:

```text
cargo tree --manifest-path <member>/Cargo.toml -e features -i serde
```

Cargo sources:

- <https://doc.rust-lang.org/cargo/reference/features.html>
- <https://doc.rust-lang.org/cargo/reference/resolver.html>
- <https://doc.rust-lang.org/cargo/commands/cargo-tree.html#feature-unification>

## Source-reviewed semantic contracts

Tower's `Service` contract requires readiness to be preserved across wrappers:
<https://docs.rs/tower-service/0.3.3/tower_service/trait.Service.html>.

The documentation states that services may panic if `call` is invoked without
first obtaining `Poll::Ready(Ok(()))` from `poll_ready`, and demonstrates why
cloning the ready service incorrectly can violate that protocol. This behavior
was source-reviewed, not executed in EXP-01.

The log facade documents globally effective compile-time filtering:
<https://docs.rs/log/0.4.33/log/#compile-time-filters>.

Its `max_level_*` and `release_max_level_*` features remove disabled logging
calls from the binary. The documentation warns libraries not to enable these
features because they are global and immutable after compilation. This
behavior was source-reviewed, not executed in EXP-01.

## Source-reviewed adapter

tracing-log documents both directional conversion and limits:
<https://docs.rs/tracing-log/latest/tracing_log/>.

- log records can become tracing events;
- unstructured log arguments do not become structured tracing fields;
- tracing can emit log records through features; and
- simultaneous conversion in both directions can recurse indefinitely.

This adapter was not executed in EXP-01.

## Limitations

- Fixture sources were disposable and are represented here by the relevant
  excerpts.
- Compile-fail outcomes are exact for the listed toolchain and package releases.
- The local HTTP conversion is deliberately incomplete.
- Feature behavior covers normal dependencies under resolver 3 only.
- No runtime, async I/O, native, or cross-target compatibility was tested.
- No claim is made that all duplicate versions cause an interchange failure.
