# FSIM-002: Breaking Public API Change

Wave: W01
Revision: 1
State: Retraced
Claim state: simulated

## Question

How does Ferris represent a cross-workspace breaking Rust API change when the
consumer has not yet migrated?

## Locked fixture

- The application and repositories are the same as FSIM-001.
- `ledger-core::parse` changes from `fn parse(&str) -> Entry` to
  `fn parse(&str) -> Result<Entry, ParseError>`.
- `ledger-cli` calls the old signature.
- Cargo package, public API, source dependency, and consumer mapping are
  available.
- No compatibility exception or migration patch exists.
- No execution is requested.

Negative control: an additive new private helper with no public exposure.

## Governing specifications

- CONTRACT-001 directional compatibility and migration;
- FOREST-002 Change Record;
- SCOPE-001 directional mappings;
- VALIDATION-001 coverage;
- PLANNING-001 contract and owner closures;
- RESOLUTION-001 eligibility and alternatives; and
- VIEW-001 explanations.

## Retraced expected behavior

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Change Record identifies a public Rust API signature change | FOREST-002 |
| Contract | Compatibility is breaking from new producer to existing consumer | CONTRACT-001 |
| Scope | Changed public item maps to consuming package, workspace, and application contract | SCOPE-001 |
| Prediction | Consumer check failure and migration work are predicted with named uncertainty | PREDICTION-001 |
| Validation | Compile-pass for migrated consumer and expected-failure for unchanged consumer are required | VALIDATION-001 |
| Planning | Plan contains producer validation, consumer migration closure, and full relevant repository gates | PLANNING-001 |
| Resolution | Current plan is ineligible for execution; alternatives are migrate, retain prior contract, defer, or reject | RESOLUTION-001 |
| Public view | `ferris plan` shows `blocked` with the violated contract and affected consumer | VIEW-001 |

The additive private-helper control does not produce contract migration work.

## Assertions

- [x] Package SemVer alone is not the compatibility result.
- [x] Direction and consumer version remain explicit.
- [x] The unchanged consumer is an expected negative fixture.
- [x] Resolution cannot rank predicted speed above contract eligibility.
- [x] No Action Plan or approval is created.

## Simulation issues

Initial trace exposed FSIM-SI-001 because the public API change lacked a
canonical triggering record. No additional issue remains after retrace.

## Specification changes

- FSIM-SCR-001.

## Claim boundary

The expected compile failure is simulated from the locked source contract. It
has not been observed by executing rustc.
