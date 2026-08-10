# ECOS-Q09: Native Dependency Boundary

**Status:** Planned

**Area:** Native integration

**Depends on:** ECOS-Q03, ECOS-Q06, ECOS-Q07

## Question

Where do C/C++ libraries, system packages, TLS or crypto providers, code
generation, and build scripts reduce portability or reproducibility?

## Starting hypothesis

Native dependencies often preserve mature capability while shifting
installation, ABI, licensing, patching, cross-compilation, and supply-chain
responsibility outside Cargo.

## Decision informed

Define the native dependency and provider evidence required by stack profiles.

## Primary roles

Interop Boundary Auditor, Native Platform Adopter, Rust Safety Steward.
