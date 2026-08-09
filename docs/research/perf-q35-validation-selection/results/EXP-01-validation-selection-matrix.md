# EXP-01: Impact-Aware Validation Selection Matrix

Date: 2026-08-09
Question: PERF-Q35
Result: conservative package selection retained all eight seeded failure
classes and reduced the warm synthetic median 57.1%; a public PARLOR control
showed a bounded 9.4% gain.

## Method

The synthetic fixture contained 17 Cargo packages:

- one shared `core`;
- eight `leaf_NN` libraries depending on `core`; and
- eight `app_NN` libraries, each depending on one corresponding leaf.

Each policy ran from a copied source tree with an isolated target directory.
Timing cases first completed the full reference plan, applied one harmless
private edit to `leaf_00`, and then measured the policy. One warm-up preceded
five repetitions. Results report median and median absolute deviation.

Environment:

```text
rustc 1.89.0 (29483883e 2025-08-04)
cargo 1.89.0 (c24e10642 2025-06-23)
WSL2 Ubuntu 24.04
source: ext4 in the WSL2 VHD
target: ext4 in the WSL2 VHD
```

The WSL root user-session warning was nonfatal and did not change command exit
status.

## Policies

| Policy | Package scope | Activity scope | Unknown-file behavior |
|---|---|---|---|
| Changed-package tests | Direct owner only | Ordinary `cargo test` | Selects no package |
| Reverse cone + matrix | Owner plus reverse dependents | Complete command matrix plus repository gate | Selects no package |
| Conservative impact | Owner plus reverse dependents | Complete command matrix plus repository gate | Full fallback |
| Full reference | All 17 packages | Complete command matrix plus repository gate | Full |

The complete matrix was:

```text
cargo check <packages> --all-targets --all-features
cargo clippy <packages> --all-targets --all-features -- -D warnings
cargo test <packages> --all-features
cargo test <packages> --all-targets --all-features --no-run
cargo check <packages> --release --all-targets --all-features
python3 validation_gate.py
```

All Cargo commands also used `--locked`, `--offline`, an explicit manifest, and
an isolated target directory.

## Seeded failures

| Scenario | Changed input | Expected distinguishing failure |
|---|---|---|
| Local behavior | `leaf_00` implementation | Direct package test |
| Core downstream | Shared `core` behavior | Downstream consumer test |
| Feature only | `leaf_02` feature-gated function | All-features check |
| Clippy only | `leaf_01` private function | Clippy with denied warnings |
| Release only | `leaf_03` release-gated function | Release check |
| Doctest only | `leaf_04` documentation example | Doctest |
| Shared data | `shared/leaf5.expected` | Runtime test using a non-package file |
| Repository gate | Root `policy.txt` | Mandatory repository script |

## Failure detection

| Policy | Caught | Missed |
|---|---:|---|
| Changed-package tests | 2/8 | Core downstream, feature, Clippy, release, shared data, repository gate |
| Reverse cone + complete matrix | 7/8 | Shared runtime data |
| Conservative graph + fallback | 8/8 | None |
| Full reference | 8/8 | None |

Detailed disposition:

| Failure | Changed-package tests | Reverse cone + matrix | Conservative | Full |
|---|---|---|---|---|
| Local behavior | Caught | Caught | Caught | Caught |
| Core downstream | Missed | Caught | Caught | Caught |
| Feature only | Missed | Caught | Caught | Caught |
| Clippy only | Missed | Caught | Caught | Caught |
| Release only | Missed | Caught | Caught | Caught |
| Doctest only | Caught | Caught | Caught | Caught |
| Shared data | Missed | Missed | Caught by full fallback | Caught |
| Repository gate | Missed | Caught | Caught | Caught |

The shared-data case is the decisive negative control. Cargo package roots and
reverse dependencies did not express a runtime file dependency at
`shared/leaf5.expected`. A repository mapping could safely narrow that case in
the future; absent such a mapping, full fallback is required.

## Warm passing-edit timing

| Policy | Selected packages | Commands | Median | MAD | Saving vs full |
|---|---:|---:|---:|---:|---:|
| Changed-package tests | 1 | 1 | 911.5 ms | 101.1 ms | 64.3% |
| Reverse cone + complete matrix | 2 | 6 | 1,076.1 ms | 11.6 ms | 57.9% |
| Conservative impact | 2 | 6 | 1,096.8 ms | 71.4 ms | 57.1% |
| Full reference | 17 | 6 | 2,553.8 ms | 40.2 ms | reference |

The unsafe changed-package policy was only 16.9% faster than conservative
selection, but it missed six of eight failure classes. The large defensible
gain came from selecting package scope while preserving validation dimensions.

## Public PARLOR control

Revision:

```text
0975fad880cb3bda0b911cd8eb4fc58edbbfaf29
```

The repository's documented contract was preserved:

```text
cargo test --release --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

The selected policy changed only Cargo package scope for release tests and
Clippy:

```text
-p parlor-go -p parlor-cli
```

Workspace formatting remained mandatory.

| PARLOR policy | Median | MAD | Saving |
|---|---:|---:|---:|
| `parlor-go` reverse cone | 1,217.6 ms | 39.2 ms | 9.4% |
| Full documented contract | 1,344.7 ms | 22.8 ms | reference |

The selected release-test median was 1,009.3 ms versus 1,124.7 ms for the full
workspace. Clippy changed little, and formatting remained workspace-wide.
`parlor-cli` also depends broadly on the game crates, so topology and mandatory
gates bounded the available saving.

## Interpretation

1. Package impact and validation coverage are different decisions.
2. Direct-package tests are an unsafe proxy for sufficient validation.
3. Reverse dependency closure is useful but incomplete for inputs outside
   Cargo ownership.
4. Features, targets, lints, release modes, doctests, execution, and repository
   gates must remain explicit.
5. Unknown changes must widen the plan.
6. Selected-plan success must not be labeled full-suite success.

## Limitations

- The synthetic workspace represents designed failure classes, not all Rust
  repository behavior.
- Timing used warm local WSL2 runs, not cold CI workers.
- The fixture did not exercise proc-macro expansion, build scripts, native
  compilation, cross compilation, generated source, databases, services,
  network dependencies, or multi-language orchestration.
- One public repository establishes transfer, not a universal expected gain.
- The conservative policy used full fallback for every unowned input; reviewed
  repository mappings may permit narrower safe plans.
- Passing all eight mutations does not prove absence of other false negatives.

## Reproduction

Retained session harnesses:

```text
perf-q35/measure_validation_selection.py
perf-q35/measure_parlor_validation.py
```

Retained raw outputs:

```text
perf-q35/results/validation-selection.json
perf-q35/results/parlor-validation.json
```

Commands:

```text
wsl.exe -d Ubuntu-24.04 -- bash -lc "python3 measure_validation_selection.py"
wsl.exe -d Ubuntu-24.04 -- bash -lc "python3 measure_parlor_validation.py"
```
