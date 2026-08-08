---
name: research
description: Run a hypothesis-led, cited FERRIUM research pass for Rust compiler, Cargo, native tooling, benchmarks, standards, or adoption decisions.
user-invocable: true
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
  - WebSearch
  - WebFetch
argument-hint: [topic-or-note-path] [--plan | --status | --resume]
---

# Research

Use this skill to investigate a FERRIUM research question before selecting,
designing, standardizing, or implementing a capability.

Research is hypothesis-led:

1. State what is currently believed.
2. State what evidence would confirm, refine, or overturn that belief.
3. Gather local and external evidence.
4. Run bounded experiments where observation is required.
5. Synthesize findings and decision implications.

Do not begin with a preferred product and collect only supporting evidence.

## Arguments

`$ARGUMENTS` accepts a topic or an existing research-note path plus an optional
mode:

- `<topic>`: run or continue the complete research workflow.
- `<path/to/note.md>`: operate on an existing note.
- `<topic> --plan`: define the question, hypothesis, evidence needs, and
  experiments without executing them.
- `<topic> --status`: report completed evidence, open experiments, confidence,
  and blockers.
- `<topic> --resume`: continue the first incomplete experiment or synthesis
  step.

When the topic is a `PERF-Qxx` identifier, resolve it from
`docs/research/questions/` and treat that question file as the investigation
plan. Use the registry dependencies and status vocabulary; do not merge several
question IDs into one research note.

When no topic is provided, infer it only when the active conversation and pulse
identify one unambiguously. Otherwise ask for the research question.

## Canonical locations

| Artifact | Location |
|---|---|
| Synthesized research note | `docs/research/YYYY-MM-DD-<slug>.md` |
| Experiment workspace | `docs/research/<slug>/` |
| Experiment plans or scripts | `docs/research/<slug>/scripts/` |
| Captured experiment results | `docs/research/<slug>/results/` |
| Raw public fixture data | `docs/research/<slug>/data/` |
| Performance question plans | `docs/research/questions/PERF-Qxx-*.md` |
| Active execution record | `context/waves/<active-wave>/pulses/<pulse>.md` |

Do not create an experiment workspace until an experiment needs a persistent
artifact. Keep simple source review in the synthesized note.

Research scripts are non-product artifacts. They must remain bounded,
reproducible, and separate from future FERRIUM packages.

## Source hierarchy

Prefer evidence in this order:

1. Local FERRIUM principles, plans, specs, prior findings, fixtures, and active
   wave records.
2. Source code, manifests, tests, benchmarks, and issue history from the project
   being studied.
3. Primary Rust sources: Rust project goals, rustc development guide, Cargo
   documentation, compiler source, RFCs, tracking issues, and rustc-perf.
4. Maintainer-authored design notes, release notes, talks, and performance
   reports.
5. Secondary commentary only when primary evidence does not answer the
   question.

Separate source-backed facts, measured observations, informed interpretations,
and hypotheses. Do not present one category as another.

## Privacy and corpus boundaries

- Public notes may name and cite public repositories.
- Private repositories are analyzed only with authorization.
- Never publish private repository names, remotes, paths, dependency names,
  source excerpts, logs, or identifiable measurements.
- Use anonymous fixture IDs and coarse build-shape descriptions for private
  evidence.
- A public product claim must also be reproducible on public or synthetic
  evidence.
- Do not send private source or results to third-party services.

## Workflow

### 1. Load operating context

Read:

1. `README.md`
2. `PRODUCT_PLAN.md`
3. `context/waves/PHASES.md`
4. the active `WAVE.md` and pulse
5. `docs/governance/ENGINEERING_PRINCIPLES.md`
6. relevant research, plans, specs, and `.roles`

Inventory existing findings before assigning IDs. New research findings use the
next available sequential `FERRIUM-XX` identifier across `docs/research/`.

### 2. Frame the decision

Record:

- the research question;
- the decision it should inform;
- the current hypothesis;
- plausible competing hypotheses;
- evidence that would change the current view;
- affected Rust/Cargo/compiler scopes;
- intended consumers;
- non-goals.

If the decision is unclear, remain in planning mode.

### 3. Inventory local evidence

Inspect the relevant:

- manifests and lockfiles;
- crate and target graph;
- profiles, features, build scripts, and procedural macros;
- tests, benchmarks, and fixtures;
- prior timing or profiler output;
- issue, commit, and pull-request history;
- active upstream or portfolio initiatives.

Record exact repository revisions and commands for measured claims. Do not
benchmark an unexplained dirty worktree.

### 4. Plan bounded investigations

For each investigation assign a stable label and define:

| Field | Requirement |
|---|---|
| Question | One uncertainty the investigation resolves |
| Method | Source review, web research, fixture measurement, controlled edit, or prototype |
| Inputs | Revisions, files, tools, environment, and cache state |
| Expected observation | What result would support or challenge the hypothesis |
| Output | Note section or `results/<label>.md` |
| Stop condition | When enough evidence exists or the method has failed |

Prefer controlled comparisons over broad data collection. Preserve negative and
inconclusive results.

### 5. Execute evidence collection

For desk research:

- search primary sources first;
- retain URLs and relevant sections;
- verify current claims against live sources when they may have changed;
- identify conflicting evidence explicitly.

For measurements:

- follow
  `docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`;
- use disposable fixture worktrees or copies;
- record environment, toolchain, command, cache state, revision, exit status,
  every sample, variance, and limitations;
- keep compiler, Cargo, codegen, link, acquisition, and validation time
  distinguishable;
- preserve failures rather than converting them into successful summaries.

For scripts:

- inspect the complete script before execution;
- prohibit destructive commands and broad cleanup;
- use a bounded timeout;
- capture stdout, stderr, and exit status;
- do not silently modify owner branches or global Cargo caches.

### 6. Write findings

Every actionable finding uses:

```markdown
### FERRIUM-XX: Finding title

**Sources**

- file path, line range, URL, issue, commit, or benchmark command

**Observation**

What the cited or measured evidence establishes.

**Implication**

What this changes for FERRIUM, Rust, Cargo, an upstream contribution, or a
prototype decision.

**Confidence:** High, medium, low, or inconclusive, with a reason when needed.
```

Do not inflate the finding count by splitting one observation into several
restatements.

### 7. Synthesize the decision

The research note should contain:

1. **Decision supported**
2. **Research question**
3. **Starting hypothesis**
4. **Local evidence**
5. **External evidence**
6. **Experiments and results**
7. **Findings**
8. **Model evolution**: what was confirmed, refined, or overturned
9. **Adopt now**
10. **Prototype behind a compatibility boundary**
11. **Reject or defer**
12. **Contribution path**: explain externally, configure/wrap, contribute
    upstream, or research further
13. **Non-goals**
14. **Open questions**
15. **Role review**

Lead with the decision-relevant answer. Evidence should make the conclusion
auditable rather than bury it.

### 8. Review through `.roles`

Always apply:

- Compiler Performance Engineer
- Ecosystem Strategist
- AI Assurance Skeptic
- Scope Keeper
- Validation Checker

Also apply:

- Rust Safety Steward for compiler, `unsafe`, cache, generic, or correctness
  boundaries;
- Interop Boundary Auditor for ABI, native dependency, linker, or mixed-language
  work;
- Rust Maintainer before proposing upstream or workflow changes;
- Native Platform Adopter when private, enterprise, CI, migration, or
  operational constraints matter.

Record each role's disposition and unresolved objection. Role acceptance means
the research addresses the role's questions; it does not approve implementation.

### 9. Update execution records

Update the active pulse with:

- research completed;
- findings or artifacts added;
- validation commands;
- unresolved measurements;
- whether the implementation gate remains closed.

Update `README.md`, `PRODUCT_PLAN.md`, or another plan only when the research
changes the public direction. Do not update TRACKER dependency records from the
child repo; leave that for an explicit portfolio pulse.

## Modes

### `--plan`

Produce or refine:

- question and decision;
- starting and competing hypotheses;
- evidence inventory;
- investigation table;
- expected outputs and stop conditions;
- relevant role reviewers.

Do not execute measurements or claim findings.

### `--status`

Report:

- note path and decision;
- completed and pending investigations;
- available result files;
- highest-confidence findings;
- contradictory or missing evidence;
- next executable step;
- current gate state.

Do not rewrite the research note.

### Default or `--resume`

Run the next incomplete investigation, persist its evidence, synthesize when
enough evidence exists, perform role review, and update the active pulse.

## Gate criteria

Research is complete only when:

- every actionable claim has a citation or measured command;
- repository revisions and environment assumptions are recorded;
- hypotheses and measured observations are distinguishable;
- negative, failed, and inconclusive evidence remains visible;
- recommendations identify ownership and validation;
- compatibility, privacy, safety, and maintenance boundaries are explicit;
- `.roles` review is recorded;
- public docs contain no private corpus details;
- implementation remains closed unless the active wave explicitly opens it.

## Guard rails

- Never change source code merely to make a research conclusion appear true.
- Never benchmark active owner branches with uncontrolled edits.
- Never delete global Cargo, rustup, compiler, or repository caches.
- Never treat compiler acceptance as behavioral proof.
- Never recommend skipped validation without coverage and uncertainty.
- Never depend directly on unstable rustc internals when a supported evidence
  surface can answer the question.
- Never create a compiler fork, backend, linker, or cache service as an
  incidental research artifact.
- Never hide evidence that weakens the preferred hypothesis.

## Commit message

`research: FERRIUM <topic>`
