# Rust Reference

This directory is a generated convenience mirror of MAXIM's Rust reference
library for Ferris research, specification, and design work.

**Do not edit mirrored guides here.** Canonical edits belong in:

- `MAXIM/rust-language/`
- `MAXIM/rust-architecture/`
- `MAXIM/rust-application-blueprints/`
- `MAXIM/rust-production-engineering/`
- `MAXIM/rust-crate-ecosystem/`
- `MAXIM/rust-interop-migration/`
- `MAXIM/rust-security-assurance/`
- `MAXIM/rust-performance/`
- `MAXIM/languages/09-RUST.md`

Ferris may cite and consume these guides, but they do not create Ferris product
authority or replace Ferris specifications, research findings, fixtures, or
implementation gates.

## Entry points

- [Compact Rust card](languages/09-RUST.md)
- [Rust language guide](rust-language/00-OVERVIEW.md)
- [Rust implementation architecture](rust-architecture/00-OVERVIEW.md)
- [Rust application blueprints](rust-application-blueprints/00-OVERVIEW.md)
- [Rust production engineering](rust-production-engineering/00-OVERVIEW.md)
- [Rust crate ecosystem](rust-crate-ecosystem/00-OVERVIEW.md)
- [Rust interop and migration](rust-interop-migration/00-OVERVIEW.md)
- [Rust security assurance](rust-security-assurance/00-OVERVIEW.md)
- [Rust performance](rust-performance/00-OVERVIEW.md)
- [Mirror manifest](MIRROR-MANIFEST.json)

## Synchronization

From the Ferris repository:

```powershell
.\scripts\sync-rust-reference.ps1
.\scripts\sync-rust-reference.ps1 -Check
```

Pass `-MaximRoot <path>` when MAXIM is not available at the default sibling
TRACKER checkout.
