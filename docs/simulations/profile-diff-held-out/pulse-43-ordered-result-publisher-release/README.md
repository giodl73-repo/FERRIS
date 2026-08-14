# Pulse 43 ordered public-result publisher release

This public, standard-library-only Python release closes the two public
publication defects exposed by Pulse 42. It publishes one complete result
directory only after an explicit ordered execution record, staged result and
receipt verification, one rename, final raw/payload-hash verification, and
an honest final-parent directory-sync posture.

It is not diagnostic authority, custody, private-data access, a product or
category conclusion, or a fix conclusion. It does not execute FERRIS, Cargo,
or any candidate. The only inputs are bounded public gate catalog and event
records.

## Frozen public controls

`public-gate-catalog.json` predeclares the execution order. Each event must
declare exactly one stable classification:

- `public-artifact-self-validation` with `validation-complete`; or
- `ordered-execution` with `gate-complete` or `terminal-stop`.

The catalog, each event variant, every field, identifier grammar, collection
size, and self-validation count are closed and bounded by
`schemas/ferris.pulse-43-ordered-result.v1.schema.json` and enforced again by
the adapter. Unknown fields, duplicate JSON members, identifiers containing
privacy-bearing components, path-like strings, duplicate catalog or execution
gates, unknown gates, and unbounded counters are rejected.

Self-validation has its own `completed_checks` and `expected_checks` summary.
It cannot create, satisfy, or advance an execution gate. Ordered execution
must consume the predeclared catalog in order and has one terminal
`terminal-stop`. A `completed` terminal must finish the catalog; a `failed` or
`stopped` terminal records its current gate. A later ordered-execution record
is rejected. Therefore an early Pulse 33 stop cannot share a valid result
with later Pulse 31 or Pulse 35 execution counts.

## Publication protocol

`ordered_result_publisher.py` creates an absent sibling staging directory,
writes and flushes/fsyncs `public-result.json` and `release-receipt.json`,
verifies their exact two-file shape, canonical payload hashes, raw hashes, and
receipt-to-result binding, synchronizes the stage, then uses exactly one
`os.replace`. It reconstructs and verifies the final directory independently
before synchronizing its parent and returning a `published` summary.

Each summary is deterministic compact JSON and contains no input path. A
publication failure contains only an explicit `absent`, `rolled-back`, or
`indeterminate` posture, a failure code, one-rename/zero-retry accounting,
and sync posture; it has no execution summary or success-shaped output.
Post-rename verification or final-parent-sync failure removes the final tree.
Rollback is only `rolled-back` after final-path absence plus a `synced` or
explicitly `unsupported` parent sync. Otherwise it is
`P43-INDETERMINATE-PUBLICATION`.

Directory synchronization uses the Pulse 41
`os.open+os.fsync-directory-v1` posture: `synced` is a durability operation;
`unsupported` is an explicit Windows/filesystem portability result, never a
durability claim. The Windows qualification observed `unsupported` for both
stage and final-parent attempts.

Run from any working directory:

```console
python ordered_result_publisher.py --catalog C:\public\catalog.json --events C:\public\events.json --final-root C:\public\published-result
```

The release tree contains nine files: six manifest payload files plus the
manifest, qualification receipt, and seal. The manifest raw/aggregate,
receipt raw/payload, and seal raw/payload identities are fixed in the
repository Rust validator and Pulse 43 records.

Qualification has 18 Python test methods, including 20 isolated publication
cycles and controls for Pulse42-shaped late events, self-validation separation,
schema/privacy/bounds rejection, copy, stage-sync, rename, missing-final,
final-verification, final-sync, rollback-removal, and rollback-sync failures.
The Rust integration test recomputes release identities and exercises complete,
early-stop, self-validation, and late-event mutation controls.
