# Held-Out Score Cutoff 002

State: Historical; scored Pulse 02 cutoff superseded for current identity and output claims
Frozen commit: `cfac256768aba20fa3b490c5008ea4bf74810776`
Authorized claims: Pulse 02 declared-workspace `graph` only

This immutable score remains valid for its frozen commit. Pulse 03 changed the
required command line, record identity inputs, evidence fields, rendering, and
diagnostic behavior, so this cutoff MUST NOT be reused to score the corrected
implementation.

## Scope

This cutoff evaluates:

- explicit local Cargo manifest selection;
- Cargo-reported workspace packages as nodes;
- Cargo-declared dependencies as edges;
- unique path-backed workspace target matching;
- unresolved external and non-member targets;
- alias, kind, optional state, and target condition retention;
- workspace-relative portable paths and graph identity;
- canonical ordering;
- 10,000-node and 50,000-edge hard bounds;
- no partial success after a bound failure;
- no sibling workspace discovery; and
- equivalent Windows and Unix graph semantics.

Plan and explanation remain covered by Cutoff 001. Affected, query, execution,
mutation, Query Forest persistence, build order, invalidation, scheduling,
freshness, validation, native, ABI, runtime, connector, MCP, AI, approval, and
remote-evidence claims are outside this score.

## Environments

Use the exact Windows and Ubuntu WSL2 environments recorded by
`SCORE_CUTOFF_001.md`, with:

- Rust `1.95.0`;
- Cargo `1.95.0`;
- immutable implementation commit
  `cfac256768aba20fa3b490c5008ea4bf74810776`; and
- separate build output outside the source checkout where necessary.

WSL2 remains a Unix renewal environment, not independent Linux hardware or
filesystem proof.

## Command binding

| SHA-256 | Command |
|---|---|
| `810e75f0d3dee10e68076ea4a252c5cad284ef355c74785fc33f0cd493f3b98e` | `cargo fmt --all -- --check` |
| `ec4556a178527361344d876629bb9533c63fc8fcd216f423dd8bce76787fce86` | `cargo test --workspace` |
| `196d9a67d5f68bf4c7de40ebb3c510406e3b856cbbb539e5eae8a61a39b39e28` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `26a5b09d0cb0b9569ff6895d27c3011b3d88311e01d3f72dd5b892810db0558f` | `cargo run --quiet -p ferris-cli -- graph --manifest-path <fixture>/Cargo.toml --format json` |

The custodian MAY substitute a prebuilt binary from the same commit when the
build command, target, and binary digest are retained.

## Custody and classification

The independent custodian MUST:

1. verify every applicable sealed package digest;
2. classify all 12 fixtures before execution;
3. execute only fixtures whose graph claims fall within Pulse 02;
4. count out-of-scope fixtures as neither pass nor fail;
5. preserve private edits and oracle predicates; and
6. return only public-safe requirement codes, dispositions, result classes,
   output digests, and non-revealing remediation.

## Stop rule

Stop without changing the implementation or oracle when:

- commit, tag, environment, command, or package digest differs;
- a fixture was exposed during development;
- scoring requires dependency resolution or owner execution;
- a result would require truncation rather than the specified blocked state;
  or
- scoring requires any capability outside Pulse 02.

## Public-safe result

- 1 applicable;
- 1 passed;
- 0 failed;
- 0 applicable blocked or invalid; and
- 11 out of scope and not executed.

See
[`PUBLIC_SAFE_SCORE_RECEIPT_002.md`](PUBLIC_SAFE_SCORE_RECEIPT_002.md).
