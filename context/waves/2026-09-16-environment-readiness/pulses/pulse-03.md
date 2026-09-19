# Pulse 03: Passive Environment Readiness

Status: Complete

## User outcome

Let a developer run `doctor` against one explicit owner requirements file and
receive one deterministic, privacy-preserving explanation of local readiness
before owner validation starts.

## Authority

The user's fresh `continue` after Pulse 02 authorizes the smallest product
implementation of READINESS-001:

- one optional `doctor --requirements <JSON>` input;
- strict bounded V1 parsing and semantic validation;
- passive platform, process-path executable, environment-name, and
  repository-relative path observations;
- deterministic readiness reports and existing command-result mappings; and
- targeted core, CLI, schema, Windows, Unix, privacy, identity, failure, and
  removal tests.

Existing `doctor` behavior without `--requirements` MUST remain unchanged.

This pulse does not authorize owner-native source adapters, version execution,
installation, repair, owner command execution, shell or lifecycle evaluation,
network, service, credential, or resource probes, Action Plan eligibility,
adopter changes, CI replacement, support, production, or savings claims.

## Maximum effort

One core module, one optional CLI argument, targeted conformance tests,
documentation and context updates, and one eleven-role closeout.

## Completion test

- the committed Pulse 02 fixtures and controls are enforced;
- supported observations are passive and retain no values or resolved paths;
- required failures map exactly to VIEW-001 classes and process codes;
- Windows and Unix-specific resolution behavior has targeted proof;
- the public CLI cannot mutate the workspace for readiness inputs;
- existing `doctor` behavior remains unchanged when the option is absent;
- targeted core, CLI, and schema validation passes;
- links, fences, staged scope, and `git diff --check` pass; and
- eleven-role review records remaining limits and stop authority.

## Stop condition

Stop after the bounded explicit-file implementation and evidence. Any source
adapter, additional requirement kind, active probe, adopter evaluation, or
support claim requires a separate approved pulse.

## Result

The bounded V1 slice is implemented in `ferris-core` and exposed only through
the optional `doctor --requirements <JSON>` CLI argument. Requirements mode
requires an explicit `--manifest-path`, uses its canonical parent as the
repository observation root, and does not enter Cargo discovery or launch an
owner executable. Legacy `doctor` remains on its prior code path when the
option is absent.

The implementation enforces the 1 MiB exact-byte boundary, nested duplicate
member rejection, schema-version classification, strict structural and
semantic rules, workspace identity equality, canonical source and requirement
ordering, owner authority, path rules, and privacy exclusions. It emits one
observation per requirement, applies the frozen aggregate precedence, retains
report-bearing non-success records, and binds requirements, report, selection,
invocation, and result identities.

Focused Windows validation passed 15 core tests, 10 CLI unit tests, and 5 CLI
integration tests. Those tests cover every frozen requirements control,
malformed, duplicate, unsupported, unavailable, and oversized inputs; advisory
and required absence; privacy; non-execution; exact input-path and
selected-manifest identity; stale declaration revalidation; Windows exact-leaf
and PATHEXT resolution; relative process paths; filesystem kinds and symlinks
where platform-feasible; human output; and the unchanged legacy record mode.
The requirements, report, and an emitted command envelope also passed their
frozen JSON schemas. Unix executable-bit and symlink behavior is covered by
cfg-specific tests but was not executed in this Windows closeout.

One contract interpretation was made explicit: readiness requires
`--manifest-path` even through `cargo ferris`. Allowing Cargo-based implicit
workspace discovery would contradict the Pulse 03 prohibition on launching
Cargo when requirements are present.

The
[eleven-role implementation review](../../../../docs/plans/reviews/FERRIS_ENVIRONMENT_READINESS_IMPLEMENTATION_REVIEW.md)
records no blocking finding. Pulse 03 authority is exhausted. Pulse 04, source
adapters, additional observations, repair, execution, adoption, support, and
production claims remain unauthorized.
