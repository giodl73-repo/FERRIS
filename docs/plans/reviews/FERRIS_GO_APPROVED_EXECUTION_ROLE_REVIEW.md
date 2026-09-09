# Ferris Go Approved Execution Role Review

Status: implementation authorized within GO-WP-003

## Pulse control

- Outcome: execute one immutable, approved local Action Plan and emit a
  verifiable receipt.
- Effort limit: one core execution module, CLI wiring, contract, fixtures, and
  focused validation; no scheduler, provider, publication, or consumer work.
- Completion test: exact argv/cwd/environment execution, identity drift
  rejection, full-tree timeout cleanup, one terminal state per selected lane,
  secret redaction, and deterministic receipt verification.
- Abandonment condition: stop if correctness requires a generalized policy
  engine, sandbox, remote executor, or consumer-specific workflow parser.

## Dispositions

| Role | Disposition | Binding condition |
| --- | --- | --- |
| Product Value Governor | `continue-within-budget` | This is the shortest proof that Ferris can turn an approved plan into saved PR iteration time; stop at the local executor. |
| Rust Safety Steward | `pass-with-conditions` | Unsafe platform calls are isolated to process ownership, use checked handles/return codes, and receive a real child-process survival test. |
| Rust Maintainer | `pass` | Keep execution separate from planning, use strict typed plan, approval, and receipt files, and avoid framework or dependency churn beyond native process APIs. |
| Native Platform Adopter | `pass-with-conditions` | Preserve owner argv and diagnostics, support Windows and Unix process trees, and report cleanup failure rather than claiming success. |
| AI Assurance Skeptic | `pass-with-conditions` | Identity and terminal-state claims require negative fixtures; receipt verification is integrity checking, not authenticity. |
| Scope Keeper | `pass` | No selection, parallel scheduling, external gates, secret injection, artifacts, publication, deployment, or consumer changes. |
| Autonomy Supervisor | `pass-with-conditions` | Execute only an exact non-expired, non-revoked approval; any drift blocks before process launch. |
| Validation Checker | `pass-with-conditions` | Run focused execution tests plus existing path/topology regressions, PITFALL, VTRACE, diff checks, and final Codex review. |

## Resolved design decisions

1. `--action-plan` accepts an ID, not a path. The fixed local file layout is an
   implementation detail of this prototype.
2. Approval is a separate strict file. GO-WP-003 verifies integrity,
   binding, expiry, and revocation but does not claim signer authentication.
3. Owner entrypoints are separate declarations. Plans bind their declaration
   and exact entrypoint identities; topology plans remain non-executable.
4. Environment values are inherited only by approved name and never enter a
   plan or receipt. Credential-bearing names and credential classes are
   unsupported in this slice.
5. Native process ownership terminates the full tree. Cleanup uncertainty is a
   terminal failure, never a success-shaped fallback.
6. Receipt identity excludes elapsed time but includes lineage, terminal
   states, exit codes, output digests, and cleanup outcomes.

## Deferred authority

GO-WP-004 and every later work package remain unauthorized.
