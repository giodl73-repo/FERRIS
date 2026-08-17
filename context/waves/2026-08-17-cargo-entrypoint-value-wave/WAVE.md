# Wave: Cargo Entrypoint Value

Status: Closed after Pulse 01

## Product outcome

Give a Ferris user the same current bounded command surface from `ferris` and
`cargo ferris`, including `validation-plan`, by shipping a conventional
`cargo-ferris` adapter that only normalizes Cargo's injected `ferris`
argv token and keeps `ferris-cli` binary-only by compiling one private shared
CLI module into both binaries.

## Classification

Release/readiness wave with one bounded implementation pulse.

## Budget

- exactly one pulse;
- exactly one implementation attempt;
- exactly one review record; and
- no successor chain, diagnostic pulse edits, or new command capability.

## Completion condition

The wave is complete only when Ferris ships a `cargo-ferris` binary that:

- exposes the same current command IDs as `ferris`;
- keeps the existing command semantics and typed result envelopes unchanged;
- proves and handles Cargo's injected external-subcommand argv token with
  platform-appropriate matching without altering direct `ferris` parsing;
- keeps help and version banners truthful for `ferris`, direct
  `cargo-ferris`, and `cargo ferris`;
- keeps direct `cargo-ferris` help and command invocation sensible; and
- passes the bounded validation commands recorded in Pulse 01.

## Abandonment condition

Stop and report `stop-value-exhausted` without edits or follow-on pulses if
Cargo external-subcommand argv behavior cannot be proven locally, or if
achieving truthful parity would require a new command capability, changed
command semantics, another architectural layer, or a successor chain.

## Owner actions

| Repo | Action |
|---|---|
| FERRIS | Implement, validate, review, and retain all product changes locally |
| TRACKER | No-op; keep this wave separate from portfolio state |
| Cargo and external repositories | No-op; ordinary Cargo external-subcommand behavior remains authoritative |

## Pulse table

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Cargo entrypoint parity | Complete | Implemented a conventional `cargo-ferris` adapter with current-surface parity and no semantic expansion |

## Non-goals

- adding any new Ferris command, flag, default, or owner capability;
- changing `plan`, `validation-plan`, `explain`, `graph`, `doctor`, or
  `profile-diff` semantics;
- replacing Cargo command discovery or adding a parallel launcher;
- network access, publication, or diagnostic pulse edits; and
- successor planning, custody hardening, or broader packaging infrastructure.

## Completion gate

- `cargo-ferris` is packaged as a built binary, `ferris-cli` remains
  binary-only, and both binary roots compile one private CLI engine;
- only the injected Cargo `ferris` argv token is normalized, using
  platform-appropriate matching, and direct `ferris` keeps literal argument
  parsing;
- help, version, and representative `validation-plan` JSON behavior are
  covered across direct `ferris`, direct `cargo-ferris`, and Cargo-style
  invocation, including mixed-case Windows Cargo invocation where supported;
- malformed invocation remains a typed invalid result;
- README install/usage guidance matches the shipped entrypoints;
- `cargo test -p ferris-cli --bin ferris --bin cargo-ferris --test cli`,
  `cargo metadata --format-version 1 --no-deps --locked --offline
  --manifest-path crates/ferris-cli/Cargo.toml`, `cargo check --workspace`,
  `rustfmt --check` over the changed CLI files, and `git diff --check` pass;
  and
- one review record captures the Product Value Governor budget, the Cargo argv
  proof, the role dispositions, and the no-successor closeout.
