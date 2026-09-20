# FERRIS Application Readiness Composition Contract Review

Date: 2026-09-19
Status: Complete
Pulse: Environment Readiness Pulse 06
Disposition: Accept APP-READINESS-001 as Draft; withhold implementation

## Executive disposition

The eleven roles accept APP-READINESS-001 as the smallest safe composition
boundary for explicit multi-workspace applications. It preserves each
READINESS-001 workspace result and makes the worst admissible state visible
without invoking Cargo or owner work.

The review does not approve product implementation. Application-root
requirements remain intentionally unsupported because reusing a workspace V1
declaration at that root would change its semantics.

## Role dispositions

| Role | Disposition | Finding |
|---|---|---|
| Rust Safety Steward | `accept-no-safety-claim` | The pulse changes no Rust and makes no safety or correctness claim. |
| Compiler Performance Engineer | `accept-no-performance-claim` | The contract defines no timing, cache, parallel execution, or savings claim. |
| Interop Boundary Auditor | `accept-lossless-references` | Child reports retain exact workspace IDs, requirement digests, report IDs, and statuses instead of being flattened. |
| AI Assurance Skeptic | `accept-explicit-evidence-only` | Aggregate status is derived from typed child states; application-root and owner-command readiness remain explicit limitations. |
| Ecosystem Strategist | `accept-owner-aligned-composition` | The request references the existing Application Definition and Cargo manifests rather than introducing another resolver or topology manifest. |
| Rust Maintainer | `accept-removable-contract` | Adoption would add optional records only; removal leaves Cargo, application definitions, requirements, and owner commands unchanged. |
| Native Platform Adopter | `accept-portable-draft` | Portable paths and platform-aware filesystem identity are specified, but no cross-platform implementation evidence exists. |
| Scope Keeper | `accept-contract-only` | No product code, dependency, command behavior, adopter, or private consumer changed. |
| Validation Checker | `accept-frozen-controls` | Schemas, exact-byte fixture digests, canonical aggregate identities, precedence cases, and structural and semantic controls are frozen. |
| Product Value Governor | `continue-within-budget` | Preventing false application-wide readiness directly addresses the private multi-workspace evidence gap within one bounded contract pulse. |
| Autonomy Supervisor | `stop-after-pulse-06` | The authorized contract decision is complete; implementation or application-root requirements require fresh approval. |

## Review corrections

Independent review identified and this pulse corrected:

1. absent application and workspace requirement base fixtures;
2. ambiguity between workspace-root and member-package manifests;
3. missing distinct and non-nested workspace-root invariants;
4. missing child-stale aggregate diagnostics;
5. unspecified diagnostic-array ordering;
6. a coverage control that failed structurally before testing coverage; and
7. illustrative rather than computed request and aggregate identities;
8. absent workspace manifests for filesystem-root controls;
9. ambiguous path relativity when the Application Definition was nested; and
10. semantic controls shadowed by earlier digest or manifest mismatch.

The final contract requires workspace-root `Cargo.toml` paths, validated
filesystem uniqueness and non-nesting, separate input-stale and
workspace-stale diagnostics, and lexicographically ordered diagnostics.

## Validation evidence

- both new JSON Schemas parse and validate their positive fixtures;
- the existing READINESS-001 schema validates all three workspace declarations;
- computed SHA-256 digests match the exact application, request, and requirement
  fixture bytes;
- aggregate report IDs reproduce from canonical JSON with `report_id` omitted;
- structural controls reject traversal, stale-as-ready, and private absolute
  path fields;
- semantic controls cover duplicate or nested roots, ID and digest mismatch,
  definition coverage, and aggregate precedence;
- changed Markdown links and code fences validate; and
- `git diff --check` passes.

## Remaining gates

- No implementation or CLI surface is approved.
- A future implementation must add strict passive `ferris.application/v0`
  loading without the current Cargo-resolving loader.
- Cross-platform filesystem-equivalence and reparse/symlink cases need
  executable tests.
- A separately versioned application-root requirements contract is required
  before observing files above workspace roots.
- A command-result envelope, adoption, private-consumer rerun, support, and
  production claims each require separate authority.

## Final authority

APP-READINESS-001 is Draft only. Pulse 06 is complete and exhausted. No
implementation, successor pulse, application-root requirement, adopter change,
or support claim follows.
