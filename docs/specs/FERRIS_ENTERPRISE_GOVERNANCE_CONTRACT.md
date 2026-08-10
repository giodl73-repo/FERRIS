# GOVERNANCE-001: Ferris Enterprise Governance Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: PRODUCT-001, APPLICATION-001, TRUST-001, and EXECUTION-001

## Purpose

This specification defines portable organization governance over Ferris
principals, plans, actions, connectors, data, resources, evidence, and
lifecycle.

## Principals

Ferris MUST distinguish:

- human user;
- maintainer or reviewer;
- workload or service identity;
- CI runner;
- AI model and agent instance;
- MCP client and server;
- connector identity; and
- external owner system.

Every authenticated action MUST retain issuer, subject, audience, tenant,
credential class, authentication time, authorization decision, and applicable
policy identity without retaining reusable credentials.

## Authorization

Authorization MUST evaluate:

- principal and delegated subject;
- organization, tenant, repository, application, workspace, environment, and
  resource scope;
- semantic command and requested action;
- data classification;
- network, secret, mutation, publication, and deployment requirements;
- required reviewers and separation of duties;
- resource and cost budget;
- policy version, expiry, exception, and revocation; and
- connector and owner-system permissions.

Authentication MUST NOT imply authorization. Connector authorization MUST NOT
imply Ferris action approval.

## Approval

An approval record MUST include:

- exact Blueprint Plan and Action Plan identity;
- approving principal and authority;
- approved commands, scopes, tools, resources, and time window;
- mandatory validation;
- isolation, budget, stop, rollback, and cleanup conditions;
- policy and exception identities; and
- revocation state.

Material plan changes MUST invalidate prior approval.

## Data governance

Every data field and artifact MUST have:

- classification;
- owner;
- permitted audiences and connectors;
- residency and transfer constraints;
- retention and deletion policy;
- redaction behavior;
- audit requirements; and
- incident and revocation handling.

Redaction MUST remain visible. It MUST NOT make incomplete evidence appear
complete.

Secrets, tokens, private keys, credential caches, and reusable authorization
codes MUST NOT be stored in Blueprint Plans, Query Forest roots, refs,
diagnostics, model prompts, or evidence packets.

## Policy and portability

Ferris policy MUST have a product-neutral canonical form. Entra ID, Azure
Policy, GitHub, Azure DevOps, Key Vault, and other systems MAY supply identity,
policy, secret, approval, or audit adapters.

No Microsoft-specific identifier may become the sole canonical application,
principal, policy, plan, artifact, or evidence identity.

## Tenant isolation

Tenant data, credentials, caches, plans, roots, logs, and connector sessions
MUST be isolated. Cross-tenant operation requires explicit policy and separate
evidence.

Emergency revocation MUST be able to disable:

- a principal;
- credential class;
- connector;
- MCP server;
- semantic command;
- policy exception;
- ref or root eligibility; and
- pending or running action.

## Audit and attestation

Ferris MUST record:

- authentication and authorization decisions;
- policy evaluation and exceptions;
- plan creation and changes;
- approval, denial, expiry, and revocation;
- connector and MCP calls;
- action start, stop, failure, rollback, and cleanup;
- evidence and attestation production; and
- access to classified or redacted data.

Audit export MUST preserve integrity, ordering where required, source,
schema, tenant, retention, and delivery status.

## Acceptance criteria

GOVERNANCE-001 may advance to Proposed only when:

1. principal and delegation fixtures exist;
2. allow, deny, expiry, revocation, separation-of-duties, and exception cases
   are executable;
3. secrets and credentials are proven absent from durable evidence;
4. tenant isolation and emergency disablement are tested;
5. generic policy works with and without Microsoft adapters;
6. audit and attestation schemas are fixed;
7. data residency, retention, deletion, and redaction tests exist; and
8. all nine roles record a disposition.

