# Failure Policy Schemas

`ferris.failure-policy/v1` is a closed owner-authored mapping from the four
classifications emitted by `ferris.cargo-failure-report/v1` to an opaque owner
action identity and one passive disposition: `halt`, `route`, or
`prepare_action`.

`ferris.failure-policy-decision/v1` records the deterministic result of
`ferris failure-policy --policy <POLICY_JSON> --diagnosis <JSON|->`. The
diagnosis must be one complete successful JSON command result emitted by
`diagnose-cargo`; `-` reads it from standard input through EOF.

The command validates both input identities and selects at most one rule.
Unmatched classifications use the required fallback. Owner action identities
are opaque: Ferris does not resolve them to commands or entrypoints. A decision
does not create an Action Plan, grant approval, retry work, or execute a
command.

`prepare-action-plan --failure-decision <JSON>` accepts the complete emitted
command result as an optional single-lane selector. It requires
`prepare_action`, validates the decision and command-result identities, and
looks up `owner_action_id` only in the separately supplied owner entrypoint
declaration. It does not accept `halt` or `route`, infer lane policy, create
approval, or execute. Action Plan V1 does not embed the decision ID; the
decision remains a separate audit artifact.
