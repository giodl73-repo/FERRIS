# Federated Onboarding Request Location

Date: 2026-09-14
Status: Complete; Pulse 03 invalid before execution

## Findings

### FERRIS-VALUE-009: Existence does not make parent traversal valid

All three corrected `EO-02` paths resolved from
`.ferris/onboarding/federated-request.json` to existing workspace manifests.
`federated-plan` nevertheless rejected the first value with
`FERRIS-FEDERATED-PLAN-MANIFEST-TRAVERSAL` because manifest paths must not
contain a parent-directory component.

### FERRIS-VALUE-010: The request must be at a common ancestor

The existing product diagnostic directs the caller to place the request at a
common ancestor and use descendant manifest paths. Therefore an `EO-02`
federated request cannot both remain below `.ferris/` and describe sibling
root-owned `workspaces/` without changing the product contract.

### FERRIS-VALUE-011: Corrected runtime staging passed

Enumerating Windows PowerShell source items staged an existing, natively
launchable `pwsh.exe` in each disposable consumer. The independent runtime
staging defect from Pulse 02 is corrected.

### FERRIS-VALUE-012: Planning stopped all execution

No valid Action Plan, `go`, receipt, owner command, Linux materialization,
consumer source mutation, or onboarding claim followed. The complete Windows
disposable root was removed and source consumers remain clean.

## Decision

Pulse 03 is invalid and exhausted. A successor requires separate authority to
stage a removable request at repository root, or separate product authority to
change traversal semantics.
