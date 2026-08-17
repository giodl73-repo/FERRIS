# Ferris Conservative Validation Plan Review

Date: 2026-08-17
Scope: Pulse 01 local `validation-plan`
Disposition: Complete within one pulse and one implementation attempt
Implementation authority: No expansion

## Product Value Governor

Disposition: `continue-within-budget`

Approved outcome before implementation: a Ferris user can request a
conservative, read-only validation plan for explicit local changed workspace
paths/packages, see selected package closure and fallback clearly, and receive
stable machine output without executing Cargo validation commands.

Approved budget: one pulse, one implementation attempt, one review record, and
no successor chain or diagnostic custody infrastructure.

Completion condition: the command must remain read-only, Cargo-metadata-only,
and explicit about fallback, unknowns, and non-equivalence.

Abandonment condition: stop `stop-value-exhausted` if truthful behavior would
require new repository declarations, another architectural layer, or a
successor pulse.

Measured result: the pulse stayed inside budget and achieved the user outcome
without widening authority. No continuation is approved.

## Rust Safety Steward

Accept. The implementation remains safe Rust, adds no `unsafe`, executes no
owner validation code, and keeps correctness boundaries explicit.

## Compiler Performance Engineer

Accept with no performance claim. The command selects package scope only; it
does not report saved latency, benchmark results, or reduced correctness cost.

## Interop Boundary Auditor

Accept. Ordinary Cargo metadata remains the only owner boundary. Package-path
selection does not claim ABI, native, runtime-data, or cross-language truth,
and unsupported paths widen rather than narrowing silently.

## AI Assurance Skeptic

Accept. The command is deterministic, read-only, and evidence-grounded. It
does not convert selected-package output into a full-validation success claim,
and failure/fallback remain visible in typed results.

## Ecosystem Strategist

Accept. This is a bounded missing capability named directly by PERF-Q35 and
BI-04. It consumes stable Cargo metadata, preserves ordinary Cargo workflows,
and avoids building a parallel resolver or CI replacement.

## Scope Keeper

Accept. One new read-only command over explicit local inputs is the full
capability boundary. Query, execution, repo-specific validation declarations,
platform-profile work, and successor infrastructure remain deferred.

## Validation Checker

Accept. The pulse records concrete commands, focused fixtures, JSON/human
assertions, workspace-boundary safety, fallback behavior, toolchain-bound
validation, and the absence of validation-command execution.

## Autonomy Supervisor

Accept. The approved outcome, budget, completion condition, and abandonment
condition were recorded before implementation. One pulse was consumed, no
corrective successor was started, and review findings were closed inside the
bounded scope rather than converted into an automatic follow-on loop.

## Validation

Commands run on the recorded worktree:

```console
cargo test -p ferris-core validation_plan_
cargo test -p ferris-cli --test cli
cargo check --workspace
cargo fmt -p ferris-core --check
cargo fmt -p ferris-cli --check
git diff --check
```

Result summary:

- selected package closure is explicit for supported package/path anchors;
- unknown workspace paths widen visibly to a full-workspace fallback;
- machine output stays typed and checkout-path-redacted;
- no Cargo validation command executes; and
- `cargo fmt -p ferris-core --check` passed;
- `cargo fmt -p ferris-cli --check` failed only on pre-existing unrelated
  diagnostic-pulse test files that this pulse was explicitly forbidden to
  touch; and
- no successor pulse is authorized or required.

## Decision

Pulse 01 is complete. Ferris now has one bounded experimental
`validation-plan` command with explicit fallback language and no widened
authority.
