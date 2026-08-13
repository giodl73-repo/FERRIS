# Pulse 09 WebAssembly Component Family Validation

Date: 2026-08-12
Implementation cutoff: `f565270ac61d68bb18347bf0c05b5a0f49463a3f`
Disposition: Windows and Unix development validation passed
Evidence class: Controlled PLATFORM-001 family

## Scope and identities

Two exact zero-dependency consumers freeze matching local WIT worlds and host
semantics. `r1` is infallible normalization; `r2` adds bounded ASCII input and
explicit `too-long` and `invalid-character` errors.

| Revision | Source-tree digest | Canonical profile digest |
|---|---|---|
| `r1` | `sha256:8bab46523bd5e5e424aca8299893073abced13e2aedd5dc4729f86acfc04a34d` | `sha256:8b42f00497ad14232690a75f4842d555e742a5169e3341f2b1202f24e9a5b9b9` |
| `r2` | `sha256:c6db59a59c645565c89075d06ff478b9e983991424e602fd3196afd07d07fffd` | `sha256:740ba73e876d89635477712062ca85749a6162e539a4dfc803c4b80a7c1391d5` |

Each revision passes locked/offline metadata, host tests and doctests,
package, target check/build/Clippy for `wasm32-wasip2`, and a non-empty `.wasm`
artifact check without changing the consumer tree.

Windows build 26310 and Ubuntu 24.04.4 WSL2 used Rust/Cargo 1.95.0. Each
workspace run reported 73 passing tests, 2 ignored helpers, and no failures.
No runtime or independent component-structure inspection was performed.

## Claim boundary

This does not establish generated-binding correctness, runtime compatibility,
composition, preview portability, registry behavior, deployment, support,
approval, held-out evidence, or PLATFORM-001 Proposed status.
