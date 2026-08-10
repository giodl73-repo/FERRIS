# Ferris Microsoft Enterprise Integration

Date: 2026-08-10
Status: Complete
Decision: add an Enterprise Governance Plane to Ferris and a governed
Connector/MCP Framework to Ecosystem Bridge. Do not add an eighth program.

## Decision supported

This research decides whether Microsoft enterprise governance, developer
platforms, connectors, and MCP require a new Ferris program.

It informs:

- [Ferris Microsoft Enterprise Integration](../plans/FERRIS_MICROSOFT_ENTERPRISE_INTEGRATION.md);
- GOVERNANCE-001;
- CONNECTOR-001;
- TRUST-001;
- EXECUTION-001;
- VIEW-001; and
- CONFORMANCE-001.

The detailed service and Rust maturity assessment is recorded in
[EXP-01](ferris-microsoft-integration/results/EXP-01-microsoft-rust-connector-matrix.md).

## Observations

Microsoft-facing Rust integration is uneven:

- the Azure SDK for Rust has stable first-party core, identity, storage, and
  Key Vault coverage;
- Cosmos DB and Event Hubs remain version-sensitive previews;
- Azure Artifacts supports Cargo through the registry protocol;
- GitHub has strong APIs, Actions, CLI, and community Rust clients;
- Azure DevOps, BuildXL, Microsoft Graph, Azure Policy/ARM, direct Azure Monitor
  export, and supply-chain attestation have weaker or indirect Rust SDK paths;
- MCP has an official Rust SDK and growing Microsoft/GitHub product support;
  and
- Microsoft enterprise identity, policy, secret, audit, and compliance systems
  are valuable adapters but should not become canonical Ferris dependencies.

## Recommendations

### Adopt now

- Add GOVERNANCE-001 and CONNECTOR-001 to the Ferris specification spine.
- Define portable identity, policy, approval, audit, data, tenant, secret,
  budget, and revocation contracts.
- Define connector maturity classes and explicit REST/CLI/process fallbacks.
- Specify a read-first Ferris MCP surface over the shared semantic engine.
- Keep action tools behind ordinary Ferris approval and execution contracts.
- Treat Microsoft integration as a high-value connector pack and investment
  wedge.

Owner: FERRIS specifications and future approved adapters.

Expected validation: identity and permission matrices, connector conformance,
CLI/MCP parity, negative and revoked cases, secret non-persistence,
cross-tenant isolation, prompt-injection controls, removal, and owner-tool
fallback.

### Prototype behind compatibility boundaries

- official Azure Identity and Key Vault adapter;
- GitHub read connector;
- Azure DevOps REST read connector;
- Azure Artifacts Cargo profile;
- OTLP export;
- read-only `ferris-mcp` server using `rmcp`; and
- action-request flow that cannot execute without approval.

### Reject or defer

- an eighth program;
- a Microsoft-only canonical policy or identity schema;
- direct implementation of missing service SDKs without a consumer;
- default-enabled mutating MCP tools;
- model-visible credentials;
- BuildXL integration before a named polyglot estate;
- claiming Microsoft sponsorship; and
- replacing GitHub, Azure DevOps, Entra, Key Vault, Azure Policy, Azure Monitor,
  artifact attestations, or MCP governance.

## Findings

### FERRIS-769: Microsoft integration fits two existing programs

**Sources:** Ferris Seven-Program Architecture; Microsoft connector matrix.

**Observed behavior:** policy and approval belong to Ferris, while service and
protocol integration belongs to Ecosystem Bridge.

**Implication:** add governance and connector capabilities without an eighth
program.

**Confidence:** High.

### FERRIS-770: the Microsoft investment case is stronger than a task-runner
case

**Sources:** Azure SDK for Rust GA; Azure DevOps/GitHub integration; Secure
Future Initiative supply-chain guidance; MCP product support.

**Observed behavior:** Microsoft spans Rust infrastructure, developer
platforms, AI agents, identity, software supply chain, cloud, and enterprise
governance.

**Implication:** Ferris can provide a differentiated governed affected-work
and validation layer across those investments.

**Confidence:** High for strategic fit; investment ownership remains a
business decision.

### FERRIS-771: Rust SDK maturity must be connector evidence

**Sources:** Azure SDK release index and EXP-01.

**Observed behavior:** first-party stable, first-party preview, community,
generated, REST, CLI, and process integrations coexist.

**Implication:** connector maturity is versioned evidence and cannot be hidden
behind one “supported” label.

**Confidence:** High.

### FERRIS-772: enterprise governance is broader than authentication

**Sources:** Ferris action, trust, profile, and conformance requirements;
Microsoft identity and supply-chain surfaces.

**Observed behavior:** enterprise action requires authorization, policy,
separation of duties, data controls, secrets, tenant isolation, budgets,
audit, attestation, revocation, and recovery.

**Implication:** GOVERNANCE-001 must be a first-class specification.

**Confidence:** High.

### FERRIS-773: MCP is a strong Ferris adapter

**Sources:** official MCP Rust SDK; Visual Studio MCP documentation; Microsoft
MCP catalog.

**Observed behavior:** MCP provides standardized tools, resources, prompts,
elicitation, sampling, logging, progress, cancellation, and transports across
agent clients.

**Implication:** Ferris can expose its shared semantic commands through MCP
without changing its internal model.

**Confidence:** High.

### FERRIS-774: MCP discovery and consent do not replace Ferris authority

**Sources:** MCP security model and Ferris governance/action contracts.

**Observed behavior:** protocol capability discovery, client consent, and
server authorization do not establish repository policy, validation coverage,
or execution approval.

**Implication:** action tools create requests and remain gated by GOVERNANCE,
EXECUTION, and CONFORMANCE.

**Confidence:** High.

### FERRIS-775: recursive agent features need explicit containment

**Sources:** MCP sampling and elicitation capabilities; AI Assurance role.

**Observed behavior:** sampling, elicitation, prompts, and tool content can
expand model authority, data exposure, cost, and prompt-injection risk.

**Implication:** sampling is disabled by default and requires model, data,
budget, recursion, audit, and stop policy.

**Confidence:** High.

### FERRIS-776: REST and CLI adapters remain valid enterprise boundaries

**Sources:** Azure DevOps, Azure Policy/ARM, BuildXL, Graph, and attestation
coverage in EXP-01.

**Observed behavior:** several important Microsoft surfaces lack stable
first-party Rust SDKs but expose supported REST, CLI, file, process, or
workflow contracts.

**Implication:** Ferris should standardize connector behavior rather than wait
for or recreate every SDK.

**Confidence:** High.

### FERRIS-777: credentials must remain outside model-visible evidence

**Sources:** Azure Identity, Key Vault, MCP, Ferris Trust and AI Assurance
requirements.

**Observed behavior:** plans, prompts, tool arguments, logs, roots, and evidence
can become durable or model-visible.

**Implication:** connectors use references and ephemeral credential handling;
secrets and reusable tokens are prohibited from durable evidence.

**Confidence:** High.

### FERRIS-778: Microsoft integrations must remain replaceable

**Sources:** seven-program owner boundaries and PRODUCT-001 portability.

**Observed behavior:** enterprises may use GitHub, Azure DevOps, both, or
neither, and may replace identity, policy, vault, telemetry, or CI systems.

**Implication:** canonical Ferris contracts remain provider-neutral and every
connector has removal and direct-owner fallback.

**Confidence:** High.

## Limitations

- Service, SDK, protocol, and preview status changes frequently.
- No Microsoft sponsorship or internal adoption is established.
- Exact Entra, Azure Policy, compliance, and MCP enterprise-control behavior
  requires tenant-specific validation.
- The research does not select implementation crates or freeze versions.
