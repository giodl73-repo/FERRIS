# Pulse 19: Public Process-Exit Agreement Diagnosis

Status: Complete; public diagnosis found no reproduction
Implementation authority: New public/development fixtures and one test-only
diagnostic harness only

## Goal

Diagnose, without fixing, the valid Pulse 17
`process-exit-agreement` failure using only newly authored public synthetic
fixtures and development evidence.

The exact diagnostic question is:

> Across the frozen public `profile-diff` branch types, at which earliest
> public layer, if any, does the declared `ResultClass` stop agreeing with the
> core classification, diagnostic classification, command envelope,
> `process_exit_code`, actual operating-system exit, required JSON stream
> route, or equivalent human-format result?

The only permitted localization outcomes are:

1. core classification;
2. envelope construction;
3. CLI emission/`ExitCode`;
4. format parity; or
5. no reproduction.

## Bounded authority

A later implementation commit under this pulse MAY add:

- one removable Rust integration-test diagnostic harness;
- one adjacent set of committed, non-secret, public synthetic fixture inputs
  and an exact public branch manifest;
- in-process observations through the existing public
  `create_profile_diff`, `CoreError`, `Diagnostic`,
  `profile_diff_error_envelope`, and `CommandEnvelope` surfaces;
- black-box launches of the existing `ferris profile-diff` binary; and
- one Windows and Ubuntu development-validation receipt.

The implementation MUST NOT change production Rust, CLI parsing, output
bytes, stream selection, `ResultClass`, exit mappings, visibility, APIs,
dependencies, schemas, or product behavior. If an existing public surface is
insufficient to observe a layer, the pulse stops rather than widening product
code.

This pulse grants no fix authority. Any correction, even a one-line exit or
emission change, requires a separately reviewed and approved later product
pulse.
PLATFORM-001 remains Draft.

## Frozen result-class map

The diagnostic harness MUST use this public map and no inferred hidden
expectation:

| Result class | Exit |
|---|---:|
| `success` | 0 |
| `difference` | 1 |
| `invalid` | 2 |
| `unsupported` | 4 |
| `incomplete` | 5 |
| `blocked` | 7 |

For every JSON process:

```text
actual OS exit == emitted process_exit_code == ResultClass map
```

For every non-success JSON process, the one diagnostic's `result_class` MUST
also equal the envelope `result_class`. Success and difference envelopes MUST
have no diagnostics. A mismatch is evidence; the harness MUST NOT normalize
or repair it.

## Required public diagnostic matrix

The fixture manifest MUST declare exactly 23 unique public input branches.
Each platform MUST execute exactly 26 CLI processes: the 23 JSON rows plus
three human-format equivalents. Windows and Ubuntu therefore produce exactly
52 CLI process rows, with no missing, duplicate, retried, or extra row.

| ID | Public fixture branch | Format | Expected class / exit | Required route |
|---|---|---|---|---|
| `J01` | Canonically identical evidence | JSON | `success` / 0 | stdout only |
| `J02` | One bounded revision or section difference | JSON | `difference` / 1 | stdout only |
| `J03` | Duplicate top-level member | JSON | `invalid` / 2 | stderr only |
| `J04` | Duplicate nested member | JSON | `invalid` / 2 | stderr only |
| `J05` | Malformed JSON | JSON | `invalid` / 2 | stderr only |
| `J06` | Unknown top-level field | JSON | `invalid` / 2 | stderr only |
| `J07` | Unknown section field | JSON | `invalid` / 2 | stderr only |
| `J08` | Invalid profile identifier metadata | JSON | `invalid` / 2 | stderr only |
| `J09` | Invalid revision metadata | JSON | `invalid` / 2 | stderr only |
| `J10` | Invalid consumer metadata | JSON | `invalid` / 2 | stderr only |
| `J11` | Invalid output-visible object-key metadata | JSON | `invalid` / 2 | stderr only |
| `J12` | Mismatched profile identities | JSON | `invalid` / 2 | stderr only |
| `J13` | Mismatched consumers | JSON | `invalid` / 2 | stderr only |
| `J14` | Unsupported schema | JSON | `unsupported` / 4 | stderr only |
| `J15` | Missing first input | JSON | `incomplete` / 5 | stderr only |
| `J16` | Missing second input | JSON | `incomplete` / 5 | stderr only |
| `J17` | Non-file first input | JSON | `incomplete` / 5 | stderr only |
| `J18` | Non-file second input | JSON | `incomplete` / 5 | stderr only |
| `J19` | Oversized first input | JSON | `incomplete` / 5 | stderr only |
| `J20` | Oversized second input | JSON | `incomplete` / 5 | stderr only |
| `J21` | Exactly 10,000 emitted changes | JSON | `difference` / 1 | stdout only |
| `J22` | More than 10,000 changes | JSON | `blocked` / 7 | stderr only |
| `J23` | Public metadata plus raw-value privacy canaries | JSON | `difference` / 1 | stdout only |
| `H01` | Exact `J01` inputs | Human | `success` / 0 | stdout only |
| `H02` | Exact `J02` inputs | Human | `difference` / 1 | stdout only |
| `H03` | Exact `J23` inputs | Human | `difference` / 1 | stdout only |

Every JSON route MUST contain exactly one complete UTF-8 command envelope
followed by one LF, with the opposite stream empty. Successful envelopes MUST
contain a profile-diff record. Non-success envelopes MUST contain a null
record and exactly one diagnostic.

The three human rows MUST use byte-identical inputs to their JSON partners.
Their actual exits and stream routes MUST equal the paired JSON class map.
Human success and difference MUST preserve the same public profile metadata,
changed and unchanged sections, change paths and kinds, and value digests
expressed by the JSON record. `J23` and `H03` MUST expose the same permitted
metadata and escaped pointer while omitting every raw-value canary.

## Localization rules

Each unique input branch MUST be classified at its earliest divergent layer:

| Outcome | Required observation |
|---|---|
| Core classification | `create_profile_diff` or `CoreError` produces a class different from the manifest, or a `CoreError` diagnostic class differs from its error class |
| Envelope construction | Core classification agrees, but the envelope class, diagnostic class, record presence, or `process_exit_code` differs |
| CLI emission/`ExitCode` | The envelope agrees in process, but the black-box OS exit, emitted envelope, newline, or stdout/stderr route differs |
| Format parity | JSON agrees, but an equivalent human row differs in class-mapped exit, stream route, public semantics, or privacy behavior |
| No reproduction | Every required observation and process row agrees on both recorded platforms |

The receipt MUST report one of these outcomes per branch and one aggregate
outcome. It MUST NOT speculate about which hidden Pulse 17 case failed.

## Windows and Ubuntu validation

The future implementation MUST validate one immutable cutoff on:

- Windows x86-64; and
- Ubuntu 24.04.4 WSL2 x86-64.

Each environment MUST use the same public fixture revision and record the
exact Git cutoff, Rust and Cargo versions, executable digest, fixture-manifest
digest, argv, working directory, actual exit, complete stdout and stderr
digests, parsed envelope fields, diagnostic class, record presence, stream
route, and localization outcome.

Ubuntu WSL2 evidence is development evidence only. It is not native Linux,
support, compatibility, certification, or held-out evidence.

## Success and failure stop conditions

The diagnostic pulse succeeds in exactly one of two ways:

1. a public branch reproducibly localizes a mismatch to one permitted layer,
   after which work stops without a fix; or
2. all 23 input branches, 26 per-platform process rows, and three format pairs
   agree on both platforms, producing the bounded outcome `no reproduction`.

Stop and mark the diagnostic evidence incomplete or invalid if:

- a fixture, manifest, parser, process launch, stream read, or cardinality
  check fails;
- a platform cannot execute the same cutoff;
- a row is missing, duplicated, retried, or added;
- raw canary data appears in output;
- localization would require a production code, API, dependency, schema, or
  behavior change;
- a result can be obtained only by accessing or inferring hidden material; or
- anyone attempts to turn this development diagnosis into a held-out pass,
  rescore, support claim, or PLATFORM-001 advancement.

## No-hidden and no-retry boundary

Fixture `P17-R3-D6B553CBC3B1240B673B8190` and all custody artifacts remain
permanently closed in quarantine. They MUST NOT be accessed, retried,
rescored, reused, reconstructed, or correlated with the new public fixtures.
The aggregate counts and `process-exit-agreement` label MUST NOT be used to
infer a hidden case, input, expected exit, or scorer predicate.

Each recorded Pulse 19 validation run launches each declared public row once
and has no internal retry, fallback variant, or favorable-result selection.
Ordinary development reruns of the new public test suite are not held-out
retries, but every retained receipt MUST identify a fresh run and preserve
the failed run rather than overwrite it.

## Evidence and review

- [Frozen public contract](../../../../docs/simulations/profile-diff-held-out/PUBLIC_CONTRACT.md)
- [Valid public-safe Pulse 17 result](../../../../docs/simulations/profile-diff-held-out/PUBLIC_SAFE_RESULT_P17_R3_D6B553CBC3B1240B673B8190.md)
- [Pulse 19 pre-implementation review](../../../../docs/plans/reviews/PULSE-19-PROCESS-EXIT-DIAGNOSTIC-ROLE-REVIEW.md)
- [Public diagnostic harness](../../../../crates/ferris-cli/tests/process_exit_diagnostic.rs)
- [Frozen public branch manifest](../../../../crates/ferris-cli/tests/process_exit_diagnostic_manifest.json)
- [Windows development receipt](../../../../docs/plans/validation/PULSE-19-PROCESS-EXIT-DIAGNOSTIC-WINDOWS.json)
- [Ubuntu WSL2 development receipt](../../../../docs/plans/validation/PULSE-19-PROCESS-EXIT-DIAGNOSTIC-UBUNTU-WSL2.json)

## Implementation result

The removable integration harness declares exactly `J01` through `J23` and
`H01` through `H03`. It generates only new public synthetic inputs under an
isolated repository target directory, observes the existing public core and
envelope surfaces, and launches the existing `ferris` executable once per
declared row with no retry.

Windows x86-64 and Ubuntu 24.04.4 WSL2 x86-64 executed the same public source
cutoff:

```text
git:0c5db524b1c6f1c5505f1362bb46aac9dd2985aa+public-source:sha256:6784a6951caae67d75b2587f6f4ae5ba045d50215d803ca0c6691ff13565b137
```

Each platform retained exactly 26 complete process rows: 23 JSON and three
human. Both receipts record 26 started and retained processes, zero retries,
23 branch localizations, zero divergent branch IDs, and aggregate
`no-reproduction`. The fixture-manifest digest is identical across the two
platform receipts.

Formatting, workspace check, the targeted diagnostic, the workspace suite,
and Clippy with warnings denied passed on Rust and Cargo 1.95.0 in both
environments. Windows also owns the Markdown and Git working-tree hygiene
checks for this checkout.

This is bounded public development evidence only. It does not alter, retry,
rescore, reuse, reconstruct, or explain the closed Pulse 17 result. No public
branch localized a product mismatch, so no product-fix pulse is warranted by
this diagnosis. Pulse 20 is a separate prospective governance protocol.
PLATFORM-001 remains Draft for its independently recorded blockers.
