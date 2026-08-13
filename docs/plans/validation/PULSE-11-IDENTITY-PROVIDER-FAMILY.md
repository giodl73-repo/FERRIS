# Pulse 11 Identity and Provider Family Validation

Date: 2026-08-13
Implementation cutoff: `3039cdb70247546ca8d53a0b318ecf2d81b778c3`
Disposition: Windows and Unix development validation passed
Evidence class: Controlled PLATFORM-001 family

Two exact zero-dependency consumers use only bounded synthetic credentials.
`r1` exposes identity without retaining the secret. `r2` adds explicit Alpha
or Beta synthetic provider selection and distinct deterministic responses.

| Revision | Source-tree digest | Canonical profile digest |
|---|---|---|
| `r1` | `sha256:6f89cb80d6ff0e688ade3cea035fa9d76a0f81a06cc807af48422b0650f2a7e5` | `sha256:04783762ae0d440608fadbd0aba66864b2bd0f7115ba8a65a999387dd7247e81` |
| `r2` | `sha256:4241e028dabad3603e26b8c8c9fe83aa440605bb22fd765a71d2845f4d38ded4` | `sha256:b7b25593443587d6684a1ecc9241732315f5b23a316ab8798cce74b6bea94776` |

Malformed, empty, and oversized inputs reject without exposing secret values.
Both revisions pass locked/offline owner commands without source mutation.
Windows build 26310 and Ubuntu 24.04.4 WSL2 used Rust/Cargo 1.95.0; each
workspace run reported 75 passing tests, 2 ignored helpers, and no failures.

These fixtures do not authenticate, authorize, encrypt, negotiate TLS, store
keys, contact providers, or establish any security property.
