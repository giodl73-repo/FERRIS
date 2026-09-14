# Pulse 01: Native-Linux Enterprise Baseline

Status: Complete
Implementation authority: Exhausted

## Decision

Determine whether `EO-01` and `EO-02` pass their unchanged owner commands on
Windows and in native WSL filesystem materializations of the same exact
revisions, outside the Windows user-profile Cargo configuration hierarchy.

This pulse addresses environment representativeness only. It does not onboard
Ferris.

## Authority

The owner's 2026-09-14 direction to solve the remaining unvalidated cases
authorizes the strict staged sequence. Because the user was unavailable for the
selection form, this pulse applies the recommended defaults: private consumers
before a real adopter and strict cross-platform evidence before promotion.

Pulse 01 MAY:

- run the unchanged owner commands in the existing clean Windows custody
  checkouts with a process-local existing Cargo `PATH`;
- create one temporary native WSL Git clone per consumer from the local custody
  checkout without network access;
- detach each clone at its exact custody revision and verify tracked-tree
  identity against the source checkout;
- install the same official checksum-verified PowerShell 7.6.6 Linux x64
  archive under `/opt/ferris-value-pulse-01`;
- run each unchanged owner command once from the native Linux clone root with
  the existing Rust 1.95 toolchain;
- record public-safe aggregate results and exact private custody; and
- delete both native clones, the PowerShell symlink, installation tree, and
  archive after evidence capture.

Pulse 01 MUST NOT:

- fetch from a remote host;
- edit, copy, or override user Cargo configuration;
- modify either source consumer;
- install a dependency or system package;
- translate or change an owner command;
- add Ferris files, tags, Action Plans, approvals, or receipts;
- claim hosted-CI, production, support, affected-only, performance, or savings
  evidence; or
- begin Pulse 02 without a separate closeout and authority decision.

## Identity and cleanliness

- Each source checkout MUST begin and end clean at its custody revision.
- Each native clone MUST have the same `HEAD` and tracked-tree identity as its
  source.
- Native clone remotes MUST be local custody paths only.
- Generated build outputs MAY exist only inside the temporary clones and MUST
  be deleted with those clones.
- Exact private paths and revisions MUST remain in custody.

## Stop conditions

Stop without retry on revision mismatch, tracked-tree mismatch, unexpected
network access, checksum mismatch, native tool preflight failure, owner command
failure, source consumer mutation, or cleanup failure.

## Completion

- both Windows owner commands pass;
- both native-WSL owner commands pass;
- no selected/full or Ferris claim is made;
- both source consumers remain clean at their exact revisions;
- both native clones and every Pulse 01 prerequisite are absent;
- public-safe research and eleven-role closeout are complete;
- changed Markdown local links and fences pass; and
- `git diff --check` passes.

## Result

- both Windows source checkouts were clean at their custody revisions;
- both temporary native WSL clones matched source `HEAD` and tracked-tree
  identities and had local-only origins;
- both unchanged Windows owner commands passed;
- both unchanged native-Linux owner commands passed with Rust 1.95;
- no consumer source or owner file changed; and
- both clones, their build outputs, the PowerShell installation, symlink, and
  archive were removed and verified absent.

The WSL elapsed-time wrapper produced invalid arithmetic values. Those values
were discarded rather than repaired or reported. Command success and owner
checks remain valid behavioral evidence; no performance conclusion follows.

## Closeout

The native-Linux baseline gate passes. The prior WSL failures were properties
of the mounted invocation environment, not either consumer's owner command.
Pulse 01 creates no Ferris onboarding, execution, savings, production, or
support evidence. Pulse 02 remains separately gated.
