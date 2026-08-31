# Ferris Current Strategy and Feature Set

Status: Current product and adoption reference

## Mission

Ferris is an owner-first build, validation, execution, and evidence layer for
Rust repositories and applications. It makes repository work selectable,
explainable, bounded, and auditable without replacing the tools or policies
that already define correctness.

> **The plan is global; the work is local.**

Cargo remains authoritative for Rust packages, resolution, compilation units,
freshness, and compilation. Repository owners remain authoritative for
commands, environments, credentials, required checks, success semantics,
publication, and rollback. Ferris owns deterministic planning, conservative
fallback, approved local execution contracts, and portable evidence.

## Architecture

```text
Cargo workspace truth + owner declarations + explicit application relationships
  -> Blueprint plans and validation selections
  -> approved Action Plans
  -> owner-native local commands
  -> deterministic receipts, replay, and compatibility evidence
```

Ferris exposes the same semantic engine through `ferris` and the Cargo-native
`cargo ferris` adapter. Blueprint is the internal planning model, not a separate
public product. Query Forest and the broader specification spine remain
longer-term architecture; they do not expand the authority of the implemented
CLI.

## Current feature set

| Capability | Commands | Current maturity | Owner boundary |
|---|---|---|---|
| Workspace intelligence | `plan`, `explain`, `graph`, `doctor` | Implemented bounded local planning and diagnostics | Cargo owns workspace and package truth |
| Validation intelligence | `validation-plan` | Implemented explicit path/package selection, deleted paths, owner domains, conservative Cargo closure, visible fallback, and revision-bound mode | Owners declare non-Cargo domains and retain every executable command |
| Application planning | `federated-plan`, `federated-validation-plan`, `revision-skew` | Implemented bounded planning over explicit workspaces, relationships, and local revision evidence | Ferris does not discover relationships or combine Cargo resolution |
| Profile comparison | `profile-diff` | Implemented experimental two-record comparison | No support, compatibility, or certification decision is inferred |
| Controlled execution | `go`, `verify` | Implemented subset for explicitly approved Action Plans, bounded owner-native processes, deterministic receipts, and receipt verification | Ferris does not invent commands, approvals, credentials, or success policy |
| Evidence replay | `replay` | Implemented receipt-to-remote-failure comparison | Replay is evidence, not proof of prevented production failures or savings |
| Scheduling analysis | `schedule` | Implemented counterfactual replay across conservative, fail-fast, flush-out, and balanced profiles | No live scheduler; only owner labels can authorize projected cancellation |
| Artifact evidence | `artifacts` | Implemented compatibility, complete fan-in, measured local file qualification, and optional fail-closed compatibility enforcement | No build, transport, cache, signing, publication, or deployment ownership |

All public records use versioned schemas or schema identifiers and deterministic
identities appropriate to their boundary. Validation planning separates the
selection-semantic `validation_plan_id` from revision context through
`ferris.validation-revision-binding/v1`. Execution similarly separates approved
plan identity from result and receipt identity.

## Core workflows

### Explicit-path validation

An owner supplies changed paths, deleted workspace-relative paths, or package
names. Ferris maps regular Rust files through Cargo package ownership and
reverse dependencies, unions any explicitly declared non-Cargo owner domains,
and widens unknown or ambiguous inputs to a visible full-owner fallback. The
result is non-executable.

### Revision-bound pull-request validation

An owner supplies an atomic base, head, and tested revision triple. Ferris uses
only bounded local Git observation to resolve commits, find exactly one merge
base, derive committed paths, inspect the tested tree, and confirm that the
current checkout still matches the tested revision. It emits a separate binding
over the resolved revisions, normalized change set, working-tree observation,
and validation-plan identity.

Ferris does not fetch, check out, mutate the repository, or attest that dirty
working-tree contents were executed.

### Approved local execution

An owner prepares and approves an Action Plan containing exact owner commands
and bounded inputs. `ferris go` validates that authority, launches only the
declared repository-local work, bounds output, cleans up process trees, and
emits deterministic execution evidence. `ferris verify` checks receipt
integrity. Ferris does not generate owner commands or turn a successful receipt
into release approval.

### Artifact qualification

`ferris artifacts` compares an explicit producer and consumer envelope, keeps
every mismatch visible, and requires complete expected fan-in. Measured mode
streams bounded local artifact and manifest files into SHA-256 identities.
`--require-compatible` can fail closed on an incompatible qualification while
still emitting the complete report.

### Federated application planning

Ferris can collate independent Cargo workspace plans or propagate validation
through explicit consumer-owned application relationships. Each Cargo workspace
retains independent resolution and owner-local work. Revision-skew reporting is
a separate read-only view; it does not establish semantic compatibility.

## Adoption evidence

| Adopter | What it exercised | Result and boundary |
|---|---|---|
| PARLOR | Cargo package/reverse-consumer selection, owner-native build/lint/test execution, failure and recovery, receipt verification, and complete removal | Local Windows and hosted Ubuntu proof passed. PARLOR saturated the useful single-repository execution surface; no universal or savings claim followed. |
| [ICELINES](../research/2026-08-30-ferris-icelines-artifact-qualification.md) | Artifact and manifest qualification across Linux, macOS, and Windows, with independent owner verification and a rejected tamper control | Cross-platform qualification passed. Ferris did not own artifact production, transport, storage, extraction, or release acceptance. |
| [BISECT](../research/2026-08-30-ferris-go-bisect-telemetry.md) | Polyglot owner-domain selection, exact base/head/tested revision evidence, repository-owned npm execution, and deletion of a duplicate changed-path parser | PR #44 retained all existing workflows. Linux and Windows matched plan, binding, and change-set identities; a 2,143-path, 2.04 MB plan exercised output beyond Node's former default buffer within the explicit 32 MiB adapter bound. No CI-equivalence or realized-savings claim followed. |
| RUNE | A materially different Cargo topology and consumer-pinned validation contract | Windows and Ubuntu proof passed. The pin stabilizes only its declared experimental contract, not the full Ferris API. |

The adopter records are evidence for bounded behavior, not a support promise.
Ferris remains an incubation product and no adopter is required to replace
existing CI.

## Claim boundaries

Ferris does not currently:

- replace Cargo, rustc, linkers, test runners, workflow providers, or repository
  policy;
- parse workflow files or infer commands from languages, frameworks, or paths;
- infer undeclared cross-repository relationships;
- fetch revisions, mutate checkouts, or treat revision binding as execution
  attestation;
- provide hermetic or remote execution, a live scheduler, or a remote cache;
- own credentials, artifact transport, signing, publication, or deployment;
- automatically delete, narrow, or replace required CI gates; or
- claim realized time savings, avoided cost, full-suite equivalence, or
  production support.

## Near-term strategy

1. Make installation, Action Plan preparation, and repository-owned adapters
   easier without weakening explicit approval or owner command authority.
2. Add materially different adopters and failure controls before generalizing
   support or performance claims.
3. Define compatibility, versioning, and support policy for the smallest useful
   public record set.
4. Improve Windows checkout portability for the retained deep fixture corpus.
5. Keep all existing owner workflows until a separate, repository-specific
   reconciliation proves that narrowing or deletion preserves required checks.

Historical plans, waves, and simulations remain evidence of how individual
capabilities were authorized and tested. This document is the authoritative
summary of the current product; historical status language does not override it.
