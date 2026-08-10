# Ferris Seven-Program Synthesis

Date: 2026-08-10
Status: Complete
Decision: organize the complete Ferris research corpus into seven programs:
Ferris, Typebook, Profiles, Blueprint, Query Forest, Conformance, and Ecosystem
Bridge.

## Decision supported

This synthesis determines the final program architecture that should govern
remaining specifications and later bounded implementation decisions.

It is based on:

- [PERF-Q01 through PERF-Q36](questions/README.md);
- [ECOS-Q01 through ECOS-Q12](questions/ecosystem/README.md);
- [BLUE-Q01 through BLUE-Q05](questions/blueprint/README.md);
- [Ferris Program](../plans/FERRIS_PROGRAM.md);
- [Blueprint Planning Engine Program](../plans/BLUEPRINT_PROGRAM.md);
- [Rust contract and interface strategy](2026-08-10-rust-contract-interface-strategy.md);
- [Rust compatibility-tested stack profiles](2026-08-10-rust-compatibility-stack-profiles.md);
- [Rust ecosystem intervention decisions](2026-08-10-rust-ecosystem-intervention-decisions.md); and
- [Rust performance contribution closeout](2026-08-09-rust-performance-contribution-program-closeout.md).

The detailed mapping is in the
[research closure matrix](ferris-seven-programs/results/EXP-01-research-closure-matrix.md).

## Corpus inventory

The current repository records:

```text
Performance questions: 36
Crates Series questions: 12
Blueprint/product questions: 5
Total completed research questions: 53
Finding sequence: FERRIUM-01 through FERRIS-758 before this synthesis
```

The original finding set has therefore been expanded and tested through a
larger question corpus. This synthesis preserves both the original insights
and the later controls, counterexamples, ownership boundaries, and
intervention decisions.

## Why seven programs

One program would collapse distinct ownership and lifecycle contracts.
Dozens of independent programs would reproduce the fragmentation Ferris is
intended to coordinate.

Seven programs are sufficient because they separate:

1. public product and operator experience;
2. product-neutral semantic meaning;
3. renewable support commitments;
4. application planning;
5. evidence and causality;
6. executable proof; and
7. external owner integration.

## Recommendations

### Adopt now

- Use the seven-program architecture as the governing specification spine.
- Keep Ferris as the only public build-system product and CLI.
- Keep Typebook independently useful and product-neutral.
- Treat profiles as portable, renewable records rather than a distribution.
- Keep Blueprint non-executable until approval projection.
- Make Query Forest the canonical evidence/history model, not a cache key.
- Treat Conformance as a program that gates every other program.
- Route ecosystem changes through current owners and contribution packets.

Owners: FERRIS for Ferris, Blueprint, Query Forest, Profiles integration,
Conformance, and Ecosystem Bridge contracts; RUNE/Typebook for neutral semantic
contracts; external projects for their local capabilities.

Expected validation: specification review, closure-matrix coverage, held-out
fixtures, selected/full-reference comparisons, cross-platform proof, negative
and unsupported controls, adoption, rollback, and removal.

### Prototype behind compatibility boundaries

- one Typebook-authored contract consumed by one profile and application;
- one multi-workspace affected plan produced through Blueprint;
- one Query Forest root and explanation;
- one `ferris` / `cargo ferris` parity fixture;
- one full-reference comparison; and
- one owner-aligned upstream contribution packet.

### Reject or defer

- seven independent CLIs or product brands;
- merging Typebook into Ferris;
- a curated crate distribution, certification, or universal score;
- a monolithic global graph or resolver;
- remote execution or artifact restoration without separate evidence;
- automatic dependency, source, profile, environment, or CI mutation; and
- implementation before specification and conformance gates.

## Findings

### FERRIS-759: one public product needs several bounded programs

**Sources:** PERF-Q01 through PERF-Q36, ECOS-Q01 through ECOS-Q12, BLUE-Q01
through BLUE-Q05.

**Observed behavior:** the corpus spans public workflow, contracts, support,
planning, evidence, validation, and external ownership with different
authorities and lifecycle rules.

**Implication:** Ferris should expose one product while retaining seven bounded
programs beneath and beside it.

**Confidence:** High.

### FERRIS-760: Typebook remains separately useful

**Sources:** Rust contract and interface strategy; ECOS-Q03, ECOS-Q04,
ECOS-Q09, and ECOS-Q11.

**Observed behavior:** semantic contracts and projections serve consumers
outside Ferris and must remain neutral across Rust, C ABI, WIT, wire, and
generated boundaries.

**Implication:** Typebook remains a separate standards program consumed by
Ferris through CONTRACT-001.

**Confidence:** High.

### FERRIS-761: profiles are renewable support records

**Sources:** ECOS-Q07, ECOS-Q10, ECOS-Q11, and ECOS-Q12.

**Observed behavior:** support varies by exact release, feature closure,
compiler, target, provider, native tools, stage, evidence date, owner, and
lifecycle.

**Implication:** Profiles require their own program and PLATFORM-001 contract;
they are not static crate lists or certification.

**Confidence:** High.

### FERRIS-762: Blueprint owns planning, not owner-local semantics

**Sources:** PERF-Q02 through PERF-Q35, BLUE-Q02, and BLUE-Q03.

**Observed behavior:** Cargo commands, compiler stages, tests, contracts,
native work, validation, and deployment each retain different graphs and
identities.

**Implication:** Blueprint composes typed owner closures and resource policy
but does not replace local planners or execute without approval.

**Confidence:** High.

### FERRIS-763: Query Forest is evidence and history, not universal identity

**Sources:** PERF-Q17 through PERF-Q32 and BLUE-Q01.

**Observed behavior:** source, Cargo, compiler, artifact, validation,
environment, action, and lifecycle identities have different compatibility
envelopes.

**Implication:** Query Forest joins typed evidence and immutable roots while
rejecting universal hashes, labels as authority, and automatic restoration.

**Confidence:** High.

### FERRIS-764: conformance is a first-class program

**Sources:** Build Latency Measurement Contract, PERF-Q01, PERF-Q35, ECOS-Q03
through ECOS-Q11, and CONFORMANCE-001.

**Observed behavior:** nearly every attractive optimization or compatibility
claim fails without negative, unsupported, version-skew, cross-platform,
full-reference, rollback, and removal controls.

**Implication:** Conformance gates each program continuously rather than
appearing only at release.

**Confidence:** High.

### FERRIS-765: ecosystem integration must route to current owners

**Sources:** PERF-Q36 and ECOS-Q01 through ECOS-Q12.

**Observed behavior:** Cargo, rustc, crates, standards, native tools,
platforms, and security systems already own their local contracts and
contribution processes.

**Implication:** the Ecosystem Bridge owns adapters and contribution packets,
not replacement systems or stewardship takeover.

**Confidence:** High.

### FERRIS-766: every cross-program edge needs identity and lifecycle

**Sources:** PERF-Q02, PERF-Q18, PERF-Q30, ECOS-Q05 through ECOS-Q11,
BLUE-Q01, and BLUE-Q03.

**Observed behavior:** compatible-looking records become unsafe or stale when
scope, version, owner, evidence date, platform, activity, expiry, or fallback
is omitted.

**Implication:** all cross-program contracts carry typed identity, scope,
owner, evidence, lifecycle, unknown, and fallback fields.

**Confidence:** High.

### FERRIS-767: the complete corpus maps without a monolith

**Sources:** EXP-01 research closure matrix.

**Observed behavior:** all 53 completed questions have a primary program and
specification path; intentional overlaps occur at contract, evidence,
planning, validation, and owner boundaries.

**Implication:** no learning requires an eighth program or one universal
component at this stage.

**Confidence:** High.

### FERRIS-768: the seven-program architecture is ready for specification use

**Sources:** FERRIS-759 through FERRIS-767 and the nine-role review.

**Observed behavior:** missions, ownership, outputs, boundaries, dependencies,
specifications, review stages, and non-goals are explicit.

**Implication:** remaining specifications should use this architecture; no
implementation authority follows.

**Confidence:** High.

## Limitations

- Program boundaries may split further when executable fixtures expose
  independent lifecycle needs.
- Typebook naming and any RUNE migration remain separate repository decisions.
- Exact first-proof repositories, commands, schemas, thresholds, and support
  commitments are not yet frozen.
- The architecture is comprehensive for the current corpus, not a claim that
  every future Rust build capability is already known.

