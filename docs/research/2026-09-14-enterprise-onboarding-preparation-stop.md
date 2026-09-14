# Enterprise Onboarding Preparation Stop

Date: 2026-09-14
Status: Complete; invalid before execution

## Findings

### FERRIS-VALUE-005: Exact pinned builds succeeded

Exact public Ferris revision
`b347dc34d62810f043122d0d279def316b890cf0` built from clean disposable
checkouts on Windows and native Linux. Windows required Git long-path support;
the WSL clone used the repository's actual common checkout rather than a
Windows-only worktree pointer.

### FERRIS-VALUE-006: Federated request paths are request-relative

The disposable `EO-02` request was stored below `.ferris/onboarding/` but named
manifests relative to the repository root. `federated-plan` correctly rejected
the first nonexistent request-relative manifest before producing a plan.

### FERRIS-VALUE-007: Missing staged executables fail closed

Windows staging used a literal wildcard that copied no PowerShell runtime
files. The Action Plan generator used Ferris's own file identity function and
rejected the absent executable before emitting valid records.

### FERRIS-VALUE-008: No execution or consumer mutation occurred

No valid Action Plan, owner command, `go`, receipt, source mutation, or removal
claim occurred. All disposable checkouts, builds, partial `.ferris/` material,
native PowerShell files, symlink, and archive were removed. Source consumers
remain clean.

## Decision

This attempt is invalid onboarding evidence. A corrected pulse must preflight
request-relative manifests and staged executable presence before planning or
identity generation.
