# ECOS-Q11: Compatibility-Tested Stack Profiles

**Status:** Complete

**Area:** Application stacks

**Depends on:** ECOS-Q03 through ECOS-Q10

## Question

Can representative application stacks be tested as renewable profiles without
creating a permanent FERRIUM distribution or lock-in?

## Starting hypothesis

Profiles can provide useful compatibility evidence if requirements, versions,
features, platforms, renewal, removal, and ownership remain explicit.

## Decision informed

Whether to prototype compatibility profiles and what prevents them from
becoming a hidden distribution.

## Outcome

Adopt renewable, consumer-scoped compatibility-profile records rather than a
permanent FERRIUM distribution. Profiles retain exact releases, features,
lock and active-target closures, compiler and target pairs, validation stages,
provenance, ownership, expiry, renewal, removal, and rollback. Six exact
fixtures established hosted-server, CLI/configuration, pure-data, embedded
`no_std`, browser-WASM, and bundled-native boundaries. No profile is a
universal recommendation or compatibility certificate.

See
[Rust compatibility-tested stack profiles](../../2026-08-10-rust-compatibility-stack-profiles.md).

## Primary roles

Native Platform Adopter, Validation Checker, Scope Keeper.
