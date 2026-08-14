# Pulse 29 Public-Artifact Checkout Normalization Nine-Role Review

Date: 2026-08-14
Disposition: Accept LF checkout normalization
Implementation authority: Attributes, public metadata, receipt, and test-only
validation only

## Review question

Do anchored recursive LF checkout rules and normalized public bindings remove
platform-dependent release bytes without changing production behavior or
reopening Pulse 28?

## Role dispositions

| Role | Disposition | Boundary |
|---|---|---|
| Rust Safety Steward | Accept | No production Rust or unsafe boundary changes; tests only hash public bytes |
| Compiler Performance Engineer | Accept | Checkout materialization and hash recomputation are reproducibility controls, not performance evidence |
| Interop Boundary Auditor | Accept | Anchored recursive attributes make `.py`, `.json`, `.md`, and nested release bytes identical across Git checkout boundaries |
| AI Assurance Skeptic | Accept | The Pulse 28 invalid result and historical mismatch remain unchanged; normalization is not recast as diagnostic success |
| Ecosystem Strategist | Accept | Standard Git attributes add no package manager, registry, owner-system, network, credential, or external mutation authority |
| Rust Maintainer | Accept | The change is isolated to release metadata, governance, and integration tests and is removable without trapping Cargo workflows |
| Native Platform Adopter | Accept | Windows `core.autocrlf=true` materialization is directly verified without claiming native Linux support |
| Scope Keeper | Accept | Pulse 29 normalizes public checkout only; it creates no retry, candidate, fix, score, certification, or PLATFORM-001 authority |
| Validation Checker | Accept | A resulting-index `checkout-index` copy passes 36 LF checks and 76 exact binding checks, and the receipt payload seal is repository-tested |

## Shared conditions

All nine roles require:

- the two anchored recursive `.gitattributes` rules to remain exact;
- all 14 Pulse 25 and 22 Pulse 27 release-tree files to materialize with LF
  and zero CR bytes;
- exact recomputation of the nine Pulse 25 and 20 Pulse 27 manifest file
  sizes and raw SHA-256 digests;
- exact recomputation of three Pulse 25 and four Pulse 27 aggregates;
- nine byte-identical collector-copy checks across Pulse 25 and Pulse 27;
- 76 passed and zero failed materialization binding checks;
- a temporary alternate index and `git checkout-index` source, not a copy of
  the current working tree;
- Windows `core.autocrlf=true`;
- receipt raw SHA-256
  `sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225`
  and payload identity
  `sha256:92e245685cbb1b6ce938701a901c4de9b9202f9149537690e646d13a113deb40`;
- no private data, build, preflight, generation, diagnostic candidate, pair,
  retry, or production source change; and
- no modification, retry, or reinterpretation of the Pulse 28 public result.

## Decision

All nine roles accept Pulse 29 as public-artifact checkout normalization only.
The normalized release bindings are stable under Windows
`core.autocrlf=true`, and the historical Pulse 28 invalid result remains
closed.

PLATFORM-001 remains Draft solely because of the immutable valid Pulse 17
`process-exit-agreement` failure.
