# Ferris Owner Validation Domains Plan

Status: planned from PARLOR, ICELINES, and BISECT evidence; implementation not
yet authorized

Date: 2026-08-30

Primary proof adopter: BISECT PR #44

## Decision

The next Ferris product slice is a strict, non-executable owner-domain contract
that composes owner-defined non-Cargo entrypoint identities with the existing
Cargo reverse-dependency plan.

Ferris will select opaque entrypoint IDs. Repository owners will continue to
define commands, environments, matrices, artifacts, required-check policy, and
success semantics. Ferris will not parse workflow YAML or infer npm, pytest,
Vue, browser, or provider semantics.

## Evidence

PARLOR proved that Cargo package ownership, reverse dependencies, and visible
full-workspace fallback work for a real owner repository. ICELINES proved that
Ferris can add measured artifact qualification without taking over transport or
owner verification.

BISECT exposed the remaining gap:

- the ten-revision sample consumed 747.2 job-minutes;
- five web-only revisions consumed 387.0 job-minutes;
- existing workflows did not build the affected web application;
- Ferris safely classified `web/docs/package.json` as
  `full_workspace_fallback`; and
- BISECT PR #44 run `33318724606` added and passed the missing owner-native
  `npm run build` lane while retaining Ferris plan
  `validation-plan:f2aa57418a85a96f332d9a0f69e08e304b9a439b28d3f4551fbc4afdafd25d66`.

This proves the need for a declared mapping. It does not prove that Ferris
should understand the mapped command.

## Work packages

### FERRIS-DOMAIN-001: Strict declaration

Add one closed, versioned contract containing:

- one portable workspace identity;
- normalized repository-relative path prefixes;
- stable owner-domain IDs;
- opaque owner entrypoint IDs; and
- a deterministic contract identity.

V1 permits only exact path prefixes, not arbitrary glob syntax. Empty,
absolute, traversal-bearing, duplicate, and overlapping prefixes are invalid.
Duplicate domain or entrypoint IDs are invalid. Unknown fields are rejected.

### FERRIS-DOMAIN-002: Conservative composition

Extend `validation-plan` through an explicit opt-in contract argument. Preserve
the current result byte-for-byte when no contract is supplied.

For a supplied contract:

- Cargo-owned Rust paths retain the existing package reverse closure;
- owner-domain paths select the declared opaque entrypoint IDs;
- mixed inputs return the union of both selections;
- unmatched paths require the existing full owner fallback;
- ambiguous ownership is invalid rather than resolved by precedence; and
- selected entrypoints never make the plan executable.

The contract identity, normalized input paths, selected domains, selected
entrypoints, and fallback reasons participate in the plan identity.

### FERRIS-DOMAIN-003: Deleted and renamed paths

Accept normalized lexical workspace-relative paths when the changed file no
longer exists. This is path classification only, not Git discovery.

- Existing paths retain canonical filesystem and symlink/reparse validation.
- Missing inputs must be relative, remain below the workspace root after
  normalization, and contain no unresolved traversal.
- Absolute missing paths, workspace escape, empty values, and ambiguous package
  roots are rejected or widened according to the existing fail-closed contract.
- A later Git-input slice will own base/head selection, rename detection,
  submodules, shallow history, dirty state, and diff identity.

### FERRIS-DOMAIN-004: Revision-bound evidence

Define distinct evidence fields for:

- tested revision;
- proposed/head revision;
- Ferris revision;
- owner-domain contract identity;
- validation-plan identity; and
- owner receipt identity.

Ferris must not label a pull-request merge revision as the source head. Digest
integrity remains distinct from provider-authenticated provenance.

### FERRIS-DOMAIN-005: BISECT replay

Upgrade BISECT PR #44 to pass actual changed paths and the native domain
contract. Retain:

1. one web-only selection that names `web-docs-build`;
2. one mixed Cargo/web selection;
3. one deleted web path;
4. one unknown-path full fallback;
5. one owner build receipt; and
6. one mutation that rejects an undeclared or ambiguous mapping.

Replay BISECT PRs #39 through #43. Keep every existing required workflow during
shadow. Measure Ferris setup cost, owner-build cost, broad-workflow cost, and
candidate avoided job-minutes separately.

### FERRIS-DOMAIN-006: Windows checkout portability

Remove the need for ICELINES' sparse-checkout workaround by shortening or
relocating deeply nested generated simulation fixtures. Add a Windows checkout
control using default Git path settings.

This package is independent of owner-domain semantics and may proceed in
parallel after the contract shape is frozen.

## Verification matrix

| Scenario | Required result |
| --- | --- |
| Existing Rust source | Existing Cargo anchor and reverse closure |
| Web-only path | Declared domain and opaque owner entrypoint |
| Mixed Rust and web paths | Union of Cargo closure and owner entrypoint |
| Deleted declared path | Same domain selection without filesystem existence |
| Unknown path | Visible full owner fallback |
| Nested or duplicate prefix | Invalid contract |
| Traversal or absolute missing path | Invalid input |
| Contract mutation | Different contract and plan identities |
| No contract supplied | Existing serialized result and identity unchanged |

Focused core, CLI, schema, public-contract, PITFALL, VTRACE, Windows, and adopter
tests must pass. Role review must include a runtime owner, an adopter/operator,
and a skeptical simplicity or failure-mode lens.

## Promotion and deletion gates

The native contract may replace BISECT's workflow-local representative-path
assertion only after current-head hosted evidence passes. Existing broad
workflows may be narrowed only after the five historical web revisions replay
without a missing owner-required signal and branch-policy reconciliation proves
that path filtering will not strand required checks.

No savings claim is promoted from the current 387.0-job-minute opportunity
sample. Observed shadow results must distinguish avoided work from new Ferris
and owner-build overhead.

## Non-goals

- executing owner entrypoints;
- embedding package-manager or test-runner semantics;
- parsing GitHub Actions, Azure Pipelines, Make, or task-runner files;
- automatically changing required checks or path filters;
- scheduling, cancellation, or artifact transport;
- authenticated provenance; and
- a universal glob or policy language.
