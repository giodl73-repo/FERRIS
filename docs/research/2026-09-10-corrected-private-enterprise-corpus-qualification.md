# Corrected Private Enterprise-Shape Corpus Qualification

Date: 2026-09-10
Status: Incomplete; stopped during Windows federated execution
Evidence ID: FERRIS-EVIDENCE-003

## Question

Can the corrected five-family private enterprise-shape corpus serve as the
primary current-Ferris regression and demonstration suite across Windows and
hosted Ubuntu?

The bounded authority is
[`Pulse 01`](../../context/waves/2026-09-10-corrected-enterprise-corpus-qualification/pulses/pulse-01.md).
Private repository identities, revisions, paths, raw output, hashes, run
identities, and identifiable timings remain outside this public repository.

## Windows result

All five repository-owned preflight suites passed at their frozen corrected
cutoffs.

| Evidence group | Complete scenarios | Matched pass | Matched failure | Conservative fallback |
|---|---:|---:|---:|---:|
| Three small-workspace families | 11 | 11 | 0 | 5 |
| High-cardinality family | 6 | 5 | 1 | 1 |
| Federated family | 0 durable | 0 | 0 | 0 |
| **Retained total** | **17** | **16** | **1** | **6** |

All 17 retained records had stable plan and selection identities across two
executions. The matched-failure record was an existing intentional owner test
failure and remained non-success in both selected and full lanes. No retained
record contains a selected-pass/full-fail divergence.

## Federated stop

The existing federated runner expands its maximum-input case into 256 absolute
changed-path arguments. From the long private custody root, Windows rejected
the process start because the command line was too long. The Ferris process for
that case did not launch.

The runner retains results in memory and writes its structured evidence only
after the full matrix. Consequently, earlier federated activity from this
attempt is not durable evidence and is classified `not observed`.

This is an owner-runner portability defect. It is not evidence that Ferris
accepted, rejected, or incorrectly planned the maximum-input case.

## Decision

The corpus is not yet qualified as the primary two-platform suite:

- the small and high-cardinality Windows evidence is accepted within its
  synthetic boundary;
- the federated Windows matrix is incomplete;
- hosted Ubuntu was not attempted; and
- no private hosted-validation branch was created.

No retry, runner correction, Ferris product fix, support statement,
production-representativeness claim, affected-only gating, or savings claim is
authorized by this result.

## Next gate

A future owner-side correction should preserve all 256 inputs while passing
relative child-process paths from the application root, matching the existing
public Ferris maximum-input regression pattern. That correction and any fresh
qualification require separate explicit approval.
