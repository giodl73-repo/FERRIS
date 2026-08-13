# Pulse 05 CLI and Configuration Family Validation

Date: 2026-08-12
Implementation cutoff: `1d2269842295b14a33e44bf99b62693697e78de4`
Disposition: Windows and Unix development validation passed
Evidence class: Controlled PLATFORM-001 family

## Scope

Two exact zero-dependency CLI consumers implement one name-resolution
operation:

- `r1`: `--name` over `FERRIS_FIXTURE_NAME` over `item`; and
- `r2`: `--name` over explicit `--config name=<value>` over environment over
  `item`.

Revision `r2` reads only the explicit path, bounds it at 1 KiB, and returns
deterministic invalid or unavailable exits for malformed, oversized,
non-UTF-8, or missing input.

## Exact identities

| Revision | Package | Source-tree digest | Canonical profile digest |
|---|---|---|---|
| `r1` | `ferris-profile-cli-configuration@0.1.0` | `sha256:75c21266ca3cf89638f84776654ad251873c5789c28d46321929f1ce613982aa` | `sha256:312962fe4cf0a23f412095ff2f07bc5bbea53092266ac1ecc1652649bcb42898` |
| `r2` | `ferris-profile-cli-configuration@0.2.0` | `sha256:f32532ce5a9a4a93b2a1a5b3fbe782228d22e2b17418a4591c1293a9c5e1603a` | `sha256:b9269f9905e3d794dbb2d4052e0255bdfdad930465c7b9db1365485d18724f3a` |

## Owner evidence

Each revision passes locked/offline metadata, check, build, Clippy with
warnings denied, all-target unit and process tests, doctest, and package in
separate external target directories. Metadata identifies exactly one package
at the expected version. Every command leaves the complete source and lock
tree byte-identical.

Process tests cover CLI/environment/default precedence, explicit config
precedence, unknown arguments, missing config, malformed config, oversized
config, and non-UTF-8 config.

## Platform evidence

Windows build 26310 and Ubuntu 24.04.4 WSL2 both used Rust/Cargo 1.95.0. Each
full workspace run reported 69 passing tests, 2 ignored bounded-command
helpers, and 0 failures. Formatting and Clippy passed on both; Windows Git
diff validation passed.

## Claim boundary

This completes only the controlled CLI/configuration family. It does not
establish implicit discovery, secrets, external parsers, installation,
deployment, native behavior, another family, performance, security, support,
approval, lifecycle completion, held-out evidence, or PLATFORM-001 Proposed
status.
