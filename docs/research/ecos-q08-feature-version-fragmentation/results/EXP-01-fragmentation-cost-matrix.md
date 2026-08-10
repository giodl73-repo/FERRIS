# EXP-01: Feature and Version Fragmentation Cost Matrix

Date: 2026-08-09
Question: ECOS-Q08
Method: exact single/dual-version binaries, minimal/derive feature binaries,
five clean check and release samples, no-op release controls, active metadata
closures, Cargo feature and duplicate trees, compile-fail identity controls,
failed cross-major update, and a synthetic mutually exclusive feature control
Result: duplicate-version and feature-expansion costs varied materially by
crate and closure. Some fragmentation caused nominal incompatibility, while
other multiplicity was required by declared constraints. Cargo diagnostics
located graph causes but did not determine cost or remediation.

## Environment

```text
host: x86_64-pc-windows-msvc
OS: Windows
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
rustc 1.95.0 (59807616e 2026-04-14)
LLVM 22.1.2
edition: 2024
feature-conflict workspace resolver: 3
CARGO_INCREMENTAL=0 for clean timing samples
```

The exact external releases were previously recorded in
[ECOS-Q03 EXP-01](../../ecos-q03-interchange-contracts/results/EXP-01-interchange-contract-probes.md)
and
[ECOS-Q02 EXP-01](../../ecos-q02-foundational-crate-census/results/EXP-01-foundational-crate-census.md).

| Family | Single/control release | Fragmented/feature release |
|---|---|---|
| HTTP | `http 1.5.0` | `http 0.2.12` + `http 1.5.0` |
| Syn | `syn 3.0.3` | `syn 2.0.119` + `syn 3.0.3` |
| Serde | `serde 1.0.229`, no defaults, `alloc` | `serde 1.0.229`, defaults + derive |

Each paired binary produced identical observable output within its family.

## Foundational closure duplicate census

The ECOS-Q06 combined foundational probe was checked first:

```powershell
cargo tree --locked --manifest-path <foundational-probe>\Cargo.toml -d
```

Result:

```text
warning: nothing to print.
```

The observed Windows graph for the nineteen exact releases had no duplicate
package versions. This is a bounded lockfile/target result, not an ecosystem
claim.

## Controlled active graphs

Active package closures used:

```powershell
cargo metadata --locked --format-version 1 `
  --filter-platform x86_64-pc-windows-msvc `
  --manifest-path <fixture>\Cargo.toml
```

| Fixture | Active packages | Duplicate family | Compiler artifacts | Build scripts | Proc macros |
|---|---:|---|---:|---:|---:|
| Single HTTP | 4 | None | 4 | 0 | 0 |
| Dual HTTP | 6 | HTTP 0.2.12 / 1.5.0 | 6 | 0 | 0 |
| Single Syn | 5 | None | 7 | 2 | 0 |
| Dual Syn | 6 | Syn 2.0.119 / 3.0.3 | 8 | 2 | 0 |
| Serde minimal | 3 | None | 5 | 2 | 0 |
| Serde derive | 8 | None | 12 | 4 | 1 |

Active package lists:

```text
single HTTP:
  root, bytes 1.12.1, http 1.5.0, itoa 1.0.18

dual HTTP:
  root, bytes 1.12.1, fnv 1.0.7,
  http 0.2.12, http 1.5.0, itoa 1.0.18

single Syn:
  root, proc-macro2 1.0.107, quote 1.0.47,
  syn 3.0.3, unicode-ident 1.0.24

dual Syn:
  root, proc-macro2 1.0.107, quote 1.0.47,
  syn 2.0.119, syn 3.0.3, unicode-ident 1.0.24

Serde minimal:
  root, serde 1.0.229, serde_core 1.0.229

Serde derive:
  root, proc-macro2 1.0.107, quote 1.0.47,
  serde 1.0.229, serde_core 1.0.229, serde_derive 1.0.229,
  syn 3.0.3, unicode-ident 1.0.24
```

The dual Syn graph shared all three supporting packages. Its multiplicity was
one additional Syn package instance.

## Clean timing method

Five check and five release builds were run for each fixture. Every sample used
a fresh target directory. Registry archives and source files were already
local. Commands emitted Cargo JSON so compiler artifacts, build scripts, and
procedural macros could be counted.

```powershell
$env:CARGO_INCREMENTAL = 0

cargo check --locked --message-format=json-render-diagnostics `
  --manifest-path <fixture>\Cargo.toml --target-dir <fresh-target>

cargo build --release --locked --message-format=json-render-diagnostics `
  --manifest-path <fixture>\Cargo.toml --target-dir <fresh-target>
```

Wall times are five-sample medians.

## Clean check results

| Pair | Control median | Variant median | Delta | Variant compiler-artifact delta |
|---|---:|---:|---:|---:|
| HTTP single -> dual | 1,606.4 ms | 1,695.1 ms | +88.7 ms / **+5.5%** | +2 |
| Syn single -> dual | 5,179.8 ms | 5,714.6 ms | +534.8 ms / **+10.3%** | +1 |
| Serde minimal -> derive | 4,811.5 ms | 10,539.5 ms | +5,728.0 ms / **+119.0%** | +7 |

Observed ranges:

| Fixture | Minimum | Median | Maximum |
|---|---:|---:|---:|
| Single HTTP | 1,557.1 ms | 1,606.4 ms | 2,579.8 ms |
| Dual HTTP | 1,552.3 ms | 1,695.1 ms | 2,341.8 ms |
| Single Syn | 4,721.5 ms | 5,179.8 ms | 6,266.8 ms |
| Dual Syn | 5,052.1 ms | 5,714.6 ms | 6,035.6 ms |
| Serde minimal | 4,380.7 ms | 4,811.5 ms | 5,165.4 ms |
| Serde derive | 10,061.3 ms | 10,539.5 ms | 11,577.3 ms |

## Clean release results

| Pair | Control median | Variant median | Delta |
|---|---:|---:|---:|
| HTTP single -> dual | 2,916.0 ms | 3,133.5 ms | +217.5 ms / **+7.5%** |
| Syn single -> dual | 13,362.9 ms | 16,565.0 ms | +3,202.1 ms / **+24.0%** |
| Serde minimal -> derive | 5,609.4 ms | 10,743.8 ms | +5,134.4 ms / **+91.5%** |

Observed ranges:

| Fixture | Minimum | Median | Maximum |
|---|---:|---:|---:|
| Single HTTP | 2,762.8 ms | 2,916.0 ms | 3,096.1 ms |
| Dual HTTP | 2,830.9 ms | 3,133.5 ms | 4,118.5 ms |
| Single Syn | 12,428.9 ms | 13,362.9 ms | 15,025.0 ms |
| Dual Syn | 16,118.8 ms | 16,565.0 ms | 17,623.2 ms |
| Serde minimal | 5,038.4 ms | 5,609.4 ms | 5,796.3 ms |
| Serde derive | 10,020.5 ms | 10,743.8 ms | 13,018.1 ms |

## Release artifact results

Sizes are medians across the five fresh target directories.

| Fixture | EXE | PDB | Whole target directory |
|---|---:|---:|---:|
| Single HTTP | 154,624 | 1,298,432 | 9,025,732 |
| Dual HTTP | 174,592 | 1,314,816 | 12,673,553 |
| Single Syn | 2,192,384 | 1,732,608 | 30,052,540 |
| Dual Syn | 4,222,976 | 1,978,368 | 47,179,743 |
| Serde minimal | 125,440 | 1,265,664 | 22,555,472 |
| Serde derive | 125,440 | 1,265,664 | 46,609,839 |

Pair deltas:

| Pair | EXE delta | Target-directory delta |
|---|---:|---:|
| HTTP single -> dual | +19,968 / **+12.9%** | +3,647,821 / **+40.4%** |
| Syn single -> dual | +2,030,592 / **+92.6%** | +17,127,203 / **+57.0%** |
| Serde minimal -> derive | 0 / **0.0%** | +24,054,367 / **+106.6%** |

The Serde result demonstrates compile-time and cache/storage expansion without
an observed shipped binary increase. The HTTP and Syn results demonstrate that
retained runtime/library code can increase the executable by different
amounts.

## No-op release controls

Each fixture's fifth release target directory was rebuilt five additional
times without changing any input:

```powershell
cargo build --release --locked --message-format=json-render-diagnostics `
  --manifest-path <fixture>\Cargo.toml --target-dir <existing-target>
```

| Fixture | Median no-op wall time | Nonfresh compiler artifacts across five runs |
|---|---:|---:|
| Single HTTP | 259.8 ms | 0 |
| Dual HTTP | 360.9 ms | 0 |
| Single Syn | 328.3 ms | 0 |
| Dual Syn | 401.5 ms | 0 |
| Serde minimal | 343.7 ms | 0 |
| Serde derive | 334.5 ms | 0 |

All thirty no-op samples reused every compiler artifact. The sub-second
medians include graph/freshness/process overhead and were variable; they do not
erase the measured clean cost.

## Effective Serde feature controls

Minimal:

```text
serde v1.0.229
└── serde feature "alloc"
```

Derive:

```text
serde v1.0.229
├── serde feature "default"
├── serde feature "derive"
├── serde feature "serde_derive"
└── serde feature "std"
```

The prior ECOS-Q03 workspace also demonstrated edge-local default suppression.
Its low branch requested only `alloc`, but the combined application enabled
the high branch's default, `std`, derive, and Serde Derive features on the same
Serde package instance.

## Mutually exclusive feature control

A synthetic resolver-3 workspace contained one backend package and two
branches:

```toml
# branch A
backend_core = {
  path = "../backend-core",
  default-features = false,
  features = ["backend-a"]
}

# branch B
backend_core = {
  path = "../backend-core",
  default-features = false,
  features = ["backend-b"]
}
```

The backend rejected the union:

```rust
#[cfg(all(feature = "backend-a", feature = "backend-b"))]
compile_error!("backend-a and backend-b are mutually exclusive");
```

Results:

| Selected package | Result |
|---|---|
| Branch A only | Pass |
| Branch B only | Pass |
| Combined application | Expected failure, exit 101 |

The combined inverse feature tree showed both requests reaching one
`backend_core` instance. Cargo performed additive feature unification as
designed; the package feature policy was not composition-safe.

## Duplicate-version identity controls

The ECOS-Q03 exact fixtures were rerun:

| Family | Versions | Observed failure |
|---|---|---|
| HTTP | 0.2.12 / 1.5.0 | E0308, distinct request types |
| Rand Core | 0.6.4 / 0.10.1 | E0277, distinct trait obligations |
| Syn | 2.0.119 / 3.0.3 | E0308, distinct AST types |

These failures establish public identity consequences. They do not imply that
every private duplicate is incompatible.

## Cargo update control

The dual HTTP fixture pinned both exact releases. This command attempted to
replace the old instance:

```powershell
cargo update --manifest-path <dual-http>\Cargo.toml `
  -p http@0.2.12 --precise 1.5.0
```

Result:

```text
error: failed to select a version for the requirement `http = "=0.2.12"`
candidate versions found which didn't match: 1.5.0
```

Exit was 101 and the lockfile hash was unchanged. Cargo protected the declared
constraint; eliminating the duplicate requires a manifest/API/owner decision.

## Cargo diagnostic boundary

Useful commands:

```powershell
cargo tree --locked -d
cargo tree --locked -i <package>@<version>
cargo tree --locked -e features -i <package>
cargo metadata --locked --format-version 1 --filter-platform <target>
```

They reveal package instances, reverse dependencies, requested/effective
features, and target-filtered closures. They do not establish:

- public type or trait exposure;
- conversion or coherence options;
- semantic compatibility;
- clean, incremental, binary, runtime, or storage cost;
- assurance or platform regressions; or
- remediation ownership and rollback.

## Raw evidence

Raw JSON and disposable probes were retained outside the repository session
workspace:

```text
ecos-q08-active-graph-results.json
ecos-q08-build-results.json
ecos-q08-build-summary.json
ecos-q08-build-deltas.json
ecos-q08-noop-results.json
ecos-q08-noop-summary.json
ecos-q08-feature-conflict-results.json
ecos-q08-version-identity-results.json
ecos-q08-update-control.json
```

Public conclusions in this result are reproducible from the commands and exact
identities above and do not depend on those local paths.

## Limitations

- One Windows host and one current toolchain were used.
- Clean samples used fresh target directories but warm registry/source caches.
- Five samples per command are not an ecosystem benchmark.
- Only one release profile and target were measured.
- Equivalent console output does not prove instruction-level equivalent user
  code.
- Runtime latency and memory were not measured.
- Target-directory bytes are toolchain- and profile-specific.
- No representative incremental edit was measured.
- No target-specific, build-dependency, or dev-dependency resolver split was
  measured beyond the documented resolver-3 scopes.
- The feature-conflict workspace is synthetic and intentionally rejects
  non-additive features.
- No dependency graph was automatically modified.

## Sources

- Cargo features:
  <https://doc.rust-lang.org/cargo/reference/features.html>
- Cargo dependency resolver:
  <https://doc.rust-lang.org/cargo/reference/resolver.html>
- Cargo tree:
  <https://doc.rust-lang.org/cargo/commands/cargo-tree.html>
- Cargo update:
  <https://doc.rust-lang.org/cargo/commands/cargo-update.html>
- Rust orphan rules:
  <https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules>
