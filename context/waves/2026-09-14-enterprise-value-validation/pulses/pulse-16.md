# Pulse 16: BISECT Real-Adopter Value Cohort

Status: Complete; invalid before planning
Implementation authority: Exhausted

## Frozen case

- repository: public BISECT;
- revision: `2b90f9265997a042133262144ec81f8024ef5c45`;
- base: `d3dd4c6ef6a66ffea490bca21a6ad2853ccaf257`;
- platform: local Windows x86_64;
- selected owner command:
  `python -m pytest tests/unit/test_data_vault.py tests/unit/test_data_vault_failures.py tests/unit/test_evidence_vault.py -q`;
- full owner reference:
  `python -m pytest tests/unit -q`; and
- Ferris revision: current clean `work/enterprise-value-validation` head before
  this authority commit.

The three selected tests and full unit suite are repository-owned at the frozen
revision. The selected command is a strict subset of the same test runner and
suite shape.

## Planning

A disposable `ferris.owner-validation-domains/v2` declaration may classify
every committed path in the base-to-head change set into explicit focused
owner entrypoints. It MUST NOT alter BISECT. The revision-bound plan must
succeed without fallback and select only the declared focused owner domains.

## Measurement

Run eight pairs:

- four pairs execute selected then full;
- four pairs execute full then selected;
- within each order, two pairs remove Python bytecode and pytest cache before
  each lane (`cold`) and two retain them (`warm`);
- selected elapsed time includes its fresh Ferris planning duration;
- lanes use separate disposable worktrees and no shared target directory;
- raw paths and command output remain private; and
- no failed or divergent pair contributes a performance result.

## Decision

The value gate passes only if:

- all sixteen owner lanes pass;
- selected-pass/full-fail divergence is zero;
- the median selected elapsed time including planning is at least 10% lower
  than the median full elapsed time; and
- both execution orders and both cache states have a positive median delta.

Otherwise the gate remains `No`, with the observed reason recorded. Package
counts are not savings evidence.

## Exclusions

No BISECT mutation, dependency installation, workflow change, required-check
narrowing, hosted execution, product change, production claim, support claim,
or commercial savings extrapolation is authorized.

## Result

The exact Ferris and two BISECT checkouts built and materialized. The
revision-bound planning preflight then stopped with
`FERRIS-CARGO-UNAVAILABLE` because the fresh invoking shell omitted the
installed Cargo directory from `PATH`.

No valid plan, selected or full owner lane, timing, or value conclusion
followed. All three disposable checkouts were removed. Pulse 16 is invalid
measurement evidence and exhausted.
