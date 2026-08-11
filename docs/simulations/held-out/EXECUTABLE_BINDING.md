# Held-Out Executable Binding

Binding revision: 2
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

## Implemented read-only command binding

Implemented fixtures MUST use the exact argument vocabulary below:

```console
ferris plan --workspace-id <portable-id> --manifest-path <checkout>/Cargo.toml --format json
ferris explain --workspace-id <portable-id> --manifest-path <checkout>/Cargo.toml --format json
ferris graph --workspace-id <portable-id> --manifest-path <checkout>/Cargo.toml --format json
ferris doctor --workspace-id <portable-id> --manifest-path <checkout>/Cargo.toml --format json
```

`affected`, `query`, `check`, `test`, `run`, and `cargo ferris` remain planned
surfaces. They have no executable binding until an implementation pulse
defines and validates their adapters.

## Machine-output framing

Each invocation emits one complete JSON envelope followed by a newline.
Successful records are written only to stdout. Non-success diagnostics are
written only to stderr. Ferris MUST NOT split one JSON envelope across the two
streams.

Every envelope MUST use `ferris.command-result/v1` and include:

- normalized request `invocation_identity`;
- complete-outcome `result_identity`;
- `result_class`;
- matching numeric `process_exit_code`;
- diagnostics whose classes match the envelope; and
- the command-specific record when one was produced.

The actual process exit code MUST equal the recorded `process_exit_code`.

Bounded owner-command output is digested using
`length-prefixed-stdout-stderr/v1`:

1. ASCII domain `ferris.command-output/v1`;
2. one NUL domain terminator;
3. stdout length as unsigned 64-bit little-endian;
4. retained stdout bytes;
5. stderr length as unsigned 64-bit little-endian; and
6. retained stderr bytes.

The existing per-stream byte limits apply before framing.

Successful doctor evidence and bounded doctor diagnostics MUST expose, for
each stream:

- retained byte count;
- observed byte count;
- observed-but-omitted byte count;
- whether additional unobserved bytes are unknown;
- completion state; and
- truncation and stream-read-failure state.

Bounded diagnostics additionally expose
`ferris.bounded-output-evidence/v0`, the retained-pair digest, and the
termination reason. Output-limit evidence uses a deterministic lower bound:
the overflowing stream records the retained limit plus one observed omitted
byte; a non-overflowing peer stream is empty and explicitly unknown. Termination
scope is the direct child, cleanup waiting is bounded to one second, and
cleanup completion is explicit. Doctor failure invocation identity binds the
complete typed diagnostic.

## Schema binding

The implemented read-only pulses may emit only:

- `ferris.command-result/v1`;
- `ferris.blueprint-plan/v0`; and
- `ferris.explanation/v0`;
- `ferris.workspace-graph/v0`; and
- `ferris.doctor-report/v0`; and
- `ferris.bounded-output-evidence/v0`.

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
