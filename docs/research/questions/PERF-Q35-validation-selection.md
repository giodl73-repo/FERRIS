# PERF-Q35: Impact-Aware Validation Selection

**Status:** Planned

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
