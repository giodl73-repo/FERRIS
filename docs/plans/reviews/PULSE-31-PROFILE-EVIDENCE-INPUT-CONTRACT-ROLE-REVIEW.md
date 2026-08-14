# Pulse 31 Profile-Evidence Input Contract Nine-Role Review

Date: 2026-08-14
Disposition: Accept public governance/schema/test-only release
Implementation authority: Public contract, schema, fixtures, documentation,
and test-only validation only

## Review question

Does Pulse 31 publish the complete existing `ferris.profile-evidence/v0`
acceptance boundary for independent generation without changing production or
reopening the invalid Pulse 30 program?

## Role dispositions

| Role | Disposition | Boundary |
|---|---|---|
| Rust Safety Steward | Accept | No production Rust, unsafe code, execution, or safety/correctness claim changes |
| Compiler Performance Engineer | Accept | The 1,048,576-byte bound is an input limit, not a performance claim or benchmark |
| Interop Boundary Auditor | Accept | UTF-8 JSON framing, strict duplicates, recursive visible-ASCII keys, exact shapes, and classification precedence are explicit |
| AI Assurance Skeptic | Accept | Pulse 30 remains invalid/null; the release closes only a public documentation gap and makes no inferred conclusion |
| Ecosystem Strategist | Accept | The contract preserves ordinary JSON/Cargo ownership and adds no resolver, registry, network, credential, or dependency |
| Rust Maintainer | Accept | One removable schema/fixture/test surface mirrors current acceptance without product churn |
| Native Platform Adopter | Accept | Byte bounds, regular-file states, LF public artifacts, rollback by removal, and no support claim are explicit |
| Scope Keeper | Accept | Governance/schema/test-only work does not generate profiles, relaunch diagnostics, or widen product authority |
| Validation Checker | Accept | Six positive fixtures, 33 negative controls, exact-size boundary, strict parsing, schema structure, and documentation links are testable |

## Shared findings

All nine roles record:

- closed root membership of exactly five required fields;
- schema constant `ferris.profile-evidence/v0`;
- 1-through-256 visible ASCII rules for identity metadata;
- a closed twelve-member `sections` object;
- recursively arbitrary JSON section values with bounded visible-ASCII member
  names at every object depth;
- duplicate-member rejection as a normative parsing rule;
- inclusive 1,048,576-byte complete-file maximum;
- explicit unavailable, non-file, oversized, malformed, duplicate, invalid-key,
  unsupported-schema, invalid-shape, and invalid-identity classifications;
- the distinction between parsed-value JSON Schema validation and companion
  byte/framing/filesystem rules;
- six positive fixtures and 33 declared-invalid controls;
- raw schema identity
  `sha256:67946b1a392d2d7537d487d343bee31439606c76b2d71862b97ff46641c3d62b`;
  and
- zero production changes, new dependencies, diagnostic executions, category
  conclusions, fix authority, support claims, or PLATFORM-001 status changes.

## Decision

All nine roles accept the public contract release. It is sufficient for an
independent generator that has no Ferris source or test access. Pulse 30
remains permanently invalid and non-retryable with zero candidates/processes
and a null category conclusion. Pulse 31 grants no diagnostic authority.
