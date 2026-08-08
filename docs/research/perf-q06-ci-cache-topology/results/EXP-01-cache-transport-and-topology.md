# EXP-01: CI Cache Transport and Topology

Date: 2026-08-08
Question: PERF-Q06
Status: Complete

## Purpose

Measure the cost and retained work of broad and dependency-only Cargo target
archives, test dependency fallback after a lockfile change, and compare those
controls with a real public GitHub Actions workflow using
`Swatinem/rust-cache`.

The experiment separates:

- Cargo artifact compatibility;
- CI cache-key matching;
- archive transport and materialization;
- command and job placement;
- retention and eviction.

The local transport measurements are exploratory with three repetitions. They
support topology and break-even modeling, not a promoted cross-platform
optimization claim.

## Environment

Local controlled runs used:

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- Cargo `1.95.0 (f2d3ce0bd 2026-03-21)`;
- rustc `1.95.0 (59807616e 2026-04-14)`;
- host `x86_64-pc-windows-msvc`;
- LLVM `22.1.2`;
- `CARGO_INCREMENTAL=0`;
- retained global registry source and archive caches;
- isolated disposable target directories.

The public CI evidence used METIS-CORE revision
`78ae34090e043e79a206f2daffaa3889389b4790` and workflow run
[`25951794528`](https://github.com/giodl73-repo/METIS-CORE/actions/runs/25951794528).

## Public workflow topology

METIS-CORE's
[`ci.yml`](https://github.com/giodl73-repo/METIS-CORE/blob/78ae34090e043e79a206f2daffaa3889389b4790/.github/workflows/ci.yml)
defined:

- one `test` job matrix on Ubuntu, Windows, and macOS;
- stable Rust, rustfmt, and Clippy;
- `Swatinem/rust-cache@v2`;
- test, release test on Ubuntu, Clippy, docs, audit, and package commands;
- separate Kani and Prusti jobs with distinct cache keys.

The test job used rust-cache defaults:

- target caching enabled;
- dependency-only artifacts;
- workspace artifacts excluded;
- incremental compilation disabled;
- job ID, OS, architecture, rustc identity, relevant environment, manifests,
  and dependency lock state in the key.

## Observed public CI run

| Job | Cache result | Payload | Cache step | First Cargo workload | Outcome |
| --- | --- | ---: | ---: | ---: | --- |
| Test macOS | Exact hit | 105,747,058 B | 7 s | Test 9 s | Root recompiled |
| Test Ubuntu | Exact hit | 193,446,288 B | 4 s | Test 12 s | Root recompiled |
| Test Windows | Exact hit | 110,250,014 B | 10 s | Test 32 s | Root recompiled |
| Prusti | Exact hit | 20,050,639 B | 2 s | Skipped | Restore had no consumer |
| Kani | Cache step skipped | N/A | N/A | Install ran 76 min | Install cancelled first |

The cache-action step includes key construction, Cargo metadata, download, and
extraction. GitHub's rounded step durations are therefore used rather than
presenting network throughput.

The logs reported:

```text
Restored from cache key "...Linux-x64...-c7e53a31" full match: true.
Compiling metis-core v0.1.0
Cache up-to-date.
```

The same test keys and byte sizes appeared in earlier run
[`25630776288`](https://github.com/giodl73-repo/METIS-CORE/actions/runs/25630776288)
on a different source commit. Source changes did not change the dependency
cache key, and workspace artifacts were rebuilt.

On 2026-08-08:

```powershell
gh api repos/giodl73-repo/METIS-CORE/actions/caches?per_page=100
```

returned zero current entries. The last observed access was 2026-05-16, which
is outside GitHub's seven-day idle-retention window.

## Local transport fixture

The pinned METIS-CORE fixture ran:

```powershell
$env:CARGO_INCREMENTAL = "0"
$env:CARGO_TARGET_DIR = "<empty-target>"
cargo test `
  --manifest-path "<fixture>\Cargo.toml" `
  --all-targets `
  --no-run `
  --locked `
  --offline `
  --message-format=json
```

Three project-cold targets established the baseline.

Two cache payloads were then created:

1. **Full target:** all test artifacts, including workspace crates.
2. **Dependency-only target:** the full target after
   `cargo clean -p metis-core`, retaining dependency artifacts.

Each payload was archived with local gzip tar, extracted into three fresh
target paths, and followed by the same Cargo command. These are local archive
materialization measurements, not GitHub network measurements.

## Local transport results

| State | Samples (ms) | Median | MAD |
| --- | --- | ---: | ---: |
| Project-cold Cargo | 41,158, 32,654, 32,567 | 32,654 | 87 |
| Full archive extract | 2,876, 2,956, 2,747 | 2,876 | 80 |
| Cargo after full restore | 13,005, 4,090, 4,083 | 4,090 | 7 |
| **Full restore total** | 15,881, 7,046, 6,830 | **7,046** | 216 |
| Dependency archive extract | 1,983, 2,129, 2,016 | 2,016 | 33 |
| Cargo after dependency restore | 12,063, 10,641, 10,947 | 10,947 | 306 |
| **Dependency restore total** | 14,046, 12,770, 12,963 | **12,963** | 193 |

The first full-restored Cargo run was a warm-up outlier. Its artifact result
was still fully fresh.

Observed Cargo JSON:

| State | Total artifacts | Registry fresh | Registry dirty | Workspace fresh | Workspace dirty |
| --- | ---: | ---: | ---: | ---: | ---: |
| Project-cold | 99 | 0 | 89 | 0 | 10 |
| Full restore | 99 | 89 | 0 | 10 | 0 |
| Dependency-only restore | 99 | 89 | 0 | 0 | 10 |

The local median totals were:

- full restore: 78.4% below project-cold;
- dependency-only restore: 60.3% below project-cold.

Those percentages exclude network download, cache lookup, verification,
runner startup, and producer upload.

## Archive size and producer cost

| Payload | Uncompressed | Archive | Pack samples (ms) | Median | MAD |
| --- | ---: | ---: | --- | ---: | ---: |
| Full target | 517,366,330 B | 121,592,634 B | 25,604, 14,119, 14,020 | 14,119 | 99 |
| Dependency-only | 365,650,443 B | 94,051,432 B | 12,899, 10,238, 9,963 | 10,238 | 275 |

Removing workspace artifacts reduced:

- uncompressed payload by 29.3%;
- compressed payload by 22.6%;
- median local pack time by 27.5%.

The dependency-only consumer paid about 5.9 seconds more than the full consumer
because it rebuilt the ten observed workspace artifacts. Whether that trade is
positive depends on expected future hits, upload cost, source-change frequency,
and quota pressure.

## Lockfile fallback fixture

Two versions of one synthetic application used:

| Fixture | `itoa` | `ryu` |
| --- | --- | --- |
| V1 | 1.0.15 | 1.0.20 |
| V2 | 1.0.14 | 1.0.20 |

The V1 target was copied, the local root artifact was removed, and V2 was
checked in the restored dependency target.

Cargo JSON reported:

| Unit | V2 result |
| --- | --- |
| `ryu 1.0.20` | Fresh |
| `itoa 1.0.14` | Dirty |
| local root | Dirty |

The dependency-only target was 93,455 bytes before V2 and 130,788 bytes after
V2, a 40.0% increase because both `itoa` versions remained during the consumer
run.

Fallback therefore salvaged the unchanged dependency, but the restored payload
needed post-build cleanup before it would be suitable for a new cache entry.

## Command-graph evidence

PERF-Q04's PARLOR command sequences provide a controlled job-boundary model:

| Producer then consumer | Consumer result |
| --- | --- |
| Check then test | All 11 test artifacts dirty |
| All-target check then test | All 11 test artifacts dirty |
| Build then test | Five ordinary libraries fresh; six test artifacts dirty |
| All-target check then Clippy | All 12 Clippy artifacts dirty |
| Dev build then release | All six release artifacts dirty |

A cache key cannot make incompatible command artifacts compatible. Conversely,
placing build and test in separate jobs can discard five observed compatible
workspace artifacts that same-job execution preserved.

## Interpretation

1. Dependency-only caching can retain substantial compile work while remaining
   stable across source commits.
2. Restore cost is material and platform-dependent; a cache hit is not a
   latency win by definition.
3. Full targets can produce a larger local win on an unchanged source revision,
   but increase payload, producer cost, source sensitivity, and quota.
4. Partial lockfile fallback can reuse unchanged dependencies and also retain
   obsolete variants until cleanup.
5. Job placement can make a cache useless: Prusti restored unused bytes, while
   Kani failed before reaching its cache.
6. Command and profile compatibility must be observed independently from cache
   key matching.
7. Idle eviction means a valid cache design can still have zero effective hit
   rate on infrequently active repositories.

## Limitations

- Local archives used gzip tar and local storage, not GitHub's zstd service or
  network.
- The full-restore fixture used unchanged source and one machine.
- The first full restore showed a warm-up outlier.
- Public CI step durations are rounded by GitHub.
- Only one public repository had an active Rust cache workflow; RUNE and PARLOR
  had no workflows at the reviewed revisions.
- No cache was deliberately poisoned in GitHub Actions. PERF-Q05's isolated
  corruption experiment and Cargo issue #8603 provide that negative evidence.
- Cross-organization transport, signatures, and revocation remain PERF-Q30.

## Retained evidence

The private experiment record retains:

- all local Cargo JSON messages and stderr;
- raw sample durations;
- archive and target sizes;
- dependency fallback results;
- exact toolchain and host data;
- public workflow run IDs, job timing, cache keys, and cache sizes.

No upstream issue, comment, branch, or pull request was created.
