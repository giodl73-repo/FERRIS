# Pulse 02: Declared Workspace Graph

Status: Complete on Windows and Unix; applicable held-out fixture passed
Implementation authority: Bounded to this document

## Goal

Implement:

```console
ferris graph --manifest-path <Cargo.toml> [--format human|json]
```

The command projects a bounded, deterministic, non-executable graph from
official Cargo metadata.

## Authorized graph

- nodes are Cargo-reported workspace-member packages;
- edges are Cargo-declared dependencies from those workspace members;
- an edge may target a workspace node only when Cargo reports a path that
  uniquely matches that member;
- registry, git, external path, ambiguous, and otherwise unresolved targets
  remain explicit unresolved declarations;
- conditions, dependency kind, alias, and optional state remain separate; and
- all paths exposed by Ferris are workspace-relative.

This is an experimental declaration projection. It is not a Query Forest root,
resolved Cargo unit graph, build order, affected graph, invalidation graph, or
runtime graph.

## Required behavior

- reuse the exact offline, locked Cargo metadata invocation from Pulse 01;
- emit `ferris.workspace-graph/v0`;
- stable node, edge, graph, and invocation identities;
- canonical ordering independent of Cargo JSON order;
- source owner and command retained;
- unresolved external targets visible rather than omitted;
- maximum 10,000 workspace nodes and 50,000 dependency declarations;
- exceedance returns `blocked` without a partial success record;
- human and JSON views preserve the same material nodes, edges, unknowns, and
  limitations; and
- no checkout-absolute path in successful public output.

## Prohibited behavior

- resolving dependency versions or units independently of Cargo;
- invoking `cargo tree`, `cargo build`, rustc, build scripts, tests, or owner
  code;
- contacting a network or changing a lockfile;
- discovering sibling workspaces;
- claiming change, affected, invalidation, scheduling, freshness, build order,
  validation coverage, native, ABI, runtime, trust, or action semantics;
- adding `query` or any other command;
- reading held-out edits or oracles; or
- changing Pulse 01 schemas or outcomes.

## Acceptance

- the simple workspace produces two nodes and one resolved workspace edge;
- a registry dependency remains an unresolved declaration;
- dependency alias, kind, optional state, and target condition are retained;
- repeated and cross-checkout inputs produce the same graph ID;
- reordered Cargo JSON produces the same canonical graph;
- malformed, unsupported, incomplete, blocked, and internal outcomes preserve
  Pulse 01 process classes;
- a synthetic graph above either bound returns exit 7 without partial output;
- Windows and Unix produce the same graph ID;
- formatting, tests, lint, documentation, and specification graph gates pass.

## Stop conditions

Stop rather than widening scope if this requires:

- Cargo nightly internals or `-Z` flags;
- a parallel resolver;
- full dependency downloads or network access;
- owner execution;
- mutation;
- pagination or durable Query Forest storage;
- more than the existing two product crates; or
- any command beyond `plan`, `explain`, and `graph`.

## Review

Entry approval:
`docs/plans/reviews/FERRIS-DECLARED-GRAPH-ENTRY-REVIEW.md`.

Measured completion:
`docs/plans/reviews/FERRIS-DECLARED-GRAPH-COMPLETION-REVIEW.md`.
