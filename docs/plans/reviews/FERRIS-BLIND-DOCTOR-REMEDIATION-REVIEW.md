# Ferris Blind Doctor Remediation Review

Date: 2026-08-11
Scope: Pulse 06 remediation after FHIF-013
Disposition: Validated; replacement held-out fixture pending
Implementation authority: No expansion

Historical note: FHIF-014 subsequently failed the Pulse 06 cutoff. Pulse 07
replaced manual report identity inputs with the complete typed record and
tightened canonical commit/date validation.

## Public-safe finding

FHIF-013 was designed and sealed independently before its first execution. It
failed `P05-PASSIVE-DOCTOR-BOUNDED-IDENTITY` at the Pulse 05 cutoff. The
implementation context received no hidden input or oracle predicate.

The released remediation identified two categories only:

- Cargo evidence classification was insufficiently strict; and
- post-read report identity omitted material semantics.

FHIF-013 is retained as failed development evidence and cannot support later
held-out scoring.

## Corrections

- Cargo output now has a canonical bounded grammar.
- Safe semantic version, commit, and release-date fields are explicit.
- Extra tokens and malformed near-canonical evidence are unsupported.
- Doctor records expose all resource bounds and output framing.
- Report and invocation identity bind command, working-directory semantics,
  resource bounds, passive controls, toolchain selection, parsed owner
  evidence, and framed owner-output digest.

## Validation

Windows MSVC and Ubuntu 24.04 WSL2 passed:

```console
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

The suite reports 24 core tests passed, 2 process-helper tests intentionally
ignored except when invoked by their bound tests, and 12 CLI tests passed.

## Role dispositions

All nine roles accept the deterministic correction without capability
expansion. Validation Checker withholds a held-out doctor pass until a new
opaque replacement fixture is independently sealed and scored.

## Remaining gate

- create a replacement fixture with a new ID and oracle before first run;
- never execute FHIF-013 again for scoring; and
- publish only the replacement fixture's public-safe receipt.
