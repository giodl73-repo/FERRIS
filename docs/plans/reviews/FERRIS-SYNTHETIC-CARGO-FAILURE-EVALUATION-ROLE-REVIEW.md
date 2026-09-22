# Ferris Synthetic Cargo Failure Evaluation Role Review

Status: Accepted for Pulse 01 closeout

## Review

| Role | Disposition | Basis |
|---|---|---|
| Rust Safety Steward | not-applicable | No Ferris product code or unsafe boundary changed. |
| Compiler Performance Engineer | not-applicable | The evaluation measured classification behavior, not build performance. |
| Interop Boundary Auditor | pass | Real Cargo stderr crossed only the existing bounded UTF-8 input boundary. |
| AI Assurance Skeptic | pass | The report preserves unknown cause, retains no stderr, and makes no repair or correctness claim. |
| Ecosystem Strategist | pass | Cargo generated the failure and remained authoritative; Ferris only classified captured evidence. |
| Product Value Governor | pass | Five existing topologies provided useful evidence without a speculative feature. Disposition: `stop-complete`. |
| Scope Keeper | pass | The pulse made no classifier, schema, synthetic-repository, build, or network change. |
| Validation Checker | pass-with-correction | Five corrected controls passed. The invalid workspace-member control and temporary Ferris manifest side effect are disclosed and fully cleaned. |
| Autonomy Supervisor | pass | One bounded corrective construction was used after the original control failed to represent the intended condition. No successor was opened. |
| Rust Maintainer | pass | The stable dependency code held across small, federated, and 96-package workspace shapes without exposing owner details. |
| Native Platform Adopter | pass-with-risk | Windows evidence passed; no Linux or macOS claim follows. |

## Claim boundary

The result covers generated missing path dependencies in five synthetic shapes
on one Windows toolchain. It is not owner adoption, broad diagnostic recall,
cross-platform proof, or evidence that Ferris can repair the failure.
