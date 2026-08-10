# PREDICTION-001: Ferris Prediction and Held-Out Evaluation Contract

Status: Draft after nine-role review
Implementation authority: None
Depends on: CAUSALITY-001, SCOPE-001, and the Build Latency Measurement Contract

## Purpose

This specification defines Prediction Records, uncertainty, held-out
evaluation, calibration, error analysis, fallback, expiry, and model
accountability.

A prediction forecasts unobserved outcomes. It never becomes an observation,
approval, or executable plan.

## Prediction record

Every Prediction Record MUST include:

- prediction ID, schema, version, and parent;
- model, rule, heuristic, or human predictor identity;
- instruction, configuration, and feature-set identity;
- application, source, prior Forest root, and triggering change;
- prediction time and evidence cutoff;
- selected SCOPE-001 coordinates;
- forecast owner-specific closures;
- expected fresh, executed, equal, invalidated, rebuilt, relinked, reused,
  restored, skipped, failed, and fallback states;
- expected validation and capability consequences;
- predicted critical path, user latency, machine work, resource demand, and
  artifact economics where applicable;
- confidence or probability with its interpretation;
- assumptions, unknowns, omitted scope, and confounders;
- observation barriers and fallback; and
- expiry and invalidation triggers.

The evidence cutoff MUST prevent later observations from being represented as
inputs to the original prediction.

## Prediction classes

Predictions MUST distinguish:

- affected scope;
- owner work;
- state transition;
- causal mechanism;
- duration or cost;
- resource demand;
- artifact eligibility and benefit;
- validation coverage;
- capability preservation;
- failure or unsupported state; and
- required fallback.

One successful class MUST NOT imply correctness of another.

## Reference and held-out data

Evaluation data MUST be divided into:

- development or rule-authoring evidence;
- calibration evidence;
- held-out fixtures or edits; and
- full-reference observations.

Held-out identity and expected outcomes MUST be frozen before prediction.
Models, prompts, heuristics, thresholds, and mappings MUST NOT be revised using
the held-out outcome and then scored as if it remained held out.

Private evidence MUST remain isolated and disclosure-reviewed. Public product
claims MUST also reproduce on public or synthetic fixtures.

## Uncertainty and calibration

Confidence MUST be tied to a named prediction class, population, and scoring
method.

Evaluation MUST report applicable:

- true and false affected selections;
- false omissions;
- precision and recall;
- error magnitude and direction;
- interval or quantile coverage;
- calibration by confidence band;
- unsupported and abstained cases;
- fallback frequency;
- stale-evidence effects; and
- results by repository shape, command, platform, and owner boundary.

False omissions that remove correctness, safety, compatibility, policy, or
validation work MUST be reported separately from harmless over-selection.

## Comparison with observation

After owner-local execution, a comparison record MUST retain:

- immutable Prediction Record;
- observed Forest root;
- scope and environment comparability;
- predicted and observed states;
- error classification;
- newly discovered inputs or mappings;
- whether fallback or replan triggered;
- capability and validation consequence;
- user and machine cost; and
- disposition: retain, recalibrate, narrow claim, widen fallback, disable, or
  require owner input.

Observed deviations MUST NOT rewrite the original prediction.

## Safe fallback

Low-confidence, conflicting, stale, out-of-distribution, missing-owner,
unsupported, or failed predictions MUST:

- abstain;
- widen to the smallest safe owner boundary;
- request an observation barrier;
- use the full-reference plan; or
- block pending owner input.

They MUST NOT silently produce an empty or narrower work set.

## Model and AI accountability

Model-produced predictions MUST record:

- model and provider identity;
- model version;
- instruction or prompt reference;
- tools and evidence exposed;
- data classification and tenant boundary;
- generated output;
- deterministic normalization;
- policy and human decisions;
- rejection or override; and
- measured error.

Prompt text, model confidence, or persuasive explanation is not owner evidence.

## Lifecycle

Prediction rules and models MUST support active, experimental, degraded,
disabled, superseded, expired, and retained-historical states.

Renewal MUST occur when schemas, owner tools, repositories, mappings,
toolchains, platforms, environments, contracts, profiles, policies, or error
distributions change.

## Acceptance criteria

PREDICTION-001 may advance to Proposed only when:

1. every prediction class has frozen held-out fixtures;
2. evidence cutoff and data-leakage controls are executable;
3. uncertainty is calibrated for named populations;
4. false omissions, over-selection, abstention, and fallback are reported
   separately;
5. prediction-versus-observation records remain immutable and attributable;
6. unknown and out-of-distribution cases widen or abstain safely;
7. model-produced predictions satisfy privacy, authority, and error controls;
8. claimed maintainer benefit survives public, synthetic, Windows, and Unix
   held-out evaluation; and
9. all nine roles record a disposition.
