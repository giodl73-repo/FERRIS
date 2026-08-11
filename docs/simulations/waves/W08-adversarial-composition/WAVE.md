# W08: Adversarial Composition

Status: Complete after retrace
Claim state: simulated

## Goal

Compose tenant collisions, revocation races, unsupported platforms, concurrent
owner mutations, partial effects, and exact approval boundaries.

## Locked specification baseline

Baseline commit: `7e8e057`

The retrace includes FSIM-SCR-016 and FSIM-SCR-017 plus one Ferris Wheel
regression turn over all affected prior scenarios.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-029](FSIM-029-cross-tenant-identity-collision.md) | Two tenants expose identical package and root-local identifiers | Isolation before aggregation | Pass without spec change |
| [FSIM-030](FSIM-030-mid-action-revocation.md) | Credential and approval are revoked during a non-interruptible operation | Revocation observation before later effects | Pass after FSIM-SCR-016 |
| [FSIM-031](FSIM-031-unsupported-platform.md) | Supported source targets an unsupported native platform | Unsupported versus degraded success | Pass without spec change |
| [FSIM-032](FSIM-032-post-preflight-owner-race.md) | Deployment generation changes after preflight and before mutation | Atomic owner-state guard | Pass after FSIM-SCR-017 |

## Wave issues

- FSIM-SI-017: emergency revocation lacked a required observation barrier
  before subsequent side effects; and
- FSIM-SI-018: preflight lacked an immediate atomic owner-state guard against
  check-to-write races.

## Ferris Wheel retrace

- FSIM-SCR-016 retraced cancellation, capability drift, connector removal,
  and running-action scenarios.
- FSIM-SCR-017 retraced ref generation, writable isolation, MCP schema drift,
  and external owner mutation scenarios.
- No prior intended behavior changed except to fail earlier before an
  unauthorized or stale side effect.

## Role review

- Rust Safety Steward: accepted after revocation and owner-state drift block
  later mutation rather than becoming warnings.
- Compiler Performance Engineer: accepted because immediate guards use owner
  conditional semantics rather than broad global serialization.
- Interop Boundary Auditor: accepted after tenant, connector, credential,
  deployment, and native-platform identities remain separate.
- AI Assurance Skeptic: accepted because no model or cached plan can override
  revocation, tenant isolation, or atomic owner state.
- Ecosystem Strategist: accepted because unsupported platforms remain honest
  and owner-native conditional APIs retain authority.
- Rust Maintainer: accepted because ordinary source support is not confused
  with unavailable native deployment support.
- Native Platform Adopter: accepted for Draft after unsupported native
  environments block without degraded-success language.
- Scope Keeper: accepted as a bounded composition wave without execution.
- Validation Checker: accepted after tenant, revocation, unsupported, race,
  cancellation, and partial-effect controls completed a Wheel turn.

## Disposition

Close W08 with no open P0 or P1 issues. Continue to W09 operational
continuity.
