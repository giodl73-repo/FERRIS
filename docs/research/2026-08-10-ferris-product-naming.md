# Ferris Product Naming Decision

Date: 2026-08-10
Status: Complete
Decision: use **Ferris** as the public product and executable name, use
`cargo ferris` as the Cargo-native entrypoint, and retain **Blueprint** as the
internal application model and planning engine.

## Decision supported

This research closes
[BLUE-Q05](questions/blueprint/BLUE-Q05-ferris-product-name.md) and supplies
the naming input to PRODUCT-001, VIEW-001, and CONFORMANCE-001.

It refines the BLUE-Q04 dual-entry decision without changing the competitive
category, one-engine requirement, Cargo authority boundary, or initial
affected-work adoption wedge.

## Evidence

### Rust and Cargo naming

The Rust Foundation trademark policy permits accurate references to Rust and
permits `cargo-foobar` naming for a Cargo subcommand, provided the tool does not
appear official, affiliated, or endorsed.

Source:
[Rust Foundation trademark policy](https://rustfoundation.org/policy/rust-trademark-policy/).

### Existing `ferris` package

The crates.io `ferris` package exists and identifies version `0.2.0` as a
hierarchical timer-wheel library. Its package metadata reports no binary
names. The repository describes allocating and copying timer wheels with
constant-time start and stop operations.

Sources:

- [crates.io `ferris` API](https://crates.io/api/v1/crates/ferris); and
- [andrewjstone/ferris](https://github.com/andrewjstone/ferris).

The package-name collision prevents publication as the exact `ferris` crate.
It does not prevent a qualified package such as `ferris-cli` from providing a
binary named `ferris`.

### Cargo package observation

The crates.io API returned 404 for `cargo-ferris` on 2026-08-10. This is a
dated observation, not a reservation or ownership claim.

Source:
[crates.io `cargo-ferris` API](https://crates.io/api/v1/crates/cargo-ferris).

### Community and cultural associations

Ferris is widely associated with the Rust community's unofficial crab mascot
and with the word root related to iron. Those associations make the name
memorable and relevant, but they do not establish official status or grant a
license to any specific artwork.

Any film association remains informal wordplay. Product branding must not
depend on protected film artwork, character likenesses, quotations, or implied
endorsement.

## Product decision

Use:

```console
ferris
cargo ferris
```

Do not require:

```console
ferris blueprint
cargo blueprint
```

Blueprint remains visible where the internal model is the subject:

- Blueprint Model;
- Blueprint Plan;
- Blueprint planning engine; and
- Blueprint schema or adapter packages where technically appropriate.

Public positioning becomes:

> **Ferris is the cross-workspace enterprise build system for Rust.**

This phrase is bounded by the statement that Cargo and all participating owner
systems retain their local semantics.

## Recommendations

### Adopt now

- Standardize Ferris as the public product.
- Standardize `ferris` and `cargo ferris` as the command surfaces.
- Retain one semantic engine.
- Retain Blueprint as the internal model and plan.
- Publish only qualified package names.
- Add an explicit independent-project statement to release materials.

### Prototype behind a compatibility boundary

- one parser and semantic command model for both entrypoints;
- a `cargo-ferris` adapter for current-workspace discovery;
- a `ferris` adapter for application and multi-workspace discovery; and
- parity fixtures proving identical semantics for identical explicit inputs.

### Reject or defer

- claiming the exact `ferris` crates.io package;
- maintaining separate command implementations;
- using existing mascot artwork without license review;
- formal film-themed branding;
- implying official Rust Project or Foundation status; and
- implementation before public-contract role review.

## Findings

### FERRIS-753: Ferris is the correct public product name

**Sources:** product discussion, Rust community naming context, and BLUE-Q04.

**Observed behavior:** Ferris is short, Rust-recognizable, iron-related, and
strong enough to carry the product category without an explanatory namespace.

**Implication:** public positioning should lead with Ferris rather than FERRIS
Blueprint.

**Confidence:** High for product direction.

### FERRIS-754: Blueprint should remain the internal planning term

**Sources:** BLUE-Q01 through BLUE-Q04 and the Blueprint architecture program.

**Observed behavior:** Blueprint already precisely names the normalized
application model and dynamic non-executable plan.

**Implication:** changing the internal term would create unnecessary
architecture churn; changing only the public boundary improves clarity.

**Confidence:** High.

### FERRIS-755: the `ferris` package collision does not block the executable

**Sources:** crates.io `ferris` API and the package repository.

**Observed behavior:** the occupied package is an unrelated library and
publishes no binary.

**Implication:** publish qualified packages that provide the `ferris`
executable.

**Confidence:** High for the dated registry state.

### FERRIS-756: `cargo ferris` fits the official external-subcommand pattern

**Sources:** Rust Foundation trademark policy and Cargo external-tool
convention cited by BLUE-Q04.

**Observed behavior:** `cargo-foobar` packages may provide `cargo foobar`
without prior permission when they do not imply official status.

**Implication:** `cargo-ferris` is the appropriate Cargo-native adapter name.

**Confidence:** High.

### FERRIS-757: branding must remain independent

**Sources:** Rust Foundation trademark policy and third-party artwork
licensing boundaries.

**Observed behavior:** Rust and Cargo marks may not be used to imply official
affiliation, while individual mascot artwork has its own license.

**Implication:** use original Ferris visual identity, accurate compatibility
language, and an independent-project statement.

**Confidence:** High.

### FERRIS-758: public-boundary specifications are ready for role review

**Sources:** FERRIS-753 through FERRIS-757 and BLUE-Q01 through BLUE-Q04.

**Observed behavior:** product identity, commands, internal terminology,
package constraints, authority, adoption wedge, and claim boundaries now
converge.

**Implication:** PRODUCT-001, VIEW-001, and CONFORMANCE-001 may enter
nine-role review without authorizing implementation.

**Confidence:** High.

## Limitations

- Package and executable names are not reserved.
- This document is not legal advice or a trademark clearance.
- Registry, trademark, and ecosystem state may change.
- User research has not yet tested the final command vocabulary.
