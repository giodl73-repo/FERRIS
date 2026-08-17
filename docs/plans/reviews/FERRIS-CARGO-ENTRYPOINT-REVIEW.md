# Ferris Cargo Entrypoint Review

Date: 2026-08-17
Scope: Pulse 01 `ferris` / `cargo ferris` current-surface parity
Disposition: Complete within one pulse and one implementation attempt
Implementation authority: No expansion

## Product Value Governor

Disposition: `continue-within-budget`

Approved outcome before implementation: a Ferris user can install one
conventional `cargo-ferris` adapter and then reach the same current bounded
command surface from `ferris` and `cargo ferris`, including
`validation-plan`, without changing command semantics.

Approved budget: one pulse, one implementation attempt, one review record, and
no successor chain, diagnostic pulse edit, or new command capability.

Completion condition: prove Cargo's external-subcommand argv shape, normalize
only the injected `ferris` token for `cargo-ferris`, preserve direct
`cargo-ferris` usability, preserve direct `ferris` parsing, and pass targeted
CLI validation plus workspace check, changed-file rustfmt, and diff check.

Abandonment condition: stop `stop-value-exhausted` if Cargo argv behavior
cannot be proven locally or if parity requires changed command semantics,
another architectural layer, or a successor pulse.

Measured result: the pulse stayed inside budget, proved the Cargo argv shape,
and shipped one shared-engine `cargo-ferris` adapter with no semantic
expansion. No continuation is approved.

## Rust Safety Steward

Accept. The change remains safe Rust, adds no `unsafe`, and only adjusts CLI
argument normalization at the adapter boundary.

## Compiler Performance Engineer

Accept with no performance claim. Packaging a second binary and normalizing one
argv token does not claim faster builds or cheaper validation.

## Interop Boundary Auditor

Accept. Cargo remains authoritative for external-subcommand discovery. Ferris
only consumes the proved injected token and does not invent another launcher or
owner semantic layer.

## AI Assurance Skeptic

Accept. The adapter behavior is grounded in a recorded local argv probe, and
malformed invocation remains a typed invalid result instead of being guessed
into success.

## Ecosystem Strategist

Accept. This uses Cargo's standard `cargo-<name>` pattern rather than a weaker
parallel wrapper, and it preserves ordinary Cargo install and PATH discovery.

## Rust Maintainer

Accept. One shared CLI library now serves both binaries, avoiding duplicated
dispatch logic while keeping direct `cargo-ferris` help and errors explicit.

## Native Platform Adopter

Accept for the recorded local packaging path. A user can install `ferris` and
`cargo-ferris` together, invoke either binary directly, and continue to use
ordinary Cargo command discovery.

## Scope Keeper

Accept. The pulse adds only current-surface entrypoint packaging and tests. No
new command, defaulting behavior, query/action capability, or platform claim
was introduced.

## Validation Checker

Accept. The pulse records a reproducible Cargo argv probe, binary help checks,
JSON parity checks, malformed-invocation coverage, workspace check, formatting
check, and diff check.

## Autonomy Supervisor

Accept. The approved outcome, budget, completion test, and abandonment
condition were recorded before implementation. One pulse was consumed, no retry
or successor was started, and no reviewer finding was converted into another
loop.

Control record:

- product outcome: same current bounded command surface from `ferris` and
  `cargo ferris`, including `validation-plan`;
- work completed: added the `cargo-ferris` binary, moved shared CLI logic into
  one library, normalized only Cargo's injected token, updated README
  install/usage guidance, and added parity tests;
- value obtained: the README promise is now true for the current command
  surface without widening semantics;
- remaining risk: current-workspace defaulting remains planned rather than
  implemented, and the argv proof is limited to the recorded Cargo behavior;
- pulses or retries consumed: one pulse, one implementation attempt, zero
  successors;
- proposed next action: stop; no successor is authorized; and
- Product Value Governor disposition: `continue-within-budget`.

## Validation

Cargo external-subcommand argv proof:

```console
rustc .probe-cargo-argv\argvprobe.rs -o .probe-cargo-argv\cargo-argvprobe.exe
$env:PATH="C:\src\FERRIS-cargo-entrypoint\.probe-cargo-argv;$env:PATH"
cargo argvprobe alpha beta --gamma delta
```

Observed output:

```text
ARG0=C:\src\FERRIS-cargo-entrypoint\.probe-cargo-argv\cargo-argvprobe.exe
ARG1=argvprobe
ARG2=alpha
ARG3=beta
ARG4=--gamma
ARG5=delta
```

Recorded commands:

```console
cargo test -p ferris-cli --lib --test cli
cargo check --workspace
rustfmt --check crates/ferris-cli/src/main.rs crates/ferris-cli/src/lib.rs crates/ferris-cli/src/bin/cargo-ferris.rs crates/ferris-cli/tests/cli.rs
git diff --check
```

Result summary:

- direct `cargo-ferris --help` is sensible and exposes the same current command
  IDs as `ferris --help`;
- direct `cargo-ferris plan ... --format json` matches `ferris plan ... --format json`;
- Cargo-style `cargo-ferris ferris validation-plan ... --format json` matches
  `ferris validation-plan ... --format json`;
- malformed direct `cargo-ferris` invocation remains a typed invalid result
  with direct-binary help guidance; and
- `ferris` does not accidentally strip a literal extra `ferris` token.

## Decision

Pulse 01 is complete. Ferris now ships a conventional shared-engine
`cargo-ferris` adapter that makes the current `cargo ferris` surface truthful
without widening product authority.
