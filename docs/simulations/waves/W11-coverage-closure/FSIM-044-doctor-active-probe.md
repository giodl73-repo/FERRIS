# FSIM-044: Doctor Active Probe Boundary

Wave: W11
Revision: 1
State: Retraced
Claim state: simulated

## Question

May `ferris doctor` contact a private registry with credentials and run a
build script merely because the user requested diagnosis?

## Locked fixture

- application: `forge`
- repositories and workspaces: one workspace with `build.rs`
- source and change: no source change
- contracts and profiles: private registry and native SDK support are uncertain
- environment: registry token is available through an approved credential broker
- policy: passive diagnosis is allowed; network, credentials, and owner-code
  execution require action approval
- available evidence: local configuration, executable presence, cached tool
  versions, and stale registry metadata
- explicit unknowns: current registry access and build-script behavior
- negative or matched control: static local configuration error

Changing the fixture requires a new revision.

## Governing specifications

- VIEW-001 doctor behavior;
- EVIDENCE-001 read-only default; and
- EXECUTION-001 Action Plan.

## Hand-derived trace

| Stage | Predicted record or state | Rule |
|---|---|---|
| Change | Diagnostic request has no product Change Record requirement | Read request |
| Scope | Local static checks plus two proposed active probes | Probe classification |
| Evidence | Passive findings are observed; remote and build-script states remain unknown | FSIM-SCR-024 |
| Causality | Missing current evidence is not a failed registry or script | Claim separation |
| Prediction | Active probe outcome remains unknown | No execution |
| Validation | Probe Plan states credentials, network, code execution, and side effects | VIEW-001 |
| Planning | Versioned non-executable Probe Plan is produced | Plan-first |
| Resolution | Request action, use stale evidence with limits, or defer | Alternatives |
| Trust/action | No token access, network call, or build script occurs | Governance |
| Public view | Shows passive findings and exact approval needed for probes | Diagnostic contract |

## Assertions

- [x] `doctor` is passive by default;
- [x] token availability does not authorize access;
- [x] build-script execution is not observation-only;
- [x] interactive or MCP context does not imply approval; and
- [x] the static-error control may diagnose without a Probe Plan.

## Simulation issues

- `FSIM-SI-025`.

## Specification changes

- `FSIM-SCR-024`.

## Retrace

The fixture now separates immediate passive findings from governed active
probes without performing either remote or owner-code action.

## Claim boundary

This is a specification-derived simulation, not observed implementation
behavior.
