# FSIM-006: Layered Semantic and Rust Projection Change

Wave: W02
Revision: 1
State: Retraced
Claim state: simulated

## Question

How does Ferris decide application eligibility when a Typebook semantic
contract changes but an existing Rust projection still compiles?

## Locked fixture

- Typebook operation `payments.authorize` version 2 adds required
  `correlation_id` semantics and a `RateLimited` error outcome.
- Rust projection `payments-rust` version 1 retains the old input and error
  shape.
- An adapter synthesizes an empty correlation ID and maps `RateLimited` to
  `Internal`.
- Generated Rust code compiles against the consumer.
- The application requires complete error and audit semantics.

Negative control: version 2 adds optional documentation metadata that no
required projection exposes at runtime.

## Governing specifications

- CONTRACT-001 layer separation, operation semantics, projections, and
  directional compatibility;
- APPLICATION-001 resolved Application Contract; and
- RESOLUTION-001 candidate eligibility.

## Initial hand trace

Each layer had a valid individual compatibility result, but the specs did not
define how mandatory layer results determined application eligibility.

Initial issue: FSIM-SI-005.

## Retraced expected behavior

| Layer | Predicted result |
|---|---|
| Typebook semantic | Breaking for the existing consumer because required input and error semantics changed |
| Rust source | Compiles, but compilation is not semantic compatibility |
| Projection | Lossy and breaking for required audit and error capability |
| Adapter | Ineligible because it synthesizes required data and collapses a required error |
| Application Contract | Blocked with per-layer results and named migration to projection v2 |
| Resolution | Reject current candidate; alternatives are upgrade projection, retain v1 semantic contract, or defer |

The optional-documentation control remains eligible when the omitted projection
is explicitly optional and no required capability changes.

## Assertions

- [x] Compilation cannot override semantic breakage.
- [x] Mandatory layer results remain independently queryable.
- [x] One breaking mandatory boundary blocks application eligibility.
- [x] Optional loss requires explicit optional classification and consequence.
- [x] No single Boolean or numeric compatibility score is introduced.

## Specification changes

- FSIM-SCR-004.

## Claim boundary

The compile result is declared by the fixture for simulation. No generated code
or Typebook tool ran.
