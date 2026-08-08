# Pulse 03: Ferris Evidence Contract

## Goal

Define, but do not yet implement, a product-neutral evidence packet for
compiler-grounded AI changes.

## Changes

- Specify source revision, toolchain, command, diagnostic, test, lint, benchmark,
  and limitation records.
- Distinguish compiler evidence from behavioral and assurance claims.
- Specify future fixtures and compatibility tests for the contract.
- Review the contract through Rust safety and AI assurance roles.

## Validation

- `git grep -n "source revision\\|toolchain\\|diagnostic\\|limitation" -- docs context`
- `git diff --check`

## Status

Pending.
