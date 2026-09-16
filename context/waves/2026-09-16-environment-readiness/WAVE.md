# Wave: Environment Readiness

Status: Active; Pulse 01 complete
Implementation authority: None

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
  observations, conflicts, missing tools, missing environment names, and
  unsupported source constructs; and
- candidate consumers: FERRIS itself, the public BISECT/ICELINES/REEL evidence
  shapes, and anonymized enterprise onboarding shapes.

No consumer mutation is authorized.

## Pulse sequence

| Pulse | Scope | Status |
|---|---|---|
| 01 | Research existing Ferris failures and owner-native requirement formats; select the contract boundary | Complete |
| 02 | Freeze draft schemas, fixtures, exit semantics, source precedence, removal, and eleven-role review | Proposed; not authorized |
| 03 | Implement explicit-file passive readiness in `doctor` with targeted conformance tests | Proposed; not authorized |
| 04 | Evaluate one separately selected owner-native format adapter against frozen public fixtures | Proposed; not authorized |

Each later pulse requires separate explicit approval. Pulse 04 is optional and
must not begin merely because Pulse 03 succeeds.

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

Research and draft contracts are documentation-only. A future implementation
must remain opt-in, leave ordinary `doctor` behavior unchanged without an
explicit requirements input, and be removable without changing Cargo,
repository commands, or owner-native environment files.
