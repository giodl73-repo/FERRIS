# AI-Assisted Rust Engineering

> Status: Guidance only.
> Implementation authority: None. This guide does not authorize product code,
> execution, mutation, approval, or changes to Ferris canonical records.

## Purpose

This series gives Rust maintainers a product-facing way to use AI assistance
without treating model output, compiler acceptance, or Ferris branding as
proof. It applies Ferris's rule that the plan is global and the work is local:
Cargo, rustc, test systems, linkers, native tools, and deployment systems keep
their owner authority. Ferris may coordinate evidence and proposals; it does
not establish owner truth.

The repository boundary and current implementation limits are defined by
[CONTEXT](../../../CONTEXT.md), [agent instructions](../../../AGENTS.md), and
the [Ferris program](../../plans/FERRIS_PROGRAM.md). This series is guidance,
not a MAXIM language reference and not a substitute for repository policy,
specifications, or an approved implementation pulse.

## Series map

1. [Boundary and ownership](01-BOUNDARY-AND-OWNERSHIP.md) - who owns each
   decision and where AI assistance must stop.
2. [Operating workflow](02-OPERATING-WORKFLOW.md) - a bounded
   propose-compile-test-review loop.
3. [Evidence and identity](03-EVIDENCE-AND-IDENTITY.md) - generated-patch
   provenance, claim classes, and evidence handling.
4. [Failure modes and controls](04-FAILURE-MODES-AND-CONTROLS.md) - controls
   for unsafe Rust, FFI, dependencies, build scripts, procedural macros,
   privacy, prompt injection, and misleading results.
5. [Adoption, rollback, and removal](05-ADOPTION-ROLLBACK-AND-REMOVAL.md) -
   reversible introduction without trapping a repository.
6. [Validation roadmap](06-VALIDATION-ROADMAP.md) - behavioral, negative,
   performance, held-out, and maturity evidence.

## Non-negotiable claims boundary

AI may propose code, tests, explanations, mappings, or a narrower validation
plan. It cannot establish:

- source, package, dependency, compiler, test, ABI, or deployment owner truth;
- behavioral correctness, memory safety, soundness, security, or compliance;
- performance improvement without representative measured evidence;
- approval, authorization, policy exception, or release readiness; or
- that a selected test set is equivalent to full-reference validation.

Those limits follow the
[product authority boundary](../../specs/FERRIS_PRODUCT_CONTRACT.md), the
[scope narrowing rules](../../specs/FERRIS_SCOPE_CONTRACT.md), and
[FP-01, FP-04, and FP-11](../../governance/ENGINEERING_PRINCIPLES.md).

## Operating principles

### Start from owner-native truth

Start with a Git revision, Cargo manifest and lock state, exact Cargo activity,
toolchain, target, feature set, profile, repository test gates, and any native
or deployment owner inputs. Do not let an AI-created file list or semantic
summary replace them. Scope is multi-dimensional; package selection is not
test, runtime, ABI, deployment, or policy coverage. See the
[cross-command scope research](../../research/2026-08-10-blueprint-cross-command-scope-model.md)
and [SCOPE-001](../../specs/FERRIS_SCOPE_CONTRACT.md).

### Treat generation as a proposal

Keep the generated patch reviewable as an ordinary diff. Record enough
provenance to distinguish source material, model action, human revisions,
commands, results, failures, and limitations. Do not store credentials,
reusable secrets, unrestricted private inputs, or raw prompts merely to make
the record look complete. See
[TRUST-001](../../specs/FERRIS_TRUST_CONTRACT.md) and
[FERRIS-001](../../specs/FERRIS_EVIDENCE_PACKET_CONTRACT.md).

### Use compiler feedback as a loop, not a verdict

rustc diagnostics can reveal language rejection and guide a revision. Cargo
can reveal owner-selected resolution, units, and freshness. Neither proves the
intended behavior. The loop must proceed through relevant behavioral,
negative, boundary, and repository validation before accountable human review.

### Widen when evidence is weak

Unknown, stale, conflicting, failed, hidden, or unsupported mappings must
widen to the smallest safe named owner boundary or block. AI must not silently
remove work. Full-reference comparison and mandatory gates remain visible.
This is the deterministic authority rule in
[SCOPE-001](../../specs/FERRIS_SCOPE_CONTRACT.md) and
[VALIDATION-001](../../specs/FERRIS_VALIDATION_COVERAGE_CONTRACT.md).

### Preserve ordinary Cargo

Adoption must preserve manifests, lockfiles, editor workflows, repository
correctness without Ferris, explicit full-reference validation, and a
documented removal path. Generated assistance should normally produce a
standard source diff and ordinary commands, not a hidden build graph,
alternate resolver, private correctness cache, or required service.

## Review risk bands

These bands are a review heuristic for this guidance, not a Ferris canonical
schema and not authorization. Repository policy may require a higher band.

| Band | Typical change | Minimum posture |
|---|---|---|
| R0 | Explanation, comments, documentation, mechanical formatting | Review the diff, links, examples, and claims; run applicable documentation checks. |
| R1 | Local safe-Rust implementation with no public contract or dependency change | Compile, run focused behavior and negative tests, preserve ordinary repository gates, and obtain maintainer review. |
| R2 | Public API, concurrency, persistence, dependency or feature change, build script, procedural macro, generated source, or material performance claim | Add owner-specific closure, compatibility and failure tests, full-reference comparison, provenance review, and explicit approval. |
| R3 | `unsafe`, FFI/ABI, cryptography, authentication, privilege, secrets, safety-critical behavior, deployment mutation, artifact restoration, or release-critical optimization | Require specialist review, dedicated safety/security/boundary evidence, rollback proof, broad validation, and accountable human approval. AI remains advisory. |

When several conditions apply, use the highest band. Unknown ownership,
hidden inputs, or unavailable validation raise the band or block the change.

## Nine review lenses

The repository's nine roles are questions to apply, not automatic approvals:

- [Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md):
  identify where Rust guarantees stop and what evidence supports a claim.
- [Compiler Performance Engineer](../../../.roles/parliament/compiler-performance-engineer.md):
  require representative workflows, baselines, variance, and causality.
- [Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md):
  make ABI, ownership, panic, lifetime, threading, and allocation explicit.
- [AI Assurance Skeptic](../../../.roles/parliament/ai-assurance-skeptic.md):
  separate evidence from model assertion and keep failures visible.
- [Ecosystem Strategist](../../../.roles/parliament/ecosystem-strategist.md):
  prefer existing tools and upstream contribution over weaker duplication.
- [Rust Maintainer](../../../.roles/stakeholders/rust-maintainer.md):
  keep patches understandable, ordinary, diagnosable, and removable.
- [Native Platform Adopter](../../../.roles/stakeholders/native-platform-adopter.md):
  require incremental adoption, support, audit, rollback, and operations.
- [Scope Keeper](../../../.roles/editorial/scope-keeper.md):
  keep the capability bounded and non-goals visible.
- [Validation Checker](../../../.roles/editorial/validation-checker.md):
  require reproducible commands, representative fixtures, negative cases,
  environment identity, and honest status.

## Adoption maturity model

The maturity model is evidence-based and reversible:

| Level | Use | Promotion evidence | Rollback expectation |
|---|---|---|---|
| M0 Observe | AI explains code or proposes tests without changing the tree. | Maintainers can identify sources, uncertainty, and prohibited claims. | Disable the assistant with no repository effect. |
| M1 Propose | AI creates reviewable patches in a bounded local workflow. | Provenance, ordinary Cargo checks, behavior tests, negative tests, and human review are repeatable. | Revert the patch and remove tooling without changing owner workflows. |
| M2 Evidence-governed | Teams use risk bands, deterministic scope, required gates, and full-reference comparisons. | False omissions, failures, privacy handling, performance variance, and rollback are measured. | Disable narrowing and return to full owner-native validation. |
| M3 Controlled scale | Multiple repositories use governed assistance with support and renewal owners. | Held-out conformance, cross-platform evidence, incident exercises, adoption cost, and removal proof meet fixed thresholds. | Freeze new actions, drain integrations, export evidence, remove hooks, and verify ordinary workflows. |

Promotion is not based on model quality claims or usage volume. A material
failure, privacy breach, false omission, unexplained performance regression,
or removal failure should stop promotion and can require regression to an
earlier level.

## Source of authority

Canonical record shapes and semantics remain with the
[specification registry](../../specs/README.md), especially Query Forest,
identity, evidence, scope, trust, governance, validation, execution, and
conformance specifications. Checklists in this series must not be serialized
as substitute Query Forest records or used to infer implementation authority.

