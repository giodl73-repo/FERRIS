# BLUE-Q05: Ferris Product Name

**Status:** Complete

## Research question

Should Ferris itself be the public enterprise build-system product, with
Blueprint retained as the internal planning model, and what command and package
boundaries follow from that decision?

## Decision

Use:

```console
ferris
cargo ferris
```

Ferris is the public product. Blueprint is the internal model and planning
engine. One semantic engine serves both adapters.

The exact `ferris` crates.io package is occupied by an unrelated timer-wheel
library, so published packages use qualified names while still providing the
`ferris` executable.

## Output

- [Ferris product naming decision](../../2026-08-10-ferris-product-naming.md)

## Specifications opened

- PRODUCT-001;
- VIEW-001; and
- CONFORMANCE-001.

## Non-goals

- implementation authority;
- claiming official Rust affiliation;
- claiming the occupied `ferris` package;
- separate enterprise and Cargo engines;
- replacing Cargo semantics; and
- formal film-themed branding.

