# Synthetic Corpus Contract Requirements

Date: 2026-09-22

Status: Controlled compatibility-gate fixture

## Question

Can one explicit requirement set capture the exact Ferris schema handling used
by the retained synthetic repositories without inspecting those repositories at
runtime or implying that synthetic evidence is production adoption?

## Observed inputs

| Repository | Revision | Observed contract use |
|---|---|---|
| `ferris-synthetic-boundaries` | `9a12c0bf3e46018f11a5ec25d8111bf894377bbd` | supplies owner domains V1 to Ferris; records validation-plan V0 output |
| `ferris-synthetic-chain` | `b3d38baf47c8477b4d07ad71b24d6e3c0d71784c` | supplies owner domains V1 to Ferris; records validation-plan V0 output |
| `ferris-synthetic-fanout` | `ffcfbf07d40e888328acddc2ffe7c1b80e24ebb1` | supplies owner domains V1 to Ferris; records validation-plan V0 output |
| `ferris-synthetic-federated` | `70d980d11c947778ba806a1e6c44dca0bbcc39c9` | supplies application V0 to Ferris; records federated-validation-plan V0 output |
| `ferris-synthetic-scale` | `577ac081ab990ec972d3750ba082525c8c02eb17` | supplies owner domains V1 to Ferris; records validation-plan V0 output |

The normalized requirement set is checked in at
`tests/fixtures/contracts/synthetic-corpus-v1.json`. It deliberately contains
only four unique direction-qualified requirements after deduplication across
the five repositories.

## Result and boundary

The current Ferris binary satisfies all four requirements. This establishes a
replayable first release gate for the controlled corpus. It does not establish
a multi-release window until a later distinct Ferris release passes the same
fixture, and it does not compare payload semantics or replace repository tests.
