# Pulse 08: Browser WASM Profile Family

Status: Authorized; implementation and validation pending
Implementation authority: Bounded to this document

## Goal and authority

Complete one zero-dependency browser-WASM family compiled for
`wasm32-unknown-unknown` with deterministic rendering tested on the owner
host.

Revision `r1` escapes caller text into one bounded status element. Revision
`r2` adds validated language metadata and an explicit `aria-live` contract.

This pulse authorizes exact local revisions, host positive and rejection
tests, target check/build/Clippy, complete test-only profiles and digests,
locked/offline isolated Cargo commands, source immutability, Windows/Unix
validation, and one nine-role review.

It does not authorize JavaScript bindings, a DOM, browser execution,
automation, network, storage, bundling, publishing, deployment, performance,
security, support, production generation, another family, or held-out access.

## Acceptance

- both revisions use zero dependencies and compile for
  `wasm32-unknown-unknown`;
- text escaping is exact and injection-shaped input remains text;
- `r2` rejects invalid language metadata before rendering;
- host tests, doctests, metadata, package, target check/build/Clippy pass;
- browser execution and operational validation remain unavailable;
- source trees and exact profile digests are stable and distinct;
- Windows and Unix use Rust/Cargo 1.95.0; and
- all nine roles accept the measured result.

## Evidence

- [Authorization review](../../../../docs/plans/reviews/PULSE-08-BROWSER-WASM-ROLE-REVIEW.md)
