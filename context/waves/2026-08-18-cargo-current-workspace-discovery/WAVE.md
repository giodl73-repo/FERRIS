# Wave: Cargo Current-Workspace Discovery

Status: Closed after Pulse 01

## Product outcome

Let a Ferris user run the existing read-only workspace commands through
`cargo ferris` without repeating `--manifest-path` when Cargo can identify the
current workspace, while preserving the explicit portable `--workspace-id`
and every existing command semantic.

## Classification

Release/readiness wave with one bounded implementation pulse.

## Product Value Governor

Disposition: `continue-within-budget`

The post-Pulse-88 audit rejects another `process-exit-agreement` diagnostic
authority as `stop-value-exhausted`: the remaining platform observation would
not change a supported product decision, the repository has no external
adopter contract, and the custody chain has already exceeded the value of the
single behavior it observes.

The approved alternative is the smallest open user-facing promise already in
the Ferris program. The Cargo adapter is specified as a current-workspace
surface, but currently requires the same explicit manifest selection as
standalone `ferris`. Cargo already owns the required discovery operation.

## Budget

- exactly one pulse;
- exactly one implementation attempt;
- exactly one final review record;
- no Pulse 89, diagnostic authority, real invocation, successor chain, or new
  command capability; and
- no default or inference for the portable workspace ID.

## Completion condition

The wave is complete only when:

- direct `cargo-ferris` and Cargo-style `cargo ferris` may omit
  `--manifest-path` for `plan`, `validation-plan`, `explain`, `graph`, and
  `doctor`;
- Cargo `locate-project --workspace --message-format json` is the sole default
  discovery authority;
- standalone `ferris` still requires an explicit manifest;
- every explicit manifest continues to bypass discovery and produce the same
  result as before;
- discovery failure is a typed non-success result with no raw path or Cargo
  output disclosure; and
- targeted CLI tests, workspace checking, formatting, and diff validation
  pass.

## Abandonment condition

Stop `stop-value-exhausted` without a successor if truthful defaulting requires
a generated workspace identity, directory crawling, configuration, another
architectural layer, changed record schemas, or more than the one authorized
implementation attempt.

## Owner actions

| Owner | Action |
|---|---|
| Cargo | Remains authoritative for current-workspace manifest discovery |
| FERRIS | Consumes the discovered manifest for the existing read-only commands |
| User/repository | Retains the explicit portable workspace ID and all owner workflows |
| TRACKER | No-op; this wave does not change portfolio state |

## Pulse table

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Cargo current-workspace discovery | Complete | Cargo adapter defaults manifest selection through Cargo-owned discovery |

## Non-goals

- defaulting, generating, or hashing `--workspace-id`;
- changing standalone `ferris`, command IDs, schemas, records, or exit classes;
- affected-scope discovery, validation execution, mutation, network access, or
  repository policy;
- changing Cargo resolution or walking parent directories independently; and
- continuing the platform diagnostic successor chain.
