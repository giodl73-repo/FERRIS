# Pulse 47 public publication-outcome witness release

This public, standard-library-only wrapper makes one already-returned Pulse 43
publication outcome persistently inspectable without reopening Pulse 46 or
accessing private diagnostic material. It is a publication witness release,
not diagnostic authority, execution, custody, a score, a conclusion, or a
remedy.

## Fixed predecessor

The wrapper verifies the exact Pulse 43 manifest raw/aggregate identities
`sha256:8eaca6ebc350a67e493d037132a27749980c16fc79143d69fd0303305b5030a4` /
`sha256:74f6c61913fbfa638f0ba6aae19cb0d2885a47e38b33b19bb80bec1abc870346`,
qualification receipt raw/payload identities
`sha256:3ebc1bfd95dfbfedd1402bb3f3f9f14ea872aec9137a7327b8ca444248091e0c` /
`sha256:9e713bb8f12deced2119fe66028a4c2ab11d6d70d6d0fe90342b996bc1bf25a2`,
release seal raw/payload identities
`sha256:4445a0b181419b303c28f1d91e1700a594d2a040fbae1cd0dc526fa7fc8e8f05` /
`sha256:b51b408cb7e93fccc3c4f92e1d29956f3c107e500a4e947548273ca01ea525a1`,
and publisher source
`sha256:38ebc7ce84ae29c2ad20ada593d8baeb0352b59e7c48438c4a9c224a0ea4a6c6`.

`witness_pulse_43` accepts the Pulse 43 catalog, events, fresh absent Pulse
43 final root, and a separate fresh absent absolute witness final root. Its
injected callable is invoked exactly once after the witness root is accepted.
The CLI first verifies and imports that exact predecessor, then passes the
public catalog/events to it once:

```console
python publication_outcome_witness.py --catalog C:\public\catalog.json --events C:\public\events.json --p43-final-root C:\public\p43-result --witness-final-root C:\public\publication-witness
```

## Closed public boundary

The callable's entire returned Pulse 43 summary is validated as one of its
closed success or failure shapes before any witness byte is made. A published
Pulse 43 result witnesses only its public result raw/payload hashes, final
files `2/2`, one rename, zero retries, exact stage/final-parent/rollback-parent
sync posture, and ordered/self-validation aggregate summaries. Terminal gate
identifiers and all input paths are omitted.

An absent, rolled-back, or indeterminate Pulse 43 failure witnesses only its
failure code, final-files absence, state, rename attempts, retries, and all
three exact sync postures. It contains no ordered-event, gate-count,
self-validation, private, executable-byte, or input-path data. Thrown,
partial, malformed, or success-shaped incomplete predecessor values fail
closed before witness publication.

## Witness transaction

The release canonicalizes and hashes the closed witness payload and writes
exactly `publication-witness.json` plus `release-receipt.json` into an absent
sibling stage. Both files are flushed and fsynced before close; exact
two-file shape, duplicate-free JSON, payload/raw hashes, and receipt binding
are verified before one directory sync and one rename. The final directory is
independently re-opened and rehashed before final-parent sync. There are zero
retries and no fallback.

No external success summary exists before final witness `2/2` verification.
If the witness transaction fails, its output has only the bounded witness
failure code and absent/rolled-back/indeterminate witness posture with
rename/retry/final-files/sync fields. It omits every captured Pulse 43 detail.
Rollback is reported only after final-path absence and a `synced` or explicit
`unsupported` rollback-parent sync. Windows/filesystem `unsupported` is an
honest portability posture, not a durability claim.

The release tree has nine LF-only files: six manifest payload files plus its
manifest, qualification receipt, and seal. The Python qualification exercises
17 methods across all three public Pulse 43 failure postures. The repository Rust validator independently recomputes release
identities, invokes a real Pulse 43 success, an injected Pulse 43
indeterminate failure, and an injected witness-transaction failure.
