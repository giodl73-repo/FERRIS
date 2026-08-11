# Held-Out Score Cutoff 005

State: Scored; all applicable Pulse 05 fixtures passed
Frozen commit: `95a0b905fb31c908a241d57ae17d984e16d8c053`
Tag: `ferris-passive-doctor-hardening-pulse-05-cutoff`
Authorized claims: corrected owner-toolchain context for current read-only commands

## Scope

This cutoff evaluates:

- selected-manifest-directory Cargo owner context;
- inherited owner toolchain selection;
- offline, rustup no-auto-install, and no-update guards;
- corrected plan, explanation, and graph evidence and invocation identity;
- preserved bounded read-only plan and graph behavior; and
- fixed invalid-control behavior.

Doctor manifest/process/output bounds and owner-output identity are covered by
development tests. No existing sealed fixture specifically evaluates passive
doctor, so this cutoff makes no held-out doctor claim.

After this cutoff was published, independently sealed FHIF-013 produced the
first doctor-specific score and failed. That result does not alter the two
passing plan/graph claims authorized here, but it confirms that this cutoff
provides no doctor held-out evidence.

## Environments

Use the Windows and Ubuntu 24.04 WSL2 environment classes recorded by
`SCORE_CUTOFF_001.md`. The custodian verified the cutoff, tag, command
bindings, and all 12 sealed package digests before execution.

## Command binding

| SHA-256 | Command |
|---|---|
| `810e75f0d3dee10e68076ea4a252c5cad284ef355c74785fc33f0cd493f3b98e` | `cargo fmt --all -- --check` |
| `ec4556a178527361344d876629bb9533c63fc8fcd216f423dd8bce76787fce86` | `cargo test --workspace` |
| `196d9a67d5f68bf4c7de40ebb3c510406e3b856cbbb539e5eae8a61a39b39e28` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `e27f4b9207356cd4a90df336fdd9f707c15757ca4442bebd3bf5f6e5ece6e2fa` | `cargo run --quiet -p ferris-cli -- plan --workspace-id held-out/<opaque-fixture-id> --manifest-path <fixture>/Cargo.toml --format json` |
| `9b9b3e3649803d145757139d0fe3441f6cbe946254059750a49c152fa1ca717a` | `cargo run --quiet -p ferris-cli -- explain --workspace-id held-out/<opaque-fixture-id> --manifest-path <fixture>/Cargo.toml --format json` |
| `ac1965501fa2c802ed624748210e1eab083b9aa0cbbe079efe8bbc1bbd192267` | `cargo run --quiet -p ferris-cli -- graph --workspace-id held-out/<opaque-fixture-id> --manifest-path <fixture>/Cargo.toml --format json` |

## Result

- applicable: 2;
- pass: 2;
- fail: 0;
- applicable blocked or invalid: 0; and
- out of scope and not executed: 10.

See `PUBLIC_SAFE_SCORE_RECEIPT_005.md`.

## Stop rule

Stop if the commit, tag, command binding, package digest, environment class,
or capability boundary differs. Earlier cutoffs MUST NOT substitute for this
current owner-context executable.
