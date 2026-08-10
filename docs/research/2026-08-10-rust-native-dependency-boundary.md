# Rust Native Dependency Boundary

Date: 2026-08-10
Status: Complete
Question: ECOS-Q09
Decision: represent native integration as renewable evidence across source
mode, provider, host and target tools, discovery inputs, ABI, generated
bindings or code, Cargo instructions, native component identity, final
artifacts, assurance coverage, and observed reproducibility. System, bundled,
prebuilt, generated, and externally supplied modes shift ownership; none
removes the native boundary.

## Decision supported

ECOS-Q09 defines the native dependency and provider evidence required by the
OSPREY Ecosystem adapter, Macro and build-script adapter, Backend and linker
adapter, and Crate Ecosystem Ledger.

It does not:

- certify a native library, provider, toolchain, SDK, or generated binding;
- treat a successful host build as a cross-platform or deployment result;
- treat bundled source as tool-free, reproducible, or automatically patched;
- treat a system library as known from its linker name;
- infer a native boundary from Cargo `links` alone;
- infer absence of native effects from no `links` declaration;
- install compilers, assemblers, package managers, SDKs, libraries, code
  generators, or runtimes;
- switch TLS, crypto, database, or code-generation providers automatically;
- claim FIPS, security, ABI, license, or bit-reproducibility from compilation;
  or
- authorize OSPREY implementation.

## Native evidence model

| Dimension | Required evidence |
|---|---|
| Rust package identity | Registry, exact package and version, checksum, package VCS revision, owners, release time, features, target, profile, and dependency kind |
| Native component identity | Component name, version, revision or source ID, bundled/system/prebuilt/external mode, source or binary hash, patch set, and relationship to the Rust package |
| Provider identity | Capability, selected provider, selection mechanism, configuration owner, supported algorithms or semantics, fallback, and failure behavior |
| Host execution | Build-script or generator package and executable, host triple, environment, working directory, declared and observed inputs, process invocations, outputs, and capability uncertainty |
| Toolchain | Compiler, assembler, archiver, linker, code generator, binding generator, package-discovery tool, build system, SDK, versions, discovery path, flags, and wrappers |
| Target environment | Target triple, ABI, sysroot, SDK, system package database, headers, libraries, runtime loader, deployment image, and cross configuration |
| Discovery | Explicit path, environment, `pkg-config`, vcpkg, platform API, default search, Cargo target override, or fallback order plus requested version and observed result |
| Source mode | System, bundled source, vendored prebuilt object, vendored executable, pregenerated binding, generated binding, or externally supplied artifact |
| Cargo contract | `links` namespace, build-script directives, `DEP_*` metadata, link kind, search path, system libraries, cfg, environment, generated files, rerun declarations, and target override |
| ABI and interface | Header or binding identity, calling convention, symbol and library name, static/dynamic linkage, runtime library, layout assumptions, minimum native version, and compatibility result |
| Artifact evidence | Native objects and archives, Rust libraries, executable, debug data, generated source, distribution package, hashes, sizes, retained behavior, and provenance join |
| Assurance | Rust and native advisories, audit scope, unsafe and FFI review, source and prebuilt-object review, native and generated licenses/notices, security contact, and explicit unknowns |
| Reproducibility | Exact inputs and tools, clean path, environment, generated-output identity, native archive identity, final artifact identity, semantic output, variance, and known nondeterminism |
| Result state | Observed pass, expected unsupported, missing tool, missing system package, discovery failure, ABI/link failure, generator failure, runtime failure, not observed, stale, or unknown |
| Renewal | Package, feature, provider, native source, system package, tool, SDK, sysroot, environment, target, profile, build script, generated output, or deployment change |

No package count, `links` count, bundled flag, or provider name can represent
these dimensions alone.

## Measured controls

Commands, exact releases, release identities, active dependency trees, build
and run results, artifact hashes, generated-output comparisons, source and
binary inventories, and limitations are in
[EXP-01](ecos-q09-native-dependency-boundary/results/EXP-01-native-boundary-matrix.md).

Observed on Windows x86-64 with Cargo and rustc 1.95.0:

- `cc 1.4.2` compiled and linked a C function even though `cl.exe` was not on
  `PATH`; cc-rs discovered Visual Studio Build Tools and the program returned
  `42`;
- setting `CC_FORCE_DISABLE=1` changed the same fixture to an expected build
  failure;
- `pkg-config 0.3.33` failed when the executable was absent, failed explicitly
  when `Q09NATIVE_NO_PKG_CONFIG` was set, and rejected unconfigured
  cross-discovery;
- an explicitly supplied synthetic `pkg-config` executable plus a target
  sysroot setting returned version, library, link-path, and include-path
  metadata for both host and cross controls; no native link was claimed;
- `rusqlite 0.40.2` with system `libsqlite3-sys 0.38.2` compiled Rust code but
  failed linking because `sqlite3.lib` was absent;
- the `bundled` SQLite fixture compiled packaged C source, linked, ran, and
  reported SQLite `3.53.2`;
- `native-tls 0.2.18` selected Schannel on Windows and constructed a connector,
  while the same manifest's Linux target closure selected OpenSSL and failed
  because target OpenSSL and cross `pkg-config` configuration were absent;
- equivalent Rustls 0.23.43 controls with `std` and `tls12` built with
  AWS-LC 1.18.0 / aws-lc-sys 0.44.0 and ring 0.17.14;
- both Rustls providers exposed the same nine measured TLS cipher suites, while
  AWS-LC additionally exposed `X25519MLKEM768`;
- the AWS-LC closure contained 19 active packages, four build-script packages,
  two `links` packages, a 7,383,040-byte native archive set, a 97,069,200-byte
  target tree, and a 1,597,952-byte executable;
- the ring closure contained 14 active packages, two build-script packages,
  one `links` package, a 777,122-byte native archive set, a 45,738,438-byte
  target tree, and a 694,272-byte executable;
- these provider artifact differences are exact fixture observations, not a
  universal cost or security ranking;
- `prost-build 0.14.4` failed without `protoc`, then passed when `PROTOC`
  identified the vendored executable;
- `protoc-bin-vendored 3.2.0` passed and selected its packaged Win32
  `libprotoc 31.1` executable;
- the vendored protoc closure added its wrapper plus eight platform binary
  packages: 27,336,457 compressed archive bytes and 87,847,644 unpacked
  protoc-executable bytes across all packaged platforms, while only the
  Win32 executable ran;
- external-explicit and vendored protoc modes emitted the same 200-byte
  generated Rust file with SHA-256
  `b1f834171614474a0f6245629c93a86cce8479de83e5f38195a8d027f500feec`;
- `bindgen 0.72.1` failed because no `clang.dll` or `libclang.dll` was
  available, matching its documented libclang requirement;
- two fresh target-directory controls and two same-path clean rebuild controls
  produced equal semantic output but different hashes for the cc and bundled
  SQLite native archives and executables;
- generated Prost Rust remained byte-identical across the selected controls;
- all nineteen evaluated crate archives matched their crates.io SHA-256
  checksums; and
- pinned `cargo audit 0.22.2` reported zero vulnerabilities and zero warnings
  for five representative lockfiles, a dated Rust-package observation that
  did not identify system OpenSSL, Schannel patch state, or all embedded native
  component advisories.

## Findings

### FERRIUM-623: the native boundary is an execution and artifact chain

**Sources:** ECOS-Q09 fixture matrix, Cargo build-script documentation, cc-rs,
pkg-config-rs, libsqlite3-sys, native-tls, rustls, AWS-LC, ring, Prost, and
bindgen primary documentation.

**Observed behavior:** Native capability crossed Rust package resolution,
host process execution, tool discovery, native compilation, generated source,
link metadata, ABI, final artifacts, runtime behavior, and external update
systems. Different controls failed at different stages.

**Implication:** OSPREY must retain typed nodes and edges for every stage. A
crate or feature label cannot stand in for the complete boundary.

**Confidence:** High.

### FERRIUM-624: the Cargo graph does not identify ambient system artifacts

**Sources:** system SQLite and Linux native-tls controls.

**Observed behavior:** The system SQLite closure named `sqlite3` through
`libsqlite3-sys`, but the required `sqlite3.lib`, native version, patch state,
and installation owner were absent. The Linux TLS closure named
`openssl-sys 0.9.117`, but no target OpenSSL installation was identified.

**Implication:** Record requested native identity separately from discovered
and linked artifact identity. An unresolved linker name is not a native package
record.

**Confidence:** High.

### FERRIUM-625: successful tool discovery can occur outside `PATH`

**Sources:** cc-native control and cc-rs compile-time requirements.

**Observed behavior:** `cl.exe`, `lib.exe`, and the MSVC linker were not
available through ordinary command lookup. cc-rs and rustc still discovered
Visual Studio Build Tools 14.44.35207; the compiler reported version
19.44.35228 and the native program ran.

**Implication:** Tool evidence must record the executable that actually ran,
its discovery mechanism and version, not only a pre-build `PATH` inventory.

**Confidence:** High.

### FERRIUM-626: system and bundled modes shift responsibility

**Sources:** exact system and bundled SQLite controls.

**Observed behavior:** System mode deferred installation and patching to the
host and failed at link. Bundled mode packaged more than 20 MB of C/header
source, invoked the native compiler, linked a 4,945,074-byte SQLite archive,
and ran SQLite 3.53.2.

**Implication:** Profiles must state who owns native installation, update,
patch, ABI, license, and deployment work in each mode. `bundled` means
relocated responsibility, not no native dependency.

**Confidence:** High.

### FERRIUM-627: target selection can replace the provider closure

**Sources:** native-tls documentation and Windows/Linux Cargo trees.

**Observed behavior:** The Windows closure contained five packages and used
Schannel. The Linux closure contained 21 packages, six build-script packages,
one procedural macro, and `openssl-sys links=openssl`, then failed target
OpenSSL discovery on the Windows host.

**Implication:** Provider and native closure evidence belongs to the exact
host/target pair. A host result must not be copied to another target.

**Confidence:** High.

### FERRIUM-628: provider choice changes capability and artifact shape

**Sources:** equivalent Rustls provider fixtures and CryptoProvider
documentation.

**Observed behavior:** AWS-LC and ring exposed the same nine measured cipher
suites. AWS-LC added one post-quantum key-exchange group and produced a larger
native archive, target tree, and executable in the controlled profile.

**Implication:** Provider comparison must include functional capability,
configuration, native source and objects, tools, artifacts, assurance,
platforms, and consumer policy. Size or package count alone is not a verdict.

**Confidence:** High for the exact fixtures.

### FERRIUM-629: bundled native source may include pregenerated objects and bindings

**Sources:** aws-lc-sys 0.44.0 and ring 0.17.14 archives and build
documentation.

**Observed behavior:** aws-lc-sys packaged 1,807 native source/header files,
48,565,290 source bytes, and 26 prebuilt object files. Ring packaged 136
native source/header files, 4,848,876 source bytes, and 17 pregenerated object
files. Both used checked-in generated material to reduce ambient tool needs.

**Implication:** Source-bundled, prebuilt-object, pregenerated-binding, and
generated-at-build modes need distinct provenance, review, and reproducibility
states.

**Confidence:** High.

### FERRIUM-630: code generators are exact build inputs

**Sources:** Prost documentation and external, explicit, and vendored protoc
controls.

**Observed behavior:** The same package graph failed without `protoc`, passed
with an explicit executable, and passed through the vendored wrapper. The
successful generator identified itself as `libprotoc 31.1`.

**Implication:** Generated-code evidence must retain generator executable,
version, hash, selection mechanism, arguments, schemas/includes, environment,
and output identity.

**Confidence:** High.

### FERRIUM-631: vendoring a tool can widen the supply chain beyond the host

**Sources:** protoc-bin-vendored Cargo tree and package archives.

**Observed behavior:** The wrapper depended on all eight platform-specific
binary packages. The Windows build ran one executable, while the lock/download
closure carried binaries for Linux, macOS, and Windows.

**Implication:** Distinguish lockfile, download, active execution, and
distribution scopes. A host-selected executable does not erase unused packaged
binary identities.

**Confidence:** High.

### FERRIUM-632: generated-source reproducibility and final-artifact reproducibility differ

**Sources:** Prost generated hashes plus cc and SQLite archive/executable
hashes.

**Observed behavior:** Prost emitted byte-identical generated Rust across
external-explicit, vendored, and separate target paths. Native archives and
final executables changed across clean builds.

**Implication:** Record generated code, native objects, archives, Rust
artifacts, debug data, and final deliverables as separate reproducibility
claims.

**Confidence:** High.

### FERRIUM-633: bundling did not produce bit-identical native artifacts

**Sources:** separate-path and same-path clean rebuild controls.

**Observed behavior:** cc and bundled SQLite retained equal semantics and, in
same-path controls, equal native archive sizes, but archive and executable
SHA-256 values changed on each clean rebuild.

**Implication:** Do not infer reproducibility from an exact Cargo.lock,
packaged native source, or a passing rebuild. A reproducibility profile needs
toolchain, path, environment, timestamp/debug, archive, linker, and final
artifact controls.

**Confidence:** High for MSVC on this host; unknown elsewhere.

### FERRIUM-634: bindgen adds a host compiler-library contract

**Sources:** bindgen requirements and the libclang failure control.

**Observed behavior:** The Rust dependency closure compiled, but the build
script could not generate bindings without a loadable Clang 9-or-newer
libclang library and explicit discovery when not in a default location.

**Implication:** Binding generation must record header closure, clang and
libclang identities, flags, target, macros, includes, generated Rust, and a
pregenerated-versus-live policy.

**Confidence:** High.

### FERRIUM-635: cross-compilation requires target-specific discovery

**Sources:** pkg-config cross-compilation documentation, explicit controls,
and Linux native-tls failure.

**Observed behavior:** Unconfigured cross `pkg-config` stopped before probing.
An explicit executable and sysroot setting allowed the synthetic control, but
did not prove target headers, libraries, ABI, link, run, or deployment.

**Implication:** Require target-specific executable, sysroot, package paths,
headers, libraries, compiler, linker, runner, and validation stages. An allow
flag alone is insufficient.

**Confidence:** High.

### FERRIUM-636: Cargo `links` is a locator, not a native verdict

**Sources:** Cargo `links` documentation, native closures, and
prettyplease 0.2.37.

**Observed behavior:** libsqlite3-sys, openssl-sys, AWS-LC, ring, and clang-sys
used `links` for native or external coordination. Prettyplease also declared
`links = "prettyplease02"` solely to transmit build metadata.

**Implication:** `links` identifies an exclusive metadata namespace and direct
dependent contract. It neither proves native linkage nor inventories all
native effects.

**Confidence:** High.

### FERRIUM-637: build-script directives are material native evidence

**Sources:** saved Cargo build-script outputs.

**Observed behavior:** Scripts emitted exact static library names, native
search paths, include paths, metadata, environment dependencies, and generated
output. These effective instructions determined later compilation and link
behavior.

**Implication:** Capture effective directives and execution identity
separately. Replayed output is not proof that a process ran during the current
invocation.

**Confidence:** High.

### FERRIUM-638: Rust-package assurance does not close native assurance

**Sources:** nineteen archive checks, five cargo-audit lockfiles, system and
bundled controls, and Q06 assurance model.

**Observed behavior:** Rust package hashes and advisory queries were
reproducible and empty for the observed snapshots. System OpenSSL was absent,
Schannel belonged to the OS, and bundled SQLite/AWS-LC/ring introduced native
source or prebuilt objects with separate patch, advisory, and license scope.

**Implication:** Join Rust and native advisories, source revisions, patch
levels, licenses, notices, review criteria, security contacts, and deployment
inventory without flattening them into one score.

**Confidence:** High.

### FERRIUM-639: native evidence expires with the environment

**Sources:** all target, provider, tool, discovery, and artifact controls.

**Observed behavior:** Results depended on package versions, features, target,
Visual Studio toolset, system libraries, environment, sysroot, generator,
prebuilt objects, generated code, and output paths. Several changed outside
the Rust package API.

**Implication:** Renew native evidence on any package, feature, target,
provider, native component, compiler, assembler, archiver, linker, generator,
SDK, sysroot, system package, environment, build script, generated output, or
deployment change.

**Confidence:** High.

## Recommendations

### Adopt now

- Add the native evidence model to the OSPREY Ecosystem, Macro and
  build-script, and Backend and linker adapters.
- Record requested, discovered, built, linked, loaded, and executed native
  identities separately.
- Preserve system, bundled-source, prebuilt-object, vendored-executable,
  pregenerated-binding, live-generation, and external-artifact modes.
- Record provider capability and policy separately from implementation and
  artifact cost.
- Capture exact tools, discovery paths, sysroots, package databases,
  environment, Cargo directives, generated outputs, ABI, and final artifacts.
- Preserve typed missing-tool, missing-package, unsupported, link-failed,
  generator-failed, not-observed, stale, and unknown states.
- Join Rust-package assurance with native component, system, and distribution
  assurance.
- Require renewable evidence rather than a native-free, bundled, portable, or
  reproducible label.

Owner: FERRIUM.

Expected validation: ECOS-Q10 selection evidence, ECOS-Q11 stack-profile
renewal on multiple operating systems and targets, ECOS-Q12 intervention
decisions, and later CONFORMANCE-001 tests.

Non-goals: package installation, provider switching, build-script execution
changes, native patching, ABI certification, FIPS certification, legal advice,
or OSPREY implementation.

### Prototype behind a compatibility boundary

- read-only native-boundary inventory over Cargo metadata, feature trees,
  build-script outputs, compiler/linker plans, generated files, and artifacts;
- host/target prerequisite probes with actionable missing-tool and
  missing-package diagnostics;
- system-versus-bundled and provider comparisons with exact capability,
  license, advisory, artifact, validation, and rollback evidence;
- generated-code and binding identity comparison across explicit and vendored
  tools;
- reproducibility observations that compare generated, native archive, Rust,
  debug, and final artifact layers independently;
- Cargo target-override and external-native metadata validation; and
- FERRIS evidence packets for immutable before/after native observations.

Owner: FERRIUM for evidence vocabulary and fixtures; Cargo, crate maintainers,
native-library owners, and tool owners retain their systems.

Expected validation: exact host/target tool inventories, positive and negative
fixtures, multiple Tier 1 platforms, cross-link and execution controls,
advisory/license coverage, held-out repositories, removal, and rollback.

Non-goals: hiding missing prerequisites, silently selecting fallbacks, or
making ordinary Cargo depend on FERRIUM.

### Reject or defer

- one native-risk, portability, reproducibility, or provider score;
- native-free claims from no Cargo `links`;
- native claims from Cargo `links` without observed artifacts;
- treating `bundled` or `vendored` as self-contained, patched, reviewed, or
  reproducible;
- treating a lockfile as a system-package lock;
- automatic installation of compilers, assemblers, SDKs, package managers,
  system libraries, runtimes, protoc, Clang, or libclang;
- automatic system/bundled, static/dynamic, TLS, crypto, database, FIPS, or
  generator provider changes;
- generated-binding refresh without API/ABI review;
- bit-reproducibility claims from one clean build;
- build-script sandbox, cache, or output suppression claims not established by
  their own contract; and
- OSPREY implementation before the Crates Series gate.

## Role review

### Rust Safety Steward

Accepts explicit FFI, provider, prebuilt-object, generated-binding, system
library, and unsafe review boundaries. Requires crypto mode, native version,
patch level, ABI assumptions, and application-owned provider configuration to
remain visible.

### Compiler Performance Engineer

Accepts active closure, native archive, executable, target footprint, and
generated-output measurements. Rejects universal build-cost conclusions from
the one cold observation per final provider and requires profile-, target-,
cache-, and tool-specific renewal.

### Interop Boundary Auditor

Accepts requested/discovered/linked/loaded identity, ABI, header, binding,
calling convention, link kind, runtime library, and deployment as separate
evidence. Requires future profiles to execute representative cross-language
calls on every supported ABI.

### AI Assurance Skeptic

Accepts exact releases, owners, checksums, revisions, primary sources, expected
failures, explicit tool controls, equal-output controls, and negative
reproducibility results. Rejects inferred system versions, native-free labels,
and generated-code equivalence beyond the measured hash.

### Ecosystem Strategist

Accepts Cargo, cc-rs, pkg-config-rs, existing provider features, crate-owned
vendoring, and upstream diagnostics as the owner mechanisms. Requires FERRIUM
to contribute evidence vocabulary and minimized cases rather than create a
native package manager or TLS/database distribution.

### Rust Maintainer

Accepts diagnostics that identify the missing tool, package, provider, edge,
environment, directive, artifact, and remediation owner. Requires ordinary
Cargo operation, reviewable configuration, no surprise installation, and a
clear removal path.

### Native Platform Adopter

Accepts system integration, bundled fallback, platform TLS, explicit provider,
cross-sysroot, runtime loader, SDK, deployment, update, notice, and rollback as
first-class concerns. Requires real installation and deployment tests before
profile adoption.

### Scope Keeper

Accepts Q09 as evidence and selection input only. Installation, provider
switching, generated-binding updates, native patching, build-script
restriction, stack approval, and OSPREY implementation remain closed.

### Validation Checker

Accepts ten fixture families, positive and expected-failure states, host and
cross discovery controls, native links and execution, provider capability and
artifact comparison, generated hash equivalence, same- and different-path
reproducibility controls, nineteen archive checks, five dated audits, sources,
and limitations. Requires multi-platform renewal in ECOS-Q11.

## Limitations

- Measurements used one Windows x86-64 host and one current Cargo/rustc pair.
- Linux was resolved and checked as a cross target but was not linked or run.
- No macOS, Linux-host, MinGW, mobile, embedded, or WASM native integration was
  executed.
- No system SQLite, target OpenSSL, NASM, protoc, Clang, or libclang was
  installed for an ambient positive control.
- The explicit pkg-config executable was synthetic and did not prove a native
  package, header, ABI, link, or runtime.
- TLS fixtures constructed providers/configuration and enumerated capability;
  they did not perform a network handshake, certificate validation, FIPS
  validation, or performance benchmark.
- Rustls artifact sizes include the exact fixture code and one profile. They
  are not provider rankings.
- Build elapsed times were single cold observations and are not promoted as
  benchmarks.
- Reproducibility controls did not localize differing bytes or test remapping,
  timestamps, PDB identity, deterministic archives, signing, or packaging.
- Five cargo-audit results covered Rust lockfiles, not every native advisory
  source or deployed system component.
- License expressions were inventoried; no legal compatibility decision was
  made.
- The fixtures did not test dynamic-library loading or distribution packaging.
- No production repository or held-out stack was modified.

## Sources

- Cargo build scripts and `links`:
  <https://doc.rust-lang.org/cargo/reference/build-scripts.html>
- Cargo target `links` overrides:
  <https://doc.rust-lang.org/cargo/reference/config.html#targettriplelinks>
- cc 1.4.2:
  <https://docs.rs/cc/1.4.2/cc/>
- pkg-config 0.3.33 cross compilation:
  <https://docs.rs/pkg-config/0.3.33/pkg_config/#cross-compilation>
- native-tls 0.2.18:
  <https://docs.rs/native-tls/0.2.18/native_tls/>
- Rustls CryptoProvider:
  <https://docs.rs/rustls/0.23.43/rustls/crypto/struct.CryptoProvider.html>
- aws-lc-rs 1.18.0:
  <https://docs.rs/aws-lc-rs/1.18.0/aws_lc_rs/>
- aws-lc-sys build support:
  <https://github.com/aws/aws-lc-rs/blob/main/aws-lc-sys/README.md>
- ring 0.17.14:
  <https://docs.rs/ring/0.17.14/ring/>
- rusqlite 0.40.2 and libsqlite3-sys features:
  <https://github.com/rusqlite/rusqlite/blob/master/README.md#optional-features>
- Prost build and protoc sourcing:
  <https://docs.rs/prost-build/0.14.4/prost_build/>
- protoc-bin-vendored 3.2.0:
  <https://docs.rs/protoc-bin-vendored/3.2.0/protoc_bin_vendored/>
- bindgen 0.72.1:
  <https://docs.rs/bindgen/0.72.1/bindgen/>
- bindgen requirements:
  <https://rust-lang.github.io/rust-bindgen/requirements.html>
- RustSec cargo-audit:
  <https://rustsec.org/>
