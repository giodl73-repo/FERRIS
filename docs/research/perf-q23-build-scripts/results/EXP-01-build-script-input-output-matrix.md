# EXP-01: Build-Script Input, Output, and Fan-Out Matrix

Date: 2026-08-09
Question: PERF-Q23
Environment: Windows 11 Enterprise, x86_64-pc-windows-msvc
Rust: rustc 1.99.0-nightly (1a98b1e13 2026-08-07)
Cargo: cargo 1.99.0-nightly (c79e8f894 2026-08-04)

## Purpose

This experiment isolates six build-script questions:

1. How much work does Cargo's package-wide default detection add compared with
   precise `rerun-if-*` declarations?
2. Do same-content rewrites remain build-script changes when checksum
   freshness is enabled?
3. What happens when a build script reads an undeclared input?
4. Does byte-identical output prevent compilation after a script reruns?
5. How do warnings, `rustc-env`, `rustc-cfg`, and `links` metadata propagate?
6. What output-lifecycle and capability boundaries does ordinary Cargo
   enforce?

The fixture is synthetic. It does not estimate the cost of C or C++ compiler
discovery, bindgen, pkg-config, native linking, schema generation, platform
SDK probing, or large ecosystem build scripts.

## Fixture

The disposable harness generated isolated workspaces containing:

- a build-script package and dependent application;
- generated Rust included from `OUT_DIR`;
- fixed external execution logs written only by actual script executions;
- 8,000 unrelated package files for the package-scan control;
- declared file and environment inputs;
- one intentionally hidden file input;
- stable-write and always-rewrite output modes;
- warning, `rustc-env`, and `rustc-cfg` controls;
- a `links` package, immediate wrapper, and transitive application;
- a capability control that invoked rustc and wrote outside `OUT_DIR`.

Commands used `--locked --offline` and Cargo JSON. Build-script execution was
established from the external execution log, not from
`build-script-executed`, because Cargo replays that JSON message for fresh
scripts.

## Package-wide default versus precise declarations

Each workspace contained 8,000 unrelated package files. Five warm no-op
checks were measured after the initial build.

| Detection mode | Warm no-op median, ms | MAD, ms | Unrelated edit reran script | Declared input edit reran script | Environment edit reran script |
|---|---:|---:|---:|---:|---:|
| No `rerun-if-*` declarations | 346.17 | 34.04 | Yes | Yes | No |
| Precise file and environment declarations | 109.51 | 8.03 | No | Yes | Yes |
| Precise declarations plus `-Zchecksum-freshness` | 99.15 | 1.09 | No | Yes | Yes |

This is a synthetic scan-cost control, not a general speedup claim. The
package-wide default was 3.16 times the precise median in this one file-heavy
fixture. A real package's result depends on package contents, include/exclude
rules, filesystem behavior, and other Cargo work.

The behavioral boundary was unambiguous:

- with no declarations, editing or same-content rewriting an unrelated package
  file reran the script and rebuilt the generator and application;
- after any `rerun-if-*` declaration, unrelated package changes did not rerun
  the script;
- a declared file edit and declared environment-value change reran the script;
- changing an undeclared environment value did not rerun the old-style script.

## Same-content rewrites and checksum freshness

| Mode | Declared input changed content | Declared input same-content rewrite |
|---|---:|---:|
| Ordinary freshness | Script reran | Script reran |
| `-Zchecksum-freshness` | Script reran | Script reran |

In the tested nightly toolchain, checksum freshness did not extend to the
build-script `rerun-if-changed` dependency. Cargo source documents the
new-style build-script fingerprint as comparing declared path mtimes against
the saved build-script output file mtime. The checksum option applies to rustc
dep-info freshness, not this `RerunIfChanged` variant.

This differs from the earlier source-file checksum control in PERF-Q20, where
an identical Rust source rewrite stayed fresh.

## Hidden-input correctness

The build script declared only `trigger.txt` but read `hidden.txt` to generate
the application's value.

| Step | Script executed | Dirty compile artifacts | Program output |
|---|---:|---|---:|
| Initial build with hidden value `1` | Yes | script, generated crate, app | `1` |
| Change hidden value to `2` | No | none | stale `1` |
| Change declared trigger | Yes | generated crate, app | `2` |

The stale output was a missed dependency, not reusable work. The script read
the new value correctly as soon as a declared input forced execution.

## Unchanged output and downstream fan-out

The fan-out workspace declared `input.txt`, `trigger.txt`, and `mode.txt`.
Stable mode wrote generated Rust only when bytes changed. Always mode rewrote
the same bytes on every execution.

| Step | Script executed | Generated file mtime changed | Generated crate compiled | App compiled |
|---|---:|---:|---:|---:|
| Warm no-op | No | No | No | No |
| Trigger edit, stable byte-identical output | Yes | No | Yes | Yes |
| Input edit, changed generated value | Yes | Yes | Yes | Yes |
| Switch to always-write, same generated value | Yes | Yes | Yes | Yes |
| Trigger edit, always rewrite same bytes | Yes | Yes | Yes | Yes |

Write-if-changed preserved the generated file mtime, but it did not make the
owning Cargo run unit or dependent compile units fresh. Once the script ran,
Cargo rebuilt the generated crate and application even when:

- generated bytes were unchanged;
- the generated file mtime was unchanged;
- `cargo::rustc-env` retained the same value;
- every emitted instruction except the previously declared rerun cause was
  unchanged.

A following no-op was fresh, so the matrix did not enter a permanent rebuild
loop.

The final `cargo check` to `cargo build` transition did not rerun the script,
but it compiled the ordinary codegen artifacts required for build mode. The
program printed `2:2`.

## Cargo build analysis

The same byte-identical trigger case was repeated with nightly
`-Zbuild-analysis`. `cargo report rebuilds -vv` reported:

```text
Status: 3 units rebuilt, 1 cached, 0 new

Rebuild impact:
  root rebuilds: 1 unit
  cascading:     2 units

Root rebuilds:
  0. generated@0.1.0 build-script (run): file modified: generated\trigger.txt
     impact: 2 dependent units rebuilt
       - app@0.1.0 app "bin" (check)
       - generated@0.1.0 (check)
```

The persisted JSONL recorded:

| Unit | Status/cause | Duration, ms |
|---|---|---:|
| Build-script run | changed declared file | 53.52 |
| Generated library check | stale dependency fingerprint | 119.66 |
| Application check | stale dependency fingerprint | 105.80 |
| Compiled build-script binary | fresh | not executed |

These are one observer-enabled diagnostic run, not primary timing samples.
They demonstrate that Cargo's new build-analysis surface can identify both the
root rerun cause and the dependent compile cascade.

## Output-directive fan-out

| Step | Script executed | Directed crate compiled | App compiled | Program output |
|---|---:|---:|---:|---|
| Initial | Yes | Yes | Yes | `one:one` |
| Warm no-op | No | No | No | `one:one` |
| Warning text only | Yes | Yes | Yes | `one:one` |
| `rustc-env` value | Yes | Yes | Yes | `two:one` |
| `rustc-cfg` value | Yes | Yes | Yes | `two:two` |

Cargo replayed the saved warning on the warm no-op even though the script did
not execute. Warning text therefore is not execution evidence.

Changing only the warning still rebuilt the crate and application because the
declared warning file first dirtied the build-script run. Cargo currently has
no post-execution unchanged-output decision that suppresses the compile
cascade.

Changed `rustc-env` and `rustc-cfg` values correctly changed the compiled
program and required downstream work.

## Persistent `OUT_DIR`

The build script created `obsolete.bin` in `OUT_DIR`, then changed modes so it
no longer wrote or referenced that file.

| Step | `obsolete.bin` exists |
|---|---:|
| Script writes the file | Yes |
| Later script execution omits it | Yes |
| Following warm no-op | Yes |
| Check-to-build transition | Yes |

Cargo preserved the directory and did not infer an output manifest. The file
belonged to the script, so the script would need to remove it explicitly if it
became invalid.

This is an output-ownership gap, not justification to clean all of `OUT_DIR`.
Persistence is intentional and may be required by build scripts.

## Capability and execution identity

The capability control observed:

- current working directory equal to the package root;
- `OUT_DIR`, `TARGET`, `HOST`, `PROFILE`, and `NUM_JOBS=24`;
- successful execution of the rustc subprocess;
- successful write to a lab path outside `OUT_DIR`;
- distinct debug and release executions with distinct `OUT_DIR` paths.

The build script wrote `kept.txt` inside each output directory and wrote an
explicit marker outside it. Ordinary Cargo did not restrict either process
execution or the out-of-directory write.

This does not test every operating-system capability and is not a security
exploit. It confirms that Cargo's documented "write only to `OUT_DIR`"
guidance is not an enforcement boundary.

## `links` metadata

The `native-sys` package declared `links = "q23native"` and emitted
`VERSION`. Its immediate wrapper read `DEP_Q23NATIVE_VERSION` and exposed it
through `rustc-env`. The transitive application build script checked for the
same variable.

| Step | Scripts executed | Dirty compile artifacts | Program output |
|---|---|---|---|
| Initial | sys `1`, wrapper `1`, app `missing` | all | `1:missing` |
| Warm no-op | none | none | `1:missing` |
| Native metadata `1` to `2` | sys `2`, wrapper `2` | sys, wrapper, app | `2:missing` |

Metadata reached the immediate dependent build script but not the transitive
application build script.

The matrix then added:

```toml
[target.x86_64-pc-windows-msvc.q23native]
VERSION = "override"
```

After an isolated target rebuild:

- the `native-sys` build script was neither compiled nor run;
- the wrapper read `override`;
- the application still did not receive the transitive `DEP_` variable;
- program output was `override:missing`.

In this fixture, target configuration acted as a boundary for the tested
native-installation case, not a general replacement for build scripts.

## Reproduction command shape

The harness generated every workspace from scratch, then used:

```powershell
cargo +nightly generate-lockfile --offline
cargo +nightly check -p app --locked --offline `
  --message-format=json-render-diagnostics
cargo +nightly build -p app --locked --offline `
  --message-format=json-render-diagnostics
```

The checksum control added:

```powershell
-Z checksum-freshness
```

The rebuild-analysis control used an isolated `CARGO_HOME` and:

```powershell
$env:CARGO_HOME = Join-Path $fixtureRoot "analysis-cargo-home"
cargo +nightly check -p app -Zbuild-analysis `
  --config build.analysis.enabled=true --offline
cargo +nightly report rebuilds -Zbuild-analysis `
  --config build.analysis.enabled=true -vv
```

Primary execution evidence came from an external append-only log written by
the script. Cargo JSON supplied compile artifact freshness and saved
build-script output. Program execution checked correctness where outputs were
observable.

## Limitations

- The experiment ran on one Windows host and one nightly.
- No C/C++ compiler, linker, SDK, pkg-config, bindgen, or network access was
  measured.
- The 8,000-file scan control is synthetic and its timings are
  filesystem-specific.
- The declared-input same-content control changes mtime on NTFS; coarse or
  unusual filesystems can behave differently.
- The build-analysis durations are single diagnostic observations.
- The out-of-directory write proves lack of enforcement only for that path and
  process context.
- No production sandbox, remote cache, output manifest, or cleanup mechanism
  was implemented.
- The target override used the host triple only.
- Cargo build-analysis and checksum freshness are unstable.

## Result

Build-script precision has three separate boundaries:

1. **Rerun precision:** declarations replace package-wide scanning and prevent
   unrelated changes from executing the script.
2. **Correctness precision:** undeclared inputs can leave generated output
   stale.
3. **Output precision:** Cargo does not currently compare a rerun script's
   effective output before rebuilding dependent units or track the script's
   complete output ownership.

Precise declarations and read-only diagnostics are safe immediate work.
Output manifests, unchanged-output suppression, restricted execution, and
cacheability require explicit compatibility and correctness contracts.
