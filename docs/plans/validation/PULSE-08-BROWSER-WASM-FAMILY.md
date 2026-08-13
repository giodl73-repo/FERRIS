# Pulse 08 Browser WASM Family Validation

Date: 2026-08-12
Implementation cutoff: `dedd439fe1bb7eb693f1af4e3d851973ae44ae52`
Disposition: Windows and Unix development validation passed
Evidence class: Controlled PLATFORM-001 family

## Scope

Two exact zero-dependency libraries render bounded caller text:

- `r1` escapes text into one status paragraph; and
- `r2` adds validated language metadata and an `aria-live="polite"` contract.

Injection-shaped text remains escaped text, and invalid or oversized input is
rejected explicitly.

## Exact identities

| Revision | Package | Source-tree digest | Canonical profile digest |
|---|---|---|---|
| `r1` | `ferris-profile-browser-wasm@0.1.0` | `sha256:a7f017050b4635842de0931fc998ae9c8aa7ba47f203fbbffe65859369765e94` | `sha256:8dc50785187176ce2daa062f4999e1a0d1ec0b983a3bf0834e90b36a1e6df3de` |
| `r2` | `ferris-profile-browser-wasm@0.2.0` | `sha256:93ea173d973fcdf38926e87cdc9ac730754756ba4f62173e6d038458b353e41e` | `sha256:b91d65c31b151c9f448ceb1e28a4e1a612dea3384f7307b2db0a0a6a95e733b2` |

## Owner and platform evidence

Each revision passes locked/offline metadata, host tests and doctests,
package, and target check, build, and Clippy for
`wasm32-unknown-unknown` in isolated target directories without changing its
source tree.

Windows build 26310 and Ubuntu 24.04.4 WSL2 used Rust/Cargo 1.95.0 and the
exact WASM target. Each workspace run reported 72 passing tests, 2 ignored
helpers, and 0 failures. Browser execution and operational validation remain
unavailable because no JavaScript binding, DOM, browser, or automation owner
is configured.

## Claim boundary

This controlled result does not establish browser compatibility,
accessibility conformance, JavaScript or DOM interoperability, runtime
behavior, security, performance, bundling, deployment, support, approval,
held-out evidence, or PLATFORM-001 Proposed status.
