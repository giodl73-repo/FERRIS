# Held-Out Score Cutoff 001

State: Frozen for independent scoring
Frozen commit: `0cc01df0835f7651a66dd884321325e8a316775c`
Authorized claims: Pulse 01 local `plan` and `explain` only

## Scope

This cutoff evaluates only claims implemented by the read-only planning pulse:

- explicit local Cargo manifest selection;
- official Cargo metadata ownership;
- offline and locked observation;
- non-executable plan output;
- workspace-relative portable identity;
- plan and explanation semantic consistency;
- no sibling workspace discovery;
- fixed process classes for the implemented failure paths; and
- equivalent Windows and Unix semantics.

Every action, mutation, affected-only, graph, query, check, test, doctor, MCP,
connector, AI, approval, remote-evidence, deployment, and removal-runtime claim
is outside this score. An out-of-scope result is not converted into success.

## Windows environment

- OS: Microsoft Windows 11 Enterprise Insider Preview;
- version: `10.0.26310`, build `26310`;
- architecture: 64-bit x86;
- CPU: 12th Gen Intel(R) Core(TM) i7-12800HX;
- filesystem: NTFS;
- culture: `en-US`;
- shell: PowerShell `7.6.4`;
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`; and
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`.

## Unix environment

- OS: Ubuntu 24.04.4 LTS under WSL2;
- kernel: `6.6.87.2-microsoft-standard-WSL2`;
- architecture: `x86_64`;
- CPU: 12th Gen Intel(R) Core(TM) i7-12800HX;
- source filesystem: WSL `9p` mount over the Windows checkout;
- locale: `C.UTF-8`;
- shell: `/bin/bash`;
- toolchain: `stable-x86_64-unknown-linux-gnu`;
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`; and
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`.

WSL2 is the first Unix renewal environment. It does not establish independent
Linux hardware or filesystem support.

## Command binding

| SHA-256 | Command |
|---|---|
| `810e75f0d3dee10e68076ea4a252c5cad284ef355c74785fc33f0cd493f3b98e` | `cargo fmt --all -- --check` |
| `ec4556a178527361344d876629bb9533c63fc8fcd216f423dd8bce76787fce86` | `cargo test --workspace` |
| `196d9a67d5f68bf4c7de40ebb3c510406e3b856cbbb539e5eae8a61a39b39e28` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `aae38459398ac0fedb89de274016985d5d4d16fe4a5f29f42822bae83c60317a` | `cargo run --quiet -p ferris-cli -- plan --manifest-path <fixture>/Cargo.toml --format json` |
| `67fa6dc0b157737eecd1f15847272c4e547f59c92b82e11fe7193a2966869fc5` | `cargo run --quiet -p ferris-cli -- explain --manifest-path <fixture>/Cargo.toml --format json` |

The custodian MAY substitute an equivalent prebuilt binary from the same
commit if its build command, target, and digest are retained.

## Custody

The 12 package identifiers and digests are recorded in
`PUBLIC_SAFE_RECEIPT.md`. The independent custodian may inspect the sealed
packages and oracles. The implementation agent and public repository may
receive only:

- applicable or out-of-scope classification;
- pass, fail, blocked, unsupported, or invalid result;
- public-safe requirement identifier;
- observed process class and digest; and
- remediation that does not reveal the hidden edit or oracle.

Any disclosed hidden input or oracle reclassifies the fixture and invalidates
its held-out score.

## Stop rule

Scoring MUST stop without changing the implementation or oracle when:

- the cutoff commit differs;
- an environment or command is materially unbound;
- a package digest differs;
- the fixture was exposed to implementation development; or
- scoring would require a capability outside Pulse 01.
