# Pulse 03: Read-Only Interface Hardening

Status: Complete on Windows and Unix; all applicable held-out fixtures passed
Implementation authority: Corrective only; no capability expansion

## Goal

Correct the reviewed identity, output, evidence, diagnostic, and CLI envelope
defects in the existing `plan`, `explain`, and declared `graph` commands.

## Required behavior

- require `--workspace-id <portable-id>` for all three commands;
- include workspace identity in plan and graph record identities;
- bind invocation identity to command, workspace, normalized manifest
  selection, metadata version, `--no-deps`, offline mode, and locked mode;
- record a portable-equivalent Cargo command, explicit working-directory
  semantics, workspace identity, and owner-output digest;
- keep raw Cargo diagnostics internal while returning a safe summary and
  source digest;
- expose material JSON semantics in human `explain` and `graph` output; and
- return `ferris.command-result/v0` with exit 2 for invalid JSON-mode command
  lines while preserving normal help and version behavior.

Workspace IDs are 1 to 128 ASCII letters, digits, `.`, `-`, `_`, `:`, or `/`.

## Prohibited behavior

- changing Cargo ownership or the metadata invocation;
- network access, owner execution, mutation, or sibling discovery;
- adding commands, resolution, affected scope, scheduling, query, or
  persistence;
- exposing raw Cargo stderr, absolute checkout paths, credentials, registry
  URLs, or private owner details; or
- reusing Pulse 01 or Pulse 02 held-out scores for the corrected executable.

## Acceptance

- identically shaped workspaces with different workspace IDs have different
  plan and graph IDs;
- identical logical workspaces in different checkout paths retain identities;
- `plan`, `explain`, and `graph` have distinct invocation identities;
- human output contains all reviewed material semantics;
- Cargo failures expose a safe diagnostic and SHA-256 source digest without
  the selected absolute path;
- malformed JSON-mode invocations return a Ferris envelope and exit 2;
- formatting, tests, lint, Windows, Unix renewal, and diff checks pass; and
- an independent review finds no unresolved blocker.

## Held-out renewal

After correction and independent review, freeze a new immutable cutoff and
request independent scoring of applicable sealed fixtures. Preserve the prior
cutoffs as historical evidence.

Cutoff `c3590a39fd053a66996909b87eaf7ca7ac73ded4` was independently
scored: FHIF-009 and FHIF-012 passed, and the remaining ten fixtures were
outside this pulse and not executed.
