# Rust Compatibility-Tested Stack Profiles

Date: 2026-08-10
Status: Complete
Question: ECOS-Q11
Decision: adopt renewable compatibility-profile records and reference
validation shapes, not a permanent FERRIUM distribution. A profile is an
expiring consumer contract over exact releases, features, lock selection,
active target closure, compiler and platform pair, validation stages,
provenance, ownership, renewal, removal, and rollback. It is not a universal
stack recommendation, certification, or installation authority.

## Decision supported

ECOS-Q11 determines whether representative Rust application stacks can be
tested and renewed without creating hidden lock-in.

It does not:

- approve one universal server, CLI, data, embedded, browser, or database
  stack;
- call any crate or stack standard, blessed, certified, safe, secure,
  maintained, portable, or future-proof;
- publish FERRIUM-owned wrapper crates, templates, lockfiles, or a
  distribution;
- infer runtime support from dependency resolution or compilation;
- automatically install toolchains, targets, native compilers, system
  packages, or JavaScript tooling;
- automatically update, downgrade, replace, remove, or reject dependencies; or
- authorize OSPREY implementation before ECOS-Q12 and the final series gate.

## Research question

Can representative application stacks be tested as renewable profiles without
creating a permanent FERRIUM distribution or lock-in?

The decision requires four distinct answers:

1. whether exact multi-crate stacks can perform representative operations;
2. whether support remains explicit per compiler, target, and validation
   stage;
3. whether a dependency update can be reviewed and rolled back as a bounded
   change; and
4. whether ownership, expiry, removal, and substitution prevent the profile
   from becoming a permanent package policy.

## Profile contract

A renewable profile needs ten layers:

1. **Consumer requirements:** operation, semantics, deployment, platform,
   compiler, runtime, provider, policy, operations, and non-goals.
2. **Profile identity:** profile ID, revision, owner, observation time,
   evidence expiry, and supersession state.
3. **Selection:** exact direct releases, requested features, source,
   `Cargo.lock` identity, and accepted alternatives.
4. **Closure:** lockfile package universe plus the active normal/build
   dependency closure for each target, including build scripts, procedural
   macros, Cargo `links`, native code, and generated code.
5. **Environment:** Cargo/rustc pair, host/target pair, installed Rust target,
   linker, runner, native tools, SDK, system packages, and deployment
   assumptions.
6. **Validation stages:** resolve, check, build, link, execute, test,
   package, deploy, and operational state recorded independently as pass,
   fail, expected rejection, unsupported, not observed, stale, or unknown.
7. **Assurance:** archive checksum, package VCS revision, owners, release date,
   license, advisory snapshot, review scope, and residual unknowns.
8. **Renewal:** proposed graph diff, changed releases, changed features,
   changed compiler floor, advisory and owner diff, validation plan, approval,
   evidence date, and next expiry.
9. **Removal:** consumer-owned replacement boundary, migration validation,
   data or wire compatibility, native cleanup, and proof that ordinary Cargo
   operation remains available.
10. **Rollback:** prior manifest and lock identity, artifact or deployment
    rollback, validation command, owner, and expiry.

The profile is therefore evidence about one exact consumer boundary. The crate
names are replaceable implementation choices below that boundary.

## Measured profiles

The detailed commands, exact release provenance, closures, target matrix, and
renewal control are recorded in
[EXP-01](ecos-q11-compatibility-stack-profiles/results/EXP-01-stack-profile-matrix.md).

Six independent fixtures exercised six application shapes:

| Profile | Exact direct stack | Representative operation |
|---|---|---|
| hosted server | Axum 0.8.9, Tokio 1.53.1, Tower 0.5.3, Serde 1.0.229, Serde JSON 1.0.151, Tracing 0.1.44 | issue an in-process `/health` request and decode the JSON response |
| CLI and configuration | Clap 4.6.6, TOML 0.9.8, Serde 1.0.229, Tracing 0.1.44, Tracing Subscriber 0.3.23 | parse CLI input, decode TOML, initialize tracing, and print configuration |
| pure-Rust data | CSV 1.4.0, Jiff 0.2.35, UUID 1.24.0, Serde 1.0.229, Serde JSON 1.0.151 | parse a CSV row, validate UUID/time values, and emit JSON |
| embedded `no_std` | Heapless 0.9.3, Postcard 1.1.3, Serde 1.0.229 | retain a fixed-capacity window and serialize a reading without `std` |
| browser WASM | wasm-bindgen 0.2.127, serde-wasm-bindgen 0.6.5, Getrandom 0.4.3, Serde 1.0.229 | convert a Serde value to `JsValue` with a WASM-specific random provider |
| bundled-native SQLite | Rusqlite 0.40.2, Serde 1.0.229, Serde JSON 1.0.151 | compile bundled SQLite, query an in-memory database, and emit JSON |

These profiles are deliberately separate. Combining them into one application
platform would merge incompatible target, runtime, compiler, and native-tool
assumptions.

## Closure and artifact results

Active packages were counted from target-specific `cargo tree` normal/build
edges. Lock packages count the broader package universe retained by the
lockfile. The difference is material and must remain visible.

| Profile | Lock packages | Active target packages | Build-script packages | Proc-macro packages | `links` packages | Observed output |
|---|---:|---:|---:|---:|---:|---:|
| hosted server | 55 | 53 | 7 | 3 | 0 | 750,080-byte Windows executable |
| CLI/configuration | 51 | 47 | 4 | 3 | 0 | 1,000,960-byte Windows executable |
| pure-Rust data | 46 | 21 | 6 | 1 | 0 | 374,784-byte Windows executable |
| embedded `no_std` | 16 | 16 | 6 | 2 | 0 | target `rlib` checks/builds |
| browser WASM | 27 | 25 | 8 | 3 | 1 | 527,921-byte `.wasm` |
| bundled-native SQLite | 41 | 26 | 7 | 1 | 1 | 1,796,608-byte Windows executable |

The unfiltered native lock universe included WASM-related packages that were
not compiled for Windows. The active Windows graph contained 26 packages, not
41. The data lockfile similarly retained 46 packages while its Windows
normal/build graph used 21. Profile evidence must therefore retain both
lock-universe and target-active closure identities.

The isolated target directories ranged from about 44.8 MiB for one embedded
check root to 175.3 MiB for the hosted-server build root. These are one-machine
storage observations, not comparative build-time benchmarks. Concurrent
registry-cache waiting invalidated elapsed-time comparisons.

## Compiler floors

The highest declared `rust-version` in an active closure was used only to
select a candidate floor. Each exact lock was then rebuilt or executed with
that compiler:

| Profile | Candidate floor | Observed stage |
|---|---:|---|
| hosted server | Rust 1.80.0 | Windows release build and execution passed |
| CLI/configuration | Rust 1.85.0 | Windows release build and execution passed |
| pure-Rust data | Rust 1.85.0 | Windows release build and execution passed |
| embedded `no_std` | Rust 1.87.0 | Thumb and RISC-V release builds passed |
| browser WASM | Rust 1.85.0 | `wasm32-unknown-unknown` release build passed |
| bundled-native SQLite | Rust 1.85.0 | Windows release build and execution passed |

Between two and nine active packages per profile did not declare
`rust-version`. The measured floor is therefore an observation for the exact
lock and command, not a metadata-derived promise for future resolution.

## Target and stage matrix

| Profile | Windows host | Linux target | WASM target | Embedded targets | Boundary result |
|---|---|---|---|---|---|
| hosted server | executed | checked | failed in `mio` | not intended | host networking profile |
| CLI/configuration | executed | checked | checked | not intended | WASM compilation does not prove browser CLI usability |
| pure-Rust data | executed | checked | checked | not tested | WASM runtime/time-zone behavior remains unobserved |
| embedded `no_std` | host unit test | not intended | not intended | Thumb and RISC-V built | hardware execution remains unobserved |
| browser WASM | deliberate compile rejection | not intended | built | not intended | browser execution remains unobserved |
| bundled-native SQLite | executed | cross-build failed before link | not intended | not intended | Linux attempt lacked `x86_64-linux-gnu-gcc` |

The native Linux failure is an environment/toolchain result, not evidence that
Rusqlite or bundled SQLite fails on a native Linux builder. A Linux claim
requires Linux-native compilation and execution.

The WASM fixture deliberately emitted a compile error on a non-WASM target.
That expected rejection prevents accidental host use from being reported as a
profile success. Node.js was present, but `wasm-bindgen`, `wasmtime`, and
`wasm-tools` executables were absent, so browser or JavaScript execution was
not observed.

## Provenance and advisory control

The eighteen direct exact releases retained:

- crates.io release time and declared Rust version;
- declared license;
- current crates.io user/team owners;
- registry checksum;
- independently hashed local `.crate` archive;
- checksum comparison; and
- packaged `.cargo_vcs_info.json` revision.

All eighteen archive hashes matched the registry checksum. Every archive
contained a VCS revision. These facts establish bounded source identity; they
do not prove review quality, source reproduction, future owner continuity, or
security.

All six profile lockfiles and the renewal control returned zero RustSec
vulnerability matches under cargo-audit 0.22.2 on 2026-08-10. This is a dated
advisory-database observation, not a security certification.

## Renewal and rollback control

A seventh fixture used the semver requirement `clap = "4.6"`:

1. pin the lock to Clap 4.6.5;
2. execute the baseline;
3. run `cargo update -p clap`;
4. inspect and validate the new lock;
5. execute Clap 4.6.6; and
6. restore the baseline lock and execute with `--locked`.

The update changed exactly two package versions:

```text
clap 4.6.5 -> 4.6.6
clap_builder 4.6.5 -> 4.6.6
```

The active closure remained 22 packages, both versions executed the required
operation, and restoring the prior lock reproduced its exact SHA-256:

```text
4b461f4034175df7e2cd637b81bcc287bb52c3b64c28cbdc0fefdf1c7a2580ce
```

This demonstrates a bounded renewal mechanism, not automatic upgrade safety.
A future update can change behavior, features, compiler requirements,
transitive closure, advisories, ownership, or native prerequisites even when
the direct semver requirement is unchanged.

## Lifecycle policy

### Ownership

- FERRIUM owns the profile schema, evidence method, and research snapshots.
- A consumer repository owns its requirements, adoption, exact lock,
  validation, deployment, exception policy, and rollback.
- Upstream crate owners retain crate APIs, releases, security response, and
  maintenance; a profile does not transfer those duties to FERRIUM.

### Renewal

A profile expires after 90 days unless the consumer defines a shorter period.
Renewal occurs earlier when:

- a direct or active dependency changes;
- a RustSec advisory or relevant security notice appears;
- registry owners or source custody changes;
- the compiler, target tier, SDK, native toolchain, provider, or deployment
  environment changes;
- a required validation stage fails or becomes unavailable; or
- consumer requirements change.

Renewal must diff the manifest, lockfile, active target graph, feature set,
compiler floor, provenance, advisories, target stages, artifacts, and
limitations. A green scheduled job may refresh evidence; it may not approve
adoption or merge an update without consumer policy.

### Removal

Every profile must name the capability boundary and at least one migration
path. Removal must:

1. stop profile-specific validation and automation;
2. remove profile metadata without breaking ordinary Cargo commands;
3. replace or remove direct dependencies through consumer-owned code changes;
4. validate public types, serialized data, wire contracts, database state,
   native artifacts, and deployment cleanup where relevant; and
5. preserve the prior profile record as historical evidence.

### Rollback

Rollback restores the previously approved manifest and lock identity, rebuilds
with the recorded toolchain and target prerequisites, reruns mandatory
validation, and restores the prior deployment or artifact according to the
consumer's operations contract. A lockfile rollback is insufficient when data,
wire, native ABI, or deployment state changed.

## Recommendation

### Adopt now

- the renewable profile record and stage vocabulary;
- exact-lock, target-active-closure, compiler-floor, provenance, advisory,
  expiry, removal, and rollback evidence;
- separate host, embedded, WASM, and native profile lanes; and
- consumer-owned approval with FERRIUM-owned evidence methodology.

### Prototype behind a compatibility boundary

- read-only generation and diffing of profile records;
- native Linux execution for the bundled-SQLite lane;
- browser execution and JavaScript glue validation for the WASM lane;
- hardware or emulator execution for the embedded lane; and
- held-out consumer repositories to test adoption and removal cost.

These are candidate ECOS-Q12 interventions, not implementation authorization.

### Reject or defer

- a permanent FERRIUM crate distribution or global lockfile;
- one "recommended Rust stack";
- profile labels that collapse check, build, link, run, test, deploy, security,
  or maintenance state;
- automatic package installation, dependency updates, provider switching, or
  source rewriting;
- certification based on compilation, one test, one audit, or one date; and
- compatibility claims for unexecuted targets.

## Findings

### FERRIUM-658: a profile is a consumer contract, not a distribution

**Sources:** six exact fixtures, ECOS-Q03 through ECOS-Q10 evidence models, and
the lifecycle policy above.

**Observed behavior:** Useful evidence required requirements, exact selection,
environment, validation, assurance, and lifecycle records beyond a crate list.

**Implication:** OSPREY must represent a profile as an expiring evidence record.
FERRIUM must not own a global package set.

**Confidence:** High.

### FERRIUM-659: profile identity must be exact and renewable

**Sources:** exact manifests, lockfiles, archive checks, target graphs, and
renewal control.

**Observed behavior:** The meaningful identity joined direct releases,
features, lock hash, active target closure, compiler, target, and evidence
date.

**Implication:** A name such as "server stack" is insufficient identity.

**Confidence:** High.

### FERRIUM-660: one universal stack would merge incompatible assumptions

**Sources:** six profile target and operation contracts.

**Observed behavior:** Hosted networking required `std` and host I/O; embedded
removed `std`; browser WASM required JavaScript bindings; native SQLite
required C compilation.

**Implication:** Profiles must remain independent capability lanes.

**Confidence:** High.

### FERRIUM-661: lock universe and active target closure are different

**Sources:** target-specific `cargo tree`, lock package counts, and build logs.

**Observed behavior:** Native retained 41 lock packages but compiled 26 on
Windows; data retained 46 but used 21.

**Implication:** Profile cost, assurance, and platform claims must state which
closure they cover.

**Confidence:** High.

### FERRIUM-662: compatibility is stage-specific

**Sources:** resolve, check, build, link, run, and test controls.

**Observed behavior:** CLI and data checked for WASM without browser execution;
the WASM profile built without JavaScript execution; Linux-native SQLite failed
before link because a cross C compiler was absent.

**Implication:** One compatible/incompatible field would misstate the evidence.

**Confidence:** High.

### FERRIUM-663: compiler floors are profile-specific observations

**Sources:** Rust 1.80, 1.85, and 1.87 profile controls.

**Observed behavior:** Server executed on 1.80; CLI, data, WASM, and native
validated on 1.85; embedded built on 1.87.

**Implication:** A profile must record the tested compiler, not inherit a lab
wide MSRV.

**Confidence:** High for the exact locks and commands.

### FERRIUM-664: declared Rust versions do not close the floor claim

**Sources:** active-closure metadata and compiler-floor controls.

**Observed behavior:** Every profile contained packages without a declared
`rust-version`.

**Implication:** The highest declaration selects a test candidate; only the
exact compiler control establishes observed eligibility.

**Confidence:** High.

### FERRIUM-665: the hosted server profile is not a browser profile

**Sources:** Windows execution, Linux check, and WASM failure.

**Observed behavior:** The in-process Axum operation passed on Windows and the
stack checked for Linux, while `mio` failed to compile for browser WASM.

**Implication:** Hosted-network and browser-network profiles require separate
contracts.

**Confidence:** High for compilation; Linux execution was not observed.

### FERRIUM-666: compiling a CLI for WASM does not establish usability

**Sources:** CLI Windows execution and WASM check.

**Observed behavior:** Clap/TOML/Tracing checked for WASM, but no browser
argument source, terminal, tracing sink, or runtime execution was tested.

**Implication:** Incidental compilation must not expand the supported profile.

**Confidence:** High.

### FERRIUM-667: the data lane is broadly compilable but runtime semantics vary

**Sources:** Windows execution plus Linux and WASM checks.

**Observed behavior:** CSV, UUID, Jiff, Serde, and JSON completed the operation
on Windows and compiled for Linux and WASM.

**Implication:** WASM time-zone/data-source and runtime behavior remain a
separate validation stage.

**Confidence:** High for observed stages.

### FERRIUM-668: embedded evidence needs architecture and execution scope

**Sources:** host unit test and Rust 1.87 Thumb/RISC-V builds.

**Observed behavior:** The `no_std` stack serialized data and built for two
architectures, but no board, emulator, allocator pressure, panic, or transport
behavior was executed.

**Implication:** Embedded profiles must retain architecture and runner state.

**Confidence:** High for compile/build evidence.

### FERRIUM-669: browser WASM needs a JavaScript execution boundary

**Sources:** WASM build, deliberate host rejection, and runtime-tool inventory.

**Observed behavior:** The 527,921-byte module built and host compilation was
rejected, but browser glue generation and execution were unavailable.

**Implication:** `.wasm` production is not browser compatibility.

**Confidence:** High.

### FERRIUM-670: bundled native source moves rather than removes the boundary

**Sources:** bundled Rusqlite Windows run and Linux cross-build failure.

**Observed behavior:** Windows compiled packaged SQLite and executed; the Linux
cross target required an unavailable `x86_64-linux-gnu-gcc`.

**Implication:** Native profiles must retain source mode, compiler, target, and
execution evidence.

**Confidence:** High for the observed environment.

### FERRIUM-671: direct-release identity is reproducible but bounded

**Sources:** eighteen registry checksums, archive hashes, VCS revisions,
owners, licenses, and release dates.

**Observed behavior:** Every archive hash matched and exposed a packaged VCS
revision.

**Implication:** Profiles can preserve exact source identity without implying
source reproduction, review quality, or future stewardship.

**Confidence:** High.

### FERRIUM-672: zero advisory matches are dated evidence

**Sources:** seven cargo-audit 0.22.2 lockfile runs on 2026-08-10.

**Observed behavior:** No RustSec vulnerabilities were reported.

**Implication:** Advisory evidence needs database, tool, scope, time, expiry,
and renewal.

**Confidence:** High for the dated query.

### FERRIUM-673: renewal can be bounded and reversible

**Sources:** Clap 4.6.5-to-4.6.6 update and exact lock restoration.

**Observed behavior:** Two package versions changed, both operations passed,
and rollback restored the exact baseline hash.

**Implication:** A profile renewal should be a reviewed graph diff with a
tested rollback, not an automatic latest-version action.

**Confidence:** High.

### FERRIUM-674: unchanged package count does not prove unchanged risk

**Sources:** renewal control.

**Observed behavior:** The active closure stayed at 22 packages while two
package identities changed.

**Implication:** Renewal must compare identities, behavior, provenance,
advisories, and compiler requirements rather than only graph size.

**Confidence:** High.

### FERRIUM-675: profile ownership is deliberately split

**Sources:** lifecycle policy and existing upstream ownership.

**Observed behavior:** FERRIUM can own evidence shape, but only consumers can
approve requirements and deployment, while upstream owners control releases.

**Implication:** Profiles must not imply transferred maintenance or support.

**Confidence:** High.

### FERRIUM-676: expiry, removal, and rollback are anti-lock-in requirements

**Sources:** renewal and rollback control plus lifecycle policy.

**Observed behavior:** Exact selection was useful because it was time-bounded,
diffable, removable, and restorable.

**Implication:** A profile without expiry, removal, substitution, and rollback
is a hidden distribution.

**Confidence:** High.

### FERRIUM-677: the bounded opportunity is profile evidence automation

**Sources:** six profile controls and the Crates Series evidence model.

**Observed behavior:** The repeated work is collecting and diffing exact
identity, closure, target, validation, assurance, and lifecycle evidence.

**Implication:** ECOS-Q12 should evaluate a read-only profile record/diff
prototype, while keeping package approval and dependency mutation closed.

**Confidence:** Medium-high pending held-out consumer evaluation.

## Role review

### Rust Safety Steward

Accepts exact closures, compile-time execution, native boundaries, advisory
scope, typed unknowns, expiry, and rollback. Rejects a profile name, passing
build, or zero advisory result as safety proof.

### Compiler Performance Engineer

Accepts active versus lock closure, build-script and proc-macro counts, target
storage, artifact size, compiler floor, and isolated roots as profile evidence.
Rejects elapsed-time comparison because concurrent cache waits contaminated the
observations.

### Interop Boundary Auditor

Accepts operation, target, runtime, provider, data, native, serialization, and
replacement boundaries. Requires browser execution, native Linux execution,
and migration compatibility before those stages are promoted.

### AI Assurance Skeptic

Accepts exact commands, expected failures, archive hashes, VCS revisions,
compiler controls, and explicit not-observed states. Requires generated
recommendations to distinguish evidence collection from consumer approval.

### Ecosystem Strategist

Accepts renewable profiles as a defensible evidence layer that composes
existing crates and upstream ownership. Rejects a FERRIUM distribution,
certification brand, or universal stack.

### Rust Maintainer

Accepts consumer-scoped exact selections with alternatives, expiry, update
diffs, and rollback. Requires diagnostics to identify the failed stage,
changed package, affected requirement, and current upstream owner.

### Native Platform Adopter

Accepts separate host/target, compiler floor, native toolchain, runner,
deployment, removal, and rollback evidence. Requires Linux-native and browser
execution before those environments are claimed.

### Scope Keeper

Accepts Q11 as profile-model research and disposable validation only. Package
publication, permanent fixtures, automated updates, production integration,
and OSPREY implementation remain closed.

### Validation Checker

Accepts six exact profile operations, seven compiler-floor controls, target
positive and negative cases, active and lock closures, eighteen provenance
checks, seven dated audits, one renewal/rollback control, sources, and
limitations. Requires held-out consumer adoption and removal evidence before a
profile automation prototype advances beyond bounded evaluation.

## Limitations

- Execution was on one Windows 11 x86-64 host.
- Linux results were cross-target checks, not Linux-native execution.
- Browser glue generation and browser/Node execution were not observed.
- Embedded hardware or emulator execution was not observed.
- The server operation used an in-process request rather than a bound network
  listener.
- No deployment, upgrade under load, data migration, rollback deployment, or
  production operations were measured.
- No unsafe-code or full build-script source census was repeated for every
  transitive package; ECOS-Q06 and ECOS-Q09 define those evidence requirements.
- Artifact hashes identify observed outputs and do not establish
  reproducibility.
- Direct crate owners are a 2026-08-10 registry snapshot and may change.
- Zero RustSec matches can change with the advisory database.
- The six fixtures are representative controls, not ecosystem-wide stack
  coverage.

## Sources

Primary ecosystem and tool sources:

- [Cargo.toml versus Cargo.lock](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- [`cargo update`](https://doc.rust-lang.org/cargo/commands/cargo-update.html)
- [`cargo tree`](https://doc.rust-lang.org/cargo/commands/cargo-tree.html)
- [Cargo `rust-version`](https://doc.rust-lang.org/cargo/reference/rust-version.html)
- [Cargo platform-specific dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies)
- [wasm-bindgen guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [crates.io API](https://crates.io/api/)
- [RustSec advisory database](https://github.com/RustSec/advisory-db)

FERRIUM evidence foundations:

- [Rust interchange contracts](2026-08-09-rust-interchange-contracts.md)
- [Rust async portability](2026-08-09-rust-async-portability.md)
- [Rust maintenance and stewardship](2026-08-09-rust-maintenance-stewardship.md)
- [Rust security and provenance](2026-08-09-rust-security-provenance.md)
- [Rust platform compatibility](2026-08-09-rust-platform-compatibility.md)
- [Rust feature and version fragmentation](2026-08-09-rust-feature-version-fragmentation.md)
- [Rust native dependency boundary](2026-08-10-rust-native-dependency-boundary.md)
- [Rust crate discovery and selection](2026-08-10-rust-crate-discovery-selection.md)
