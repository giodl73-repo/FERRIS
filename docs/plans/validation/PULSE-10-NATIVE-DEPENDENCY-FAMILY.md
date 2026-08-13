# Pulse 10 Native Dependency Family Validation

Date: 2026-08-13
Implementation cutoff: `41b7086cb43bc6b9a37b7ba5920cfdec39950f4a`
Disposition: Windows and Unix development validation passed
Evidence class: Controlled PLATFORM-001 family

## Scope and identities

Two exact zero-crate-dependency consumers use minimal conditional system FFI.
Windows links `kernel32`; Unix resolves libc. `r1` reads the current process
identity. `r2` adds the current thread identity on Windows or parent-process
identity on Unix.

| Revision | Source-tree digest | Canonical profile digest |
|---|---|---|
| `r1` | `sha256:9c7d1e08c77984c0e74772e16007340c5f64521fde84adefd6bed055242c9338` | `sha256:28ccbccd2522ded51ae7223058ef37a654c38d9f57d4f6789c6f7af12f9e3024` |
| `r2` | `sha256:73b130cceba67bc191d4ad323e5e2969b456dd53b7dfcd0d0c0d89102ecf6594` | `sha256:f796f520e662af45fd952008503adc807f97fccce829bb26e91a446be13708ba` |

Each revision passes locked/offline metadata, check, build, Clippy, tests,
doctests, and package without changing its source tree. The declarations use
no pointers, caller memory, callbacks, allocation, or dynamic loading.

Windows build 26310 and Ubuntu 24.04.4 WSL2 used Rust/Cargo 1.95.0. Each
workspace run reported 74 passing tests, 2 ignored helpers, and no failures.

## Claim boundary

The ambient native components remain installed, patched, ABI-versioned, and
serviced outside Cargo. This evidence does not establish native package
identity, broad ABI portability, arbitrary FFI safety, deployment, support,
approval, held-out evidence, or PLATFORM-001 Proposed status.
