# EXP-01: Security and Provenance Census

Date: 2026-08-09
Question: ECOS-Q06
Method: exact archive verification, package VCS resolution, trusted-publication
comparison, Cargo closure inspection, advisory matching, syntax census,
build-script review, and license inventory
Result: the nineteen selected releases had matching registry archive hashes,
resolvable package VCS revisions, no matching records in the observed RustSec
and OSV queries, six active build scripts, feature-selected procedural-macro
execution, and materially different direct unsafe-code surfaces. None of these
observations independently establishes safety.

## Environment

```text
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
cargo-audit 0.22.2
host: Windows
RustSec advisory-db: 565436d86a136c840d01ad4a7851fc7391295404
```

The queue is the nineteen exact releases selected by
[ECOS-Q02 EXP-01](../../ecos-q02-foundational-crate-census/results/EXP-01-foundational-crate-census.md).
That result records every exact crates.io checksum, release date, declared Rust
version, owner set, and repository. The
[ECOS-Q05 result](../../ecos-q05-maintenance-stewardship/results/EXP-01-stewardship-census.md)
records exact publication authority and the three trusted-publishing releases.

## Package integrity and source revision

For each release, the cached `.crate` archive was SHA-256 hashed, compared with
the crates.io checksum, extracted, and inspected for `.cargo_vcs_info.json`.
The named commit was then resolved in the canonical repository.

```powershell
Get-FileHash -Algorithm SHA256 <package>.crate
git -C <canonical-repository> cat-file -e <vcs-sha>^{commit}
```

Cargo documents `.cargo_vcs_info.json` as a best-effort VCS snapshot. It
explicitly states that package provenance is not verified and that the tarball
is not guaranteed to match the named VCS revision:
<https://doc.rust-lang.org/cargo/commands/cargo-package.html#cargo_vcs_infojson-format>.

| Exact release | Archive hash matched registry | Package VCS revision | Dirty |
|---|---|---|---|
| `libc 0.2.189` | Yes | `ef0906e20828777175f65caa7e681a0ce33c559a` | Not reported |
| `cfg-if 1.0.4` | Yes | `3510ca6abea34cbbc702509a4e50ea9709925eda` | Not reported |
| `getrandom 0.4.3` | Yes | `5e7cd5733536844a9856dc7259bd4696bbe5e3ae` | Not reported |
| `proc-macro2 1.0.107` | Yes | `ed8a5497669cd63db33bf24646f261b012bbbc4a` | Not reported |
| `quote 1.0.47` | Yes | `723dcb47d3f0ddc896e17287c8a8d3f2ea2317d5` | Not reported |
| `syn 3.0.3` | Yes | `23dbaab4b0c43f56cd803894054cf366661e53b0` | Not reported |
| `serde_core 1.0.229` | Yes | `7fc3b4c30c94f73a96ebd1553f2b090d928fc3a8` | Not reported |
| `serde 1.0.229` | Yes | `7fc3b4c30c94f73a96ebd1553f2b090d928fc3a8` | Not reported |
| `log 0.4.33` | Yes | `f405739f3a15a3f00680c793e1e1fa7e57d26ba4` | Not reported |
| `tracing-core 0.1.36` | Yes | `10a9e838a35e6ded79d66af246be2ee05417136d` | Not reported |
| `bytes 1.12.1` | Yes | `76c0fbb54ed4336caf9d2311658a2f4a5627c21d` | Not reported |
| `http 1.5.0` | Yes | `e559023f67e3fad6ecc3ee91307be178e0f13626` | **Yes** |
| `tower-service 0.3.3` | Yes | `646804d77eebf072dac180cb5e1256b9ee7e0229` | Not reported |
| `futures-core 0.3.33` | Yes | `89cc254cb8bfcc78e6a31e7711d0757f97bcb531` | Not reported |
| `rand_core 0.10.1` | Yes | `5cd7df496d0bff1d95534ac8b66a2610f5bf7808` | Not reported |
| `cc 1.4.2` | Yes | `a91e05ec40f26d4637d4bff9e9764221d0a59dd8` | Not reported |
| `pkg-config 0.3.33` | Yes | `f4ac872e02f9e2c111c55f3ed0eee6284d41d50f` | Not reported |
| `hashbrown 0.17.1` | Yes | `c62a63a61b7caf2de8f9ecb7b06a66b0ab6bdf3d` | Not reported |
| `memchr 2.8.3` | Yes | `5fdb40c054e1fff359a2f7bdf7f87a13b34b465d` | Not reported |

All nineteen revisions resolved. `http 1.5.0` was packaged from a dirty
worktree, so its archive cannot be reconstructed from the named commit alone.

For the three selected trusted-publishing releases, crates.io's publication
commit equaled the checksum-covered package VCS revision:

| Release | Trusted repository commit | Package VCS revision | Equal |
|---|---|---|---|
| `getrandom 0.4.3` | `5e7cd5733536844a9856dc7259bd4696bbe5e3ae` | same | Yes |
| `rand_core 0.10.1` | `5cd7df496d0bff1d95534ac8b66a2610f5bf7808` | same | Yes |
| `cc 1.4.2` | `a91e05ec40f26d4637d4bff9e9764221d0a59dd8` | same | Yes |

The agreement joins two release metadata sources. It does not prove that the
workflow reviewed, reproduced, or safely built the package.

## Dependency and executable closure

The exact queue was resolved in one disposable Cargo fixture. The root feature
`derive = ["serde/derive"]` provided a controlled feature-selected compiler
execution case.

```powershell
cargo metadata --format-version 1 --manifest-path <probe>\Cargo.toml
cargo metadata --format-version 1 --features derive --manifest-path <probe>\Cargo.toml
```

| Profile | Active packages | Build scripts | Procedural macros | Cargo `links` packages |
|---|---:|---:|---:|---:|
| Default Windows closure | 28 | 6 | 0 | 0 |
| Serde derive closure | 29 | 6 | 1 | 0 |

The derive profile added `serde_derive 1.0.229`. The default closure added
`allocator-api2 0.2.21`, `equivalent 1.0.2`, `find-msvc-tools 0.1.10`,
`foldhash 0.2.0`, `itoa 1.0.18`, `once_cell 1.21.4`, `shlex 2.0.1`, and
`unicode-ident 1.0.24` beyond the selected top-level releases.

The probe lockfile contained 31 dependency packages, while the active default
metadata closure contained 28. Lockfile advisory coverage and one
target-feature closure are therefore different scopes.

Cargo compiles and runs a package build script before the package. It permits
the script to inspect environment and filesystem state, create outputs, invoke
tools, and emit compiler or linker instructions:
<https://doc.rust-lang.org/cargo/reference/build-scripts.html>.

The Rust Reference states that procedural macros run during compilation with
the compiler's file and other resource access and therefore share build-script
security concerns:
<https://doc.rust-lang.org/reference/procedural-macros.html>.

## Build-script source review

| Exact release | Observed direct effects |
|---|---|
| `libc 0.2.189` | Reads target and policy environment; invokes `rustc`, and target-conditionally `freebsd-version` or `emcc`; emits cfg and rerun instructions |
| `getrandom 0.4.3` | Reads sanitizer configuration; emits cfg and rerun instructions |
| `proc-macro2 1.0.107` | Reads compiler and wrapper environment; creates and removes an `OUT_DIR` probe directory; invokes rustc compile and version probes; emits cfg and rerun instructions |
| `quote 1.0.47` | Reads `RUSTC`; invokes a rustc version probe; emits cfg and rerun instructions |
| `serde_core 1.0.229` | Reads target, package, and compiler environment; writes generated `private.rs` in `OUT_DIR`; invokes a rustc version probe; emits cfg and rerun instructions |
| `serde 1.0.229` | Reads package and compiler environment; writes generated `private.rs` in `OUT_DIR`; invokes a rustc version probe; emits cfg and rerun instructions |

No selected package declared Cargo's native `links` key in the observed
Windows closures. That is not proof of a Rust-only closure: `cc` and
`pkg-config` are APIs that downstream build scripts can use to invoke native
compilers or discover system libraries.

## Advisory matching

The lockfile was checked against a pinned RustSec database:

```powershell
cargo audit --json `
  --db <session>\ecos-q05-rustsec `
  --file <session>\ecos-q06-probe\Cargo.lock
git -C <session>\ecos-q05-rustsec rev-parse HEAD
```

Observed result:

```text
dependency-count: 31
vulnerabilities: 0
warnings: 0
advisory-db: 565436d86a136c840d01ad4a7851fc7391295404
```

`cargo audit` describes its purpose as matching Cargo dependencies to
vulnerabilities reported in the RustSec Advisory Database:
<https://github.com/rustsec/rustsec/tree/master/cargo-audit>.

An OSV batch query submitted the nineteen exact crates.io package-version
pairs. All nineteen results were empty at observation time. The OSV API
documents exact version and commit queries:
<https://google.github.io/osv.dev/api/>.

Zero matches means that the submitted identities did not match records in
those observed databases. It does not establish absence of vulnerabilities,
reachability, exploitability, review coverage, disclosure freshness, or native
component safety.

## Direct unsafe-syntax census

A disposable `syn 3.0.3` parser visited 710 Rust files from the extracted
`src` trees and the six root build scripts. All files parsed. It counted
syntax, not semantic soundness or expanded code.

```powershell
cargo build --quiet --manifest-path <scanner>\Cargo.toml
<scanner.exe> <exact-package>\src
<scanner.exe> <exact-package>\build.rs
```

| Exact release | Files | Unsafe blocks | Unsafe functions | Unsafe impls | Unsafe traits | Foreign items |
|---|---:|---:|---:|---:|---:|---:|
| `libc 0.2.189` | 387 | 8 | 194 | 10 | 0 | 7,551 |
| `cfg-if 1.0.4` | 1 | 0 | 0 | 0 | 0 | 0 |
| `getrandom 0.4.3` | 35 | 78 | 7 | 0 | 0 | 14 |
| `proc-macro2 1.0.107` | 15 | 3 | 3 | 0 | 0 | 0 |
| `quote 1.0.47` | 7 | 0 | 0 | 0 | 0 | 0 |
| `syn 3.0.3` | 55 | 30 | 2 | 3 | 0 | 0 |
| `serde_core 1.0.229` | 19 | 2 | 0 | 0 | 0 | 0 |
| `serde 1.0.229` | 24 | 2 | 0 | 0 | 0 | 0 |
| `log 0.4.33` | 9 | 3 | 2 | 1 | 0 | 0 |
| `tracing-core 0.1.36` | 14 | 20 | 3 | 5 | 0 | 0 |
| `bytes 1.12.1` | 19 | 61 | 57 | 12 | 1 | 0 |
| `http 1.5.0` | 21 | 49 | 3 | 10 | 0 | 0 |
| `tower-service 0.3.3` | 1 | 0 | 0 | 0 | 0 | 0 |
| `futures-core 0.3.33` | 7 | 3 | 0 | 2 | 0 | 0 |
| `rand_core 0.10.1` | 6 | 0 | 0 | 0 | 0 | 0 |
| `cc 1.4.2` | 16 | 13 | 2 | 2 | 0 | 0 |
| `pkg-config 0.3.33` | 1 | 0 | 0 | 0 | 0 | 0 |
| `hashbrown 0.17.1` | 28 | 314 | 104 | 25 | 1 | 0 |
| `memchr 2.8.3` | 45 | 112 | 205 | 2 | 0 | 0 |

Totals were 698 unsafe blocks, 582 unsafe functions, 72 unsafe impls, two
unsafe traits, eleven explicitly unsafe extern blocks, 7,565 foreign items,
two mutable statics, and 148 macro invocations whose unexpanded tokens
contained `unsafe`. `tower-service` contained the one observed
`forbid(unsafe_code)` attribute. The six build scripts contained no directly
parsed unsafe syntax.

The scanner does not:

- evaluate `cfg` for a target or feature set;
- expand declarative or procedural macros;
- inspect generated Rust;
- prove that an unsafe operation is reachable;
- evaluate safety comments or invariants;
- identify undefined behavior or soundness defects; or
- cover native code invoked or linked by build tooling.

## License inventory

All nineteen archives declared a license expression and packaged at least one
license file.

| Expression | Releases |
|---|---:|
| `MIT OR Apache-2.0` | 15 |
| `MIT` | 3 |
| `Unlicense OR MIT` | 1 |

SPDX expressions represent license choices and combinations:
<https://spdx.github.io/spdx-spec/v2.3/SPDX-license-expressions/>.
Presence of a valid expression and files does not decide a consumer's policy,
notice obligations, generated-code terms, native-library terms, or final
distribution compatibility.

## Tool boundaries

| Evidence tool | Establishes | Does not establish |
|---|---|---|
| Registry checksum | Exact archive bytes match a registry identity | Source review, repository equivalence, publisher intent |
| Package VCS metadata | Best-effort named revision and dirty state | Verified provenance or tarball-to-commit equivalence |
| Trusted publishing | Authenticated provider, repository, run, and commit for a release | Correct source, review, reproducibility, or safe behavior |
| `cargo audit` / OSV | Matches submitted identities to database records | Vulnerability absence, reachability, or complete native coverage |
| Syntax census | Direct parsed unsafe and FFI-related syntax | Expanded code, reachability, invariants, or soundness |
| Cargo metadata | Resolved packages, targets, features, and `links` declarations | Hidden runtime effects or downstream use of build helper APIs |
| `cargo-deny` | Configurable advisory, license, source, and dependency policy | Human code audit or safe execution |
| `cargo-vet` | Project-defined audit criteria, exemptions, imports, and differential audits | Universal security approval |

Primary tool sources:

- <https://embarkstudios.github.io/cargo-deny/>
- <https://mozilla.github.io/cargo-vet/how-it-works.html>

## Limitations

- One host, target family, and toolchain were measured.
- The queue is structurally selected rather than representative of every Rust
  application stack.
- Registry and OSV service state are time-dependent.
- RustSec was pinned, but the JSON report did not itself include the database
  commit; the commit was recorded separately.
- The OSV service query was not a frozen database snapshot.
- Package VCS commits resolving does not prove tag identity or archive
  reproducibility.
- Trusted publishing was present for only three selected releases.
- Build scripts were source-reviewed, not sandbox-traced.
- Procedural macro execution was identified by Cargo role, not dynamically
  traced.
- Native code and generated outputs were not exhaustively inventoried.
- Unsafe syntax was direct, unexpanded, and target-unfiltered.
- License metadata was inventoried without issuing legal conclusions.
