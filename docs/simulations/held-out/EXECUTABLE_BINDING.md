# Held-Out Executable Binding

Binding revision: 1
State: Public source and command binding frozen
Oracle and edit packs: Separately sealed

## Public source corpus

| Source ID | Repository | Revision |
|---|---|---|
| HSRC-CARGO | `https://github.com/rust-lang/cargo.git` | `593ae3e482ae226d1f4533b29106f76e43649b70` |
| HSRC-SERDE | `https://github.com/serde-rs/serde.git` | `747814f7d5fbab872df3b02f070c165b91bde062` |
| HSRC-RIPGREP | `https://github.com/BurntSushi/ripgrep.git` | `3fce3b5bb0236da2df6d99672afb8a719642eca7` |

The revisions are immutable inputs. A renewal creates a new binding revision;
it does not move these revisions.

## Toolchain and owner baseline

- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`;
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`;
- initial host: `stable-x86_64-pc-windows-msvc`;
- renewed host: Ubuntu 24.04.4 LTS on WSL2,
  `stable-x86_64-unknown-linux-gnu`, with the exact Rust and Cargo versions;
- owner metadata command:

```console
cargo +1.95.0 metadata --format-version 1 --no-deps --offline --locked --manifest-path <checkout>/Cargo.toml
```

The validation owner MUST record OS build, CPU architecture, filesystem,
locale, shell, environment allowlist, and command digest for each scored run.

## Fixture bindings

| Fixture | Public sources | Sealed input |
|---|---|---|
| FHIF-001 | HSRC-SERDE, HSRC-RIPGREP | `ferris-held-out-v1/FHIF-001` |
| FHIF-002 | HSRC-SERDE | `ferris-held-out-v1/FHIF-002` |
| FHIF-003 | HSRC-CARGO, HSRC-RIPGREP | `ferris-held-out-v1/FHIF-003` |
| FHIF-004 | HSRC-SERDE, HSRC-RIPGREP | `ferris-held-out-v1/FHIF-004` |
| FHIF-005 | HSRC-CARGO | `ferris-held-out-v1/FHIF-005` |
| FHIF-006 | HSRC-CARGO | `ferris-held-out-v1/FHIF-006` |
| FHIF-007 | HSRC-RIPGREP | `ferris-held-out-v1/FHIF-007` |
| FHIF-008 | HSRC-CARGO | `ferris-held-out-v1/FHIF-008` |
| FHIF-009 | HSRC-SERDE | `ferris-held-out-v1/FHIF-009` |
| FHIF-010 | HSRC-RIPGREP | `ferris-held-out-v1/FHIF-010` |
| FHIF-011 | HSRC-CARGO | `ferris-held-out-v1/FHIF-011` |
| FHIF-012 | HSRC-CARGO, HSRC-SERDE, HSRC-RIPGREP | `ferris-held-out-v1/FHIF-012` |

The sealed input name identifies an independently held edit, configuration,
failure-seed, and oracle package. Public source access does not disclose that
package.

## Public command binding

Every fixture binds the applicable subset of:

```console
ferris plan --application <fixture-application> --format json
ferris affected --application <fixture-application> --change <change-id> --format json
ferris graph --application <fixture-application> --format json
ferris query --application <fixture-application> --query <query-id> --format json
ferris explain --record <record-id> --format json
ferris check --application <fixture-application> --format json
ferris test --application <fixture-application> --format json
ferris doctor --application <fixture-application> --format json
cargo ferris <command> --format json
```

Action fixtures additionally bind:

```console
ferris run --action-plan <action-plan-id> --format json
```

No implementation pulse is authorized to run the action command until a later
action-specific review.

## Schema binding

The first implementation pulse may emit only:

- `ferris.command-result/v0`;
- `ferris.blueprint-plan/v0`; and
- `ferris.explanation/v0`.

Version `v0` is an experimental implementation schema. It MUST NOT be accepted
as conformance evidence for a later schema without an explicit migration and
viewer.

## Readiness boundary

The public source revisions, owner command, command vocabulary, process codes,
and initial output schema IDs are frozen.

The independent custodian recorded opaque package identifiers and SHA-256
digests in the
[public-safe receipt](PUBLIC_SAFE_RECEIPT.md).

Read-only development semantics are renewed on Windows and Unix. Held-out
scoring remains blocked until the scoring environments are frozen with their
complete hardware, OS, filesystem, locale, shell, environment, and command
evidence, and until an immutable implementation cutoff is selected.
Development MUST use separate fixtures.
