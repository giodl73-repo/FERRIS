# Pulse 39 checkout-verifier release

This public, standard-library-only release corrects the Pulse 38 checkout
orchestration ambiguity without retrying, reopening, or reinterpreting the
permanently invalid Pulse 38 result.

`checkout_verifier.py` accepts an explicit checkout root plus the canonical
repository-relative Pulse 25 and Pulse 27 release roots. It enumerates the
fixed 36-file public catalog, makes exactly 1 root-anchored
`git -C <checkout-root> check-attr -z --stdin text eol` invocation, and makes
exactly 1 separate root-anchored read-only `git -C <checkout-root> --version`
probe. It parses NUL framing exactly, requires `text=set` and `eol=lf`, and
rejects every CR byte. It produces deterministic public JSON containing relative
paths, counts, the Git version, and status. It neither reads private custody material nor
executes FERRIS, a diagnostic candidate, seed/corpus materialization, a build,
or a preflight.

The independently reproduced failure class was process-cwd ambiguity: from
below the checkout root, a repo-root-relative `git check-attr` path may exit
zero while reporting `text` and `eol` as `unspecified`; a single combined argv
string exits one. The release therefore has exactly 1 check-attr invocation,
exactly 1 Git version probe, and exactly 2 total Git processes; it has 0 retries
and no fallback check-attr form.

The qualification receipt records the disposable Windows
`core.autocrlf=true` cutoff checkout at
`6807bd68aa01cbf0c819198765b7d6b5aa443328`: 36/36 attributes, 36 LF files
with zero CR bytes, and the already public 76/76 normalized binding receipt.
This is infrastructure qualification only. It grants no diagnostic authority,
product authority, fix authority, score, certification, support claim, or
PLATFORM-001 status change.
