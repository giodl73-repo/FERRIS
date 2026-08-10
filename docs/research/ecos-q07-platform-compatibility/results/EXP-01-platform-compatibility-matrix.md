# EXP-01: Platform Compatibility Matrix

Date: 2026-08-09
Question: ECOS-Q07
Method: exact package-root cross-target checks, minimal-consumer historical
compiler checks, feature/provider negative controls, representative
cross-target links, and one host execution control
Result: compatibility varied by feature closure, Cargo/rustc pair, target
library capability, provider, architecture, host tool, and validation stage.
No single crate or target label represented the observed outcomes.

## Environment

```text
host: x86_64-pc-windows-msvc
OS: Windows
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
LLVM 22.1.2
rustup 1.29.0
```

Installed current-toolchain targets used:

```text
x86_64-pc-windows-msvc
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
x86_64-apple-darwin
wasm32-unknown-unknown
wasm32-wasip2
thumbv6m-none-eabi
thumbv7em-none-eabihf
riscv32imac-unknown-none-elf
```

Historical toolchains used:

```text
1.31.0  1.32.0  1.36.0  1.56.0  1.57.0
1.61.0  1.64.0  1.65.0  1.71.0  1.85.0
```

The queue is the nineteen exact releases selected by
[ECOS-Q02 EXP-01](../../ecos-q02-foundational-crate-census/results/EXP-01-foundational-crate-census.md).
That result records checksums, release dates, owners, licenses, repositories,
and declared Rust versions.

## Package-root cross-target matrix

Each extracted package root was checked as a library with current Cargo and
rustc. Default features were tested on eight targets. No-default-features was
also tested on unknown WASM and the selected Thumb target.

```powershell
cargo check --quiet --lib --manifest-path <package>\Cargo.toml `
  --target <target>

cargo check --quiet --lib --no-default-features `
  --manifest-path <package>\Cargo.toml --target <target>
```

| Mode | Target | Pass | Fail |
|---|---|---:|---:|
| Default | `x86_64-pc-windows-msvc` | 19 | 0 |
| Default | `x86_64-unknown-linux-gnu` | 19 | 0 |
| Default | `aarch64-unknown-linux-gnu` | 19 | 0 |
| Default | `x86_64-apple-darwin` | 19 | 0 |
| Default | `wasm32-unknown-unknown` | 17 | 2 |
| Default | `wasm32-wasip2` | 18 | 1 |
| Default | `thumbv7em-none-eabihf` | 5 | 14 |
| Default | `riscv32imac-unknown-none-elf` | 5 | 14 |
| No default | `wasm32-unknown-unknown` | 17 | 2 |
| No default | `thumbv7em-none-eabihf` | 11 | 8 |
| **Total** |  | **149** | **41** |

The five default package roots that passed both bare-metal targets were:

```text
cfg-if 1.0.4
hashbrown 0.17.1
libc 0.2.189
log 0.4.33
rand_core 0.10.1
```

The eleven no-default package roots that passed the Thumb target were:

```text
bytes 1.12.1
cfg-if 1.0.4
futures-core 0.3.33
hashbrown 0.17.1
libc 0.2.189
log 0.4.33
memchr 2.8.3
rand_core 0.10.1
serde_core 1.0.229
serde 1.0.229
tracing-core 0.1.36
```

The 41 failures classified as:

| Failure class | Count | Interpretation |
|---|---:|---|
| `std` or a `std`-requiring closure on bare metal, including explicit HTTP no-`std` rejection | 34 | Feature/library capability mismatch |
| Explicit `getrandom` unsupported backend | 5 | Expected unsupported configuration |
| Serde Core package-self lint on WASM | 2 | Package-root validation failure, not consumer compilation failure |

The direct Serde Core WASM failures were:

```text
error: unused imports: `OsStr` and `OsString`
```

Exact minimal consumers then passed both `wasm32-unknown-unknown` and
`wasm32-wasip2`. Package-root and dependency-consumer scopes are therefore
recorded separately.

## Declared Rust-version consumer checks

Each case used a new edition-2018 library with one exact dependency and no use
of the package's tests or dev dependencies.

```powershell
cargo new --lib --edition 2018 <probe>
cargo add --manifest-path <probe>\Cargo.toml <package>@=<version>
rustup run <declared-version> cargo check `
  --manifest-path <probe>\Cargo.toml
```

| Exact dependency | Declared Rust version | Observed toolchain | Default consumer |
|---|---:|---:|---|
| `pkg-config 0.3.33` | 1.31 | 1.31.0 | Pass |
| `cfg-if 1.0.4` | 1.32 | 1.32.0 | Pass |
| `futures-core 0.3.33` | 1.36 | 1.36.0 | Pass |
| `serde 1.0.229` | 1.56 | 1.56.0 | Pass |
| `serde_core 1.0.229` | 1.56 | 1.56.0 | Pass |
| `bytes 1.12.1` | 1.57 | 1.57.0 | Pass |
| `http 1.5.0` | 1.57 | 1.57.0 | Pass |
| `memchr 2.8.3` | 1.61 | 1.61.0 | Pass |
| `cc 1.4.2` | 1.64 | 1.64.0 | Pass |
| `libc 0.2.189` | 1.65 | 1.65.0 | Pass |
| `tracing-core 0.1.36` | 1.65 | 1.65.0 | Pass |
| `proc-macro2 1.0.107` | 1.71 | 1.71.0 | Pass |
| `quote 1.0.47` | 1.71 | 1.71.0 | Pass |
| `syn 3.0.3` | 1.71 | 1.71.0 | Pass |
| `log 0.4.33` | 1.71 | 1.71.0 | Pass |
| `getrandom 0.4.3` | 1.85 | 1.85.0 | Pass |
| `rand_core 0.10.1` | 1.85 | 1.85.0 | Pass |
| `hashbrown 0.17.1` | 1.85 | 1.85.0 | Pass |
| `tower-service 0.3.3` | Not declared | 1.31.0 selected control | Pass |

These passes establish only the observed exact default consumer. No compiler
below a declaration was tested, so they do not prove a minimum.

## Feature, provider, and architecture controls

| Control | Toolchain/target | Result | Meaning |
|---|---|---|---|
| Serde default | Rust 1.56 host | Pass | Top-level default closure met declared version |
| Serde derive exact path closure | Cargo/rustc 1.56 host | Fail before rustc | Cargo 1.56 rejected Syn 3 namespaced `dep:` feature syntax |
| Bytes `extra-platforms` | Rust 1.57 host | Pass | Feature did not raise this observed host compiler requirement |
| Bytes no-default | current `thumbv7em-none-eabihf` | Pass | Selected target has required atomics |
| Bytes no-default | current `thumbv6m-none-eabi` | Fail | Pointer-width atomic CAS methods absent |
| Bytes no-default + `extra-platforms` | current `thumbv6m-none-eabi` | Fail | `portable-atomic` required an explicit CAS provider |
| Bytes plus `portable-atomic/critical-section` | current `thumbv6m-none-eabi` | Pass | Explicit provider satisfied compile-time capability |
| Serde no-default | current `thumbv7em-none-eabihf` | Pass | Feature-controlled no-`std` consumer |
| Tracing Core no-default | current `thumbv7em-none-eabihf` | Pass | Feature-controlled no-`std` consumer |
| Hashbrown no-default | current `thumbv7em-none-eabihf` | Pass | `alloc`-based consumer compiled |
| Proc Macro2 no-default | current `thumbv7em-none-eabihf` | Fail | Direct `extern crate std` |
| Quote no-default | current `thumbv7em-none-eabihf` | Fail | Proc Macro2 closure required `std` |
| Syn no-default | current `thumbv7em-none-eabihf` | Fail | Proc Macro2 closure required `std` |
| Getrandom default | current `wasm32-unknown-unknown` | Expected fail | Environment/backend is ambiguous |
| Getrandom `wasm_js` | current `wasm32-unknown-unknown` | Pass | Explicit JavaScript provider selected |
| Getrandom default | current `wasm32-wasip2` | Pass | WASI backend selected by target |
| Getrandom no-default | current `wasm32-wasip2` | Pass | Default crate features were not the backend gate |
| HTTP no-default | current `wasm32-unknown-unknown` | Expected fail | Crate explicitly requires `std` |
| Tower Service default | current `thumbv7em-none-eabihf` | Expected fail | Direct `std` API |

The Serde derive result is feature-closure and Cargo-client evidence. Syn 3
declares Rust 1.71, and its normalized manifest uses index and feature syntax
that Cargo 1.56 cannot consume. Serde's facade-level default MSRV result must
not be generalized to derive.

The Bytes no-CAS sequence produced three distinct states:

```text
no fallback:
  AtomicUsize::fetch_add/fetch_sub/compare_exchange unavailable

extra-platforms only:
  dependents require atomic CAS but not available on this target by default

extra-platforms + portable-atomic critical-section:
  pass
```

The final pass is compile evidence only. A real critical-section
implementation and its safety assumptions remain application/platform
responsibilities.

## Representative link and execution controls

A minimal binary with exact `cfg-if 1.0.4` was built for six targets.

```powershell
cargo build --manifest-path <probe>\Cargo.toml --target <target>
```

| Target | Check state | Link state | Execution state |
|---|---|---|---|
| `x86_64-pc-windows-msvc` | Pass | Pass | Pass |
| `x86_64-unknown-linux-gnu` | Pass | Fail: linker `cc` absent | Not observed |
| `aarch64-unknown-linux-gnu` | Pass | Fail: linker `cc` absent | Not observed |
| `x86_64-apple-darwin` | Pass | Fail: `cc`, `xcrun`, and Apple SDK absent | Not observed |
| `wasm32-unknown-unknown` | Pass | Pass | Not observed |
| `wasm32-wasip2` | Pass | Pass | Not observed |

No Wasmtime or Wasmer executable was installed. Node was present, but it was
not treated as proof of a matching unknown-WASM or WASIp2 execution contract.

## Build-helper boundary

`cc 1.4.2` describes itself as a build-script library that calls an external
compiler and archiver. Its documentation states that it does not ship a
compiler and identifies target-specific `CC`, `CXX`, `AR`, and flags.

`pkg-config 0.3.33` shells out to `pkg-config`. For cross-compilation it
requires target-specific path/sysroot configuration or explicit permission and
warns that permission without suitable sysroots and search paths is likely to
break builds.

Checking these two Rust libraries did not invoke a downstream native build and
does not establish native portability. That execution boundary remains for
ECOS-Q09.

## Target-tier interpretation

The Rust platform-support documentation states:

- Tier 1 targets are built and tested by the Rust project;
- Tier 2 targets are guaranteed to build, but tests are not always run;
- Tier 2 target entries distinguish full `std`, no-`std` only, and
  work-in-progress support; and
- Tier 3 targets may or may not work and lack official builds.

The measured targets included Tier 1 host-tool targets, Tier 2 full-`std`
targets, and Tier 2 bare-metal no-`std` targets. Tier was retained as upstream
toolchain evidence and was not promoted to crate or application support.

## Raw evidence

Raw JSON and disposable probes were retained outside the repository session
workspace:

```text
ecos-q07-cross-target-results.json
ecos-q07-msrv-results.json
ecos-q07-feature-results.json
ecos-q07-no-std-results.json
ecos-q07-bytes-thumbv6m-results.json
ecos-q07-bytes-thumbv6m-provider-result.json
ecos-q07-link-results.json
ecos-q07-serde-core-consumer-results.json
```

Public conclusions in this result are reproducible from the commands and exact
identities above and do not depend on those local paths.

## Limitations

- One Windows host was used.
- Current package-root checks did not run package tests, examples, or doctests.
- Historical checks covered default consumers and selected features only.
- The current Cargo resolver and historical Cargo clients do not necessarily
  select or parse identical registry closures.
- The package-root matrix is not equivalent to downstream consumer
  compilation.
- Cross-target checks did not prove link, run, ABI, deployment, or operational
  support.
- No browser, worker, Node WASM, or WASI runtime execution was measured.
- No firmware was linked or run on embedded hardware.
- Native compiler, SDK, sysroot, and system-package paths were not installed to
  turn the representative link failures into passes.
- Operating-system minimum versions were not executed.
- The nineteen-release queue is not an approved stack.

## Sources

- Rust platform support:
  <https://doc.rust-lang.org/rustc/platform-support.html>
- Cargo Rust version:
  <https://doc.rust-lang.org/cargo/reference/rust-version.html>
- Cargo resolver:
  <https://doc.rust-lang.org/cargo/reference/resolver.html#rust-version>
- `getrandom 0.4.3`:
  <https://docs.rs/getrandom/0.4.3/getrandom/#webassembly-support>
- `cc 1.4.2`:
  <https://docs.rs/cc/1.4.2/cc/>
- `pkg-config 0.3.33`:
  <https://docs.rs/pkg-config/0.3.33/pkg_config/#cross-compilation>
