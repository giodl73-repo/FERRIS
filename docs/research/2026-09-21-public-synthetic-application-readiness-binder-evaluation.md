# Public Synthetic Application Readiness Binder Evaluation

Date: 2026-09-21
Status: Complete
Decision: Accept the corrected binder at the 16-workspace boundary; stop before
owner adoption or execution preparation

## Product outcome and scope

This bounded evaluation applied `bind-application-readiness` and
`doctor --application-readiness` to the public
`giodl73-repo/ferris-synthetic-federated` corpus at revision
`24405310548b9bc2f3232d2d5e60e13b923d470c`. The corpus contains 16 independent
Cargo workspaces and 128 packages, the V1 application maximum.

The evaluation reused the checked-in `ferris.application/v0` definition and
created 16 temporary evaluator-owned requirements declarations. Each declared
only Cargo visibility, the workspace `Cargo.toml`, and Windows/Linux x86_64
support. Ferris did not discover requirements, invoke Cargo, or run the
repository's validation commands.

The budget allowed one public corpus, Windows and Linux runs, one corrective
successor for defects exposed by that run, and evidence closeout. It excluded
owner adoption, a new schema version, requirements inference, Action Plan
preparation, approval, and execution.

## Evaluated source

| Input | Identity |
|---|---|
| Consumer | `giodl73-repo/ferris-synthetic-federated@24405310548b9bc2f3232d2d5e60e13b923d470c` |
| Ferris base | `fd16d11d736d686cba94ad2171063994d4b4e72f` |
| Ferris built-source diff Git blob | `769e2f9554118e34e1fd42fabb056277ee82b062` |
| Windows binary SHA-256 | `e512eb385e6d0968f007a680815343dc6384f537af3d6fa69802e7abde131c9d` |
| Linux binary SHA-256 | `9fd7ffb3cfc040850edd907c0c21ebdbdcf3d69955ff6a7036e7e1abcbba6467` |

Windows used PowerShell 7.6.6, Windows NT 10.0.26310.0, and Microsoft Rust
1.95.0. Linux used WSL2 kernel 6.18.33.2, x86_64 GNU/Linux, and upstream Rust
1.95.0. The different binary hashes are expected platform artifacts, not
portable Ferris identities.

## Onboarding surface

| Explicit item | Count |
|---|---:|
| Existing Application Definition | 1 |
| Temporary requirements declarations | 16 |
| Workspace-to-requirements associations | 16 |
| Explicit output path | 1 |
| Successful/control Ferris commands per platform | 4 |

The four commands were two independent binds, one Cargo-hidden readiness
observation, and one Cargo-visible observation. The binder removed digest and
cross-record copying but intentionally retained all 16 owner associations.

## Compatibility corrections

The first Linux build exposed a non-Windows compile defect: the absent
`PATHEXT` branch initialized `None` without a concrete `Option` type. An
explicit `Option<OsString>` annotation restored the intended no-`PATHEXT`
behavior without changing runtime semantics.

The first valid-input attempt exposed a product incompatibility before output
creation. The existing Application Definition uses hierarchical IDs such as
`synthetic/federated/ws-00`, while READINESS-001 accepted exactly two lowercase
segments. Requirements IDs must equal Application Definition IDs, so no valid
request could represent the unchanged consumer.

The corrective change broadened the existing V1 identity grammar from exactly
two to two-or-more lowercase slash-separated segments. The 128-byte total and
64-byte segment bounds remain. Existing records remain valid, malformed empty
segments remain invalid, and no ID translation was added. A binder regression
now proves that three-segment application and workspace IDs bind and compose.

One earlier harness attempt also resolved relative requirements paths from the
wrong working directory. Ferris returned `invalid`, exit 2, and created no
request. The successful commands used explicit absolute input paths; generated
records retained only application-relative paths.

## Results

| Check | Windows x86_64 | Linux x86_64 |
|---|---|---|
| Bind 1 | exit 0 | exit 0, 29 ms |
| Bind 2 | exit 0 | exit 0, 35 ms |
| Repeated request equality | Byte-identical | Byte-identical |
| Cross-platform request equality | `sha256:585e9c36959f56fa78127a4ec523a6ca6a170e5bb2f4f721020d6bf28eb952b5` | Same bytes |
| Request size | 4,818 bytes | 4,818 bytes |
| Cargo hidden | `blocked`, exit 7 | `blocked`, exit 7 |
| Cargo visible | `ready`, exit 0 | `ready`, exit 0 |
| Tracked consumer diff | None | None |
| Final consumer status | Clean | Disposable clone removed |

The Linux timings are local observations only. Windows timing was not retained
after the first harness comparison used an unavailable PowerShell byte-array
method. No performance or savings conclusion follows.

## Findings

### FERRIS-807: The non-Windows readiness branch did not compile

The Windows-only `PATHEXT` value left an untyped `None` on other targets. The
Linux compiler rejected it before evaluation. The explicit standard-library
type annotation fixes compilation without changing executable observation.

### FERRIS-808: V1 identity grammars prevented composition of an existing Ferris application

The federated planner accepted the corpus's hierarchical owner IDs, but the
readiness schemas rejected them. This was a Ferris contract integration defect,
not an adopter data defect. The additive grammar correction preserves owner IDs
and avoids a migration or translation layer.

### FERRIS-809: The binder is deterministic at the 16-workspace platform boundary

Two binds on each platform produced identical request bytes. Windows and Linux
also produced the same 4,818-byte request and SHA-256. The output contains only
portable relative paths and exact owner-file digests.

### FERRIS-810: Readiness catches a missing shared prerequisite without owner work

Hiding Cargo from the Ferris process blocked the application on both platforms;
restoring process-local Cargo visibility made it ready. No Cargo or repository
command ran, so the result establishes declared prerequisite visibility only.

### FERRIS-811: Evaluation remained removable but did not prove owner adoption

The tracked consumer tree never changed. Windows returned to a clean status
after removing 18 temporary files, and the Linux disposable clone and outputs
were removed. The 16 declarations were evaluator-authored, so maintenance,
review, support, and long-term owner acceptance remain unmeasured.

## Reproduction outline

1. Build `ferris-cli` for the target platform.
2. Use the exact consumer revision above.
3. Create one strict requirements declaration per existing workspace ID.
4. Pass all 16 explicit `WORKSPACE_ID=PATH` associations to the binder twice.
5. Compare request bytes and SHA-256.
6. Run application readiness with Cargo hidden and visible only to Ferris.
7. Verify `git diff --quiet`, remove temporary records, and verify clean status.

Focused product verification used:

```text
cargo test -p ferris-core application_readiness
cargo test -p ferris-cli --test cli application_readiness
CARGO_TARGET_DIR=/tmp/ferris-app-readiness-target cargo test -p ferris-core binder_accepts_hierarchical_application_workspace_ids
git diff --check
```

The focused runs passed 15 Windows core tests, five Windows CLI tests, and the
Linux hierarchical-ID regression. The complete Linux core unit binary passed
119 tests with two helper tests ignored. Three later Linux platform-profile
integration tests could not run because their Rust targets were not installed.
The complete Windows core run retained three known failures because Microsoft
Cargo reports `1.95.0-ms-20260618.5+ed80dadd6a`, which the existing doctor
parser classifies as unsupported. Neither limitation intersects binder logic.

## Limits and next decision

- Synthetic evidence does not establish natural enterprise adoption.
- Temporary declarations do not establish owner willingness to maintain them.
- `ready` does not establish compilation, tests, command success, capacity,
  credentials, services, or application-root readiness.
- WSL2 Linux is native Linux execution but not a hosted CI or support matrix.
- No support, production, performance, savings, or CI-replacement claim follows.

The binder evaluation is complete. The next product decision should use an
owner-maintained requirements declaration and owner-declared command source to
assess whether explicit, non-executable Action Plan preparation is justified.
That work must remain separate from readiness observation, approval, and
execution.

## Review

The
[public synthetic binder role review](../plans/reviews/FERRIS_PUBLIC_SYNTHETIC_APPLICATION_READINESS_BINDER_ROLE_REVIEW.md)
accepts the corrected bounded evidence and stops this pulse.
