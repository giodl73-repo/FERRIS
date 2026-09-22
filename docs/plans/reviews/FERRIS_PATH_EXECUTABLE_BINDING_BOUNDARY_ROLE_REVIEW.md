# Ferris PATH Executable Binding Boundary Role Review

Date: 2026-09-21

Status: accepted no-go; implementation authority withheld

## Findings

| Role | Finding | Disposition |
| --- | --- | --- |
| Product Value Governor | Removing executable staging could reduce adoption cost, but current evidence does not justify a new execution schema before an adopter-maintained declaration demonstrates the blocker. | `stop-value-exhausted` |
| Rust Safety Steward | No Rust safety claim is involved. Reusing a presence result as process authority would conceal a time-of-check/time-of-use and identity invariant. | `pass-no-implementation` |
| Compiler Performance Engineer | This boundary has no measured build-latency outcome. Executable resolution overhead is not the product question. | `pass-no-performance-claim` |
| Interop Boundary Auditor | Windows `PATHEXT`, Unix execute bits, launchers, and scripts do not share one lossless leaf-to-process mapping. V1 must not collapse those semantics. | `veto-v1-reinterpretation` |
| AI Assurance Skeptic | A `satisfied` observation proves neither candidate identity nor later launch identity. The evidence supports the no-go only. | `pass` |
| Ecosystem Strategist | Ferris should preserve Cargo and environment-manager ownership. A future binding would authorize an exact tool, not duplicate installation or toolchain selection. | `pass-with-boundary` |
| Rust Maintainer | Implicit staging or mutable search-path execution would make ordinary Cargo usage harder to explain and audit. The current limitation remains explicit and removable. | `pass` |
| Native Platform Adopter | The gap is operationally real, but cross-platform launcher, path-disclosure, drift, and rollback behavior need a dedicated contract and adopter evidence. | `defer-new-version` |
| Scope Keeper | The pulse inspected one readiness kind and one execution field, changed no schema or runtime behavior, and preserved both V1 contracts. | `pass` |
| Validation Checker | The conclusion follows directly from implementation and contract invariants. No runtime claim or synthetic execution is added. | `pass-documentary` |
| Autonomy Supervisor | Investigation consumed the authorized continuation. A platform-local executable binding is a new architecture pulse and needs explicit approval. | `stop` |

## Decision

Do not connect environment-readiness V1 to Action Plan V1 executable identity.
Retain repository-local content binding and the explicit PATH-resolution
limitation.

The next product priority is one adopter-maintained declaration pair. If that
evidence shows executable staging blocks adoption, propose one separately
versioned platform-local executable-binding contract with explicit drift,
privacy, launcher, and rollback semantics.

## Control Closeout

- Product outcome: determine whether existing readiness evidence can safely
  remove PATH executable staging.
- Work completed: compared the V1 declaration, observation, identity, and
  launch invariants.
- Value obtained: prevented a presence check from becoming implicit execution
  authority and identified the exact architecture decision still required.
- Remaining risk: ordinary PATH-resolved tools remain outside Action Plan V1.
- Pulses or retries consumed: one architecture evaluation, zero implementation
  attempts.
- Proposed next action: gather adopter-maintained declaration evidence.
- Product Value Governor: `stop-value-exhausted`.
