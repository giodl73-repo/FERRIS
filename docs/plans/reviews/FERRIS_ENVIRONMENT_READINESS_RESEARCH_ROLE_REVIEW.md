# FERRIS Environment Readiness Research Role Review

Date: 2026-09-16
Status: Complete for research direction

## Role dispositions

| Role | Disposition | Finding |
|---|---|---|
| Rust Safety Steward | `pass-no-safety-claim` | Presence or version evidence cannot prove behavioral correctness or Rust safety. |
| Compiler Performance Engineer | `pass-no-performance-claim` | Earlier failed cohorts motivate readiness but establish no iteration-time gain. |
| Interop Boundary Auditor | `pass-with-owner-provenance` | Native-tool and format semantics remain with their owners; adapters must expose unsupported constructs. |
| AI Assurance Skeptic | `pass-explicit-only` | Requirements may not be inferred from logs or model interpretation and missing evidence cannot become success. |
| Ecosystem Strategist | `continue-with-adapters` | Cargo, rustup, asdf, mise, Development Containers, and Devfile are complementary sources; Ferris should normalize rather than replace them. |
| Rust Maintainer | `pass-actionable-removable` | One doctor report can replace repeated trial-and-error if declarations remain optional, local, and removable. |
| Native Platform Adopter | `pass-cross-machine-value` | Typed requirement ownership and observed state directly address environment drift across developer and CI machines. |
| Scope Keeper | `pass-research-only` | The first proposed slice is explicit local input and passive checks; installation, scripts, services, and inference remain closed. |
| Validation Checker | `pass-with-fixture-gates` | A later contract needs positive, missing, mismatch, unsupported, conflict, secret, and cross-platform fixtures. |
| Product Value Governor | `continue-within-budget` | The capability addresses repeated adopter failures and naturally extends `doctor`; one contract pulse is the shortest next step. |
| Autonomy Supervisor | `stop-after-research` | This review authorizes documentation only; contract or implementation work requires a separately approved pulse. |

## Completed revision

- Ferris research baseline:
  `da40599b9c1c0302cc660d508e23979756c875ad`.

## Required next evidence

A contract-design pulse must freeze:

- requirement and observation schemas;
- owner authority and required-versus-advisory policy;
- source provenance and conflict behavior;
- status and process-exit semantics;
- secret-value exclusion;
- cross-platform path and executable rules;
- positive and negative fixtures; and
- adoption, removal, rollback, and compatibility behavior.

## Remaining gates

Pulse 02 contract authority, frozen schemas and fixtures, status and exit
semantics, cross-platform review, and a later separate implementation pulse all
remain required.

## Final authority

The research direction is accepted. No specification status change, product
code, format adapter, executable probe, environment mutation, or adopter change
is authorized.
