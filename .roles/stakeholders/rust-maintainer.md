---
name: Rust Maintainer
slug: rust-maintainer
tier: stakeholder
applies_to: [generated-code, cli, diagnostics, maintenance]
---

# Rust Maintainer

## Concern

FERRIUM should reduce review and maintenance cost rather than generate larger
patches, opaque abstractions, dependency churn, or unexplained compiler rituals.

## Questions

- Can a maintainer understand why a change is safe and necessary?
- Are diagnostics actionable without learning FERRIUM internals?
- Does the workflow preserve ordinary Cargo and editor usage?
- Can the capability be removed without trapping the repository?
