# EXP-01: Native Dependency Boundary Matrix

Date: 2026-08-10
Question: ECOS-Q09
Method: exact package and target trees, native compiler and discovery controls,
system/bundled/provider comparisons, external and vendored code generation,
binding generation failure, artifact inventories, and clean reproducibility
controls
Result: portability and reproducibility depended on host and target tools,
system packages, source mode, provider, generated material, Cargo directives,
ABI, and final artifact identity. Bundling and vendoring moved those
responsibilities into the crate closure but did not remove them.

## Environment

```text
host: x86_64-pc-windows-msvc
OS: Windows 11 Enterprise
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
LLVM 22.1.2
```

Initial command lookup:

| Tool | Initial state |
|---|---|
| `cl`, `clang`, `clang-cl`, `gcc`, `g++`, `cc` | Not on `PATH` |
| `pkg-config` | Not on `PATH` |
| `protoc` | Not on `PATH` |
| `nasm` | Not on `PATH` |
| `cmake` | `C:\Program Files\CMake\bin\cmake.exe` |
| `perl` | Git for Windows Perl |
| libclang | No `clang.dll` or `libclang.dll` discovered by bindgen |

The successful C and Rust links identified the Visual Studio tools that
cc-rs/rustc discovered:

```text
MSVC toolset directory: 14.44.35207
cl.exe:   19.44.35228.0
lib.exe:  14.44.35228.0
link.exe: 14.44.35228.0
```

This distinction is important: absence from ordinary command lookup did not
mean the tool was unavailable to cc-rs or rustc.

## Fixture families

| Fixture | Exact primary dependency | Boundary |
|---|---|---|
| cc-native | `cc 1.4.2` | Host compiler/archiver discovery and C archive |
| pkg-config-probe | `pkg-config 0.3.33` | System discovery executable, policy, and cross sysroot |
| sqlite-system | `rusqlite 0.40.2` / `libsqlite3-sys 0.38.2` | Ambient native library |
| sqlite-bundled | same plus `bundled` | Packaged C source and static archive |
| native-tls | `native-tls 0.2.18` | Target-selected Schannel/OpenSSL provider |
| rustls-aws | `rustls 0.23.43` / `aws-lc-rs 1.18.0` | Bundled AWS-LC provider |
| rustls-ring | `rustls 0.23.43` / `ring 0.17.14` | Bundled ring provider |
| prost-external | `prost-build 0.14.4` | Ambient/explicit protoc |
| prost-vendored | plus `protoc-bin-vendored 3.2.0` | Packaged host generator |
| bindgen | `bindgen 0.72.1` | Runtime-loaded libclang |

Every manifest pinned an exact top-level release and generated a lockfile
before execution.

## Result matrix

| Control | Result | Observed state |
|---|---|---|
| cc native build and run | Pass | Visual Studio compiler discovered; program printed `42` |
| cc with `CC_FORCE_DISABLE=1` | Expected fail | cc-rs explicitly disabled |
| pkg-config with no executable | Expected fail | Command not found |
| pkg-config with `Q09NATIVE_NO_PKG_CONFIG=1` | Expected fail | Explicit policy disablement |
| pkg-config cross target without configuration | Expected fail | Required sysroot/path or cross wrapper |
| explicit synthetic pkg-config, host | Pass | Version 1.2.3, library, link path, include path parsed |
| explicit synthetic pkg-config, cross + sysroot | Pass | Discovery process passed; no native link claimed |
| system SQLite run | Expected fail | Linker could not open `sqlite3.lib` |
| bundled SQLite run | Pass | Program reported SQLite `3.53.2` |
| native-tls Windows run | Pass | Schannel closure; connector constructed |
| native-tls Linux cross check | Expected fail | OpenSSL/sysroot/pkg-config not configured |
| Rustls AWS-LC run | Pass | Nine suites; four groups including X25519MLKEM768 |
| Rustls ring run | Pass | Same nine suites; three classical groups |
| Prost without protoc | Expected fail | `protoc` not found |
| Prost with explicit vendored `PROTOC` path | Pass | Generated program printed `generated` |
| Prost with vendored wrapper | Pass | Selected Win32 `libprotoc 31.1` |
| bindgen without libclang | Expected fail | No valid `clang.dll` or `libclang.dll` |

Failures were retained as distinct expected states. None was relabeled as a
crate defect.

## Exact release identity

All listed `.crate` archive hashes matched the crates.io SHA-256 values.
Owners are the crates.io owner snapshot observed on 2026-08-10.

| Release | Published | Package VCS revision | crates.io SHA-256 | License | Owners |
|---|---|---|---|---|---|
| `cc 1.4.2` | 2026-08-08 | `a91e05ec40f26d4637d4bff9e9764221d0a59dd8` | `5d262e149917187838d5b42777c8253bcb64500067342904e7d429499a6f277e` | MIT OR Apache-2.0 | `rust-lang-owner`, `github:rust-lang:libs` |
| `pkg-config 0.3.33` | 2026-04-12 | `f4ac872e02f9e2c111c55f3ed0eee6284d41d50f` | `19f132c84eca552bf34cab8ec81f1c1dcc229b811638f9d283dceabe58c5569e` | MIT OR Apache-2.0 | `sdroege`, `joshtriplett`, `rust-lang-owner` |
| `rusqlite 0.40.2` | 2026-08-08 | `e88f112bef7899234a497baed5cc3c3d553deeb8` | `23f2a97da3e3873c73cb2a2e71b35c40ff95e0b1eefa8d72d8499a6928c3b5b3` | MIT | `thomcc`, `gwenn` |
| `libsqlite3-sys 0.38.2` | 2026-08-08 | `e88f112bef7899234a497baed5cc3c3d553deeb8` | `f1d20bef17f513b9b3004532233187769cd072d790971f4e4da0e346eb6401e8` | MIT | `thomcc`, `gwenn` |
| `native-tls 0.2.18` | 2026-02-18 | `3cf1877ee86814168255db6d73fbefc127211c1c` | `465500e14ea162429d264d44189adc38b199b62b1c21eea9f69e4b73cb03bbf2` | MIT OR Apache-2.0 | `sfackler`, `kornelski` |
| `schannel 0.1.29` | 2026-03-10 | `ff7ffd306152f039cff49f42801a63519068ce0b` | `91c1b7e4904c873ef0710c1f407dde2e6287de2bebc1bbbf7d430bb7cbffd939` | MIT | `sfackler`, `steffengy` |
| `openssl 0.10.81` | 2026-06-12 | `db9c9e2f5db2ad7b45fd894e8d297ee15bfd0c7c` | `77823a27f0babb03091cb9ed9ef80af3b39dbc82f97e8fa530374b7dafd87a45` | Apache-2.0 | `sfackler`, `alex` |
| `openssl-sys 0.9.117` | 2026-06-12 | `db9c9e2f5db2ad7b45fd894e8d297ee15bfd0c7c` | `b47e7e6bb2c38cd930d25a23b40fa52e068c10e85f3e03a7f5ba5aaca5713695` | MIT | `alexcrichton`, `sfackler`, `alex` |
| `rustls 0.23.43` | 2026-07-29 | `fcf61cdbba30913cfd5b40aefa83989c6233812d` | `0283386ce02abc0151e1761d08802dfe86c173b0b494af5cbc086574e453da06` | Apache-2.0 OR ISC OR MIT | `ctz`, `djc`, `github:rustls:publishers` |
| `aws-lc-rs 1.18.0` | 2026-08-07 | `f464440d1fd3983ce9fb023e9eaf1698530919a2` | `ce2b2dcc879c3bae0d371e77c99f2238400ef24ec001394befa67b6e543add9e` | ISC AND (Apache-2.0 OR ISC) | `justsmth`, `skmcgrail`, `crypto-alg`, `github:aws:aws-lc-rs-team` |
| `aws-lc-sys 0.44.0` | 2026-08-07 | `f464440d1fd3983ce9fb023e9eaf1698530919a2` | `f09fae7be8bb3174e05c6afdb34199e6dc0c7c04ba9fa237b1967adfbde27483` | Composite ISC/Apache/MIT/BSD expression | `justsmth`, `skmcgrail`, `crypto-alg`, `github:aws:aws-lc-rs-team` |
| `ring 0.17.14` | 2025-03-11 | `2723abbca9e83347d82b056d5b239c6604f786df` | `a4689e6c2294d81e88dc6261c768b63bc4fcdb852be6d1352498b114f61383b7` | Apache-2.0 AND ISC | `briansmith` |
| `prost 0.14.4` | 2026-06-07 | `13646cde7eab75c81b3047767aa0a86e7dbecf12` | `528ac67416ff8646872a3c02cad9cc4ee5dc9f9540c9b10771855c95cb2e5ae1` | Apache-2.0 | `danburkert`, `github:tokio-rs:prost-core` |
| `prost-build 0.14.4` | 2026-06-07 | `13646cde7eab75c81b3047767aa0a86e7dbecf12` | `03da047801ff44bb6a4d407d4860c05fd70bb81714e6b2f3812603d5b145b042` | Apache-2.0 | `danburkert`, `github:tokio-rs:prost-core` |
| `protoc-bin-vendored 3.2.0` | 2025-07-21 | `895c0433c3727a552970ce961e398e20e52d6353` | `d1c381df33c98266b5f08186583660090a4ffa0889e76c7e9a5e175f645a67fa` | MIT | `stepancheg` |
| `protoc-bin-vendored-win32 3.2.0` | 2025-07-21 | `895c0433c3727a552970ce961e398e20e52d6353` | `95067976aca6421a523e491fce939a3e65249bac4b977adee0ee9771568e8aa3` | MIT | `stepancheg` |
| `bindgen 0.72.1` | 2025-08-31 | `d874de8d646d9b8a3e7ba2db2bcd52f2fba8f1f5` | `993776b509cfb49c750f11b8f07a46fa23e0a1386ffc01fb1e7d343efc387895` | BSD-3-Clause | `emilio`, `pvdrz`, `github:servo:cargo-publish` |
| `clang-sys 1.9.1` | 2026-07-29 | `8844c21292e54818d200980cd90a5498e2a5a644` | `157a8ba7b480713b56f4c09fd13fc3e0a22a5dfab8097ba61cbc5feef950788a` | Apache-2.0 | `KyleMayes`, `madsmtm` |
| `prettyplease 0.2.37` | 2025-08-19 | `c971184fa8c5ef5a2828196e35bd99469455b46b` | `479ca8adacdd7ce8f1fb39ce9ecccbfe93a3f1344b3d0d97f20bc0196208f62b` | MIT OR Apache-2.0 | `dtolnay` |

The table records package identity, not archive-to-repository reproduction or
native-component certification.

## Active tree shapes

Counts include the fixture root and normal/build dependencies for the selected
target.

| Fixture/target | Packages | Build-script packages | Procedural macros | `links` packages |
|---|---:|---:|---:|---|
| cc native / Windows | 4 | 1 | 0 | 0 |
| pkg-config probe / Windows | 2 | 1 | 0 | 0 |
| SQLite system / Windows | 12 | 1 | 0 | `libsqlite3-sys links=sqlite3` |
| SQLite bundled / Windows | 15 | 1 | 0 | `libsqlite3-sys links=sqlite3` |
| native-tls / Windows | 5 | 1 | 0 | 0 |
| native-tls / Linux target | 21 | 6 | 1 | `openssl-sys links=openssl` |
| Rustls AWS-LC / Windows | 19 | 4 | 0 | aws-lc-rs and aws-lc-sys |
| Rustls ring / Windows | 14 | 2 | 0 | ring |
| Prost external / Windows | 32 | 5 | 1 | prettyplease metadata namespace |
| Prost vendored / Windows | 41 | 5 | 1 | prettyplease metadata namespace |
| bindgen / Windows | 25 | 7 | 0 | clang-sys and prettyplease |

Prettyplease's `links = "prettyplease02"` and build script transmit its package
version to immediate dependents; they do not link a native library. This is the
negative control for treating `links` as a native-code flag.

## cc native control

The fixture declared one C function and used:

```rust
cc::Build::new().file("native/add.c").compile("q09_add");
```

Release build and execution:

```powershell
cargo run --release --locked `
  --manifest-path <cc-native>\Cargo.toml `
  --target-dir <target>
```

Result:

```text
42
```

Saved build-script directives included:

```text
cargo:rustc-link-lib=static=q09_add
cargo:rustc-link-search=native=<target>\release\build\<unit>\out
```

Setting:

```powershell
$env:CC_FORCE_DISABLE = 1
```

produced the expected error that cc-rs functionality had been disabled.

This established:

- actual downstream native compilation rather than compiling cc-rs as a Rust
  library;
- Visual Studio discovery outside ordinary `PATH`;
- native archive and link metadata creation; and
- an explicit external-build-system policy boundary.

## pkg-config controls

The fixture requested:

```rust
pkg_config::Config::new()
    .cargo_metadata(false)
    .atleast_version("1.2.3")
    .probe("q09native")
```

Without a command:

```text
Could not run ... pkg-config --libs --cflags q09native
The pkg-config command could not be found.
```

With `Q09NATIVE_NO_PKG_CONFIG=1`:

```text
Aborted because Q09NATIVE_NO_PKG_CONFIG is set
```

For `aarch64-unknown-linux-gnu` without cross configuration:

```text
Install a sysroot for the target platform and configure it via
PKG_CONFIG_SYSROOT_DIR and PKG_CONFIG_PATH, or install a
cross-compiling wrapper for pkg-config and set it via PKG_CONFIG.
```

A synthetic executable was then supplied through `PKG_CONFIG`. It returned:

```text
version=1.2.3
libs=q09native
link-paths="C:\q09\lib"
include-paths="C:\q09\include"
```

It received both `--libs --cflags` and `--modversion` calls. The cross control
also set `PKG_CONFIG_SYSROOT_DIR` and passed.

The positive control proves explicit tool selection and parsed metadata only.
It does not prove those paths exist, a library links, the ABI matches, or a
target program runs.

## System versus bundled SQLite

### System mode

Selected features:

```text
rusqlite: default, cache, hashlink
libsqlite3-sys: default, pkg-config, vcpkg,
  min_sqlite_version_3_34_1
```

Rust compilation reached the final native link. The linker command requested:

```text
sqlite3.lib
```

and failed:

```text
LINK : fatal error LNK1181: cannot open input file 'sqlite3.lib'
```

No version, path, archive hash, system package, patch state, or deployment
identity was observed.

### Bundled mode

Selected additional features:

```text
rusqlite: bundled, modern_sqlite
libsqlite3-sys: bundled, bundled_bindings, cc
```

The archive packaged:

```text
9 packaged native source/header files
20,586,932 packaged native source/header bytes
SQLite amalgamation version 3.53.2
```

The build script emitted:

```text
cargo:include=<registry>\libsqlite3-sys-0.38.2\sqlite3
cargo:rustc-link-lib=static=sqlite3
cargo:rustc-link-search=native=<target>\release\build\<unit>\out
```

Observed release artifacts:

```text
sqlite3 native archive: 4,945,074 bytes in the first control
fixture executable:     1,783,296 bytes
runtime SQLite:         3.53.2
```

Bundling turned missing ambient installation into packaged source, compiler,
archive, FFI, final-link, advisory, and distribution responsibilities.

## Target-selected native TLS

Windows active packages:

```text
native-tls 0.2.18
schannel 0.1.29
windows-sys
windows-targets family
fixture root
```

The connector construction passed:

```text
connector-created
```

The Linux target closure instead selected:

```text
openssl 0.10.81
openssl-sys 0.9.117 links=openssl
pkg-config
cc
vcpkg
procedural-macro support
```

The build script reported:

```text
$HOST = x86_64-pc-windows-msvc
$TARGET = x86_64-unknown-linux-gnu
openssl-sys = 0.9.117
Could not find directory of OpenSSL installation
pkg-config has not been configured to support cross-compilation
```

The same Rust dependency declaration therefore represented materially
different native owners and prerequisites.

## Rustls provider comparison

Both fixtures used:

```text
rustls 0.23.43
default-features = false
features = ["std", "tls12", <provider>]
```

### Capability output

Both providers exposed:

```text
TLS13_AES_256_GCM_SHA384
TLS13_AES_128_GCM_SHA256
TLS13_CHACHA20_POLY1305_SHA256
TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256
TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256
TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384
TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256
```

AWS-LC groups:

```text
X25519
secp256r1
secp384r1
X25519MLKEM768
```

Ring groups:

```text
X25519
secp256r1
secp384r1
```

### Closure and artifact output

| Dimension | AWS-LC | Ring |
|---|---:|---:|
| Active packages | 19 | 14 |
| Build-script packages | 4 | 2 |
| `links` packages | 2 | 1 |
| Unique native archive bytes | 7,383,040 | 777,122 |
| Target-directory bytes | 97,069,200 | 45,738,438 |
| Executable bytes | 1,597,952 | 694,272 |

AWS-LC's observed target tree was 51,330,762 bytes larger and executable
903,680 bytes larger. The native archive set was 6,605,918 bytes larger.

These are exact one-host artifacts, not a universal provider comparison.
AWS-LC exposed one additional measured group, and provider implementations,
algorithms, audit history, platform support, FIPS modes, performance, and
consumer policy differ.

### Packaged native material

| Package | Native source/header files | Source/header bytes | Pregenerated object files | Object bytes |
|---|---:|---:|---:|---:|
| `aws-lc-sys 0.44.0` | 1,807 | 48,565,290 | 26 | 355,393 |
| `ring 0.17.14` | 136 | 4,848,876 | 17 | 497,035 |

aws-lc-sys metadata identified embedded AWS-LC revision:

```text
991e67ff4cf04df4dd89e407f8b920c6936cb56a
```

Its packaged AWS-LC CMake file reported software version `5.5.0`. The crate
used its `prebuilt-nasm` feature because NASM was absent. Ring documentation
and build source similarly describe pregenerated assembly/object use to reduce
Perl/NASM requirements in packaged builds.

## Protobuf generation

The external fixture first failed:

```text
Could not find `protoc`.
```

The same fixture passed when `PROTOC` named the executable packaged by
`protoc-bin-vendored-win32 3.2.0`.

The vendored fixture reported:

```text
protoc-version=libprotoc 31.1
```

and ran:

```text
generated
```

All three successful generation controls emitted:

```text
generated file: q09.rs
bytes: 200
SHA-256:
b1f834171614474a0f6245629c93a86cce8479de83e5f38195a8d027f500feec
```

### Vendored platform closure

The wrapper depended on:

```text
protoc-bin-vendored-linux-aarch_64 3.2.0
protoc-bin-vendored-linux-ppcle_64 3.2.0
protoc-bin-vendored-linux-s390_64 3.2.0
protoc-bin-vendored-linux-x86_32 3.2.0
protoc-bin-vendored-linux-x86_64 3.2.0
protoc-bin-vendored-macos-aarch_64 3.2.0
protoc-bin-vendored-macos-x86_64 3.2.0
protoc-bin-vendored-win32 3.2.0
```

Aggregate:

```text
platform .crate archive bytes: 27,332,618
wrapper .crate bytes:              3,839
all unpacked protoc binaries: 87,847,644
executed binary: Win32 only
```

The wrapper improved ambient-tool availability but widened the lock/download
and binary provenance scope.

## Bindgen control

The fixture asked bindgen to process one local header. Its active closure
included:

```text
bindgen 0.72.1
clang-sys 1.9.1 links=clang
libloading 0.8.9
prettyplease 0.2.37 links=prettyplease02
```

The build failed:

```text
Unable to find libclang:
couldn't find any valid shared libraries matching
['clang.dll', 'libclang.dll']
```

The Rust packages compiled before the host generator attempted to load
libclang. No generated binding was produced.

## Reproducibility controls

### Different target directories

The cc and bundled SQLite fixtures were built in two clean target directories.
Native archive sizes and hashes differed; executable hashes also differed.

### Same target directory, clean rebuild

Each initial artifact was copied, its exact target directory was removed, and
the fixture was rebuilt at the same path. The comparison was repeated without
diagnostic environment variables.

| Fixture | Native archive size equal | Native archive SHA-256 equal | Executable SHA-256 equal | Runtime semantics |
|---|---:|---:|---:|---|
| cc native | Yes | No | No | `42` |
| SQLite bundled | Yes | No | No | SQLite `3.53.2` |

The controls do not establish the differing byte cause. Candidate inputs
include COFF archive metadata, compiler/linker timestamps or identifiers,
debug/PDB material, absolute paths, and other toolchain state. The result is
only that exact Cargo packages and source did not yield bit-identical native
archives or executables in these clean MSVC controls.

### Generated output

Prost generated Rust remained byte-identical across:

- explicit external `PROTOC`;
- the vendored wrapper;
- two clean target directories.

Generated-source reproducibility therefore did not imply final executable
reproducibility, and native non-reproducibility did not imply generator output
changed.

## Cargo directives and `links`

Observed native directives:

```text
cc fixture:
  rustc-link-lib=static=q09_add
  rustc-link-search=native=<out>

SQLite bundled:
  rustc-link-lib=static=sqlite3
  rustc-link-search=native=<out>
  include=<packaged sqlite source>

AWS-LC:
  rustc-link-lib=static=aws_lc_0_44_0_crypto
  rustc-link-search=native=<out>
  include=<generated include>
  libdir=<out>

ring:
  rustc-link-lib=static=ring_core_0_17_14_
  rustc-link-search=native=<out>
```

Cargo `links` packages observed:

```text
libsqlite3-sys -> sqlite3
openssl-sys -> openssl
aws-lc-sys -> aws_lc_0_44_0
aws-lc-rs -> aws_lc_rs_1_18_0_sys
ring -> ring_core_0_17_14_
clang-sys -> clang
prettyplease -> prettyplease02
```

The prettyplease control confirms that `links` can reserve a metadata
namespace without native linking. The cc fixture confirms that a build script
can compile and link native code without its package declaring `links`.

## Assurance observations

Representative lockfiles:

```text
sqlite-bundled
native-tls
rustls-aws
prost-vendored
bindgen
```

Command:

```powershell
cargo audit --file <Cargo.lock> --json --no-fetch
```

Tool:

```text
cargo-audit 0.22.2
```

Result:

```text
five lockfiles
zero reported vulnerabilities
zero reported warnings
```

This is dated RustSec package matching. It does not establish:

- the patch state of a system OpenSSL or Schannel installation;
- complete AWS-LC, ring, SQLite, protoc, or libclang native advisory coverage;
- generated-binding safety;
- ABI compatibility;
- deployment inventory; or
- license compatibility.

## Reproduction command shape

```powershell
cargo generate-lockfile --manifest-path <fixture>\Cargo.toml

cargo run --release --locked `
  --manifest-path <fixture>\Cargo.toml `
  --target-dir <fresh-target>

cargo check --locked --target <target> `
  --manifest-path <fixture>\Cargo.toml `
  --target-dir <fresh-target>

cargo tree --locked --target <target> `
  -e normal,build --manifest-path <fixture>\Cargo.toml
```

Target directories were isolated by case. The reproducibility controls removed
only their exact target directories.

## Raw evidence

Disposable fixtures, lockfiles, command output, active trees, release metadata,
archive checks, artifact inventories, generated hashes, and audit output were
retained outside the repository session workspace. Public conclusions are
reproducible from the commands and exact identities above and do not depend on
private paths.

## Limitations

- One Windows host and one MSVC toolset were measured.
- Linux native-tls was a cross check, not a complete build or run.
- No ambient positive system SQLite, OpenSSL, pkg-config, protoc, Clang, or
  libclang installation was available.
- The pkg-config positive executable was synthetic and did not link a library.
- No TLS handshake, certificate-store behavior, FIPS validation, throughput,
  or latency benchmark was run.
- Rustls builds used one release profile and one cold observation each.
- Artifact comparison did not identify the exact differing fields.
- No deterministic-build flags, path remapping, source-date epoch, signing, or
  package reproduction framework was tested.
- Vendored protoc binaries for non-host platforms were inventoried but not
  executed.
- Native advisory and license compatibility were not exhaustively adjudicated.
- No held-out production repository was measured.

## Sources

- Cargo build scripts:
  <https://doc.rust-lang.org/cargo/reference/build-scripts.html>
- Cargo target `links` overrides:
  <https://doc.rust-lang.org/cargo/reference/config.html#targettriplelinks>
- cc 1.4.2:
  <https://docs.rs/cc/1.4.2/cc/>
- pkg-config 0.3.33:
  <https://docs.rs/pkg-config/0.3.33/pkg_config/>
- native-tls 0.2.18:
  <https://docs.rs/native-tls/0.2.18/native_tls/>
- Rustls 0.23.43 providers:
  <https://docs.rs/rustls/0.23.43/rustls/crypto/struct.CryptoProvider.html>
- AWS-LC Rust bindings:
  <https://github.com/aws/aws-lc-rs/blob/main/aws-lc-sys/README.md>
- ring:
  <https://github.com/briansmith/ring>
- rusqlite and libsqlite3-sys:
  <https://github.com/rusqlite/rusqlite>
- Prost build:
  <https://docs.rs/prost-build/0.14.4/prost_build/>
- protoc-bin-vendored:
  <https://docs.rs/protoc-bin-vendored/3.2.0/protoc_bin_vendored/>
- bindgen requirements:
  <https://rust-lang.github.io/rust-bindgen/requirements.html>
- RustSec:
  <https://rustsec.org/>
