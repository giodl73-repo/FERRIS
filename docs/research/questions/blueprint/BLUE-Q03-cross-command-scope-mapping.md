# BLUE-Q03: Cross-Command Scope Mapping

**Status:** Complete

## Research question

How should Blueprint map repository files, Rust modules and items, Cargo
packages, targets and units, compiler owners, tests, contracts, native
components, services, platforms, deployment, validation, and evidence across
commands whose compilation and runtime scopes differ?

## Decision

Create SCOPE-001 separately from IDENTITY-001.

Scope is multi-dimensional. Blueprint records owner, subject, activity,
configuration, platform, lifecycle, and evidence coordinates joined by typed,
directional, cardinality-aware, conditional mappings.

Every command separates:

- package and target selection;
- activity and compiler scope;
- artifact scope;
- runtime execution scope;
- validation and capability coverage; and
- omitted, unsupported, stale, not-observed, and unknown scope.

Unknown mappings widen to the smallest safe owner boundary.

AI starts from stable owner-native anchor scopes. It may propose finer semantic
mappings, but deterministic policy or human approval must authorize any
narrowing that removes work or validation. Scope-detail collection remains
subject to an explicit economics budget.

## Outputs

- [Blueprint cross-command scope model](../../2026-08-10-blueprint-cross-command-scope-model.md)
- [EXP-01 cross-command scope matrix](../../blue-q03-scope-mapping/results/EXP-01-cross-command-scope-matrix.md)

## Non-goals

- one universal scope hierarchy;
- package selection as complete validation scope;
- test filters as compilation scope;
- file paths as semantic identity;
- compilation as runtime or deployment coverage;
- automatic validation deletion; and
- implementation before SCOPE-001 and held-out conformance.
