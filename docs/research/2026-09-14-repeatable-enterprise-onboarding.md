# Repeatable Enterprise Onboarding and Removal

Date: 2026-09-14
Status: Complete

## Findings

### FERRIS-VALUE-013: Existing Ferris completed cross-platform onboarding

At exact public Ferris revision
`b347dc34d62810f043122d0d279def316b890cf0`, both frozen private consumers
completed explicit planning, owner-approved Action Plan execution, and receipt
verification on Windows and native Linux. All four owner lanes and receipts
succeeded.

### FERRIS-VALUE-014: Owner-native environment requirements stayed explicit

Windows required the installed MSVC x64 developer environment plus the
non-secret allowlist `LIB`, `PATH`, `PATHEXT`, `SYSTEMROOT`, `TEMP`, `TMP`, and
`WINDIR`. Linux required `/root/.cargo/bin` in its process-local `PATH` and a
checksum-verified removable PowerShell 7.6.6 runtime. No prerequisite or
environment change persisted.

### FERRIS-VALUE-015: Removal restored ordinary owner workflows

Every `.ferris/` tree and the temporary federated request were removed before
post-removal validation. All four unchanged owner commands passed, and exact
tracked consumer trees remained clean.

### FERRIS-VALUE-016: Cleanup was complete

All disposable Windows and Linux checkouts, builds, runtime copies, the
external Linux runtime, archive, and platform roots were removed. Source
consumers remained unchanged.

## Decision

The repeatable enterprise onboarding and removal gate is proven for these two
frozen private synthetic consumers and the exercised Windows/native-Linux
environments. This evidence does not establish performance, savings, required
CI replacement, production readiness, support, or external-adopter fit.
