# Pulse 17 Public Repository-Selection Binding

Status: Stage A passed; three repository selections frozen
Contract revision: 3
Contract cutoff:
`4371f4f6eb54097bff9badb29278c530d49e2f36`
Workflow ID: `pulse17.public-repository-binding.r3`

## Authority and claim boundary

This document publishes the repository-selection result supplied by the
independent Stage A validation owner. It does not select a different
repository, inspect or construct a hidden change, modify a selected checkout,
execute an owner command, qualify a scorer, construct a sealed package, run
the 112 scored processes, or claim held-out success.

Stage A passed with 789 assertions, 28 LF artifacts, 11 schemas and 69 closed
objects, 41 positive records plus two nested diffs, 38 mutations, 31 vectors
and 161 identities, 59 evidence joins, 10 process archetypes, 112 slots, 26
workflow records, exactly 40 branches, and 48 links. It reported zero public
blockers and preserved first-score integrity.

The machine-readable binding is
[`repository-selections/binding.json`](repository-selections/binding.json).
Each selected record is a complete
`ferris.repository-selection/v1` instance:

- [`hosted.json`](repository-selections/hosted.json);
- [`cross_target_no_std.json`](repository-selections/cross_target_no_std.json);
  and
- [`native_bound.json`](repository-selections/native_bound.json).

## Frozen selections

| Slot | Repository and full commit | License evidence | Eligibility evidence |
|---|---|---|---|
| `hosted` | [`cncf/gitvote@d4bce0e2670cc61ea53f24838366d21eeca0a68a`](https://github.com/cncf/gitvote/tree/d4bce0e2670cc61ea53f24838366d21eeca0a68a) | Apache-2.0; [`LICENSE`](https://github.com/cncf/gitvote/blob/d4bce0e2670cc61ea53f24838366d21eeca0a68a/LICENSE), `sha256:cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30` | Single-package `gitvote` Cargo workspace; deterministic [`Router::oneshot` tests with `MockDB` and `MockGH`](https://github.com/cncf/gitvote/blob/d4bce0e2670cc61ea53f24838366d21eeca0a68a/src/handlers.rs#L364-L396); no live service or account |
| `cross_target_no_std` | [`dalek-cryptography/curve25519-dalek@07bef73ff85998a206cd2cea7f2605c801d0d1c9`](https://github.com/dalek-cryptography/curve25519-dalek/tree/07bef73ff85998a206cd2cea7f2605c801d0d1c9) | BSD-3-Clause; [`curve25519-dalek/LICENSE`](https://github.com/dalek-cryptography/curve25519-dalek/blob/07bef73ff85998a206cd2cea7f2605c801d0d1c9/curve25519-dalek/LICENSE), `sha256:e422d37b97f4f78146b815dae1e1bf311d4de21d195e39837d3be0ba540db4ca` | Exact [`#![no_std]`](https://github.com/dalek-cryptography/curve25519-dalek/blob/07bef73ff85998a206cd2cea7f2605c801d0d1c9/curve25519-dalek/src/lib.rs#L10), target/backend conditions, host tests, and `thumbv7em-none-eabi` check |
| `native_bound` | [`BurntSushi/ripgrep@e89fff89ac9af12e8d4ce9d5fd07beb408ca730f`](https://github.com/BurntSushi/ripgrep/tree/e89fff89ac9af12e8d4ce9d5fd07beb408ca730f) | Unlicense OR MIT; [`UNLICENSE`](https://github.com/BurntSushi/ripgrep/blob/e89fff89ac9af12e8d4ce9d5fd07beb408ca730f/crates/searcher/UNLICENSE), `sha256:7e12e5df4bae12cb21581ba157ced20e1986a0508dd10d0e8a4ab9a4cf94e85c`; [`LICENSE-MIT`](https://github.com/BurntSushi/ripgrep/blob/e89fff89ac9af12e8d4ce9d5fd07beb408ca730f/crates/searcher/LICENSE-MIT), `sha256:0f96a83840e146e43c0ec96a22ec1f392e0680e6c1226e6f3ba87e0740af850f` | `grep-searcher` calls [`memmap2::Mmap::map`](https://github.com/BurntSushi/ripgrep/blob/e89fff89ac9af12e8d4ce9d5fd07beb408ca730f/crates/searcher/src/searcher/mmap.rs#L84); standard host OS memory mapping only |

All three selections freeze `Cargo.toml`, root `Cargo.lock`, default features
with an empty feature suffix, clean detached checkouts, zero submodules, zero
Git LFS objects, Rust and Cargo 1.95.0, and one attempt for every command.

## Required schema derivations

The binding timestamp is
`2026-08-13T12:51:09.420-07:00`. The Windows x86-64 execution setting is
frozen as `x86_64-pc-windows-msvc`, and the recorded locale policy is
`C.UTF-8`. These are normative future execution settings, not claims that a
held-out execution occurred.

Package names come from the full-SHA public Cargo manifests. The native
prerequisite version is the bounded label `host-provided`; it does not claim a
separately installed native package. The native license digests are SHA-256
over the exact public raw file bytes at the selected commit. The Stage A
curve25519-dalek license digest is retained exactly as supplied.

The exact Windows command roots are:

```text
C:\ferris-pulse17\checkouts\<slot>
C:\ferris-pulse17\runs
```

The seven phase maps expand the supplied command templates with those roots.
The schema key and target-directory component for the conceptual
`full-reference` phase are `full_reference`. No shell reinterpretation,
feature suffix, wrapper, shared cache, retry, or post-materialization network
request is permitted.

## Change and comparison policy

Each slot retains the revision-3 one-file, one-logical-change category and the
1 through 16,384-byte unified-diff bound:

- `hosted-observable-behavior`;
- `target-conditional-behavior`; and
- `native-boundary-behavior`.

No path may be added or deleted. The changed path remains sealed and is not
published here.

Selected-versus-full comparison excludes only `/revision`,
`/sections/stages/phase`, and `/sections/lifecycle/state`. Omissions,
promotions, prohibited conclusions, and privacy-canary hits must each equal
zero. This binding does not perform that comparison.

## Public evidence and identity derivation

`binding.json` contains one public Stage A evidence value per slot. Its
`evidence_digest` is recomputed by replacing that member with the empty string
and hashing:

```text
sha256(
  UTF8("ferris.repository-selection-stage-a-evidence/v1") || NUL ||
  compact_sorted_json(evidence)
)
```

Every eligibility assertion and native prerequisite joins to that slot
evidence digest. License, eligibility, command-surface, change-policy, and
selection receipt identities use the revision-3 algorithms in
[`IDENTITY.md`](IDENTITY.md). `record_digest` is SHA-256 over the compact,
sorted-key JSON value of the complete selection record.

| Slot | Selection receipt identity | Record digest |
|---|---|---|
| `hosted` | `sha256:fd771d160af6a296f83ebaf36f3a07a066b3deff5570dd23e41f791fccd19359` | `sha256:e3121b46f74de34f7bf26194744e23c33d1b6b71f80881f74de299ac5ac29fec` |
| `cross_target_no_std` | `sha256:b20f40b1cce19b182cc6a1ed5380d697eff255e6012dbe0cdb977d4acded1cda` | `sha256:86ee9b2a10a71ea931abd612e6e3f2477f48fb1d921348f17f93d00d051d319a` |
| `native_bound` | `sha256:d3fd3233e41694d7d1f83e439f4c7ddf8197c58677d41014bf16312b88357a62` | `sha256:c594688cf3d040213da01f5d166d8fde2770f0a9dc83efe5e471576ed0bda6e4` |

## Stage A observed bounds

| Slot | Files | Checkout bytes | Owner time | Retained output |
|---|---:|---:|---:|---|
| `hosted` | 103 | 2,459,642 | 103,162 ms | stdout 6,505; stderr 10,589 |
| `cross_target_no_std` | 133 | 2,692,985 | host 63,527 ms; cross 8,131 ms | each stream below 8,192 bytes |
| `native_bound` | 222 | 3,205,911 | 20,627 ms | stdout 3,976; stderr 1,149 |

## Work still unperformed

Hidden package construction, scorer qualification, sealed change
construction, repository owner-command workflow execution, and the
112-process first score remain unperformed. No held-out pass is claimed.
