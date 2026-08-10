# Pulse 03: FERRIS Application Contract Architecture

## Goal

Define, but do not yet implement, the layered contract and evidence
architecture for the FERRIS enterprise Rust application platform.

## Changes

- Rename the platform and repository identity from FERRIUM to FERRIS while
  retaining historical `FERRIUM-*` finding identifiers.
- Keep RUNE as the product-neutral semantic contract repository and make
  FERRIS a consumer and contributor rather than copying RUNE code.
- Define ordinary Rust public APIs plus Cargo SemVer as the rebuilt-crate
  contract.
- Define RUNE descriptors as the stable semantic identity, compatibility, and
  projection layer.
- Use C ABI with opaque handles for independently versioned native components.
- Use WIT and the WebAssembly Component Model for sandboxed polyglot
  components.
- Use explicit wire IDLs for remote services and durable messages.
- Define renewable enterprise crate profiles with provider substitution,
  support, security, renewal, removal, and rollback.
- Specify source revision, toolchain, command, diagnostic, test, lint, benchmark,
  and limitation records.
- Distinguish compiler evidence from behavioral and assurance claims.
- Add CONTRACT-001 and PLATFORM-001 to the specification sequence.
- Specify future Rust API, C ABI, WIT, wire, provider-substitution, migration,
  renewal, removal, and compatibility tests.
- Review the contract through Rust safety and AI assurance roles.

## Decisions

- Do not standardize `rustc` metadata, `extern "Rust"`, Rust vtables, or a
  stable ABI for arbitrary Rust types.
- Do not use Cargo features as runtime interface negotiation.
- Do not merge RUNE into FERRIS without a measured release, ownership, or
  adoption problem and a separate cross-repository pulse.
- Do not select one permanent runtime, TLS provider, database, or application
  stack during contract specification.

## Validation

- `git grep -n "RUNE\\|C ABI\\|WIT\\|CONTRACT-001\\|PLATFORM-001" -- README.md PRODUCT_PLAN.md docs context`
- `git diff --check`

## Status

In progress. Architecture and research decisions are recorded; normative
specifications and held-out proofs remain pending.
