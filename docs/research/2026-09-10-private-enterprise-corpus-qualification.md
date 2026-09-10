# Private Enterprise-Shape Corpus Qualification

Date: 2026-09-10
Status: Incomplete; stopped before Ferris scenario execution
Evidence ID: FERRIS-EVIDENCE-002

## Question

Does the current integrated Ferris revision preserve every already-declared
scenario across the five-family private enterprise-shape synthetic corpus on
Windows and hosted Ubuntu?

The bounded authority and stop rules are recorded in
[`Pulse 01`](../../context/waves/2026-09-10-enterprise-corpus-qualification/pulses/pulse-01.md).
Private repository identities, revisions, paths, raw output, hashes, and
identifiable timings remain outside this public repository.

## Frozen scope

- Ferris revision:
  `e66795f4292a9e23a95fffc13a17bece8f5fff61`.
- Five exact private corpus revisions, represented publicly only as `EC-01`
  through `EC-05`.
- Existing immutable scenario tags and repository-owned validation commands.
- Local Windows followed by hosted Ubuntu only if every Windows owner-integrity
  gate passed.
- No product, schema, dependency, scenario, or owner-command change.

## Preflight result

The release binary built successfully with stock Rust and Cargo 1.95.0. The
first owner-preflight attempt was invalid because a globally configured compiler
cache restored artifacts produced by a different Rust compiler identity. No
Ferris scenario had started. The one authorized infrastructure correction
disabled that wrapper and cleaned only the private custody build trees.

| Family | Corrected Windows owner preflight |
|---|---|
| `EC-01` | Formatting, Clippy, and locked/offline owner tests passed |
| `EC-02` | Formatting, Clippy, and locked/offline owner tests passed |
| `EC-03` | Formatting, Clippy, and locked/offline owner tests passed |
| `EC-04` | Formatting, Clippy, and all 96-package locked/offline owner tests passed; deterministic generated-topology integrity failed |
| `EC-05` | All sixteen workspace formatting, Clippy, locked/offline owner tests, and generated-corpus integrity passed |

`EC-04` regenerated a `TOPOLOGY.json` file whose bytes did not match the
checked-in owner artifact. The repository-owned verifier failed closed. This
report does not infer whether the difference is newline normalization,
serialization behavior, generator drift, or checked-in artifact drift because
the pulse did not authorize a corpus correction or another diagnostic campaign.

## Decision

The qualification is incomplete:

- zero Ferris scenario plans ran;
- zero deterministic plan comparisons ran;
- zero selected/full owner comparisons ran;
- zero typed-rejection cases ran;
- zero hosted Ubuntu jobs ran; and
- no temporary hosted-validation branch was created.

The private corpus remains a strong candidate for primary Ferris regression and
demonstration coverage, but current-main qualification is not established.
Owner-fixture integrity correctly precedes product evaluation.

## Next gate

Any continuation requires separate user approval for:

1. a bounded owner-side investigation and correction of the `EC-04`
   deterministic generator or checked-in topology contract;
2. independent Windows and Ubuntu proof that the corrected owner-integrity
   contract is stable; and
3. a new qualification pulse using a cutoff that contains that completed
   correction.

No product fix, corpus correction, rerun, affected-only gating, support,
production-representativeness, or savings claim follows from this result.
