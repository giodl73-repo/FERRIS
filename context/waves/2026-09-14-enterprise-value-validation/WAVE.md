# Wave: Enterprise Value Validation

Status: Active; Pulse 01 complete
Implementation authority: None pending Pulse 02 decision

## Systems-development gap

Ferris has strong bounded evidence for deterministic planning, conservative
fallback, approved owner-command execution, receipt verification, and complete
removal in selected adopters. It does not yet have:

- a repeatable fresh-enterprise onboarding proof;
- measured positive affected-only value with zero correctness divergence;
- evidence sufficient to narrow or replace a required CI gate; or
- a bounded compatibility and support policy for production adoption.

Those gaps are ordered. Ferris MUST NOT claim savings, CI replacement, or
support readiness before repeatable onboarding and owner correctness are
observed.

## Measurable decision

Determine whether Ferris can progress through four gates under strict evidence:

1. two private enterprise consumers complete cross-platform onboarding and
   removal without owner-file changes;
2. one real adopter demonstrates repeatable positive selected-versus-full value
   with zero selected-pass/full-fail divergence;
3. one owner approves an advisory CI narrowing shadow without deleting required
   checks; and
4. the smallest exercised record and platform set receives explicit
   compatibility, installation, rollback, and support boundaries.

Failure at a gate blocks later gates. A broad commercial or savings claim
requires all applicable gates and separate owner approval.

## Research basis

- The
  [`current strategy`](../../../docs/plans/FERRIS_CURRENT_STRATEGY_AND_FEATURES.md)
  identifies onboarding, owner declaration coverage, compatibility, support,
  and retained workflows as the near-term priorities.
- The
  [`real-history shadow`](../../../docs/research/2026-09-09-ferris-real-history-shadow.md)
  found 37 of 40 revisions widened to full validation and the only admissible
  narrowed comparison was slower. It rejects package count as a savings proxy.
- The
  [`enterprise corpus qualification`](../../../docs/research/2026-09-10-post-portability-private-enterprise-corpus-qualification.md)
  proves deterministic Windows/Ubuntu behavior for five synthetic families but
  explicitly does not establish production value or savings.
- The
  [`enterprise onboarding Pulse 05`](../../../docs/research/2026-09-13-private-enterprise-onboarding-pulse-05.md)
  proves Windows owner correctness and shows that mounted repositories beneath
  a user-profile `.cargo` ancestor are not a neutral WSL environment.

## Candidate consumers

- `EO-01` and `EO-02`: custody-bound private enterprise consumers for the
  repeatable onboarding gate;
- BISECT: candidate real adopter for owner-domain and revision-bound value;
- PARLOR: candidate controlled-execution and removal reference; and
- ICELINES or RUNE: independent topology or artifact controls if required by a
  later separately authorized pulse.

Exact private identities, revisions, paths, raw output, and identifiable
timings remain in custody.

## Pulse sequence

| Pulse | Gate | Status | Decision |
|---:|---|---|---|
| 01 | Native-Linux enterprise baseline | Complete | Exact native clones and unchanged owner commands passed on Windows and Linux; cleanup complete |
| 02 | Repeatable onboarding and removal | Proposed | Prepare pinned Ferris inputs, execute and verify on both platforms, remove all Ferris material, and rerun owner commands |
| 03 | Real-adopter value cohort | Proposed | Freeze a representative cohort and compare owner-shaped selected and full lanes with alternating cold/warm order |
| 04 | Advisory CI reconciliation | Proposed | Shadow one owner-approved narrowing while retaining required checks and measuring divergence |
| 05 | Compatibility and support boundary | Proposed | Define only the exercised installation, schema, platform, rollback, and support surface |

Only Pulse 01 is authorized.

## Pulse 01 result

Both custody source checkouts began and ended clean. Temporary native WSL clones
matched each source `HEAD` and tracked tree, used only local origins, and ran
outside the Windows-profile Cargo configuration hierarchy. Both unchanged owner
commands passed on Windows and native Linux with Rust 1.95. All temporary
clones, build outputs, PowerShell files, symlink, and archive were removed.

The WSL timing wrapper produced invalid arithmetic and those timings were
discarded. No performance claim uses them. Pulse 01 establishes repeatable
cross-platform owner baselines only; Ferris onboarding remains Pulse 02.

## Shared boundaries

- Cargo owns package resolution, workspace membership, lock state, units,
  freshness, and compilation.
- Repository owners own commands, environments, credentials, required checks,
  success semantics, publication, and rollback.
- Ferris MUST NOT parse workflow files, infer owner commands, hide fallback,
  weaken owner checks, or treat scope reduction as time or cost savings.
- Typebook and RUNE remain product-neutral and independently usable.
- Observation, planning, approval, execution, and commercial claims remain
  separate records and authorities.

## Reviewers

Required reviewers are Rust Safety Steward, Compiler Performance Engineer,
Interop Boundary Auditor, AI Assurance Skeptic, Ecosystem Strategist, Rust
Maintainer, Native Platform Adopter, Scope Keeper, Validation Checker, Product
Value Governor, and Autonomy Supervisor.

## Non-goals

- automatic workflow deletion or required-check replacement;
- production support or service-level commitments;
- remote execution, cache, signing, publication, or deployment;
- AI command inference or autonomous approval;
- product or schema implementation in Pulse 01; and
- realized savings or commercial claims before the applicable later gate.
