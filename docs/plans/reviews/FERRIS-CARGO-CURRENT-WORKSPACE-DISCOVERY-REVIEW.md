# Ferris Cargo Current-Workspace Discovery Review

Date: 2026-08-18
Scope: Pulse 01 adapter-only manifest default
Disposition: Complete within one pulse and one implementation attempt
Implementation authority: No expansion

## Product Value Governor

Disposition: `continue-within-budget`

Approved outcome before implementation: a Ferris user can invoke the existing
read-only workspace commands through `cargo ferris` without repeating
`--manifest-path` when Cargo can locate the current workspace.

Approved budget: one pulse, one implementation attempt, one review record, no
successor, no generated workspace identity, and no continuation of the
platform diagnostic authority chain.

Completion condition: Cargo remains the sole discovery authority; explicit
manifests retain exact parity; standalone `ferris` remains explicit; portable
workspace identity remains explicit; and failures remain bounded, typed, and
path-free.

Abandonment condition: stop `stop-value-exhausted` if truthful defaulting
requires directory crawling, configuration, schema changes, another
architectural layer, or a successor pulse.

Measured result: the pulse stayed inside budget and removed the repeated Cargo
adapter manifest argument without changing the existing product semantics.
The separate post-Pulse-88 audit records `stop-value-exhausted` for another
real diagnostic authority because no supported adopter or product decision
would change from that invocation.

## Rust Safety Steward

Pass. The change remains safe Rust, adds no `unsafe`, bounds the Cargo process
duration and retained output, and preserves typed failures.

## Compiler Performance Engineer

Pass with no performance claim. One bounded `cargo locate-project` call is
added only when the Cargo adapter has no explicit manifest. No build-latency,
validation-cost, or cache claim is made.

## Interop Boundary Auditor

Pass. Cargo owns workspace discovery and metadata. Ferris parses Cargo's JSON
root result and does not duplicate parent-directory rules, alter resolution,
or infer platform behavior.

## AI Assurance Skeptic

Pass. Tests compare discovered and explicit records, prove failure redaction,
prove workspace-ID fail-fast behavior, and cover successful Cargo output that
also carries owner diagnostics. No model assertion grants success.

## Ecosystem Strategist

Pass. The implementation fulfills the documented Cargo-native
current-workspace adapter promise through Cargo's official command rather than
introducing another manifest convention.

## Rust Maintainer

Pass. One shared resolver serves both Cargo adapter forms and all five existing
workspace commands. Standalone behavior is selected through the existing
`InvocationKind` seam; no duplicate command engine or public schema is added.

## Native Platform Adopter

Pass. Users can run from a nested workspace directory on the recorded Windows
path, retain explicit override and rollback, and receive a typed result when
Cargo cannot locate a workspace. No support claim is added.

## Scope Keeper

Pass. The pulse removes only one repeated adapter argument. It does not default
workspace identity, add a command, execute validation, mutate a workspace,
change schemas, or reopen Pulse 83-88 authority.

## Validation Checker

Pass. Validation covers direct and Cargo-style adapters, all five workspace
commands, nested-directory discovery, explicit bypass, standalone rejection,
invalid workspace identity before discovery, path-free failure, successful
Cargo stderr diagnostics, workspace checking, formatting, and diff hygiene.

## Autonomy Supervisor

Pass. The outcome, one-pulse budget, completion test, and abandonment
condition were recorded before implementation. One implementation attempt was
used. One independent review finding about successful Cargo stderr was fixed
inside the original slice and covered by a regression test. No successor was
started.

Control record:

- product outcome: Cargo-native current-workspace manifest discovery for the
  existing read-only commands;
- work completed: one bounded Cargo discovery helper, adapter-only defaulting,
  explicit bypass, typed errors, tests, and usage documentation;
- value obtained: `cargo ferris` no longer requires a redundant manifest path
  from inside a workspace;
- remaining risk: the user must still supply a portable workspace ID, and
  discovery remains subject to the installed Cargo owner's behavior;
- pulses or retries consumed: one pulse, one implementation attempt, zero
  successors;
- proposed next action: stop; seek a named external adopter before expanding
  the experimental contract; and
- Product Value Governor disposition: `continue-within-budget`.

## Independent review

Codex review completed with no accepted or actionable findings after the
Cargo-stderr correction. A separate high-confidence code review also returned
no significant findings.

## Decision

Pulse 01 is complete. All eleven roles accept the bounded current-workspace
manifest default. No role grants a generated workspace identity, new command,
execution, mutation, support, compatibility, platform, or successor authority.
