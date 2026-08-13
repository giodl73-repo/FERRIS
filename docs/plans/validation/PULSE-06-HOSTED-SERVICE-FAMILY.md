# Pulse 06 Hosted Service Family Validation

Date: 2026-08-12
Implementation cutoff: `de5b5242a26ed5ce15d1dae2d3ec333a3a7663d2`
Disposition: Windows and Unix development validation passed
Evidence class: Controlled PLATFORM-001 family

## Scope

Two exact zero-dependency service libraries implement one in-process request
operation:

- `r1` provides a fixed health response plus deterministic malformed-request
  and cancellation states; and
- `r2` adds explicit readiness ownership and retains unavailable until the
  owner transitions the service to ready.

Neither revision opens a listener, uses a network, persists service state, or
claims a deployment environment.

## Exact identities

| Revision | Package | Source-tree digest | Canonical profile digest |
|---|---|---|---|
| `r1` | `ferris-profile-hosted-service@0.1.0` | `sha256:06782503954abbf93a789a17dd6268b6b081b089ff4f39abd568875f51d2c779` | `sha256:eda5c59e22c562d1bc913e729d4671eccaa69b66970d0f03fe5f5728e219f165` |
| `r2` | `ferris-profile-hosted-service@0.2.0` | `sha256:f9e9d269b7da388fe4cf6dc67df46ae7214d4f9a686ccb44b91a7a6412d928b0` | `sha256:455eac23cecd05dd6549a65592381502cec3a9534ee07d277195a73fc9d0b87e` |

## Owner evidence

Each revision passes locked/offline metadata, check, build, Clippy with
warnings denied, all-target tests, doctest, and package in separate external
target directories. Metadata identifies exactly one package at the expected
version. Every command leaves the complete source and lock tree
byte-identical.

Behavior tests cover successful health, malformed requests, cancellation,
readiness unavailable, and the explicit not-ready to ready transition.
Unavailable remains a non-success state in both the owner behavior and the
canonical profile.

## Platform evidence

Windows build 26310 and Ubuntu 24.04.4 WSL2 both used Rust/Cargo 1.95.0. Each
full workspace run reported 70 passing tests, 2 ignored bounded-command
helpers, and 0 failures. Formatting and Clippy passed on both; Windows Git
diff validation passed.

## Claim boundary

This completes only the controlled hosted-service family. It does not
establish socket, network, framework, database, credential, TLS, deployment,
operations, performance, security, support, approval, lifecycle completion,
held-out evidence, or PLATFORM-001 Proposed status.
