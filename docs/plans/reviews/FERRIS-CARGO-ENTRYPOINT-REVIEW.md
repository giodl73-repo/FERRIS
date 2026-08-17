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
`validation-plan`, without changing command semantics, while `ferris-cli`
stays binary-only.

Approved budget: one pulse, one implementation attempt, one review record, and
no successor chain, diagnostic pulse edit, or new command capability.

Completion condition: prove Cargo's external-subcommand argv shape, normalize
only the injected `ferris` token for `cargo-ferris` with platform-appropriate
matching, preserve direct `cargo-ferris` usability, preserve direct `ferris`
parsing, keep version banners truthful for `ferris`, `cargo-ferris`, and
`cargo ferris`, keep `ferris-cli` binary-only, and pass targeted CLI
validation plus metadata, workspace, changed-file rustfmt, and diff checks.

Abandonment condition: stop `stop-value-exhausted` if Cargo argv behavior
cannot be proven locally or if parity requires changed command semantics,
another architectural layer, or a successor pulse.

Measured result: the pulse stayed inside budget, proved the Cargo argv shape,
and shipped one shared-engine `cargo-ferris` adapter with no semantic
expansion, truthful invocation banners, Windows mixed-case injected-token
handling, and binary-only packaging restored. No continuation is approved.

## Rust Safety Steward

Accept. The change remains safe Rust, adds no `unsafe`, and only adjusts CLI
argument normalization at the adapter boundary.

## Compiler Performance Engineer

Accept with no performance claim. Packaging a second binary and normalizing one
argv token does not claim faster builds or cheaper validation.

## Interop Boundary Auditor

Accept. Cargo remains authoritative for external-subcommand discovery. Ferris
only consumes the proved injected token with platform-appropriate matching and
does not invent another launcher or owner semantic layer.

## AI Assurance Skeptic

Accept. The adapter behavior is grounded in a recorded local argv probe, and
malformed invocation remains a typed invalid result instead of being guessed
into success.

## Ecosystem Strategist

Accept. This uses Cargo's standard `cargo-<name>` pattern rather than a weaker
parallel wrapper, and it preserves ordinary Cargo install and PATH discovery.

## Rust Maintainer

Accept. One shared private CLI module now serves both binaries without a public
library target, avoiding duplicated dispatch logic while keeping direct
`cargo-ferris` help and errors explicit.

## Native Platform Adopter

Accept for the recorded local packaging path. A user can install `ferris` and
`cargo-ferris` together, invoke either binary directly, use `cargo ferris`,
and on Windows rely on mixed-case Cargo invocation where the platform resolves
the binary.

## Scope Keeper

Accept. The pulse adds only current-surface entrypoint packaging and tests. No
new command, defaulting behavior, query/action capability, public library
crate, or platform claim was introduced.

## Validation Checker

Accept. The pulse records a reproducible Cargo argv probe, binary help checks,
version checks, JSON parity checks, mixed-case Windows coverage, binary-only
metadata coverage, malformed-invocation coverage, workspace check, formatting
check, and diff check.

## Autonomy Supervisor

Accept. The approved outcome, budget, completion test, and abandonment
condition were recorded before implementation. One pulse was consumed, no retry
or successor was started, and no reviewer finding was converted into another
loop.

Control record:

- product outcome: same current bounded command surface from `ferris` and
  `cargo ferris`, including `validation-plan`;
- work completed: retained the `cargo-ferris` binary, moved shared CLI logic
  into one private source module compiled into both binary roots, normalized
  only Cargo's injected token with platform-appropriate matching, kept version
  banners truthful for each invocation form, and added parity plus binary-only
  metadata tests;
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
cargo test -p ferris-cli --bin ferris --bin cargo-ferris --test cli --quiet --locked
cargo build -p ferris-cli --bins --locked
cargo metadata --format-version 1 --no-deps --locked --offline --manifest-path crates/ferris-cli/Cargo.toml
cargo check --workspace --locked
rustfmt --check crates/ferris-cli/src/main.rs crates/ferris-cli/src/entrypoint.rs crates/ferris-cli/src/bin/cargo-ferris.rs crates/ferris-cli/tests/cli.rs
$env:PATH="C:\src\FERRIS-cargo-entrypoint\target\debug;$env:PATH"
target\debug\ferris.exe --help
target\debug\ferris.exe --version
target\debug\cargo-ferris.exe --help
target\debug\cargo-ferris.exe --version
target\debug\cargo-ferris.exe validation-plan --workspace-id ferris.test/simple --manifest-path tests\fixtures\simple-workspace\Cargo.toml --changed-path tests\fixtures\simple-workspace\alpha\src\lib.rs --changed-package fixture-alpha --format json
cargo ferris --help
cargo ferris --version
cargo ferris validation-plan --workspace-id ferris.test/simple --manifest-path tests\fixtures\simple-workspace\Cargo.toml --changed-path tests\fixtures\simple-workspace\alpha\src\lib.rs --changed-package fixture-alpha --format json
cargo Ferris --help
cargo Ferris --version
cargo Ferris validation-plan --workspace-id ferris.test/simple --manifest-path tests\fixtures\simple-workspace\Cargo.toml --changed-path tests\fixtures\simple-workspace\alpha\src\lib.rs --changed-package fixture-alpha --format json
git diff --check
```

Result summary:

- direct `cargo-ferris --help` is sensible, `cargo ferris --help` reports the
  Cargo-style usage line, and both expose the same current command IDs as
  `ferris --help`;
- version banners now match `ferris`, direct `cargo-ferris`, and
  `cargo ferris`;
- direct `cargo-ferris validation-plan ... --format json` matches
  `ferris validation-plan ... --format json`;
- Cargo-style `cargo ferris validation-plan ... --format json` matches
  `ferris validation-plan ... --format json`;
- Windows mixed-case `cargo Ferris` invocation is supported for help, version,
  and `validation-plan`, and normalizes to the same Cargo-style semantics;
- `cargo metadata` reports `ferris-cli` as binary-only with `cargo-ferris` and
  `ferris` bin targets and no lib target;
- malformed direct `cargo-ferris` invocation remains a typed invalid result
  with direct-binary help guidance; and
- `ferris` does not accidentally strip a literal extra `ferris` token.

## Decision

Pulse 01 is complete. Ferris now ships a conventional shared-engine
`cargo-ferris` adapter that makes the current `cargo ferris` surface truthful
without widening product authority.
