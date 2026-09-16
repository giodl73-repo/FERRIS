# Pulse 01: Environment Readiness Research

Status: Complete
Implementation authority: None

## User outcome

Determine whether Ferris can make missing cross-machine prerequisites clear
before owner tests run without becoming an installer or replacing the formats
that own those prerequisites.

## Maximum effort

One local evidence inventory, one primary-source format comparison, one
research note, one bounded wave, and one eleven-role review.

## Completion test

- prior Ferris failures are classified by owning layer;
- current owner-native formats are cited from primary documentation;
- one product boundary and first candidate slice are selected;
- affected crates, fixtures, tools, and consumers are named;
- non-goals, removal, and rollback are explicit; and
- no product code or adopter is changed.

## Result

Research findings `FERRIS-787` through `FERRIS-790` support a normalized,
owner-declared readiness contract on the existing `doctor` surface. Existing
formats are complementary evidence sources, not one interchangeable authority.

The smallest candidate slice checks explicit platform, executable,
environment-name, and repository-relative path requirements and emits typed,
deterministic observations. Installation, repair, version-command execution,
shell evaluation, lifecycle tasks, services, networks, secrets, resources,
inference, and owner work remain closed.

## Abandonment condition

Stop if the contract requires Ferris to resolve or install dependencies,
interpret executable owner workflows, retain sensitive values, or select a
winner among conflicting owner-native sources without explicit owner policy.

## Authority

Pulse 01 is documentation and research only. Pulses 02 through 04 remain
proposed and require separate explicit user approval.
