---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:testing-staging-fault-injection-recovery
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Testing, Staging, Fault Injection, and Recovery
status: source-custody
source_custody: partial
current_path: rust-production-engineering/12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md
canonical_path: rust-production-engineering/12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md
backsource_ids: [mdloom-backfill:rust-production-engineering:12-testing-staging-fault-injection-recovery]
concepts: [testing, staging, fault injection, recovery, property testing, load testing, restore drills]
root_concepts: [verification]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Testing, Staging, Fault Injection, and Recovery

## The Big Picture

Production confidence comes from testing claims at the layer where they can
fail. Unit tests prove local logic; integration tests prove concrete adapters;
staging tests deployment shape; fault injection proves containment; recovery
drills prove that durable evidence can restore service.

```
+============================================================================+
|                          EVIDENCE LADDER                                   |
|                                                                            |
|  types/static checks                                                       |
|        v                                                                   |
|  unit + property tests                                                     |
|        v                                                                   |
|  integration + protocol/contract tests                                     |
|        v                                                                   |
|  package/deployment tests                                                  |
|        v                                                                   |
|  load + saturation + fault injection                                       |
|        v                                                                   |
|  backup restore + failover + incident game day                             |
|                                                                            |
|  each rung proves a different claim; higher does not replace lower         |
+============================================================================+
```

Tests should name the invariant and failure boundary they prove. "End-to-end"
is not a synonym for complete.

## Test Portfolio

| Test class | Best at | Blind spot |
|---|---|---|
| Unit | deterministic state transitions | real adapters and timing |
| Property/model | broad invariant exploration | environment integration |
| Integration | driver, protocol, schema behavior | fleet-scale dynamics |
| Contract | producer/consumer compatibility | implementation capacity |
| Deployment smoke | package, config, identity, health | long-tail failures |
| Load/soak | saturation, leaks, tails | logical rare states unless modeled |
| Fault injection | containment and recovery paths | unknown unmodeled faults |
| Restore drill | RPO/RTO and backup usability | normal serving behavior |

## Executable Deterministic Failure Test

This example injects an interruption point between data write and commit marker,
then checks recovery policy. It is deliberately in-memory so `cargo test` is
deterministic and independent of an external data service; it does not simulate
process death or durable media.

```rust
#[derive(Default, Debug)]
struct Journal {
    pending: Option<u64>,
    committed: Option<u64>,
}

#[derive(Clone, Copy)]
enum FailPoint {
    Never,
    AfterPendingWrite,
}

fn update(journal: &mut Journal, value: u64, fail: FailPoint) -> Result<(), ()> {
    journal.pending = Some(value);
    if matches!(fail, FailPoint::AfterPendingWrite) {
        return Err(());
    }
    journal.committed = journal.pending.take();
    Ok(())
}

fn recover(journal: &mut Journal) {
    // Policy: an uncommitted pending value is discarded.
    journal.pending = None;
}

#[test]
fn recovery_discards_partial_update() {
    let mut journal = Journal::default();
    assert!(update(&mut journal, 42, FailPoint::AfterPendingWrite).is_err());
    recover(&mut journal);
    assert_eq!(journal.pending, None);
    assert_eq!(journal.committed, None);
}

#[test]
fn successful_update_survives_recovery() {
    let mut journal = Journal::default();
    update(&mut journal, 42, FailPoint::Never).unwrap();
    recover(&mut journal);
    assert_eq!(journal.committed, Some(42));
}
```

Put the code in `src/lib.rs` of a new Cargo library, run
`cargo generate-lockfile`, then `cargo test --locked`. The example proves one
recovery policy, not filesystem durability. A real store test must terminate
the process at controlled write, flush, rename, and commit boundaries and
reopen the durable state.

## Fault Model

```
process: panic | abort | deadlock | leak | CPU starvation
host:    kill | reboot | clock jump | disk full | descriptor exhaustion
network: delay | loss | reset | partition | duplicate | reorder
storage: timeout | stale read | uncertain commit | corruption | full volume
control: bad config | expired credential | failed rollout | revoked identity
```

Inject faults at interfaces rather than scattering random sleeps. A trait-based
adapter, proxy, test transport, database failpoint, or platform chaos tool can
make the fault precise and reproducible.

## Safe Fault Injection

| Control | Requirement |
|---|---|
| Scope | exact service, cohort, dependency, and time window |
| Abort condition | automatic stop on user-impact or safety signal |
| Observability | fault marker appears in traces/metrics/events |
| Ownership | named conductor and affected on-call owner |
| Recovery | tested reversal before injection |
| Data safety | no destructive production experiment without explicit approval |

Start in deterministic tests, then representative staging, then a small
production cohort only when the remaining unknown requires real conditions.

## What Staging Can and Cannot Prove

Staging is useful when it reproduces artifact, configuration schema, identity
flow, network policy, resource limits, and deployment controls. It rarely
reproduces production scale, data shape, dependency contention, or operator
behavior.

Avoid permanent staging drift. Create environments from the same deployment
definitions and verify differences explicitly. Synthetic or sanitized data
must preserve the distributions that drive query plans, memory, and latency.

## Load, Soak, and Recovery

| Exercise | Measure |
|---|---|
| Step load | knee where latency/queueing accelerates |
| Spike | admission and shedding behavior |
| Soak | leaks, fragmentation, cache growth, rotation |
| Dependency slowdown | deadline propagation and pool saturation |
| Instance loss | redistribution and capacity margin |
| Restore | recoverable point, elapsed time, validation |

Test beyond the expected steady state so overload behavior is known. A service
that meets latency at average load but collapses without shedding at 1.2x load
is not production-ready.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | built-in test harness, proptest/quickcheck, mock/test adapters, fuzzing |
| Runtime | deterministic time, task tests, scheduler-specific diagnostics |
| Platform | ephemeral environments, network proxies, chaos tools, snapshot/restore |

Tokio's paused time is useful for Tokio timer logic, but it does not test
another runtime or real kernel timing. Kubernetes chaos tools are optional
platform mechanisms; process termination and dependency proxies work elsewhere.

## Old World -> New World Bridge

The universal bridge is from **test cases** to **evidence about failure
semantics**. Production engineering asks not only "does it work?" but "what
state remains after interruption at every boundary?"

Visual Studio test infrastructure, Azure DevOps environments, and fault
injection services are familiar mechanisms. Rust's traits and enums make
deterministic failure adapters particularly direct, but they do not replace
real platform drills.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Unit test | local deterministic rule can fail |
| Property/model test | an invariant spans many input/state combinations |
| Integration test | concrete protocol, driver, or schema behavior matters |
| Staging smoke | package/config/identity/deployment shape needs proof |
| Load test | capacity and tail behavior are release risks |
| Fault injection | containment or recovery claim is otherwise untested |
| Soak test | time-dependent growth, leaks, or rotation matter |
| Restore drill | backups and RTO/RPO are operational promises |

## Common Confusion Points

- **Mocks can prove the mock.** Use them for policy, then test the real adapter.
- **Staging success does not prove production scale.**
- **Random chaos without a hypothesis creates noise, not evidence.**
- **A backup job success message does not prove restore.**
- **Code coverage measures execution, not assertion quality or fault coverage.**
- **Paused runtime time does not model all wall-clock and OS behavior.**

## Primary Sources

- Rust testing: https://doc.rust-lang.org/book/ch11-00-testing.html
- Rust Fuzz Book: https://rust-fuzz.github.io/book/
- proptest: https://docs.rs/proptest/
- Tokio testing: https://tokio.rs/tokio/topics/testing
- Google SRE testing reliability: https://sre.google/sre-book/testing-reliability/
- NIST contingency planning: https://csrc.nist.gov/pubs/sp/800/34/r1/final

## Related Guides

- Previous: [11-CI-CD-AND-PROMOTION.md](11-CI-CD-AND-PROMOTION.md)
- Next: [13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md](13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md)
- Persistence recovery: [08-PERSISTENCE-TRANSACTIONS-AND-DATA-ACCESS.md](08-PERSISTENCE-TRANSACTIONS-AND-DATA-ACCESS.md)
- Release evidence: [15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md](15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md)
