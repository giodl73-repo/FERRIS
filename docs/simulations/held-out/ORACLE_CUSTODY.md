# Held-Out Oracle Custody

Status: Frozen protocol

## Separation

The repository records fixture contracts but not executable expected-output
oracles.

For each bound fixture, the validation owner MUST retain separately:

- expected canonical records and result classes;
- mandatory inclusion and prohibition predicates;
- seeded hidden failures and unknowns;
- acceptable conservative alternatives;
- selected-versus-full-reference comparison;
- capability and validation consequences;
- allowed variance and performance thresholds; and
- stop, disable, rollback, and removal predicates.

## Access

Before scoring, implementation authors, model prompts, mapping authors, and
threshold tuners MUST NOT receive the oracle.

Validation Checker owns release of the oracle for scoring. Scope Keeper
confirms that the fixture was not used for development. AI Assurance Skeptic
reviews prompt, model, tool, and evidence exposure.

## Leakage

Oracle leakage includes:

- showing expected machine output to the implementation agent;
- tuning a prompt, rule, mapping, threshold, or fallback against the outcome;
- debugging directly on the held-out input;
- using a prior scored run as training or calibration evidence; or
- selecting only favorable fixture variants after observing results.

A leaked fixture is preserved as development evidence and replaced. Its prior
score remains in history but cannot support a held-out claim.

## Scoring release

After an immutable implementation build, configuration, and evidence cutoff
are frozen:

1. validation supplies the input fixture;
2. Ferris and owner-native full-reference commands run independently;
3. outputs and environment evidence are sealed;
4. the oracle is released to the scorer;
5. deviations are classified without rewriting the implementation or oracle;
6. the result is retained as pass, fail, unsupported, abstained, blocked, or
   invalid; and
7. any debugging creates a later development revision and replacement held-out
   fixture.

This protocol does not itself provide executable fixtures or authorize an
implementation.
