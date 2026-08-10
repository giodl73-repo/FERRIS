# BLUE-Q07: Microsoft Governance and Connectors

**Status:** Complete

## Research question

Do Microsoft enterprise governance, developer-platform connectors, and MCP
require an eighth Ferris program, and what Rust integration maturity and
authority boundaries should govern them?

## Decision

Do not add an eighth program.

- Add the Enterprise Governance Plane to Ferris.
- Add the Connector and MCP Framework to Ecosystem Bridge.
- Add GOVERNANCE-001 and CONNECTOR-001.
- Treat Microsoft integrations as replaceable connector profiles.
- Use MCP as a governed adapter over the shared Ferris semantic engine.

## Outputs

- [Ferris Microsoft enterprise integration research](../../2026-08-10-ferris-microsoft-enterprise-integration.md)
- [Ferris Microsoft enterprise integration plan](../../../plans/FERRIS_MICROSOFT_ENTERPRISE_INTEGRATION.md)
- [EXP-01 Microsoft Rust connector matrix](../../ferris-microsoft-integration/results/EXP-01-microsoft-rust-connector-matrix.md)
- [Nine-role review](../../../plans/reviews/FERRIS-MICROSOFT-INTEGRATION-ROLE-REVIEW.md)

## Non-goals

- implementation authority;
- mandatory Microsoft dependencies;
- model-visible credentials;
- default mutating MCP tools;
- replacing Microsoft or GitHub services; and
- claiming Microsoft sponsorship.
