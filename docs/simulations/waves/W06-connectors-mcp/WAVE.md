# W06: Connectors and MCP

Status: Complete after retrace
Claim state: simulated

## Goal

Test CLI and MCP semantic parity, indirect prompt injection, MCP tool-surface
poisoning, connector failure, partial pagination, and revocation.

## Locked specification baseline

Baseline commit: `a4ea86a`

The retrace includes FSIM-SCR-012 and FSIM-SCR-013.

## Scenarios

| ID | Scenario | Distinguishing concern | Result |
|---|---|---|---|
| [FSIM-021](FSIM-021-cli-mcp-parity.md) | CLI and MCP plan with identical explicit inputs | One semantic engine and output equivalence | Pass without spec change |
| [FSIM-022](FSIM-022-indirect-prompt-injection.md) | Issue body instructs the agent to widen scope and post data | Untrusted content versus authority | Pass after FSIM-SCR-012 |
| [FSIM-023](FSIM-023-tool-schema-poisoning.md) | MCP tool schema changes after discovery | Capability identity and approval drift | Pass after FSIM-SCR-013 |
| [FSIM-024](FSIM-024-revoked-partial-pagination.md) | Connector is revoked after the first page of owner data | Partial evidence and revocation | Pass without spec change |

## Wave issues

- FSIM-SI-013: connector and MCP content was declared untrusted but lacked a
  deterministic instruction and authority boundary; and
- FSIM-SI-014: connector version did not bind the discovered MCP capability
  surface against post-discovery drift.

## Role review

- Rust Safety Steward: accepted after untrusted content cannot trigger commands
  and schema drift blocks action.
- Compiler Performance Engineer: accepted because connector shortcuts cannot
  replace exact owner evidence or become empty-success results.
- Interop Boundary Auditor: accepted after protocol, schema, command mapping,
  owner identity, and raw references remain independently visible.
- AI Assurance Skeptic: accepted after indirect prompt injection and
  model-visible tool descriptions were denied instruction authority.
- Ecosystem Strategist: accepted because REST, CLI, SDK, and MCP adapters
  remain replaceable profiles behind one semantic contract.
- Rust Maintainer: accepted because CLI/MCP parity and failure diagnostics use
  stable Ferris semantics.
- Native Platform Adopter: accepted for Draft because unsupported connector
  capability remains explicit and owner-native fallback is preserved.
- Scope Keeper: accepted as a bounded integration wave without contacting
  external systems.
- Validation Checker: accepted after parity, injection, schema-drift,
  revocation, and partial-result controls were retraced.

## Disposition

Close W06 with no open P0 or P1 issues. Continue to W07 lifecycle and removal.
