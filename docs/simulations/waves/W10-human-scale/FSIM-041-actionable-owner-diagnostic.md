# FSIM-041: Actionable Native Owner Diagnostic

Wave: W10
Revision: 1
State: Retraced
Claim state: simulated

## Question

What must Ferris show when a native linker emits thousands of lines but the
material failure is one unsupported ABI and no safe automatic fix exists?

## Locked fixture

- application: `forge`
- repositories and workspaces: Rust workspace with native library
- source and change: target architecture change
- contracts and profiles: requested ABI is unsupported
- environment: linker emits 12,000 lines before failing
- policy: logs are bounded; unsafe flag substitution is prohibited
- available evidence: owner exit, ABI diagnostic, command, and exact profile
- explicit unknowns: whether the upstream library will add support
- negative or matched control: supported ABI link failure caused by missing path

Changing the fixture requires a new revision.

## Governing specifications

- VIEW-001 bounded output and diagnostics;
- PLATFORM-001 support; and
- CAUSALITY-001 owner explanations.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Architecture Change Record | FOREST-002 |
| Scope | Native link and ABI dimensions | SCOPE-001 |
| Evidence | Full log retained by authorized reference; material owner diagnostic extracted | FSIM-SCR-021 |
| Causality | Unsupported ABI is owner-reported cause | CAUSALITY-001 |
| Prediction | Future support remains unknown | No promise |
| Validation | Link stage is unsupported; later stages not observed | PLATFORM-001 |
| Planning | Select supported ABI, request owner input, or defer | Safe alternatives |
| Resolution | No flag-based workaround is eligible | Hard support constraint |
| Trust/action | No mutation or retry loop | EXECUTION-001 |
| Public view | Stable code, owner, impact, prohibited shortcut, safe actions, and log reference | FSIM-SCR-023 |

## Assertions

- [x] 12,000 lines are not dumped as the only diagnosis;
- [x] material owner exit and ABI evidence remain linked;
- [x] unsupported is not failed or unknown;
- [x] unsafe flag substitution is explicitly prohibited; and
- [x] the missing-path control routes to a different diagnostic.

## Simulation issues

- `FSIM-SI-022`;
- `FSIM-SI-024`.

## Specification changes

- `FSIM-SCR-021`;
- `FSIM-SCR-023`.

## Retrace

The fixture now gives a bounded, owner-aligned diagnosis and safe alternatives
without hiding the retained raw evidence.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
