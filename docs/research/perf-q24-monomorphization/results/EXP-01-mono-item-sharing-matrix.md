# EXP-01: Mono-Item Growth, Sharing, and Link Retention

Date: 2026-08-09
Question: PERF-Q24
Environment: Windows 11 Enterprise, x86_64-pc-windows-msvc
Rust: rustc 1.99.0-nightly (1a98b1e13 2026-08-07)
LLVM: 23.1.0

## Purpose

This experiment separates five questions that aggregate build timing cannot:

1. How does one generic family scale with more concrete type arguments?
2. Does an unused generic parameter still create separate instances?
3. How much does a thin generic shell around a non-generic core reduce backend
   work?
4. Which crates collect and emit the same instance with generic sharing on or
   off?
5. Do intermediate duplicate symbols survive final linking or collapse through
   linker identical-code folding?

The fixture is synthetic. It isolates instance topology and does not estimate
the prevalence of these patterns in ecosystem crates.

## Evidence surfaces

The harness used:

- repeated external wall timing with median and MAD;
- `-Zprint-mono-items=yes`;
- `-Zdump-mono-stats=<path> -Zdump-mono-stats-format=json`;
- `-Ztime-passes -Ztime-passes-format=json`;
- object, rlib, target-directory, and executable byte counts;
- `llvm-nm --demangle --defined-only`;
- MSVC linker maps and `/VERBOSE:ICF`;
- Cargo JSON artifact freshness;
- an alternating executable runtime control.

Representative direct-rustc diagnostic command:

```powershell
rustc +nightly main.rs --edition=2024 --emit=obj `
  -Copt-level=0 -Ccodegen-units=1 -Cdebuginfo=0 `
  -Csymbol-mangling-version=v0 `
  -Zprint-mono-items=yes `
  -Zdump-mono-stats=mono-stats `
  -Zdump-mono-stats-format=json `
  -Ztime-passes -Ztime-passes-format=json
```

Representative Cargo diagnostic flags:

```text
-Copt-level=<level>
-Ccodegen-units=<count>
-Zshare-generics=<yes|no>
-Csymbol-mangling-version=v0
-Zhuman-readable-cgu-names
-Zprint-mono-items=yes
-Zdump-mono-stats=<path>
-Zdump-mono-stats-format=json
-Ztime-passes
-Ztime-passes-format=json
```

All Cargo builds used `--locked --offline -j 1` and isolated target
directories. Diagnostic builds are not primary benchmark runs.

## Single-crate instance growth

The scale fixture generated 1, 32, 128, and 512 concrete types. Each type was
passed through one of these shapes:

- `used`: the generic parameter selected type-specific behavior;
- `unused`: the generic parameter did not affect the function body;
- `thin`: a small generic shell called one non-generic heavy core;
- `erased`: every call used the one non-generic heavy core directly;
- `control`: the same type declarations existed without the generic family.

The family instance counts were exact at every scale:

| Concrete types | `used` instances | `unused` instances | `thin` shell instances | `erased` core instances |
|---:|---:|---:|---:|---:|
| 1 | 1 | 1 | 1 | 1 |
| 32 | 32 | 32 | 32 | 1 |
| 128 | 128 | 128 | 128 | 1 |
| 512 | 512 | 512 | 512 | 1 |

The 512-type case made the backend differences visible:

| Shape | Total mono items | Family estimate | Total mono estimate | Object bytes | Wall median, ms | MAD, ms |
|---|---:|---:|---:|---:|---:|---:|
| Control | 11 | 0 | 1,064 | 20,952 | 214.78 | 2.63 |
| Type-dependent generic | 1,045 | 12,288 | 15,453 | 311,201 | 292.25 | 2.22 |
| Unused-parameter generic | 533 | 12,288 | 13,917 | 310,211 | 272.83 | 0.79 |
| Thin generic shell plus core | 534 | 1,048 | 2,677 | 231,851 | 270.97 | 1.78 |
| Non-generic core | 22 | 24 | 1,653 | 33,853 | 231.11 | 6.24 |

`-Zdump-mono-stats` reported the unused family as:

```json
{
  "name": "heavy_unused",
  "instantiation_count": 512,
  "size_estimate": 24,
  "total_estimate": 12288
}
```

The compiler therefore collected 512 copies even though the type parameter did
not affect the body. The former `-Zpolymorphize` implementation that attempted
to remove such parameters was deleted in December 2024.

Moving the 24-unit body into one non-generic core left 512 two-unit shells plus
one 24-unit core. That reduced the family estimate from 12,288 to 1,048, a
91.5% reduction, and reduced object bytes by 25.3% in this fixture. It did not
remove the 512 shell instances.

The deterministic instance counts, compiler estimates, and object bytes
support the structural conclusion. The wall rows remain three-repetition,
observer-affected synthetic diagnostics and do not support a promoted speedup
claim.

## Cross-crate sharing topology

The sharing workspace used this dependency diamond:

```text
                 mono-base
                 /       \
           mono-left   mono-right
                 \       /
                  mono-app
```

`mono-base` defined two generic functions and one shared concrete type.
`mono-left` and `mono-right` instantiated both functions with that same type.
`mono-app` also used both functions.

With sharing enabled, both siblings still instantiated each function because
neither sibling is upstream of the other. The application reused one exported
upstream instance. With sharing disabled, the application instantiated a third
copy.

| Configuration | Instances per tested family | Wall median, ms | MAD, ms | Executable bytes | Target bytes |
|---|---:|---:|---:|---:|---:|
| Dev, sharing on | 2 | 1,021.14 | 63.78 | 158,208 | 3,157,888 |
| Dev, sharing off | 3 | 1,000.58 | 57.57 | 159,744 | 3,159,695 |
| Release, 16 CGUs, sharing on | 2 | 1,040.52 | 55.79 | 144,896 | 3,049,200 |
| Release, 16 CGUs, sharing off | 3 | 1,063.38 | 36.70 | 145,408 | 3,049,985 |
| Release, 1 CGU, sharing on | 2 | 991.13 | 56.49 | 145,408 | 3,031,484 |
| Release, 1 CGU, sharing off | 3 | 921.93 | 50.16 | 145,408 | 3,030,368 |
| Release, ThinLTO, sharing on | 2 | 2,738.69 | 97.14 | 143,360 | 2,556,536 |
| Release, ThinLTO, sharing off | 3 | 2,891.78 | 69.69 | 143,360 | 2,556,302 |

These are three-repetition exploratory diagnostic builds. The sharing rows do
not show one consistent wall-time direction outside ThinLTO. They establish
instance ownership and artifact differences, not a general compile-time
percentage.

The default behavior matched current rustc source:

- development `opt-level=0` behaved like sharing on;
- release `opt-level=3` behaved like sharing off.

Current rustc defaults sharing on for optimization levels 0, 1, `s`, and `z`,
and off for levels 2 and 3. An explicit unstable flag overrides the default.

## Collection is not final binary duplication

`llvm-nm` found one externally visible copy in each sibling rlib with sharing
enabled. With sharing disabled and one release CGU, each sibling contained an
internal copy and the application collected a third internal copy.

The final linker map told a different story.

With sharing disabled, all three `shared_kernel` symbols resolved to the same
final address:

```text
mono-app   shared_kernel ... 0000000140001000
mono-left  shared_kernel ... 0000000140001000
mono-right shared_kernel ... 0000000140001000
```

With sharing enabled, both sibling symbols also resolved to one address:

```text
mono-left  shared_kernel ... 00000001400017b0
mono-right shared_kernel ... 00000001400017b0
```

The linker reported 3,267 bytes of total identical-code-folding savings with
sharing off and 3,173 bytes with sharing on. Both one-CGU executables were
145,408 bytes.

This fixture therefore had:

- three collected and emitted instances with sharing off;
- two collected and emitted instances with sharing on;
- one retained address after final-link identical-code folding in both cases.

Sharing removed pre-link compiler work. It did not reduce the final one-CGU
binary because the linker independently folded the identical bodies.

## Runtime control

The release one-CGU binaries ran the same 40,000,000-iteration generic kernel.
Twenty-one paired samples alternated sharing-on and sharing-off order:

| Mode | Samples | Median, ms | MAD, ms | Minimum, ms | Maximum, ms |
|---|---:|---:|---:|---:|---:|
| Sharing on | 21 | 935.920 | 9.090 | 925.310 | 998.810 |
| Sharing off | 21 | 939.310 | 10.610 | 917.950 | 976.330 |

The 0.36% median difference is below the observed dispersion. This fixture
does not establish a runtime gain or regression from generic sharing.

That negative result is important: source shape, optimization, LTO, target,
and call topology determine whether exported sharing blocks a useful local
optimization. A generic-sharing recommendation requires its own runtime
control.

## Incremental consumer edit

After an initial development build, only `mono-left` changed a non-generic
constant:

| Sharing | Rebuild wall, ms | Dirty artifacts | Fresh artifacts | Recollected tested instances |
|---|---:|---|---|---:|
| On | 708.33 | `mono-left`, `mono-app` | `mono-base`, `mono-right` | 1 |
| Off | 974.77 | `mono-left`, `mono-app` | `mono-base`, `mono-right` | 2 |

Sharing let the rebuilt application use an unchanged sibling's exported
instance, but it did not keep the application fresh. Crate invalidation,
instance ownership, codegen, and final linking remain different decisions.

## Shared-target forest control

Two unrelated workspace roots depended on the same path library and wrote to
one disposable shared target directory:

| Build | Shared dependency | Application instance | Wall, ms | Target bytes after build |
|---|---|---:|---:|---:|
| `forest-app-a` | Compiled | 1 | 718.65 | 2,932,251 |
| `forest-app-b` | Fresh | 1 | 380.61 | 5,850,046 |

The second workspace reused the dependency artifact but collected and emitted
its own `forest_kernel::<Shared>` instance. Target storage increased by
2,917,795 bytes.

This is not permission to share writable targets across unrelated real
repositories. PERF-Q05 already demonstrated provenance and isolation hazards.
The control establishes only that Cargo artifact reuse and generic-instance
reuse are separate layers.

## Limitations

- The fixture is synthetic and small.
- Only x86_64 Windows MSVC, rustc nightly 1a98b1e13, LLVM 23.1.0, and the MSVC
  linker's identical-code folding were measured.
- Compile rows use three repetitions and diagnostic flags; they are
  exploratory, observer-affected evidence.
- The experiment did not measure peak memory, Linux or macOS linkers, dynamic
  libraries, `#[inline]` matrices, trait objects, fat LTO, alternate codegen
  backends, or public generic-heavy repositories.
- `size_estimate` is a rustc mono-item estimate, not LLVM IR lines, object
  bytes, machine instructions, or retained binary bytes.
- Linker folding of this identical body does not prove semantically equivalent
  generic bodies will always fold.
- The runtime control covers one kernel and does not generalize to applications.

## Reproduction boundary

The retained research harness generates and deletes disposable fixtures and
writes a summarized JSON record. It is intentionally not committed with the
research decision. A future upstream claim must be reduced to a rustc-perf or
compiler test fixture rather than depending on this lab harness.
