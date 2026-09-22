# PATH Executable Binding Boundary

Date: 2026-09-21

Status: completed bounded architecture evaluation; no implementation

## Question

Can an explicit V1 environment-readiness executable requirement safely bind a
`PATH`-resolved owner tool into a prepared Action Plan without staging the
tool, retaining a machine path, weakening content identity, or changing V1
execution semantics?

## Controls

- Product outcome: remove repository-local executable staging for ordinary
  owner commands such as `cargo test --workspace`.
- Maximum effort: inspect one executable readiness requirement, one owner
  entrypoint, and the public synthetic preparation result; do not execute or
  approve work.
- Completion test: identify a deterministic binding that preserves readiness
  privacy and the existing Action Plan identity guarantees.
- Abandonment condition: stop if composition requires a new schema layer,
  retained absolute paths, trust in mutable `PATH`, executable staging, or a
  change to V1 semantics.

## Contract Comparison

| Property | Environment readiness V1 | Action Plan execution V1 |
| --- | --- | --- |
| Declared executable | Leaf name | Repository-relative path |
| Resolution | Current process `PATH` and, on Windows, `PATHEXT` | Canonical path below the repository root |
| Retained resolved path | Prohibited | Required operationally by the repository-relative declaration |
| Content identity | Not observed or retained | Executable must be a content-bound command file |
| Purpose | Passive presence observation | Exact process authorization and launch |
| Drift handling | A later observation may differ | Drift blocks before launch |

Readiness emits only a status and evidence-retention assertions. A satisfied
observation does not identify which candidate won search-path resolution, does
not bind its bytes, and cannot prove that a later resolution selects the same
file. The report intentionally excludes enough information that an Action Plan
preparer cannot reconstruct an executable identity from it.

Execution cannot consume the leaf name directly. V1 validates the executable
as a portable repository-relative path, requires that path in the command file
bindings, verifies its digest, and launches the canonical repository-local
file. Reinterpreting the field as a search-path name would change the meaning
of an existing schema and introduce a resolution race between preparation and
launch.

## Decision

The contracts do not compose into a safe V1 executable binding. A readiness
report may establish that an owner-declared tool was present when observed; it
is not execution authority and MUST NOT be used as executable identity.

Do not:

- copy or stage the resolved executable implicitly;
- persist an absolute path in a readiness report;
- treat `satisfied` as proof of executable content;
- resolve a mutable `PATH` leaf at launch without a content binding; or
- reinterpret `ferris.action-plan/v1` command fields.

A future solution would require a separately authorized execution-version
contract for platform-local tools. At minimum it would need an owner-declared
leaf name, platform-specific resolution rules, executable content identity,
pre-launch re-resolution and drift rejection, path-disclosure policy, and
Windows launcher/script semantics. That is a new architecture layer, not a
readiness extension or preparation convenience.

## Product Finding

No code change is justified in this pulse. The public synthetic evaluation
already proves the preparation behavior and isolates staging as the adoption
gap. The next useful evidence remains an adopter-maintained readiness and
owner-entrypoint declaration pair. It can establish whether this compatibility
gap blocks real adoption before Ferris commits to a new execution schema.

## Inspection

The boundary is visible in:

- `crates/ferris-core/src/readiness.rs`: executable observation returns only a
  status and always records `resolved_path_retained: false`;
- `crates/ferris-core/src/execution.rs`: preparation and launch canonicalize
  the executable below the repository root and verify its file binding;
- `docs/specs/FERRIS_ENVIRONMENT_READINESS_CONTRACT.md`: resolved candidate
  paths and metadata are not retained; and
- `docs/specs/FERRIS_ACTION_PLAN_EXECUTION_CONTRACT.md`: V1 explicitly excludes
  PATH-resolved executables.
