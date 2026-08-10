# BLUE-Q06: Seven-Program Architecture

**Status:** Complete

## Research question

How should the complete PERF, ECOS, and Blueprint/product research corpus be
organized into a small number of bounded programs without losing findings,
creating a monolith, or presenting every internal capability as a separate
product?

## Decision

Use seven programs:

1. Ferris;
2. Typebook;
3. Profiles;
4. Blueprint;
5. Query Forest;
6. Conformance; and
7. Ecosystem Bridge.

Ferris remains the only public enterprise build-system product and CLI.
Typebook remains independently useful and product-neutral. The remaining
programs are replaceable capability and governance boundaries.

## Outputs

- [Ferris seven-program synthesis](../../2026-08-10-ferris-seven-program-synthesis.md)
- [Ferris seven-program architecture](../../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)
- [EXP-01 research closure matrix](../../ferris-seven-programs/results/EXP-01-research-closure-matrix.md)
- [Nine-role review](../../../plans/reviews/FERRIS-SEVEN-PROGRAM-ROLE-REVIEW.md)

## Input corpus

- PERF-Q01 through PERF-Q36;
- ECOS-Q01 through ECOS-Q12; and
- BLUE-Q01 through BLUE-Q05.

These 53 input questions are all represented in EXP-01. BLUE-Q06 records the
synthesis decision itself.

## Non-goals

- implementation authority;
- seven public CLIs or products;
- merging Typebook into Ferris;
- a universal graph, resolver, cache, or certification;
- automatic mutation; and
- transferring current owner authority to Ferris.
