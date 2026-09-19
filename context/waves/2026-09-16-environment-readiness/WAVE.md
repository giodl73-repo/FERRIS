# Wave: Environment Readiness

Status: Complete through Pulse 03
Implementation authority: Implemented bounded explicit-file passive readiness

## Systems-development gap

Ferris fails closed when owner work encounters missing tools, environment
names, configuration, or resources, but it usually discovers those gaps one at
a time during execution. Developers and CI agents can therefore run the same
owner command in materially different environments without one clear,
comparable readiness explanation.

No single existing format owns all relevant truth. Cargo and rustup own Rust
packages and toolchains; environment managers own selected tools; container and
Devfile formats own constructed environments; repository owners own native
prerequisites and validation semantics.

## Measurable decision

Decide whether Ferris should extend `doctor` with one product-neutral,
owner-declared requirement-to-observation contract and bounded read-only
adapters.

The direction advances only if it:

1. catches the dependency classes already observed in adopter work;
2. preserves every external owner's authority and source provenance;
3. distinguishes missing, mismatched, unsupported, unavailable, and
   not-observed states;
4. retains no environment values or reusable secrets;
5. performs no installation, repair, shell evaluation, lifecycle command,
   network access, or owner validation; and
6. remains optional and completely removable.

## Affected surfaces

- candidate crate: `crates/ferris-core`;
- candidate adapter: `crates/ferris-cli`;
- candidate command: existing `doctor`;
- future schemas: environment requirements and readiness report;
- future fixtures: explicit requirements, source references, Windows and Unix
  observations, missing tools, missing environment names, unsupported source
  constructs, and separately authorized future adapter conflicts; and
- candidate consumers: FERRIS itself, the public BISECT/ICELINES/REEL evidence
  shapes, and anonymized enterprise onboarding shapes.

No consumer mutation is authorized.

## Pulse sequence

| Pulse | Scope | Status |
|---|---|---|
| 01 | Research existing Ferris failures and owner-native requirement formats; select the contract boundary | Complete |
| 02 | Freeze draft schemas, fixtures, exit semantics, source precedence, removal, and eleven-role review | Complete |
| 03 | Implement explicit-file passive readiness in `doctor` with targeted conformance tests | Complete |
| 04 | Evaluate one separately selected owner-native format adapter against frozen public fixtures | Proposed; not authorized |

Each later pulse requires separate explicit approval. Pulse 04 is optional and
must not begin merely because Pulse 03 succeeds.

## Pulse 02 result

READINESS-001 is Draft after eleven-role review. V1 accepts one explicit owner
declaration and defines only platform, executable, environment-name, and
repository-relative path observations. It freezes required/advisory policy,
typed states, deterministic identity, VIEW-001 result and exit mapping, source
provenance without precedence, strict privacy exclusions, compatibility,
removal, schemas, exemplars, state vectors, and negative controls.

V1 does not interpret source formats, compare versions, run commands, retain
environment values or resolved paths, or emit source conflicts. Pulse 03 is not
authorized by Pulse 02.

## Pulse 03 authority

The user's fresh `continue` after Pulse 02 authorizes one optional
`doctor --requirements <JSON>` implementation of the frozen V1 contract and
targeted conformance tests. Existing `doctor` behavior without that option
must remain unchanged. No source adapter, new requirement kind, active probe,
repair, adopter change, support, or production claim is authorized.

## Pulse 03 result

`doctor --requirements <JSON>` now accepts one strict, bounded
`ferris.environment-requirements/v1` declaration when an explicit
`--manifest-path` identifies the observation root. The implementation validates
the frozen structural and semantic rules, performs only passive platform,
process-`PATH`, environment-name, and non-following repository-path
observations, and emits deterministic readiness reports in the existing
command-result envelope. Report-bearing non-success outcomes preserve their
typed record and VIEW-001 exit code.

No-requirements `doctor` still follows the unchanged legacy implementation.
The explicit manifest requirement in readiness mode prevents Cargo workspace
discovery from violating the no-execution boundary. Windows behavior was
executed locally; Unix executable-bit and symlink semantics are retained as
cfg-specific tests for Unix execution. The
[implementation review](../../../docs/plans/reviews/FERRIS_ENVIRONMENT_READINESS_IMPLEMENTATION_REVIEW.md)
records the completed scope and remaining limits. Pulse 04 remains unauthorized.

## Reviewers

All eleven repository roles apply. Product Value Governor controls whether the
user outcome justifies continuation; Ecosystem Strategist reviews
adopt-versus-wrap decisions; Native Platform Adopter reviews cross-machine
diagnostics; Rust Maintainer reviews ordinary workflow and removal; Scope
Keeper and Autonomy Supervisor enforce pulse boundaries.

## Non-goals

This wave does not authorize dependency installation, environment repair,
package-manager behavior, command inference, shell or workflow parsing,
lifecycle execution, active service probes, network access, secrets, CI
replacement, support certification, or changes to Cargo/rustup resolution.

## Removal and rollback

Research and draft contracts are documentation-only. The Pulse 03
implementation remains opt-in, leaves ordinary `doctor` behavior unchanged
without an explicit requirements input, and is removable without changing
Cargo, repository commands, or owner-native environment files.
