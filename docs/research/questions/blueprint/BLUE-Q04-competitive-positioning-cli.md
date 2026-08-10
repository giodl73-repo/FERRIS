# BLUE-Q04: Competitive Positioning and CLI

**Status:** Complete

**Naming note:** BLUE-Q05 supersedes the public product and command spelling.
BLUE-Q04 remains authoritative for the competitive category, one-engine
architecture, Cargo boundary, adoption wedge, and claim limits.

## Research question

How should FERRIS Blueprint position against enterprise build systems,
affected-task orchestrators, Rust task tools, CI systems, caches, and
environment managers, and should its public interface be standalone, a Cargo
subcommand, or both?

## Decision

Position FERRIS Blueprint as Cargo-native cross-workspace orchestration and
application control.

Use one engine with two entrypoints:

```console
ferris blueprint
cargo blueprint
```

- `ferris blueprint` exposes complete application, repository, contract,
  policy, profile, CI, and deployment scope.
- `cargo blueprint` defaults to the current Cargo workspace through Cargo's
  external-subcommand convention.

Do not use an unqualified `blueprint` binary or generic `blueprint-*` package
family because existing crates and binaries occupy that namespace.

## Outputs

- [Blueprint competitive positioning and CLI strategy](../../2026-08-10-blueprint-competitive-positioning.md)
- [EXP-01 competitor matrix](../../blue-q04-competitive-positioning/results/EXP-01-competitor-matrix.md)

## Initial adoption wedge

Affected-only planning, checks, and tests across several existing Cargo
workspaces, with explanation and full-reference fallback.

## Non-goals

- Cargo replacement;
- BUILD-file migration in the first proof;
- unsupported hermeticity or remote-execution claims;
- a second independent command implementation;
- only a Cargo subcommand;
- only an unqualified standalone Blueprint command; and
- implementation before CLI and conformance specifications.
