# EXP-01: Partial Dependency Compilation Matrix

Date: 2026-08-09

Question: what work does Rust's current `hint-mostly-unused` mechanism avoid or
move, when does that improve end-to-end builds, and how does it differ from
full crate slicing?

## Environment

- Windows 11 build 26310 on local NTFS
- Intel Core i7-12800HX, 16 cores and 24 logical processors
- 31.7 GiB RAM
- `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`, LLVM 23.1.0
- Target: `x86_64-pc-windows-msvc`
- Cargo unstable feature: `-Zprofile-hint-mostly-unused`
- Generated dependency: 1,200 functions with 24 arithmetic operations each
- Generated matrix repetitions: three exploratory samples per row
- Public control: five clean samples per mode
- Public control dependency: METIS-CORE revision
  `78ae34090e043e79a206f2daffaa3889389b4790`

Representative commands:

```powershell
python measure_crate_slicing.py
cargo +nightly -Zprofile-hint-mostly-unused build --offline
$env:RUSTFLAGS = "-Zprint-mono-items=yes"
cargo +nightly -Zprofile-hint-mostly-unused build --offline
```

Every comparison used a fresh target directory. The harness recorded wall
time, child-process CPU time, sampled peak RSS, dependency rlib bytes,
executable bytes, exit status, and output digest. Diagnostic mono-item and
self-profile builds were separate from primary timings.

## Generated development controls

Median results:

| Consumer shape | Baseline wall | Hinted wall | Wall delta | CPU delta | Peak RSS | Dependency rlib |
|---|---:|---:|---:|---:|---:|---:|
| Sparse: 1 of 1,200 public functions | 4,062.5 ms | 2,620.0 ms | -35.5% | -49.3% | 600.3 -> 221.8 MiB | 11,601.0 -> 6,584.5 KiB |
| Dense: all 1,200 public functions | 5,289.2 ms | 5,545.6 ms | +4.8% | +13.4% | 599.9 -> 602.9 MiB | 11,601.0 -> 6,584.5 KiB |

Output digests and executable sizes matched in each pair.

The sparse hinted wall-time row had 15.5% MAD/median and is exploratory; CPU,
memory, artifact, mono-item, and public-control evidence all show the same
causal direction. The dense row is the required negative case: a smaller
dependency archive did not imply a faster complete build.

## Mono-item ownership

| Shape | Baseline dependency | Hinted dependency | Baseline consumer | Hinted consumer |
|---|---:|---:|---:|---:|
| Sparse public | 1,200 functions | 0 | 0 external functions | 1 external function |
| Dense public | 1,200 functions | 0 | 0 external functions | 1,200 external functions |

The hint moved codegen ownership. Sparse demand removed most work from the
complete build; dense demand moved all work to the root and added overhead.

## Frontend and codegen self-profile

The generated dependency retained the following query counts:

| Query family | Baseline | Hinted |
|---|---:|---:|
| `typeck_root` | 1,200 | 1,200 |
| `mir_borrowck` | 1,200 | 1,200 |
| `mir_built` | 1,200 | 1,200 |
| `optimized_mir` | 1,204 | 1,200 |
| `check_mono_item` | 1,204 | no 1,200-function dependency set |

The baseline performed LLVM emission for the 1,200 public functions. The
hinted dependency emitted a nearly empty module, but parsing through MIR
remained. This is codegen slicing, not full crate slicing.

## Existing-laziness controls

### Generic API

The sparse generic control used one type from a wide generic surface. Baseline
rustc already instantiated only the used external generic instance. The hint
was redundant and increased the exploratory release median 13.3%.

### Private API

The private-wide control exposed one public entry into 1,200 private
functions. Baseline codegen retained only the reachable private function and
public entry. The explicit hint did not create the public non-generic
opportunity seen in the wide dependency.

### Release optimization

The generated public functions were already cross-crate-inlinable in release
mode. Baseline and hinted rlibs were both 6,650.7 KiB, and mono-item placement
was already downstream. Release wall differences were noisy and are not
promoted.

### Inline policy

An initial fixture used `#[inline(never)]`. That policy prevented the
mostly-unused mechanism from deferring the functions. Removing the attribute
exposed the intended behavior.

Optimization, cross-crate inlinability, and explicit inline policy are
eligibility inputs.

## Multiple-consumer control

Four release binaries each referenced the same 600 functions. The resulting
consumer set contained 2,400 deferred external mono items. In this release
shape, the baseline already deferred the functions without the explicit hint,
and the hint changed neither rlib content nor ownership materially.

The control demonstrates the duplication risk: moving shared codegen out of a
dependency can make several consumers repeat it.

## Whole-crate correctness negative case

The sparse consumer referenced only `f_0`. The dependency's unused `f_1199`
was changed to contain an invalid assignment. The hinted build failed with
`E0308`.

Unused bodies remain type-checked. The existing mechanism does not defer
whole-crate errors or treat unreferenced source as semantically irrelevant.

## Public METIS-CORE control

A small binary called `part_recursive` from METIS-CORE. Five interleaved clean
development builds produced:

| Mode | Wall samples, ms | Median | MAD | METIS rlib | Executable |
|---|---|---:|---:|---:|---:|
| Baseline | 15,281.7; 15,345.5; 14,589.3; 13,703.0; 12,703.0 | 14,589.3 ms | 756.2 ms | 11,582.2 KiB | 694.0 KiB |
| Hinted | 9,776.2; 10,001.8; 9,704.1; 10,165.2; 10,584.7 | 10,001.8 ms | 225.6 ms | 644.2 KiB | 694.0 KiB |

The hinted median was 31.4% lower and the dependency rlib was 94.4% smaller.
Every run printed the same partition result:

```text
[0, 1, 0, 0]
```

Diagnostic mono-item output showed:

| Mode | Items before consumer compile marker | Items after marker |
|---|---:|---:|
| Baseline | 2,365 | 20 |
| Hinted | 0 | 2,200 |

Cargo can compile independent crates concurrently, so marker ranges are a
coarse ownership summary rather than a stable schema. The complete absence of
METIS mono items before the hinted consumer marker, combined with the rlib
reduction and downstream increase, confirms the same ownership shift as the
synthetic control.

## Interpretation

- Current Rust support can materially reduce dependency-owned development
  codegen for sparse public non-generic APIs.
- It does not skip frontend semantic correctness work.
- Generic APIs and private unreachable code already provide important forms of
  laziness.
- Dense and repeated consumer demand can erase the benefit or regress.
- Rlib shrinkage is a mechanism indicator, not an end-to-end success metric.
- Effective optimization and inline policy determine eligibility.
- Full crate slicing would target a larger parse-through-MIR and scheduling
  opportunity and requires a different compiler-owned correctness model.

## Limitations

- Generated rows used three exploratory samples.
- The public control used one consumer path through one small library.
- Peak RSS was sampled at the process-tree level and can miss short peaks.
- Mono-item output is unstable diagnostic text.
- The release fixture was already cross-crate-inlinable and does not support a
  promoted release claim.
- No full crate-slicing implementation, proc-macro, generated-code,
  dynamic-dispatch, coherence, LTO, cross-platform, or runtime-performance
  claim was tested.
