# Pulse 21: REEL Scene-Delivery Value Cohort

Status: Complete; invalid owner prerequisite
Implementation authority: Exhausted

## User outcome

Determine whether Ferris planning plus a repository-owned focused test target
reduces validation time for one real REEL change while preserving the
repository-owned full Cargo reference.

## Frozen case

- repository: public REEL;
- head: `5c896f5a9d3fb7bd4a709ace66dfe119c6a568bb`;
- base: `9719aacc4d4554ad8d38502ae33135803836fd3c`;
- Ferris: `1baa6a8`;
- selected owner command: `cargo test --test scene_delivery`;
- full owner command: `cargo test --all-targets --all-features`; and
- platform: local Windows x86_64.

The selected command is documented by REEL and is a strict target subset of the
full command owned by REEL CI. Neither command includes ignored FFmpeg tests.
The separately documented ignored episode-delivery command is excluded because
FFmpeg is unavailable and the ordinary full reference does not run ignored
tests.

## Preflight and measurement

A separate disposable checkout MUST pass both exact commands with semantic test
evidence before planning or measurement. Any failure ends the pulse.

A disposable `ferris.owner-validation-domains/v2` declaration may map the 13
committed paths to focused owner entrypoints and MUST remain outside REEL.
Planning MUST bind the exact base, head, and tested revision and succeed without
fallback.

After preflight, use independent selected and full checkouts for eight pairs:

- four selected-first and four full-first;
- two checkout-local cold and two warm pairs within each order;
- every selected lane starts with a fresh revision-bound plan;
- selected elapsed includes planning and the selected command;
- all owner invocations require exit `0`, test-run evidence, and no bare Cargo
  usage; and
- no failed or divergent pair contributes a performance result.

The gate passes only if all sixteen lanes pass, selected-pass/full-fail
divergence is zero, overall median selected elapsed is at least 10% below full,
and both orders and both cache states have positive median deltas.

## Boundaries and stop condition

No FFmpeg installation, ignored-test substitution, REEL or ICELINES mutation,
dependency or workflow change, command inference, required-check narrowing,
hosted execution, product change, production claim, support claim, or broad
savings extrapolation is authorized.

Any preflight, planning, owner-command, semantic-log, capacity, cleanup, or
threshold failure ends the pulse. No corrective successor follows
automatically.

Product Value Governor disposition: `continue-within-budget`.

## Result

The exact selected command ran eight scene-delivery tests and passed six while
correctly leaving two FFmpeg tests ignored. The exact full command reached real
tests but exited `101`: two non-ignored `vfx_effect_pass_v0319` tests panicked
with `program not found` because FFmpeg was unavailable.

The full owner reference therefore failed its mandatory preflight. Planning and
measurement did not start, no timing is admissible, and no narrower substitute
was introduced. All disposable roots were removed and the BISECT, ICELINES,
and REEL research clones remained clean. Pulse 21 is exhausted and grants no
FFmpeg installation, retry, or later-gate authority.
