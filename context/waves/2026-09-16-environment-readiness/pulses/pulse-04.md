# Pulse 04: asdf Adapter Evaluation

Status: Complete

## User outcome

Determine whether Ferris can use an existing `.tool-versions` file to explain
local readiness without asking an owner to duplicate tool declarations or
letting Ferris guess asdf plugin behavior.

## Authority

The user's fresh `continue` after Pulse 03 authorizes one bounded evaluation of
asdf `.tool-versions` against the frozen READINESS-001 V1 contract:

- freeze the applicable asdf documentation and source revision;
- retain one exact public upstream fixture and bounded synthetic controls;
- add one dependency-free, test-only lexical evaluator;
- decide whether a lossless passive V1 adapter is possible; and
- record research, limitations, role dispositions, and the go/no-go result.

No Ferris product adapter, schema change, source discovery, asdf invocation,
plugin execution, installation, version resolution, home-directory lookup,
adopter mutation, support claim, or Pulse 05 is authorized.

## Maximum effort

One source-format evaluation, one small fixture family, one test-only harness,
one research note, and one eleven-role closeout.

## Completion test

- the evaluated asdf revision and primary sources are immutable and cited;
- fixtures cover ordinary versions, comments, multiple ordered versions,
  `system`, `path:`, and `ref:` forms;
- the evaluator matches the documented lexical shape without executing asdf;
- every semantic loss against READINESS-001 V1 is explicit;
- the decision does not infer executable names from plugin IDs;
- no product code, schema, dependency, adopter, or owner file changes;
- tests, links, fences, staged scope, and `git diff --check` pass; and
- eleven-role review records the decision and remaining gate.

## Stop condition

Stop with a no-go decision if `.tool-versions` requires plugin or runtime
semantics to identify executable leaves, if its ordered version alternatives
cannot be represented by V1, or if a safe adapter requires a schema change.
Any adapter implementation or contract revision requires separate approval.

## Result

The evaluation stopped with the defined no-go decision. At frozen asdf revision
`ca98e44ff49cb0a38966b23b42db962203478b59`, `.tool-versions` declares plugin
IDs and ordered version selections. asdf determines executable paths and
environments later from installed package contents and optional plugin
callbacks. The upstream fixture itself declares `golang` while the repository
invokes `go`, disproving direct plugin-ID-to-executable translation.

Three fixture groups cover ordinary versions, comments, ordered alternatives,
`system`, `path:`, and `ref:`. The dependency-free test-only evaluator
reproduces their lexical rows and proves that each retains semantics outside
READINESS-001 V1. No product code, schema, dependency, adopter, owner file, or
environment changed.

Findings `FERRIS-791` through `FERRIS-793` and the
[eleven-role review](../../../../docs/plans/reviews/FERRIS_ASDF_READINESS_ADAPTER_EVALUATION_REVIEW.md)
retain the evidence and no-go decision. Pulse 04 authority is exhausted. A
contract revision, explicit owner mapping, product adapter, or Pulse 05
requires separate approval.
