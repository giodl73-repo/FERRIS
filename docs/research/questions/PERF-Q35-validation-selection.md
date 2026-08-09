# PERF-Q35: Impact-Aware Validation Selection

**Status:** Complete

**Area:** Validation loop

**Depends on:** PERF-Q03, PERF-Q17, PERF-Q20, PERF-Q21

## Question

Can build-impact analysis reduce check, lint, test, and release feedback time
without concealing behavioral coverage loss?

## Starting hypothesis

Dependency and test-impact evidence can prioritize targeted feedback, but
repository-mandated gates and high-risk changes must remain explicit.

## Investigation focus

- Map source and API changes to affected targets and tests.
- Compare recommendations with held-out failures and full-suite results.
- Define uncertainty, mandatory gates, and human approval boundaries.

**Model changes if:** impact prediction misses meaningful failures too often to
support narrower feedback.

## Decision informed

Whether BI-04 can advance to a bounded validation-planning prototype.

## Primary roles

Validation Checker, AI Assurance Skeptic, Rust Safety Steward.

## Decision

Adopt a read-only validation-plan and coverage ledger. Prototype package
selection only when required activities, features, targets, profiles,
doctests, execution modes, and repository gates remain explicit. Use reverse
dependency closure for owned Rust changes and full fallback for unknown or
unmapped inputs. Require periodic full reference runs, held-out mutation
testing, visible selected-versus-full evidence, and human approval.

Changed-package tests caught only two of eight seeded failures. Conservative
selection caught all eight and reduced the warm synthetic median 57.1%.
Public PARLOR preserved its documented contract with a bounded 9.4% gain.

See
[Impact-aware validation selection](../2026-08-09-impact-aware-validation-selection.md).
