# Pulse 14: Local Profile Evidence Diff

Status: Complete; required corrections and full local validation passed
Implementation authority: Bounded to this document

## Goal and authority

Add one removable local read-only command:

```console
ferris profile-diff --before <PROFILE_JSON> --after <PROFILE_JSON> [--format human|json]
```

The command reads only the two explicit files. It has no workspace identity
and MUST NOT invoke Cargo or another owner tool, contact a network, discover
files, generate a profile, select packages, mutate input, repository, or
environment state, infer compatibility or support, grant approval, expose raw
section values, or create durable state.

## Experimental input contract

The input schema is `ferris.profile-evidence/v0`. It is a development fixture
contract for this pulse, not the canonical PLATFORM-001 profile schema.

Each input contains only:

- `schema`;
- `profile_id`, `revision`, and `consumer`, each 1 to 256 bytes using visible
  ASCII `!` through `~`; and
- `sections` with exactly `identity`, `closure`, `features`, `toolchain`,
  `targets`, `providers`, `native`, `stages`, `assurance`, `stewardship`,
  `support`, and `lifecycle`.

Section values are arbitrary JSON and remain uninterpreted caller evidence.
Explicit `unsupported`, `unavailable`, `not_observed`, `stale`, and `unknown`
values are compared as data and are not promoted into Ferris conclusions.
Every JSON object member name is output-visible JSON Pointer metadata and MUST
use 1 to 256 bytes of visible ASCII `!` through `~`; `/` and `~` are supported
and escaped according to JSON Pointer. Duplicate object members, unknown
top-level fields, and unknown section fields are invalid.

Callers MUST NOT place secrets in `profile_id`, `revision`, `consumer`, or
JSON object member names because those fields are emitted. Section values are
digest-redacted and MUST NOT be emitted raw.

## Result behavior

The command emits `ferris.command-result/v2` containing a
`ferris.profile-diff/v0` record. The record is non-executable and includes:

- a content-based deterministic diff identity;
- before and after `profile_id`, `revision`, `consumer`, and canonical content
  digest references;
- sorted changed and unchanged section names;
- sorted, correctly escaped JSON Pointer changes;
- `added`, `removed`, or `changed` kinds;
- before and after value digests only, never raw section values;
- explicit unknowns and limitations; and
- `executable: false`.

Selection, invocation, diff, and result identities depend on canonical input
content and command semantics after both files are read. Checkout and request
paths do not affect successful identities. Human output is rendered from the
same typed record and exposes every record field without raw section values.
Before content is available, failure identities bind the complete lexically
normalized explicit request paths. If only the first input is read, failure
identity binds its canonical content digest and the complete normalized second
request.

Process results are:

| Condition | Class | Exit |
|---|---|---:|
| Same evidence | `success` | 0 |
| Any difference, including revision only | `difference` | 1 |
| Malformed JSON, invalid shape, empty identity field, profile mismatch, or consumer mismatch | `invalid` | 2 |
| Unsupported input schema | `unsupported` | 4 |
| Missing, unreadable, non-file, or oversized explicit input | `incomplete` | 5 |
| More than 10,000 leaf/path changes | `blocked` | 7 |
| Internal invariant failure | `internal` | 11 |

The `incomplete` classification is deliberate: those failures mean the
explicit caller evidence could not be obtained completely. The change-count
classification follows the existing bounded graph convention and blocks
rather than truncates.

## Bounds

- maximum 1 MiB per explicit input;
- maximum 10,000 emitted leaf/path changes;
- no partial successful diff when a bound is exceeded;
- no input path or raw section value in a successful record; and
- no owner process, network, discovery, mutation, cache, or durable record.

## Acceptance

- identical evidence exits 0;
- section or revision differences exit 1;
- canonical identities survive object-key reordering and path relocation;
- duplicate object members and unsafe output-visible metadata are rejected;
- added, removed, changed, and escaped JSON Pointer cases are exact;
- arrays, index shifts, and added or removed empty containers are retained;
- mismatch, unsupported, malformed, missing, oversized, and overflow cases
  retain typed results and fixed exits;
- distinctive secret-like section values appear in neither JSON nor human
  output, while documented output-visible object keys remain visible;
- human and JSON output derive from the same typed record;
- existing tests remain green; and
- `cargo fmt --all`, `cargo test --locked --workspace`,
  `cargo clippy --locked --workspace --all-targets -- -D warnings`, and
  `git diff --check` pass.

## Stop conditions

Stop instead of widening the pulse if implementation requires Cargo, rustc,
an owner tool, network access, repository discovery, profile generation,
package selection, mutation, a stable PLATFORM-001 schema claim, semantic
interpretation of evidence states, raw-value output, durable state, approval,
support inference, or more than the two existing product crates.

## Support, approval, and removal

This command does not establish compatibility, correctness, security,
freshness, readiness, support, certification, policy compliance, adoption
approval, renewal approval, or release approval. A difference is only a
content difference between two explicit experimental evidence fixtures.

Removal is deletion of the command and its library code plus ordinary
transient Cargo build output. It requires no change to either input, Cargo
metadata, a manifest, lockfile, source tree, owner workflow, or environment.

The completed nine-role implementation review is
[Pulse 14 Nine-Role Implementation Review](../../../../docs/plans/reviews/PULSE-14-ROLE-REVIEW.md).
