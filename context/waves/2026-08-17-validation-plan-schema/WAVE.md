# Wave: Validation Plan Schema Publication

Status: Closed after Pulse 01 and its single permitted corrective pass

## Product outcome

Give downstream consumers checked-in Draft 2020-12 closed structural success
schemas they can use to validate the representable current
`ferris.validation-plan/v0` JSON boundary and the success specialization of
`ferris.command-result/v2`, without changing Ferris runtime behavior or
claiming cross-field semantic exactness.

## Classification

Schema/documentation wave with one bounded implementation pulse and one
permitted corrective pass on its unpushed publication commit.

## Budget

- exactly one pulse;
- exactly one publication attempt plus one corrective pass on `f183157`;
- exactly one role review record; and
- no runtime behavior changes, successor chain, or diagnostic custody layer.

## Completion condition

The wave is complete only when Ferris:

- publishes a product-facing `docs/schemas/validation-plan/` directory with a
  README plus closed Draft 2020-12 schemas for
  `ferris.validation-plan/v0` and the `validation-plan`
  `ferris.command-result/v2` success specialization;
- describes them as closed structural success schemas rather than exact
  serializers and records the runtime semantic invariants outside portable
  JSON Schema;
- validates the checked-in documents through a dependency-free test-local
  Draft 2020-12 subset validator that resolves root/local `$ref` values,
  rejects unsupported keywords, and applies the documents to real CLI success
  outputs for selected-package closure and full-workspace fallback;
- records deterministic negative mutation controls for extra fields, missing
  required fields, invalid enums, cardinality, and whole-item uniqueness;
- keeps separate semantic assertions for activity/package identity order,
  fallback derivation, identity-key uniqueness, and cross-field references;
- updates validation-plan-facing documentation without claiming a generic
  profile-diff specialization; and
- passes the bounded validation commands recorded in Pulse 01.

## Abandonment condition

Stop and report `stop-value-exhausted` without widening scope if publication
would require stable non-success failure-envelope schemas, runtime changes, a
new validator dependency, or large duplication of unstable generic
command-result definitions. In that case publish only the clearly named
success boundary and document the omission.

## Owner actions

| Repo | Action |
|---|---|
| FERRIS | Add schemas, tests, documentation, and review locally |
| TRACKER | No-op; keep this wave separate from portfolio state |
| Cargo and external repositories | No-op; existing owner tools remain authoritative |

## Pulse table

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Validation plan schema publication | Complete after corrective pass | Published closed structural success schemas exercised from disk, with separate semantic conformance and no runtime changes |

## Non-goals

- changing `ferris-core`, `ferris-cli`, or any runtime JSON output;
- adding a new Rust JSON Schema validator dependency;
- publishing a generic schema for every Ferris `ferris.command-result/v2`
  command;
- publishing stable non-success `validation-plan` failure-envelope schemas;
- extending diagnostic release custody, held-out profile-diff scoring, or
  platform-profile authority;
- adding a successor pulse, another architectural layer, or broader validation
  infrastructure; and
- executing Cargo validation commands or claiming full-suite, release,
  platform, support, or CI equivalence.

## Completion gate

- the schema directory parses as JSON and every object schema is closed;
- targeted CLI schema tests read and apply both documents, reject unsupported
  validator keywords, and pass structural plus separate semantic controls;
- relevant existing CLI validation-plan tests pass;
- `cargo check --workspace --locked` and `git diff --check` pass;
- `rustfmt --edition 2024 --check crates\ferris-cli\tests\validation_plan_schema.rs`
  passes for the touched Rust test file; and
- one role review records the budget, no-runtime-change boundary, validation
  commands, and closeout decision.
