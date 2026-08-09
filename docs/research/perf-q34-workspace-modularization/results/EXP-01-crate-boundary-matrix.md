# EXP-01: Crate Boundary Response Matrix

Date: 2026-08-09

Question: PERF-Q34

Status: Complete

## Purpose

Measure when matched Rust code benefits from additional crate boundaries and
when those boundaries add invocation, metadata, generic, test, and dependency
fan-out cost.

The experiment separates:

- one crate containing sixteen source modules;
- two, four, eight, and sixteen sibling library crates;
- an eight-crate serial dependency chain;
- a generic function consumed once inside one crate versus from eight sibling
  crates; and
- leaf-versus-foundation edits in the public PARLOR workspace.

## Environment

- Host substrate: WSL2 on the PERF-Q33 workstation
- Guest kernel:
  `Linux 6.6.87.2-microsoft-standard-WSL2 x86_64`
- Logical processors exposed: 24
- Source placement: ext4 in the WSL2 virtual disk
- Target placement: ext4 in the WSL2 virtual disk
- rustc: `1.97.1 (8bab26f4f 2026-07-14)`
- LLVM: `22.1.6`
- Cargo: `1.97.1 (c980f4866 2026-06-30)`
- Profile: development, incremental enabled, `debug = 0`
- Samples: one excluded warm-up and five interleaved measured repetitions
- Timing: external wall clock plus `/usr/bin/time -v`
- Cargo evidence: JSON compiler-artifact messages
- Acquisition: offline after fixture preparation

Project-cold means a new target directory. Toolchain, registry, guest, host,
and physical storage caches were not cleared.

## Matched topology fixture

The generated fixture contained sixteen logical units with eighty public
non-generic functions per unit, for 1,280 functions total. Every topology used
the same function bodies and application call sequence.

| Topology | Library crates | Dependency shape |
|---|---:|---|
| `flat-1` | 1 | One library containing sixteen modules |
| `siblings-2` | 2 | Two independent libraries consumed by one app |
| `siblings-4` | 4 | Four independent libraries consumed by one app |
| `siblings-8` | 8 | Eight independent libraries consumed by one app |
| `siblings-16` | 16 | Sixteen independent libraries consumed by one app |
| `chain-8` | 8 | Eight serial libraries; app consumes the final crate |

Each logical unit also contained one private helper and one unit test. The
application contained one binary and one test.

## Workloads

1. Clean workspace build into a new target directory.
2. Warm no-op workspace build.
3. Private helper edit in the last logical unit.
4. Revert to the original bytes.
5. Private helper edit in the first logical unit.
6. Clean `cargo test --workspace --no-run` into a new target directory.

The last-unit and first-unit edits are equivalent for sibling topologies. They
become leaf and foundation edits in the serial chain.

## Wall-time matrix

Values are median milliseconds with median absolute deviation in parentheses.

| Topology | Clean build | No-op | Last-unit edit | First-unit edit | Revert | Test no-run |
|---|---:|---:|---:|---:|---:|---:|
| `flat-1` | 1,127.4 (77.2) | 35.5 (3.8) | 931.9 (20.9) | 925.3 (78.2) | 961.7 (44.8) | 874.8 (84.6) |
| `siblings-2` | 974.9 (10.5) | 29.4 (3.7) | 937.9 (144.0) | 1,156.9 (187.1) | 1,026.8 (94.8) | 1,233.5 (259.5) |
| `siblings-4` | 1,078.8 (210.8) | 31.2 (3.4) | 921.7 (4.0) | 990.0 (98.3) | 909.4 (56.2) | 1,080.2 (236.9) |
| `siblings-8` | 954.6 (18.4) | 36.1 (0.7) | 927.2 (44.8) | 890.9 (79.7) | 877.6 (96.8) | 1,099.6 (138.7) |
| `siblings-16` | 1,017.0 (66.9) | 33.9 (6.8) | 902.8 (124.5) | 941.7 (56.4) | 865.9 (32.1) | 1,411.3 (266.7) |
| `chain-8` | 1,293.4 (124.6) | 30.5 (2.5) | 916.6 (42.6) | 1,183.6 (46.0) | 919.6 (84.3) | 1,621.0 (185.2) |

Compared with `flat-1`:

- the fastest clean row, `siblings-8`, was 15.3% faster;
- `chain-8` was 14.7% slower;
- sibling private-edit medians were within 3.7% of the flat control except for
  the unstable `siblings-2` first-edit row;
- the serial-chain foundation edit was 27.9% slower and rebuilt nine artifacts;
- test compilation was 23.5% to 61.3% slower for sibling splits and 85.3%
  slower for the chain.

The four-crate clean row and several test rows exceeded 10% MAD/median. They
remain visible and are not used to rank a universal optimum.

## Work and artifact multiplication

| Topology | Clean artifacts | Last-unit edit | First-unit edit | Test artifacts | Test median CPU |
|---|---:|---:|---:|---:|---:|
| `flat-1` | 2 | 2 | 2 | 3 | 1.04 s |
| `siblings-2` | 3 | 2 | 2 | 5 | 2.24 s |
| `siblings-4` | 5 | 2 | 2 | 9 | 2.96 s |
| `siblings-8` | 9 | 2 | 2 | 17 | 4.42 s |
| `siblings-16` | 17 | 2 | 2 | 33 | 9.40 s |
| `chain-8` | 9 | 2 | 9 | 17 | 3.53 s |

Clean build wall time hid some additional CPU behind sibling parallelism. For
example, `siblings-16` used 1.60 median CPU seconds versus 0.57 seconds for
`flat-1`, despite finishing 9.8% sooner by wall clock.

The test workload exposed the boundary cost directly. Each library package
contributed an ordinary library target and a test harness. Sixteen sibling
libraries plus the application produced 33 compiled artifacts and used about
nine times the flat control's CPU.

## Incremental containment

Cargo rebuilt the edited library and application for a sibling edit, regardless
of whether the workspace had two, four, eight, or sixteen sibling libraries.
It did not rebuild unaffected siblings.

The flat crate rebuilt its library and application, but rustc's module-oriented
incremental codegen reused unaffected work products. Splitting the same source
did not produce a repeatable edit-latency improvement over that existing
intra-crate containment.

The chain demonstrated topology sensitivity:

- editing the final library rebuilt the final library and app;
- editing the first library rebuilt all eight libraries and the app.

Current Cargo and rustc behavior did this even though the edit changed a
private function body. PERF-Q20 explains why Relink-Don't-Rebuild is needed to
prune semantically unaffected downstream compilation.

## Generic-boundary control

One generic kernel function was called eight times with `u64`.

The flat control placed all eight callers in one consumer crate. The split
control placed one caller in each of eight sibling consumer crates. Both then
fed one application.

| Metric | One consumer crate | Eight sibling consumers | Delta |
|---|---:|---:|---:|
| Clean wall median | 802.1 ms | 844.6 ms | +5.3% |
| Median CPU | 0.18 s | 0.58 s | +222.2% |
| Target bytes | 1,552,562 | 3,780,670 | +143.5% |
| `shared_kernel::<u64>` mono items | 1 | 8 | 8x |
| Compiled artifacts | 3 | 10 | +7 |

Wall-time dispersion prevents promoting the 5.3% wall difference. CPU,
artifact bytes, and mono-item ownership establish the duplicated work.

This matches PERF-Q24: generic sharing follows dependency direction. Sibling
crates cannot consume each other's instance while being compiled.

The mono-item command was:

```text
RUSTC_BOOTSTRAP=1 RUSTFLAGS="-Zprint-mono-items=yes" \
  cargo build --workspace --locked --offline
```

`RUSTC_BOOTSTRAP` was used only for diagnostic output. Ordinary stable Cargo
builds produced the timing results.

## Public PARLOR control

Repository:
[PARLOR](https://github.com/giodl73-repo/PARLOR)

Revision: `0975fad880cb3bda0b911cd8eb4fc58edbbfaf29`

The six-package workspace contains:

```text
parlor-core
  -> parlor-go
  -> parlor-checkers
  -> parlor-chess
  -> parlor-backgammon
all five libraries
  -> parlor-cli
```

The disposable copy received one behavior-neutral private unused function at
the end of either `parlor-go` or `parlor-core`.

| Workload | Wall median | MAD | Median CPU | Compiled artifacts |
|---|---:|---:|---:|---:|
| Clean workspace build | 1,025.3 ms | 68.5 ms | 1.16 s | 6 |
| Private item in `parlor-go` | 772.6 ms | 116.7 ms | 0.18 s | 2 |
| Private item in `parlor-core` | 746.4 ms | 39.5 ms | 0.39 s | 6 |

Wall time was dominated by a roughly similar Cargo, rustc, and linking floor.
The work evidence was decisive:

- the game-leaf edit rebuilt `parlor-go` and `parlor-cli`;
- the shared-core edit rebuilt all six packages; and
- the core edit used 2.17x the measured CPU.

This validates the graph-position result on public source without claiming
that PARLOR should be restructured.

## Interpretation

### Clean build

Sibling crate boundaries can expose Cargo-level parallelism when a large crate
would otherwise be one frontend and codegen process. The improvement is not
monotonic because every additional crate adds rustc startup, metadata,
artifact, archive, and coordination work.

Serial boundaries do not expose that parallelism. They can lengthen the
critical path while retaining the per-crate overhead.

### Incremental edit

A crate boundary helps only if the edit remains on a narrow dependent cone and
the work avoided inside a larger crate exceeds the additional invocation and
link floor.

Module-level incremental compilation already provided strong containment in
the flat fixture. "The crate is large" was therefore not sufficient evidence
for splitting it.

### Tests and validation

Package boundaries multiply test binaries and linking work. That can be the
dominant regression even when clean development builds improve.

Validation selection may later avoid running every package target, but
PERF-Q35 owns that question. PERF-Q34 does not count skipped tests as a crate
boundary gain.

### Generics

Moving callers into sibling crates can duplicate identical concrete generic
instances. A boundary advisor must inspect monomorphization ownership rather
than treating source-line or crate counts as sufficient.

### Non-performance boundaries

Independent reuse, publication, semver, features, platform support, capability
isolation, build scripts, procedural macros, unsafe review, and ownership can
justify a crate boundary even when it costs build time. This experiment does
not reduce repository architecture to latency.

## Limitations

- The matched fixture is synthetic and intentionally dependency-free.
- The public control validates fan-out, not a complete monolithic
  counterfactual.
- Only Linux rustc inside WSL2 was measured.
- The fixture used development builds with debuginfo disabled.
- No release, LTO, dynamic-link, proc-macro, build-script, native dependency,
  feature-set, or cross-target restructuring was performed.
- Wall times were short enough that process and linker floors were material.
- No source was automatically transformed.
- The experiment does not identify a universal crate count.

## Retained evidence

The retained experiment bundle contains:

- `measure_modularization.py`;
- `measure_controls.py`;
- `results/modularization.json`; and
- `results/controls.json`.

Generated source trees and target directories were removed after evidence
capture.
