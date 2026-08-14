# Pulse 22 Process-Exit Diagnostic Replication Nine-Role Review

Date: 2026-08-14
Disposition: Bounded program executed; invalid with no category conclusion
Implementation authority: Public contract, machine-readable controls,
test-only validation, independent custody execution, and public-safe closeout
only

## Review question

May an independent custodian construct and execute one fresh public-rule-based
diagnostic search for `process-exit-agreement`, with a precommitted
`sanitized-reproducer` tier, without accessing Pulse 17, changing product
behavior, creating certification evidence, or changing PLATFORM-001?

## Role dispositions

| Role | Disposition | Boundary |
|---|---|---|
| Rust Safety Steward | Approve | Public governance and safe test-only validation create no safety, soundness, or correctness claim; no production Rust is authorized |
| Compiler Performance Engineer | Approve | The 512-case/platform, 1,024-process, timeout, stream, and minimization limits are custody bounds, not benchmarks |
| Interop Boundary Auditor | Approve | The only observed boundary is the existing command envelope, process exit, and stream route; no ABI, WIT, native-library, or owner boundary changes |
| AI Assurance Skeptic | Approve | The independent public-only oracle is frozen before generation, the private seed is precommitted, the first mismatch is preserved, and Pulse 17 remains inaccessible and immutable |
| Ecosystem Strategist | Approve | The program adds no resolver, registry, support policy, owner-system mutation, or alternative source of Cargo truth |
| Rust Maintainer | Approve | One removable contract validator and closed public declaration are reviewable; no CLI, output, dependency, visibility, or API change is approved |
| Native Platform Adopter | Approve | Direct Windows and Ubuntu 24.04 WSL2 launches are mandatory; WSL evidence is diagnostic only and is not native Linux support |
| Scope Keeper | Approve | Authority ends at a sanitized reproducer, bounded `no-reproduction`, or an explicit incomplete disposition; no fix, certification, hidden access, or PLATFORM-001 advancement |
| Validation Checker | Approve | Exact class, metadata, JSON, path, size, change-count, ordering, platform, process, minimization, lineage, disposition, and zero-overlap controls are machine-testable |

## Shared conditions

All nine roles require:

- no actual case construction or selection before custody;
- exactly one independently seeded search execution;
- maximum 512 unique cases per platform and 1,024 search launches;
- zero candidate retries and stop after the first mismatching cross-platform
  case pair;
- a separate deterministic minimization phase with at most 128
  transformations;
- publication only under the precommitted `sanitized-reproducer` tier;
- permanent certification ineligibility for the search package and public
  reproducer; and
- byte-for-byte preservation of the Pulse 17 public result and permanent
  quarantine of its fixture.

## Execution evidence

- immutable cutoff: `94890e53631d9110128bb420bf0cbbb074187e7c`;
- independently generated and selected cases: 188;
- retained search processes: 1 Windows, 0 Ubuntu;
- completed cross-platform pairs: 0;
- candidate retries: 0;
- minimization transformations and processes: 0;
- disposition: `invalid`; and
- blocker: `collector-durability-fsync-invalid-descriptor`.

The collector failure occurred before the required Ubuntu partner launch.
The atomic pair did not complete, so the run supplies no target-category
conclusion. The search cannot resume, retry, or be converted into
`no-reproduction`.

## Decision

All nine roles preserve the independent result as `invalid`. No role treats
the retained Windows process as a completed pair, infers a target-category
outcome, authorizes a retry, or converts the run into `no-reproduction`. No
role approves product behavior change, hidden material access, Pulse 17
activity, scoring, certification, support, or PLATFORM-001 advancement.
