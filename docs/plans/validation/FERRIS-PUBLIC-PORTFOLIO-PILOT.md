# Ferris Public Portfolio Pilot

Date: 2026-08-19
Status: Measured local public-repository evidence
Product behavior changed: No

## Purpose

This record moves the federated validation proof from synthetic controls to
four exact public Rust repository revisions. It tests whether Ferris can
retain owner-local Cargo planning while composing explicit application-level
review relationships.

The complete execution record is
[Pulse 01](../../../context/waves/2026-08-19-public-portfolio-validation-pilot/pulses/pulse-01.md).
The structured result is in the
[pilot receipt](FERRIS-PUBLIC-PORTFOLIO-PILOT-RECEIPT.json).

## Replay layout

Place clean exact checkouts under one common parent and create an Application
Definition at that parent. Directory names may differ; semantic identities
will then differ while structural dispositions remain comparable.

```json
{
  "schema": "ferris.application/v0",
  "application_id": "ferris.public/portfolio-pilot",
  "workspaces": [
    {
      "workspace_id": "ferris.public/ferris",
      "manifest_path": "FERRIS/Cargo.toml"
    },
    {
      "workspace_id": "ferris.public/parlor",
      "manifest_path": "PARLOR/Cargo.toml",
      "depends_on": ["ferris.public/ferris"]
    },
    {
      "workspace_id": "ferris.public/rune",
      "manifest_path": "RUNE/Cargo.toml",
      "depends_on": ["ferris.public/ferris"]
    },
    {
      "workspace_id": "ferris.public/icelines",
      "manifest_path": "ICELINES/Cargo.toml"
    }
  ]
}
```

The PARLOR and RUNE edges are an explicit contract-migration review policy.
They are not Cargo dependencies and must not be inferred automatically.

## Decision

Adopt this result as public pilot evidence for the current read-only
application-planning wedge. Do not convert it into a support, execution,
automatic relationship, or build-time savings claim.

The next evidence step, if separately authorized, is an owner-run CI matrix
consumer that reads Ferris output while keeping commands and approval in the
consumer repository.
