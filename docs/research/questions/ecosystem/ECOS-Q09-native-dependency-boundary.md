# ECOS-Q09: Native Dependency Boundary

**Status:** Complete

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

## Decision

Adopt renewable native-boundary evidence across exact Rust package and native
component identity, system/bundled/prebuilt/generated/external source mode,
provider capability, host and target tools, discovery and sysroot inputs, ABI,
Cargo directives and `links` metadata, generated code and bindings, native and
final artifacts, assurance coverage, reproducibility, deployment, ownership,
and renewal. Preserve typed missing-tool, missing-package, unsupported,
link-failed, generator-failed, not-observed, stale, and unknown states without
automatically installing prerequisites, changing providers, refreshing
bindings, or claiming bundled portability.

See
[Rust native dependency boundary](../../2026-08-10-rust-native-dependency-boundary.md)
and
[EXP-01](../../ecos-q09-native-dependency-boundary/results/EXP-01-native-boundary-matrix.md).

## Primary roles

Interop Boundary Auditor, Native Platform Adopter, Rust Safety Steward.
