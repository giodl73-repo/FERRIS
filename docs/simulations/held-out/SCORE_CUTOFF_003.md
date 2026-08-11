# Held-Out Score Cutoff 003

State: Historical; scored Pulse 03 cutoff superseded for current owner-toolchain evidence
Frozen commit: `c3590a39fd053a66996909b87eaf7ca7ac73ded4`
Tag: `ferris-read-only-hardening-pulse-03-cutoff`
Authorized claims: corrected local `plan`, `explain`, and declared `graph`

Pulse 05 changed Cargo metadata working-directory and rustup guard semantics,
evidence fields, and invocation identity. This immutable score remains valid
for its frozen commit but MUST NOT be reused for the current executable.

## Scope

This cutoff evaluates the bounded Pulse 03 corrections:

- explicit portable workspace identity;
- workspace-separated plan and graph identities;
- command- and mode-bound invocation identity;
- complete human and JSON material semantics;
- portable-equivalent Cargo evidence and owner-output digest;
- safe Cargo failure diagnostics with source digests;
- JSON-mode invalid command envelopes; and
- preserved Pulse 01 and Pulse 02 read-only behavior.

Affected scope, query, execution, mutation, Query Forest persistence, build
order, invalidation, scheduling, freshness, validation coverage, native, ABI,
runtime, connector, MCP, AI, approval, deployment, and remote evidence remain
outside this score.

## Environments

Use the Windows and Ubuntu 24.04 WSL2 environment classes recorded by
`SCORE_CUTOFF_001.md`. The custodian verified the cutoff, tag, command
binding, and all 12 sealed package digests before execution.

## Command binding

Every applicable command uses:

```console
--workspace-id held-out/<opaque-fixture-id>
```

| SHA-256 | Command |
|---|---|
| `810e75f0d3dee10e68076ea4a252c5cad284ef355c74785fc33f0cd493f3b98e` | `cargo fmt --all -- --check` |
| `ec4556a178527361344d876629bb9533c63fc8fcd216f423dd8bce76787fce86` | `cargo test --workspace` |
| `196d9a67d5f68bf4c7de40ebb3c510406e3b856cbbb539e5eae8a61a39b39e28` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `e27f4b9207356cd4a90df336fdd9f707c15757ca4442bebd3bf5f6e5ece6e2fa` | `cargo run --quiet -p ferris-cli -- plan --workspace-id held-out/<opaque-fixture-id> --manifest-path <fixture>/Cargo.toml --format json` |
| `9b9b3e3649803d145757139d0fe3441f6cbe946254059750a49c152fa1ca717a` | `cargo run --quiet -p ferris-cli -- explain --workspace-id held-out/<opaque-fixture-id> --manifest-path <fixture>/Cargo.toml --format json` |
| `ac1965501fa2c802ed624748210e1eab083b9aa0cbbe079efe8bbc1bbd192267` | `cargo run --quiet -p ferris-cli -- graph --workspace-id held-out/<opaque-fixture-id> --manifest-path <fixture>/Cargo.toml --format json` |

## Custody and result

The independent custodian classified all 12 fixtures before execution:

- 2 applicable;
- 2 passed;
- 0 failed;
- 0 applicable blocked or invalid; and
- 10 out of scope and not executed.

See `PUBLIC_SAFE_SCORE_RECEIPT_003.md`. Hidden inputs and oracle predicates
remain sealed.

## Stop rule

Scoring stops if the commit, tag, command, package digest, environment class,
or capability boundary differs. Prior cutoffs MUST NOT be substituted for
this corrected executable.
