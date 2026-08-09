# EXP-01: Complete and Incremental Linker Matrix

Date: 2026-08-09
Question: PERF-Q29
Status: Complete

## Decision tested

Determine when Windows native linking is a material Rust iteration cost,
whether `rust-lld` improves complete links, whether MSVC incremental linking
improves repeated links, and which Rust object-identity, optimization, debug,
storage, and correctness constraints govern safe adoption.

## Environment

- Windows build 26310 on local NTFS
- Intel Core i7-12800HX
- 16 physical cores, 24 logical processors
- 31.7 GiB memory
- `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`
- LLVM 23.1.0
- target `x86_64-pc-windows-msvc`
- MSVC linker 14.44.35228.0
- `rust-lld` from the pinned nightly toolchain

Power mode and antivirus observer effects were not isolated and remain
unknown.

## Fixtures

### Tier 0 synthetic control

The PERF-Q26 2,712-line fixture was reused:

```text
105,277 bytes
SHA-256 1c69c22480b78b8ddcd0ce39a5a14c1d950aa983b3131711ad99a2e2bd083e80
```

The matrix crossed:

```text
debuginfo = line-tables-only | full
codegen-units = 1 | 16
linker = link.exe | rust-lld | link.exe /INCREMENTAL
```

Complete and unchanged incremental rows used three repetitions after one
preparation run.

### Tier 1 public control

[FLETCH](https://github.com/giodl73-repo/FLETCH) was measured at:

```text
361fb7eda730bf660de92e47d502b91dcfaf473f
```

The public revision did not expose a committed lockfile in the disposable
clone. Cargo generated:

```text
Cargo.lock SHA-256
daac4b800c86a147724d19c128e25e601f50e02a290086fa604565ca6c6e11a8
```

The selected `fletch-cli` binary links a dependency-heavy Windows application
with HTTP, async, TLS, CLI, serialization, and workspace-library inputs. Its
captured link contained 181 object inputs plus Rust and native libraries.
Primary complete and unchanged incremental rows used five repetitions.

## Link isolation method

The pinned nightly supports:

```powershell
rustc +nightly <source> -Zno-link -o <output>
rustc +nightly -Zlink-only <bundle.rlink>
```

`-Zno-link` emitted an `.rlink` bundle and native objects. Link-only execution
removed transient object inputs after the link, so passive timing was not
sufficient for replay.

The experiment supplied a temporary linker proxy at link-only execution. The
proxy:

1. captured the exact response file, environment, and working directory;
2. copied native object and generated NATVIS inputs before execution;
3. forwarded the unmodified command to MSVC `link.exe`;
4. restored consumed objects for isolated direct-link repetitions; and
5. rewrote only output, PDB, ILK, linker engine, and explicitly named policy
   controls for each comparison.

The primary timer covered the native linker process tree, not rustc frontend or
object generation.

The `.rlink` probe did not preserve an earlier custom linker choice or tested
custom `-Clink-arg`; link-only used the target default unless the choice was
supplied again. The format is therefore treated as an unstable isolation aid,
not a durable complete link-plan contract.

## FLETCH complete-link matrix

| Linker | Wall median ms | MAD ms | CPU median ms | Peak RSS MiB | EXE bytes | PDB bytes |
|---|---:|---:|---:|---:|---:|---:|
| MSVC full | 749.1 | 19.3 | 1,671.9 | 722.3 | 10,763,264 | 85,045,248 |
| `rust-lld` full | 716.2 | 7.0 | 1,343.8 | 508.8 | 10,762,240 | 85,295,104 |

`rust-lld` shortened the matched complete-link median 4.4% and reduced peak
process-tree RSS 29.6%. The executable sizes were effectively equal. The PDB
was slightly larger in this case.

All three produced executables completed `--help` successfully and exposed the
same command help after normalizing the executable name. This is a smoke
control, not exhaustive behavioral, ABI, debugger, unwind, or reproducibility
validation.

## FLETCH MSVC incremental matrix

The ordinary rustc development link requested:

```text
/OPT:REF,NOICF
/DEBUG
```

Microsoft documents `/OPT:REF` as incompatible with `/INCREMENTAL`. A forced
incremental request under the ordinary command emitted `LNK4075` and performed
a full link. The prepared comparison therefore changed the policy to:

```text
/OPT:NOREF,NOICF
/DEBUG
/INCREMENTAL
/ILK:<isolated>
/VERBOSE:INCR
```

| State | Wall ms | CPU ms | Peak RSS MiB | EXE bytes | PDB bytes | ILK bytes |
|---|---:|---:|---:|---:|---:|---:|
| Initial prepared link | 680.9 | 1,703.1 | 724.7 | 19,646,464 | 105,132,032 | 53,204,694 |
| Unchanged median | 183.7 | 171.9 | 111.0 | 19,646,464 | 113,684,480 | 53,204,774 |
| Missing ILK fallback | 711.1 | 1,171.9 | 550.3 | 19,646,464 | 113,684,480 | 53,204,694 |

The unchanged replay reported:

```text
LINK : 0 new modules and 0 (out of 610) modules have changed since prior linking
```

It was 75.5% faster than the ordinary MSVC complete-link median. That speed
required a prepared image and database:

- executable bytes increased 82.5%;
- PDB bytes increased 33.7% in the unchanged state; and
- the ILK added 53.2 MB.

Deleting the ILK produced a full-link-scale result and recreated the database.
Microsoft also documents full-link fallback for a missing output image,
changed output or ILK timestamp, changed linker options, and added or omitted
objects.

## Rust body-edit control

The fixture added one behavior-preserving black-box operation immediately
after CLI parsing, then regenerated the no-link outputs with the same Cargo
target root and profile.

The one source edit changed the complete root-object path set:

```text
old object paths: 181
new object paths: 182
shared paths: 0
removed paths: 181
added paths: 182
```

The CGU filename suffix and content-derived names changed across the crate.
Replaying the existing incremental image with the new response file reported:

```text
LINK : 182 new modules and 181 (out of 610) modules have changed since prior linking
LINK : too many modules have changed since prior linking; performing full link
```

The operation took 910.4 ms, slower than the ordinary 749.1 ms full-link
median. The source was then restored and the disposable fixture returned to
its pinned revision.

This is the decisive negative result. MSVC incremental linking can make an
unchanged native link much faster, but the measured ordinary Rust body edit
did not preserve the object identities required to realize that benefit.

## Synthetic controls

| Debug / CGU | Objects | MSVC full ms | `rust-lld` ms | Incremental initial ms | Incremental unchanged ms |
|---|---:|---:|---:|---:|---:|
| Line tables / 1 | 3 | 82.9 | 84.1 | 113.5 | 54.6 |
| Line tables / 16 | 18 | 101.2 | 90.5 | 101.3 | 71.8 |
| Full / 1 | 3 | 82.7 | 83.7 | 128.7 | 52.4 |
| Full / 16 | 18 | 87.3 | 90.2 | 106.2 | 60.4 |

The small fixture exposed startup and process granularity more than scalable
linker throughput. Full debug increased final EXE and PDB bytes, and sixteen
CGUs increased object count, but neither produced a monotonic dominant
complete-link cost. The dependency-heavy FLETCH result is the stronger
representative linker signal.

The synthetic prepared incremental outputs were approximately 3.8 to 4.1
times the ordinary executable size and added 2.68 to 2.75 MB ILK files. This
reinforces that incremental preparation is a policy and storage trade, not a
free linker switch.

## Ecosystem controls

### MSVC `link.exe`

Microsoft documents functionally equivalent but larger incrementally prepared
images, padding, possible jump thunks, ILK state, fallback conditions, and the
requirement to produce final release images non-incrementally.

### LLVM `lld-link`

`lld-link` supports COFF/PE linking and full PDB production. LLVM documents
that it does not support `/DEBUG:FASTLINK`; the open incremental-linking issue
remains unresolved. It is a complete-link alternative, not MSVC-style
stateful incremental linking.

### Wild

Wild is a Rust linker focused on fast iterative development. Its current
project documentation states that incremental linking is the end goal but is
not implemented. It currently supports Linux targets and lists Windows
support as unimplemented.

### mold

mold is a highly parallel fast complete linker for Unix object formats. It
does not provide the Windows COFF/PDB path measured here. Wild's project
documentation also records that mold does not implement incremental linking
and that its author does not intend to.

## Negative controls and failures retained

- `-Zlink-only` consumed transient object inputs; replay required capture and
  restoration.
- Passive child-process capture missed temporary generated inputs and was
  rejected in favor of an explicit proxy.
- `.rlink` did not preserve the tested custom linker or custom link argument.
- `/OPT:REF` caused MSVC to ignore `/INCREMENTAL` and perform a full link.
- Missing ILK state produced a full-link-scale operation.
- One ordinary Rust body edit renamed the complete root-object set and forced
  a full link.
- `rust-lld` required explicit Windows linker flavor when replayed directly.
- The host did not provide interactive PDB debugger validation.

## Limitations

- Primary evidence is one Windows MSVC host.
- FLETCH is one dependency-heavy public executable.
- The source-edit control changed one body but did not sweep edit shapes,
  crate sizes, incremental policies, LTO, native libraries, or proc macros.
- Linker outputs passed only a CLI smoke test.
- PDB stream equivalence, debugger stepping, locals, types, unwind, panic,
  crash dumps, native mixed stacks, signing, deterministic output, and release
  runtime were not validated.
- CPU samples are process-polling estimates and are weaker than wall medians.
- The synthetic matrix used three repetitions and is a control, not a
  prevalence claim.
- `.rlink`, `-Zno-link`, and `-Zlink-only` are unstable compiler surfaces.

## Reproducibility summary

Primary object generation:

```powershell
cargo +nightly rustc -p fletch-cli --bin fletch-cli -- -Zno-link
```

Link isolation:

```powershell
rustc +nightly -Zlink-only <fletch_cli.rlink> -Clinker=<capture-proxy>
```

Direct complete-link replay used the captured response file with isolated
`/OUT` and `/PDB` paths. The LLVM row replaced the program with:

```powershell
rust-lld.exe -flavor link @<response>
```

The incremental row replaced `/OPT:REF,NOICF` with `/OPT:NOREF,NOICF` and
added:

```text
/INCREMENTAL
/ILK:<isolated>
/VERBOSE:INCR
```

Every public result records the pinned revision, toolchain, command policy,
cache state, wall distribution, resources, output bytes, fallback diagnostics,
and limitations.
