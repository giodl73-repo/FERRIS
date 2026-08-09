# Build-Script Input, Output, and Rerun Precision

Date: 2026-08-09
Question: PERF-Q23
Status: Complete
Decision: adopt precise build-script input inventory, execution, rerun-cause,
output-directive, output-lifecycle, and fan-out observability; consume Cargo's
nightly build-analysis evidence behind a versioned boundary; prototype
read-only undeclared-input and output-manifest diagnostics; contribute
minimized Cargo cases where useful; defer script-output caching,
unchanged-output suppression, sandbox enforcement, automatic cleanup, source
rewrites, package-manager replacement, and implementation.

## Executive conclusion

Cargo build scripts combine four systems that must not be collapsed:

- a host binary that Cargo compiles;
- a run unit whose freshness depends on package-wide or declared inputs;
- a saved instruction stream that Cargo replays when the run unit is fresh;
- files and effects produced by an unrestricted process.

Precise declarations materially improve both performance and correctness. In a
synthetic package with 8,000 unrelated files, package-wide default detection
measured 346.17 ms median for warm no-op checks versus 109.51 ms with one file
and one environment declaration. Unrelated package edits reran the old-style
script but stayed fresh with precise declarations.

Declaration is not completeness. A build script that declared only one trigger
but read another file left stale generated output after the hidden file changed.
The new value appeared only when the declared trigger forced execution.

Rerun precision is also not output precision. When a declared trigger changed,
Cargo rebuilt the owning crate and its application even when the script:

- emitted byte-identical generated Rust;
- preserved the generated file mtime with write-if-changed;
- retained the same `rustc-env` value;
- changed only warning text;
- otherwise emitted the same effective instructions.

Cargo issue 3404 has tracked this missing unchanged-output decision since 2016.
FERRIUM should expose the rerun root and cascade, not claim every cascade is
avoidable. Native discovery, generated configuration, link directives, and
other scripts can have real effects that are not represented by one file.

The output lifecycle has a second gap. `OUT_DIR` is intentionally persistent.
A file remained after the script stopped producing it because Cargo has no
script output manifest and cannot infer ownership. Automatic directory
cleaning would break legitimate persistence; a future output contract must be
script-specific.

The security boundary is equally explicit. The fixture invoked rustc and wrote
outside `OUT_DIR`. Cargo's documentation instructs scripts to write only inside
that directory, but ordinary Cargo execution did not enforce it in this
fixture. A separate process or runner is not a sandbox without capability
restrictions.

The most immediate ecosystem change is Cargo's new nightly
`-Zbuild-analysis`. On the byte-identical trigger case,
`cargo report rebuilds -vv` reported the build-script run as the root and the
owning crate plus application as cascading rebuilds. Its JSONL also preserved
unit identities, fingerprint causes, dependencies, and durations. FERRIUM
should consume and extend this evidence rather than invent a competing private
cause model.

Stable Cargo JSON and repeated wall timing remain the default baseline.
Nightly build analysis is optional and gated by exact Cargo version and schema.
This research is diagnostic-only: rollout changes no Cargo behavior, rollback
is disabling the diagnostic, and support does not include automatic
remediation or source rewriting. The measured platform scope is
x86_64-pc-windows-msvc; no other ABI, SDK, or native-toolchain portability
claim is made.

## Decision supported

This research determines:

- which build-script input, output, rerun, and capability dimensions belong in
  the compiler query plan;
- where ordinary Cargo already supports precise and correct behavior;
- which stale-input and unnecessary-cascade failures diagnostics should expose;
- what Cargo's build-analysis surface already provides;
- which output-manifest, suppression, sandbox, and cache work requires an
  upstream or explicit compatibility boundary.

It does not authorize caching build-script execution, suppressing a rebuild,
cleaning an output directory, rewriting `build.rs`, restricting an existing
script, replacing Cargo, or opening the implementation gate.

## Evidence reviewed

### Local evidence

- [Build-script input, output, and fan-out matrix](perf-q23-build-scripts/results/EXP-01-build-script-input-output-matrix.md)
- [Rust latency telemetry](2026-08-07-rust-latency-telemetry.md)
- [Cargo build-unit identity](2026-08-07-cargo-build-unit-identity.md)
- [Incremental reuse boundaries](2026-08-07-rust-incremental-reuse-boundaries.md)
- [Cross-workspace artifact reuse](2026-08-08-cross-workspace-artifact-reuse.md)
- [Procedural-macro cost, inputs, and reuse](2026-08-08-procedural-macro-cost-input-reuse.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### Cargo sources and documentation

- [Cargo build-script reference](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
- [Cargo fingerprint model and build-script mtime handling](https://github.com/rust-lang/cargo/blob/c79e8f89441b3e73d6d65d125c0c745792808c74/src/compiler/fingerprint/mod.rs)
- [Cargo build-script output model](https://github.com/rust-lang/cargo/blob/c79e8f89441b3e73d6d65d125c0c745792808c74/src/compiler/custom_build.rs)
- [Cargo external dep-info integration](https://github.com/rust-lang/cargo/blob/c79e8f89441b3e73d6d65d125c0c745792808c74/src/compiler/output_depinfo.rs)
- [Cargo build-analysis documentation](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-analysis)
- [Cargo build-analysis tracking issue 15844](https://github.com/rust-lang/cargo/issues/15844)
- [Cargo unchanged build-script output issue 3404](https://github.com/rust-lang/cargo/issues/3404)
- [Cargo broad rerun discovery issue 14240](https://github.com/rust-lang/cargo/issues/14240)
- [Cargo mtime-only symlink failure issue 15134](https://github.com/rust-lang/cargo/issues/15134)
- [Cargo build-script reduction initiative 14948](https://github.com/rust-lang/cargo/issues/14948)
- [Cargo semantic build-script tracking issue 14903](https://github.com/rust-lang/cargo/issues/14903)

### Capability, output, and external-build prior art

- [Cargo build-script sandbox issue 5720](https://github.com/rust-lang/cargo/issues/5720)
- [Cargo custom build-script runner proposal 15672](https://github.com/rust-lang/cargo/issues/15672)
- [Cargo target-local temporary directory issue 16427](https://github.com/rust-lang/cargo/issues/16427)
- [`rules_rust` `cargo_build_script`](https://bazelbuild.github.io/rules_rust/cargo.html#cargo_build_script)

## Current Cargo model

### Compile and run are different units

Cargo compiles `build.rs` as a host executable and runs it as a
`RunCustomBuild` unit. The run unit may exist separately for profiles,
features, or targets. The experiment executed the same script once for debug
and once for release and observed distinct output directories.

Build-script source or build-dependency changes rebuild the host executable.
Declared runtime inputs can rerun the existing executable without recompiling
it.

### Fresh output is replayed

Cargo stores the script's stdout and stderr in the build directory. When the
run unit is fresh, Cargo parses and reuses the saved instructions. Stable
`build-script-executed` JSON therefore describes effective script output but
does not prove the process executed during that invocation.

The warning control made this visible: Cargo displayed the saved warning on a
warm no-op while the external execution log remained unchanged.

### Default and declared freshness are different modes

If a script emits no `rerun-if-changed` or `rerun-if-env-changed`
instructions, Cargo uses package-wide change detection over the package files
selected by include/exclude rules.

Once the script emits any rerun instruction, Cargo tracks only the named paths
and environment variables. This is not an additive hint layered over the
default package scan; it replaces the default runtime dependency set.

Declared file paths use mtime comparison against the saved script output file.
Declared environment variables store and compare the value Cargo received.

## Input precision

### Package-wide defaults are conservative, not universally wrong

The default protects old scripts that declare nothing. It is correct to rerun
when any package file could be an input.

Its cost and false-positive rate grow with package breadth. The 8,000-file
control measured a 3.16 times higher warm no-op median and reran after an
unrelated file changed.

This does not prove every default-mode script should emit one narrow path.
Scripts that discover source trees, native installations, or generated
configuration need a complete practical declaration. An incomplete narrow
declaration is worse than conservative reruns.

### Declared inputs prevent unrelated reruns

The precise fixture stayed fresh after both a content edit and a same-content
rewrite of an unrelated package file. Declared file and environment changes
reran as expected.

The first diagnostic opportunity is therefore read-only:

- identify scripts with no declarations;
- list their package scan roots and file counts;
- list broad directory declarations;
- show the files or environment values that actually dirtied a run;
- recommend review, not an automatic rewrite.

Cargo issue 14240 asks for help discovering overly broad declarations. Cargo
build analysis now supplies part of the root-cause evidence.

### Mtime remains a correctness and performance boundary

`-Zchecksum-freshness` did not suppress the same-content declared-input rerun.
Cargo source documents `RerunIfChanged` as an mtime comparison.

Mtime can create both:

- false positives, such as a same-content rewrite;
- false negatives, such as a symlink switching to another target with the same
  mtime in Cargo issue 15134.

A content-addressed future declaration would need path, symlink, directory,
missing-file, metadata, and performance semantics. FERRIUM should not imply
that hashing bytes alone solves every filesystem dependency.

### Hidden inputs are correctness defects

The hidden-file control retained stale program output until a declared trigger
forced execution.

Build scripts can also observe environment, working directory, tool discovery,
filesystem metadata, subprocesses, clocks, randomness, network, and native
libraries. A declaration inventory can prove known inputs; it cannot prove the
absence of hidden ones without capability enforcement or system-call-level
observation.

Any cache or suppression decision must reject unknown inputs rather than
silently treating them as absent.

## Output precision

### A rerun currently propagates before output equivalence is known

The build-script run is a dependency of the owning crate. When the run unit
became dirty, Cargo marked the crate and application dirty through dependency
fingerprints.

Write-if-changed preserved generated bytes and mtime but did not suppress this
dependency cascade. Cargo issue 3404 requests a mechanism for exactly this
case.

An unchanged-output decision cannot compare only one generated file. The
effective output includes:

- generated files and directories;
- `rustc-link-*` instructions;
- `rustc-cfg` and `rustc-check-cfg`;
- `rustc-env`;
- immediate-dependent metadata;
- warnings and errors;
- externally visible writes or tool effects if unrestricted execution remains
  allowed.

Suppressing the owning crate before this effective output is complete would be
unsound.

### Write-if-changed is useful but not a Cargo freshness mechanism

Preserving a generated file's mtime can reduce work for external tools or
systems that watch that file. It also avoids needless filesystem churn.

It does not currently make Cargo's dependent compile units fresh after the
script run unit changed. FERRIUM diagnostics should report both facts rather
than promise a downstream compile saving from write-if-changed alone.

### `OUT_DIR` persistence requires ownership

The obsolete file survived later executions that stopped producing it.

Cargo does not know whether unknown files are intentionally reused; the
experiment confirmed that it preserves them. A future manifest needs:

- script execution identity;
- declared output paths and types;
- atomic publication state;
- content identity;
- retained-versus-ephemeral policy;
- stale-output removal ownership;
- failure recovery and rollback;
- profile and target separation.

Cleaning all of `OUT_DIR` before every run would discard supported persistence
and is rejected.

## Directive and native fan-out

Changing only warning text reran the script and rebuilt the crate plus
application while preserving runtime output. Changed `rustc-env` and
`rustc-cfg` values correctly changed compiled behavior.

`links` metadata reached only the immediate dependent build script. Changing
the metadata reran that wrapper and rebuilt the transitive application, but
the application build script did not receive the `DEP_` value.

Cargo supports target configuration that overrides a `links` build script. In
the fixture, the override prevented the native build script from being
compiled or run and supplied metadata to the immediate wrapper.

In this fixture, the override was useful for one externally managed native
configuration. It requires target-specific configuration and complete
equivalent link, cfg, environment, and metadata values. It is not an automatic
cache or a general native-installation claim.

## Capability boundary

The build script inherited a package-root working directory, Cargo variables,
Cargo jobserver/concurrency context represented by `NUM_JOBS`, subprocess
capability, and ordinary filesystem access. It successfully wrote outside
`OUT_DIR`.

The experiment did not test every capability, but it matched Cargo's model:
`OUT_DIR` is a convention and output location, not a security sandbox.

Cargo issues 5720 and 15672 describe sandbox and runner directions. A custom
runner can create a place to apply policy, but the runner itself is not the
policy. A credible sandbox contract must name filesystem, environment,
network, process, temporary-directory, toolchain, native-library, IPC, time,
randomness, and platform semantics plus escape behavior and rollback.

Cargo issue 16427's proposal to direct temporary files into the target tree
improves containment and cleanup for well-behaved tools. Its own description
notes that environment variables do not prevent explicit access elsewhere.

## Build analysis and FERRIUM's wedge

Cargo's nightly build-analysis logs already preserve:

- a run identifier and exact command;
- workspace, target directory, host, profile, jobs, and compiler;
- registered units and dependency edges;
- fingerprint status and dirty reason;
- unit start, finish, unblocking, and duration;
- `cargo report rebuilds` root and cascading impact.

This materially changes FERRIUM's opportunity. A private parallel rebuild
schema would duplicate active upstream work and likely diverge.

FERRIUM should:

- use stable Cargo JSON and wall timing as the ordinary baseline;
- optionally ingest versioned nightly build-analysis JSONL;
- add build-script-specific interpretation such as default-mode breadth,
  declaration inventory, hidden-input controls, output ownership, native
  metadata flow, and adoption risk;
- preserve raw unknown causes and schema-version limitations;
- contribute actionable minimized cases upstream.

Build-analysis remains unstable. Issue 15844 lists schema evolution,
programmable output, nested Cargo calls, dirty-reason stability, and
actionability as unresolved.

## Prior art and ecosystem direction

`rules_rust` makes build-script action inputs explicit through source, data,
tool, environment, compile-data, dependency, and toolchain attributes. This
demonstrates a useful hermetic action model and the operational cost of naming
inputs.

It does not transparently convert every Cargo build script into a complete
hermetic action. Compatibility depends on declared data, tools, shell
environment, C/C++ toolchains, working directory, outputs, and Bazel
integration.

Cargo's issue 14948 separately aims to remove common reasons to write scripts.
Issue 14903 tracks semantic and composable build scripts. These upstream
directions are preferable to embedding more general-purpose logic in a
FERRIUM-specific script language.

## Findings

### FERRIUM-284: build-script compilation and execution are separate identities

**Sources:** Cargo fingerprint model and EXP-01 debug/release capability
control.

**Observed behavior:** Cargo reused or rebuilt the host script executable
separately from running it. Debug and release produced distinct run/output
identities.

**Implication:** Reports must separate compile, run, profile, target, feature,
and output-directory identities.

**Confidence:** High.

### FERRIUM-285: package-wide default detection can impose broad scan and rerun cost

**Sources:** Cargo documentation, fingerprint source, issue 14240, and EXP-01
8,000-file control.

**Observed behavior:** The default-mode warm no-op median was 346.17 ms versus
109.51 ms with precise declarations; unrelated changes reran only the default
script.

**Implication:** Inventory default-mode and broad-directory scripts, but do not
automatically narrow them without proving completeness.

**Confidence:** High for behavior; medium for performance generalization.

### FERRIUM-286: any rerun declaration replaces package-wide runtime detection

**Sources:** Cargo documentation, fingerprint source, and EXP-01.

**Observed behavior:** Once declarations existed, unrelated package changes
stayed fresh while declared path and environment changes reran the script.

**Implication:** Treat declarations as the script's explicit runtime
dependency contract, not advisory additions to the default scan.

**Confidence:** High.

### FERRIUM-287: declared file freshness remains mtime-based

**Sources:** Cargo fingerprint source, issue 15134, and EXP-01 checksum
control.

**Observed behavior:** Same-content declared-file rewrites reran with and
without checksum freshness.

**Implication:** Expose mtime false positives and false negatives separately;
do not promise that source checksum freshness fixes build-script paths.

**Confidence:** High.

### FERRIUM-288: declared environment values provide precise rerun edges

**Sources:** Cargo documentation, output model, and EXP-01.

**Observed behavior:** Changing the declared environment value reran the
script; an undeclared environment change did not.

**Implication:** Inventory environment declarations and distinguish Cargo
input from Cargo-generated variables that scripts receive automatically.

**Confidence:** High.

### FERRIUM-289: hidden build-script inputs can leave generated artifacts stale

**Sources:** EXP-01 hidden-file matrix.

**Observed behavior:** A hidden file changed from `1` to `2`, but Cargo ran no
work and the application retained `1` until a declared trigger changed.

**Implication:** Classify hidden-input freshness as a correctness failure, not
a cache hit or optimization.

**Confidence:** High.

### FERRIUM-290: script reruns propagate before effective output equivalence is known

**Sources:** EXP-01 fan-out matrix, Cargo build-analysis output, fingerprint
source, and issue 3404.

**Observed behavior:** Byte-identical stable output still rebuilt the owning
crate and application.

**Implication:** Explain the root and cascade now; prototype suppression only
after defining complete effective output and capability semantics.

**Confidence:** High.

### FERRIUM-291: write-if-changed preserves files but not Cargo compile freshness

**Sources:** EXP-01 stable-write and always-write controls.

**Observed behavior:** Stable mode preserved generated bytes and mtime, while
both stable and always-write modes rebuilt dependent Cargo units after rerun.

**Implication:** Recommend write-if-changed only for filesystem/output hygiene,
not as a guaranteed Cargo rebuild optimization.

**Confidence:** High.

### FERRIUM-292: saved warnings can replay without execution

**Sources:** Cargo output model and EXP-01 warning control.

**Observed behavior:** The warning appeared on a warm no-op while the execution
log did not change.

**Implication:** Do not infer script execution from warnings or
`build-script-executed` output.

**Confidence:** High.

### FERRIUM-293: cfg, environment, link, and metadata outputs have distinct fan-out

**Sources:** Cargo documentation, output model, and EXP-01 directive and links
matrices.

**Observed behavior:** `rustc-env` and `rustc-cfg` changed compiled behavior;
`links` metadata reached the immediate wrapper but not the transitive app
script.

**Implication:** Preserve output kind and receiving edge instead of reporting
one generic "build-script output changed."

**Confidence:** High.

### FERRIUM-294: `OUT_DIR` is persistent and lacks script output ownership

**Sources:** Cargo documentation and EXP-01 stale-output control.

**Observed behavior:** An obsolete file survived later executions that stopped
producing it.

**Implication:** Research per-script output manifests; reject automatic whole
directory cleanup.

**Confidence:** High.

### FERRIUM-295: ordinary build scripts retain broad native capabilities

**Sources:** Cargo documentation, issues 5720, 15672, and 16427, plus EXP-01.

**Observed behavior:** The script spawned rustc and wrote outside `OUT_DIR`.

**Implication:** Process execution and output-directory convention are not a
sandbox. Capability claims require explicit enforcement.

**Confidence:** High.

### FERRIUM-296: `links` overrides provide a supported external-native boundary

**Sources:** Cargo override documentation and EXP-01.

**Observed behavior:** Target configuration prevented the linked package's
script from compiling or running and supplied equivalent metadata.

**Implication:** Diagnose this option for known installations; do not treat it
as transparent or portable without complete target-specific values.

**Confidence:** High.

### FERRIUM-297: Cargo build analysis now exposes rebuild roots and cascades

**Sources:** build-analysis documentation, issue 15844, and EXP-01 JSONL/report
control.

**Observed behavior:** Cargo identified the changed build-script path as the
root, listed two cascading compile units, and recorded unit durations.

**Implication:** Consume this upstream evidence behind a versioned nightly
boundary rather than defining a competing private rebuild schema.

**Confidence:** High.

### FERRIUM-298: explicit action prior art demonstrates both value and migration cost

**Sources:** `rules_rust` `cargo_build_script` documentation.

**Observed behavior:** The rule explicitly names sources, data, tools,
environment, dependencies, working directory, and toolchain integration.

**Implication:** Use it as contract prior art, not evidence that arbitrary
Cargo scripts are already hermetic.

**Confidence:** High on the model; medium on ecosystem migration.

### FERRIUM-299: safe build-script reuse requires input, capability, and output contracts

**Sources:** findings FERRIUM-284 through FERRIUM-298.

**Observed behavior:** Declared inputs alone did not describe hidden reads,
generated output ownership, subprocesses, external writes, or effective
instruction equivalence.

**Implication:** A future cache or unchanged-output decision must bind all
three contracts and reject unknown effects.

**Confidence:** High on the requirement; medium on final design.

### FERRIUM-300: observability should precede build-script optimization

**Sources:** findings FERRIUM-284 through FERRIUM-299 and the role review.

**Observed behavior:** Conservative reruns, stale hidden inputs, mtime limits,
output fan-out, persistent files, native capabilities, and active upstream
telemetry remain separate problems.

**Implication:** Adopt read-only diagnosis and correctness controls now. Defer
cache, suppression, cleanup, and enforcement until representative and
platform-specific contracts exist.

**Confidence:** High.

## Recommendations

### Adopt now

- Add build-script compile identity, run identity, detection mode, declared
  paths and environment, rerun cause, effective outputs, generated-file state,
  `OUT_DIR` lifecycle, native metadata flow, and downstream fan-out to the
  measurement contract.
- Preserve default-mode, precise, same-content, hidden-input, unchanged-output,
  stale-output, directive, `links`, and target-override controls.
- Use minimally instrumented wall time as primary evidence.
- Treat warnings and stable Cargo build-script JSON as replayable output, not
  execution proof.
- Consume nightly Cargo build-analysis JSONL and reports behind an exact
  toolchain/schema compatibility boundary.
- Keep adoption read-only and disable-only: diagnostics do not change Cargo
  behavior, and disabling them is the rollback.
- Contribute minimized Cargo cases when a representative script exposes a
  diagnostic, mtime, propagation, or output-equivalence gap.

### Prototype behind a compatibility boundary

- A read-only build-script declaration and package-scan inventory.
- Broad-path and missing-declaration diagnostics with explicit uncertainty.
- A read-only effective-output and stale-`OUT_DIR` manifest diagnostic.
- Comparison of rerun root versus changed effective output.
- Restricted runner experiments only with named capabilities, target and
  native-tool compatibility, fallback, and rollback.

### Reject or defer

- Caching arbitrary build-script execution.
- Suppressing owning or downstream compilation from file equality alone.
- Treating declarations as proof that hidden inputs do not exist.
- Cleaning all of `OUT_DIR` before or after a script.
- Calling a custom runner, container, or separate process a sandbox without
  capability enforcement.
- Rewriting build scripts or declarations automatically.
- Replacing Cargo or creating a FERRIUM build language.
- Duplicating Cargo's build-analysis schema.
- Opening the FERRIUM implementation gate.

## Role review

| Role | Review disposition |
|---|---|
| Rust Safety Steward | Accepted: hidden inputs remain correctness failures, effective output precedes suppression, persistent output is not cleaned automatically, and no cache or sandbox claim is authorized. |
| Compiler Performance Engineer | Accepted after scoping checksum, capability, build-analysis, and output-lifecycle conclusions to the tested nightly and fixture; primary distributions remain separate from diagnostic timings. |
| Interop Boundary Auditor | Accepted with deferral: the jobserver is described as concurrency context, and ABI, SDK, host/target, native compiler, linker, and cross-platform contracts remain unmeasured outside x86_64 Windows MSVC. |
| AI Assurance Skeptic | Accepted after narrowing the target-override and enforcement language to the measured fixture; synthetic, single-host, replay, and hidden-input limitations remain visible. |
| Ecosystem Strategist | Accepted: Cargo build analysis remains upstream unstable evidence, the canonical taxonomy stays in the measurement contract, and FERRIUM does not duplicate Cargo's schema or create a build language. |
| Rust Maintainer | Accepted for read-only diagnosis: ordinary Cargo remains authoritative, no declaration or source rewrite is automatic, and every speculative output-ownership mechanism remains behind a compatibility boundary. |
| Native Platform Adopter | Accepted for research only: stable evidence is the default, nightly analysis is optional, rollout changes no build behavior, rollback is disable-only, and no unsupported platform or ABI claim is made. |
| Scope Keeper | Accepted after narrowing the question to build-script invocation, inputs, rerun causes, output changes, and ownership; procedural-macro reuse, remote artifacts, and implementation remain separate. |
| Validation Checker | Accepted after aligning Complete status and recording the isolated `CARGO_HOME` command; commands, controls, values, limitations, and finding sequence are preserved. |

## Non-goals

- Claim every build-script rerun is unnecessary.
- Claim the 8,000-file synthetic ratio represents ecosystem packages.
- Claim write-if-changed prevents Cargo compilation.
- Claim checksum freshness covers declared build-script paths.
- Claim `OUT_DIR` should be empty between executions.
- Claim a `links` override is portable without target-specific configuration.
- Claim ordinary process separation is sandboxing.
- Design a production cache key, output manifest, or sandbox protocol.
- Open the FERRIUM implementation gate.
