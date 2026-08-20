---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:operational-readiness-release-gates
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Operational Readiness and Release Gates
status: source-custody
source_custody: partial
current_path: rust-production-engineering/15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md
canonical_path: rust-production-engineering/15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md
backsource_ids: [proof-backfill:rust-production-engineering:15-operational-readiness-release-gates]
concepts: [operational readiness, release gates, production readiness review, evidence, rollout, rollback, risk acceptance]
root_concepts: [operational readiness]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Operational Readiness and Release Gates

## The Big Picture

Operational readiness is an evidence-backed decision that a system can be
introduced, changed, observed, contained, and recovered within accepted risk.
It is not a launch meeting, a checklist completed from memory, or a substitute
for accountable judgment.

```
+============================================================================+
|                         READINESS GATE MODEL                               |
|                                                                            |
| design claims --> implementation --> executable proof --> owner decision   |
|      |                 |                  |                    |           |
|      v                 v                  v                    v           |
| invariants         artifact/config    tests/drills/SLOs      accept/hold   |
| failure model      platform contract  security/provenance    limit scope   |
| recovery model     migrations         runbooks/alerts        record risk   |
|                                                                            |
| decision applies to one artifact, environment, exposure, and time window   |
+============================================================================+
```

Readiness decays. A new dependency, data class, runtime, platform, traffic
profile, or irreversible migration can invalidate previous evidence.

## Gate Categories

| Category | Minimum question |
|---|---|
| Product/service | What user work is protected and what degradation is acceptable? |
| Architecture | Where are state, trust, capacity, and failure boundaries? |
| Configuration/identity | Can startup validate inputs and rotate credentials safely? |
| Reliability | Are deadlines, admission, retries, and failure isolation bounded? |
| Observability | Can impact, saturation, release, and causality be identified? |
| Data/recovery | Are invariants, migration, backup, restore, and reconciliation proven? |
| Delivery/supply chain | Is the artifact immutable, traceable, verified, and promotable? |
| Platform/lifecycle | Do start, health, drain, resource, and termination contracts align? |
| Operations | Are SLO, alerts, runbooks, ownership, access, and escalation ready? |
| Security/privacy | Are threats, dependencies, secrets, data handling, and incident paths reviewed? |

Not every workload needs the same implementation. A batch tool, embedded
device, desktop agent, and multi-region service require different evidence, but
none should leave its relevant failure model implicit.

## Mandatory Evidence Set

```
artifact:
  digest, revision, toolchain, lockfile, SBOM/provenance, symbols

behavior:
  config validation, capacity limits, timeout/retry policy, shutdown

state:
  schema compatibility, migration, duplicate/uncertain outcome, restore

operations:
  SLI/SLO, dashboards, alerts, runbooks, owner, escalation, access

release:
  cohort plan, hold points, abort signals, rollback/roll-forward
```

Each item should link to durable evidence. "Team tested it" is not a test
result; "we can roll back" is not a rollback procedure.

## Executable Pre-Release Baseline

Scope: a GNU/Linux CI runner for a Rust application with default features. This
is a mechanical baseline, not the whole readiness review.

```bash
set -eu

test -z "$(git status --porcelain)"
test -n "${RUNBOOK_URL:-}"
test -n "${SERVICE_OWNER:-}"
test -n "${RELEASE_REVISION:-}"

cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo build --workspace --release --locked

artifact="target/release/orders"
test -x "$artifact"
sha256sum "$artifact"
printf 'owner=%s\nrunbook=%s\nrevision=%s\n' \
  "$SERVICE_OWNER" "$RUNBOOK_URL" "$RELEASE_REVISION"
```

The pipeline must adapt the artifact path for Windows and multi-binary
workspaces. Operational gates should additionally query stored drill results,
attestations, vulnerability policy, migration approval, and rollout controls
rather than relying only on environment variables. Here `--all-targets` covers
Cargo target kinds on the CI host; supported target triples and feature
combinations require their own explicit matrix.

## Evidence Quality

| Weak claim | Strong evidence |
|---|---|
| "Graceful shutdown implemented" | test sends platform stop, verifies readiness drop, drain, and deadline |
| "Retries are safe" | idempotency/transaction proof plus bounded-attempt test |
| "Backups enabled" | dated restore drill with achieved RPO/RTO |
| "Dashboards exist" | reader tasks answered from current panels |
| "Rollback supported" | compatibility matrix and exercised rollback or roll-forward |
| "On-call ready" | paging test, access check, runbook game day |
| "Secure supply chain" | verified provenance/signature and dependency policy result |

Evidence should name environment, artifact, command or procedure, observed
result, and date. A screenshot without query, scope, or artifact identity is
weak evidence.

## Risk-Based Gate Classes

| Change | Additional gates |
|---|---|
| Stateless logic | canary signals and rollback |
| New dependency | timeout/capacity/failure/ownership contract |
| Schema change | expand/contract, backfill, restore, rollback window |
| New async runtime or major runtime update | scheduler/blocking/load/shutdown evidence |
| New platform | lifecycle, identity, networking, resource, diagnostic parity |
| New sensitive data | threat/privacy/retention/access review |
| Irreversible operation | dual control, dry run, reconciliation, roll-forward |

Gate rigor should follow blast radius and reversibility, not team prestige or
deadline pressure.

## Progressive Release Gates

```
pre-deploy --> deploy no exposure --> smoke --> small cohort --> broad cohort
     |               |                 |           |              |
     +-- artifact ---+-- platform -----+-- SLI ----+-- capacity ---+
                                                        |
                                              full exposure / hold
```

At each hold point define:

- minimum observation duration and sample;
- SLI/error-budget comparison;
- saturation and dependency thresholds;
- correctness/data reconciliation check;
- who may continue, halt, rollback, or roll forward.

Low-volume systems may require synthetic transactions, longer observation, or
explicit human verification rather than automated percentage comparisons.

## Exceptions and Risk Acceptance

An exception records the missing evidence, reason, affected scope, compensating
controls, named risk owner, expiration, and removal plan. It must not silently
turn a mandatory gate into an optional convention.

```
gate missing --> hold
                   |
                   +--> remediate and rerun
                   |
                   +--> bounded exception with owner + expiry + controls
```

Expired exceptions fail closed in the delivery policy when practical.

## Library, Runtime, and Platform Choices

| Layer | Readiness responsibility |
|---|---|
| Library | behavior, API/error semantics, dependency and unsafe boundaries |
| Runtime | scheduling, blocking, timer, cancellation, shutdown evidence |
| Platform | identity, resources, health, rollout, dump/log access, recovery |

Readiness must identify runtime and platform assumptions explicitly. A Tokio
load test does not certify another executor; a Kubernetes probe test does not
certify Windows SCM behavior.

## Old World -> New World Bridge

The universal bridge is from **launch checklist** to **continuously evaluated
assurance case**: claims are tied to evidence and invalidated when assumptions
change.

Traditional production readiness reviews, ship-room criteria, and Azure DevOps
environment approvals are direct prior art. Policy automation improves
consistency, but a human risk owner remains necessary for ambiguous or
irreversible decisions.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Mandatory automated gate | evidence is objective, repeatable, and machine-verifiable |
| Human review | tradeoff, data risk, or irreversible transition needs judgment |
| Canary hold point | live behavior must be compared before broad exposure |
| Rollback gate | prior artifact remains state/protocol compatible |
| Roll-forward gate | state transition cannot safely reverse |
| Time-bounded exception | risk is consciously accepted with compensating controls |
| Re-review | runtime, platform, dependency, data, scale, or ownership changed materially |

## Common Confusion Points

- **A checklist item is not evidence.**
- **Readiness is scoped to an artifact and exposure, not awarded forever.**
- **Passing functional tests does not prove overload or recovery behavior.**
- **Rollback is not available after every data transition.**
- **Manual approval without evidence is ceremony.**
- **Automated policy cannot own business risk.**
- **A platform certification does not transfer across lifecycle semantics.**

## Primary Sources

- Google SRE launch coordination: https://sre.google/sre-book/reliable-product-launches/
- NIST SSDF: https://csrc.nist.gov/Projects/ssdf
- SLSA specification: https://slsa.dev/spec/
- OWASP ASVS: https://owasp.org/www-project-application-security-verification-standard/
- OpenSSF Scorecard: https://scorecard.dev/
- CNCF TAG App Delivery operator readiness guidance: https://tag-app-delivery.cncf.io/

## Related Guides

- Previous: [14-SLOS-RUNBOOKS-OWNERSHIP-AND-COST.md](14-SLOS-RUNBOOKS-OWNERSHIP-AND-COST.md)
- Module map: [00-OVERVIEW.md](00-OVERVIEW.md)
- Delivery evidence: [11-CI-CD-AND-PROMOTION.md](11-CI-CD-AND-PROMOTION.md)
- Recovery evidence: [12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md](12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md)
- Module status: [STATUS.md](STATUS.md)
