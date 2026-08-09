# EXP-01: Debug Information and Object Emission Matrix

Date: 2026-08-09
Question: PERF-Q28
Status: Complete

## Decision tested

Determine how Rust debug-information levels affect backend latency, process
resources, native object and archive bytes, incremental storage, linker input,
PDB output, and available debugging evidence on Windows MSVC without treating
object emission and linking as one timer.

## Environment

- Windows 11 Enterprise Insider Preview 10.0.26310
- NTFS local workspace
- Intel Core i7-12800HX
- 16 physical cores, 24 logical processors
- 31.7 GiB memory
- `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`
- LLVM 23.1.0
- target `x86_64-pc-windows-msvc`
- `llvm-readobj`, `llvm-size`, and `llvm-ar` from the pinned toolchain

Power mode was not captured and remains unknown.

## Evidence tiers

### Tier 0 synthetic control

The PERF-Q26 2,712-line fixture contains scalar, floating-point, loop, branch,
match, and generic code. Its source identity was:

```text
105,277 bytes
SHA-256 1c69c22480b78b8ddcd0ce39a5a14c1d950aa983b3131711ad99a2e2bd083e80
```

Primary object-only and complete-link measurements used five isolated
repetitions. Debug-level order rotated and reversed across rounds. The common
direct-rustc policy was:

```text
--edition=2024
-Copt-level=0
-Ccodegen-units=1
-Cembed-bitcode=no
-Zthinlto=no
```

Object-only commands used `--emit=obj --out-dir <isolated>`. Link commands
used `-o <isolated>\fixture.exe`. Object-only and complete-link durations are
reported independently and are not subtracted to manufacture a linker timer.

### Tier 1 public control

[METIS-CORE](https://github.com/giodl73-repo/METIS-CORE) was measured at:

```text
78ae34090e043e79a206f2daffaa3889389b4790
```

The disposable clone generated its lockfile offline:

```text
SHA-256 1f4fd21fe5fb1ffd141cc6fe5956e71837159c5b5b4309b30c34c0d9f3577cdd
```

Five isolated clean Cargo builds were collected for each effective development
debug value:

```text
0
line-tables-only
limited
full
```

The value was supplied through `CARGO_PROFILE_DEV_DEBUG`. Cargo's ordinary
development incremental policy remained enabled. Target bytes were classified
as build-script, incremental, Rlib, PDB, executable, object, metadata, and
other bytes.

## Debug-level semantics

Pinned rustc documents:

- `0` or `none`: no crate debug information;
- `line-tables-only`: filename and line information without variable or
  parameter information;
- `1` or `limited`: debug information without type or variable-level
  information;
- `2` or `full`: full debug information.

Cargo's development profile defaults to `2` or `full`. On MSVC targets, rustc
selects CodeView/PDB debug information and supports `packed` split debuginfo as
the stable target mode.

## Synthetic object-only matrix

| Debug level | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB | Object bytes | `.debug$S` bytes | `.debug$T` bytes |
|---|---:|---:|---:|---:|---:|---:|---:|
| None | 273.1 | 1.9 | 234.4 | 84.9 | 244,840 | 0 | 0 |
| Line tables | 318.3 | 17.4 | 265.6 | 87.0 | 464,128 | 160,928 | 14,140 |
| Limited | 316.7 | 16.2 | 281.2 | 85.3 | 464,128 | 160,928 | 14,140 |
| Full | 362.7 | 5.0 | 296.9 | 89.0 | 859,598 | 341,532 | 103,792 |

Relative to no crate debuginfo:

- line tables increased primary wall time 16.6% and object bytes 89.6%;
- limited increased wall time 16.0% and produced the same measured CodeView
  sections and total object bytes as line tables;
- full increased wall time 32.8%, CPU 26.7%, peak RSS 4.8%, and object bytes
  251.1%.

Full debug sections accounted for 445,324 bytes, or 51.8% of the object. The
complete object increase was larger than the named debug-section increase.
The remaining bytes were outside the named debug-section payloads, including
changed non-debug sections and object-container, relocation, and symbol
overhead that this experiment did not fully partition. Debug-section size
alone is therefore not total emission cost.

`llvm-readobj --codeview` found:

| Debug level | Procedure records | Local records | Line records | Type records | File checksums |
|---|---:|---:|---:|---:|---:|
| None | 0 | 0 | 0 | 0 | 0 |
| Line tables | 586 | 0 | 4,441 | 510 | 52 |
| Limited | 586 | 0 | 4,441 | 510 | 52 |
| Full | 586 | 8,626 | 4,527 | 2,532 | 55 |

These counts establish emitted CodeView capability, not interactive debugger
quality. Line tables and limited were indistinguishable on this fixture and
toolchain; that result is not generalized to other targets or formats.

## Diagnostic backend regions

One separate self-profile capture per endpoint corroborated the primary
result. These diagnostics are observer-affected and are not substituted for
the five-repetition wall-clock matrix.

| Self-profile event | None self ms | Full self ms |
|---|---:|---:|
| `codegen_module` | 9.6 | 21.0 |
| `LLVM_module_optimize` | 7.0 | 13.8 |
| `LLVM_module_codegen_emit_obj` | 46.3 | 102.9 |
| `LLVM_passes` | 55.0 | 122.0 |

The named LLVM object-emission event more than doubled under full debuginfo.
A separate time-pass probe showed `codegen_to_LLVM_IR` rising from 8.5 ms to
23.5 ms and `LLVM_passes` from 54.7 ms to 120.4 ms.

The debug cost is therefore not only a final filesystem write. Type and
location metadata are created during IR translation, preserved and processed
through LLVM, lowered into CodeView records, emitted into COFF objects, and
then consumed by the native link/PDB pipeline.

## Complete-link matrix

| Debug level | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB | EXE bytes | PDB bytes | All final bytes |
|---|---:|---:|---:|---:|---:|---:|---:|
| None | 421.9 | 24.0 | 390.6 | 124.0 | 236,544 | 1,372,160 | 1,608,704 |
| Line tables | 458.4 | 23.4 | 406.2 | 126.2 | 236,544 | 1,560,576 | 1,797,120 |
| Limited | 429.8 | 5.9 | 375.0 | 124.1 | 236,544 | 1,560,576 | 1,797,120 |
| Full | 488.5 | 36.4 | 500.0 | 130.2 | 270,336 | 1,822,720 | 2,093,056 |

The complete command includes compilation and linking. Its wall distribution
does not isolate the linker. A separate time-pass diagnostic did:

| Region | None ms | Full ms |
|---|---:|---:|
| `LLVM_passes` | 52.5 | 118.9 |
| `run_linker` | 92.9 | 93.5 |
| `link_binary` | 111.0 | 114.1 |

On this fixture, nearly all of the full-debug difference appeared before or
during object emission. The measured native linker region changed by less than
one millisecond at `run_linker` granularity.

The no-crate-debuginfo link still produced a 1.31 MiB PDB and a CodeView debug
directory entry naming `fixture.pdb`. Precompiled dependencies, native runtime
inputs, public symbols, and linker-produced records prevent total PDB bytes
from being attributed to the current crate. The useful current-crate signal is
the matched delta: line tables added 188,416 PDB bytes and full added 450,560
bytes over the no-crate-debuginfo baseline.

## Codegen-unit control

| Configuration | Wall median ms | Object count | Object bytes | Debug-section bytes |
|---|---:|---:|---:|---:|
| CGU 1, none | 264.3 | 1 | 244,840 | 0 |
| CGU 16, none | 254.1 | 16 | 268,382 | 0 |
| CGU 1, full | 365.9 | 1 | 859,598 | 445,324 |
| CGU 16, full | 309.3 | 16 | 1,033,008 | 592,912 |

Sixteen CGUs shortened the full-debug object-only median 15.5% through backend
parallelism, while increasing object bytes 20.2% and debug-section bytes 33.1%.
Debug guidance must preserve both latency and storage/link-input effects; CGU
parallelism is not a free reduction in emission work.

## Split, strip, and temporary-file controls

The stable command surface on nightly rustc rejected MSVC
`split-debuginfo=off` and `unpacked`:

```text
error: `-Csplit-debuginfo=off` is unstable on this platform
error: `-Csplit-debuginfo=unpacked` is unstable on this platform
```

With nightly `-Zunstable-options`, `off` still produced the same EXE and PDB
sizes as packed mode. `unpacked` produced the same EXE and PDB plus an
859,610-byte retained object. Neither is a portable stable alternative to the
target's packed PDB model.

`-Cstrip=debuginfo` and `-Cstrip=symbols` produced the same measured full-debug
EXE and PDB bytes as the unstripped packed control. This agrees with current
rustc documentation that stripping no longer suppresses MSVC PDB production.
Strip is a final-link policy, not an avoided-debug-generation policy.

`-Csave-temps=yes` retained 862,652 object bytes and 1,078,648 other temporary
bytes in addition to the normal EXE and PDB. Saved temporaries are a diagnostic
mode with their own storage and observer effects.

## Public METIS-CORE matrix

| Debug level | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB | Target bytes | Incremental bytes | Root Rlib bytes |
|---|---:|---:|---:|---:|---:|---:|---:|
| None | 9,913.3 | 832.1 | 8,015.6 | 408.4 | 42,925,647 | 10,492,788 | 3,441,176 |
| Line tables | 11,700.1 | 318.9 | 7,984.4 | 460.6 | 48,850,064 | 13,311,698 | 4,958,844 |
| Limited | 11,757.1 | 756.4 | 8,187.5 | 445.4 | 48,848,237 | 13,311,137 | 4,958,288 |
| Full | 12,080.9 | 449.1 | 8,984.4 | 426.1 | 69,873,116 | 20,379,864 | 11,867,068 |

Relative to no crate debuginfo, full debug:

- increased clean-build wall time 21.9%;
- increased CPU 12.1%;
- increased total target storage 62.8%;
- increased incremental-directory bytes 94.2%;
- increased the root Rlib 244.9%.

The root Rlib contained 139 objects in every mode. Its CodeView evidence was:

| Debug level | Debug-section bytes | Procedure records | Local records | Line records | Type records |
|---|---:|---:|---:|---:|---:|
| None | 0 | 0 | 0 | 0 | 0 |
| Line tables | 1,233,316 | 4,528 | 0 | 17,788 | 3,699 |
| Limited | 1,232,760 | 4,528 | 0 | 17,788 | 3,699 |
| Full | 7,635,516 | 4,528 | 34,062 | 18,988 | 74,645 |

Full debug information occupied 64.3% of the root Rlib. The target-wide storage
increase was smaller because dependency metadata, build outputs, fingerprints,
and other non-debug artifacts remained.

The none-build wall MAD was 8.4%; all primary METIS rows remained below the
contract's 10% instability threshold. CPU and RSS did not move monotonically,
so storage and wall conclusions are stronger than a universal memory claim.

## Debugger control

The pinned Windows rustup toolchain reported:

```text
error: the 'rust-lldb.exe' binary ... is not applicable to the
'nightly-x86_64-pc-windows-msvc' toolchain
```

`cdb`, `lldb`, `llvm-pdbutil`, and `dumpbin` were not available on the host.
`llvm-readobj` verified CodeView sections and executable PDB identity, but it
did not inspect PDB streams or execute source breakpoints.

Interactive breakpoint, local-variable, type-rendering, optimized-frame, panic,
and mixed-language debugger usability therefore remain unmeasured. The
CodeView record counts are capability proxies only.

## Limitations

- Primary measurements use one x86_64 Windows MSVC system.
- The synthetic fixture is intentionally backend-heavy.
- METIS-CORE is one public library and does not produce a final executable in
  the measured command.
- Cargo development incremental state was included in the public clean target
  storage; this is deliberate but does not isolate non-incremental archive
  bytes.
- PDB stream contents and interactive debugger behavior were not inspected.
- Line-tables and limited equivalence is one target/toolchain result.
- Self-profile and time-pass diagnostics are observer-affected single probes.
- Antivirus, filesystem cache, power mode, and concurrent machine load were
  not independently controlled.
- Linux DWARF, split DWARF, macOS dSYM, Windows GNU, embedded targets, dynamic
  libraries, FFI, release optimization, and test-harness links were not
  measured.
- PERF-Q29 retains broader linker and incremental-linking decisions.

## Reproducibility surface

Representative commands:

```powershell
rustc +nightly main.rs --edition=2024 --crate-name=perf_q28_fixture `
  -Copt-level=0 -Ccodegen-units=1 -Cembed-bitcode=no -Zthinlto=no `
  -Cdebuginfo=full --emit=obj --out-dir <isolated>

rustc +nightly main.rs --edition=2024 --crate-name=perf_q28_fixture `
  -Copt-level=0 -Ccodegen-units=1 -Cembed-bitcode=no -Zthinlto=no `
  -Cdebuginfo=full -Csplit-debuginfo=packed -o <isolated>\fixture.exe

llvm-readobj --sections --codeview <object>
llvm-readobj --coff-debug-directory <executable>
llvm-ar x <root-rlib>

$env:CARGO_PROFILE_DEV_DEBUG = "full"
cargo +nightly build --manifest-path <metis>\Cargo.toml `
  --target-dir <isolated> --offline --locked
```

CPU is summed process-tree CPU time. Peak RSS is the largest sampled
process-tree resident set. Every primary row reports all successful samples,
median, and median absolute deviation.
