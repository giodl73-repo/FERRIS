# EXP-01: Microsoft Rust Connector Matrix

Date: 2026-08-10
Result: Rust support is strong for Azure core services and MCP, usable through
community or protocol adapters for several developer surfaces, and materially
incomplete for Azure DevOps, BuildXL, Graph, policy management, direct Azure
Monitor export, and attestation workflows.

| Surface | Rust path | Maturity | Ferris disposition |
|---|---|---|---|
| Azure Core and Identity | official `azure_core`, `azure_identity` | first-party stable | adopt behind connector profile |
| Azure Blob and Queue Storage | official Azure SDK crates | first-party stable | adopt for evidence/artifact transport only after trust and economics gates |
| Azure Key Vault | official secrets, keys, certificates crates | first-party stable | adopt for secret references; never persist secrets |
| Cosmos DB | official Azure SDK crates | first-party preview | prototype only with exact version profile |
| Event Hubs | official Azure SDK crate | first-party preview | prototype telemetry transport only |
| Service Bus | evolving or incomplete official coverage | unsupported/preview by operation | REST/protocol or defer |
| GitHub | `octocrab`, REST/GraphQL, `gh`, Actions | strong community and owner tools | adopt connector with owner evidence |
| GitHub artifact attestations | Actions, REST, `gh` | owner workflow, no complete Rust SDK | integrate workflow rather than replace |
| Azure DevOps | REST and `az devops` | no official Rust SDK | bounded REST/CLI connector |
| Azure Artifacts Cargo | Cargo registry protocol and credential provider | official service support | adopt as profile-tested registry |
| Microsoft Graph | `graph-rs-sdk` or direct/generated REST | community | prototype with maintenance and auth review |
| Azure Policy and ARM | REST, templates, CLI | no stable broad Rust management SDK | evidence and deployment adapter only |
| Azure Monitor | OpenTelemetry/OTLP, Azure SDK tracing, community exporter | mixed | prefer OTLP boundary |
| BuildXL | process, files, logs, custom scheduling adapter | no Rust SDK | defer until named polyglot consumer |
| MCP | official `modelcontextprotocol/rust-sdk` / `rmcp` | official protocol SDK | adopt for governed CLI-equivalent adapter |
| SBOM | Cargo CycloneDX and ecosystem tools | community/standards | consume and validate standard outputs |
| Sigstore/in-toto/SLSA | Rust libraries plus owner CLIs/actions with uneven coverage | mixed | adapter boundary; do not invent attestation semantics |

## Sources

- [Azure SDK for Rust GA](https://devblogs.microsoft.com/azure-sdk/from-beta-to-stable-announcing-the-azure-sdk-for-rust-ga/)
- [Azure SDK Rust release index](https://azure.github.io/azure-sdk/releases/latest/rust.html)
- [Azure SDK for Rust repository](https://github.com/Azure/azure-sdk-for-rust)
- [Azure Artifacts Cargo packages](https://learn.microsoft.com/en-us/azure/devops/artifacts/get-started-cargo?view=azure-devops)
- [GitHub Actions for Rust](https://docs.github.com/en/actions/tutorials/build-and-test-code/rust)
- [GitHub artifact attestations](https://docs.github.com/en/actions/security-for-github-actions/using-artifact-attestations)
- [Azure DevOps GitHub integration](https://learn.microsoft.com/en-us/azure/devops/cross-service/github-integration?view=azure-devops)
- [OpenTelemetry Rust](https://opentelemetry.io/docs/languages/rust/)
- [Azure SDK Rust logging](https://learn.microsoft.com/en-us/azure/developer/rust/sdk/logging)
- [official MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [Visual Studio MCP servers](https://learn.microsoft.com/en-us/visualstudio/ide/mcp-servers?view=visualstudio)
- [BuildXL](https://github.com/microsoft/BuildXL)

Availability and maturity are dated observations, not support commitments by
Ferris.
