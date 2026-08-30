# Ferris Go PARLOR Local Shadow

Status: passed locally and on GitHub-hosted Ubuntu

## Repository

- Repository: `giodl73-repo/PARLOR`
- Source revision: `50dde4282486053aee8f849a9f483f6acf854347`
- Platform: Windows `x86_64`
- Workspace: six Rust crates implementing chess, backgammon, checkers, Go, a
  shared core, and a CLI

## Owner gates

The Action Plan reproduced PARLOR's documented validation commands in owner
order:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --release --workspace`

The plan bound 36 tracked repository files plus a repository-local copy of the
Cargo executable. It inherited an explicit non-secret environment allowlist and
used no credentials.

## Result

| Evidence | Result |
| --- | --- |
| Action Plan | `sha256:2c8a9093f53559f83bb7bdb6590f6dbd8ae81d86f6c7f7cdaaddf12c24186eaf` |
| Receipt | `sha256:30a2c91013c5c88ef7b921c559f48810a484211c18fd862b277958db58a2cdb0` |
| Formatting | succeeded in 438 ms |
| Clippy | succeeded in 2,178 ms |
| Release workspace tests | succeeded in 4,701 ms |
| Tests observed | 28 passed, 0 failed |
| Cleanup | complete for every lane |
| Receipt verification | valid |

The same three owner commands passed directly. After deleting the complete
`.ferris/` directory, the owner commands passed again and the worktree returned
to a clean state. Ferris therefore preserved direct owner operation in this
bounded local proof.

## What this proves

- `ferris go` can execute a real external Rust repository's documented build,
  lint, and test gates.
- Exact source, command, file, environment, platform, approval, and receipt
  identities survive a complete successful run.
- Removing the local Ferris material does not require an owner workflow edit.

## What this does not prove

- Automatic Action Plan or approval creation.
- Changed-path selection feeding execution.
- A production toolchain-executable contract. This proof staged a bound copy of
  Cargo inside `.ferris/` because GO-WP-003 accepts only repository-relative
  executables.
- Iterations prevented, tail reduction, or production savings.

The most important adoption finding is therefore ergonomic: execution is real,
but the repository-local executable staging and manual plan/approval preparation
must become an owner-friendly, auditable adapter before routine use.

## Consumer integration follow-up

PARLOR PR [#4](https://github.com/giodl73-repo/PARLOR/pull/4) adds that
consumer-owned adapter and a non-required GitHub Actions shadow. Run
[`33291906253`](https://github.com/giodl73-repo/PARLOR/actions/runs/33291906253)
passed:

- PARLOR's direct owner validation;
- exact Ferris commit checkout and build;
- changed-path planning with visible full-workspace fallback for the integration
  PR;
- ten Ferris execution lanes separating quality, product build, test build, and
  package test evidence;
- receipt verification and owner-workflow artifact upload.

An earlier Ubuntu attempt failed closed when inherited `HOME` appeared in a
normal Cargo diagnostic. PARLOR fixed the consumer adapter by staging the
resolved toolchain Cargo binary and narrowing its environment; Ferris's leak
detector was not weakened.

The same PR adds a `parlor-go` legal-move-count feature. Local selection of that
game source chose exactly `parlor-go` and reverse consumer `parlor-cli`; shared
core and repository-level scenarios widened to all six packages.

The expanded scenario suite, negative controls, requirement dispositions, and
remaining non-PARLOR gaps are recorded in
[`2026-08-30-ferris-go-parlor-validation-matrix.md`](2026-08-30-ferris-go-parlor-validation-matrix.md).
