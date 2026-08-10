# EXP-01: Async Portability Probes

Date: 2026-08-09
Question: ECOS-Q04
Method: exact-version compile-pass, compile-fail, runtime-pass, and
expected-panic fixtures
Result: ten expected outcomes observed; runtime coupling varied by API rather
than package name.

## Environment

```text
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
host: Windows
edition: 2024
```

## Exact package identities

| Crate | Version | Released | License | Declared Rust | crates.io checksum |
|---|---:|---:|---|---:|---|
| `tokio` | 1.53.1 | 2026-07-20 | MIT | 1.71 | `202caea871b69668250d242070849eb495be178ed697a3e98aebce5bc81a0bed` |
| `tokio-util` | 0.7.19 | 2026-07-21 | MIT | 1.71 | `494815d09bf52b5548659851081238f0ca39ff638363907596da739561c62c52` |
| `futures` | 0.3.33 | 2026-07-18 | MIT OR Apache-2.0 | 1.71 | `a88cf1f829d945f548cf8fec32c61b1f202b6d93b45848602fc02af4b12ad218` |
| `futures-io` | 0.3.33 | 2026-07-18 | MIT OR Apache-2.0 | 1.36 | `4577ecaa3c4f96589d473f679a71b596316f6641bc350038b962a5daf0085d7a` |
| `futures-executor` | 0.3.33 | 2026-07-18 | MIT OR Apache-2.0 | 1.71 | `6754879cc9f2c66f88c6e5c35344bb0bdb0708b0352b1201815667c7eabc7458` |
| `async-channel` | 2.5.0 | 2025-07-06 | Apache-2.0 OR MIT | 1.60 | `924ed96dd52d1b75e9c1a3e6275715fd320f5f9439fb5a4a11fa51f4221158d2` |
| `hyper` | 1.11.0 | 2026-07-20 | MIT | 1.63 | `d22053281f852e11534f5198498373cbb59295120a20771d90f7ed1897490a72` |
| `hyper-util` | 0.1.20 | 2026-02-02 | MIT | 1.64 | `96547c2556ec9d12fb1578c4eaf448b04993e7fb79cbaad930a656880a6bdfa0` |
| `reqwest` | 0.13.4 | 2026-05-25 | MIT OR Apache-2.0 | 1.85.0 | `219c5811de6525e5416c7d5d53bb656d3afdbc6c5af816e0802bcfa42dbdc1c3` |
| `tonic` | 0.14.6 | 2026-05-07 | MIT | 1.88 | `ac2a5518c70fa84342385732db33fb3f44bc4cc748936eb5833d2df34d6445ef` |
| `sqlx` | 0.9.0 | 2026-05-21 | MIT OR Apache-2.0 | 1.94.0 | `378620ccc25c62c89d8be1c819e76a88d59bdcc3304733330788948e619bfd71` |
| `embassy-executor` | 0.10.0 | 2026-03-20 | MIT OR Apache-2.0 | Not declared | `5d0d3b15c9d7dc4fec1d8cb77112472fb008b3b28c51ad23838d83587a6d2f1e` |

Registry source: <https://crates.io/api/v1/crates/>.

Current owner and repository metadata:

| Family | crates.io owners | Repository |
|---|---|---|
| Tokio | `carllerche`, `Darksonn`, `github:tokio-rs:core` | <https://github.com/tokio-rs/tokio> |
| futures | `alexcrichton`, `cramertj`, `rust-lang-owner`, `taiki-e` | <https://github.com/rust-lang/futures-rs> |
| async-channel | `github:smol-rs:admins` | <https://github.com/smol-rs/async-channel> |
| hyper | `github:hyperium:core`, `seanmonstar` | <https://github.com/hyperium/hyper> |
| hyper-util | `seanmonstar` | <https://github.com/hyperium/hyper-util> |
| reqwest | `seanmonstar` | <https://github.com/seanmonstar/reqwest> |
| tonic | `carllerche`, `github:hyperium:tonic`, `LucioFranco` | <https://github.com/hyperium/tonic> |
| SQLx | `abonander`, `mehcode` | <https://github.com/launchbadge/sqlx> |
| Embassy | `Dirbaio`, `github:embassy-rs:crates-io` | <https://github.com/embassy-rs/embassy> |

Registry checksums identify exact published sources. Corresponding repository
revisions were not established.

## Result matrix

| Fixture | Command | Expected | Observed |
|---|---|---:|---:|
| neutral future | `cargo run --quiet --manifest-path pass-neutral-future/Cargo.toml` | 0 | 0 |
| timer outside runtime | `cargo run --quiet --manifest-path fail-tokio-timer-outside/Cargo.toml` | nonzero | 101, panic |
| spawn outside runtime | `cargo run --quiet --manifest-path fail-tokio-spawn-outside/Cargo.toml` | nonzero | 101, panic |
| I/O trait mismatch | `cargo check --quiet --manifest-path fail-io-trait/Cargo.toml` | nonzero | 101, E0277 |
| I/O compat wrapper | `cargo check --quiet --manifest-path pass-io-compat/Cargo.toml` | 0 | 0 |
| dropped JoinHandle | `cargo run --quiet --manifest-path pass-drop-detaches/Cargo.toml` | 0 | 0 |
| aborted task | `cargo run --quiet --manifest-path pass-abort-cancels/Cargo.toml` | 0 | 0 |
| non-Send spawn | `cargo check --quiet --manifest-path fail-nonsend-spawn/Cargo.toml` | nonzero | 101, Send bound |
| local non-Send spawn | `cargo run --quiet --manifest-path pass-local-spawn/Cargo.toml` | 0 | 0 |
| Tokio primitives on futures-executor | `cargo run --quiet --manifest-path pass-tokio-primitives-on-futures/Cargo.toml` | 0 | 0 |

## Probe A: runtime-neutral future

```rust
async fn answer() -> u32 {
    42
}

assert_eq!(futures::executor::block_on(answer()), 42);
assert_eq!(tokio_runtime.block_on(answer()), 42);
```

Result: pass under both executors.

Standard Future source:
<https://doc.rust-lang.org/std/future/trait.Future.html>.

## Probe B: runtime-context panics

```rust
futures::executor::block_on(async {
    tokio::time::sleep(Duration::from_millis(1)).await;
});
```

```rust
let _ = tokio::spawn(async {});
```

Both exited 101 with:

```text
there is no reactor running, must be called from the context of a Tokio 1.x runtime
```

Primary sources:

- <https://docs.rs/tokio/1.53.1/tokio/time/fn.sleep.html>
- <https://docs.rs/tokio/1.53.1/tokio/task/fn.spawn.html>

## Probe C: I/O trait identity and adapter

Without adapter:

```rust
fn needs_futures_io<T: futures_io::AsyncRead>(_: T) {}
let (stream, _) = tokio::io::duplex(64);
needs_futures_io(stream);
```

Result: E0277:

```text
DuplexStream implements similarly named trait tokio::io::AsyncRead,
but not futures_io::AsyncRead
```

With adapter:

```rust
use tokio_util::compat::TokioAsyncReadCompatExt;
needs_futures_io(stream.compat());
```

Result: pass.

Primary source:
<https://docs.rs/tokio-util/0.7.19/tokio_util/compat/>.

## Probe D: task-handle lifecycle

Detached task:

```rust
let handle = tokio::spawn(async move {
    tx.send(7).unwrap();
});
drop(handle);
assert_eq!(rx.await.unwrap(), 7);
```

Result: pass; the result arrived after the JoinHandle was dropped.

Explicit abort:

```rust
started_rx.await.unwrap();
handle.abort();
let error = handle.await.unwrap_err();
assert!(error.is_cancelled());
```

Result: pass.

Primary source:
<https://docs.rs/tokio/1.53.1/tokio/task/struct.JoinHandle.html>.

## Probe E: Send and local spawning

`tokio::spawn` with `Rc` retained across an await failed:

```text
future cannot be sent between threads safely
the trait Send is not implemented for Rc<i32>
```

The equivalent future passed through `LocalSet::spawn_local`.

Primary source:
<https://docs.rs/tokio/1.53.1/tokio/task/fn.spawn.html>.

## Probe F: selected Tokio primitives on another executor

```rust
tokio::task_local! {
    static NUMBER: u32;
}

futures::executor::block_on(NUMBER.scope(7, async {
    let (tx, rx) = tokio::sync::oneshot::channel();
    tx.send(NUMBER.get()).unwrap();
    assert_eq!(rx.await.unwrap(), 7);
}));
```

Result: pass. This does not prove all Tokio synchronization or context APIs are
runtime-neutral; it disproves package-level classification.

Task-local source:
<https://docs.rs/tokio/1.53.1/tokio/task/struct.LocalKey.html>.

## Source-reviewed contracts

### Cancellation and select

Tokio `select!` cancels non-winning branches by dropping their futures and
requires operation-specific cancellation-safety review:
<https://docs.rs/tokio/1.53.1/tokio/macro.select.html>.

### Blocking work

Started `spawn_blocking` tasks cannot be aborted. Runtime shutdown waits for
them unless a timeout stops waiting, but the work continues:
<https://docs.rs/tokio/1.53.1/tokio/task/fn.spawn_blocking.html>.

### Runtime capability injection

Hyper exposes executor, timer, and I/O runtime traits:
<https://docs.rs/hyper/1.11.0/hyper/rt/>.

hyper-util supplies Tokio wrappers:
<https://docs.rs/hyper-util/0.1.20/hyper_util/rt/tokio/struct.TokioIo.html>.

### Application runtime coupling

- Reqwest async client requires Tokio:
  <https://docs.rs/reqwest/0.13.4/reqwest/>.
- Tonic transport is built on Tokio, hyper, and tower:
  <https://docs.rs/tonic/0.14.6/tonic/>.
- SQLx selects Tokio or async-std through features and context:
  <https://docs.rs/sqlx/0.9.0/sqlx/>.
- Embassy exposes target-specific executor platforms:
  <https://docs.rs/embassy-executor/0.10.0/embassy_executor/>.

## Limitations

- Fixtures were disposable and are represented by relevant excerpts.
- One host and toolchain were measured.
- No alternate desktop runtime was installed.
- I/O adapters were compile-tested only.
- Cancellation-safe and unsafe operations were not exhaustively executed.
- Blocking shutdown was source-reviewed rather than executed.
- No real network, file, process, signal, WASM, embedded, or no_std target was
  exercised.
