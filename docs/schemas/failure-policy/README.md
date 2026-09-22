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
