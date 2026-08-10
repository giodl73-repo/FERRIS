# ECOS-Q02: Foundational Crate Census

**Status:** Complete

**Area:** Ecosystem infrastructure

**Depends on:** ECOS-Q01

## Question

Which crates function as ecosystem infrastructure, and what evidence supports
that classification?

## Starting hypothesis

Foundational status depends on dependency reach, interchange role,
stewardship, maintenance, portability, and replacement cost—not downloads
alone.

## Investigation focus

- Define foundational-crate criteria.
- Inventory candidate crates by capability.
- Measure dependency and feature closures.
- Record ownership, governance, release, license, MSRV, unsafe, macro,
  build-script, and native boundaries.

## Decision informed

Select the crates whose contracts and lifecycle require deeper Crates Series
verification.

## Primary roles

Ecosystem Strategist, Rust Maintainer, AI Assurance Skeptic.

## Decision

Adopt a structural foundational-crate test and carry nineteen exact releases
into ECOS-Q03 through ECOS-Q09 as a verification queue:

- nine contract foundations;
- three construction foundations;
- five platform or build foundations; and
- two implementation substrates.

This is not an approved stack or dependency recommendation. Domain foundations,
application choices, widely reused utilities, and focused helpers remain
separate categories.

See
[Rust foundational crate census](../../2026-08-09-rust-foundational-crate-census.md).
