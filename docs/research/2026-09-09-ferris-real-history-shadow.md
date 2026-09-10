# Ferris Real-History Validation Shadow

Status: completed bounded local evaluation
Evidence ID: FERRIS-EVIDENCE-001

## Question

Can Ferris use committed history to select materially less Rust package work
without missing an owner-native full-reference failure or costing more time than
the full reference?

This evaluation used Ferris
`e66795f4292a9e23a95fffc13a17bece8f5fff61`, the merge of PR #26. It did not
change either adopter, replace any owner workflow, or treat package selection
as command authorization. The separately authorized
[Pulse 01](../../context/waves/2026-09-09-real-history-validation-shadow/pulses/pulse-01.md)
bounded the cohort, local owner execution, retained evidence, and non-claims.

## Cohort

The fixed cohort contains 40 first-parent revisions:

- 12 from BISECT `origin/main`, ending at
  `e0764d2aa2dc25b52f6a40c47b0787766b18e749`; and
- 28 from ICELINES `origin/master`, ending at
  `935136020140bd5b408d26cbb0777dd6f0fb5ef9`.

Root commits without a parent comparison were excluded. Every included
revision was checked out detached in a clean worktree. Ferris received the
first parent as `--base-revision` and the revision itself as both
`--head-revision` and `--tested-revision`.

## Reproduction contract

Ferris was built with:

```text
cargo build --locked -p ferris-cli --bin ferris
```

Each cohort row used this command template from the clean adopter worktree:

```text
ferris validation-plan
  --workspace-id <workspace-id>
  --manifest-path <clean-worktree>/Cargo.toml
  --base-revision <first-parent>
  --head-revision <revision>
  --tested-revision <revision>
  --format json
```

| Repository | Workspace ID | Sample ref | Owner domains |
|---|---|---|---|
| BISECT | `shadow/bisect` | `origin/main` | none |
| ICELINES | `shadow/icelines` | `origin/master` | none |

The local environment was Windows `10.0.26310` on `x86_64`, Git
`2.55.0.windows.3`, Ferris `0.1.0`, rustc
`1.95.0 (ed80dadd6a 2026-06-18) (1.95.0-ms-20260618.5+ed80dadd6a)`, and Cargo
`1.95.0 (1.95.0-ms-20260618.5+ed80dadd6a)`. Ferris used the Cargo executable
available on `PATH`; no owner-domain file or explicit changed path was supplied.
The complete revision, parent, workspace-specific plan identity, and change-set
identity needed to instantiate the template are retained in the cohort CSV.

The machine-readable cohort is
[`2026-09-09-ferris-real-history-shadow.csv`](evidence/2026-09-09-ferris-real-history-shadow.csv).
It contains repository, revision, parent, date, subject, result, planning time,
both observed plan and change-set identity pairs, their equality result, input
count, package counts, and fallback status.

The bounded execution ledger is
[`2026-09-09-ferris-real-history-execution.csv`](evidence/2026-09-09-ferris-real-history-execution.csv).
It records every selected/full lane, exact owner command sequence, command exit
codes, required non-secret environment, aggregate duration, and classified
outcome. Neither evidence file contains source, raw command output, local paths,
or credentials.

## Planning results

| Measure | Result |
|---|---:|
| Successful revision-bound plans | 40 / 40 |
| Repeated plan/change-set identity mismatches | 0 / 40 |
| Full-reference fallbacks | 37 / 40 (92.5%) |
| Narrowed revisions | 3 / 40 (7.5%) |
| Full-reference package-units | 672 |
| Safe effective package-units | 610 |
| Theoretical package-unit reduction | 62 / 672 (9.2%) |
| Planning median | 2,374.5 ms |
| Planning p95 | 5,049 ms |
| Planning range | 1,868-5,505 ms |

A package-unit is one package selected for one revision. It is a scope measure,
not CPU time, compiler work, test count, cost, or realized savings. A fallback
counts as the full package set even when Ferris also reports a narrower Cargo
closure. Planning timings are one local pass and are reported as operational
overhead, not as a benchmark.

The three non-fallback plans were:

| Repository | Revision | Selected/full packages | Owner subject |
|---|---|---:|---|
| BISECT | `d9550b09` | 2 / 39 | Prevent Windows CLI startup stack overflow |
| BISECT | `453ca260` | 20 / 39 | Restore warning-clean workspace checks |
| ICELINES | `0be0c480` | 1 / 7 | Allow fantasy leagues to use installed schemes |

The high fallback rate is correct under the current contract. Repository-root,
workflow, lockfile, documentation, and other unmapped inputs cannot safely
narrow package validation without owner declarations.

## Selected-versus-full execution

Only owner commands present at each historical revision were eligible.
Execution used separate clean Cargo target directories for selected and full
lanes.

### ICELINES `0be0c480`

Ferris selected `icelines-cli`. The selected lane ran the repository's seven
CI commands for that package. The full reference ran the documented
`cargo test --workspace`.

| Lane | Result | Wall clock |
|---|---:|---:|
| Selected owner package matrix | passed | 1,297,708 ms (21.63 min) |
| Owner full workspace test | passed | 1,204,558 ms (20.08 min) |

This case has no observed selected-pass/full-fail false negative, but the
selected lane was 93,150 ms (7.7%) slower. Multiple package-specific Cargo
invocations outweighed the smaller package scope. Because the selected lane
ran first and both lanes shared the machine-level Cargo registry/cache, the
timing is directional rather than a promotion benchmark. It still rejects a
claim that package narrowing alone provides value.

### BISECT `d9550b09` and `453ca260`

Neither revision produced admissible timing evidence under the current Rust
toolchain:

- `d9550b09` selected execution failed while compiling `bisect-runner` because
  historical unused variables are denied as warnings, while its full-reference
  lane stopped in `bisect-core` because a historical test name violates
  `non_snake_case` under denied warnings; and
- `453ca260` selected packages mapped to the complete retained owner command
  set, so selected and full were one `selected-equals-full` lane. That lane
  stopped on the same `bisect-core` `non_snake_case` diagnostic for both roles.

Only `d9550b09` produced different selected and full diagnostics. Both BISECT
cases are recorded as historical toolchain/source incompatibility, not as
Ferris false negatives, prevented iterations, or performance results.

## Decision

This cohort supports three bounded conclusions:

1. validation-plan and committed change-set identities are deterministic across
   these 40 real revisions; full revision-binding identity was not retained;
2. current fail-closed mapping avoids broad under-selection chiefly by falling
   back, not by demonstrating broad precision; and
3. affected package count is not a useful savings proxy when owner commands
   fragment Cargo execution.

The result does **not** promote affected-only execution. The observed
false-negative denominator is one executable narrowed selected/full pair, which
is too small, and that pair had negative wall-clock value.

## Next gate

Do not add more selector machinery first. The next evaluation should:

1. add owner declarations for repository-root policy, workflow, lockfile,
   generated, and documentation inputs in one adopter;
2. define one owner-native selected command that preserves the full command's
   compilation shape instead of splitting it into many Cargo invocations;
3. freeze a larger executable cohort before inspecting outcomes;
4. run selected and full lanes in alternating order with cold and warm
   repetitions; and
5. require zero selected-pass/full-fail cases before considering any advisory
   workflow narrowing.

Existing owner workflows remain authoritative and unchanged.

The completed technical and wave-control role dispositions, exact revision
closure, remaining gates, and exhausted implementation authority are recorded in the
[post-evaluation role review](../plans/reviews/FERRIS_REAL_HISTORY_VALIDATION_SHADOW_ROLE_REVIEW.md).
