# FERRIS Private Federated Runner Portability Role Review

Date: 2026-09-10
Stage: Post-correction closeout
Status: Accepted within measured boundary; private pull request open

## Evidence reviewed

- Authority:
  [`Pulse 01`](../../../context/waves/2026-09-10-private-federated-runner-portability/pulses/pulse-01.md).
- Parent failure:
  [`Corrected private enterprise-shape corpus qualification`](../../research/2026-09-10-corrected-private-enterprise-corpus-qualification.md).
- Change: one private owner-runner file uses the application root as the Ferris
  child working directory and preserves relative in-application inputs.
- Windows owner integrity: formatting, Clippy, all locked/offline workspace
  tests, and generated-corpus verification passed.
- Windows federated matrix: nine durable scenarios, eight matched passes, one
  expected matched failure, three typed failures, and stable repeated
  identities.
- Boundary cases: the unchanged maximum path-only and mixed cases each retained
  all 256 explicit inputs.
- Hosted Ubuntu: the private pull request's existing owner workflow passed; it
  did not execute the federated runner.
- Repository delta: one owner-tooling file; no product, scenario, fixture,
  topology, manifest, command, expectation, or evidence-schema file.

## Role dispositions

| Role | Disposition | Finding |
|---|---|---|
| Rust Safety Steward | `pass` | No Rust or unsafe code changed; compiler acceptance is not used as a behavioral proof. |
| Compiler Performance Engineer | `pass-without-performance-claim` | The correction removes a process-launch limit but authorizes no latency, build-time, or savings claim. |
| Interop Boundary Auditor | `pass` | The child-process boundary now uses a bounded working directory and relative owner inputs while retaining the absolute outside-path negative control. |
| AI Assurance Skeptic | `pass-with-limitation` | Durable Windows results support the narrow fix; hosted runner behavior remains explicitly not observed. |
| Ecosystem Strategist | `pass` | The correction stays in owner tooling and reuses ordinary process and Cargo behavior rather than adding a Ferris protocol. |
| Rust Maintainer | `pass` | The one-file change is small, dependency-free, explainable, and removable. |
| Native Platform Adopter | `pass-with-limitation` | Windows runner operation and hosted Ubuntu owner integrity passed; hosted Ubuntu runner execution remains unmeasured. |
| Scope Keeper | `pass` | Only `EC-05` process arguments changed; no input, scenario, oracle, product, or qualification retry entered scope. |
| Validation Checker | `pass-with-limitation` | Complete Windows owner and runner evidence exists, including negative controls; hosted runner execution is not claimed. |
| Product Value Governor | `stop-value-exhausted` | The Windows launch blocker is corrected and reviewable; merge or another qualification requires a separate product decision. |
| Autonomy Supervisor | `stop` | The one correction and one private pull request are complete; no merge, retry, or successor is authorized. |

## Control checkpoint

| Control | Record |
|---|---|
| Product outcome | Restore the unchanged private federated matrix on Windows without reducing its maximum inputs. |
| Work completed | One owner-runner correction, complete Windows owner and federated validation, and hosted Ubuntu owner validation. |
| Value obtained | All nine federated scenarios now emit durable Windows evidence, including both 256-input boundaries. |
| Remaining risk | The corrected runner itself was not executed on hosted Ubuntu; Ferris has not been freshly qualified against the correction. |
| Pulses/retries consumed | One correction pulse; no corrective successor. |
| Proposed next action | Stop pending explicit merge or later qualification direction. |
| Product Value Governor | `stop-value-exhausted` |

## Final authority

The correction pulse is exhausted. This review grants no private merge, Ferris
product change, corpus scenario change, qualification rerun, affected-only
gating, production support, representativeness, performance, or savings claim.
