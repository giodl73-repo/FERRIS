# Pulse 32: Independent Public-Input Diagnostic Authority

Status: Complete; authority published and unexecuted
Implementation authority: Governance, public fixtures, review, and test-only
validation only

## Goal

Authorize one new independent `process-exit-agreement` diagnostic program
using the normalized Pulse 25 collector, Pulse 27 exact-two-pair adapter, and
only the public Pulse 31 `ferris.profile-evidence/v0` input rules.

Pulses 22, 24, 26, 28, and 30 remain permanently invalid, non-retryable, and
unable to produce category conclusions. Pulse 32 is not their retry, resume,
reseed, rescore, reuse, continuation, correlation, or inference.

## Immutable cutoff

Any later execution is bound to
`29517d732db13cc2ffa304684b344f3538ab587d`.

The cutoff contains the complete Pulse 31 public input artifacts and the
normalized Pulse 25/Pulse 27 infrastructure, but it does not contain this
authority. The authority is later.

## Public input binding

Custody must obtain the Pulse 31 artifacts as exact immutable-cutoff Git
blobs. The normative input contract raw digest is
`sha256:26fdb4b9eed558f1f03a66eaec13749bfbad7ea4612c6f7e58bb8e7b79e69295`;
the recursive schema raw digest is
`sha256:67946b1a392d2d7537d487d343bee31439606c76b2d71862b97ff46641c3d62b`.

The declaration additionally pins all six positive fixtures by path, byte
length, and raw digest, plus the 33 mutation controls by:

- mutation-file raw digest
  `sha256:b33985e51f54c2ed0121b94571b622ee47bbd00450c8ab1c3d65d0f463276158`;
- exact control ID; and
- per-control `sha256-canonical-json-sort-keys-v1` digest.

## Mandatory gates

Pulse 32 inherits every Pulse 30 normalization, package, adapter preflight,
freshness, coverage, oracle, search, collection, minimization, and
publication rule without weakening:

- 36/36 LF release files and 76/76 normalized bindings before copy;
- exact 20-file package and complete digest recomputation;
- one adapter invocation, two pairs, four rows, two seals, and two fresh
  verifiers enforcing `2/2/2`;
- zero retries and zero residue;
- eight coverage interactions, eight oracle fields, and six target
  predicates;
- 512 cases per platform, 1,024 search processes, and one search execution;
- at most 128 minimization transformations and 256 minimization processes;
  and
- sanitized-reproducer or
  `bounded no-reproduction; no fix authority` publication.

After adapter preflight and before generation, custody must verify the
contract, schema, six positive fixtures, mutation file, and all 33
per-control digests. It must then independently self-validate all six
positive acceptances and all 33 exact negative classifications using only
those public artifacts.

Ferris production source and tests are outside the authorized read scope.
The generator and classifier must use only the public Pulse 31 rules. A
digest, classification, scope, or self-validation failure closes the package
`invalid-before-generation` and prohibits generation and candidate launch.

## Evidence

- [Normative contract](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PUBLIC_INPUT.md)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-public-input.v1.schema.json)
- [Authorized declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-input.json)
- [Mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-public-input-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-32-PUBLIC-INPUT-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_public_input.rs)

Declaration identity:
`sha256:88bdbd263fed865e94d16cbd0e6f78a2f330cdae5788f7d7bf93c51afd758812`

The mutation suite contains 538 controls. No preflight, self-validation,
generation, candidate, minimization, result, or publication was executed by
this pulse.

## Decision

The authority is complete and unexecuted. One new independent custodian may
later execute the bounded program against the exact cutoff after every gate
passes. No production, score, certification, support, fix, closed-result, or
PLATFORM-001 status authority follows.

## Stop conditions

Stop rather than widen if work would execute this program, use another
cutoff, include authority in the cutoff, bypass an inherited gate, read
Ferris source/tests for generation or classification, access prior custody or
hidden material, retry or infer from a closed program, weaken public input
bindings or inherited bounds, or change PLATFORM-001 status.
