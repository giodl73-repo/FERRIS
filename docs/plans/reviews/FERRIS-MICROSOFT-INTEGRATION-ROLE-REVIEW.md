# Ferris Microsoft Integration Nine-Role Review

Date: 2026-08-10
Plan: Ferris Microsoft Enterprise Integration
Specifications: GOVERNANCE-001 and CONNECTOR-001
Disposition: Accepted as Draft after required revisions
Implementation authority: None

## Executive disposition

All nine roles accept the Enterprise Governance Plane and Connector/MCP
Framework as capabilities within the existing seven programs.

No role supports an eighth program, Microsoft-only canonical model, or
default-enabled mutating MCP tools.

## Role dispositions

### Rust Safety Steward

Accept with the requirement that connector, MCP, build, test, attestation, and
policy success never proves behavioral safety or soundness. Unsafe native
adapters require separate review.

### Compiler Performance Engineer

Accept if connector, policy, identity, network, throttling, and telemetry
latency remain visible and do not contaminate compiler or build-performance
claims.

### Interop Boundary Auditor

Accept after requiring explicit REST, CLI, process, protocol, ABI, data,
identity, error, retry, cancellation, and version boundaries for every
connector.

### AI Assurance Skeptic

Accept after disabling MCP sampling by default, treating tool content as
untrusted, separating discovery from authority, recording model actions, and
prohibiting model-visible credentials.

### Ecosystem Strategist

Accept as a differentiated Microsoft investment wedge provided Ferris uses
official SDKs where mature, preserves community and owner tools, and does not
rebuild missing SDKs without a consumer.

### Rust Maintainer

Accept if CLI and MCP share semantics, connector failures are actionable, and
ordinary `cargo`, `gh`, `az`, Azure DevOps, and GitHub workflows remain usable
after connector removal.

### Native Platform Adopter

Accept for Draft. Entra, Key Vault, artifacts, policy, monitoring, BuildXL,
Graph, deployment, compliance, support, and recovery require explicit support
profiles before adoption.

### Scope Keeper

Accept after keeping governance in Ferris and connectors in Ecosystem Bridge.
Reject an eighth program, mandatory Microsoft dependency, or broad SDK-building
program.

### Validation Checker

Accept as Draft. Proposed status requires identity, permission, tenant,
consent, throttling, retry, version-skew, prompt-injection, secret-exposure,
revocation, audit, CLI/MCP parity, rollback, and removal fixtures.

## Remaining gates

1. Freeze connector manifest, governance policy, audit, and MCP schemas.
2. Select exact SDK and protocol versions.
3. Test one stable SDK, one preview SDK, one REST, one CLI/process, and one MCP
   connector.
4. Prove credential non-persistence and tenant isolation.
5. Freeze CLI/MCP parity and action-request fixtures.
6. Record named Microsoft and generic fallback environments.
7. Repeat all nine role reviews over measured results.

## Decision

Advance GOVERNANCE-001 and CONNECTOR-001 to Draft.

Do not authorize implementation.
