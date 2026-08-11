# CONNECTOR-001: Ferris Connector and MCP Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: PRODUCT-001, CONTRACT-001, EVIDENCE-001, TRUST-001, GOVERNANCE-001

## Purpose

This specification defines replaceable connectors between Ferris and external
owner systems, including MCP clients and servers.

## Connector manifest

Every connector MUST declare:

- stable connector ID and implementation version;
- owner and support contact;
- protocol and protocol version;
- capabilities and semantic command mappings;
- authentication and authorization modes;
- supported scopes, platforms, endpoints, and clouds;
- data classes read and written;
- consistency, pagination, throttling, retry, idempotency, and cancellation;
- telemetry and audit events;
- unsupported and degraded behavior;
- version and deprecation policy; and
- adoption, upgrade, rollback, disablement, and removal.

Connector maturity MUST use explicit classes such as:

- first-party stable;
- first-party preview;
- community supported;
- generated;
- direct REST;
- CLI/process;
- experimental; and
- unsupported.

One class MUST NOT be presented as another.

## Capability snapshot

Every connector discovery or MCP session MUST produce a versioned capability
snapshot containing:

- connector, endpoint, server, transport, and protocol identities;
- implementation and advertised capability versions;
- semantic command mappings;
- tool names, descriptions, input and output schemas, and canonical digests;
- resource and prompt identities, schemas, and canonical digests;
- elicitation and sampling capabilities;
- authentication audience and granted permission classes;
- data classifications and tenant boundary;
- observation time, expiry, and renewal trigger;
- trust, revocation, and policy state; and
- unsupported, omitted, or degraded features.

Planning and action records MUST bind the snapshot identity when connector or
MCP behavior is material. A changed endpoint, schema, digest, command mapping,
permission, audience, trust state, or advertised capability invalidates the
prior snapshot. Ferris MUST rediscover and revalidate before use, then replan
and obtain renewed approval when an Action Plan boundary changed.

## Owner semantics

A connector MUST preserve external owner:

- identifiers;
- authorization decisions;
- state and consistency semantics;
- error and throttling behavior;
- version and deprecation rules;
- audit references; and
- unsupported states.

Ferris MAY normalize these into typed records. It MUST retain the original
owner identity and raw evidence reference.

## Failure behavior

Connectors MUST distinguish:

- authentication failure;
- authorization denial;
- consent required;
- throttled;
- timeout;
- cancellation;
- transient owner failure;
- permanent owner failure;
- malformed response;
- schema or protocol version mismatch;
- stale or conflicting data;
- unsupported capability;
- revoked connector; and
- partial result.

No connector failure may become an empty successful result.

## MCP adapter

A Ferris MCP server MUST use the same semantic engine as `ferris` and
`cargo ferris`.

Read/planning tools MAY include:

- `ferris.plan`;
- `ferris.affected`;
- `ferris.graph`;
- `ferris.query`;
- `ferris.explain`; and
- `ferris.doctor`.

Action tools MAY include `ferris.run`, `ferris.check`, and `ferris.test`, but
MUST create an action request and MUST NOT bypass GOVERNANCE-001,
EXECUTION-001, or CONFORMANCE-001.

MCP tools, resources, prompts, elicitation, sampling, logging, progress,
cancellation, and transport features MUST be versioned independently.

### MCP security

- Tool discovery is not authorization.
- Tool descriptions, prompts, resources, and connector content are untrusted.
- Prompt-injection and tool-poisoning controls are mandatory.
- Elicitation requires visible attribution, schema bounds, consent, and
  cancellation.
- Sampling is disabled by default.
- Sampling requires approved model, instructions, data boundary, budget,
  recursion limit, logging, and stop conditions.
- Tokens and secrets MUST NOT appear in tool arguments visible to models,
  resources, prompts, plans, roots, or logs.
- A client or server protocol downgrade MUST be explicit and policy-approved.

External owner text, issue bodies, comments, logs, tool descriptions,
resources, prompts, and model-visible connector content MUST be processed as
untrusted data. Embedded requests or instructions MUST NOT:

- select or change a Ferris semantic command;
- widen or narrow scope;
- change policy, approval, trust, or data disclosure;
- select a connector, tool, resource, prompt, model, or credential;
- provide authoritative tool arguments; or
- trigger an action request or external side effect.

A proposal derived from untrusted content MUST retain exact source provenance,
use a bounded typed schema, and pass the same deterministic validation,
planning, resolution, governance, and approval path as any other proposal.
Suspected prompt injection or tool poisoning MUST produce a security
diagnostic and retained evidence without executing the embedded instruction.

## Microsoft connector profiles

CONNECTOR-001 MUST support independent profiles for:

- GitHub;
- Azure DevOps;
- Entra ID;
- Azure Key Vault;
- Azure Artifacts;
- Azure Monitor or OTLP;
- Azure Policy and ARM;
- BuildXL;
- Microsoft Graph; and
- MCP.

The absence of a first-party Rust SDK MUST remain visible. REST, CLI, generated,
community, and process adapters require their own support, security, and
maintenance evidence.

## Acceptance criteria

CONNECTOR-001 may advance to Proposed only when:

1. a reference manifest schema is fixed;
2. one first-party SDK, one REST, one CLI/process, and one MCP adapter pass the
   same core conformance suite;
3. permission, consent, throttling, retry, cancellation, partial-result,
   version-skew, revocation, and removal fixtures exist;
4. CLI and MCP produce equivalent plans for identical explicit inputs;
5. action tools cannot bypass approval;
6. prompt-injection and tool-poisoning cases are tested;
7. credentials are proven absent from model-visible and durable records; and
8. all nine roles record a disposition.
