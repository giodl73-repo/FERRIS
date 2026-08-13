# Pulse 09: WebAssembly Component Profile Family

Status: Complete
Implementation authority: Bounded to this document

## Goal and authority

Complete one zero-dependency WebAssembly-component family with an exact local
WIT world, host-tested semantics, and `wasm32-wasip2` artifact compilation.
Revision `r1` exports one bounded normalization operation. Revision `r2` adds
an explicit result error and maximum-input contract.

This pulse authorizes two exact consumers and WIT contracts, host tests,
target check/build/Clippy, artifact existence checks, test-only profiles and
digests, isolated locked/offline commands, immutability, cross-platform
validation, and one nine-role review.

It does not authorize generated bindings, a component runtime, composition,
registry, network, preview compatibility, deployment, support, production
generation, another family, or held-out access.

## Acceptance

- WIT revisions and Rust semantics are exact and distinct;
- both compile for `wasm32-wasip2` and produce a non-empty `.wasm` artifact;
- host positive and rejection tests pass;
- runtime execution remains unavailable;
- source and profile digests remain stable;
- Windows and Unix use Rust/Cargo 1.95.0; and
- all nine roles accept measured evidence.

## Evidence

- [Authorization review](../../../../docs/plans/reviews/PULSE-09-WASM-COMPONENT-ROLE-REVIEW.md)
- [Windows and Unix validation](../../../../docs/plans/validation/PULSE-09-WASM-COMPONENT-FAMILY.md)

Implementation cutoff:
`f565270ac61d68bb18347bf0c05b5a0f49463a3f`.

Both revisions passed host semantic, exact WIT, `wasm32-wasip2` artifact,
immutability, and repository gates on Windows and Ubuntu WSL2. Runtime
execution remains unavailable.
