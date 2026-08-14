# Pulse 35 Public Corpus-Materializer Release Nine-Role Review

Date: 2026-08-14  
Disposition: Accept after semantic-coverage correction  
Implementation authority: Public release source, closed machine contract,
governance, and test-only validation only

## Review question

Does Pulse 35 prospectively provide a deterministic public-rule
`ferris.profile-evidence/v0` corpus materializer that closes the Pulse 34
generation-materialization gap without reopening Pulse 34, using private
custody material, executing a diagnostic or FERRIS candidate, changing product
code, or authorizing a new diagnostic?

## Role dispositions

| Role | Disposition | Boundary |
|---|---|---|
| Rust Safety Steward | Accept | Python stdlib and test-only Rust validation add no unsafe or production Rust; accepted syntax is not a behavioral correctness claim |
| Compiler Performance Engineer | Accept | Twenty synthetic cycles establish deterministic materialization only, not a benchmark, build measurement, or performance claim |
| Interop Boundary Auditor | Accept | Closed concrete descriptors bind role state, artifacts, bytes, host-independent request resolution, result posture, independently recomputed exact tuple witnesses, the 512 maximum, and one-attempt publication/rollback |
| AI Assurance Skeptic | Accept | The sole Pulse 34 fact remains visible; no private causal inference, corpus disclosure, candidate result, score, or success-shaped diagnostic claim is made |
| Ecosystem Strategist | Accept | The adapter preserves Cargo and Ferris ownership, adds no resolver, dependency, registry, network, credential, or product integration |
| Rust Maintainer | Accept | `crates/ferris-cli/src` is unchanged; removal deletes public release/governance/test records without changing CLI, API, output, or exit behavior |
| Native Platform Adopter | Accept | Exact 32-byte CSPRNG seed input is also mandatory for verification; keyed HMAC derivation, UNC-preserving resolution, one replacement, recorded synced/unsupported directory status, final-sync rollback, re-entry, and no extra/residual output are checked |
| Scope Keeper | Accept | One prospective infrastructure release closes materialization only; Pulse 34 stays immutable and no new diagnostic authority is created |
| Validation Checker | Accept | Unit and Rust tests check private-seed verification, independent classifier/exact-tuple derivation, semantic-tamper rejection, all exact value and interaction catalogs, 20 cycles, seed length, same/different seed, staging/publish sync failures, zero logical retries, and privacy |

## Shared findings

All nine roles record:

- the sealed manifest and 20-cycle qualification identities recorded in the
  current release records;
- derived coverage of the 17 exact Pulse 34 value domains plus the
  interaction-requirements domain (`18/18`) and all eight exact tuple
  catalogs (`8/8`), with an inherited maximum of 512 logical cases and exact
  complete corpus count 70;
- exact binding to the nine Pulse 31 public artifacts and the Pulse 34 public
  authority/result only;
- strict public precedence for state/size/UTF-8/JSON/duplicates/keys/schema/
  shape/identity plus concrete raw witness derivation before publication;
- atomic staging, recorded `synced`/`unsupported` directory statuses, one
  replacement with zero logical retries, post-creation cleanup, confirmed
  rollback after final-sync failure, and rejection of pre-existing, partial,
  extra, replayed, and residue-bearing output;
- no FERRIS, diagnostic, candidate, owner-command, network, product-file,
  private-custody, hidden-corpus, or private-seed activity;
- no seed bytes or private paths in manifests, receipts, reports, or seals;
  published profile IDs/tokens are explicitly documented HMAC-derived
  pseudorandom values rather than claimed seed-independent; and
  and
- no score, certification, support, fix, native-platform, or PLATFORM-001
  authority.

## Decision

All nine roles accept the bounded release. It enables a future independent
custodian to materialize fresh public-rule inputs prospectively, but it does
not alter or resume Pulse 34 and does not authorize diagnostic execution.
