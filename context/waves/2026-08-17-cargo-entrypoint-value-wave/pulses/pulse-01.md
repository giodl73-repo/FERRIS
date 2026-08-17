# Pulse 01: Cargo Entrypoint Parity

Status: Complete
Implementation authority: Bounded to this document
Budget: One implementation attempt, one review record, no successor

## Goal

Implement a conventional Cargo external-subcommand adapter so that:

1. `ferris` and `cargo ferris` expose the same current command surface,
   including `validation-plan`;
2. `cargo-ferris` reuses the same CLI code as `ferris`;
3. only Cargo's injected `ferris` argv token is normalized when the
   `cargo-ferris` binary is invoked as an external subcommand;
4. direct `cargo-ferris` help and command invocation remain sensible; and
5. no current Ferris command semantics, envelopes, or capabilities change.

## Authorized files

- root Cargo workspace files;
- `crates/ferris-cli/`;
- this wave, its review record, and directly related public documentation.

## Required behavior

- ship a `cargo-ferris` binary from the existing CLI package;
- share one parser, dispatcher, and typed result path between `ferris` and
  `cargo-ferris`;
- prove Cargo's argv shape locally before relying on injected-token
  normalization;
- strip only the leading injected `ferris` token for `cargo-ferris`, leaving
  all other arguments intact;
- keep direct `cargo-ferris --help` and direct `cargo-ferris <command> ...`
  usable;
- include `validation-plan` in the shared current surface; and
- preserve typed invalid results for malformed invocation and literal extra
  tokens passed to `ferris`.

## Prohibited behavior

- adding a new command, alias family, option, or default;
- changing current command outputs, exit codes, or meaning;
- introducing a parallel Cargo wrapper, manifest mutation, or networked
  install flow;
- editing diagnostic pulses, AGENTS files, or unrelated crates; or
- creating a successor pulse, review loop, or custody layer.

## Acceptance

- a local Cargo argv probe demonstrates that `cargo <name> ...` invokes
  `cargo-<name>` with `<name>` as `argv[1]`;
- targeted CLI tests execute both built binaries for help, representative
  `plan` JSON, representative `validation-plan` JSON, malformed invocation,
  and no-accidental-stripping behavior;
- `cargo test -p ferris-cli --lib --test cli`;
- `cargo check --workspace`;
- `rustfmt --check` over the changed CLI files; and
- `git diff --check`.

## Stop conditions

Stop the pulse rather than widening scope if it requires:

- a second implementation attempt;
- an unproven assumption about Cargo argv behavior;
- changed command semantics or an added capability;
- another crate family, diagnostic pulse, or architectural layer; or
- a successor pulse or review chain.

## Removal

Removal requires deleting the `cargo-ferris` binary target, the shared CLI
adapter normalization, this pulse's tests, and directly related documentation.
It MUST NOT require changing core Ferris command semantics, fixtures, or
ordinary Cargo workflows.

## Review

Measured outcome is recorded in
`docs/plans/reviews/FERRIS-CARGO-ENTRYPOINT-REVIEW.md`.
