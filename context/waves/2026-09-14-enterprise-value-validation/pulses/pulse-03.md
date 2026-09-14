# Pulse 03: Corrected Repeatable Enterprise Onboarding

Status: Complete; invalid before execution
Implementation authority: Exhausted

## Objective

Determine whether the existing Ferris CLI can be onboarded, executed, verified,
and removed repeatably in both private consumers on Windows and native Linux
after correcting the two preparation defects observed in Pulse 02.

## Frozen inputs

- public Ferris revision:
  `b347dc34d62810f043122d0d279def316b890cf0`;
- `EO-01` revision:
  `0b10d5427491dcf55a76704777b0006cd2667022`;
- `EO-02` revision:
  `4b66226c5f180b73bf27d8737d760cfbcbd39f5f`;
- unchanged owner validation entrypoints and success semantics; and
- PowerShell 7.6.6 Linux x64 archive with SHA-256
  `ddbc4a2d113bbd46d283cfedcbcd117a70caefd7673f41f2b4e0000badf103bc`.

## Authorized corrections

The pulse may:

1. store the `EO-02` request below `.ferris/onboarding/` with manifests named
   as `../../workspaces/<workspace>/Cargo.toml`;
2. stage Windows PowerShell by enumerating source items rather than passing a
   wildcard to a literal-path copy; and
3. stop before planning unless every request-relative manifest exists and each
   staged `pwsh.exe` or `pwsh` exists and launches natively.

No other preparation, product, consumer, owner-command, or environment change
is authorized.

## Execution boundary

After all preflights pass, the pulse may:

- run `plan` for `EO-01` and `federated-plan` for `EO-02`;
- generate exactly one owner-validation Action Plan per consumer and platform
  using public `ferris-core` identity functions;
- run `go` and `verify` for each generated approval and receipt;
- retain only public-safe aggregate evidence;
- delete each disposable consumer's complete `.ferris/` tree;
- rerun the four unchanged owner commands;
- prove exact clean revisions; and
- remove every disposable checkout, build, runtime, symlink, and archive.

## Stop conditions

Any failed preflight, planning, identity, execution, verification, removal,
owner command, revision, cleanliness, or cleanup gate stops the pulse. There is
no same-pulse retry, command translation, environment accommodation, or result
reinterpretation.

## Exclusions

This pulse grants no product change, consumer source change, persistent
prerequisite, hosted CI, affected-only gating, performance or savings claim,
workflow replacement, production, support, external adopter, or commercial
authority.

## Validation

- all frozen revisions and tracked trees match custody;
- all preparation preflights pass before planning;
- four planning/execution records and four receipts verify;
- `.ferris/` is absent before post-removal owner commands;
- all four owner commands pass;
- source and disposable revisions remain exact and clean;
- private custody JSON remains valid;
- changed Markdown links and fences pass; and
- `git diff --check` passes.

## Result

The Windows disposable checkouts and exact pinned Ferris build succeeded.
Enumerated PowerShell staging produced launchable `pwsh.exe` files, and all
three `EO-02` paths resolved to existing manifests during the preparation
preflight.

`federated-plan` then rejected the first `../../workspaces/...` value with
`FERRIS-FEDERATED-PLAN-MANIFEST-TRAVERSAL`. Ferris forbids parent-directory
components and requires the request at a common ancestor. This disproves the
assumption that request-relative parent traversal could preserve the
all-onboarding-under-`.ferris/` layout.

The planning stop applied. No valid Action Plan, `go`, receipt, owner command,
consumer mutation, or Linux materialization followed. The Windows disposable
root and all partial onboarding and runtime files were removed. Source
consumers remain clean.

## Closeout

Pulse 03 is invalid onboarding evidence and exhausted. A successor must choose
between a removable repository-root request file or a product change; neither
is authorized here.
