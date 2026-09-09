# Ferris Go BISECT Telemetry Audit

Date: 2026-08-30

Status: accepted historical audit; owner-domain and revision-bound migration
subsequently proven

## Frame

BISECT already owns four GitHub Actions workflows, a 39-member Cargo workspace,
a six-cell Python version/OS matrix, two browser E2E jobs, two Rust engine
families, shared-kernel checks, and formal verification. Those owner tests and
their correctness policy remain authoritative.

The missing shared capability is not another test runner. It is an
owner-declared polyglot selection boundary that can compose Cargo reverse
dependencies with Python and web entrypoints, emit reviewable evidence, and
leave unknown paths on a visible conservative fallback.

The current workaround is to run nearly the entire portfolio for every pull
request. This audit is falsified as a useful Ferris opportunity if non-Rust
ownership cannot be declared without copying pipeline logic, if selected lanes
miss a historically failing test, or if setup and shadow cost consume the
candidate savings.

## Audit

The sample is the ten most recent pull-request runs for each active `CI`,
`Pipeline Tests`, and `Formal Verification` workflow retrieved through the
GitHub Actions API on 2026-08-30. Job-minutes are summed elapsed job duration,
not GitHub billing minutes.

| Workflow | Runs | Failures | Median wall time | Median job-minutes | Total job-minutes |
| --- | ---: | ---: | ---: | ---: | ---: |
| `CI` | 10 | 0 | 5.3 min | 12.2 min | 129.2 min |
| `Pipeline Tests` | 10 | 3 | 12.4 min | 64.6 min | 612.1 min |
| `Formal Verification` | 10 | 0 | 0.7 min | 0.6 min | 5.9 min |
| **Combined** | **30 workflow runs** | **3** | — | **77.3 min per PR revision** | **747.2 min** |

`Pipeline Tests` accounts for 81.9% of observed PR job-minutes. Its six Python
matrix cells account for 538.4 minutes; E2E accounts for 43.4 minutes; and its
Ubuntu default-engine Rust job accounts for 30.6 minutes. That Rust job repeats
the same seven package test commands already exercised by the Ubuntu
`c-ffi` job in `CI`, although setup and cache configuration differ.

Three consecutive failed revisions each failed
`test_retained_v3_package_verifies_portably` near 4% of the Python suite after
about 6.5 minutes. The failures were distinct retained-evidence mismatches:
summary, compiled-contract artifact hash, and analysis. Matrix fail-fast
cancelled remaining Python cells, so this sample does not establish that those
cells are safely cancellable or diagnostically redundant.

The five latest PRs, #39 through #43, changed only
`web/docs/package.json` and/or `web/docs/package-lock.json`. They consumed 387.0
job-minutes across the three workflows, with a median 77.5 job-minutes per
revision. None of the workflows runs `npm ci` or `npm run build`, so the sample
simultaneously shows broad unrelated execution and a missing affected-owner
check. Savings cannot be claimed by simply skipping the existing suites.

## Ferris projection

Ferris commit `4d9e68f571fe2a649e29625d4a0de519ea18d524` was run against the
current BISECT checkout:

| Input | Result | Plan identity |
| --- | --- | --- |
| `web/docs/package.json` | Safe `full_workspace_fallback` across all 39 Cargo packages because the path is outside a package-owned Rust anchor. | `validation-plan:f2aa57418a85a96f332d9a0f69e08e304b9a439b28d3f4551fbc4afdafd25d66` |
| `crates/bisect-core/src/lib.rs` | `owned_rust_path` with a 14-package reverse-dependency closure, including `bisect_py`. | `validation-plan:b424d059cf9a2a0f9eee938043f3d51741a96506910434c88c009821a0426c56` |

This is correct conservative behavior. It also proves that Cargo-only ownership
cannot capture BISECT's Python suite, browser tests, web application, generated
research fixtures, or formal checks. Ferris must not infer those relationships
from directory names.

## Compare

BISECT's `shared-kernels.yml` is the internal precedent to reuse: the owner
already scopes one workflow with explicit path filters and retains the exact
owner test command. The broad `CI` and `Pipeline Tests` workflows are the
negative precedent to avoid for unrelated web-only changes.

GitHub Actions supports native path filters and concurrency cancellation. Those
mechanisms remain the provider execution controls; Ferris should produce
selection evidence that an owner may translate into them, not become a GitHub
workflow parser or scheduler. Required-check behavior must be reconciled before
workflow-level path filtering because a skipped required workflow can remain
pending.

## Role review

| BISECT role | Finding | Disposition |
| --- | --- | --- |
| `BENCHMARK` | A web-only slice must add the missing web build before deleting unrelated tests; selection is invalid if it omits a historically failing retained-package check for a relevant change. | `pass-with-condition` |
| `SCALE` | Ten PR revisions establish an opportunity sample, not a population savings estimate. Report job-minutes, wall time, and coverage separately. | `pass-with-condition` |
| `TRENCH` | Unknown and cross-domain paths must widen visibly; cancellation needs owner-labelled actionability and an explicit test. | `pass` |
| `COVENANT` | Retained plans must bind source revision, owner declarations, commands, and provider observations; integrity is not provider authenticity. | `pass-with-condition` |

## Smallest next slice

1. Add an owner-native `web-docs` entrypoint that runs `npm ci` and
   `npm run build`; do not skip existing required workflows.
2. Add a strict owner declaration mapping `web/docs/**` to that entrypoint while
   preserving Cargo selection and full fallback for unknown or cross-domain
   changes.
3. Replay PRs #39-#43 and retain one selected plan, one owner execution receipt,
   and one forced unknown-path fallback.
4. Measure shadow overhead and candidate avoided job-minutes separately.
5. Only after owner review, remove the overlapping `Pipeline Tests/rust-tests`
   job or apply provider path filters; do not change both in the evidence slice.

This slice should stop if it requires parsing workflow YAML, embedding npm or
pytest semantics in Ferris, treating file globs as complete without owner
authority, or claiming the observed 387.0 job-minutes as realized savings.

## First shadow result

BISECT PR #44 implemented the owner-native half of this slice without changing
existing workflows. Current-head run `33318724606` passed `npm run build` in 51
seconds and retained:

- head revision `0186ba17ef1215ed234b4cde09bfbb6d5c22f9b7`;
- tested merge revision `864e0e91d258ffc96d262aa048b16266ac5bb506`;
- pinned Ferris revision
  `1aef069f00ace2567c9610c79820eb437cb4b3a3`;
- validation plan
  `validation-plan:f2aa57418a85a96f332d9a0f69e08e304b9a439b28d3f4551fbc4afdafd25d66`;
  and
- `full_workspace_fallback` with `fallback.required_by_inputs: true`.

The full unchanged BISECT PR matrix also passed. At this point the workflow
still used a representative existing path because that Ferris revision could
not classify deleted paths or select a declared non-Cargo owner.

## Native migration result

The subsequent migration completed in BISECT PR #44. Ferris now owns bounded
base/head/tested change derivation, BISECT deleted its duplicate general-purpose
path parser, and the repository retained only an independent `web/docs` oracle
plus owner-native `npm ci` and `npm run build`.

Hosted Linux run
[`33349671844`](https://github.com/giodl73-repo/BISECT/actions/runs/33349671844)
and exact Windows replay matched validation-plan, revision-binding, and
change-set identities. That historical range derived 2,143 paths and emitted a
2.04 MB plan, exercising output beyond Node's former default buffer within the
adapter's explicit 32 MiB bound.

Current-head shadow, CI, Pipeline Tests, and Formal Verification also passed,
with every existing workflow still enabled. Any workflow narrowing remains a
separate owner decision after historical and branch-policy reconciliation.
These results do not establish CI equivalence or realized savings.
