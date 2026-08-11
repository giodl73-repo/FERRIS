# EXECUTION-001: Ferris Approved Execution Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: RESOLUTION-001, PLANNING-001, TRUST-001, and GOVERNANCE-001

## Purpose

This specification defines action requests, exact executable projections,
approval binding, preflight, isolation, command execution, deviations,
cancellation, rollback, cleanup, audit, and observed outcomes.

The specification does not authorize an implementation or any action. It
defines what a separately approved future execution capability must satisfy.

## Action request

Every request MUST identify:

- Resolution Record and selected Blueprint Plan;
- requesting principal and delegated subject;
- requested semantic command;
- exact scope and desired outcome;
- mutation, network, credential, publication, deployment, and external-system
  requirements;
- resource envelope;
- required validation;
- risk and data classification;
- rollback and cleanup;
- time window; and
- prohibited actions.

## Action Plan

An Action Plan is the exact executable projection of one selected Blueprint
Plan. It MUST contain:

- action ID, schema, version, and parent;
- immutable plan and resolution identities;
- exact commands, working directories, environment allowlist, tools, and
  versions;
- ordered dependencies and concurrency;
- filesystem and external-system mutation boundaries;
- network endpoints and connector identities;
- ephemeral credential classes and audiences;
- isolation and tenant boundary;
- resource limits;
- preconditions and observation checks;
- required validation;
- stop, cancellation, timeout, retry, and idempotency rules;
- rollback and cleanup;
- expected outputs and evidence; and
- expiry.

The Action Plan MUST NOT silently add work absent from the Blueprint Plan
except through a declared fallback or a new resolution and approval.

## Approval binding

Execution requires a GOVERNANCE-001 Approval Record bound to:

- exact Action Plan;
- exact Blueprint Plan and Resolution Record;
- principal and delegated subject;
- commands and scope;
- tools, connectors, endpoints, and credential classes;
- resource and time limits;
- validation, rollback, cleanup, and stop conditions;
- policy and exception identities; and
- approval expiry and revocation state.

Any material change invalidates approval.

## Preflight

Before execution, Ferris MUST verify:

- identities and schema versions;
- current policy and authorization;
- approval validity;
- source, lock, contract, profile, toolchain, platform, and environment state;
- TRUST-001 decisions;
- isolation and writable paths;
- credentials available through approved non-durable channels;
- resources and foreground reserve;
- rollback feasibility;
- required owner tools; and
- absence of conflicting running actions.

If planning identified owner freshness insufficiency, preflight MUST verify the
exact approved remediation. A wider package selection without owner
invalidation, repaired declaration, or isolated empty state MUST fail
preflight.

Failed, stale, missing, revoked, or unknown preconditions MUST block or request
a new plan. They MUST NOT become warnings on a mutating action.

## Execution behavior

Ferris MUST:

- execute only listed commands or semantic owner operations;
- preserve owner-local exit and error semantics;
- record start, progress, output references, exit, and duration;
- enforce resource, network, filesystem, tenant, and credential bounds;
- stop on declared conditions;
- propagate cancellation;
- isolate partial outputs;
- avoid exposing secrets in arguments, output, logs, or evidence; and
- retain deviations as observed evidence.

No broad catch, empty-success fallback, or automatic retry may conceal owner
failure. Retries require classified transient behavior, idempotency, bounds,
and retained attempts.

## Observation barriers and deviations

At every PLANNING-001 barrier, execution MUST compare observed state with plan
conditions.

A material deviation MUST produce one of:

- safe declared fallback;
- stop and preserve evidence;
- rollback;
- request replan and new resolution;
- request renewed approval; or
- escalate to owner input.

Execution MUST NOT rewrite the original plan or approval to match observed
behavior.

## Cancellation protocol

Every cancellation attempt MUST record:

- cancellation request identity;
- Action Plan and Execution Record identities;
- requester, authority, reason, scope, and time;
- requested deadline and stop boundary;
- owner operations receiving the request;
- interruptibility and next safe point;
- acknowledgement and propagation;
- work completed before effective stop;
- irreversible and externally visible effects;
- remaining work;
- required rollback, compensation, and cleanup; and
- diagnostics and final cancellation state.

Cancellation state MUST distinguish requested, denied, acknowledged,
propagating, owner-deferred, completed-before-stop, cancelled, failed, and
unknown.

A cancellation request or acknowledgement MUST NOT be reported as effective
cancellation. If an owner operation cannot stop immediately, Ferris MUST stop
launching dependent work, preserve the owner state, and follow the approved
safe-point, rollback, compensation, cleanup, or escalation rule. Effects that
complete before the stop becomes effective remain observed effects.

## Rollback and cleanup

Rollback MUST identify:

- reversible and irreversible effects;
- checkpoint or prior state;
- exact commands or owner operations;
- data, schema, artifact, deployment, connector, and credential consequences;
- validation of restored state;
- owner and timeout; and
- fallback when rollback fails.

Cleanup MUST cover temporary files, processes, locks, worktrees, credentials,
network sessions, staged artifacts, deployment state, and retained logs.

Rollback or cleanup failure MUST be explicit and escalated. It MUST NOT be
reported as successful execution.

## External actions

External posting, issue creation, pull requests, deployment, publication,
promotion, or destructive owner-system mutation requires an explicit action
kind, connector, authorization, approval, validation, rollback or compensating
action, and audit.

A prepared evidence packet or connector capability does not authorize posting.

## Execution record

Every Execution Record MUST retain:

- Action Plan and approval identities;
- actual principal, tools, commands, connectors, credentials classes, and
  environment;
- start, progress, completion, and cancellation;
- outputs and side effects;
- deviations and retries;
- owner failures;
- validation;
- rollback and cleanup;
- resource use;
- resulting Forest root; and
- final state.

The terminal outcome MUST preserve independent dimensions:

- execution: not started, succeeded, succeeded with conditions, failed,
  cancelled, timed out, blocked, partially completed, stale, or unknown;
- rollback: not required, not attempted, running, succeeded, failed, partial,
  or unknown;
- cleanup: not required, not attempted, running, succeeded, failed, partial,
  or unknown;
- residual effects: none, retained-safe, externally visible, irreversible,
  recovery required, or unknown; and
- recovery owner, deadline, evidence, and escalation.

An overall result MUST NOT be succeeded when required rollback or cleanup
failed, is partial, remains running, or has unknown residual effects. Human
and machine views MAY provide one summary class only when all independent
dimensions remain available without loss.

## Acceptance criteria

EXECUTION-001 may advance to Proposed only when:

1. exact Action Plan and approval schemas are fixed;
2. changed plan, policy, principal, command, scope, environment, credential,
   and expiry invalidate execution;
3. filesystem, network, credential, tenant, resource, and connector isolation
   are executable;
4. timeout, cancellation, retry, partial failure, deviation, stale state, and
   owner failure remain visible;
5. observation barriers require fallback, stop, replan, or renewed approval;
6. rollback and cleanup succeed and fail in controlled fixtures;
7. external posting cannot occur from a packet, connector, or MCP request
   without explicit approval; and
8. all nine roles record a disposition.
