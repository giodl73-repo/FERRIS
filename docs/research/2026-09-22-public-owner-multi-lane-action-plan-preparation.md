# Public Owner Multi-Lane Action Plan Preparation Evaluation

Date: 2026-09-22

Status: completed bounded implementation and adopter evaluation

## Question

Can Ferris compile PARLOR's explicit ten-lane owner topology into the existing
unsigned Action Plan without inferring commands or policy, creating approval,
or launching owner work?

## Controls

- Consumer: `giodl73-repo/PARLOR` at
  `ff856a6940ccdb05f8f028c3b07a461c3a34f9f3`.
- Owner source: committed `tools/ferris-go/run.py` and
  `tools/ferris-go/topology.json`.
- Input boundary: one generated `ferris.owner-entrypoints/v1` declaration and
  one strict `ferris.action-plan-lanes/v1` projection of the adapter's explicit
  lane policy.
- No command discovery, dependency inference, approval creation, execution, or
  retained consumer mutation.
- Completion: exact semantic equivalence with the owner-authored Action Plan,
  byte-identical repeats, zero receipts, and clean removal.

## Implementation

`prepare-action-plan --lanes <LANES_JSON>` accepts a bounded repository-local
lane-policy file. The record supplies:

- repository and topology identity;
- ordered lane IDs and owner gates;
- requiredness;
- dependencies naming only earlier lanes;
- owner entrypoint references; and
- timeout and output bounds.

Ferris validates every referenced declaration identity, command, executable,
working directory, and bound file. It copies exact commands from the
declaration, rechecks both inputs and source revision before output, leaves
`approval_id` empty, and atomically creates a non-overwriting Action Plan V1.
The existing direct single-lane interface remains available.

## Result

PARLOR's owner adapter prepared ten entrypoints and its established ten-lane
Action Plan. The lane-policy projection preserved this topology:

```text
quality-fmt
  -> quality-clippy
    -> build-release
      -> build-tests-release
        -> test-core
        -> test-backgammon
        -> test-checkers
        -> test-chess
        -> test-go
        -> test-cli
```

Ferris compiled the explicit policy with these results:

- prepared lanes: 10;
- prepared Action Plan ID:
  `sha256:dd5822f4e8ea3a07405a0dd6c4bca75ef395cf9c271a6a8380628a877974eee3`;
- owner adapter Action Plan ID: the same identity;
- semantic comparison after clearing the owner's independent approval ID:
  equal;
- repeated prepared bytes: equal;
- prepared output SHA-256:
  `9fcd3d3f74f078b50a5ea68a27b6a376d1156f83702232c6dbd9e54a830699bc`;
- receipts created: zero; and
- consumer state after removal: clean at the original revision.

No owner validation command was launched in this evaluation. PARLOR's prior
owner evidence remains authoritative for command behavior.

## Product Finding

The multi-lane compiler removes Ferris identity and command-copy assembly from
owner adapters while preserving all owner policy. PARLOR's current Python
adapter still resolves and stages Cargo, selects changed packages, authors the
lane policy, creates local approval, and invokes execution; those authorities
do not move into Ferris.

The next adoption gate is a committed owner `ferris.action-plan-lanes/v1`
record or equivalent owner-generated input using this command. Do not add
dependency discovery, automatic approval, or PATH executable support based on
this result.

## Reproduction

```console
ferris prepare-action-plan \
  --entrypoints <OWNER_ENTRYPOINTS_JSON> \
  --lanes <OWNER_LANES_JSON> \
  --output <ACTION_PLAN_JSON> \
  --format json
```

Targeted implementation controls:

```console
cargo test -p ferris-cli --test execution prepares_deterministic_multi_lane_action_plan_from_explicit_policy -- --exact
cargo test -p ferris-cli --test execution multi_lane_preparation_rejects_forward_dependency -- --exact
cargo test -p ferris-cli --test execution prepares_deterministic_unsigned_action_plan_without_launching -- --exact
```

The complete execution contract suite passed after the ordered-dependency
validator correction: 23 passed, 0 failed. The bounded core and CLI Clippy
target also passed after the accumulated Rust 1.95 lint corrections.
