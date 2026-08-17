# Wave: Conservative Validation Plan

Status: Closed after Pulse 01

## Product outcome

Give a Ferris user one read-only command that can accept explicit changed
workspace paths and package names, explain a conservative selected Cargo
package closure, and show the full-workspace fallback without executing Cargo
validation commands.

## Classification

Release/readiness wave with one bounded implementation pulse.

## Budget

- exactly one pulse;
- exactly one implementation attempt;
- exactly one review record; and
- no successor chain, diagnostic custody layer, or platform-profile expansion.

## Completion condition

The wave is complete only when Ferris can emit stable human and JSON
`validation-plan` results over explicit local inputs, preserve the existing
typed command-result envelope, show selected and fallback validation honestly,
and pass the bounded validation commands recorded in Pulse 01.

## Abandonment condition

Stop and report `stop-value-exhausted` without edits or follow-on pulses if the
existing research, contracts, or CLI architecture cannot support a small
truthful command without inventing repository semantics, or if the first
implementation attempt reveals a design-level blocker that would require a
successor pulse, new architectural layer, or broader validation infrastructure.

## Owner actions

| Repo | Action |
|---|---|
| FERRIS | Implement, validate, review, and retain all product changes locally |
| TRACKER | No-op; keep this wave separate from portfolio state |
| Cargo and external repositories | No-op; ordinary Cargo metadata remains authoritative |

## Pulse table

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Conservative validation plan | Complete | Implemented one read-only package/path validation planner with explicit full-workspace fallback |

## Non-goals

- executing `cargo check`, `cargo build`, `cargo test`, Clippy, formatting, or
  repository gates;
- git, sibling-workspace, or network discovery;
- repository-owned validation declarations, workflow edits, or CI replacement;
- AI narrowing, approvals, MCP, connectors, deployment, or mutation;
- pulse chains, successor planning, or diagnostic custody infrastructure; and
- full-suite, release, support, platform, or confidence claims.

## Completion gate

- `validation-plan` accepts only explicit local changed paths and package names;
- ordinary Cargo metadata remains the only owner tool invocation;
- unsupported or unknown paths widen visibly to a full-workspace fallback or
  fail safely at the workspace boundary;
- selected and fallback validation remain distinct in machine and human output;
- targeted tests, relevant CLI tests, `cargo check --workspace`, and
  `git diff --check` pass;
- `cargo fmt` check is run and any failure outside this pulse's authorized
  files is reported rather than reformatted; and
- one review record captures all nine role dispositions, the approved budget,
  completion result, and the no-successor closeout.
