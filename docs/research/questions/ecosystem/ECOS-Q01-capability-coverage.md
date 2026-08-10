# ECOS-Q01: Rust Capability Coverage

**Status:** Complete

**Area:** Application-platform capability map

**Depends on:** None

## Question

Which common application capabilities are provided by Rust `core`, `alloc`, or
`std`; by the official toolchain or Rust project; by external crates; or are
still materially absent?

## Starting hypothesis

Most common capabilities exist, but many are outside the standard library and
their unresolved gaps concern discovery, interchange, provider choice,
portability, governance, maintenance, and lifecycle rather than missing
algorithms.

## Investigation focus

- Define capability and coverage classes without ranking individual crates.
- Separate language and `std` guarantees from official tools and ecosystem
  conventions.
- Identify capabilities that are unavailable versus available but fragmented,
  runtime-coupled, provider-dependent, data-update-dependent, or native-bound.
- Compare bundled platform scope with .NET, Go, and Java without assuming that
  larger standard libraries are automatically better.
- Define the capability concepts required by OSPREY.

**Model changes if:** substantial application areas have no credible Rust
implementation or official ownership path.

## Decision informed

Freeze the Crates Series capability taxonomy and coverage classes used by
ECOS-Q02 through ECOS-Q12.

## Primary roles

Ecosystem Strategist, Rust Maintainer, Native Platform Adopter.

## Decision

Adopt five coverage classes: Guaranteed, Official, Ecosystem available,
Fragmented, and Material gap.

Rust has broad application capability, but much of it is governed outside
`std`. The primary opportunity is a renewable capability, interchange,
provider, platform, data, stewardship, security, and lifecycle evidence model,
not a replacement standard library.

No external crate is ranked or approved by ECOS-Q01.

See [Rust capability coverage](../../2026-08-09-rust-capability-coverage.md).
