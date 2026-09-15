# Real-Adopter Value Candidate Selection

Date: 2026-09-15
Status: Complete

## Findings

### FERRIS-VALUE-021: BISECT receives no enterprise implementation

Ferris used disposable BISECT checkouts and an external owner-domain
declaration. No enterprise-consumer code, Ferris configuration, dependency,
commit, branch, or push entered BISECT. The enterprise-relevant behavior is
Ferris's revision-bound planning and opaque owner-entrypoint selection, not an
adopter-side code transfer.

GitHub reported `giodl73-repo/BISECT` as public on 2026-09-15.

### FERRIS-VALUE-022: ICELINES is the strongest pure-Cargo value candidate

At `935136020140bd5b408d26cbb0777dd6f0fb5ef9`, ICELINES contains seven Cargo
workspace packages and 106 Cargo targets. Its first-parent change from
`41fec3dab0dd0d28e55a3b5d5f98c2ac650f108f` modifies five paths, including
`icelines-core` and `icelines-fetch`, and records a repository-owned performance
correction.

The frozen CI matrix already owns `cargo test -p icelines-core --lib` and
`cargo test -p icelines-fetch`. The frozen README owns
`cargo test --workspace` as the full reference. This creates a command-shape
preserving selected/full comparison without Ferris parsing a workflow or
inventing a test command.

### FERRIS-VALUE-023: REEL is useful but weaker for this decision

At `5c896f5a9d3fb7bd4a709ace66dfe119c6a568bb`, REEL contains two Cargo packages
and 74 Cargo targets. Its latest first-parent change modifies 13 paths across
root-package scene and episode delivery behavior. REEL owns focused scene
delivery commands, but that case primarily narrows within one root package and
requires FFmpeg. It is therefore weaker than ICELINES for the immediate
cross-Cargo value decision.

## Decision

Use ICELINES for one preflight-gated local Windows value cohort. Retain BISECT
as polyglot owner-domain evidence and REEL as an independent future adopter
candidate. Do not copy synthetic enterprise fixtures or implementation into
any adopter.
