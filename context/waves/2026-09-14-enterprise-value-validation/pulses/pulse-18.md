# Pulse 18: ICELINES Pure-Cargo Value Cohort

Status: Authorized
Implementation authority: One bounded local measurement only

## User outcome

Determine whether Ferris planning can reduce representative owner validation
time for one real multi-package Cargo change without changing ICELINES, its
commands, or its required checks.

## Frozen case

- repository: public ICELINES;
- head: `935136020140bd5b408d26cbb0777dd6f0fb5ef9`;
- base: `41fec3dab0dd0d28e55a3b5d5f98c2ac650f108f`;
- Ferris: `1baa6a8`;
- selected owner commands:
  - `cargo test -p icelines-core --lib`;
  - `cargo test -p icelines-fetch`;
- full owner reference: `cargo test --workspace`; and
- platform: local Windows x86_64.

The head changes five paths across `icelines-core`, `icelines-fetch`, design
documentation, and its retained role evidence. The selected commands already
exist in the frozen repository's CI matrix. The full command already exists in
its README.

## Environment and preflight

The pulse may expose the existing Cargo installation and installed Visual
Studio 2022 x64 build environment only to invoking processes. Ordinary Cargo
dependency resolution under the unchanged lockfile is owner-native behavior;
no manifest, lockfile, toolchain, dependency, workflow, or persistent
environment change is authorized.

Before measurement, one separate disposable checkout MUST pass both selected
commands and the full command. Any failure ends the pulse with no measured
cohort.

## Planning and measurement

A disposable `ferris.owner-validation-domains/v2` declaration may map the five
committed paths to the two existing focused owner entrypoints. It MUST remain
outside ICELINES.

After preflight, use separate selected and full checkouts for eight pairs:

- four selected-first and four full-first;
- two cold and two warm pairs within each order;
- cold removes only that lane's checkout-local `target` before execution;
- warm retains the preceding checkout-local target state;
- every selected lane starts with a fresh revision-bound Ferris plan;
- selected elapsed includes planning plus both selected owner commands; and
- no failed or divergent pair contributes a performance result.

The gate passes only if all sixteen lanes pass, selected-pass/full-fail
divergence is zero, overall median selected elapsed is at least 10% below full,
and both orders and both cache states have positive median deltas.

## Boundaries and stop condition

No ICELINES or REEL mutation, dependency change, workflow change, command
inference, required-check narrowing, hosted execution, product change,
production claim, support claim, or broad savings extrapolation is authorized.
REEL is retained only as an independently researched candidate and contributes
no Pulse 18 measurement.

This is one fresh user-approved attempt after the closed BISECT attempts.
Preflight failure, any owner-lane failure, cleanup failure, or failure of the
frozen threshold ends the pulse. No corrective successor follows
automatically.

Product Value Governor disposition: `continue-within-budget`.
