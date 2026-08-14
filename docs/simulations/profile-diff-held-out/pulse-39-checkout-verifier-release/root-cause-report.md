# Pulse 39 public root-cause report

## Finding

Pulse 38 is permanently `invalid`, non-retryable, and null-conclusion at
cutoff `6807bd68aa01cbf0c819198765b7d6b5aa443328`. The independently
reproduced infrastructure failure was orchestration ambiguity, not a release
byte, Git-attribute, product, or diagnostic finding.

When a caller runs below the checkout root but passes repository-root-relative
paths to `git check-attr`, Git can exit zero and emit `text`/`eol` as
`unspecified`. A weak wrapper can mistake that incomplete first check for a
usable outcome. Separately, a combined command-line argument form exits one.

## Corrected public control

The verifier always invokes exactly 1 check-attr argument-vector command rooted
at the explicit checkout:

```text
git -C <checkout-root> check-attr -z --stdin text eol
```

It separately invokes exactly 1 root-anchored read-only
`git -C <checkout-root> --version` probe to produce the required tool-version
output. It supplies exactly the fixed 36 repository-relative paths through NUL
stdin, requires a complete NUL-framed two-attribute record for each path, and
fails
on any Git error, malformed record, duplicate/missing/unexpected path,
unspecified attribute, incorrect value, CR byte, or cardinality mismatch.
There are exactly 1 check-attr invocation, 1 Git version probe, and 2 total Git
processes, with 0 retries and no fallback check-attr form.

## Boundaries

This release is public checkout-verifier infrastructure only. It does not
execute FERRIS, a diagnostic candidate, build, preflight, seed, corpus
materialization, or private custody data. It does not alter any historical
Pulse result, create a diagnostic authority, grant a fix, or change
PLATFORM-001.
