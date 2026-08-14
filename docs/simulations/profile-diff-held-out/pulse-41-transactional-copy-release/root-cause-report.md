# Pulse 41 bounded public post-copy classes

## Immutable result boundary

Pulse 40 is permanently `invalid`, non-retryable, and null-conclusion at
`pulse-39-release-custody`. Its public record is `8/8` copied and `0/8`
post-copy verified. This release neither retries nor reinterprets that result,
and it creates no diagnostic or private-custody authority.

## Bounded finding

The exact private cause is not provable from the public record. The following
publicly reproducible infrastructure classes can independently produce the
same count shape, so none is claimed as the private cause:

1. retaining a stale staging path after the directory rename;
2. passing a root with the release directory duplicated or omitted;
3. resolving a relative release root from the wrong cwd; and
4. treating a pre-final-sync staging verification as final verification.

## Corrected control

The adapter accepts only explicit absolute non-traversal source and final
roots. It binds all eight canonical Pulse 39 paths and raw bytes, creates one
exclusive sibling stage, flushes and `os.fsync`s every destination file before
close, and verifies that stage by final-relative canonical paths. It syncs
every created staging directory bottom-up (`tests`, then the staging root) and
records a deterministic aggregate posture before exactly one `os.replace`.

After the rename it discards staging objects, reconstructs the absolute final
root from the original final-root input, and independently checks cardinality,
path set, size, and digest for all eight files. It then attempts the final
parent directory sync. A real post-rename verification or sync failure removes
the final tree, establishes path absence, and syncs the final parent. Only a
`synced` or explicit `unsupported` rollback-parent posture proves absence; an
unproved rollback is explicit indeterminate publication, never success.

`unsupported` directory sync is a recorded portability posture, not a
durability claim. It is distinct from an operational sync failure injected or
reported by the synchronizer, which triggers the rollback control after
rename.
