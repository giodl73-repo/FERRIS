# Committed Failure Response Adoption

Date: 2026-09-22

Status: Complete controlled synthetic adopter evaluation

## Question

Can a retained Rust repository own and reuse Ferris failure policy, entrypoint,
and lane declarations across real Cargo dependency, lockfile, offline-policy,
and unclassified failures without granting approval or executing a plan?

## Control Record

- Product outcome: replace evaluator-authored preparation inputs with one
  committed repository-owned failure-response workflow.
- Consumer: `giodl73-repo/ferris-synthetic-chain` at
  `fcb022f6d953fd761d663102e431054a9eb88066`.
- Ferris: `0808037f` with Cargo and rustc 1.95.0 on Windows.
- Maximum effort: one adopter repository, four real Cargo failures, one
  committed policy and lane set, and one owner binder.
- Completion: all four classifications select the declared response, only
  `prepare_action` creates an unsigned plan, the owner oracle passes, and all
  disposable state is removed.
- Abandonment: stop on command inference, approval or execution, source-repo
  mutation outside the committed integration, or a required schema change.
- Product Value Governor disposition: `continue-within-budget`.

## Committed Integration

The adopter now owns:

- `.ferris/failure-policy.json`, with explicit dependency and lockfile routes,
  offline cache preparation, and a halt fallback;
- `.ferris/action-plan-lanes.json`, preserving the authoritative
  `cargo test --workspace` command as one required owner gate; and
- `tools/prepare-ferris.ps1`, which takes explicit Ferris and Cargo paths,
  stages Cargo, binds file and revision identities, and invokes only passive
  diagnosis, policy selection, and unsigned preparation.

Generated entrypoints, decisions, and plans are ignored under
`.ferris/runtime/`. The script creates no approval, invokes no Action Plan, and
removes an older failure plan before reporting a non-action disposition.

## Real Cargo Matrix

Each case ran in a disposable archive of the committed adopter revision. Raw
stderr remained only in the temporary evaluation directory and was removed.

| Case | Construction | Cargo exit | Stderr bytes | Ferris classification | Owner disposition | Plan |
|---|---|---:|---:|---|---|---|
| Dependency | Missing local path dependency | 101 | 461 | `dependency` | `route` to `owner.dependencies.review` | none |
| Lockfile | New direct workspace dependency with `cargo check --locked --offline` | 101 | 251 | `lockfile` | `route` to `owner.lockfile.review` | none |
| Offline policy | Unavailable registry dependency with `cargo check --offline` | 101 | 400 | `offline_policy` | `prepare_action` for `owner.cargo-cache.populate` | unsigned V1 |
| Unclassified | Invalid Rust source with `cargo check --workspace --offline` | 101 | 326 | `unclassified` | `halt` at `owner.failure.manual-review` | none |

The offline result prepared action plan
`sha256:4c074e82082c8a21d7cf846681e025b987134f088761aa29c4fd7e75cdb811f0`
with exact argv `fetch --locked`, an empty `approval_id`, and no receipt. The
committed lane policy independently prepared unsigned validation plan
`sha256:c9d5ea0b588858953216e5b9b05717359b5ad4cf10eb9e1547338ec53370784f`.

`cargo test --workspace` passed unchanged: four unit tests and two doc-test
targets passed. The source checkout remained clean and both the runtime and
disposable evaluation directories were removed.

## Corrections and Burden

Two initial declaration attempts exposed strict but actionable validation: V1
accepts only credential class `none`, and inherited environment names must be
sorted. After explicit continuation approval, the wrapper was corrected to
read the direct Action Plan output shape and to remove stale generated plans.

The owner policy is 34 lines and the lane set is 17 lines. Producing a valid
revision- and content-bound declaration required a 197-line, 7,139-byte owner
script, including canonical JSON identity construction and executable staging.
The rules are reusable; declaration binding is the dominant onboarding cost.

## Decision

This closes the controlled committed `ferris.action-plan-lanes/v1` adoption
gate, but it is synthetic Windows evidence rather than production adoption or
cross-platform support.

The next product candidate is a bounded owner-entrypoint binding command over
explicit command intent and an explicit executable path. It should remove
custom identity assembly while preserving exact owner argv, content binding,
current-revision checks, and the existing no-approval/no-execution boundary.
Do not infer commands, resolve PATH implicitly, or change Action Plan V1 based
on this result.
