# EXP-01: Compatibility Stack Profile Matrix

Date: 2026-08-10
Question: ECOS-Q11
Environment: Windows 11 Enterprise, x86-64
Decision input: determine whether exact application-stack fixtures can produce
renewable compatibility evidence with explicit target, compiler, lifecycle,
removal, and rollback boundaries.

## Scope

This experiment measures:

1. six independent exact-release application profiles;
2. one representative operation per profile;
3. lockfile universe and target-active normal/build closure;
4. build-script, procedural-macro, Cargo `links`, storage, and artifact
   observations;
5. host, Linux, WASM, Thumb, and RISC-V stage outcomes;
6. exact compiler-floor controls;
7. direct-release archive, revision, owner, license, and date provenance;
8. dated RustSec advisory queries; and
9. one dependency renewal and exact rollback control.

It does not establish production deployment, universal compatibility,
security, performance, or maintenance guarantees.

## Tool identity

```text
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
LLVM 22.1.2
rustup 1.29.0 (28d1352db 2026-03-05)
cargo-audit 0.22.2
host x86_64-pc-windows-msvc
```

Compiler controls:

```text
Rust 1.80.0
Rust 1.85.0
Rust 1.87.0
```

Installed cross targets used by the matrix:

```text
x86_64-unknown-linux-gnu
wasm32-unknown-unknown
thumbv7em-none-eabihf
riscv32imac-unknown-none-elf
```

## Fixture definitions

Fixtures were disposable and stored outside the repository. Each used edition
2021, `publish = false`, exact direct releases, a dedicated lockfile, and
isolated target directories.

### Hosted server

```toml
[dependencies]
axum = { version = "=0.8.9", features = ["json"] }
serde = { version = "=1.0.229", features = ["derive"] }
serde_json = "=1.0.151"
tokio = { version = "=1.53.1", features = ["macros", "rt-multi-thread"] }
tower = { version = "=0.5.3", features = ["util"] }
tracing = "=0.1.44"
```

Operation:

```text
Axum /health route
  -> Tower in-process request
  -> collect body
  -> deserialize {"status":"ok"}
  -> print server=ok
```

### CLI and configuration

```toml
[dependencies]
clap = { version = "=4.6.6", features = ["derive"] }
serde = { version = "=1.0.229", features = ["derive"] }
toml = "=0.9.8"
tracing = "=0.1.44"
tracing-subscriber = { version = "=0.3.23", features = ["fmt"] }
```

Operation:

```text
--config <inline TOML>
  -> Clap parsing
  -> TOML decoding
  -> tracing subscriber initialization
  -> cli=ferrium workers=4
```

### Pure-Rust data

```toml
[dependencies]
csv = "=1.4.0"
jiff = "=0.2.35"
serde = { version = "=1.0.229", features = ["derive"] }
serde_json = "=1.0.151"
uuid = { version = "=1.24.0", features = ["serde"] }
```

Operation:

```text
CSV row
  -> Serde row decoding
  -> UUID parsing
  -> Jiff timestamp parsing
  -> JSON serialization
```

### Embedded `no_std`

```toml
[lib]
crate-type = ["rlib"]

[dependencies]
heapless = { version = "=0.9.3", default-features = false }
postcard = { version = "=1.1.3", default-features = false }
serde = { version = "=1.0.229", default-features = false, features = ["derive"] }
```

The library declared `#![no_std]`, retained one `u16` in a
`heapless::Vec<u16, 8>`, and serialized a two-field reading with Postcard. A
host unit test checked non-empty encoding and fixed-capacity window behavior.

### Browser WASM

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
getrandom = { version = "=0.4.3", features = ["wasm_js"] }
serde = { version = "=1.0.229", features = ["derive"] }
serde-wasm-bindgen = "=0.6.5"
wasm-bindgen = "=0.2.127"
```

The library converted a serializable value to `JsValue` and deliberately used
`compile_error!` for non-`wasm32` targets.

### Bundled-native SQLite

```toml
[dependencies]
rusqlite = { version = "=0.40.2", features = ["bundled"] }
serde = { version = "=1.0.229", features = ["derive"] }
serde_json = "=1.0.151"
```

Operation:

```text
open in-memory SQLite
  -> create and insert row
  -> query row
  -> serialize {"name":"ferrium","value":42}
```

## Command shape

Each primary target used:

```text
cargo generate-lockfile --manifest-path <fixture>/Cargo.toml
cargo tree --locked --target <target> \
  -e normal,build --prefix none --format "{p}" \
  --manifest-path <fixture>/Cargo.toml
cargo metadata --format-version 1 --locked \
  --filter-platform <target> \
  --manifest-path <fixture>/Cargo.toml
cargo <run|build|check|test> --release --locked \
  --target <target-when-crossing> \
  --target-dir <isolated-target> \
  --manifest-path <fixture>/Cargo.toml
```

`cargo tree` normal/build edges defined the active target closure. Counting all
packages returned by unfiltered metadata would have included target-inactive
packages. The lockfile package count was retained separately.

## Primary results

| Profile | Target | Result | Lock | Active | Build scripts | Proc macros | `links` | Target root | Artifact |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| server | Windows MSVC | run 0 | 55 | 53 | 7 | 3 | 0 | 175.3 MiB | 750,080-byte EXE |
| CLI | Windows MSVC | run 0 | 51 | 47 | 4 | 3 | 0 | 121.5 MiB | 1,000,960-byte EXE |
| data | Windows MSVC | run 0 | 46 | 21 | 6 | 1 | 0 | 81.1 MiB | 374,784-byte EXE |
| embedded | Thumb | check 0; Rust 1.87 build 0 | 16 | 16 | 6 | 2 | 0 | 44.8 MiB check root | `rlib` at floor control |
| embedded | RISC-V | check 0; Rust 1.87 build 0 | 16 | 16 | 6 | 2 | 0 | 44.7 MiB check root | `rlib` at floor control |
| WASM | `wasm32-unknown-unknown` | build 0 | 27 | 25 | 8 | 3 | 1 | 128.8 MiB | 527,921-byte WASM |
| native | Windows MSVC | run 0 | 41 | 26 | 7 | 1 | 1 | 107.9 MiB | 1,796,608-byte EXE |

Observed output:

```text
server=ok
cli=ferrium workers=4
{"id":"550e8400-e29b-41d4-a716-446655440000","timestamp":"2026-08-10T16:00:00Z","value":42}
{"name":"ferrium","value":42}
```

The embedded host control passed one unit test. The native target contained a
4,945,068-byte `sqlite3.lib`/`libsqlite3.a` archive.

Observed artifact SHA-256 values:

| Profile | SHA-256 |
|---|---|
| server | `408cfb5ddc59dd86d7a5524d19cf79c61617f3b5b2e381cbf2059233f6ba5a41` |
| CLI | `dd93be51ac83c9d3fd8b38cb473819eb7ef2fa9e7a1179e3f2883711154b14f1` |
| data | `ec14335b47cff1d5636949a1bfa2280aa0ddbef6a9d7d8a5aa434a7ddc25bce6` |
| WASM | `1da2241e5a90e631b4dfb88eea676caee976b77ab66f60af5aeaeb3ee0a3267c` |
| native | `c3fe45e69fdf7204ff67f1cb322a68e01f96e24b309605f4696c4bbc0f0512de` |

Artifact identity is not used as a reproducibility claim. All measured
artifacts were disposable.

## Target matrix

### Hosted server

```text
x86_64-pc-windows-msvc: release execution passed
x86_64-unknown-linux-gnu: release check passed
wasm32-unknown-unknown: check failed in mio with 48 errors
```

Representative WASM diagnostics included unavailable `mio` I/O registration
operations. This is an expected boundary for the selected host-network stack.

### CLI/configuration

```text
x86_64-pc-windows-msvc: release execution passed
x86_64-unknown-linux-gnu: release check passed
wasm32-unknown-unknown: release check passed
```

No WASM execution was attempted. Argument acquisition, terminal behavior,
tracing output, JavaScript bindings, and browser deployment were not defined.

### Pure-Rust data

```text
x86_64-pc-windows-msvc: release execution passed
x86_64-unknown-linux-gnu: release check passed
wasm32-unknown-unknown: release check passed
```

The WASM graph selected Jiff's WASM-relevant time-zone packages. Runtime
time-zone/data-source behavior was not executed.

### Embedded `no_std`

```text
host unit test: 1 passed
thumbv7em-none-eabihf: release check and build passed
riscv32imac-unknown-none-elf: release check and build passed
```

No hardware, emulator, allocator exhaustion, panic, interrupt, transport, or
power behavior was observed.

### Browser WASM

```text
wasm32-unknown-unknown: release build passed
x86_64-pc-windows-msvc: deliberate compile_error, exit 101
```

Available execution tools:

```text
node: present
wasm-bindgen CLI: absent
wasmtime: absent
wasm-tools: absent
```

The raw module was built, but JavaScript glue and runtime execution were not
observed.

### Bundled-native SQLite

```text
x86_64-pc-windows-msvc: release execution passed
x86_64-unknown-linux-gnu: cross-build failed before link
```

The cross-build failure was:

```text
cc-rs: failed to find tool "x86_64-linux-gnu-gcc"
```

This records a missing cross C compiler. It does not establish native Linux
incompatibility.

## Compiler-floor controls

The selected candidate floors came from the highest declared `rust-version`
within each active graph. Packages missing that field remained explicit.

| Profile | Highest declaration | Active packages without declaration | Exact control |
|---|---:|---:|---|
| server | 1.80 | 9 | Rust 1.80 Windows run passed |
| CLI | 1.85 | 4 | Rust 1.85 Windows run passed |
| data | 1.85.0 | 2 | Rust 1.85 Windows run passed |
| embedded | 1.87 | 5 | Rust 1.87 Thumb and RISC-V builds passed |
| WASM | 1.85 | 2 | Rust 1.85 WASM build passed |
| native | 1.85.0 | 7 | Rust 1.85 Windows run passed |

Commands:

```text
cargo +1.80.0 run --release --locked <server>
cargo +1.85.0 run --release --locked <cli>
cargo +1.85.0 run --release --locked <data>
cargo +1.85.0 build --release --locked \
  --target wasm32-unknown-unknown <wasm>
cargo +1.85.0 run --release --locked <native>
cargo +1.87.0 build --release --locked \
  --target <thumb-or-riscv> <embedded>
```

## Exact direct-release provenance

All archive hashes matched the crates.io registry checksum. `Revision` is the
first twelve characters of the packaged `.cargo_vcs_info.json` Git SHA.
Owners are the current crates.io user/team snapshot.

| Crate | Release | Published | Rust | License | Owners | Revision | Hash match |
|---|---:|---|---:|---|---|---|---|
| `axum` | 0.8.9 | 2026-04-14 | 1.80 | MIT | carllerche, davidpdrsn, tokio-rs teams | `c59208c86fde` | yes |
| `tokio` | 1.53.1 | 2026-07-20 | 1.71 | MIT | carllerche, Darksonn, tokio-rs core | `75fef53d0a85` | yes |
| `tower` | 0.5.3 | 2026-01-12 | 1.64.0 | MIT | carllerche, seanmonstar, tower-rs publish | `4b0a6b0e688b` | yes |
| `serde` | 1.0.229 | 2026-07-18 | 1.56 | MIT OR Apache-2.0 | dtolnay, serde-rs publish | `7fc3b4c30c94` | yes |
| `serde_json` | 1.0.151 | 2026-07-20 | 1.71 | MIT OR Apache-2.0 | dtolnay, serde-rs publish | `de8500740cdc` | yes |
| `tracing` | 0.1.44 | 2025-12-18 | 1.65.0 | MIT | carllerche, hawkw, tokio-rs publish | `2d55f6faf9be` | yes |
| `tracing-subscriber` | 0.3.23 | 2026-03-13 | 1.65.0 | MIT | hawkw, davidbarsky, tokio-rs publish | `54ede4d5d85a` | yes |
| `clap` | 4.6.6 | 2026-08-06 | 1.85 | MIT OR Apache-2.0 | kbknapp, rust-cli, clap-rs teams | `4a622b4340d5` | yes |
| `toml` | 0.9.8 | 2025-10-09 | 1.76 | MIT OR Apache-2.0 | ehuss, epage, toml-rs team | `93e9146aea8d` | yes |
| `csv` | 1.4.0 | 2025-10-17 | 1.73 | Unlicense/MIT | BurntSushi | `4a3997e91d66` | yes |
| `jiff` | 0.2.35 | 2026-07-25 | 1.70 | Unlicense OR MIT | BurntSushi | `2584cff71dae` | yes |
| `uuid` | 1.24.0 | 2026-07-15 | 1.85.0 | Apache-2.0 OR MIT | KodrAus, rust-lang-nursery libs | `6a8aeab3d028` | yes |
| `heapless` | 0.9.3 | 2026-04-30 | 1.87 | MIT OR Apache-2.0 | japaric, adamgreig, korken89, rust-embedded libs | `ca95c4beaf73` | yes |
| `postcard` | 1.1.3 | 2025-07-24 | not declared | MIT OR Apache-2.0 | jamesmunns | `718aa6a68504` | yes |
| `wasm-bindgen` | 0.2.127 | 2026-08-08 | 1.77 | MIT OR Apache-2.0 | RReverser, daxpedda, guybedford, publish team | `a579ee62b631` | yes |
| `serde-wasm-bindgen` | 0.6.5 | 2024-02-27 | not declared | MIT | RReverser | `f073bd40ee5a` | yes |
| `getrandom` | 0.4.3 | 2026-06-17 | 1.85 | MIT OR Apache-2.0 | dhardy, rust-random maintainers | `5e7cd5733536` | yes |
| `rusqlite` | 0.40.2 | 2026-08-08 | not declared | MIT | thomcc, gwenn | `e88f112bef78` | yes |

Provenance command shape:

```text
GET https://crates.io/api/v1/crates/<crate>/owners
SHA-256 <cargo-registry-cache>/<crate>-<version>.crate
tar -xOf <archive> <crate>-<version>/.cargo_vcs_info.json
```

## Advisory control

Command:

```text
cargo audit fetch
cargo audit --no-fetch --json -f <profile>/Cargo.lock
```

Results under cargo-audit 0.22.2 on 2026-08-10:

| Lock | Exit | RustSec vulnerability matches |
|---|---:|---:|
| server | 0 | 0 |
| CLI | 0 | 0 |
| data | 0 | 0 |
| embedded | 0 | 0 |
| WASM | 0 | 0 |
| native | 0 | 0 |
| renewal baseline | 0 | 0 |

No conclusion was drawn about undisclosed vulnerabilities, unsafe-code review,
malicious behavior, future advisories, or non-Rust dependencies.

## Renewal control

The renewal fixture used:

```toml
[dependencies]
clap = { version = "4.6", features = ["derive"] }
```

Procedure:

```text
cargo update -p clap --precise 4.6.5
copy Cargo.lock <baseline-lock>
cargo run --release --locked -- --name baseline
cargo update -p clap
copy Cargo.lock <updated-lock>
cargo run --release --locked -- --name updated
restore <baseline-lock> Cargo.lock
cargo run --release --locked -- --name rollback
```

Update result:

```text
clap 4.6.5 -> 4.6.6
clap_builder 4.6.5 -> 4.6.6
active package count: 22 -> 22
baseline run: passed
updated run: passed
rollback run: passed
```

Lock hashes:

```text
baseline 4b461f4034175df7e2cd637b81bcc287bb52c3b64c28cbdc0fefdf1c7a2580ce
updated  e5fdc06ebd5553de90185639f09e39402d8ede24583841efcb9eead55065fc74
restored 4b461f4034175df7e2cd637b81bcc287bb52c3b64c28cbdc0fefdf1c7a2580ce
```

The exact rollback demonstrates control of lock selection. It does not cover
data, wire, ABI, deployment, or production-state rollback.

## Required profile record

The evidence suggests the following minimum record:

```text
profile_id
profile_revision
consumer_requirements
capability_boundary
direct_releases_and_features
lock_identity
target_active_closure
compiler_and_cargo
host_and_target
native_and_runtime_prerequisites
validation_stage_results
provenance_and_advisory_snapshot
alternatives
owner
observed_at
expires_at
renewal_triggers
removal_plan
rollback_identity_and_validation
limitations
```

## Limitations

- One Windows host supplied all execution evidence.
- Linux was cross-checked rather than run natively.
- The browser WASM module was not processed by the wasm-bindgen CLI or
  executed.
- Embedded targets were not executed on hardware or an emulator.
- The hosted server did not bind a real network socket.
- Target-directory byte counts are compiler/cache artifacts, not shipped size
  or benchmark results.
- Build timing was excluded because concurrent package-cache waits were
  present.
- Artifact hashes do not establish reproducible builds.
- Direct provenance does not replace transitive assurance review.
- All owner, advisory, release, and registry observations expire.
