# BISECT Real-Adopter Value Cohort

Date: 2026-09-14
Status: Complete; invalid measurement

## Frozen case

- BISECT revision:
  `2b90f9265997a042133262144ec81f8024ef5c45`;
- base revision:
  `d3dd4c6ef6a66ffea490bca21a6ad2853ccaf257`;
- Ferris revision: `8e2d0bd`;
- platform: local Windows x86_64;
- Python: 3.13.15; and
- Cargo: `1.95.0-ms-20260618.5`.

The selected command named the three repository-owned vault test files. The
full reference named the repository-owned `tests/unit` suite. The frozen
design required eight alternating selected/full pairs, explicit cold and warm
states, all sixteen lanes passing, zero selected-pass/full-fail divergence,
and at least 10% positive median value after charging fresh planning time to
the selected lane.

## Findings

### FERRIS-VALUE-017: Corrected Cargo binding enabled focused planning

Prepending the existing Cargo directory only to invoking processes resolved
Cargo before Ferris planning. Exact Ferris built successfully. All eight fresh
revision-bound plans completed without fallback, classified all 29 committed
inputs, and selected 29 declared focused owner entrypoints.

### FERRIS-VALUE-018: The frozen owner environment could not run pytest

Every selected and full owner command exited `1` before test collection because
Python reported `No module named pytest`. This affected all eight selected and
all eight full lanes equally. The pulse did not install a dependency because
dependency installation and environment mutation were explicitly excluded.

### FERRIS-VALUE-019: Failed owner lanes provide no performance evidence

The decision contract excludes every failed pair from performance results.
Consequently, zero elapsed durations are admissible, no medians are computed,
and no savings or positive-value claim follows. Focused selection correctness
does not substitute for runnable owner validation.

### FERRIS-VALUE-020: The bounded retry budget is exhausted

Pulse 16 was invalid before planning because Cargo was absent from the fresh
process `PATH`. Pulse 17 corrected that single input but stopped at the next
owner-environment prerequisite. These are two consecutive invalid attempts at
the same outcome, which triggers the Product Value Governor and Autonomy
Supervisor stop condition.

## Decision

The real-adopter value gate remains `No`. Advisory CI reconciliation and
compatibility/support readiness remain blocked by the wave order. No BISECT
file, workflow, dependency, or environment was changed; all disposable
checkouts were removed and the retained source clone remained clean. Any
environment-qualified measurement would require fresh explicit user and
Product Value Governor approval rather than an automatic corrective pulse.
