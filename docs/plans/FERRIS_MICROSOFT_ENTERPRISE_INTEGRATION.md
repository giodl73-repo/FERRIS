# Ferris Microsoft Enterprise Integration

Status: Draft after nine-role review
Implementation authority: None
Programs: Ferris and Ecosystem Bridge
Specifications: GOVERNANCE-001 and CONNECTOR-001

## Decision

Add two explicit capabilities to the existing seven-program architecture:

1. an **Enterprise Governance Plane** within Ferris; and
2. a governed **Connector and MCP Framework** within Ecosystem Bridge.

Do not create an eighth program. Governance is part of the public Ferris
authority and lifecycle contract. Connectors are replaceable external-owner
adapters. MCP is one connector protocol, not Ferris's internal model or
authority system.

## Microsoft investment thesis

Ferris is a credible Microsoft investment because it connects five active
enterprise concerns:

- Rust and memory-safe native development;
- GitHub and Azure DevOps repository estates;
- Copilot and agent-generated changes;
- software supply-chain governance; and
- Azure identity, policy, artifacts, telemetry, and deployment.

The investment is not justified by another task runner. It is justified by a
governed, explainable affected-work and validation system that can operate
across existing Cargo workspaces while integrating with Microsoft identity,
developer platforms, security controls, and agent protocols.

Official evidence includes:

- [Azure SDK for Rust GA](https://devblogs.microsoft.com/azure-sdk/from-beta-to-stable-announcing-the-azure-sdk-for-rust-ga/);
- [Azure SDK for Rust releases](https://azure.github.io/azure-sdk/releases/latest/rust.html);
- [Azure DevOps and GitHub integration](https://learn.microsoft.com/en-us/azure/devops/cross-service/github-integration?view=azure-devops);
- [Azure Artifacts Cargo support](https://learn.microsoft.com/en-us/azure/devops/artifacts/get-started-cargo?view=azure-devops);
- [GitHub Actions Rust workflows](https://docs.github.com/en/actions/tutorials/build-and-test-code/rust);
- [GitHub artifact attestations](https://docs.github.com/en/actions/security-for-github-actions/using-artifact-attestations);
- [Microsoft Secure Future Initiative supply-chain guidance](https://learn.microsoft.com/en-us/security/zero-trust/sfi/protect-software-supply-chain);
- [Visual Studio MCP integration](https://learn.microsoft.com/en-us/visualstudio/ide/mcp-servers?view=visualstudio);
- [official MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk);
- [Microsoft MCP catalog](https://github.com/microsoft/mcp); and
- [BuildXL](https://github.com/microsoft/BuildXL).

## Enterprise Governance Plane

The governance plane applies organization policy to Ferris plans, actions,
evidence, connectors, and lifecycle.

It owns:

- organization, tenant, project, repository, application, and environment
  identity;
- human, workload, service, agent, and connector principals;
- authentication and delegated authorization;
- role-based and attribute-based permissions;
- separation of duties and required reviewers;
- policy-as-code and policy versioning;
- plan, tool, data, network, secret, and mutation permissions;
- approval, denial, exception, expiry, and renewal;
- data classification, residency, retention, redaction, and deletion;
- secrets, tokens, managed identity, and credential non-persistence;
- audit, attestation, incident, and recovery records;
- cost, concurrency, storage, and execution budgets; and
- tenant isolation, connector allow-lists, and emergency revocation.

Candidate Microsoft integrations include Entra ID, managed identity, Azure
Policy, Key Vault, GitHub Enterprise policy, Azure DevOps permissions,
environment approvals, GitHub artifact attestations, Azure Monitor, and
Microsoft security and compliance systems.

Ferris policy remains product-owned and portable. Microsoft services are
adapters, not hard dependencies of the canonical policy model.

## Connector framework

Every connector declares:

- connector ID, owner, version, protocol, and support state;
- capability manifest;
- authentication and authorization mode;
- tenant, organization, project, repository, subscription, and resource
  scope;
- request, response, pagination, throttling, retry, idempotency, and
  cancellation behavior;
- data classification and redaction;
- network and secret requirements;
- source-of-truth and consistency semantics;
- positive, negative, failure, unsupported, permission, throttling,
  version-skew, and revocation tests;
- telemetry and audit events;
- adoption, upgrade, rollback, disablement, and removal; and
- direct REST, CLI, file, or owner-tool fallback where applicable.

Connectors MUST preserve external owner semantics. A connector result is
evidence, not automatic approval or policy truth.

## Microsoft Connector Pack

The initial pack is a compatibility-tested adapter collection, not a bundled
Microsoft distribution.

| Connector | Initial purpose | Rust path |
|---|---|---|
| GitHub | repositories, revisions, pull requests, checks, Actions, attestations | `octocrab`, REST/GraphQL, `gh`, Actions |
| Azure DevOps | repositories, work items, pipelines, checks, artifacts | REST and `az devops`; no official Rust SDK |
| Entra ID | user, workload, managed identity, tokens, claims | official `azure_identity` where applicable |
| Azure Key Vault | connector and signing secrets without persistence | official Azure SDK Key Vault crates |
| Azure Artifacts | private Cargo feeds and package provenance | Cargo registry protocol and credential provider |
| Azure Monitor | Ferris traces, metrics, logs, and audit export | OpenTelemetry/OTLP; Azure SDK tracing support |
| Azure Policy/ARM | deployment-policy observation and evidence | REST/CLI until stable Rust management coverage |
| BuildXL | larger polyglot build-plan and execution integration | process/file adapter; no Rust SDK |
| Microsoft Graph | organization and collaboration context where approved | community SDK or generated/direct REST adapter |
| MCP | agent tools, resources, schemas, prompts, and governed action requests | official `rmcp` SDK |

Each connector has an independent support profile. A first-party SDK, community
crate, REST adapter, CLI adapter, or process adapter MUST remain visibly
different support classes.

## Ferris MCP surface

A Ferris MCP server MAY expose:

### Read and planning tools

- `ferris.plan`;
- `ferris.affected`;
- `ferris.graph`;
- `ferris.query`;
- `ferris.explain`; and
- `ferris.doctor`.

These remain subject to repository, evidence, data-classification, and resource
policy.

### Action-request tools

- `ferris.go`;
- `ferris.check`; and
- `ferris.test`.

These tools MUST submit an action request. They MUST NOT bypass Ferris approval,
policy, isolation, validation, audit, rollback, or cleanup.

### Resources

MCP resources MAY expose versioned, policy-filtered:

- application definitions;
- profile summaries;
- Blueprint Plans;
- graph and scope projections;
- Query Forest roots and refs;
- evidence schemas;
- connector capability manifests; and
- conformance results.

### Prompts, elicitation, and sampling

- Prompts MAY guide users but MUST NOT establish policy or approval.
- Elicitation MUST be visible, attributable, schema-bounded, and cancellable.
- Sampling or recursive model invocation MUST be disabled by default.
- Enabling sampling requires explicit policy, model identity, budget,
  data-boundary, audit, and stop conditions.
- Tool descriptions and connector content are untrusted inputs and MUST be
  covered by prompt-injection and tool-poisoning controls.

## Adoption sequence

1. Read-only local `ferris` planning.
2. Read-only MCP tools over the same semantic engine.
3. GitHub repository and pull-request evidence.
4. Azure DevOps repository and pipeline evidence.
5. Entra-authenticated connector execution.
6. Azure Artifacts and Key Vault integration.
7. OpenTelemetry export and supply-chain attestations.
8. Separately approved action-request tools.
9. BuildXL, Azure Policy, Graph, or broader enterprise connectors only after
   named consumer demand and support review.

## Success measures

- one plan has identical semantics through CLI and MCP;
- every connector states its maturity and current owner;
- denied, expired, revoked, throttled, unsupported, and version-skew cases are
  explicit;
- credentials never enter Query Forest evidence;
- agent requests cannot bypass approval;
- GitHub and Azure DevOps evidence can coexist without one becoming canonical
  by default;
- connector removal restores direct owner workflows;
- telemetry explains connector and policy latency without exposing secrets;
  and
- Microsoft-specific integrations remain replaceable by generic contracts.

## Non-goals

- an eighth Ferris program;
- an identity provider, secrets vault, policy engine, CI platform, package
  registry, observability backend, or attestation service replacement;
- making Microsoft services mandatory;
- claiming official Microsoft sponsorship;
- treating MCP as an internal trust boundary;
- exposing mutating MCP tools by default;
- storing bearer tokens or secrets in plans, roots, logs, or evidence; and
- implementing gaps in Microsoft SDK coverage without a named consumer and
  maintenance owner.
