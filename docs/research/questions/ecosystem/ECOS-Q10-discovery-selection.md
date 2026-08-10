# ECOS-Q10: Discovery and Selection

**Status:** Complete

**Area:** Maintainer decisions

**Depends on:** ECOS-Q01 through ECOS-Q09

## Question

Can evidence improve crate selection beyond keyword search, popularity,
anecdote, and stale recommendation lists?

## Starting hypothesis

Selection is multi-dimensional and consumer-specific; a useful system exposes
criteria and tradeoffs rather than one universal score.

## Decision informed

Whether an evidence-backed capability map is a defensible FERRIUM capability.

## Decision

Adopt an evidence-backed discovery and selection record that separates
consumer intent, retrieval source and ranking policy, candidate role and exact
release identity, evidence coverage, mandatory eligibility, tradeoff frontier,
owner decision, and renewal. Treat search rank, downloads, recency, categories,
keywords, curation, reputation, and composite scores as attributed
candidate-generation signals. Preserve declared, inferred, measured, stale,
conflicting, unsupported, and unknown states without producing one universal
crate score or automatic approval.

See
[Rust crate discovery and selection](../../2026-08-10-rust-crate-discovery-selection.md)
and
[EXP-01](../../ecos-q10-discovery-selection/results/EXP-01-discovery-selection-matrix.md).

## Primary roles

Ecosystem Strategist, Rust Maintainer, AI Assurance Skeptic.
