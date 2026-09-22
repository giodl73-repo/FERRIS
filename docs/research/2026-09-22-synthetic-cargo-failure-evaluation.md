# Synthetic Cargo Failure Evaluation

Status: Complete controlled Windows evaluation

## Question

Does passive Cargo failure diagnosis recognize real missing path-dependency
stderr across the retained synthetic repository topologies without exposing
the diagnostic or modifying those repositories?

## Method

The evaluation used Ferris commit
`8f568b9869595b264ceee0144243a8c4706fa86a` with Cargo and rustc
`1.95.0 (ed80dadd6a 2026-06-18)`. For each repository, the harness:

1. required a clean source checkout and read package structure through
   `cargo metadata --no-deps`;
2. extracted `HEAD` into a disposable archive copy;
3. used Cargo to add one generated external path dependency to a real package;
4. removed only that generated dependency target;
5. captured the resulting `cargo metadata --no-deps` stderr in memory; and
6. passed it to `ferris diagnose-cargo --stderr - --format json`.

Raw Cargo stderr was not written into this repository or retained in the
report. Each result was checked for the temporary root and generated dependency
name. The source checkouts were checked again after evaluation.

## Results

| Repository shape | Packages in evaluated workspace | Cargo exit | Input bytes | Ferris classification | Diagnostic |
|---|---:|---:|---:|---|---|
| boundaries | 3 | 101 | 595 | `dependency` | `FERRIS-CARGO-DEPENDENCY-BLOCKED` |
| chain | 3 | 101 | 574 | `dependency` | `FERRIS-CARGO-DEPENDENCY-BLOCKED` |
| fanout | 4 | 101 | 576 | `dependency` | `FERRIS-CARGO-DEPENDENCY-BLOCKED` |
| federated workspace 00 | 8 | 101 | 628 | `dependency` | `FERRIS-CARGO-DEPENDENCY-BLOCKED` |
| scale | 96 | 101 | 594 | `dependency` | `FERRIS-CARGO-DEPENDENCY-BLOCKED` |

Every report set `classified=true`, `raw_output_retained=false`, and
`executable=false`. No report contained the generated dependency name or
temporary path. All five source repositories remained clean, and the temporary
evaluation tree was removed.

## Control Corrections

The first construction omitted an existing dependency directory from an
archive. Because that dependency was also a workspace member, Cargo reported a
missing workspace manifest rather than a missing dependency. The observation
was not treated as a classifier failure. One corrective construction generated
an external path dependency with Cargo and then removed only its target.

Because the disposable root was initially placed below the Ferris `target`
directory, `cargo new` temporarily appended the generated crates to the
enclosing Ferris workspace. The entries were detected after cleanup and the
manifest was restored exactly. Future harnesses should place generated crates
outside any Cargo workspace or pass an explicit workspace-isolation control.

## Decision

The existing dependency vocabulary is adequate for this bounded real-Cargo
missing-path case across all five shapes. No classifier or schema change is
justified by this evaluation.

This is controlled synthetic Windows evidence only. It does not establish
owner adoption, cross-platform behavior, prevalence of this failure shape,
semantic root cause, repair correctness, or a production support commitment.
