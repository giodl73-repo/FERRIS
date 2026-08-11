# Ferris Declared Graph Completion Review

Date: 2026-08-11
Scope: Pulse 02 local declared-workspace `graph`
Disposition: Complete on Windows and Unix; applicable held-out fixture passed
Implementation authority: No expansion

## Measured result

`ferris graph` uses the Pulse 01 Cargo invocation:

```console
cargo metadata --format-version 1 --no-deps --offline --locked --manifest-path <Cargo.toml>
```

It emits `ferris.workspace-graph/v0` with:

- Cargo-reported workspace packages as nodes;
- Cargo-declared dependencies as edges;
- workspace-relative paths;
- dependency alias, kind, optional state, and target condition;
- unique path-backed workspace targets where Cargo supplies enough evidence;
- explicit unresolved external, outside-workspace, ambiguous, and unmatched
  targets;
- stable canonical ordering and graph identity;
- 10,000-node and 50,000-edge hard bounds; and
- no partial success when either bound is exceeded.

The graph is non-executable and explicitly rejects affected, invalidation,
schedule, build-order, freshness, validation, native, ABI, and runtime claims.

## Validation

Windows and Ubuntu 24.04.4 WSL2 used Rust and Cargo 1.95.0.

```console
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Results on both environments:

- 11 core tests passed;
- 7 CLI tests passed;
- no lint warning;
- two nodes and three declared edges in the development graph fixture;
- path-backed workspace edges resolved;
- registry dependency remained unresolved;
- alias, development kind, optional state, and `cfg(windows)` condition
  remained visible;
- checkout-absolute paths were absent from successful output;
- checkout path and Cargo JSON order did not change graph identity;
- Windows extended-length and backslash dependency paths normalized correctly;
- graph bounds returned `blocked` without a record; and
- Windows and Unix produced:

```text
graph:4468bf268af9c45dfea90c35f680b9f27cac989625b01202546a2fa09d5127f9
```

An independent implementation review found no blocking issue. Its one
non-blocking Windows-path regression gap was fixed before completion.

Independent held-out scoring froze
`cfac256768aba20fa3b490c5008ea4bf74810776`. The custodian classified all 12
fixtures before execution:

- FHIF-012 passed `P02-DECLARED-GRAPH`;
- no applicable fixture failed, blocked, or was invalid; and
- eleven fixtures were outside Pulse 02 and were not executed.

The
[public-safe score receipt](../../simulations/held-out/PUBLIC_SAFE_SCORE_RECEIPT_002.md)
contains dispositions and output digests without hidden inputs or oracles.

## Role dispositions

### Rust Safety Steward

Accept. Safe metadata projection remains distinct from compilation, safety,
reachability, and runtime evidence.

### Compiler Performance Engineer

Accept with no performance claim. The declaration graph is not a unit graph,
schedule, critical path, invalidation graph, or freshness model.

### Interop Boundary Auditor

Accept. Alias, kind, optional state, condition, path evidence, and unresolved
state remain distinct; no ABI or native relationship is inferred.

### AI Assurance Skeptic

Accept. No model participates, missing targets remain unresolved rather than
guessed, and held-out scoring returned only public-safe results.

### Ecosystem Strategist

Accept. Cargo owns the declarations. Ferris does not resolve dependencies or
replace `cargo tree`.

### Rust Maintainer

Accept. Human and JSON output use package and dependency vocabulary, expose
limitations, preserve ordinary Cargo, and require no manifest changes.

### Native Platform Adopter

Accept on the recorded Windows and Unix environments. No native, SDK,
packaging, deployment, or service graph is claimed.

### Scope Keeper

Accept. One command and one experimental schema were added within the existing
two crates and fixed resource bounds.

### Validation Checker

Accept. Positive, unresolved, alias, kind, condition, ordering, path,
cross-platform, bound, failure, and applicable held-out evidence are present.

## Remaining gates

- approve a new pulse before affected-only scope, query, execution, or any
  other capability.

## Decision

Pulse 02 is complete on the recorded Windows and Unix environments and passed
all applicable held-out scoring. Its declared graph is the maximum authority
until a later separate pulse.
