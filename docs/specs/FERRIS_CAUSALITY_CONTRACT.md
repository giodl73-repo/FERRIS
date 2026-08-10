# CAUSALITY-001: Ferris Causality and Explanation Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: FOREST-003 and the Build Latency Measurement Contract

## Purpose

This specification defines causal claim classes, evidence requirements,
confidence, confounders, unknowns, and bounded explanations over observed
Query Forest evidence.

Causality explains an observed result. It does not predict unobserved work,
select a plan, approve an action, or execute a command.

## Claim classes

Every causal claim MUST use one of:

- directly observed cause;
- observed transitive cause;
- owner-declared dependency;
- experimentally supported cause;
- inferred candidate cause;
- correlated condition;
- ruled-out candidate;
- conflicting evidence; or
- unknown cause.

An inferred or correlated relationship MUST NOT be presented as directly
observed. A prediction becoming correct later does not retroactively make its
original claim an observation.

## Causal statement

Every statement MUST identify:

- observed effect;
- candidate or established cause;
- source and target scope;
- direction and mechanism;
- canonical edge path;
- owner systems crossed;
- command, environment, and evidence root;
- observation times;
- claim class;
- confidence and basis;
- alternative explanations;
- controlled and uncontrolled confounders;
- limitations and omitted scope; and
- decision owner when escalation is required.

Confidence MUST describe evidence strength for the named claim. It MUST NOT be
used as a universal safety, correctness, compatibility, trust, or approval
score.

## Stage-specific build causality

Build explanations MUST distinguish at least:

- source or input changed;
- owner selection changed;
- identity namespace changed;
- freshness changed within one namespace;
- owner tool or provider executed;
- result proved equal or reusable;
- downstream invalidation propagated;
- compilation occurred;
- code generation occurred;
- link inputs changed;
- relinking occurred;
- validation executed;
- runtime behavior changed; and
- final capability or artifact changed.

A cache miss, provider execution, rebuilt package, changed filename, or elapsed
time alone MUST NOT stand in for the full causal chain.

## Evidence rules

Direct or transitive observed claims MUST cite FOREST-003 projected records
that resolve to canonical observations.

Experimental claims MUST identify:

- fixture and revision;
- controlled change;
- baseline and comparison;
- command and environment;
- repetitions and variance where measurable;
- negative or counterfactual control;
- failures and excluded runs; and
- transfer limitations.

Owner documentation MAY establish declared semantics. It does not establish
that the declared mechanism occurred in one observed run without run evidence.

## Confounders

Explanations MUST preserve applicable:

- source-layout movement;
- toolchain or dependency drift;
- feature, target, profile, and configuration changes;
- build-script, macro, generator, native, and environment inputs;
- cache and prior-state differences;
- filesystem, VM, security, indexing, power, thermal, and background pressure;
- concurrency and lock contention;
- missing instrumentation; and
- clock, sampling, or aggregation limitations.

An environment-equivalent comparison MUST name which dimensions matched and
which remain uncontrolled.

## Causal paths

A transitive explanation MUST:

- retain every material owner boundary;
- identify direct versus inferred edges;
- detect cycles;
- bound path length or aggregation;
- identify fan-out and fan-in;
- preserve unknown segments; and
- name the nearest safe fallback explanation when detail is unavailable.

Path compression MAY improve presentation. The uncompressed evidence path MUST
remain queryable.

## User explanations

Maintainer-facing explanations SHOULD answer:

- what changed;
- what work occurred;
- what was reused or skipped;
- what waited or blocked;
- why the work propagated;
- what remains unknown;
- which validation and capabilities were covered; and
- what evidence supports the answer.

Diagnostics MUST use Cargo, Rust, native, validation, contract, and repository
owner vocabulary before internal Forest terminology.

## AI boundary

AI MAY summarize evidence, propose candidate causes, or suggest missing probes.
It MUST:

- cite the evidence available;
- label inference and uncertainty;
- retain alternative explanations;
- avoid inventing absent edges;
- avoid converting timing correlation into causation; and
- require deterministic evidence or human review before a candidate cause
  influences narrowing or action.

## Acceptance criteria

CAUSALITY-001 may advance to Proposed only when:

1. direct, transitive, declared, experimental, inferred, correlated,
   ruled-out, conflicting, and unknown claims have fixtures;
2. provider execution, result equality, invalidation, compilation, codegen,
   linking, validation, and capability effects remain distinct;
3. controlled edits and negative controls reproduce known causal chains;
4. environment confounders and source-layout effects remain visible;
5. every explanation traces to canonical evidence;
6. AI-generated narratives cannot upgrade claim class or hide alternatives;
7. maintainers answer the bounded workflow question faster than with raw
   owner tools without losing material uncertainty; and
8. all nine roles record a disposition.
