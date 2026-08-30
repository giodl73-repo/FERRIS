# Ferris Go PARLOR Shadow Role Review

Status: local and GitHub-hosted proof accepted with adoption conditions

## Bounded role findings

| Repository role | Finding | Disposition |
| --- | --- | --- |
| `power-user` | One Ferris invocation ran all documented gates, and direct commands still worked after complete Ferris removal. Manual plan construction is too much ceremony for routine use. | `pass-with-condition` |
| `platform-eng` | Exact Windows runs and PARLOR PR #4's GitHub-hosted Ubuntu shadow pass with uploaded receipt evidence. | `pass` |
| `ciso` | File, command, environment, approval, and receipt identities fail closed. The local approval and receipt are integrity records, not authenticated attestations. | `pass-with-condition` |
| `founder` | The proof is opt-in and removable; no PARLOR workflow or owner command changed. | `pass` |
| Product value | The proof establishes functionality, not prevented iterations or time savings. | `pass` |

## Decision

Accept this result as the first external local and GitHub-hosted `ferris go`
proof. Do not generalize one PARLOR workflow into fleet equivalence or promote
the savings hypothesis.

PARLOR now owns a reviewed adapter that creates immutable plans, stages the
resolved Cargo toolchain executable, derives PR changes, and preserves direct
owner validation. The workflow remains non-required pending explicit maintainer
adoption.

## Complete scenario-matrix review

The expanded PR #4 through #8 evidence is reviewed in
`docs/research/2026-08-30-ferris-go-parlor-validation-matrix.md`.

| Ferris role | Finding | Disposition |
| --- | --- | --- |
| `rust-maintainer` | Ordinary Cargo remains usable; selection and failure diagnostics name packages, lanes, and corrective boundaries. The adapter is removable but still adds setup and maintenance cost. | `pass-with-condition` |
| `native-platform-adopter` | Windows and GitHub-hosted Linux pass. Two real environment mismatches failed visibly and were corrected without weakening detection. macOS, credentials, and checked-in adapter rollback remain unproved. | `pass-with-condition` |
| `ai-assurance-skeptic` | Behavioral failure, approval expiry, source drift, and receipt tampering remain non-success. The evidence establishes integrity and observed execution, not authenticity, correctness, or savings. | `pass` |
| `validation-checker` | Each admitted claim names a run, receipt, command, or exact negative diagnostic. Unsupported scenarios remain explicitly partial, not validated, or insufficient evidence. | `pass` |
| `scope-keeper` | PARLOR exercises an owner adapter and does not move consumer logic into Ferris. Artifact, scheduling, external-evidence, and publication lanes remain outside this proof. | `pass` |
| `product-value-governor` | Additional feature-only PARLOR scenarios no longer change a product decision. Continue only for a named matrix gap; otherwise move to adopter-owner evidence. | `stop-value-exhausted` |

The role decision is **accepted with conditions** for PARLOR's
single-repository envelope. It is not acceptance of GO-VAL-002, GO-VAL-004,
GO-VAL-005, GO-VAL-006, or GO-VAL-007, and it does not promote either savings
target.
