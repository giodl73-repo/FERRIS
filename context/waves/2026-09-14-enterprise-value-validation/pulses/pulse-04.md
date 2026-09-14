# Pulse 04: Root-Request Repeatable Enterprise Onboarding

Status: Authorized
Implementation authority: One disposable evidence attempt only

## Objective

Repeat Pulse 03 with the `EO-02` federated request at the repository root, as
required by the existing descendant-only path contract.

## Authority

The pulse inherits Pulse 03's exact Ferris and consumer revisions, PowerShell
archive, owner commands, Action Plan construction, platform scope, preflights,
stop conditions, exclusions, and cleanup requirements.

It may additionally stage one untracked disposable
`ferris-onboarding-request.json` at the `EO-02` repository root with
`workspaces/<workspace>/Cargo.toml` paths. The file is onboarding material, not
owner source or owner truth.

## Removal invariant

Before post-removal owner validation:

- `.ferris/` MUST be absent;
- `ferris-onboarding-request.json` MUST be absent; and
- the exact consumer revision and tracked tree MUST remain clean.

Any failed gate stops the pulse without retry. No product change, persisted
consumer change, performance claim, CI authority, production claim, or support
claim is authorized.
