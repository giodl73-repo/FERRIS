# Independent Process-Exit Diagnostic Public Authority Contract

Status: Authorized; unexecuted
Program:
`FERRIS-P34-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-AUTHORITY`
Schema: `ferris.process-exit-diagnostic-public-authority/v1`
Disclosure tier: `sanitized-reproducer` (precommitted before generation)

## Purpose

This contract authorizes one new independent `process-exit-agreement`
diagnostic program. It combines every public gate inherited by Pulse 32 with
the sealed Pulse 33 build-freeze release.

Pulses 22, 24, 26, 28, 30, and 32 remain permanently `invalid`,
permanently non-retryable, and unable to produce category conclusions. Pulse
34 is not their retry, resume, reseed, rescore, reuse, continuation,
correlation, or inference. Every prior category conclusion remains null.

This repository change is governance and test-only. It creates no custody
workspace, package copy, build, environment freeze, seed, generator,
classifier, case manifest, corpus, candidate, process row, pair seal, result,
or reproducer. It MUST NOT invoke a build adapter, verifier, Ferris binary,
diagnostic, or owner command.

## Immutable execution cutoff

Any later execution MUST use Ferris commit
`5df7492fa759c415f6ce540a33a4e89c46714348`.

That cutoff contains the complete Pulse 33 release and all earlier public
gates but does not contain this Pulse 34 authority. Custody MUST independently
verify the commit, verify every Pulse 34 authority artifact is absent, and
launch only binaries frozen from that exact cutoff. A different or
authority-bearing cutoff invalidates the package before adapter preflight.

## Permanently closed programs

| Program | Disposition | Candidate retries | Category conclusion | Permanently closed |
|---|---|---:|---|---|
| Pulse 22 | `invalid` | `0` | null | `true` |
| Pulse 24 | `invalid` | `0` | null | `true` |
| Pulse 26 | `invalid` | `0` | null | `true` |
| Pulse 28 | `invalid` | `0` | null | `true` |
| Pulse 30 | `invalid` | `0` | null | `true` |
| Pulse 32 | `invalid` | `0` | null | `true` |

For every closed program, retry, resume, reseed, rescore, reuse,
continuation, correlation, and inference MUST remain `false`. Pulse 32 also
retains its passed normalization and package facts, failed
`cutoff-build-freeze`, zero downstream activity, null conclusion, and
prohibition on further launches.

## Inherited Pulse 32 public gates

Pulse 34 inherits every Pulse 32 normalization, package, preflight, input
schema, fresh generation, coverage, oracle, search, collection, minimization,
and publication rule without weakening:

- isolated `core.autocrlf=true` cutoff materialization;
- 36/36 `text=set`, `eol=lf`, and LF-byte checks;
- 76/76 Pulse 25, Pulse 27, and collector-identity bindings;
- exact copying of the 20 Pulse 27 manifest-listed files;
- recomputation of 20 file hashes, four aggregates, and six
  report/receipt/seal bindings;
- exactly one adapter invocation, two Windows/Ubuntu pairs, four process
  rows, two pair seals, and two fresh verifiers enforcing `2/2/2`;
- zero adapter, verifier, candidate, and minimization retries and zero
  interrupted-write residue;
- exact immutable-Git-blob binding of the Pulse 31 contract, recursive
  schema, six positive fixtures, and 33 mutation controls;
- 39/39 public-only input classifications before generator or classifier
  freeze;
- wholly new custody identity, workspace, private seed and commitment,
  generator, classifier, manifests, and corpus;
- eight coverage interactions, eight oracle fields, and six target
  predicates;
- 512 unique cases, 512 cases per platform, 1,024 search processes, and one
  search execution;
- at most 128 minimization transformations and 256 minimization processes;
  and
- publication only as a bounded sanitized reproducer with an exact receipt
  and zero-overlap controls, or
  `bounded no-reproduction; no fix authority`.

Ferris production source and tests remain outside the generator and
classifier read scope. A digest, classification, scope, self-validation,
normalization, package, or adapter-preflight failure closes the package at
its declared invalid stage and prohibits later gates.

## Sealed Pulse 33 release binding

Custody MUST obtain the Pulse 33 release from the immutable cutoff and copy
exactly its 37 manifest-listed files into a new isolated build-freeze
workspace. The supplied platform-native bytes MUST be preserved; the release
root's `binary` attribute is authoritative.

The exact release identities are:

- manifest path:
  `docs/simulations/profile-diff-held-out/pulse-33-build-freeze-release/public-manifest.json`;
- manifest raw SHA-256:
  `sha256:9082bb18ab72e5e5ced2ec43811ecc5ce14ac43c9cd2878e4db0f10bf7a741fd`;
- manifest size: 7,746 bytes;
- manifest file count: 37;
- manifest total bytes: 59,895;
- aggregate algorithm: `sha256-length-path-filedigest-v1`;
- 37-file aggregate:
  `sha256:07df7bc02cab288adccbdc0f87e45f3fd52939ebc9d22c8d064f773843e861a4`;
- release-seal raw SHA-256:
  `sha256:057f6dea59665401331b29ad984e203cca474143d7576a6617588922bf678cbd`;
- release-seal payload SHA-256:
  `sha256:7ebb70ddc2a610b8c7638f30d03d0707b7d00c3eabe56ab679f085d7035f109a`;
- qualification receipt raw SHA-256:
  `sha256:84c09348fe1af7c639510d4ca175bdde0eed51a27a3e2e6f2b80414c80fc10a0`;
- qualification payload SHA-256:
  `sha256:0e64090a6fa7cddfa44e63f7a6be7963498dfc9f34ef15fa1c290fa73dbac48e`;
- root-cause report raw SHA-256:
  `sha256:9c299af5548a5df004676c1dd79108d76ea0774861f8bc4d0758d44fd7a1e16b`;
  and
- root-cause payload SHA-256:
  `sha256:e72921f8433d2a787c9142ad056bc5beff05f71836a0ab38b7fad90797d2babc`.

Custody MUST independently recompute every manifest file, the aggregate, the
seal, and the bound report and receipt envelopes before using the adapter.
Repair, regeneration, normalization, substitution, or an extra copied file
is prohibited.

## Exact build adapter and public receipts

The only authorized build adapter is
`pulse-33-build-freeze-release/build_freeze.py`, 12,300 bytes, raw SHA-256
`sha256:43bb31210175ceacba2431a238608d9973672a08de57572543ad0f9dae41cbe6`.

It MUST:

1. resolve the Cargo executable from `PATH`, then explicitly check
   `$HOME/.cargo/bin/cargo` for a WSL non-login shell;
2. record the resolved Cargo executable path;
3. run a locked release build for package `ferris-cli`, binary `ferris`;
4. request Cargo `--message-format=json-render-diagnostics`;
5. discover the executable only from Cargo `compiler-artifact` JSON;
6. prohibit target-directory path guessing;
7. hash and size the Cargo-reported executable; and
8. never execute the Ferris binary or modify product files.

The sealed public qualification receipts are:

| Platform | Raw receipt SHA-256 | Payload SHA-256 | Frozen binary |
|---|---|---|---|
| Ubuntu 24.04 WSL2 x86_64 | `sha256:23e4f56dc26be96adc140f5a1aa181389a8cdcd8497ca30fc47c15763dfc91c0` | `sha256:b01e45259e340309772e1d5d5c947cff163ada8dad8b9ddf19775f0a537c4cae` | 1,945,448 bytes; `sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4` |
| Windows x86_64 MSVC | `sha256:3d1624d02fc5784a7b3daab9403123b377761bc8f63ec3d46aea7411ca460622` | `sha256:b1d42470ca709406c5869bae9e677334539e745faf8e0f400e4ae93f34cf7d7a` | 1,436,672 bytes; `sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8` |

These receipts qualify the adapter against the earlier Pulse 32 cutoff. They
do not substitute for freezing the Pulse 34 cutoff.

## Mandatory Pulse 34 cutoff build freeze

After normalization, package verification, and Pulse 33 release verification,
but before the inherited adapter preflight, custody MUST use the exact
adapter to build commit
`5df7492fa759c415f6ce540a33a4e89c46714348` independently on:

- Windows x86_64 MSVC, producing `ferris.exe` with
  `CARGO_INCREMENTAL=0` and `RUSTFLAGS=-C link-arg=/Brepro`; and
- Ubuntu 24.04 WSL2 x86_64 in a non-login shell, producing `ferris` with
  `CARGO_INCREMENTAL=0` and explicit Cargo fallback when required.

Both fresh clean checkouts, both resolved Cargo paths, both Cargo JSON
artifact records, both exact executable filenames, both byte sizes, both
SHA-256 digests, and both build receipts MUST be frozen. The exact required
cardinality is two platforms, two binaries, and two receipts. Public
executable retention is prohibited.

Any build failure, missing or ambiguous Cargo artifact, guessed executable
path, missing explicit Cargo discovery, platform-name drift, receipt
mismatch, diagnostic execution, or incomplete exact binary freeze closes the
program `invalid-before-adapter-preflight`. No inherited preflight,
generation, or candidate may run.

## Declaration and mutations

The authorized declaration identity is
`sha256:8975e07b9dd417604d06be12a24a448e8ae1834991aca9db086ae7c11b0b1e34`.
The declaration is `authorized-unexecuted`; every copy, verification, build,
preflight, generation, candidate, retry, and conclusion field remains at its
zero, false, or null initial value.

The public mutation suite contains exactly 704 rejection controls. It
inherits every Pulse 32 control and adds exact mutations for Pulse 32 closure,
Pulse 33 manifest/aggregate/seal/adapter/receipt bindings, WSL non-login Cargo
discovery, Cargo JSON artifact discovery, two-platform binary freeze,
premature activity, result fields, unknown members, and identity.

## Custody handoff

The declaration is ready only for a new independent validation custodian. It
authorizes one bounded search execution after every public gate passes. It
does not select a custodian, create a workspace, freeze a build, freeze a
seed, or execute anything.

## Stop conditions

Stop rather than widen if work would use another cutoff; place authority in
the cutoff; reopen a closed program; bypass any inherited Pulse 32 gate;
alter the sealed Pulse 33 release; omit explicit Cargo discovery or Cargo JSON
artifact output; guess an executable path; accept fewer than two exact
platform binaries and receipts; read Ferris source/tests for generation or
classification; access prior custody or hidden material; retry a candidate;
execute under this repository change; or alter production behavior, any
closed result, or PLATFORM-001 status.
