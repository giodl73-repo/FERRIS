# asdf `.tool-versions` Readiness Adapter Evaluation

Date: 2026-09-19
Status: Complete; no product adapter selected
Ferris baseline: `ebc68f65eed33f1b68ac7893674e9e4d404f6615`
asdf baseline: `ca98e44ff49cb0a38966b23b42db962203478b59`

## Decision supported

Ferris should not translate asdf `.tool-versions` directly into
READINESS-001 V1 requirements. The file declares plugin-owned version
selection, not a stable mapping from plugin IDs to executable leaves. Its
ordinary versions, ordered fallbacks, `system`, `path:`, and `ref:` forms also
carry semantics that V1 intentionally cannot represent or observe.

The explicit `doctor --requirements` path remains the supported boundary.
Future asdf integration would first need a separately approved contract that
can retain adapter disposition and owner-supplied executable mapping without
invoking asdf or its plugins.

## Research question

Can Ferris read one explicit `.tool-versions` file and losslessly convert it
into the existing platform, executable, environment-name, and
repository-relative-path requirements without executing asdf, inspecting
plugins, discovering parent or user configuration, or adding a schema?

## Starting hypothesis

A narrow adapter might treat each `.tool-versions` tool token as an executable
name and ignore versions while preserving source provenance.

Competing hypotheses were:

1. plugin names are sufficient executable identities;
2. only `system` selections can map safely to V1 executable presence; or
3. every useful mapping requires owner or plugin semantics absent from the
   source file.

Evidence would support an adapter only if the file alone determines an
executable leaf and its selection can be represented without discarding
version or fallback meaning.

## Local evidence

READINESS-001 V1 accepts exact executable leaf names and checks only passive
process-`PATH` presence. It deliberately performs no version comparison,
command execution, source precedence, or adapter interpretation
([READINESS-001](../specs/FERRIS_ENVIRONMENT_READINESS_CONTRACT.md), especially
the requirement model and passive observation algorithms).

Pulse 04 freezes three cases under
[`tests/fixtures/environment-readiness-adapters/asdf-tool-versions/`](../../tests/fixtures/environment-readiness-adapters/asdf-tool-versions/):

- the exact four-line asdf root `.tool-versions` file at the frozen upstream
  revision, LF SHA-256
  `383996cf07387f46a209f3739c189a18106de9a4d24baaed0a5ddb3030db1d64`;
- an ordered ordinary-version and `system` fallback; and
- `path:` and `ref:` selections.

The dependency-free test-only evaluator mirrors asdf's lexical line parsing
and records the V1 semantic loss for each case. It produces no Ferris
requirement and is not linked into product code.

## External evidence

The frozen asdf configuration documentation says `.tool-versions` applies in
its directory and descendants, accepts ordinary versions, `ref:`, `path:`, and
`system`, and permits multiple ordered versions
([configuration.md, lines 5-39](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/docs/manage/configuration.md#L5-L39)).

The frozen asdf version documentation says installation creates shims for
every executable in a package and `asdf exec` determines both the final
executable path and plugin-provided environment
([versions.md, lines 155-161](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/docs/manage/versions.md#L155-L161)).
Plugin authors may provide `bin/list-bin-paths`, which selects directories
containing executables and is invoked during install or reshim
([create.md, lines 436-474](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/docs/plugins/create.md#L436-L474)).

asdf's parser removes `#` comments, splits the remaining line into a tool token
and ordered version tokens, and does not add executable identity
([toolversions.go, lines 246-258](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/internal/toolversions/toolversions.go#L246-L258)).
The upstream root fixture declares plugin ID `golang`, while the repository
invokes the Go executable as `go`
([`.tool-versions`](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/.tool-versions),
[Makefile](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/Makefile)).

## Experiment and result

The test-only evaluator ran:

```powershell
cargo test -p ferris-core --test asdf_readiness_adapter_evaluation --quiet
```

Both tests passed. The parser reproduced all three frozen case rows and every
case retained at least one unsupported V1 semantic:

| Case | Observed semantic loss |
|---|---|
| Upstream asdf root | Plugin-to-executable mapping is absent; version selection is absent from V1 |
| Ordered fallback | Ordered alternatives and `system` require asdf selection semantics; versions remain absent from V1 |
| `path:` and `ref:` | The path is an asdf installation source and the ref is plugin-resolved; neither is a V1 repository path or passive executable observation |

## Findings

### FERRIS-791: An asdf tool ID is not an executable identity

**Sources**

- [asdf shim behavior](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/docs/manage/versions.md#L155-L161)
- [asdf plugin binary-path behavior](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/docs/plugins/create.md#L436-L474)
- [frozen upstream fixture](../../tests/fixtures/environment-readiness-adapters/asdf-tool-versions/upstream-asdf.tool-versions)

**Observation**

`.tool-versions` names an asdf plugin. Executable shims and their final paths
depend on installed package contents and optional plugin callbacks. The public
fixture demonstrates a concrete spelling difference: `golang` is the plugin
token while `go` is the invoked executable.

**Implication**

Ferris MUST NOT reinterpret the tool token as a READINESS-001 executable leaf.
Doing so would create false missing or false satisfied observations.

**Confidence:** High; the behavior and counterexample are both frozen from the
same upstream revision.

### FERRIS-792: Version selection cannot be erased by a V1 adapter

**Sources**

- [asdf version forms and fallback order](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/docs/manage/configuration.md#L24-L39)
- [Pulse 04 evaluation matrix](../../tests/fixtures/environment-readiness-adapters/asdf-tool-versions/evaluation.json)
- [READINESS-001 passive executable semantics](../specs/FERRIS_ENVIRONMENT_READINESS_CONTRACT.md)

**Observation**

An ordinary version, ordered alternatives, `system`, `path:`, and `ref:` all
affect which tool installation asdf selects. V1 can observe only whether one
exact executable leaf is present on the current process `PATH`; it cannot
represent a requested version, fallback order, plugin ref, or installation
source.

**Implication**

An adapter that drops the versions would claim a complete interpretation after
discarding owner policy. A V1 product adapter is therefore not lossless even
when a plugin happens to share an executable name.

**Confidence:** High; every documented selection form is outside the frozen V1
model.

### FERRIS-793: Safe asdf integration needs an explicit owner mapping boundary

**Sources**

- [READINESS-001 source semantics](../specs/FERRIS_ENVIRONMENT_READINESS_CONTRACT.md)
- [Pulse 04 fixtures](../../tests/fixtures/environment-readiness-adapters/asdf-tool-versions/)
- FERRIS-791 and FERRIS-792

**Observation**

V1 source dispositions are provenance-only and always `declared_only`; they
cannot state that an adapter found a source but declined lossy interpretation.
Calling asdf or plugin callbacks would cross the pulse's passive,
non-executable boundary.

**Implication**

Ferris should retain explicit owner requirements now. A future contract MAY
accept an owner-supplied plugin-to-executable mapping and an adapter disposition
that remains visibly unsupported when version semantics cannot be represented.
That contract and any adapter require separate authority.

**Confidence:** High for the V1 no-go; the shape of any future contract remains
a proposal.

## Model evolution

The starting hypothesis was overturned. A lexical parser is straightforward,
but lexical access does not provide the semantic identity needed by
READINESS-001. The smallest safe outcome is a documented no-go, not a partial
product adapter.

## Adopt now

- Keep `doctor --requirements` as the only implemented readiness input.
- Treat `.tool-versions` as owner provenance only when an explicit requirements
  declaration cites it.
- Preserve exact unsupported semantics instead of guessing executable names.

## Prototype behind a compatibility boundary

No product prototype is approved. A future contract experiment could test an
explicit owner mapping from asdf plugin ID to one or more executable leaves,
while preserving unrepresented version selection as unsupported.

## Reject or defer

- direct plugin-ID-to-executable translation;
- dropping version and fallback tokens;
- invoking `asdf which`, `asdf where`, `asdf exec`, install, reshim, or plugins;
- discovering parent, home, `.asdfrc`, or environment override configuration;
- retaining `path:` values in Ferris reports; and
- changing READINESS-001 in this pulse.

## Contribution path

Explain the boundary in Ferris rather than ask asdf to change its format. asdf
correctly owns plugin and version selection; Ferris needs an owner mapping if a
future adapter is justified.

## Non-goals

This evaluation makes no support, adoption, performance, completeness, or
cross-platform claim. It does not parse an adopter file or modify product code,
schemas, dependencies, owner workflows, or environments.

## Open questions

1. Is an explicit plugin-to-executable mapping useful enough to justify a V2
   source-disposition contract?
2. Should version requirements become a distinct future observation kind, or
   remain entirely owner-executed?

## Role review

The
[Pulse 04 eleven-role review](../plans/reviews/FERRIS_ASDF_READINESS_ADAPTER_EVALUATION_REVIEW.md)
accepts the no-go decision and closes adapter authority.
