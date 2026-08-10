# Ferris Specification Simulation Method

Date: 2026-08-10
Status: Complete
Decision: adopt no-code, wave-based hand simulation of the complete Ferris
specification spine before Proposed status or implementation.

## Research question

How should Ferris exercise its 22 Draft specifications deeply enough to expose
ambiguity, contradiction, unsafe defaults, ownership leaks, and missing failure
states without writing implementation code or mistaking AI predictions for
observations?

## Decision informed

Define the Ferris specification simulation program, scenario artifact format,
issue vocabulary, change-record lifecycle, wave progression, and first
simulation wave.

## Comparative local evidence

The method reviewed:

- `C:\src\craftworks\docs\research\simulation-research.md`;
- `C:\src\craftworks\design\_template\SCENARIOS.md`;
- `C:\src\craftworks\design\admin\research\simulate-examples.md`;
- `C:\src\signal\CHARTER.md`;
- `C:\src\signal\EVIDENCE-STANDARD.md`;
- `C:\src\signal\research\all-cannings-2008-dark-junction\TRACE-SPEC.md`; and
- `C:\src\signal\research\all-cannings-2008-half-turn-exchange\TRACE-SPEC.md`.

These paths are methodology sources. Ferris does not copy their domain models
or claim their scenario results.

## Findings

### FERRIS-779: hand-derived expectations expose specification defects before code

**Sources:** CRAFTWORKS simulation research Sections 3, 4, and 9; CRAFTWORKS
scenario template.

**Observed constraint:** CRAFTWORKS derives expected intermediate and final
artifacts directly from specifications before comparing them with
implementation. Ambiguous or forced traces become design findings rather than
being hidden by code behavior.

**Implication:** Ferris scenarios should hand-derive canonical records,
decisions, states, and public views from the Draft specifications. No generated
or executable artifact is needed during specification simulation.

**Confidence:** High.

### FERRIS-780: waves should increase composition and adversarial pressure

**Sources:** CRAFTWORKS simulation research Sections 6 and 9; CRAFTWORKS
cross-domain simulation examples Section 4.

**Observed constraint:** The reusable progression is baseline, feature
composition, edge cases, adversarial inputs, failure recovery, and
cross-system behavior.

**Implication:** Ferris should organize scenarios into coherent waves rather
than attempting one exhaustive end-to-end scenario.

**Confidence:** High.

### FERRIS-781: scenario issues need a change-record lifecycle distinct from research findings

**Sources:** CRAFTWORKS simulation research Section 7.

**Observed constraint:** CRAFTWORKS separates scenario findings from Design
Change Requests and retraces affected scenarios after amendment.

**Implication:** Ferris should retain `FERRIS-*` for research findings and use
`FSIM-SI-*` for simulation issues. Normative amendments use
`FSIM-SCR-*` Specification Change Records.

**Confidence:** High.

### FERRIS-782: a locked fixture must separate inputs, derivation, simulation, and claims

**Sources:** SIGNAL Evidence Standard; SIGNAL dark-junction and half-turn
locked trace specifications.

**Observed constraint:** SIGNAL freezes source selection, object choice,
operators, controls, pass conditions, uncertainty, and claim boundaries before
evaluation. It distinguishes observed, derived, simulated, hypothesized,
unsupported, falsified, and superseded states.

**Implication:** A Ferris fixture must freeze application, source change,
owner evidence, environment, policy, unknowns, and expected comparison before
AI traces the specs. The result state is `simulated`, never `observed`.

**Confidence:** High.

### FERRIS-783: held-out and negative controls are needed even for specification simulation

**Sources:** SIGNAL Charter commitments 5 and 7; SIGNAL half-turn trace bias
boundary; CRAFTWORKS wave and validation patterns.

**Observed constraint:** A successful trace is weaker when its expected result
was adjusted after seeing the outcome. Negative cases and matched controls
limit narrative fitting.

**Implication:** Each Ferris wave should freeze at least one negative or
adversarial scenario before tracing, and later waves should include held-out
fixtures authored separately from the spec-fix pass.

**Confidence:** High.

### FERRIS-784: AI is a simulator, not the implementation or evidence source

**Sources:** SIGNAL Evidence Standard; PREDICTION-001; EVIDENCE-001.

**Observed constraint:** A simulated result is conditional on declared inputs
and rules. It is not direct observation of a runtime.

**Implication:** AI output must cite governing spec sections, expose
interpretive choices, and classify every predicted record. AI agreement with a
spec does not validate runtime behavior.

**Confidence:** High.

### FERRIS-785: simulation completion requires retrace after every normative fix

**Sources:** CRAFTWORKS finding-to-DCR lifecycle and scenario template.

**Observed constraint:** A scenario remains open until the spec amendment is
applied and its expected trace is updated.

**Implication:** Ferris waves use:

```text
freeze fixture
  -> hand-trace specs
    -> record FSIM-SI issues
      -> approve FSIM-SCR changes
        -> amend specs
          -> retrace affected scenarios
            -> close or supersede issues
```

**Confidence:** High.

### FERRIS-786: specification simulation must remain implementation-free

**Sources:** Ferris CONTEXT.md and specification registry; CRAFTWORKS design
simulation distinction.

**Observed constraint:** Ferris implementation remains unauthorized.

**Implication:** Scenario fixtures are Markdown or static data declarations.
No crate, executable, generated runtime, CI action, connector call, external
post, or owner-system mutation is permitted.

**Confidence:** High.

## Recommendations

### Adopt now

- Establish `docs/simulations/`.
- Use scenario waves of three to six fixtures.
- Freeze fixtures before AI tracing.
- Require spec citations and explicit simulated claim state.
- Track `FSIM-SI-*` issues and `FSIM-SCR-*` changes.
- Apply normative fixes only after a wave-level review.
- Retrace every affected scenario after each fix.

Owner: FERRIS.
Validation: Markdown structure, spec citations, issue closure, retrace
consistency, and nine-role review at wave gates.

### Prototype behind a compatibility boundary

- Later derive machine-readable fixtures from stable scenario artifacts.
- Later compare an authorized implementation's actual output with the
  hand-derived prediction.

Owner: Conformance program.
Validation: CONFORMANCE-001.

### Reject or defer

- executable Ferris code during simulation;
- using AI output as runtime evidence;
- changing a fixture after tracing without a new revision;
- closing an issue without a spec amendment or explicit no-change rationale;
- one giant scenario standing in for wave coverage; and
- advancing specifications to Proposed based only on simulation.
