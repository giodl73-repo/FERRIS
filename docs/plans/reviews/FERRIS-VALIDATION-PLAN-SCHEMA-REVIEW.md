# Ferris Validation Plan Schema Review

Date: 2026-08-17; corrected 2026-08-19
Scope: Pulse 01 validation-plan schema publication
Disposition: Findings closed by the single permitted corrective pass
Implementation authority: No runtime expansion

## Corrective review findings

Commit `f183157` overstated the checked-in documents as exact serializers and
tested real outputs primarily through parallel handwritten structural
predicates rather than by applying the published schema documents.

The single permitted corrective pass:

- reclassifies both documents as closed structural success schemas;
- tightens representable serializer invariants with cardinality,
  whole-item-uniqueness, package-identity lexical, and package/activity
  presence constraints;
- explicitly keeps activity/package identity equality and order,
  fallback/package equality, `required_by_inputs` derivation, and uniqueness by
  identity key outside the portable schema claim;
- replaces the handwritten structural oracle with a dependency-free
  test-local validator for exactly the Draft 2020-12 subset used by the two
  documents, including root/local `$ref` resolution and unsupported-keyword
  failure; and
- applies the checked-in documents to actual selected-package and fallback CLI
  successes plus structural negative mutations, while retaining separate
  semantic-conformance controls.

No runtime, federated schema, failure schema, or production dependency changed.

## Product Value Governor

Disposition: `continue-within-budget`

Approved outcome before implementation: downstream consumers can validate the
current `validation-plan` success record and success-specialized
command-result envelope with checked-in Draft 2020-12 schemas, while Ferris
runtime behavior stays unchanged.

Approved budget as corrected: one pulse, one publication attempt, the single
permitted corrective pass on `f183157`, one role review record, and no
successor chain, diagnostic custody layer, or runtime behavior change.

Completion condition: the repo must publish product-facing schemas, keep the
boundary command-specific rather than generic, cover real CLI success,
including full-workspace fallback, record deterministic negative mutation
controls, and pass bounded validation.

Abandonment condition: stop `stop-value-exhausted` if publication requires
stable non-success failure-envelope schemas, runtime changes, a new validator
dependency, or large duplication of unstable generic command-result
definitions.

Measured result: the pulse and its one corrective pass stayed inside the
authorized boundary, published the structural schema directory, exercised the
documents themselves against real CLI outputs and negative mutations, and
left runtime behavior unchanged. No continuation is approved.

## Rust Safety Steward

Accept. The change is documentation and test support only, adds no `unsafe`,
and does not widen Ferris safety claims beyond the existing read-only command
boundary.

## Compiler Performance Engineer

Accept with no performance claim. The pulse adds no benchmarks, latency
numbers, or runtime path changes.

## Interop Boundary Auditor

Accept after correction. The published schemas describe visible structural
JSON boundaries without claiming ABI, native, runtime, cross-language, or
cross-field semantics they cannot express.

## AI Assurance Skeptic

Accept after correction. The review records the schema files and validation
commands without calling the documents exact serializers. Real non-success
outputs stay runtime-visible as typed failures without being promised as
published schema contracts.

## Ecosystem Strategist

Accept. The pulse uses existing serde/JSON tooling plus a dependency-free
test-local schema interpreter for a concrete consumer need instead of
introducing a production validator stack or parallel generic publication
program.

## Rust Maintainer

Accept after correction. The patch is focused, removes no existing behavior,
keeps validation in one dedicated test file, and makes the structural versus
semantic boundary reviewable.

## Native Platform Adopter

Accept after correction. Consumers get explicit structural success-contract
files they can validate in their own tooling without changing how Ferris runs
or requiring extra local setup.

## Scope Keeper

Accept. This remains one bounded schema/documentation pulse for the existing
`validation-plan` command. Stable non-success envelope publication, runtime
behavior, broader command-result generalization, and diagnostic custody work
remain deferred. The single corrective pass is consumed.

## Validation Checker

Accept after correction. The test reads and applies both checked-in schemas to
real selected-package and full-workspace-fallback successes, rejects structural
negative mutations, fails on unsupported schema keywords, and separately checks
the semantic invariants outside portable JSON Schema.

## Autonomy Supervisor

Accept. The product outcome, budget, completion condition, and abandonment
condition remain bounded. One pulse and its single permitted corrective pass
were consumed; no successor or automatic follow-on loop was started.

## Validation

Commands run on the recorded worktree:

```console
cargo test -p ferris-cli --test validation_plan_schema
cargo test -p ferris-cli --test cli validation_plan_
cargo check --workspace --locked
rustfmt --edition 2024 --check crates\ferris-cli\tests\validation_plan_schema.rs
python -c "import json, pathlib; [json.loads(path.read_text(encoding='utf-8')) for path in pathlib.Path('docs/schemas/validation-plan').glob('*.json')]"
git diff --check
```

Result summary:

- both checked-in schema documents parse as JSON, keep closed object
  boundaries, and use only the explicitly supported test-validator subset;
- real CLI selected-package-closure and full-workspace-fallback successes are
  validated by resolving and applying the checked-in documents;
- negative mutations for extra fields, missing required fields, invalid enums,
  cardinality, and whole-item uniqueness are rejected by those documents;
- separate controls demonstrate and enforce the non-schema semantic boundary
  for activity/package identity equality and order, fallback derivation, and
  identity-key uniqueness;
- the published command-result schema is validation-plan-specific rather than a
  generic profile-diff claim and intentionally stays success-only; and
- `rustfmt --edition 2024 --check crates\ferris-cli\tests\validation_plan_schema.rs`
  passed for the touched Rust file; and
- no runtime files changed.

## Decision

The corrective findings are closed. Pulse 01 now truthfully publishes
checked-in product-facing closed structural validation-plan success schemas,
document-driven structural tests, and separate semantic-conformance assertions
with no runtime behavior or production dependency change.
